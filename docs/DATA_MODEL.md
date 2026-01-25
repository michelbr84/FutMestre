# CMRust Data Model

## Entity Relationships

```
Nation ──┬── Club ────── Player
         │     │         Staff
         │     │         Tactics
         │     │         Budget
         │     └── Stadium
         │
         └── Competition ── Fixtures
                         └── Table
```

## Core Entities

### Nation
- ID, Name, Continent
- Reputation (0-100)
- Youth Rating (talent production)

### Club
- ID, Name, Nation reference
- Stadium reference
- Reputation, Budget
- Player IDs, Staff IDs
- Tactics configuration

### Player
- ID, Names, Nationality
- Birth date, Position
- Attributes (Technical, Physical, Mental, GK)
- Value, Wage, Contract
- Morale, Fitness, Injury status

### Competition
- ID, Name, Type (League/Cup)
- Nation reference
- Team list, Fixtures, Table
- Reputation

### Contract
- Player/Staff reference
- Club reference
- Wage, Duration
- Clauses (release, bonus)

## Economy Types

### Money
- Stored as i64 cents
- Formatting with currency symbol
- Arithmetic operations

### Wage
- Per-week amount
- Conversion to monthly/yearly

### Budget
- Balance, Transfer budget, Wage budget
- Revenue/Expense tracking

## IDs

All IDs are string-based newtypes:
- `ClubId`, `PlayerId`, `NationId`
- `CompetitionId`, `MatchId`, `StaffId`
- Auto-implements Display, Hash, Eq
