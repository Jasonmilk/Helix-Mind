//! Parquet 列式存储（P6-1, F7 名实相符）。
//!
//! 模块名 `parquet_store` 现与实现一致：真 Parquet 列式（Apache Arrow 官方 Rust 生态，
//! `parquet` + `arrow-array`/`arrow-schema`，不引入 `arrow` 以避免 arrow-arith/chrono 冲突）。
//!
//! 列约束（docs/spec/parquet-projection.md）：
//! - **必须投影（12 列）**：id, node_type, content, heat, dominance, utility,
//!   created_at, last_accessed_at, access_count, sensitivity, phase_state, subject_dependency。
//! - **可延迟列**（meta.concentration/tension, notes, derived_from, attribution_ledger,
//!   abstract_provenance, source）：P6 暂不投影，加载时回落默认值（诚实声明）。

use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use arrow_array::{Array, ArrayRef, Float64Array, Int64Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema};
use helix_mind_core::error::MindError;
use helix_mind_core::graph::Node;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::arrow::arrow_writer::ArrowWriter;
use parquet::file::properties::WriterProperties;

use crate::codec::{
    node_type_str, phase_state_str, sensitivity_str, str_to_node_type, str_to_phase_state,
    str_to_sensitivity, str_to_subject_dependency, subject_dependency_str,
};

const COL_ID: &str = "id";
const COL_NODE_TYPE: &str = "node_type";
const COL_CONTENT: &str = "content";
const COL_HEAT: &str = "heat";
const COL_DOMINANCE: &str = "dominance";
const COL_UTILITY: &str = "utility";
const COL_CREATED_AT: &str = "created_at";
const COL_LAST_ACCESSED_AT: &str = "last_accessed_at";
const COL_ACCESS_COUNT: &str = "access_count";
const COL_SENSITIVITY: &str = "sensitivity";
const COL_PHASE_STATE: &str = "phase_state";
const COL_SUBJECT_DEPENDENCY: &str = "subject_dependency";

/// 深冷归档 / 晶体投影：列式 schema（必须投影 12 列）。
fn schema() -> Schema {
    Schema::new(vec![
        Field::new(COL_ID, DataType::Utf8, false),
        Field::new(COL_NODE_TYPE, DataType::Utf8, false),
        Field::new(COL_CONTENT, DataType::Utf8, false),
        Field::new(COL_HEAT, DataType::Float64, false),
        Field::new(COL_DOMINANCE, DataType::Float64, false),
        Field::new(COL_UTILITY, DataType::Float64, false),
        Field::new(COL_CREATED_AT, DataType::Int64, false),
        Field::new(COL_LAST_ACCESSED_AT, DataType::Int64, false),
        Field::new(COL_ACCESS_COUNT, DataType::Int64, false),
        Field::new(COL_SENSITIVITY, DataType::Utf8, false),
        Field::new(COL_PHASE_STATE, DataType::Utf8, false),
        Field::new(COL_SUBJECT_DEPENDENCY, DataType::Utf8, false),
    ])
}

pub struct ParquetStore;

