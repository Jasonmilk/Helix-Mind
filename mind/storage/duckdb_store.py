"""DuckDB storage engine for Helix-Mind."""

import duckdb
from pathlib import Path
from typing import Optional


class DuckDBStore:
    """DuckDB connection and table management."""

    def __init__(self, db_path: Optional[Path] = None):
        """Initialize DuckDB connection.

        Args:
            db_path: Path to the DuckDB database file.
        """
        if db_path is None:
            db_path = Path("./data/duckdb/helix_mind.db")

        self.db_path = db_path
        self.db_path.parent.mkdir(parents=True, exist_ok=True)
        self.conn = duckdb.connect(str(self.db_path))
        self._init_tables()
        self._load_extensions()

    def _load_extensions(self) -> None:
        """Load DuckDB extensions."""
        # Load full-text search extension
        self.conn.execute("INSTALL fts; LOAD fts;")
        # VSS extension is optional, vector search uses built-in functions

    def _init_tables(self) -> None:
        """Initialize database tables."""
        # Nodes table
        self.conn.execute("""
            CREATE TABLE IF NOT EXISTS nodes (
                id VARCHAR PRIMARY KEY,
                type VARCHAR,
                layer VARCHAR,
                title VARCHAR,
                summary VARCHAR,
                full_content TEXT,
                confidence FLOAT,
                source VARCHAR,
                version INTEGER DEFAULT 1,
                is_active BOOLEAN DEFAULT true,
                embedding FLOAT[384],
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        """)

        # Edges table
        self.conn.execute("""
            CREATE TABLE IF NOT EXISTS edges (
                source_id VARCHAR,
                target_id VARCHAR,
                rel_type VARCHAR,
                valid_from TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                valid_until TIMESTAMP,
                properties JSON,
                PRIMARY KEY (source_id, target_id, rel_type)
            )
        """)

        # Create full-text search index on title and summary
        self.conn.execute("""
            PRAGMA create_fts_index(
                'nodes',
                'id',
                'title', 'summary'
            )
        """)

    def close(self) -> None:
        """Close the database connection."""
        self.conn.close()
