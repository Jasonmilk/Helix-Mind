"""GitHub Wiki adapter for cloning and indexing GitHub Wiki repositories."""

import git
from pathlib import Path
from typing import Dict, Any

from mind.adapters.wiki_adapter import WikiAdapter


class GitHubWikiAdapter(WikiAdapter):
    """Adapter for syncing GitHub Wiki repositories."""

    def __init__(self, config_path: str, store: DuckDBStore, cache_dir: Path = None):
        """Initialize GitHubWikiAdapter.

        Args:
            config_path: Path to YAML configuration file.
            store: DuckDB storage instance.
            cache_dir: Directory for caching cloned wikis.
        """
        super().__init__(config_path, store)
        self.cache_dir = cache_dir or Path("./data/wiki_cache")
        self.cache_dir.mkdir(parents=True, exist_ok=True)

    def _sync_github_wiki(self, source: Dict[str, Any]) -> int:
        """Sync a GitHub Wiki repository.

        Args:
            source: Source configuration dictionary with 'url' and 'target_layer'.

        Returns:
            Number of nodes indexed.
        """
        repo_url = source["url"]
        target_layer = source.get("target_layer", "L1")

        # Extract repo name from URL
        repo_name = repo_url.split("/")[-1].replace(".wiki.git", "")
        local_path = self.cache_dir / repo_name

        # Clone or pull repository
        if local_path.exists():
            try:
                repo = git.Repo(local_path)
                repo.remotes.origin.pull()
            except Exception:
                pass
        else:
            try:
                git.Repo.clone_from(repo_url, local_path)
            except Exception:
                return 0

        # Index all markdown files
        count = 0
        for md_file in local_path.glob("*.md"):
            if self._index_markdown(md_file, target_layer):
                count += 1

        return count

    def _index_markdown(self, file_path: Path, layer: str) -> bool:
        """Index a single markdown file.

        Args:
            file_path: Path to markdown file.
            layer: Target knowledge layer.

        Returns:
            True if successful.
        """
        import frontmatter

        try:
            with open(file_path) as f:
                post = frontmatter.parse(f.read())
                metadata, content = post

            node = {
                "id": file_path.stem,
                "type": "Concept",
                "layer": layer,
                "title": metadata.get("title", file_path.stem),
                "summary": metadata.get("summary", ""),
                "full_content": content,
                "confidence": 1.0,
                "source": "github_wiki",
            }

            return self._index_node(node)

        except Exception:
            return False
