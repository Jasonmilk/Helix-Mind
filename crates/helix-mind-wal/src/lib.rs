//! 领域 WAL（ADR-0015）—— 独立追加日志文件作为事实来源。
//!
//! 架构：WAL 是唯一事实来源（append-only + fsync），SQLite 是液态投影。
//! 本模块实现：
//! - **写路径**：`WalWriter`（单条 fsync / 批量），段轮转（固定大小段 + 跨段索引）。
//! - **BLAKE3 哈希链**：每条事件记录携带 `prev_hash`（前一记录载荷哈希），
//!   **跨段连续**（段 N 首条记录的 prev_hash = 段 N-1 末条记录哈希）。
//!   用于**完整性校验（防位腐坏与意外损坏）**，**不声明防恶意篡改**
//!   （无密钥哈希链可被重算；防篡改需未来密钥基础设施，ADR-0015）。
//! - **replay**：`WalReader` 跨段遍历 + 逐条哈希链验证 + 崩溃截断恢复，
//!   返回 `ReplayOutcome`（含最后段有效长度，供续接 truncate 截断尾）。

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;

use helix_mind_core::graph::{AuditEntry, Edge, Node};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// WAL 事件（事实来源的原子单元）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalEvent {
    /// 节点写入/更新（L1-L3）。
    NodeWritten(Node),
    /// 边新增。
    EdgeAdded(Edge),
    /// 节点标记为隐性（突触切断）。
    NodeMarkedRecessive(Uuid),
    /// 审计日志追加。
    AuditWritten(AuditEntry),
}

/// WAL 错误。
#[derive(Debug, Error)]
pub enum WalError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("hash chain broken at segment {segment} record {record}: expected {expected}, got {actual}")]
    HashChainBroken {
        segment: u64,
        record: u64,
        expected: String,
        actual: String,
    },
    #[error("corrupt wal: {0}")]
    Corrupt(String),
}

/// 创世哈希（第一条记录的 prev_hash，也是段 0 的起始 prev_hash）。
const GENESIS_HASH: [u8; 32] = [0u8; 32];

/// WAL 配置。
#[derive(Debug, Clone)]
pub struct WalConfig {
    /// WAL 根目录（`dir/<seq>.wal` 段文件）。
    pub dir: PathBuf,
    /// 段大小上限（字节）。默认 64MB。
    pub segment_size: u64,
}

impl WalConfig {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self {
            dir: dir.into(),
            segment_size: 64 * 1024 * 1024,
        }
    }
}

/// 一条已校验的 WAL 记录（replay 产物）。
#[derive(Debug, Clone)]
pub struct WalRecord {
    /// 记录所在段序号。
    pub segment: u64,
    /// 事件。
    pub event: WalEvent,
    /// 该记录载荷的 BLAKE3 哈希（hex 大写，展示用）。
    pub hash: String,
    /// 该记录载荷的 BLAKE3 哈希（字节，续接链用）。
    pub hash_bytes: [u8; 32],
}

/// replay 结果。
#[derive(Debug, Clone)]
pub struct ReplayOutcome {
    /// 全部已校验记录（段升序、段内顺序）。
    pub records: Vec<WalRecord>,
    /// 最后一段序号（无记录时为 None）。
    pub last_segment: Option<u64>,
    /// 最后一段的有效字节数（不含截断尾；供续接 truncate）。
    pub last_segment_valid_len: u64,
    /// 最后一段的有效记录数。
    pub last_segment_records: u64,
}

fn hash_payload(payload: &[u8]) -> [u8; 32] {
    blake3::hash(payload).into()
}

fn segment_path(dir: &Path, seq: u64) -> PathBuf {
    dir.join(format!("{:08}.wal", seq))
}

fn discover_segments(dir: &Path) -> Vec<u64> {
    let mut segs: Vec<u64> = std::fs::read_dir(dir)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .filter_map(|e| {
                    let name = e.file_name();
                    let s = name.to_string_lossy();
                    s.strip_suffix(".wal").and_then(|n| n.parse::<u64>().ok())
                })
                .collect()
        })
        .unwrap_or_default();
    segs.sort_unstable();
    segs
}

fn hex_upper(b: &[u8]) -> String {
    let mut s = String::with_capacity(b.len() * 2);
    for byte in b {
        s.push_str(&format!("{:02X}", byte));
    }
    s
}

// ───────────────────────── 写路径 ─────────────────────────

