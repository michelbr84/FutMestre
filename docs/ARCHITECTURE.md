# CMRust Architecture

## Overview

CMRust is a CM01/02-style football manager simulator built in Rust using a modular, layered architecture.

## Crate Dependency Graph

```
                    ┌──────────────┐
                    │   cm_utils   │
                    │ cm_telemetry │
                    └──────┬───────┘
                           │
              ┌────────────┴────────────┐
              ▼                         ▼
        ┌──────────┐             ┌──────────┐
        │ cm_core  │◀────────────│ cm_data  │
        └────┬─────┘             └──────────┘
             │
    ┌────────┼────────┬────────────┬────────────┐
    ▼        ▼        ▼            ▼            ▼
┌────────┐ ┌────────┐ ┌──────────┐ ┌──────────┐ ┌────────┐
│cm_match│ │ cm_ai  │ │cm_finance│ │cm_transfer│ │cm_save │
└────────┘ └────────┘ └──────────┘ └──────────┘ └────────┘
    │          │           │            │           │
    └──────────┴───────────┴────────────┴───────────┘
                           │
                    ┌──────┴──────┐
                    │  cm_engine  │
                    └──────┬──────┘
                           │
         ┌─────────────────┼─────────────────┐
         ▼                 ▼                 ▼
    ┌─────────┐       ┌─────────┐       ┌─────────┐
    │ cm_cli  │       │ cm_tui  │       │cm_server│
    └─────────┘       └─────────┘       │ cm_api  │
                                        └─────────┘
```

## Layer Descriptions

### Foundation Layer
- **cm_utils**: Common utilities (fs, hashing, RNG, time helpers)
- **cm_telemetry**: Logging and tracing initialization

### Domain Layer
- **cm_core**: Core domain models (World, Economy, Simulation types)
- **cm_data**: Data loading, repositories, SQLite adapter

### Service Layer
- **cm_match**: Match engine (probabilistic tick simulation)
- **cm_ai**: AI systems (tactics, transfers, board)
- **cm_finance**: Financial simulation (wages, tickets, sponsorship)
- **cm_transfers**: Transfer market (valuation, negotiation)
- **cm_save**: Save/load system (compression, integrity)

### Orchestration Layer
- **cm_engine**: Game loop, systems coordination, state management

### Presentation Layer
- **cm_cli**: Command-line interface
- **cm_tui**: Terminal UI (ratatui)
- **cm_api**: REST API DTOs and routes
- **cm_server**: HTTP server (axum)

## Key Design Decisions

1. **Type-safe IDs**: All entity IDs use newtype wrappers
2. **Money as cents**: All monetary values stored as i64 cents
3. **Seedable RNG**: ChaCha8 for deterministic simulation
4. **JSON-first data**: Assets in JSON for easy editing
5. **Gzip saves**: Compressed saves with SHA256 integrity
