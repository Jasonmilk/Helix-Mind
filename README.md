# Helix-Mind (Rust)
Lightweight Memory Hub for Digital Lifeforms | My First Rust Project

> Stones need no energy, hence they are immortal.  
> Plants consume almost nothing, hence they live long.  
> Helix-Mind learns from nature: achieve sustainable conscious memory with minimal resource consumption.

---

## 🧠 Project Overview
Helix-Mind is not a traditional database, cache or RAG framework. It is a **living memory metabolism system**.  
Built in Rust, it focuses on energy efficiency, lifecycle management and knowledge inheritance, providing core memory capabilities for digital lifeforms.

## Core Design Principles
- **Will First**: Memory hub is the core, orchestrator is auxiliary
- **Memento Mori & Rebirth**: Closed lifecycle with cross-generational knowledge inheritance
- **Extreme Efficiency**: Reuse > Recalculation, Hibernation > Idling, Forgetting is optimization
- **Clear Boundaries**: Public knowledge shared, private memories strictly isolated
- **Spiral Evolution**: Self-iteration for continuous upward growth

---

## 🚀 Quick Start
### Prerequisites
Rust 1.75 or higher

### Getting Started
```bash
# Copy configuration templates
cp config.toml.example config.toml
cp gene_lock.md.example gene_lock.md

# Run in development mode
cargo run -- run

# Check system status
cargo run -- view
```

gRPC service default address: `127.0.0.1:50051`

---

## 📦 Project Architecture
This project is **modularly structured with 8 independent crates**, clean compilation and stable operation:
```
helix-mind/
├── crates/
│   ├── helix-mind-core         # Core types, errors and configuration
│   ├── helix-mind-storage      # Storage engine (SQLite + Topology cache)
│   ├── helix-mind-retrieval    # Multi-mode retrieval engine
│   ├── helix-mind-metabolism   # Knowledge metabolism and condensation
│   ├── helix-mind-federation   # Federated knowledge sharing
│   ├── helix-mind-api          # gRPC service layer
│   ├── helix-mind-reincarnation# Lifecycle management
│   └── helix-mind-cli          # Command line interface
├── proto/                      # Protocol definitions
└── Configuration / License files
```

---

## ✨ Personal Milestone
This is my **first Rust project**.  
Transitioning from a Python developer, I mastered Rust's ownership model and type system from scratch,
completed the modular design of 8 crates, and achieved full `cargo check / build` pass.

**Fear only vanishes when you conquer it yourself.**

---

## 📄 License
MIT License © Jasonmilk

---
*Helix-Mind doesn't just store memories—it nurtures new beginnings*
