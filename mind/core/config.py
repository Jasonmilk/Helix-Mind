"""Configuration management for Helix-Mind."""

from pydantic_settings import BaseSettings
from pydantic import Field
from pathlib import Path


class MindSettings(BaseSettings):
    """Helix-Mind configuration settings."""

    # Server
    host: str = Field("0.0.0.0", env="MIND_HOST")
    port: int = Field(8020, env="MIND_PORT")

    # Data directories
    data_dir: Path = Field(Path("./data"), env="MIND_DATA_DIR")
    duckdb_path: Path = Field(
        Path("./data/duckdb/helix_mind.db"), env="MIND_DUCKDB_PATH"
    )

    # Storage paths
    snapshots_dir: Path = Field(
        Path("./data/knowledge_base/snapshots"), env="MIND_SNAPSHOTS_DIR"
    )
    patches_dir: Path = Field(
        Path("./data/knowledge_base/patches"), env="MIND_PATCHES_DIR"
    )
    sessions_dir: Path = Field(
        Path("./data/memory_dag/sessions"), env="MIND_SESSIONS_DIR"
    )

    # Embedding model
    embedding_model: str = Field("BAAI/bge-small-en", env="MIND_EMBEDDING_MODEL")

    # Index settings
    fts_limit: int = Field(5, env="MIND_FTS_LIMIT")
    vector_limit: int = Field(10, env="MIND_VECTOR_LIMIT")
    graph_max_depth: int = Field(3, env="MIND_GRAPH_MAX_DEPTH")

    # RRF settings
    rrf_k: int = Field(60, env="MIND_RRF_K")
    top_k: int = Field(5, env="MIND_TOP_K")

    # Wiki adapter cache
    wiki_cache_dir: Path = Field(
        Path("./data/wiki_cache"), env="MIND_WIKI_CACHE_DIR"
    )

    # Logging
    log_level: str = Field("INFO", env="MIND_LOG_LEVEL")

    class Config:
        """Pydantic config."""

        env_file = ".env"
        env_file_encoding = "utf-8"


def load_config() -> MindSettings:
    """Load configuration from environment variables."""
    return MindSettings()
