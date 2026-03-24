from fastapi import FastAPI, BackgroundTasks, HTTPException
from pydantic import BaseModel
from core.memory_manager import MemoryManager
from core.brain import Brain
from config import settings
import logging
import uvicorn
from pathlib import Path
import json

logging.basicConfig(level=logging.INFO, format="%(asctime)s [%(levelname)s] %(message)s")

app = FastAPI(title="Helix-Mind", description="认知中枢与任务调度")

memory = MemoryManager()
brain = Brain(memory)

# 人格文件目录
PERSONAS_DIR = Path(__file__).parent / "personas"
PERSONAS_DIR.mkdir(exist_ok=True)  # 确保目录存在

class Requirement(BaseModel):
    text: str

class Report(BaseModel):
    task_id: str
    status: str
    detail: str

@app.get("/health")
async def health_check():
    return {"status": "ok", "brain_model": settings.brain_model}

@app.get("/v1/persona/{name}")
async def get_persona(name: str):
    """返回指定人格的 system_prompt 和参数（供 Tuck 调用）"""
    persona_file = PERSONAS_DIR / f"{name}.json"
    if not persona_file.exists():
        # 人格不存在时返回空人格，不影响对话
        return {"system_prompt": "", "params": {}}
    try:
        with open(persona_file, "r", encoding="utf-8") as f:
            data = json.load(f)
        # 确保返回的字段包含 system_prompt 和 params
        return {
            "system_prompt": data.get("system_prompt", ""),
            "params": data.get("params", {})
        }
    except Exception as e:
        logging.error(f"读取人格文件 {name}.json 失败: {e}")
        return {"system_prompt": "", "params": {}}

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
