# CMRust Transfer System

## Overview

The transfer system handles player movement between clubs.

## Valuation

Player value calculated from:
- Base: Current ability rating × 1M
- Age modifier: Peak (24-29) = 1.2×, Young (<21) = 0.8×, Old (>32) = 0.5×
- Contract: Last year = 0.7×, 3+ years = 1.1×
- Reputation: Club rep affects perceived value
- Form: Recent performance ±20%

```rust
pub fn calculate_value(player: &Player, world: &World) -> Money {
    let base = player.ability as i64 * 1_000_000;
    let age_mod = age_modifier(player.age());
    let contract_mod = contract_modifier(&player.contract);
    Money::from_major(base * age_mod * contract_mod)
}
```

## Transfer Windows

| Window | Period |
|--------|--------|
| Summer | Jul 1 - Aug 31 |
| Winter | Jan 1 - Jan 31 |

## Negotiation Flow

1. **Enquiry**: Initial interest check
2. **Bid**: Formal offer to selling club
3. **Counter**: Seller may counter-offer
4. **Agreement**: Clubs agree on fee
5. **Contract**: Player negotiates personal terms
6. **Complete**: Transfer registered

## Loan System

- Standard loan: No fee, wages shared
- Loan with option: Purchase clause at end
- Loan with obligation: Must buy if conditions met

## Clauses

- Release clause: Fixed buyout price
- Sell-on clause: % of future sale
- Appearance bonus: Fee per X games
- Goal bonus: Fee per Y goals
