use thiserror::Error;

#[derive(Debug, Error)]
pub enum MindError {
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Retrieval error: {0}")]
    Retrieval(String),
    
    #[error("Metabolism error: {0}")]
    Metabolism(String),
    
    #[error("Federation error: {0}")]
    Federation(String),

    #[error("HTTP request error: {0}")]
    Http(String),
    
    #[error("Lifecycle error: {0}")]
    Lifecycle(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Cycle detected in graph")]
    CycleDetected,
    
    #[error("Energy budget exhausted")]
    EnergyExhausted,
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("UUID error: {0}")]
    Uuid(#[from] uuid::Error),

    #[error("Database error: {0}")]
    Database(String),
}

// 注意：这里不再实现 From<rusqlite::Error> 和 From<r2d2::Error>
// 这些转换将在 helix-mind-storage 层通过 .map_err() 手动完成