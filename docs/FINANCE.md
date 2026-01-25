# CMRust Finance System

## Overview

The finance system tracks all monetary transactions for clubs.

## Revenue Sources

| Source | Frequency | Calculation |
|--------|-----------|-------------|
| Matchday | Per match | attendance × ticket_price |
| TV Rights | Monthly | league_position × base_amount |
| Sponsorship | Monthly | reputation × multiplier |
| Prize Money | Per competition | position_bonus |
| Player Sales | On transfer | fee - agent_cut |

## Expenses

| Expense | Frequency | Calculation |
|---------|-----------|-------------|
| Wages | Weekly | sum(player_wages + staff_wages) |
| Stadium | Monthly | capacity × maintenance_rate |
| Training | Monthly | facilities × staff_count |
| Youth | Monthly | academy_level × base_cost |
| Debt Interest | Monthly | debt × interest_rate |

## Budget Management

```rust
pub struct Budget {
    pub balance: Money,           // Current cash
    pub transfer_budget: Money,   // Available for signings
    pub wage_budget: Money,       // Monthly wage cap
}
```

## Financial Fair Play (FFP)

Simplified rules:
- Revenue must cover wages (wage ratio < 70%)
- No more than 30% loss over 3 years
- Transfer spending balanced by sales

## Ledger System

All transactions recorded:
```rust
pub struct LedgerEntry {
    pub date: NaiveDate,
    pub category: TransactionCategory,
    pub amount: Money,
    pub description: String,
}
```
