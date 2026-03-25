import httpx
import json
import re
import logging
import asyncio
from core.memory_manager import MemoryManager
from core.dag_manager import DAGManager
from config import settings

logger = logging.getLogger("mind.brain")

class Brain:
    def __init__(self, memory: MemoryManager):
        self.memory = memory
        self.model = settings.brain_model
        self.dag = DAGManager()

    async def decompose_task(self, user_requirement: str):
        gene_lock = self.memory.read("gene_lock")
        dag_index = self.dag.generate_index_map()
        
        messages =[
            {"role": "user", "content": f"【基因锁】:\n{gene_lock}\n\n【全局地图】:\n{dag_index}\n\n【用户需求】:\n{user_requirement}\n\n请开始推导。"}
        ]

        headers = {
            "Content-Type": "application/json",
            "X-Tuck-Persona": "dag_thinker"
        }
        if settings.tuck_api_key:
            headers["Authorization"] = f"Bearer {settings.tuck_api_key}"

        last_action = ""
        
        # 限制最多 5 跳，防止死循环破产
        for hop in range(5):
            # 物理截流：如果历史记录过长，切掉早期的步骤，只留 System, 需求和最后 2 步
            if len(messages) > 7:
                messages =[messages[0]] + messages[-4:]

            payload = {"model": self.model, "messages": messages, "temperature": 0.3, "stream": True}
            
            logger.info(f"🧠 [Brain Hop {hop+1}/5] 深度推演中...")
            try:
                async with httpx.AsyncClient(timeout=300.0) as client:
                    async with client.stream("POST", settings.tuck_url, json=payload, headers=headers) as resp:
                        if resp.status_code == 504:
                            logger.warning(f"⚠️ [504] 网关超时，缓存接力中...")
                            continue
                        resp.raise_for_status()
                        
                        full_content = ""
                        async for line in resp.aiter_lines():
                            if line.startswith("data: ") and "[DONE]" not in line:
                                try:
                                    chunk = json.loads(line[6:])
                                    content = chunk["choices"][0].get("delta", {}).get("content", "")
                                    if content:
                                        full_content += content
                                        if len(full_content) % 50 == 0: print(".", end="", flush=True)
                                except: pass
                        
                        logger.info(" [接收完毕]")
                        messages.append({"role": "assistant", "content": full_content})
                        
                        # 剥离 R1 的思维链
                        action_text = full_content.split("</think>")[-1].strip() if "</think>" in full_content else full_content
                        
                        # 防止模型卡在重复指令里出不来
                        if action_text == last_action:
                            logger.warning("⚠️ 发现重复动作，强制纠偏。")
                            messages.append({"role": "user", "content": "[系统警告] 你的动作与上一步完全重复！请改变思路或执行 FINISH。"})
                            continue
                        last_action = action_text

                        # --- 宽容的正则解析器 ---
                        
                        # 1. FINISH 解析
                        finish_match = re.search(r"\[ACTION:\s*FINISH\((.*?)\)\]", action_text, re.DOTALL | re.IGNORECASE)
                        if finish_match:
                            tasks = self._extract_json_array(finish_match.group(1))
                            if tasks:
                                self.memory.append_todo(tasks)
                                logger.info(f"✅ 大脑推导完毕，拆解出 {len(tasks)} 个原子任务入栈！")
                                return tasks
                                
                        # 2. FETCH 解析
                        fetch_match = re.search(r"\[ACTION:\s*FETCH\((.*?)\)\]", action_text, re.IGNORECASE)
                        if fetch_match:
                            node_id = fetch_match.group(1).strip(' "\'')
                            logger.info(f"🔍 提取节点: {node_id}")
                            node_data = self.dag.fetch_node(node_id)
                            messages.append({"role": "user", "content": f"【FETCH 结果】:\n{node_data}"})
                            continue
                            
                        # 3. WRITE_NODE 解析
                        write_match = re.search(r"\[ACTION:\s*WRITE_NODE\((.*?)\)\]", action_text, re.DOTALL | re.IGNORECASE)
                        if write_match:
                            args = write_match.group(1).split(';;', 3)
                            if len(args) == 4:
                                res = self.dag.write_node(args[0].strip(), args[1].strip(), args[2].strip(), args[3].strip())
                                logger.info(f"✍️ 写入节点: {args[0].strip()}")
                                messages.append({"role": "user", "content": f"【WRITE 结果】: {res}"})
                                continue
                                
                        # 4. TENTACLE 解析
                        tentacle_match = re.search(r"\[ACTION:\s*TENTACLE\((.*?)\)\]", action_text, re.IGNORECASE)
                        if tentacle_match:
                            query = tentacle_match.group(1).strip()
                            logger.info(f"🐙 触手搜索: {query}")
                            mock_data = f"[Tentacle 暂未实装] 无法检索 '{query}'，请依靠现有图谱或直接 FINISH 分解任务。"
                            messages.append({"role": "user", "content": f"【TENTACLE 结果】:\n{mock_data}"})
                            continue

                        # 无合法 ACTION 的降级处理
                        logger.warning("⚠️ 未检测到规范的 [ACTION]，纠偏中...")
                        messages.append({"role": "user", "content": "【系统警告】未检测到规范的 [ACTION: xxx]。请严格遵循格式！"})

            except Exception as e:
                logger.error(f"❌ 引擎故障: {e}")
                await asyncio.sleep(5)
                
        logger.error("🚨 大脑因果寻路超载 (5跳)，强制中止。")
        return[]

    def _extract_json_array(self, text: str) -> list | None:
        try: return json.loads(text)
        except:
            match = re.search(r'\[.*\]', text, re.DOTALL)
            if match:
                try: return json.loads(match.group(0))
                except: pass
        return None
