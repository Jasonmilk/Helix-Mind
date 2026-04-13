"""Vector similarity search using local embedding models."""

from typing import List, Dict, Any

from sentence_transformers import SentenceTransformer

from mind.storage.duckdb_store import DuckDBStore


class VectorIndex:
    """Vector similarity search using local embedding model."""

    def __init__(self, store: DuckDBStore, model_name: str = "BAAI/bge-small-en"):
        """Initialize VectorIndex.

        Args:
            store: DuckDB storage instance.
            model_name: Name of the sentence transformer model.
        """
        self.store = store
        self.model = SentenceTransformer(model_name)

    def search(self, query: str, limit: int = 10) -> List[Dict[str, Any]]:
        """Perform vector similarity search.

        Args:
            query: Search query string.
            limit: Maximum number of results.

        Returns:
            List of matching nodes with similarity scores.
        """
        query_emb = self.model.encode(query).tolist()

        results = self.store.conn.execute("""
            SELECT id, title, summary, confidence,
                   array_cosine_similarity(embedding, ?::FLOAT[384]) AS similarity
            FROM nodes
            WHERE is_active = true AND embedding IS NOT NULL
            ORDER BY similarity DESC
            LIMIT ?
        """, [query_emb, limit]).fetchall()

        return [
            {
                "id": r[0],
                "title": r[1],
                "summary": r[2],
                "confidence": r[3],
                "similarity": r[4],
            }
            for r in results
        ]

    def update_embedding(self, node_id: str, content: str) -> None:
        """Update embedding for a node.

        Args:
            node_id: Node ID.
            content: Content to embed.
        """
        emb = self.model.encode(content).tolist()
        self.store.conn.execute(
            "UPDATE nodes SET embedding = ? WHERE id = ?", [emb, node_id]
        )

    def generate_embedding(self, text: str) -> List[float]:
        """Generate embedding for text.

        Args:
            text: Input text.

        Returns:
            Embedding vector as list of floats.
        """
        return self.model.encode(text).tolist()
