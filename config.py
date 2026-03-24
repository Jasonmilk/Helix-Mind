import os
from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    tuck_url: str = "http://10.0.0.54:8686/v1/chat/completions"
    tuck_api_key: str = ""
    mind_port: int = 8020
    brain_model: str = "DeepSeek-R1-0528-Qwen3-8B-IQ4_NL.gguf"
    memory_base_dir: str = "memory_base"
    max_retries: int = 4
    
    # 允许环境变量覆盖
    model_config = SettingsConfigDict(env_file=".env", env_file_encoding="utf-8", extra="ignore")

settings = Settings()
