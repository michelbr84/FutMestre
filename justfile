# justfile - Command runner for CMRust
# Install: cargo install just
# Usage: just <recipe>

# Default recipe
default:
    @just --list

# Format all code
fmt:
    cargo fmt --all

# Check formatting
fmt-check:
    cargo fmt --all -- --check

# Run clippy lints
clippy:
    cargo clippy --workspace --all-targets -- -D warnings

# Run all tests
test:
    cargo test --workspace

# Run tests with output
test-verbose:
    cargo test --workspace -- --nocapture

# Build debug
build:
    cargo build --workspace

# Build release
build-release:
    cargo build --workspace --release

# Run TUI
run-tui:
    cargo run -p cm_tui

# Run CLI help
run-cli:
    cargo run -p cm_cli -- --help

# Create new game
new-game club="LIV" manager="Manager":
    cargo run -p cm_cli -- new-game --club {{club}} --manager "{{manager}}"

# Simulate a match
simulate-match home="LIV" away="ARS" seed="42":
    cargo run -p cm_cli -- simulate-match --home {{home}} --away {{away}} --seed {{seed}}

# Advance days
advance-day days="7":
    cargo run -p cm_cli -- advance-day --days {{days}}

# Run server
run-server:
    cargo run -p cm_server

# Run benchmarks
bench:
    cargo bench --workspace

# Check for security advisories
audit:
    cargo deny check advisories

# Check licenses
licenses:
    cargo deny check licenses

# Full deny check
deny:
    cargo deny check

# Generate documentation
doc:
    cargo doc --workspace --no-deps --open

# Clean build artifacts
clean:
    cargo clean

# Full CI check (fmt, clippy, test)
ci: fmt-check clippy test

# Watch and run tests
watch-test:
    cargo watch -x "test --workspace"
