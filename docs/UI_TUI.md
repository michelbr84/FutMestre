# CMRust TUI Guide

## Overview

The TUI provides a terminal-based interface using ratatui.

## Navigation

| Key | Action |
|-----|--------|
| ↑/↓ | Navigate lists |
| ←/→ | Switch tabs |
| Enter | Select/Confirm |
| Esc | Back/Cancel |
| Q | Quit |
| Space | Toggle selection |
| Tab | Next field |

## Screens

### Main Menu
- New Game
- Load Game
- Settings
- Quit

### Inbox
- Message list with categories
- Read/Unread filtering
- Quick actions

### Squad
- Player list with stats
- Position grouping
- Injury/Suspension indicators

### Tactics
- Formation selector
- Mentality slider
- Player positions

### Match Day
- Live score
- Match events
- Substitution panel

### Transfers
- Shortlist
- Active negotiations
- Budget display

### Finance
- Income/Expense summary
- Wage overview
- Projections

## Theme Configuration

Edit `assets/ui/theme.toml`:
```toml
[colors]
background = "#1a1a2e"
foreground = "#eaeaea"
accent = "#e94560"
success = "#0f3460"
warning = "#f39c12"
error = "#e74c3c"

[styles]
header = { bold = true }
selected = { bg = "accent", fg = "background" }
```

## Running

```bash
cargo run -p cm_tui
```
