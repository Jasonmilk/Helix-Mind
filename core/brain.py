import httpx
import json
import re
import logging
import asyncio
import sys
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
        
        print("\n" + "═"*60)
        print("🌌 神经符号逻辑引擎 (Neuro-Symbolic Engine) 已激活")
        print("═"*60 + "\n")

        for hop in range(5):
            if len(messages) > 7:
                messages = [messages[0]] + messages[-4:]

            payload = {"model": self.model, "messages": messages, "temperature": 0.3, "stream": True}
            
            print(f"\n\033[1;36m🧠 [因果跳跃 {hop+1}/5] 脑波深潜中...\033[0m")
            try:
                async with httpx.AsyncClient(timeout=300.0) as client:
                    async with client.stream("POST", settings.tuck_url, json=payload, headers=headers) as resp:
                        if resp.status_code == 504:
                            print(f"\033[1;33m⚠️ [504] 网关超时，正在通过 KV-Cache 物理击穿...\033[0m")
                            continue
                        resp.raise_for_status()
                        
                        full_content = ""
                        # 赛博朋克级视觉体验：灰色打印思维链
                        for line in resp.iter_lines():
                            if line:
                                decoded_line = line.decode('utf-8', errors='replace')
                                if decoded_line.startswith("data: ") and "[DONE]" not in decoded_line:
                                    try:
                                        chunk = json.loads(decoded_line[6:])
                                        content = chunk["choices"][0].get("delta", {}).get("content", "")
                                        if content:
                                            full_content += content
                                            # 实时终端打印，90m 代表暗灰色，极具科幻感
                                            sys.stdout.write(f"\033[90m{content}\033[0m")
                                            sys.stdout.flush()
                                    except Exception: pass
                        
                        print("\n\033[1;32m[脑波接收完毕]\033[0m\n")
                        messages.append({"role": "assistant", "content": full_content})
                        
                        action_text = full_content.split("</think>")[-1].strip() if "</think>" in full_content else full_content
                        
                        if action_text == last_action:
                            print("\033[1;33m⚠️ 发现重复动作，强制纠偏。\033[0m")
                            messages.append({"role": "user", "content": "[系统警告] 你的动作与上一步完全重复！请改变思路或执行 FINISH。"})
                            continue
                        last_action = action_text

                        # --- 动作路由 ---
                        finish_match = re.search(r"\[ACTION:\s*FINISH\((.*?)\)\]", action_text, re.DOTALL | re.IGNORECASE)
                        if finish_match:
                            tasks = self._extract_json_array(finish_match.group(1))
                            if tasks:
                                self.memory.append_todo(tasks)
                                print(f"\033[1;32m✅ 逻辑闭环达成！拆解出 {len(tasks)} 个物理任务入栈。\033[0m")
                                return tasks

                        fetch_match = re.search(r"\[ACTION:\s*FETCH\((.*?)\)\]", action_text, re.IGNORECASE)
                        if fetch_match:
                            node_id = fetch_match.group(1).strip(' "\'')
                            print(f"\033[1;34m🔍 提取 DAG 节点: {node_id}\033[0m")
                            node_data = self.dag.fetch_node(node_id)
                            messages.append({"role": "user", "content": f"【FETCH 结果】:\n{node_data}"})
                            continue
                            
                        write_match = re.search(r"\[ACTION:\s*WRITE_NODE\((.*?)\)\]", action_text, re.DOTALL | re.IGNORECASE)
                        if write_match:
                            args = write_match.group(1).split(';;', 3)
                            if len(args) == 4:
                                res = self.dag.write_node(args[0].strip(), args[1].strip(), args[2].strip(), args[3].strip())
                                print(f"\033[1;35m✍️ 缔造新真理节点: {args[0].strip()}\033[0m")
                                messages.append({"role": "user", "content": f"【WRITE 结果】: {res}"})
                                continue

                        # 【补全终极拼图】：接入真实的 Helix-Tentacle
                        tentacle_match = re.search(r"\[ACTION:\s*TENTACLE\((.*?)\)\]", action_text, re.IGNORECASE)
                        if tentacle_match:
                            query = tentacle_match.group(1).strip()
                            print(f"\033[1;35m🐙 发现知识断层！召唤 Helix-Tentacle 前往外网脱水: {query}\033[0m")
                            try:
                                # 假设您的触手微服务运行在 8010 端口
                                async with httpx.AsyncClient(timeout=60.0) as t_client:
                                    t_resp = await t_client.post(
                                        "http://127.0.0.1:8010/v1/tentacle/search_and_dehydrate", 
                                        json={"query": query}
                                    )
                                    t_resp.raise_for_status()
                                    dehydrated_knowledge = t_resp.json().get("result", "脱水失败，未获取到有效信息。")
                                    dehydrated_knowledge = dehydrated_knowledge[:2000] # 防爆截断
                                    
                                    print(f"\033[1;32m✅ 触手脱水成功，返回 {len(dehydrated_knowledge)} 字节暗物质。\033[0m")
                                    messages.append({"role": "user", "content": f"【TENTACLE 物理脱水报告】:\n{dehydrated_knowledge}\n\n请基于此外部知识继续推演，并尝试固化为节点。"})
                            except Exception as te:
                                print(f"\033[1;31m❌ [Tentacle Error] 触手连接失败: {te}\033[0m")
                                messages.append({"role": "user", "content": f"【TENTACLE 触手断裂】: 无法获取外部网络数据 ({te})。请仅基于现有 DAG 尝试完成推理，或直接 FINISH。"})
                            continue

                        print("\033[1;33m⚠️ 未检测到规范的 [ACTION]，强制纠偏中...\033[0m")
                        messages.append({"role": "user", "content": "【系统警告】未检测到规范的 [ACTION: xxx]。请严格遵循格式！"})

            except Exception as e:
                print(f"\033[1;31m❌ 大脑故障: {e}\033[0m")
                await asyncio.sleep(5)
                
        print("\033[1;31m🚨 大脑因果寻路超载 (5跳)，强制中止。\033[0m")
        return []

    def _extract_json_array(self, text: str) -> list | None:
        try: return json.loads(text)
        except:
            match = re.search(r'\[.*\]', text, re.DOTALL)
            if match:
                try: return json.loads(match.group(0))
                except: pass
        return None
