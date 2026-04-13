# Helix-Mind: Memory Microservice

Helix-Mind 是 Helix 生态的记忆系统中枢，基于 DuckDB + Parquet 存储引擎，提供三层混合索引（BM25 + 向量 + 图扩散）和 Snapshot + Patch 版本化机制。

## 快速启动

### 1. 安装依赖

```bash
cd helix-mind
pip install -e ".[dev]"
```

### 2. 配置环境

```bash
cp .env.example .env
# 编辑 .env 文件，配置数据目录和模型路径
```

### 3. 启动服务

```bash
python -m uvicorn mind.main:app --host 0.0.0.0 --port 8020
```

### 4. 验证服务

```bash
curl http://localhost:8020/v1/mind/search?query=test&limit=5
```

## 核心功能

- **DuckDB 存储引擎**: 嵌入式数据库，统一管理节点、边和索引
- **三层混合索引**: BM25 关键词检索 + 向量语义检索 + 图扩散检索
- **Snapshot + Patch**: Parquet 快照 + JSON Patch 增量修正
- **Wiki 适配器**: GitHub Wiki 和本地 Markdown 自动索引
- **睡眠管道**: 记忆巩固与加权评分

## API 文档

启动服务后访问：http://localhost:8020/docs

## 项目结构

```
helix-mind/
├── mind/
│   ├── main.py                 # FastAPI 入口
│   ├── api/                    # REST API 路由
│   ├── core/                   # 核心配置与模型
│   ├── storage/                # DuckDB 存储引擎
│   ├── index/                  # 混合索引实现
│   ├── adapters/               # Wiki 适配器
│   ├── sleep/                  # 睡眠管道
│   └── utils/                  # 工具函数
├── config/                     # 配置文件
├── data/                       # 数据目录
├── tests/                      # 测试用例
└── pyproject.toml              # 依赖管理
```

## 开发测试

```bash
# 运行测试
pytest

# 代码检查
ruff check mind/

# 类型检查
mypy mind/
```

## 许可证

MIT License
