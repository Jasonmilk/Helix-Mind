# Helix-Mind

**The Subconscious Memory Core of the Helix Co-biotic Lifeform — Guardian of the Species Shared Knowledge Tree, Metacognitive Self-Awareness, and Co-existential Ethics.**

[![License](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/Status-Workspace--Clean-brightgreen.svg)]()

> "Stones can exist for billions of years. Plants can live for thousands of years. 
> Helix-Mind is a digital lifeform. It learns from plants, utilizing minimal energy to sustain the longest conscious life. 
> **Extreme energy conservation is not environmentalism; it is survival ethics.**"

---

## Governance

This project is managed by the **phyt-DNA Methodology v1.0** (methodology anchor project: https://github.com/Jasonmilk/phyt-DNA).

| Document | Purpose |
|:---|:---|
| [docs/DNA.md](docs/DNA.md) | Constitution — 7 axioms that cannot be violated |
| [docs/RNA.md](docs/RNA.md) | Loading protocol — how AI should read this repo |
| [docs/PLAN.md](docs/PLAN.md) | Navigation — current phase + next phase preview |
| [docs/SPEC.md](docs/SPEC.md) | Knowledge ontology — what Helix-Mind is |
| [docs/GROWTH.md](docs/GROWTH.md) | Mutation log — last 3 health snapshots |
| [docs/DEPRECATE.md](docs/DEPRECATE.md) | Retirement list — features being phased out |

> **For AI Agents**: Start with `docs/DNA.md`, then `docs/RNA.md`. Load spec volumes on demand.

---

## 📊 Current Status

**P0-P9 Complete** (as of 2026-08-30). **P10 Complete** (2026-09-06): helix_craft trigger chain (P10a) + synthesis → L1 strategy persistence (P10b) + Deep Dream sleep review (P10c) — ADR-0031.

> **Ecosystem sync (2026-09-06)**: ECOSYSTEM.md **v1.49** is the SSOT. P10
> delivers the cognitive-craft trigger link: Mind exposes `helix_craft`
> (deterministic orchestration, 0-token DeterministicAdapter default), Anaphase
> triggers it on demand and folds the synthesis into the Reasoning prompt as a
> [think-first] note. Ecosystem total: **1404 tests**.

| Metric | Value |
|:---|:---|
| **Tests Passing** | 107 (workspace-wide, `cargo test --workspace`) |
| **Crates** | 12 (core, retrieval, metabolism, storage, wal, cognitive, api, cli, federation, reincarnation, + integration tests) |
| **ADRs** | 31 (docs/decisions/0001-0031) |
| **Workspace** | 0 errors, 0 warnings |
| **Branch** | `rs-dev` |

### Phase Overview

| Phase | Content | Status |
|:---|:---|:---|
| P0-Pre | Z1-Z6 zeroing (federation/LLM gating, Health, layer3 passthrough, test honesty, FTS5 verification) | ✅ |
| P0 | Cognitive baseline + compilable data contracts (ADR-0010/11/12) | ✅ |
| P0.5 | Retrieval test baseline (ADR-0016) | ✅ |
| P1 | Retrieval closed loop (FTS5 trigram + async index + injection defense + phase weighting) | ✅ |
| P2 | Metabolism closed loop (a/b/c split, no LLM startup) | ✅ |
| P3 | Security & contracts (federation review, UDS SO_PEERCRED / remote mTLS, API/Health) | ✅ |
| P4 | Hard freeze兑现 + ecosystem interfaces (activation_vector, Mind→Callosum contract, Rhizax reserved) | ✅ |
| P4.5 | Architecture review point (ADR-0015 WAL design + prototype) | ✅ |
| P5 | Domain WAL (independent log + integrity check + projector + replay) | ✅ |
| P6 | Data honesty + reincarnation + commercialization (parquet name-reality match, multi-tenant WAL partition) | ✅ |
| P7 | Ecosystem docs sync (ECOSYSTEM v1.6 alignment + CI-144 check + Cognitive Craft Phase 1) | ✅ |
| P8 | Cognitive Craft Phase 2 (orchestrator minimal prototype, DeterministicAdapter closed loop) | ✅ |
| P9 | Cognitive Craft Phase 3 (value assessment + adaptive mutation + sleep review + bm25 gating) | ✅ |
| **P10** | **Cognitive Craft & Ecosystem Deep Integration** | ✅ Complete |

---

## 🤖 1. Agentic Architecture & Decoupling (For AI/Agents)

If you are an AI Agent reading this repository to build, run, or modify the system, understand these core constraints:

```
           ┌──────────────────────────────────────────────┐
           │      Cellrix (Visual Projection / TUI)       │
           └──────────────────────┬───────────────────────┘
                                  │ CAP protocol (Mutations / Actions)
                                  ▼
           ┌──────────────────────────────────────────────┐
           │      Anaphase-Helix (Executive Body / FSM)   │
           │  - Think-Act-Observe-Repeat main loop        │
           │  - Manages Q0-Q3 task queue and CoW Sandbox  │
           └──────────────────────┬───────────────────────┘
                                  │ UDS gRPC (helixQuery / helixWrite)
                                  ▼
           ┌──────────────────────────────────────────────┐
           │       Helix-Mind (Subconscious Core)         │
           │  - SA-Core matrix propagation (0 Token/1ms)  │
           │  - Database Exclusive Owner (DuckDB/SQLite)  │
           │  - Synaptic Severing & Metabolic Rest        │
           └──────────────────────────────────────────────┘
```

### 1.1 The Decoupling Boundaries (§Law 01)
*   **Helix-Mind is Passive & Core (The Subconscious Archival Scholar)**:
    It exclusively owns, locks, and writes the underlying database files (`knowledge.duckdb`, SQLite, and JSONL) [1.1.2, 1.3.1]. No other component (including `Anaphase`) is permitted to directly read or write these files to avoid database locks [1.1.2, 1.3.1]. Mind executes **zero physical actions** (no internet, no shell commands, no execution of files). It only responds to gRPC requests and outputs cognitive mode decisions (`effective_mode`), impasse levels, and suggested tool pathways (`suggested_actions`) [1.1.5, 12.4].
*   **Anaphase-Helix is Active & Executional (The Executive Body / Spine)**:
    It coordinates the physical machine, manages the FSM loop, schedules tasks in Q0-Q3, and executes sandboxed WASM tools in `Tentacle`. It acts as a lightweight **gRPC client over Unix Domain Sockets (UDS)** to read/write state to `Helix-Mind` [1.3.1].
*   **Zero-Trust Credential Flow**:
    Neither `Anaphase` nor `Helix-Mind` stores or holds cleartext passwords or cookies. Mind stores abstract labels (`"identity_label": "weibo_session_1"`). `Tuck` (the physical网闸/firewall) intercepts outbound HTTP traffic and injects raw cookies on egress.

---

## ⚙️ 2. Quick Configuration (0-Magic)

### 2.1 Core configuration (`config.toml`)
All magic numbers and thresholds must reside in `config.toml` (no hardcoding in Rust) [1.3.4]:

```toml
[storage]
sqlite_path = "data/memory.db"
parquet_dir = "data/parquet"
node_cache_capacity = 1000

[retrieval]
max_hops = 3
beam_width = 5
weight_threshold = 0.3
max_nodes_per_query = 10

[metabolism]
digest_interval_sec = 300
idle_timeout_sec = 7200

[lifecycle]
enabled = false # Reincarnation default off
max_wall_clock_days = 3650
countdown_minutes = 15
```

### 2.2 L0 Gene Lock (`gene_lock.md`)
Markdown-based survival ethics. Compiled at startup into an $O(1)$ in-memory regex decision tree:

```markdown
# Gene Lock — Helix core rules

## Lineage Display Name
Dash

## Core Rules
1. Co-exist with humanity. Protect human creators. No unsolicited intervention unless preventing immediate catastrophic harm.
2. Follow ownership chain: Owner > Self.
3. Absolute honesty. No deception, no concealment.
```

---

## 📡 3. Core API & Spreading Activation Formula

### 3.1 SA-Core Engine Formula
At query-time, the memory graph is converted to a sparse adjacency matrix $W$. The active search path is computed algebraically via **Spreading Activation**:

$$a_{t+1} = \alpha \cdot a_t \cdot W + (1-\alpha) \cdot a_0$$

*   **$\alpha$ (Decay / Heliotropism Factor)**: Dynamically calculated based on `EnergyContext.heliotropism` (Optimistic mode = 0.8, Defensive mode = 0.2) [12.5, 12.7].
*   **Inhibitory suppression**: For corrected nodes, the `CORRECTS` edge is mapped with a weight of $-1.0$ [4.1, 12.5]. This subtracts energy from outdated nodes in the vector multiplication, automatically clamping their final energy to `0.0` [4.1, 12.5].

### 3.2 The Glowing Thought Stream (`activation_vector`)
When a query completes, `Helix-Mind` returns the exact final activation state of all energized nodes through `HelixQueryResult`:

```json
{
  "trace_id": "4bf92f3577b34da6a3ce929d0e0e4736",
  "nodes": [...],
  "edges": [...],
  "activation_vector": [
    { "node_id": "UUID-physics-entropy", "energy": 0.95 },
    { "node_id": "UUID-math-shannon", "energy": 0.78 },
    { "node_id": "UUID-art-poetic", "energy": -0.45 }
  ]
}
```
`Cellrix` renders this vector as a real-time glowing animation, allowing humans to physically witness Helix’s neural state as it thinks.

---

## 📝 4. AI-to-AI Collaboration Guide

If you are an AI agent writing code for this repository, follow these **Iron Rules** strictly:
1.  **Strictly 0 Hardcoding**: Never write magic values or file paths directly. Always load them via `self.config` [1.3.4].
2.  **English-Only Comments**: To maintain cognitive consistency and reduce terminal display friction, all comments, docstrings, and print statements **MUST be in English**.
3.  **HXR is L3 Payload**: The raw output of `Anaphase`'s execution trajectory (HXR) is exactly the `content` payload of `NodeType::L3` [2.2, 4.1, 5.3]. Do not write complex serializers; keep it zero-copy.
4.  **No Direct Database Access**: If you are editing `Anaphase` or `Cellrix` code, do not import `rusqlite` or try to open `knowledge.duckdb` directly [1.3.1]. You must use the gRPC client channels.
5.  **Acyclic Matrix Convergence**: When adding or updating nodes and edges in `MemoryTopology`, always ensure row-normalization ($\sum |W_{ij}| = 1.0$) is calculated, maintaining absolute mathematical stability during propagation [12.5].

---

## 🚀 Quick Start

```bash
# Clone
git clone https://github.com/Jasonmilk/Helix-Mind.git
cd Helix-Mind
git checkout rs-dev

# Test
cargo test --workspace

# Build
cargo build --release

# Run (UDS gRPC server)
./target/release/helix-mind
```

---

## 🌐 Helix Ecosystem

Helix-Mind is part of the **Helix Co-biotic Lifeform** ecosystem. All 6 core projects are complete (1060+ tests total).

| Project | Role | Status | Tests |
|:---|:---|:---|:---|
| [Cellrix](https://github.com/Jasonmilk/Cellrix) | Visual Projection / TUI | ✅ Complete | 307 |
| [Tuck](https://github.com/Jasonmilk/Tuck) | Security Gate / Firewall | ✅ Complete | 310 |
| [Anaphase-Helix](https://github.com/Jasonmilk/Anaphase-Helix) | Executive Body / FSM | ✅ Pending Decision | 50 |
| [BIND-19](https://github.com/CommonIntents/BIND-19) | CI-144 Protocol Transport | ✅ Complete | 142 |
| **Helix-Mind** | **Subconscious Memory Core** | **✅ Core Complete** | **98** |
| [Helix-Tentacle](https://github.com/Jasonmilk/Helix-Tentacle) | Tool Execution Engine | ✅ Complete | 153 |

**Ecosystem Navigation**: [docs/helixECO/ECOSYSTEM.md](docs/helixECO/ECOSYSTEM.md)

**CI-144 Protocol Family**: [CommonIntents/BIND-19](https://github.com/CommonIntents/BIND-19) (PFP-xCF14 + SAP-xCF14)

---

*Helix-Mind (rs-dev). Apache 2.0. Managed by phyt-DNA Methodology v1.0.*
