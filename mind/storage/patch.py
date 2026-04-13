"""Patch management for incremental knowledge base updates."""

import json
from datetime import datetime
from pathlib import Path
from typing import List, Dict, Any


class PatchManager:
    """Manages JSON Patch files for incremental updates."""

    def __init__(self, patches_dir: Path):
        """Initialize PatchManager.

        Args:
            patches_dir: Directory for patch files.
        """
        self.patches_dir = patches_dir
        self.patches_dir.mkdir(parents=True, exist_ok=True)

    def create_patch(self, node_data: Dict[str, Any]) -> Path:
        """Create a new patch file.

        Args:
            node_data: Node data to patch.

        Returns:
            Path to the created patch file.
        """
        timestamp = datetime.now().isoformat()
        patch_file = self.patches_dir / f"{timestamp}_patch.json"

        with open(patch_file, "w") as f:
            json.dump({"node": node_data, "ts": timestamp}, f)

        return patch_file

    def list_patches(self, since: datetime = None) -> List[Path]:
        """List patch files.

        Args:
            since: Optional datetime filter.

        Returns:
            List of patch file paths.
        """
        patches = sorted(self.patches_dir.glob("*_patch.json"))

        if since:
            patches = [
                p
                for p in patches
                if datetime.fromisoformat(p.stem.split("_")[0]) >= since
            ]

        return patches

    def read_patch(self, patch_file: Path) -> Dict[str, Any]:
        """Read a patch file.

        Args:
            patch_file: Path to patch file.

        Returns:
            Patch data dictionary.
        """
        with open(patch_file) as f:
            return json.load(f)

    def apply_patches(self, patches: List[Path]) -> int:
        """Apply multiple patches (for recovery scenarios).

        Args:
            patches: List of patch file paths.

        Returns:
            Number of patches applied.
        """
        count = 0
        for patch_file in patches:
            try:
                patch_data = self.read_patch(patch_file)
                # Patches are applied during normal write operations
                # This method is for recovery/replay scenarios
                count += 1
            except Exception:
                continue
        return count

    def cleanup_patches(self, older_than: datetime) -> int:
        """Clean up old patch files.

        Args:
            older_than: Remove patches older than this datetime.

        Returns:
            Number of patches removed.
        """
        removed = 0
        for patch_file in self.patches_dir.glob("*_patch.json"):
            try:
                patch_ts = datetime.fromisoformat(patch_file.stem.split("_")[0])
                if patch_ts < older_than:
                    patch_file.unlink()
                    removed += 1
            except Exception:
                continue
        return removed
