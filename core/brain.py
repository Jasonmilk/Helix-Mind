import httpx
import json
import re
import logging
import asyncio
import sys
import time
from pathlib import Path
from core.memory_manager import MemoryManager
from core.dag_manager import DAGManager
from config import settings

logger = logging.getLogger("mind.brain")

class Brain:
    def __init__(self, memory: MemoryManager):
        self.memory = memory
        self.model = settings.brain_model
        self.dag = DAGManager()
        self.persona_prompt = self._load_persona()
        if self.persona_prompt:
            print("\033[1;32m✅ 思考者人格已加载\033[0m")

    def _load_persona(self) -> str:
        persona_path = Path(__file__).parent.parent / "personas" / "dag_thinker.json"
        if not persona_path.exists(): return ""
        try:
            with open(persona_path, "r", encoding="utf-8") as f:
                return json.load(f).get("system_prompt", "")
        except Exception: return ""

    async def decompose_task(self, user_requirement: str):
        gene_lock = self.memory.read("gene_lock")
        dag_index = self.dag.generate_index_map()

        messages = []
        if self.persona_prompt:
            messages.append({"role": "system", "content": self.persona_prompt})
            
        messages.append({
            "role": "user",
            "content": f"【基因锁】:\n{gene_lock}\n\n【全局地图】:\n{dag_index}\n\n【用户需求】:\n{user_requirement}\n\n请开始推导。"
        })

        # 【审查吸收】：移除导致 404 的 X-Tuck-Persona 废弃头
        headers = {"Content-Type": "application/json"}
        if settings.tuck_api_key: headers["Authorization"] = f"Bearer {settings.tuck_api_key}"

        last_action = ""
        referenced_nodes = {}

        print("\n" + "═"*60)
        print("🌌 神经符号逻辑引擎 (Neuro-Symbolic Engine) 已激活")
        print("═"*60 + "\n")

        for hop in range(5):
            if len(messages) > 7: messages = [messages[0]] + messages[-4:]

            payload = {"model": self.model, "messages": messages, "temperature": 0.4, "stream": True}
            print(f"\n\033[1;36m🧠[因果跳跃 {hop+1}/5] 脑波深潜中...\033[0m")
            
            try:
                async with httpx.AsyncClient(timeout=300.0) as client:
                    async with client.stream("POST", settings.tuck_url, json=payload, headers=headers) as resp:
                        if resp.status_code == 504:
                            print(f"\033[1;33m⚠️[504] 网关超时...\033[0m")
                            continue
                        resp.raise_for_status()

                        full_content = ""
                        async for line in resp.aiter_lines():
                            if line.startswith("data: "):
                                data = line[6:]
                                if data == "[DONE]": break
                                try:
                                    chunk = json.loads(data)
                                    content = chunk["choices"][0].get("delta", {}).get("content", "")
                                    if content:
                                        full_content += content
                                        sys.stdout.write(f"\033[90m{content}\033[0m")
                                        sys.stdout.flush()
                                except json.JSONDecodeError: continue

                        print("\n\033[1;32m[脑波接收完毕]\033[0m\n")
                        messages.append({"role": "assistant", "content": full_content})

                        action_text = full_content.split("</think>")[-1].strip() if "</think>" in full_content else full_content

                        if action_text == last_action:
                            messages.append({"role": "user", "content": "[系统警告] 重复动作！请执行 FINISH。"})
                            continue
                        last_action = action_text

                        feedback_msgs =[]
                        is_finished = False
                        final_tasks =[]

                        # 1. FETCH
                        for fetch_match in re.finditer(r"\[ACTION:\s*FETCH\((.*?)\)\]", action_text, re.IGNORECASE):
                            node_id = fetch_match.group(1).strip(' "\'')
                            print(f"\033[1;34m🔍 提取 DAG 节点: {node_id}\033[0m")
                            
                            node_path = self.dag.get_node_path(node_id)
                            if node_path: referenced_nodes[node_id] = node_path
                            
                            feedback_msgs.append(f"【FETCH {node_id} 结果】:\n{self.dag.fetch_node(node_id)}")

                        # 2. TENTACLE (审查吸收：精简 payload，解析 dehydrated_content，并自动固化)
                        for ten_match in re.finditer(r"\[ACTION:\s*TENTACLE\((.*?)\)\]", action_text, re.IGNORECASE):
                            query = ten_match.group(1).strip()
                            print(f"\033[1;35m🐙 召唤 Helix-Tentacle: {query}\033[0m")
                            try:
                                async with httpx.AsyncClient(timeout=120.0) as t_client:
                                    t_resp = await t_client.post("http://127.0.0.1:8010/v1/tentacle/process", json={"text": query})
                                    t_resp.raise_for_status()
                                    data = t_resp.json()
                                    
                                    # 提取真实的脱水结果
                                    result = data.get("dehydrated_content", data.get("answer", "脱水失败"))[:2000]
                                    
                                    # 【神来之笔】：自动将外部知识固化为临时节点，传递给双手
                                    safe_query = re.sub(r'[^a-zA-Z0-9]', '_', query)[:15]
                                    temp_id = f"ext_{safe_query}_{int(time.time())}"
                                    self.dag.write_node(temp_id, f"外部脱水知识: {query}", "", result)
                                    node_path = self.dag.get_node_path(temp_id)
                                    if node_path: referenced_nodes[temp_id] = node_path
                                    
                                    feedback_msgs.append(f"【TENTACLE 报告】(已固化至节点 {temp_id}):\n{result}")
                            except Exception as te:
                                print(f"\033[1;31m❌ [Tentacle Error]: {te}\033[0m")
                                feedback_msgs.append(f"【TENTACLE 断裂】无法获取数据。请仅基于现有 DAG 尝试推理。")

                        # 3. WRITE_NODE
                        for write_match in re.finditer(r"\[ACTION:\s*WRITE_NODE\((.*?)\)\]", action_text, re.DOTALL | re.IGNORECASE):
                            args = write_match.group(1).split(';;', 3)
                            if len(args) == 4:
                                node_id = args[0].strip()
                                res = self.dag.write_node(node_id, args[1].strip(), args[2].strip(), args[3].strip())
                                print(f"\033[1;35m✍️ 缔造新真理节点: {node_id}\033[0m")
                                
                                node_path = self.dag.get_node_path(node_id)
                                if node_path: referenced_nodes[node_id] = node_path
                                
                                feedback_msgs.append(f"【WRITE 结果】: {res}")

                        # 4. FINISH
                        finish_match = re.search(r"\[ACTION:\s*FINISH\((.*?)\)\]", action_text, re.DOTALL | re.IGNORECASE)
                        if finish_match:
                            tasks = self._extract_json_array(finish_match.group(1))
                            if tasks:
                                is_finished = True
                                final_tasks = tasks

                        if feedback_msgs:
                            messages.append({"role": "user", "content": "\n".join(feedback_msgs) + "\n请继续。"})

                        if is_finished:
                            # 审查吸收：采用结构化的 Markdown 列表传递依赖
                            capsule_tasks =[]
                            if referenced_nodes:
                                deps_lines = [f"  - {path}" for path in referenced_nodes.values()]
                                deps_str = "\n".join(deps_lines)
                                capsule_suffix = f"\n\n【前置架构蓝图】请使用 toolkit.read_file 优先通读以下设计图纸：\n{deps_str}"
                            else:
                                capsule_suffix = ""

                            for task in final_tasks:
                                capsule_tasks.append(f"{task}{capsule_suffix}")
                                
                            self.memory.append_todo(capsule_tasks)
                            print(f"\033[1;32m✅ 闭环达成！拆解出 {len(capsule_tasks)} 个全息任务。\033[0m")
                            return capsule_tasks

                        if not feedback_msgs and not is_finished:
                            print("\033[1;33m⚠️ 强制纠偏中...\033[0m")
                            messages.append({"role": "user", "content": "未检测到规范的 [ACTION: xxx]。"})

            except Exception as e:
                print(f"\033[1;31m❌ 大脑故障: {e}\033[0m")
                await asyncio.sleep(5)

        print("\033[1;31m🚨 大脑超载，强制中止。\033[0m")
        return []

    def _extract_json_array(self, text: str) -> list | None:
        try: return json.loads(text)
        except:
            match = re.search(r'\[.*\]', text, re.DOTALL)
            if match:
                try: return json.loads(match.group(0))
                except: pass
        return None
