"""Hybrid retrieval with RRF (Reciprocal Rank Fusion) ranking."""

from typing import List, Dict, Any

from mind.index.fts_index import FTSIndex
from mind.index.vector_index import VectorIndex
from mind.index.graph_traverse import GraphTraverse
from mind.storage.duckdb_store import DuckDBStore


class HybridRetriever:
    """Three-layer hybrid retrieval with RRF fusion."""

    def __init__(
        self,
        store: DuckDBStore,
        rrf_k: int = 60,
        top_k: int = 5,
    ):
        """Initialize HybridRetriever.

        Args:
            store: DuckDB storage instance.
            rrf_k: RRF constant parameter.
            top_k: Number of results to return.
        """
        self.store = store
        self.rrf_k = rrf_k
        self.top_k = top_k

        self.fts = FTSIndex(store)
        self.vector = VectorIndex(store)
        self.graph = GraphTraverse(store)

    def _fetch_node_summary(self, node_id: str) -> Dict[str, Any]:
        """Fetch node summary by ID.

        Args:
            node_id: Node ID.

        Returns:
            Node summary dictionary.
        """
        result = self.store.conn.execute("""
            SELECT id, title, summary, confidence
            FROM nodes
            WHERE id = ? AND is_active = true
        """, [node_id]).fetchone()

        if result:
            return {
                "id": result[0],
                "title": result[1],
                "summary": result[2],
                "confidence": result[3],
            }
        return {}

    def search(
        self,
        query: str,
        top_k: int = None,
        enable_graph: bool = False,
    ) -> List[Dict[str, Any]]:
        """Perform hybrid search with RRF fusion.

        Args:
            query: Search query string.
            top_k: Number of results to return.
            enable_graph: Whether to include graph-based results.

        Returns:
            List of merged and ranked node summaries.
        """
        if top_k is None:
            top_k = self.top_k

        # L1: Keyword search (BM25)
        fts_results = self.fts.search(query, limit=10)
        fts_ranked = {r["id"]: (r, i) for i, r in enumerate(fts_results)}

        # L2: Vector search (only if L1 results are insufficient or low score)
        vec_results = {}
        if not fts_ranked or max(
            (r["score"] for r in fts_ranked.values()), default=0
        ) < 0.3:
            vector_results = self.vector.search(query, limit=10)
            vec_results = {r["id"]: (r, i) for i, r in enumerate(vector_results)}

        # RRF fusion
        scores: Dict[str, float] = {}

        for node_id, (_, rank) in fts_ranked.items():
            scores[node_id] = scores.get(node_id, 0) + 1 / (self.rrf_k + rank)

        for node_id, (_, rank) in vec_results.items():
            scores[node_id] = scores.get(node_id, 0) + 1 / (self.rrf_k + rank)

        # Sort by RRF score and return top_k
        sorted_ids = sorted(scores.keys(), key=lambda x: scores[x], reverse=True)[
            :top_k
        ]

        return [self._fetch_node_summary(nid) for nid in sorted_ids if nid]

    def search_with_evidence(
        self,
        query: str,
        top_k: int = None,
        max_depth: int = 3,
    ) -> Dict[str, Any]:
        """Search with evidence chain tracing.

        Args:
            query: Search query string.
            top_k: Number of results to return.
            max_depth: Maximum depth for evidence tracing.

        Returns:
            Dictionary with results and evidence chains.
        """
        results = self.search(query, top_k=top_k)

        # Trace evidence for each result
        enriched_results = []
        for node in results:
            evidence_chain = self.graph.trace_evidence(node["id"], max_depth=max_depth)
            related = self.graph.get_related(node["id"])
            enriched_results.append(
                {
                    **node,
                    "evidence_chain": evidence_chain,
                    "related_nodes": related,
                }
            )

        return {"nodes": enriched_results, "total": len(enriched_results)}
