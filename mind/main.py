"""FastAPI application entry point for Helix-Mind."""

import click
import structlog
from pathlib import Path
from contextlib import asynccontextmanager

from fastapi import FastAPI

from mind.core.config import load_config, MindSettings
from mind.storage.duckdb_store import DuckDBStore
from mind.storage.knowledge_dag import KnowledgeDAG
from mind.storage.memory_dag import MemoryDAG
from mind.index.hybrid_retriever import HybridRetriever
from mind.api.routes import router
from mind.api.search import init_search_api
from mind.api.write import init_write_api
from mind.api.snapshot import init_snapshot_api


# Configure structured logging
structlog.configure(
    processors=[
        structlog.processors.TimeStamper(fmt="iso"),
        structlog.processors.JSONRenderer(),
    ],
)

logger = structlog.get_logger()


@asynccontextmanager
async def lifespan(app: FastAPI):
    """Application lifespan manager."""
    # Startup
    config = app.state.config
    logger.info("Starting Helix-Mind", config=config.model_dump())

    # Initialize storage
    store = DuckDBStore(config.duckdb_path)
    app.state.store = store

    # Initialize DAGs
    knowledge_dag = KnowledgeDAG(store, config.data_dir)
    memory_dag = MemoryDAG(config.sessions_dir)
    app.state.knowledge_dag = knowledge_dag
    app.state.memory_dag = memory_dag

    # Initialize retriever
    retriever = HybridRetriever(store, rrf_k=config.rrf_k, top_k=config.top_k)
    app.state.retriever = retriever

    # Initialize API dependencies
    init_search_api(store, retriever)
    init_write_api(store)
    init_snapshot_api(store)

    logger.info("Helix-Mind started successfully")

    yield

    # Shutdown
    logger.info("Shutting down Helix-Mind")
    store.close()


def create_app(config: MindSettings = None) -> FastAPI:
    """Create FastAPI application.

    Args:
        config: Optional configuration override.

    Returns:
        FastAPI application instance.
    """
    if config is None:
        config = load_config()

    app = FastAPI(
        title="Helix-Mind",
        description="Memory Microservice for Helix Ecosystem",
        version="0.1.0",
        lifespan=lifespan,
    )

    app.state.config = config
    app.include_router(router)

    @app.get("/health")
    async def health_check():
        """Health check endpoint."""
        return {"status": "healthy", "version": "0.1.0"}

    return app


# CLI entry point
@click.command()
@click.option("--host", default="0.0.0.0", help="Host to bind")
@click.option("--port", default=8020, help="Port to bind")
@click.option("--reload", is_flag=True, help="Enable auto-reload")
def cli(host: str, port: int, reload: bool):
    """Run Helix-Mind server."""
    import uvicorn

    config = load_config()
    app = create_app(config)

    uvicorn.run(app, host=host, port=port, reload=reload)


if __name__ == "__main__":
    cli()
