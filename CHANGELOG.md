# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial workspace with 14 crates
- Core domain models (cm_core)
- JSON data importer (cm_data)
- Match engine with probabilistic simulation (cm_match)
- Game loop with 13 systems (cm_engine)
- Save system with compression and integrity (cm_save)
- CLI with new-game, advance-day, simulate-match (cm_cli)
- TUI skeleton with ratatui (cm_tui)
- REST API skeleton with axum (cm_api, cm_server)
- AI system stubs (cm_ai)
- Finance system stubs (cm_finance)
- Transfer system stubs (cm_transfers)
- Utility crates (cm_utils, cm_telemetry)

### Changed
- N/A

### Fixed
- N/A

## [0.1.0] - TBD

### Added
- MVP release with playable CLI simulation