impl ParquetStore {
    /// 将节点数组写为 Parquet 列式文件（必须投影列）。
    pub fn save_nodes<P: AsRef<Path>>(path: P, nodes: &[Node]) -> Result<(), MindError> {
        let schema = Arc::new(schema());

        let ids = StringArray::from(nodes.iter().map(|n| n.id.to_string()).collect::<Vec<_>>());
        let node_types =
            StringArray::from(nodes.iter().map(|n| node_type_str(&n.node_type).to_string()).collect::<Vec<_>>());
        let contents = nodes
            .iter()
            .map(|n| serde_json::to_string(&n.content).map_err(|e| MindError::Storage(e.to_string())))
            .collect::<Result<Vec<_>, _>>()?;
        let contents = StringArray::from(contents);
        let heats = Float64Array::from(nodes.iter().map(|n| n.heat).collect::<Vec<_>>());
        let dominances = Float64Array::from(nodes.iter().map(|n| n.dominance).collect::<Vec<_>>());
        let utilities = Float64Array::from(nodes.iter().map(|n| n.utility).collect::<Vec<_>>());
        let created_ats =
            Int64Array::from(nodes.iter().map(|n| n.created_at.timestamp()).collect::<Vec<_>>());
        let last_accessed_ats = Int64Array::from(
            nodes.iter().map(|n| n.last_accessed_at.timestamp()).collect::<Vec<_>>(),
        );
        let access_counts =
            Int64Array::from(nodes.iter().map(|n| n.access_count as i64).collect::<Vec<_>>());
        let sensitivities = StringArray::from(
            nodes
                .iter()
                .map(|n| n.sensitivity.as_ref().map(|s| sensitivity_str(s).to_string()).unwrap_or_default())
                .collect::<Vec<_>>(),
        );
        let phase_states = StringArray::from(
            nodes.iter().map(|n| phase_state_str(&n.phase_state).to_string()).collect::<Vec<_>>(),
        );
        let subject_dependencies = StringArray::from(
            nodes
                .iter()
                .map(|n| subject_dependency_str(&n.subject_dependency).to_string())
                .collect::<Vec<_>>(),
        );

        let columns: Vec<ArrayRef> = vec![
            Arc::new(ids),
            Arc::new(node_types),
            Arc::new(contents),
            Arc::new(heats),
            Arc::new(dominances),
            Arc::new(utilities),
            Arc::new(created_ats),
            Arc::new(last_accessed_ats),
            Arc::new(access_counts),
            Arc::new(sensitivities),
            Arc::new(phase_states),
            Arc::new(subject_dependencies),
        ];

        let batch = RecordBatch::try_new(schema.clone(), columns)
            .map_err(|e| MindError::Storage(e.to_string()))?;

        let file = File::create(path.as_ref()).map_err(|e| MindError::Io(e))?;
        let props = WriterProperties::builder().build();
        let mut writer =
            ArrowWriter::try_new(file, schema, Some(props)).map_err(|e| MindError::Storage(e.to_string()))?;
        writer.write(&batch).map_err(|e| MindError::Storage(e.to_string()))?;
        writer.close().map_err(|e| MindError::Storage(e.to_string()))?;

        Ok(())
    }

    /// 从 Parquet 列式文件读取节点数组（可延迟列回落默认值）。
    pub fn load_nodes<P: AsRef<Path>>(path: P) -> Result<Vec<Node>, MindError> {
        let file = File::open(path.as_ref()).map_err(|e| MindError::Io(e))?;
        let builder = ParquetRecordBatchReaderBuilder::try_new(file)
            .map_err(|e| MindError::Storage(e.to_string()))?;
        let reader = builder
            .build()
            .map_err(|e| MindError::Storage(e.to_string()))?;

        let mut nodes = Vec::new();
        for batch_res in reader {
            let batch = batch_res.map_err(|e| MindError::Storage(e.to_string()))?;
            let n = batch.num_rows();

            let ids = col_string(&batch, 0)?;
            let node_types = col_string(&batch, 1)?;
            let contents = col_string(&batch, 2)?;
            let heats = col_f64(&batch, 3)?;
            let dominances = col_f64(&batch, 4)?;
            let utilities = col_f64(&batch, 5)?;
            let created_ats = col_i64(&batch, 6)?;
            let last_accessed_ats = col_i64(&batch, 7)?;
            let access_counts = col_i64(&batch, 8)?;
            let sensitivities = col_string(&batch, 9)?;
            let phase_states = col_string(&batch, 10)?;
            let subject_dependencies = col_string(&batch, 11)?;

            for i in 0..n {
                let content: helix_mind_core::graph::NodeContent = serde_json::from_str(&contents[i])
                    .map_err(|e| MindError::Storage(e.to_string()))?;
                let mut node = Node {
                    id: uuid::Uuid::parse_str(&ids[i]).map_err(|e| MindError::Uuid(e))?,
                    node_type: str_to_node_type(&node_types[i]),
                    content,
                    heat: heats[i],
                    dominance: dominances[i],
                    utility: utilities[i],
                    access_count: access_counts[i] as u64,
                    phase_state: str_to_phase_state(&phase_states[i]),
                    subject_dependency: str_to_subject_dependency(&subject_dependencies[i]),
                    ..Default::default()
                };
                node.created_at = parse_epoch(created_ats[i]);
                node.last_accessed_at = parse_epoch(last_accessed_ats[i]);
                node.sensitivity = if sensitivities[i].is_empty() {
                    None
                } else {
                    Some(str_to_sensitivity(&sensitivities[i]))
                };
                nodes.push(node);
            }
        }

        Ok(nodes)
    }
}

