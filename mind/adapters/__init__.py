"""Adapters module initialization."""

from mind.adapters.wiki_adapter import WikiAdapter
from mind.adapters.github_wiki import GitHubWikiAdapter

__all__ = ["WikiAdapter", "GitHubWikiAdapter"]
