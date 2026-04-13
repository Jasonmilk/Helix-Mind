"""Main routes registry for Helix-Mind API."""

from fastapi import APIRouter

from mind.api.search import router as search_router
from mind.api.write import router as write_router
from mind.api.snapshot import router as snapshot_router

router = APIRouter()

# Include all sub-routers
router.include_router(search_router)
router.include_router(write_router)
router.include_router(snapshot_router)
