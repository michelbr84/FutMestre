# CM Rust ⚽

A **CM01/02-style** football manager simulator written in Rust.

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## 🎮 Quick Start

```bash
# Clone and build
git clone https://github.com/your-repo/cmrust.git
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
├── cm_telemetry    # Logging and tracing
├── cm_core         # Domain models (world, economy, sim)
├── cm_data         # Data loading (JSON importer, SQLite, repos)
├── cm_match        # Match engine (probabilistic simulation)
├── cm_ai           # AI systems (9 modules)
├── cm_finance      # Financial simulation (9 modules)
├── cm_transfers    # Transfer market (9 modules)
├── cm_save         # Save/load with gzip + SHA256
├── cm_engine       # Game loop with 13 systems
├── cm_cli          # CLI (new-game, advance-day, simulate-match)
├── cm_api          # REST API DTOs and routes
├── cm_server       # Axum HTTP server
└── cm_tui          # Ratatui terminal UI
```

## 📊 Features

### ✅ Implemented (Skeleton + Core Logic)
- [x] **Match Engine**: Tick-by-tick probabilistic simulation with highlights
- [x] **World Model**: Nations, Clubs, Players, Staff, Competitions, Stadiums
- [x] **Data Import**: JSON-based world loader with auto-generated defaults
- [x] **Save System**: Compressed saves with SHA256 integrity verification
- [x] **CLI**: 3 commands (new-game, advance-day, simulate-match)
- [x] **Game Loop**: Day-by-day processing with 13 game systems

### 🔨 Stub Implementations (Ready for Extension)
- [ ] Transfer negotiations and agent system
- [ ] Financial simulation (wages, sponsorship, FFP)
- [ ] AI manager personalities and decision-making
- [ ] TUI screens (squad, tactics, inbox, match day)
- [ ] REST API endpoints
- [ ] Competition fixture generation
- [ ] Training and youth academy

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p cm_core
cargo test -p cm_match
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

Based on ~660h development estimate:

| Phase | Status | Description |
|-------|--------|-------------|
| M0 | ✅ | Setup repo/workspace, lint/format |
| M1 | ✅ | cm_core (ids, entities, rules) |
| M2 | ✅ | cm_data (JSON schema, importer) |
| M3 | ✅ | cm_engine (loop, systems, time) |
| M4 | ✅ | cm_match v1 (ticks, events, ratings) |
| M5 | 🔨 | Competitions (fixtures, tables) |
| M6 | 🔨 | cm_transfers v1 (valuation, negotiation) |
| M7 | 🔨 | cm_finance v1 (wages, sponsorship) |
| M8 | 🔨 | cm_ai v1 (personalities, tactics) |
| M9 | ✅ | cm_save v1 (snapshot, compression) |
| M10 | 🔨 | cm_tui v1 (screens, widgets) |
| M11 | ✅ | cm_cli v1 (commands) |
| M12 | 🔨 | cm_api + cm_server v1 |
| M13 | ⬜ | Tests + benches |
| M14 | ⬜ | Docs + release + docker |

**Legend**: ✅ Complete | 🔨 Skeleton/Stub | ⬜ Not Started

## 📝 License

MIT License - see [LICENSE](LICENSE) for details.

## 🤝 Contributing

Contributions welcome! Please check the roadmap above for areas that need work.
# cmrust
