# CMRust AI System

## Overview

The AI system controls CPU-managed clubs, making decisions for tactics, transfers, and more.

## Modules

### Personalities (`personalities.rs`)
Manager personality types that influence decisions:
- **Aggressive**: High transfer activity, attacking tactics
- **Conservative**: Low spending, defensive focus
- **Youth Developer**: Academy priority, patience with young players
- **Pragmatic**: Balanced approach, result-focused

### Tactics AI (`tactics_ai.rs`)
- Formation selection based on squad
- Mentality adjustment during matches
- Set piece assignments

### Transfer AI (`transfer_ai.rs`)
- Player valuation assessment
- Target list generation
- Bid/offer logic
- Contract negotiation

### Matchday AI (`matchday_ai.rs`)
- Starting XI selection
- In-match substitutions
- Tactical tweaks based on score

### Board AI (`board_ai.rs`)
- Objective setting
- Manager evaluation
- Budget allocation

### Scouting AI (`scouting_ai.rs`)
- Region exploration
- Report generation
- Wonderkid identification

## Decision Flow

```
Game Day Start
    │
    ├── Evaluate squad fitness
    ├── Check injuries/suspensions
    ├── Select formation
    ├── Choose starting XI
    └── Set mentality

During Match
    │
    ├── Monitor score
    ├── Evaluate performance
    └── Make substitutions

End of Day
    │
    ├── Review results
    ├── Update morale
    └── Plan transfers
```
