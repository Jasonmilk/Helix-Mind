# Helix-Mind

一个面向数字生命体的轻量化记忆中枢 | 我的第一个 Rust 项目

> 石头无需能源故而不朽，植物能耗极低故而长寿。
> Helix-Mind 以自然为师：用最小的资源消耗，实现可持续的有意识记忆系统。

---

## 🧠 项目定位
Helix-Mind 不是传统的数据库、缓存或 RAG 框架，而是一套**活的记忆代谢系统**。
基于 Rust 构建，专注于节能、生命周期管理与知识传承，为数字生命提供核心记忆能力。

## 核心设计理念
- 意志优先：记忆中枢为核心，编排器为辅助
- 向死而生：生命周期闭环，知识可跨代传承
- 极致节能：复用 > 重计算，休眠 > 空转，遗忘是系统优化
- 边界清晰：公共知识共享，私有记忆严格隔离
- 螺旋进化：自我迭代，持续向上成长

---

## 🚀 快速上手
### 环境要求
Rust 1.75 及以上版本

### 启动步骤
```bash
# 复制配置文件模板
cp config.toml.example config.toml
cp gene_lock.md.example gene_lock.md

# 开发模式运行
cargo run -- run

# 查看系统状态
cargo run -- view
```

gRPC 服务默认地址：`127.0.0.1:50051`

---

## 📦 项目架构
本项目由 **8 个独立 Crate** 模块化构建，编译干净、运行稳定：
```
helix-mind/
├── crates/
│   ├── helix-mind-core         # 核心类型、异常与配置
│   ├── helix-mind-storage      # 存储引擎（SQLite + 拓扑缓存）
│   ├── helix-mind-retrieval    # 多模式检索引擎
│   ├── helix-mind-metabolism   # 知识代谢与凝练
│   ├── helix-mind-federation   # 联邦知识共享
│   ├── helix-mind-api          # gRPC 服务层
│   ├── helix-mind-reincarnation# 生命周期管理
│   └── helix-mind-cli          # 命令行工具
├── proto/                      # 协议定义
└── 配置文件 / 许可协议
```

---

## ✨ 个人里程碑
这是我 **第一个 Rust 项目**。
从 Python 开发者转型，从零攻克 Rust 所有权、类型系统，
完成 8 个 Crate 的模块化设计，实现 `cargo check / build` 全通过。

**恐惧，只有亲手征服才会消失。**

---

## 📄 许可证
MIT License © Jasonmilk

---
*Helix-Mind 不只是存储记忆，它孕育新生*
