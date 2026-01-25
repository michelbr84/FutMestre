#!/bin/bash
# dev_seed.sh - Quick seed for development

set -e

echo "🌱 Seeding development environment..."

# Build if needed
cargo build -p cm_cli

# Create a new game
cargo run -p cm_cli -- new-game \
    --data-dir assets/data \
    --out saves/dev_game.cmsave \
    --club LIV \
    --manager "Dev Manager" \
    --start-date 2001-07-01

echo "✅ Development game created at saves/dev_game.cmsave"
