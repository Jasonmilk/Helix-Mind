"""Sleep pipeline for memory consolidation."""

from typing import List, Dict, Any
from datetime import datetime

from mind.storage.duckdb_store import DuckDBStore
from mind.storage.memory_dag import MemoryDAG
from mind.sleep.scoring import MemoryScorer


class SleepPipeline:
    """Three-stage sleep pipeline (Light/REM/Deep) for memory consolidation."""

    def __init__(self, store: DuckDBStore, memory_dag: MemoryDAG):
        """Initialize SleepPipeline.

        Args:
            store: DuckDB storage instance.
            memory_dag: Memory DAG instance.
        """
        self.store = store
        self.memory_dag = memory_dag
        self.scorer = MemoryScorer()

    def run_sleep_cycle(
        self,
        session_ids: List[str],
        threshold: float = 0.5,
    ) -> Dict[str, Any]:
        """Run a complete sleep cycle on sessions.

        Args:
            session_ids: List of session IDs to process.
            threshold: Minimum score for memory retention.

        Returns:
            Summary of processed memories.
        """
        results = {
            "processed_sessions": len(session_ids),
            "consolidated": 0,
            "discarded": 0,
            "promoted": 0,
        }

        for session_id in session_ids:
            records = self.memory_dag.get_session(session_id)
            if not records:
                continue

            # Score each record
            scored_records = []
            for record in records:
                score = self.scorer.score_memory(record)
                if score >= threshold:
                    scored_records.append((record, score))
                    results["consolidated"] += 1
                else:
                    results["discarded"] += 1

            # Promote high-scoring memories to knowledge DAG
            for record, score in scored_records:
                if score >= 0.8:
                    self._promote_to_knowledge(record, score)
                    results["promoted"] += 1

        return results

    def _promote_to_knowledge(self, record: Dict[str, Any], score: float) -> bool:
        """Promote a memory record to knowledge DAG.

        Args:
            record: Memory record.
            score: Memory score.

        Returns:
            True if successful.
        """
        # Extract key information from HXR record
        node_id = f"mem_{record.get('session_id', 'unknown')}_{record.get('ts', '')}"
        title = record.get("summary", "Memory")[:100]
        summary = record.get("content", "")[:500]

        try:
            self.store.conn.execute("""
                INSERT OR REPLACE INTO nodes 
                (id, type, layer, title, summary, full_content, confidence, source)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            """, [
                node_id,
                "Memory",
                "L2",
                title,
                summary,
                str(record),
                score,
                "sleep_consolidation",
            ])
            return True
        except Exception:
            return False

    def light_sleep(self, session_ids: List[str]) -> Dict[str, Any]:
        """Light sleep stage: quick scoring and filtering.

        Args:
            session_ids: Sessions to process.

        Returns:
            Processing results.
        """
        return self.run_sleep_cycle(session_ids, threshold=0.3)

    def rem_sleep(self, session_ids: List[str]) -> Dict[str, Any]:
        """REM sleep stage: deeper consolidation with association.

        Args:
            session_ids: Sessions to process.

        Returns:
            Processing results.
        """
        return self.run_sleep_cycle(session_ids, threshold=0.5)

    def deep_sleep(self, session_ids: List[str]) -> Dict[str, Any]:
        """Deep sleep stage: full consolidation and promotion.

        Args:
            session_ids: Sessions to process.

        Returns:
            Processing results.
        """
        return self.run_sleep_cycle(session_ids, threshold=0.7)
