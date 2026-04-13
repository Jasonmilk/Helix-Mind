"""Custom exceptions for Helix-Mind."""


class MindError(Exception):
    """Base exception for Helix-Mind."""

    pass


class NodeNotFoundError(MindError):
    """Raised when a node is not found."""

    def __init__(self, node_id: str):
        self.node_id = node_id
        super().__init__(f"Node not found: {node_id}")


class VersionConflictError(MindError):
    """Raised when there is a version conflict during node update."""

    def __init__(self, node_id: str, expected: int, actual: int):
        self.node_id = node_id
        self.expected = expected
        self.actual = actual
        super().__init__(
            f"Version conflict for node {node_id}: expected {expected}, got {actual}"
        )


class StorageError(MindError):
    """Raised when there is a storage operation error."""

    pass


class IndexError(MindError):
    """Raised when there is an index operation error."""

    pass