/// WAL 写入器（单写入者，外部串行）。
pub struct WalWriter {
    dir: PathBuf,
    segment_size: u64,
    /// 当前段文件。
    file: File,
    /// 当前段序号。
    segment_seq: u64,
    /// 当前段字节数。
    segment_len: u64,
    /// 当前段已写记录数。
    segment_records: u64,
    /// 上一条记录载荷哈希（跨段延续）。
    last_hash: [u8; 32],
    /// 已产生的段列表（段序号升序）。
    segments: Vec<u64>,
}

impl WalWriter {
    /// 打开（或续接）WAL。若已有段：执行**全链校验**（replay）后恢复到末尾，
    /// 并 truncate 掉截断尾（崩溃残留），保证后续 append 从有效边界续写。
    pub fn open(config: &WalConfig) -> Result<Self, WalError> {
        std::fs::create_dir_all(&config.dir)?;
        let segments = discover_segments(&config.dir);

        // 全链校验 + 续接点。
        let reader = WalReader::new(config);
        let outcome = reader.replay_all()?;

        let (segment_seq, last_hash, valid_len, segment_records) =
            if let Some(last) = outcome.records.last() {
                (last.segment, last.hash_bytes, outcome.last_segment_valid_len, outcome.last_segment_records)
            } else if segments.is_empty() {
                (0, GENESIS_HASH, 0, 0)
            } else {
                // 段文件存在但无有效记录（空段/全截断）：从段 0 重新开始。
                (0, GENESIS_HASH, 0, 0)
            };

        let path = segment_path(&config.dir, segment_seq);
        let file = if path.exists() {
            let f = OpenOptions::new().append(true).read(true).open(&path)?;
            f.set_len(valid_len)?; // 清除截断尾
            f
        } else {
            OpenOptions::new()
                .create_new(true)
                .write(true)
                .read(true)
                .open(&path)?
        };

        // segments 列表：全部已存在段 + 当前段。
        let mut all = segments;
        if !all.contains(&segment_seq) {
            all.push(segment_seq);
        }
        all.sort_unstable();

        Ok(Self {
            dir: config.dir.clone(),
            segment_size: config.segment_size,
            file,
            segment_seq,
            segment_len: valid_len,
            segment_records,
            last_hash,
            segments: all,
        })
    }

    /// 追加一条事件（不 fsync；批量策略下周期 `sync`）。
    pub fn append(&mut self, event: &WalEvent) -> Result<u64, WalError> {
        // payload = prev_hash(32) + event_json
        let json = serde_json::to_vec(event)
            .map_err(|e| WalError::Corrupt(format!("event serialize: {}", e)))?;
        let mut payload = Vec::with_capacity(32 + json.len());
        payload.extend_from_slice(&self.last_hash);
        payload.extend_from_slice(&json);

        let record_len = (payload.len() as u32).to_le_bytes();
        self.file.write_all(&record_len)?;
        self.file.write_all(&payload)?;

        let new_hash = hash_payload(&payload);
        self.last_hash = new_hash;
        self.segment_len += 4 + payload.len() as u64;
        self.segment_records += 1;

        let seg = self.segment_seq;

        // 段轮转：超限开新段。
        if self.segment_len >= self.segment_size {
            self.rotate()?;
        }

        Ok(seg)
    }

    /// 追加并立即 fsync（强一致路径）。
    pub fn append_synced(&mut self, event: &WalEvent) -> Result<u64, WalError> {
        let seg = self.append(event)?;
        self.file.sync_all()?;
        Ok(seg)
    }

    /// fsync 当前段（批量策略）。
    pub fn sync(&mut self) -> Result<(), WalError> {
        self.file.sync_all()?;
        Ok(())
    }

    /// 段轮转：关闭当前段，开新段（序号 +1），哈希链跨段延续。
    fn rotate(&mut self) -> Result<(), WalError> {
        self.file.sync_all()?;
        self.segment_seq += 1;
        let path = segment_path(&self.dir, self.segment_seq);
        self.file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .read(true)
            .open(path)?;
        self.segment_len = 0;
        self.segment_records = 0;
        self.segments.push(self.segment_seq);
        Ok(())
    }

    /// 已产生的段列表。
    pub fn segments(&self) -> &[u64] {
        &self.segments
    }

    /// 当前段序号。
    pub fn current_segment(&self) -> u64 {
        self.segment_seq
    }
}

// ───────────────────────── 读取 / replay ─────────────────────────

/// WAL 读取器（replay）。
pub struct WalReader {
    config: WalConfig,
}