fn col_string(batch: &RecordBatch, idx: usize) -> Result<Vec<String>, MindError> {
    batch
        .column(idx)
        .as_any()
        .downcast_ref::<StringArray>()
        .map(|a| (0..a.len()).map(|i| a.value(i).to_string()).collect())
        .ok_or_else(|| MindError::Storage("parquet column type mismatch (expected Utf8)".into()))
}

fn col_f64(batch: &RecordBatch, idx: usize) -> Result<Vec<f64>, MindError> {
    batch
        .column(idx)
        .as_any()
        .downcast_ref::<Float64Array>()
        .map(|a| (0..a.len()).map(|i| a.value(i)).collect())
        .ok_or_else(|| MindError::Storage("parquet column type mismatch (expected Float64)".into()))
}

fn col_i64(batch: &RecordBatch, idx: usize) -> Result<Vec<i64>, MindError> {
    batch
        .column(idx)
        .as_any()
        .downcast_ref::<Int64Array>()
        .map(|a| (0..a.len()).map(|i| a.value(i)).collect())
        .ok_or_else(|| MindError::Storage("parquet column type mismatch (expected Int64)".into()))
}

fn parse_epoch(ts: i64) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::from_timestamp(ts, 0).unwrap_or_else(chrono::Utc::now)
}

#[cfg(test)]
mod tests {
    use super::*;
    use helix_mind_core::graph::{NodeContent, NodeType, PhaseState, Sensitivity, SubjectDependency};

    #[test]
    fn save_load_roundtrip_true_parquet() {
        let path = std::env::temp_dir().join(format!("helix_pq_{}.parquet", uuid::Uuid::new_v4()));
        let nodes = vec![
            Node {
                content: NodeContent::Text("认知相态的河流".into()),
                node_type: NodeType::L2,
                heat: 0.9,
                dominance: 0.7,
                utility: 0.6,
                access_count: 3,
                phase_state: PhaseState::Crystal,
                subject_dependency: SubjectDependency::Low,
                sensitivity: Some(Sensitivity::Public),
                ..Default::default()
            },
            Node {
                content: NodeContent::Text("第二节点".into()),
                node_type: NodeType::L3,
                heat: 0.4,
                dominance: 0.1,
                utility: 0.2,
                access_count: 1,
                phase_state: PhaseState::Liquid,
                subject_dependency: SubjectDependency::High,
                sensitivity: None,
                ..Default::default()
            },
        ];

        ParquetStore::save_nodes(&path, &nodes).unwrap();

        // 文件必须是真 Parquet（magic bytes: PAR1 头尾），不再是 JSON。
        let bytes = std::fs::read(&path).unwrap();
        assert_eq!(&bytes[0..4], b"PAR1", "must be real parquet file");
        assert_eq!(&bytes[bytes.len() - 4..], b"PAR1", "must be real parquet file");

        let loaded = ParquetStore::load_nodes(&path).unwrap();
        assert_eq!(loaded.len(), 2);
        match &loaded[0].content {
            NodeContent::Text(t) => assert_eq!(t, "认知相态的河流"),
            _ => panic!("expected text"),
        }
        assert_eq!(loaded[0].node_type, NodeType::L2);
        assert_eq!(loaded[0].heat, 0.9);
        assert_eq!(loaded[0].phase_state, PhaseState::Crystal);
        assert_eq!(loaded[0].subject_dependency, SubjectDependency::Low);
        assert_eq!(loaded[0].sensitivity, Some(Sensitivity::Public));
        assert_eq!(loaded[0].access_count, 3);
        assert_eq!(loaded[1].node_type, NodeType::L3);
        assert_eq!(loaded[1].phase_state, PhaseState::Liquid);
        assert_eq!(loaded[1].sensitivity, None);

        let _ = std::fs::remove_file(&path);
    }
}
