"""Write API routes for Helix-Mind."""

from fastapi import APIRouter, HTTPException
from typing import Optional
from datetime import datetime

from mind.core.models import NodeWriteRequest
from mind.storage.duckdb_store import DuckDBStore
from mind.storage.knowledge_dag import KnowledgeDAG
from pathlib import Path

router = APIRouter(prefix="/v1/mind", tags=["write"])

# Global store (initialized at startup)
_store: Optional[DuckDBStore] = None


def init_write_api(store: DuckDBStore) -> None:
    """Initialize write API dependencies.

    Args:
        store: DuckDB storage instance.
    """
    global _store
    _store = store


@router.post("/nodes")
async def write_node(node: NodeWriteRequest) -> dict:
    """Write a node to knowledge DAG (Patch mode).

    Args:
        node: Node data.

    Returns:
        Write result with version info.

    Raises:
        409: Version conflict.
        503: Storage not initialized.
    """
    if _store is None:
        raise HTTPException(status_code=503, detail="Storage not initialized")

    dag = KnowledgeDAG(_store, Path("./data"))

    node_data = node.model_dump()
    success = dag.write_node(node_data)

    if not success:
        # Fetch current version for conflict response
        current = _store.conn.execute(
            "SELECT version FROM nodes WHERE id = ?", [node.id]
        ).fetchone()
        raise HTTPException(
            status_code=409,
            detail={
                "error": "version_conflict",
                "node_id": node.id,
                "expected": node.expected_version,
                "actual": current[0] if current else None,
            },
        )

    # Get updated version
    updated = _store.conn.execute(
        "SELECT version, updated_at FROM nodes WHERE id = ?", [node.id]
    ).fetchone()

    return {
        "id": node.id,
        "version": updated[0],
        "updated_at": updated[1].isoformat() if updated[1] else None,
        "status": "success",
    }
