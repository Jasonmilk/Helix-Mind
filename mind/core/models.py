"""Pydantic models for Helix-Mind."""

from pydantic import BaseModel, Field
from typing import Optional, List, Dict, Any
from datetime import datetime


class Node(BaseModel):
    """Knowledge node model."""

    id: str
    type: str
    layer: str
    title: str
    summary: str
    full_content: str
    confidence: float = 1.0
    source: str = "unknown"
    version: int = 1
    is_active: bool = True
    created_at: Optional[datetime] = None
    updated_at: Optional[datetime] = None


class Edge(BaseModel):
    """Knowledge edge model."""

    source_id: str
    target_id: str
    rel_type: str
    valid_from: Optional[datetime] = None
    valid_until: Optional[datetime] = None
    properties: Optional[Dict[str, Any]] = None


class NodeWriteRequest(BaseModel):
    """Request model for writing a node."""

    id: str
    type: str
    layer: str
    title: str
    summary: str
    full_content: str
    confidence: float = 1.0
    source: str = "unknown"
    expected_version: Optional[int] = None


class NodeSummary(BaseModel):
    """Summary view of a node."""

    id: str
    title: str
    summary: str
    confidence: float
    score: Optional[float] = None
    similarity: Optional[float] = None


class SearchResponse(BaseModel):
    """Search response model."""

    nodes: List[NodeSummary]
    total: int


class SnapshotResponse(BaseModel):
    """Snapshot creation response."""

    snapshot_id: str
    created_at: datetime
    nodes_count: int
    edges_count: int
