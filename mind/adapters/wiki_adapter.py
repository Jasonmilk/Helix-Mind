"""Wiki adapter for importing external knowledge sources."""

import yaml
from pathlib import Path
from typing import Dict, Any, List

from mind.storage.duckdb_store import DuckDBStore


class WikiAdapter:
    """Base adapter for importing wiki and markdown content."""

    def __init__(self, config_path: str, store: DuckDBStore):
        """Initialize WikiAdapter.

        Args:
            config_path: Path to YAML configuration file.
            store: DuckDB storage instance.
        """
        with open(config_path) as f:
            self.config = yaml.safe_load(f)
        self.store = store

    def sync(self) -> int:
        """Synchronize all configured sources.

        Returns:
            Number of nodes indexed.
        """
        count = 0
        for source in self.config.get("sources", []):
            if source["type"] == "github_wiki":
                count += self._sync_github_wiki(source)
            elif source["type"] == "local_markdown":
                count += self._sync_local_markdown(source)
        return count

    def _sync_github_wiki(self, source: Dict[str, Any]) -> int:
        """Sync GitHub Wiki source (to be implemented by subclass).

        Args:
            source: Source configuration dictionary.

        Returns:
            Number of nodes indexed.
        """
        raise NotImplementedError

    def _sync_local_markdown(self, source: Dict[str, Any]) -> int:
        """Sync local markdown files.

        Args:
            source: Source configuration dictionary.

        Returns:
            Number of nodes indexed.
        """
        import frontmatter

        source_dir = Path(source["path"])
        target_layer = source.get("target_layer", "L1")
        count = 0

        for md_file in source_dir.glob("**/*.md"):
            try:
                with open(md_file) as f:
                    post = frontmatter.parse(f.read())
                    metadata, content = post

                node = {
                    "id": md_file.stem,
                    "type": "Concept",
                    "layer": target_layer,
                    "title": metadata.get("title", md_file.stem),
                    "summary": metadata.get("summary", ""),
                    "full_content": content,
                    "confidence": 1.0,
                    "source": "local_import",
                }

                self.store.conn.execute("""
                    INSERT OR REPLACE INTO nodes 
                    (id, type, layer, title, summary, full_content, confidence, source)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                """, [
                    node["id"],
                    node["type"],
                    node["layer"],
                    node["title"],
                    node["summary"],
                    node["full_content"],
                    node["confidence"],
                    node["source"],
                ])
                count += 1

            except Exception:
                continue

        return count

    def _index_node(self, node: Dict[str, Any]) -> bool:
        """Index a single node.

        Args:
            node: Node data dictionary.

        Returns:
            True if successful.
        """
        try:
            self.store.conn.execute("""
                INSERT OR REPLACE INTO nodes 
                (id, type, layer, title, summary, full_content, confidence, source)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            """, [
                node["id"],
                node["type"],
                node["layer"],
                node["title"],
                node["summary"],
                node["full_content"],
                node["confidence"],
                node["source"],
            ])
            return True
        except Exception:
            return False
