"""Knowledge DAG management with Parquet snapshots and JSON patches."""

import json
import pyarrow as pa
import pyarrow.parquet as pq
from datetime import datetime
from pathlib import Path
from typing import Optional, List, Dict, Any

from mind.storage.duckdb_store import DuckDBStore


class KnowledgeDAG:
    """Knowledge DAG读写 with Snapshot + Patch versioning."""

    def __init__(self, store: DuckDBStore, data_dir: Path):
        """Initialize KnowledgeDAG.

        Args:
            store: DuckDB storage instance.
            data_dir: Base data directory.
        """
        self.store = store
        self.snapshots_dir = data_dir / "knowledge_base" / "snapshots"
        self.patches_dir = data_dir / "knowledge_base" / "patches"
        self.snapshots_dir.mkdir(parents=True, exist_ok=True)
        self.patches_dir.mkdir(parents=True, exist_ok=True)

    def write_node(self, node: Dict[str, Any]) -> bool:
        """Write a node using Patch mode.

        Args:
            node: Node data dictionary.

        Returns:
            True if successful, False if version conflict.
        """
        # Optimistic locking: check version
        current = self.store.conn.execute(
            "SELECT version FROM nodes WHERE id = ?", [node["id"]]
        ).fetchone()

        expected_version = node.get("expected_version")
        if current and expected_version is not None and expected_version != current[0]:
            return False

        # Write Patch file (incremental update)
        patch_file = self.patches_dir / f"{datetime.now().isoformat()}_patch.json"
        with open(patch_file, "w") as f:
            json.dump({"node": node, "ts": datetime.now().isoformat()}, f)

        # Update DuckDB table
        new_version = (current[0] + 1) if current else 1
        self.store.conn.execute("""
            INSERT OR REPLACE INTO nodes 
            (id, type, layer, title, summary, full_content, 
             confidence, source, version, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
        """, [
            node["id"],
            node["type"],
            node["layer"],
            node["title"],
            node["summary"],
            node["full_content"],
            node["confidence"],
            node["source"],
            new_version,
        ])

        return True

    def create_snapshot(self) -> str:
        """Create a Parquet snapshot of the knowledge base.

        Returns:
            Snapshot ID (timestamp string).
        """
        snapshot_id = datetime.now().strftime("%Y%m%d_%H%M%S")
        snapshot_dir = self.snapshots_dir / snapshot_id
        snapshot_dir.mkdir(parents=True, exist_ok=True)

        # Export nodes and edges to Parquet
        nodes_df = self.store.conn.execute("SELECT * FROM nodes").fetchdf()
        edges_df = self.store.conn.execute("SELECT * FROM edges").fetchdf()

        nodes_df.to_parquet(snapshot_dir / "nodes.parquet")
        edges_df.to_parquet(snapshot_dir / "edges.parquet")

        # Update 'latest' symlink
        latest_link = self.snapshots_dir / "latest"
        if latest_link.exists() or latest_link.is_symlink():
            latest_link.unlink()
        latest_link.symlink_to(snapshot_dir, target_is_directory=True)

        return snapshot_id

    def get_node(self, node_id: str, mode: str = "summary") -> Optional[Dict[str, Any]]:
        """Get a node by ID.

        Args:
            node_id: Node ID.
            mode: 'summary' or 'full'.

        Returns:
            Node data dictionary or None if not found.
        """
        if mode == "summary":
            result = self.store.conn.execute("""
                SELECT id, type, layer, title, summary, confidence, 
                       source, version, created_at, updated_at
                FROM nodes WHERE id = ? AND is_active = true
            """, [node_id]).fetchone()

            if result:
                return {
                    "id": result[0],
                    "type": result[1],
                    "layer": result[2],
                    "title": result[3],
                    "summary": result[4],
                    "confidence": result[5],
                    "source": result[6],
                    "version": result[7],
                    "created_at": result[8].isoformat() if result[8] else None,
                    "updated_at": result[9].isoformat() if result[9] else None,
                }
        else:  # full mode
            result = self.store.conn.execute("""
                SELECT * FROM nodes WHERE id = ? AND is_active = true
            """, [node_id]).fetchone()

            if result:
                columns = [desc[0] for desc in self.store.conn.description]
                return dict(zip(columns, result))

        return None

    def delete_node(self, node_id: str) -> bool:
        """Soft delete a node.

        Args:
            node_id: Node ID.

        Returns:
            True if node was deleted, False if not found.
        """
        result = self.store.conn.execute(
            "UPDATE nodes SET is_active = false, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
            [node_id],
        )
        return result.rowcount > 0
