from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    # 网关配置
    tuck_url: str = "http://127.0.0.1:8000/v1/chat/completions"
    tuck_api_key: str = ""
    
    # 认知中枢配置
    mind_port: int = 8020
    brain_model: str = "8b-r1-model-name"  # 替换为你的 8B 大脑模型
    memory_base_dir: str = "memory_base"
    
    # 思考参数
    max_retries: int = 3
    
    model_config = SettingsConfigDict(env_file=".env", env_file_encoding="utf-8")

settings = Settings()
