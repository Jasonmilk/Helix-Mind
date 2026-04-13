"""Snapshot API routes for Helix-Mind."""

from fastapi import APIRouter, HTTPException
from typing import Optional
from datetime import datetime

from mind.storage.duckdb_store import DuckDBStore
from mind.storage.knowledge_dag import KnowledgeDAG
from pathlib import Path

router = APIRouter(prefix="/v1/mind", tags=["snapshot"])

# Global store (initialized at startup)
_store: Optional[DuckDBStore] = None


def init_snapshot_api(store: DuckDBStore) -> None:
    """Initialize snapshot API dependencies.

    Args:
        store: DuckDB storage instance.
    """
    global _store
    _store = store


@router.post("/snapshot")
async def create_snapshot() -> dict:
    """Manually trigger a knowledge base snapshot.

    Returns:
        Snapshot creation result with ID and counts.

    Raises:
        503: Storage not initialized.
    """
    if _store is None:
        raise HTTPException(status_code=503, detail="Storage not initialized")

    dag = KnowledgeDAG(_store, Path("./data"))
    snapshot_id = dag.create_snapshot()

    # Get counts
    nodes_count = _store.conn.execute("SELECT COUNT(*) FROM nodes").fetchone()[0]
    edges_count = _store.conn.execute("SELECT COUNT(*) FROM edges").fetchone()[0]

    return {
        "snapshot_id": snapshot_id,
        "created_at": datetime.now().isoformat(),
        "nodes_count": nodes_count,
        "edges_count": edges_count,
        "status": "success",
    }
