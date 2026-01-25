# CMRust Match Engine

## Overview

The match engine simulates football matches using a probabilistic tick-based system.

## Simulation Flow

```
MatchInput → simulate_match() → MatchResult
    │                              │
    ├── TeamStrength              ├── home_goals
    ├── Tactics                   ├── away_goals
    ├── Weather                   ├── highlights
    └── Seed                      └── events
```

## Tick System

Each match is divided into ticks (~1 minute each):
1. Determine which team has possession
2. Calculate action probabilities based on strength
3. Generate events (shots, passes, fouls)
4. Apply outcomes (goals, cards, injuries)
5. Update player fatigue and ratings

## Team Strength Calculation

```
attack = avg(finishing, dribbling, pace) * tactics_mod
defense = avg(tackling, positioning, strength) * tactics_mod
midfield = avg(passing, stamina, decisions) * tactics_mod
```

## Event Types

- **Goal**: Successful shot
- **Yellow Card**: Foul with booking
- **Red Card**: Serious foul / second yellow
- **Injury**: Player injured during play
- **Substitution**: Tactical change

## Tactics Influence

| Mentality | Attack Bonus | Defense Bonus |
|-----------|--------------|---------------|
| Attacking | +15% | -10% |
| Balanced | 0% | 0% |
| Defensive | -10% | +15% |
| Counter | +5% | +5% |

## Determinism

With the same seed, match results are 100% reproducible:
```rust
let input = MatchInput { seed: Some(42), ... };
let r1 = simulate_match(&input);
let r2 = simulate_match(&input);
assert_eq!(r1.home_goals, r2.home_goals);
```
