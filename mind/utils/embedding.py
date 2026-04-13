"""Embedding service wrapper for local models."""

from typing import List, Optional
from sentence_transformers import SentenceTransformer


class EmbeddingService:
    """Service for generating text embeddings using local models."""

    def __init__(self, model_name: str = "BAAI/bge-small-en"):
        """Initialize EmbeddingService.

        Args:
            model_name: Name of the sentence transformer model.
        """
        self.model_name = model_name
        self._model: Optional[SentenceTransformer] = None

    @property
    def model(self) -> SentenceTransformer:
        """Lazy-load the embedding model."""
        if self._model is None:
            self._model = SentenceTransformer(self.model_name)
        return self._model

    def encode(self, text: str) -> List[float]:
        """Generate embedding for a single text.

        Args:
            text: Input text.

        Returns:
            Embedding vector as list of floats.
        """
        return self.model.encode(text).tolist()

    def encode_batch(self, texts: List[str]) -> List[List[float]]:
        """Generate embeddings for multiple texts.

        Args:
            texts: List of input texts.

        Returns:
            List of embedding vectors.
        """
        embeddings = self.model.encode(texts)
        return embeddings.tolist()

    def get_dimension(self) -> int:
        """Get the embedding dimension.

        Returns:
            Dimension of the embedding vectors.
        """
        # Generate a test embedding to get dimension
        test_emb = self.encode("test")
        return len(test_emb)
