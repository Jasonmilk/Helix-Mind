from fastapi import FastAPI, BackgroundTasks, HTTPException
from pydantic import BaseModel
from core.memory_manager import MemoryManager
from core.brain import Brain
from config import settings
import logging
import uvicorn

logging.basicConfig(level=logging.INFO, format="%(asctime)s [%(levelname)s] %(message)s")

app = FastAPI(title="Helix-Mind", description="认知中枢与任务调度")

memory = MemoryManager()
brain = Brain(memory)

class Requirement(BaseModel):
    text: str

class Report(BaseModel):
    task_id: str
    status: str
    detail: str

@app.get("/health")
async def health_check():
    return {"status": "ok", "brain_model": settings.brain_model}

@app.post("/v1/mind/think")
async def trigger_thinking(req: Requirement, background_tasks: BackgroundTasks):
    try:
        memory.write("hippocampus", f"【新需求入栈】: {req.text}\n", append=True)
        background_tasks.add_task(brain.decompose_task, req.text)
        return {"status": "Thinking... Brain is decomposing the task."}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/v1/mind/todo/pop")
async def pop_todo():
    try:
        task = memory.pop_todo()
        if task:
            return {"has_task": True, "task": task}
        return {"has_task": False, "task": None}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/v1/mind/report")
async def report_result(report: Report):
    try:
        log_entry = f"【任务汇报 - ID:{report.task_id} | 状态:{report.status}】:\n{report.detail}\n"
        memory.write("hippocampus", log_entry, append=True)
        return {"status": "recorded"}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

if __name__ == "__main__":
    uvicorn.run("mind_server:app", host="0.0.0.0", port=settings.mind_port, reload=True)
