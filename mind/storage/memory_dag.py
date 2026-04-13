"""Memory DAG management with JSONL event streams."""

import json
from pathlib import Path
from datetime import datetime
from typing import List, Dict, Any


class MemoryDAG:
    """Memory DAG读写 with JSONL append-only logs."""

    def __init__(self, sessions_dir: Path):
        """Initialize MemoryDAG.

        Args:
            sessions_dir: Directory for session JSONL files.
        """
        self.sessions_dir = sessions_dir
        self.sessions_dir.mkdir(parents=True, exist_ok=True)

    def append_hxr(self, session_id: str, record: Dict[str, Any]) -> None:
        """Append an HXR record to a session JSONL file.

        Args:
            session_id: Session identifier.
            record: HXR record dictionary.
        """
        record["session_id"] = session_id
        record["ts"] = datetime.now().isoformat()

        session_file = self.sessions_dir / f"{session_id}.jsonl"
        with open(session_file, "a") as f:
            f.write(json.dumps(record) + "\n")

    def get_session(self, session_id: str) -> List[Dict[str, Any]]:
        """Read complete session records.

        Args:
            session_id: Session identifier.

        Returns:
            List of HXR records.
        """
        session_file = self.sessions_dir / f"{session_id}.jsonl"
        if not session_file.exists():
            return []

        with open(session_file) as f:
            return [json.loads(line) for line in f]

    def list_sessions(self) -> List[str]:
        """List all session IDs.

        Returns:
            List of session IDs.
        """
        sessions = []
        for file in self.sessions_dir.glob("*.jsonl"):
            sessions.append(file.stem)
        return sessions

    def delete_session(self, session_id: str) -> bool:
        """Delete a session file.

        Args:
            session_id: Session identifier.

        Returns:
            True if deleted, False if not found.
        """
        session_file = self.sessions_dir / f"{session_id}.jsonl"
        if session_file.exists():
            session_file.unlink()
            return True
        return False
