"""Graph traversal using recursive CTE for evidence chain tracing."""

from typing import List, Any

from mind.storage.duckdb_store import DuckDBStore


class GraphTraverse:
    """Graph diffusion using recursive CTE queries."""

    def __init__(self, store: DuckDBStore):
        """Initialize GraphTraverse.

        Args:
            store: DuckDB storage instance.
        """
        self.store = store

    def trace_evidence(
        self, node_id: str, max_depth: int = 3
    ) -> List[str]:
        """Trace evidence chain along DERIVED_FROM edges.

        Args:
            node_id: Starting node ID.
            max_depth: Maximum traversal depth.

        Returns:
            List of ancestor node IDs.
        """
        results = self.store.conn.execute("""
            WITH RECURSIVE trace(id, depth) AS (
                SELECT source_id, 1 
                FROM edges 
                WHERE target_id = ? AND rel_type = 'DERIVED_FROM'
                UNION ALL
                SELECT e.source_id, t.depth + 1
                FROM edges e, trace t
                WHERE e.target_id = t.id 
                  AND e.rel_type = 'DERIVED_FROM' 
                  AND t.depth < ?
            )
            SELECT DISTINCT id FROM trace
        """, [node_id, max_depth]).fetchall()

        return [r[0] for r in results]

    def get_related(self, node_id: str) -> List[str]:
        """Get nodes related via RELATES_TO edges.

        Args:
            node_id: Source node ID.

        Returns:
            List of related node IDs.
        """
        results = self.store.conn.execute("""
            SELECT target_id 
            FROM edges 
            WHERE source_id = ? AND rel_type = 'RELATES_TO'
        """, [node_id]).fetchall()

        return [r[0] for r in results]

    def get_edges(
        self,
        source_id: str = None,
        target_id: str = None,
        rel_type: str = None,
    ) -> List[Dict[str, Any]]:
        """Query edges with optional filters.

        Args:
            source_id: Filter by source node ID.
            target_id: Filter by target node ID.
            rel_type: Filter by relationship type.

        Returns:
            List of edge dictionaries.
        """
        conditions = []
        params = []

        if source_id:
            conditions.append("source_id = ?")
            params.append(source_id)
        if target_id:
            conditions.append("target_id = ?")
            params.append(target_id)
        if rel_type:
            conditions.append("rel_type = ?")
            params.append(rel_type)

        where_clause = " AND ".join(conditions) if conditions else "1=1"

        query = f"""
            SELECT source_id, target_id, rel_type, valid_from, valid_until, properties
            FROM edges
            WHERE {where_clause}
        """

        results = self.store.conn.execute(query, params).fetchall()

        return [
            {
                "source_id": r[0],
                "target_id": r[1],
                "rel_type": r[2],
                "valid_from": r[3].isoformat() if r[3] else None,
                "valid_until": r[4].isoformat() if r[4] else None,
                "properties": r[5],
            }
            for r in results
        ]

    def add_edge(
        self,
        source_id: str,
        target_id: str,
        rel_type: str,
        properties: dict = None,
    ) -> bool:
        """Add an edge to the graph.

        Args:
            source_id: Source node ID.
            target_id: Target node ID.
            rel_type: Relationship type.
            properties: Optional edge properties.

        Returns:
            True if successful.
        """
        try:
            self.store.conn.execute("""
                INSERT OR REPLACE INTO edges 
                (source_id, target_id, rel_type, properties)
                VALUES (?, ?, ?, ?)
            """, [source_id, target_id, rel_type, properties])
            return True
        except Exception:
            return False
