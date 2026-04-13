"""Search API routes for Helix-Mind."""

from fastapi import APIRouter, Query, HTTPException
from typing import Optional, List

from mind.core.models import NodeSummary, SearchResponse
from mind.index.hybrid_retriever import HybridRetriever
from mind.storage.duckdb_store import DuckDBStore

router = APIRouter(prefix="/v1/mind", tags=["search"])

# Global store and retriever (initialized at startup)
_store: Optional[DuckDBStore] = None
_retriever: Optional[HybridRetriever] = None


def init_search_api(store: DuckDBStore, retriever: HybridRetriever) -> None:
    """Initialize search API dependencies.

    Args:
        store: DuckDB storage instance.
        retriever: Hybrid retriever instance.
    """
    global _store, _retriever
    _store = store
    _retriever = retriever


@router.get("/search", response_model=SearchResponse)
async def search_nodes(
    query: str = Query(..., description="Search keyword"),
    limit: int = Query(5, ge=1, le=20, description="Maximum results"),
    labels: Optional[List[str]] = Query(None, description="Optional label filters"),
) -> SearchResponse:
    """Hybrid search returning node summaries.

    Args:
        query: Search query string.
        limit: Maximum number of results (1-20).
        labels: Optional label filters (not yet implemented).

    Returns:
        SearchResponse with list of node summaries.
    """
    if _retriever is None:
        raise HTTPException(status_code=503, detail="Search service not initialized")

    nodes = _retriever.search(query, top_k=limit)
    return SearchResponse(nodes=nodes, total=len(nodes))


@router.get("/nodes/{node_id}")
async def get_node(
    node_id: str,
    mode: str = Query("summary", regex="^(summary|full)$"),
) -> dict:
    """Get a specific node by ID.

    Args:
        node_id: Node identifier.
        mode: Response mode ('summary' or 'full').

    Returns:
        Node data dictionary.

    Raises:
        404: If node not found.
    """
    if _store is None:
        raise HTTPException(status_code=503, detail="Storage not initialized")

    from mind.storage.knowledge_dag import KnowledgeDAG
    from pathlib import Path

    dag = KnowledgeDAG(_store, Path("./data"))
    node = dag.get_node(node_id, mode=mode)

    if node is None:
        raise HTTPException(status_code=404, detail=f"Node not found: {node_id}")

    return node
