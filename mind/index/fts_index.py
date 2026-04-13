"""Full-text search index using DuckDB FTS extension."""

from typing import List, Dict, Any

from mind.storage.duckdb_store import DuckDBStore


class FTSIndex:
    """BM25 keyword search using DuckDB FTS extension."""

    def __init__(self, store: DuckDBStore):
        """Initialize FTSIndex.

        Args:
            store: DuckDB storage instance.
        """
        self.store = store

    def search(self, query: str, limit: int = 5) -> List[Dict[str, Any]]:
        """Perform keyword search using BM25.

        Args:
            query: Search query string.
            limit: Maximum number of results.

        Returns:
            List of matching nodes with scores.
        """
        results = self.store.conn.execute("""
            SELECT id, title, summary, confidence, score
            FROM (
                SELECT *, score 
                FROM nodes_fts 
                WHERE nodes_fts MATCH ?
            )
            WHERE is_active = true
            ORDER BY score DESC
            LIMIT ?
        """, [query, limit]).fetchall()

        return [
            {
                "id": r[0],
                "title": r[1],
                "summary": r[2],
                "confidence": r[3],
                "score": r[4],
            }
            for r in results
        ]
