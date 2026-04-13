"""Memory scoring with six-dimensional weighted evaluation."""

from typing import Dict, Any


class MemoryScorer:
    """Six-dimensional weighted memory scorer."""

    def __init__(
        self,
        recency_weight: float = 0.2,
        frequency_weight: float = 0.2,
        relevance_weight: float = 0.2,
        completeness_weight: float = 0.15,
        clarity_weight: float = 0.15,
        actionability_weight: float = 0.1,
    ):
        """Initialize MemoryScorer.

        Args:
            recency_weight: Weight for recency score.
            frequency_weight: Weight for frequency score.
            relevance_weight: Weight for relevance score.
            completeness_weight: Weight for completeness score.
            clarity_weight: Weight for clarity score.
            actionability_weight: Weight for actionability score.
        """
        self.weights = {
            "recency": recency_weight,
            "frequency": frequency_weight,
            "relevance": relevance_weight,
            "completeness": completeness_weight,
            "clarity": clarity_weight,
            "actionability": actionability_weight,
        }

    def score_memory(self, record: Dict[str, Any]) -> float:
        """Calculate overall memory score.

        Args:
            record: Memory record dictionary.

        Returns:
            Overall score between 0 and 1.
        """
        scores = {
            "recency": self._score_recency(record),
            "frequency": self._score_frequency(record),
            "relevance": self._score_relevance(record),
            "completeness": self._score_completeness(record),
            "clarity": self._score_clarity(record),
            "actionability": self._score_actionability(record),
        }

        total_score = sum(
            scores[dim] * weight for dim, weight in self.weights.items()
        )

        return min(1.0, max(0.0, total_score))

    def _score_recency(self, record: Dict[str, Any]) -> float:
        """Score based on recency (newer = higher).

        Args:
            record: Memory record.

        Returns:
            Score between 0 and 1.
        """
        from datetime import datetime, timedelta

        ts_str = record.get("ts", "")
        if not ts_str:
            return 0.5

        try:
            ts = datetime.fromisoformat(ts_str)
            age = datetime.now() - ts

            # Decay: 1 day = 1.0, 7 days = 0.5, 30 days = 0.1
            days = age.total_seconds() / 86400
            if days <= 1:
                return 1.0
            elif days <= 7:
                return 0.5 + 0.5 * (7 - days) / 6
            elif days <= 30:
                return 0.1 + 0.4 * (30 - days) / 23
            else:
                return 0.1
        except Exception:
            return 0.5

    def _score_frequency(self, record: Dict[str, Any]) -> float:
        """Score based on access/mention frequency.

        Args:
            record: Memory record.

        Returns:
            Score between 0 and 1.
        """
        # Placeholder: actual implementation would track access count
        access_count = record.get("access_count", 1)
        return min(1.0, access_count / 10)

    def _score_relevance(self, record: Dict[str, Any]) -> float:
        """Score based on task relevance.

        Args:
            record: Memory record.

        Returns:
            Score between 0 and 1.
        """
        # Placeholder: check if record contains key task indicators
        content = record.get("content", "") + record.get("summary", "")
        keywords = ["task", "goal", "result", "decision", "conclusion"]
        matches = sum(1 for kw in keywords if kw.lower() in content.lower())
        return min(1.0, matches / len(keywords))

    def _score_completeness(self, record: Dict[str, Any]) -> float:
        """Score based on information completeness.

        Args:
            record: Memory record.

        Returns:
            Score between 0 and 1.
        """
        required_fields = ["session_id", "ts", "content"]
        present = sum(1 for field in required_fields if record.get(field))
        return present / len(required_fields)

    def _score_clarity(self, record: Dict[str, Any]) -> float:
        """Score based on content clarity (length, structure).

        Args:
            record: Memory record.

        Returns:
            Score between 0 and 1.
        """
        content = record.get("content", "")
        length = len(content)

        # Optimal length: 100-1000 chars
        if 100 <= length <= 1000:
            return 1.0
        elif length < 100:
            return length / 100
        else:
            return max(0.5, 1.0 - (length - 1000) / 9000)

    def _score_actionability(self, record: Dict[str, Any]) -> float:
        """Score based on actionability (contains actions/decisions).

        Args:
            record: Memory record.

        Returns:
            Score between 0 and 1.
        """
        content = record.get("content", "").lower()
        action_indicators = [
            "should",
            "must",
            "will",
            "decided",
            "action",
            "next step",
            "todo",
        ]
        matches = sum(1 for indicator in action_indicators if indicator in content)
        return min(1.0, matches / len(action_indicators))
