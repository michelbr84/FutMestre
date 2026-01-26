# CM Rust ⚽

A **Championship Manager 01/02-style** football manager simulator written in Rust.

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-300%2B%20passing-green.svg)](#-testing)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## 🎮 Quick Start

```bash
# Clone and build
git clone https://github.com/michelbr84/cmrust.git
cd cmrust
cargo build --release

# Simulate a match
cargo run -p cm_cli -- simulate-match --home LIV --away ARS --seed 42

# Create a new game
cargo run -p cm_cli -- new-game --club LIV --manager "Your Name"

# Advance simulation
cargo run -p cm_cli -- advance-day --days 7
```

## 🏗️ Architecture

14-crate workspace following domain-driven design:

```
crates/
├── cm_utils        # Utilities (fs, hashing, RNG, time)
├── cm_telemetry    # Logging, tracing, and metrics
├── cm_core         # Domain models (world, economy, IDs)
├── cm_data         # Data layer (JSON importer, SQLite, queries)
├── cm_match        # Match engine (tick simulation, events, ratings)
├── cm_ai           # AI systems (matchday, transfers, squad builder, press)
├── cm_finance      # Financial simulation (wages, sponsorship, FFP)
├── cm_transfers    # Transfer market (valuation, negotiation, contracts)
├── cm_save         # Save/load with gzip + SHA256 verification
├── cm_engine       # Game loop with 13+ systems (morale, training, inbox)
├── cm_cli          # CLI commands (new-game, advance-day, simulate-match)
├── cm_api          # REST API DTOs and routes
├── cm_server       # Axum HTTP server
└── cm_tui          # Ratatui terminal UI
```

## 📊 Features

### ✅ Implemented
- **Modern 4K GUI**: Tauri-based interface with Glassmorphism design and 4K support.
### ✅ Implemented
- **Modern 4K GUI**: Tauri-based interface with Glassmorphism design and 4K support.
- **CM-Style Layout**: Authentic Top Bar + Sidebar + Content Split View + Bottom Actions layout.
- **Backend Integrations**: Full Rust <> JS bridge for Player Data, Attributes, Simulation, and Saving.
- **Squad Screen**: Interactive DataGrid with filters, position badges, and player profiles.
- **Match Engine**: Tick-by-tick probabilistic simulation (Stubbed).
- **World Model**: Nations, Clubs, Players (with 40+ attributes), Staff, Competitions, Stadiums, Referees
- **AI Systems**: Matchday lineup selection, transfer decision-making, squad analysis, press conferences
- **Data Import**: JSON-based world loader with auto-generated defaults + SQLite persistence
- **Save System**: Compressed saves with SHA256 integrity verification
- **CLI**: Commands for match simulation, game creation, day advancement
- **Game Loop**: Day-by-day processing with morale, training, injury, and financial systems
- **Transfer System**: Bid evaluation, contract negotiations, player valuation

### 🔨 In Progress
- [ ] Competition fixture generation and league tables
- [ ] TUI screens (squad management, tactics, inbox, match day)
- [ ] REST API endpoints
- [ ] Training and youth academy progression

## 🧪 Testing

The project has **300+ unit tests** covering all major systems:

```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p cm_core      # Core domain models
cargo test -p cm_match     # Match engine
cargo test -p cm_ai        # AI systems
cargo test -p cm_transfers # Transfer system
```

## 📁 Project Structure

```
cmrust/
├── Cargo.toml          # Workspace manifest
├── crates/             # All 14 crates
├── assets/data/        # Game data (JSON)
├── saves/              # Save game directory
├── .cargo/config.toml  # Cargo configuration
├── rustfmt.toml        # Formatting rules
└── clippy.toml         # Lint rules
```

## 🎯 Roadmap

| Phase | Status | Description |
|-------|--------|-------------|
| M0 | ✅ | Setup repo/workspace, lint/format |
| M1 | ✅ | cm_core (IDs, entities, rules) |
| M2 | ✅ | cm_data (JSON schema, SQLite, importer) |
| M3 | ✅ | cm_engine (loop, 13+ systems, time) |
| M4 | ✅ | cm_match (ticks, events, ratings) |
| M5 | 🔨 | Competitions (fixtures, tables) |
| M6 | ✅ | cm_transfers (valuation, negotiation) |
| M7 | 🔨 | cm_finance (wages, sponsorship, FFP) |
| M8 | ✅ | cm_ai (matchday, transfers, press, squad) |
| M9 | ✅ | cm_save (snapshot, compression) |
| M10 | 🔨 | cm_tui (screens, widgets) |
| M11 | ✅ | cm_cli (commands) |
| M12 | 🔨 | cm_api + cm_server |
| M13 | ✅ | Tests (300+ passing) |
| M14 | ⬜ | Docs + release + docker |

**Legend**: ✅ Complete | 🔨 In Progress | ⬜ Not Started

## 📝 License

MIT License - see [LICENSE](LICENSE) for details.

## 🤝 Contributing

Contributions welcome! Please check the roadmap above for areas that need work.
