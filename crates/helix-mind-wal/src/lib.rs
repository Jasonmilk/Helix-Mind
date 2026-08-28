//! 领域 WAL 最小原型（P4.5 架构审查点产出 ②，ADR-0015）。
//!
//! 验证目标（正确架构的验证步骤，非 MVP 妥协）：
//! 1. `append`（长度前缀记录，append-only）可行；
//! 2. `fsync`（sync_all）持久化语义可靠；
//! 3. 单段顺序读取完整（read_all）；
//! 4. 崩溃恢复：部分写入（未刷盘的截断尾部）被安全忽略，完整记录保留；
//! 5. 延迟量级：单条 fsync 在 SSD 上目标 <10ms p99（审查③ F8 修正）。
//!
//! 本原型只做上述最小验证；段轮转、BLAKE3 哈希链、异步投影、replay 属 P5。

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;

/// WAL 原型错误。
#[derive(Debug, Error)]
pub enum WalError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// 一条 WAL 记录（长度前缀格式 `[len: u32 LE][data]`）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalRecord {
    /// 记录在段内的起始偏移。
    pub offset: u64,
    /// 载荷数据。
    pub data: Vec<u8>,
}

/// 单段追加日志文件。
pub struct WalSegment {
    path: PathBuf,
    file: File,
    /// 当前段字节长度（已写完整记录数 × 记录长度之和）。
    length: u64,
}

impl WalSegment {
    /// 创建新段（失败若文件已存在）。
    pub fn create(path: impl AsRef<Path>) -> Result<Self, WalError> {
        let path = path.as_ref().to_path_buf();
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .read(true)
            .open(&path)?;
        Ok(Self {
            path,
            file,
            length: 0,
        })
    }

    /// 打开已存在段（追加模式，长度从元数据恢复）。
    pub fn open(path: impl AsRef<Path>) -> Result<Self, WalError> {
        let path = path.as_ref().to_path_buf();
        let file = OpenOptions::new().append(true).read(true).open(&path)?;
        let length = file.metadata()?.len();
        Ok(Self {
            path,
            file,
            length,
        })
    }

    /// 追加一条记录（不 fsync）。返回记录起始偏移。
    pub fn append(&mut self, data: &[u8]) -> Result<u64, WalError> {
        let offset = self.length;
        let len = data.len() as u32;
        self.file.write_all(&len.to_le_bytes())?;
        self.file.write_all(data)?;
        self.length = offset + 4 + len as u64;
        Ok(offset)
    }

    /// 追加并立即 fsync（强一致路径）。
    pub fn append_synced(&mut self, data: &[u8]) -> Result<u64, WalError> {
        let offset = self.append(data)?;
        self.file.sync_all()?;
        Ok(offset)
    }

    /// fsync（批量策略下周期调用）。
    pub fn sync(&mut self) -> Result<(), WalError> {
        self.file.sync_all()?;
        Ok(())
    }

    /// 当前段字节长度。
    pub fn length(&self) -> u64 {
        self.length
    }

    /// 段文件路径。
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// 顺序读取全部完整记录。
    ///
    /// 崩溃恢复语义：若尾部存在不完整记录（部分写入未刷盘），
    /// 在截断处停止，忽略不完整尾部，仅返回完整记录（"已持久化的才承认"）。
    pub fn read_all(&mut self) -> Result<Vec<WalRecord>, WalError> {
        self.file.seek(SeekFrom::Start(0))?;
        let mut buf = Vec::new();
        self.file.read_to_end(&mut buf)?;

        let mut records = Vec::new();
        let mut pos = 0usize;
        while pos + 4 <= buf.len() {
            let len =
                u32::from_le_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            if pos + 4 + len > buf.len() {
                // 截断尾部：忽略不完整记录，停止（崩溃恢复语义）。
                break;
            }
            let data = buf[pos + 4..pos + 4 + len].to_vec();
            records.push(WalRecord {
                offset: pos as u64,
                data,
            });
            pos += 4 + len;
        }
        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("helix_wal_proto_{}_{}", name, std::process::id()))
    }

    #[test]
    fn append_read_roundtrip() {
        let path = temp_path("roundtrip");
        let _ = std::fs::remove_file(&path);
        let mut seg = WalSegment::create(&path).unwrap();
        let records = vec![
            b"alpha".to_vec(),
            b"".to_vec(),
            b"omega record payload".to_vec(),
        ];
        for r in &records {
            seg.append(r).unwrap();
        }
        let read = seg.read_all().unwrap();
        assert_eq!(read.len(), 3);
        for (i, r) in read.iter().enumerate() {
            assert_eq!(r.data, records[i]);
            assert_eq!(r.offset, {
                // 手工验证偏移：记录 0 偏移 0；后续为前序长度前缀+载荷之和
                let mut off = 0u64;
                for j in 0..i {
                    off += 4 + records[j].len() as u64;
                }
                off
            });
        }
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn fsync_persists_across_reopen() {
        let path = temp_path("persist");
        let _ = std::fs::remove_file(&path);
        {
            let mut seg = WalSegment::create(&path).unwrap();
            seg.append_synced(b"persisted-payload").unwrap();
        }
        // 重开段，数据必须仍在（fsync 持久化语义）。
        let mut seg = WalSegment::open(&path).unwrap();
        assert_eq!(seg.length(), 4 + "persisted-payload".len() as u64);
        let read = seg.read_all().unwrap();
        assert_eq!(read.len(), 1);
        assert_eq!(read[0].data, b"persisted-payload");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn truncated_tail_recovery_ignores_partial_write() {
        let path = temp_path("truncate");
        let _ = std::fs::remove_file(&path);
        {
            let mut seg = WalSegment::create(&path).unwrap();
            seg.append_synced(b"complete-1").unwrap();
            seg.append_synced(b"complete-2").unwrap();
        }
        // 模拟崩溃：在文件尾部追加一个不完整的记录（只写了长度前缀，数据缺失）。
        {
            let mut f = OpenOptions::new().append(true).open(&path).unwrap();
            let fake_len = 100u32;
            f.write_all(&fake_len.to_le_bytes()).unwrap();
        }
        let mut seg = WalSegment::open(&path).unwrap();
        let read = seg.read_all().unwrap();
        // 只承认完整记录：2 条，截断尾部被忽略。
        assert_eq!(read.len(), 2);
        assert_eq!(read[0].data, b"complete-1");
        assert_eq!(read[1].data, b"complete-2");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn single_fsync_latency_orders_of_magnitude() {
        // 验证 fsync 延迟量级（目标 <10ms p99 on SSD，审查③ F8）。
        // CI 环境波动大，用宽松上限防 flaky；实际值打印供观察。
        let path = temp_path("latency");
        let _ = std::fs::remove_file(&path);
        let mut seg = WalSegment::create(&path).unwrap();
        let payload = vec![0u8; 256];

        let n = 50;
        let mut latencies = Vec::with_capacity(n);
        for _ in 0..n {
            let t = Instant::now();
            seg.append_synced(&payload).unwrap();
            latencies.push(t.elapsed());
        }
        latencies.sort();
        let p50 = latencies[n / 2];
        let p99 = latencies[(n as f64 * 0.99) as usize];
        println!(
            "WAL proto: single-fsync p50={:?} p99={:?} (target <10ms p99 on SSD)",
            p50, p99
        );
        // 宽松断言防 CI 抖动；目标量级见打印与 ADR-0015。
        assert!(p99 < Duration::from_millis(500), "p99={:?}", p99);
        let _ = std::fs::remove_file(&path);
    }
}
