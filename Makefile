# Makefile - Alternative to justfile for those who prefer make

.PHONY: all fmt clippy test build release clean doc ci

all: fmt clippy test

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

test:
	cargo test --workspace

build:
	cargo build --workspace

release:
	cargo build --workspace --release

clean:
	cargo clean

doc:
	cargo doc --workspace --no-deps --open

run-tui:
	cargo run -p cm_tui

run-cli:
	cargo run -p cm_cli -- --help

run-server:
	cargo run -p cm_server

new-game:
	cargo run -p cm_cli -- new-game --club LIV --manager "Manager"

simulate-match:
	cargo run -p cm_cli -- simulate-match --home LIV --away ARS --seed 42

ci: fmt-check clippy test