impl WalReader {
    pub fn new(config: &WalConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /// 单段读取：从 `start_hash` 开始校验哈希链，返回记录、末端哈希与有效字节数。
    /// 截断尾部（不完整记录）被忽略（崩溃恢复语义）。
    fn read_segment(
        &self,
        segment: u64,
        start_hash: [u8; 32],
    ) -> Result<(Vec<RawRecord>, [u8; 32], u64), WalError> {
        let path = segment_path(&self.config.dir, segment);
        let mut file = File::open(&path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        let mut records = Vec::new();
        let mut pos = 0usize;
        let mut prev_hash = start_hash;
        let mut record_idx = 0u64;

        while pos + 4 <= buf.len() {
            let len =
                u32::from_le_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            if pos + 4 + len > buf.len() {
                break; // 截断尾部
            }
            let payload = &buf[pos + 4..pos + 4 + len];
            if payload.len() < 32 {
                return Err(WalError::Corrupt(format!(
                    "payload too short at segment {} record {}",
                    segment, record_idx
                )));
            }
            let recorded_prev = &payload[..32];
            if recorded_prev != prev_hash {
                return Err(WalError::HashChainBroken {
                    segment,
                    record: record_idx,
                    expected: hex_upper(&prev_hash),
                    actual: hex_upper(recorded_prev),
                });
            }
            let hash = hash_payload(payload);
            records.push(RawRecord {
                payload: payload.to_vec(),
                hash,
            });
            prev_hash = hash;
            record_idx += 1;
            pos += 4 + len;
        }

        Ok((records, prev_hash, pos as u64))
    }

    /// 遍历全部段，跨段串联哈希链，返回校验结果。
    pub fn replay_all(&self) -> Result<ReplayOutcome, WalError> {
        let segments = discover_segments(&self.config.dir);
        let mut records = Vec::new();
        let mut prev_hash = GENESIS_HASH;
        let mut last_segment: Option<u64> = None;
        let mut last_valid_len = 0u64;
        let mut last_records = 0u64;

        for &seq in &segments {
            let (raw, end_hash, valid_len) = self.read_segment(seq, prev_hash)?;
            prev_hash = end_hash;
            last_segment = Some(seq);
            last_valid_len = valid_len;
            last_records = raw.len() as u64;
            for r in raw {
                let event: WalEvent = serde_json::from_slice(&r.payload[32..])
                    .map_err(|e| WalError::Corrupt(format!("event deserialize: {}", e)))?;
                records.push(WalRecord {
                    segment: seq,
                    event,
                    hash: hex_upper(&r.hash),
                    hash_bytes: r.hash,
                });
            }
        }

        Ok(ReplayOutcome {
            records,
            last_segment,
            last_segment_valid_len: last_valid_len,
            last_segment_records: last_records,
        })
    }
}

struct RawRecord {
    payload: Vec<u8>,
    hash: [u8; 32],
}

#[cfg(test)]
mod tests {
    use super::*;
    use helix_mind_core::graph::{NodeContent, NodeType};

    fn temp_dir(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("helix_wal_{}_{}", name, std::process::id()))
    }

    fn event_node(text: &str) -> WalEvent {
        WalEvent::NodeWritten(Node {
            content: NodeContent::Text(text.into()),
            node_type: NodeType::L2,
            ..Default::default()
        })
    }

    #[test]
    fn append_replay_roundtrip_with_hash_chain() {
        let dir = temp_dir("full_roundtrip");
        let _ = std::fs::remove_dir_all(&dir);
        let cfg = WalConfig::new(&dir);
        let mut w = WalWriter::open(&cfg).unwrap();

        let ev1 = event_node("one");
        let ev2 = event_node("two");
        w.append_synced(&ev1).unwrap();
        w.append_synced(&ev2).unwrap();
        drop(w);

        let reader = WalReader::new(&cfg);
        let outcome = reader.replay_all().unwrap();
        assert_eq!(outcome.records.len(), 2);
        match &outcome.records[0].event {
            WalEvent::NodeWritten(n) => match &n.content {
                NodeContent::Text(t) => assert_eq!(t, "one"),
                _ => panic!("expected text content"),
            },
            _ => panic!("expected node"),
        }
        assert_eq!(outcome.records[1].hash.len(), 64); // 32 bytes hex upper
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn hash_chain_detects_tampering() {
        let dir = temp_dir("tamper");
        let _ = std::fs::remove_dir_all(&dir);
        let cfg = WalConfig::new(&dir);
        let mut w = WalWriter::open(&cfg).unwrap();
        w.append_synced(&event_node("alpha")).unwrap();
        w.append_synced(&event_node("beta")).unwrap();
        drop(w);

        // 篡改第一条记录载荷（模拟位腐坏/意外损坏），哈希链必须被破坏。
        let path = segment_path(&dir, 0);
        let mut buf = std::fs::read(&path).unwrap();
        buf[4 + 32] ^= 0x01; // 翻转 event_json 首字节
        std::fs::write(&path, &buf).unwrap();

        let reader = WalReader::new(&cfg);
        match reader.replay_all() {
            Err(WalError::HashChainBroken { .. }) => {}
            other => panic!("expected hash-chain break, got {:?}", other.map(|v| v.records.len())),
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn truncated_tail_ignored_in_replay() {
        let dir = temp_dir("tail");
        let _ = std::fs::remove_dir_all(&dir);
        let cfg = WalConfig::new(&dir);
        let mut w = WalWriter::open(&cfg).unwrap();
        w.append_synced(&event_node("a")).unwrap();
        w.append_synced(&event_node("b")).unwrap();
        drop(w);

        // 模拟崩溃：尾部追加不完整记录。
        {
            let mut f = OpenOptions::new()
                .append(true)
                .open(segment_path(&dir, 0))
                .unwrap();
            let fake = 500u32;
            f.write_all(&fake.to_le_bytes()).unwrap();
        }

        let reader = WalReader::new(&cfg);
        let outcome = reader.replay_all().unwrap();
        assert_eq!(outcome.records.len(), 2, "truncated tail ignored");
        // valid_len 不含截断尾（小于文件物理长度）。
        let physical = std::fs::metadata(segment_path(&dir, 0)).unwrap().len();
        assert!(outcome.last_segment_valid_len < physical);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn segment_rotation_keeps_chain_across_segments() {
        let dir = temp_dir("rotate");
        let _ = std::fs::remove_dir_all(&dir);
        let mut cfg = WalConfig::new(&dir);
        cfg.segment_size = 64; // 小段触发轮转
        let mut w = WalWriter::open(&cfg).unwrap();

        let mut last_append_seg = 0u64;
        for i in 0..10 {
            last_append_seg =
                w.append_synced(&event_node(&format!("payload-{}", i))).unwrap();
        }
        assert!(w.segments().len() >= 2, "rotation should create >1 segment");
        let cur_seg = w.current_segment();
        drop(w);

        let reader = WalReader::new(&cfg);
        let outcome = reader.replay_all().unwrap();
        assert_eq!(outcome.records.len(), 10, "all events survive rotation");
        assert!(outcome.records.iter().any(|r| r.segment > 0));
        // 最后一条记录位于最后一次 append 写入的段（非轮转后的空段）。
        assert_eq!(outcome.records.last().unwrap().segment, last_append_seg);
        assert_eq!(outcome.last_segment, Some(cur_seg));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn reopen_continues_after_last_segment() {
        let dir = temp_dir("reopen");
        let _ = std::fs::remove_dir_all(&dir);
        let cfg = WalConfig::new(&dir);
        {
            let mut w = WalWriter::open(&cfg).unwrap();
            w.append_synced(&event_node("first")).unwrap();
        }
        let mut w = WalWriter::open(&cfg).unwrap();
        w.append_synced(&event_node("second")).unwrap();
        drop(w);

        let reader = WalReader::new(&cfg);
        let outcome = reader.replay_all().unwrap();
        assert_eq!(outcome.records.len(), 2, "reopen continues the chain");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn reopen_truncates_crash_tail_before_appending() {
        let dir = temp_dir("reopen_tail");
        let _ = std::fs::remove_dir_all(&dir);
        let cfg = WalConfig::new(&dir);
        {
            let mut w = WalWriter::open(&cfg).unwrap();
            w.append_synced(&event_node("a")).unwrap();
        }
        // 模拟崩溃残留：追加截断尾。
        {
            let mut f = OpenOptions::new()
                .append(true)
                .open(segment_path(&dir, 0))
                .unwrap();
            let fake = 500u32;
            f.write_all(&fake.to_le_bytes()).unwrap();
        }
        // 续接：必须 truncate 截断尾后追加，replay 只得到完整记录。
        {
            let mut w = WalWriter::open(&cfg).unwrap();
            w.append_synced(&event_node("b")).unwrap();
        }
        let reader = WalReader::new(&cfg);
        let outcome = reader.replay_all().unwrap();
        assert_eq!(outcome.records.len(), 2, "crash tail truncated, chain valid");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
