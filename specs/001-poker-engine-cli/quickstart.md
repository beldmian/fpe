# Quickstart: Poker GTO Strategy Engine

**Feature**: 001-poker-engine-cli
**Date**: 2026-01-09

## Prerequisites

- Rust 1.75+ (stable)
- Cargo (comes with Rust)

## Installation

```bash
# Clone the repository
git clone https://github.com/fpe/poker-engine.git
cd poker-engine

# Build release version
cargo build --release

# Binary will be at ./target/release/fpe
```

## Quick Examples

### 1. Basic Preflop Decision

You have AK offsuit, facing a raise. What's the GTO response?

```bash
fpe analyze \
  --hero AhKd \
  --villain-range "TT+,AQs+,AKo" \
  --pot 3.5 \
  --stack 100 \
  --to-call 2
```

Output:
```
Strategy:
┌─────────────────┬───────────┬─────────────┐
│ Action          │ Frequency │ EV (BB)     │
├─────────────────┼───────────┼─────────────┤
│ Raise (3x)      │ 62.3%     │ +1.45       │
│ Call            │ 37.7%     │ +0.89       │
│ Fold            │ 0.0%      │ +0.00       │
└─────────────────┴───────────┴─────────────┘
```

### 2. Postflop C-Bet Decision

You raised preflop with AK, villain called. Flop is T92 rainbow. Should you c-bet?

```bash
fpe analyze \
  --hero AhKd \
  --board Ts9c2h \
  --villain-range "22-99,ATs+,KJs+,QJs,JTs" \
  --pot 7 \
  --stack 95 \
  --position IP
```

### 3. Facing a Bet

You have QQ on a dry board, villain bets half pot.

```bash
fpe analyze \
  --hero QsQh \
  --board Kc7d2s \
  --villain-range "AA,KK,AK,KQ,KJ,KT,77,22,AQ" \
  --pot 15 \
  --stack 85 \
  --to-call 7.5 \
  --position OOP
```

### 4. JSON Output for Scripting

```bash
fpe analyze \
  --hero AhKd \
  --villain-range "AA,KK,QQ" \
  --pot 10 \
  --stack 100 \
  --json
```

Output:
```json
{
  "input": {
    "hero": "AhKd",
    "villain_range": "AA,KK,QQ",
    "pot": 10.0,
    "stack": 100.0
  },
  "strategy": [
    {"action": "fold", "frequency": 0.0, "ev": 0.0},
    {"action": "call", "frequency": 0.45, "ev": -1.2},
    {"action": "raise_3x", "frequency": 0.55, "ev": 0.8}
  ]
}
```

### 5. High-Precision Analysis

For important spots, increase iterations:

```bash
fpe analyze \
  --hero AsAh \
  --board KsQsJs \
  --villain-range "top 25%" \
  --pot 50 \
  --stack 150 \
  --iterations 100000 \
  --verbose
```

## Range Notation Guide

| Format | Example | Meaning |
|--------|---------|---------|
| Pair | `AA` | Pocket Aces (6 combos) |
| Suited | `AKs` | Ace-King suited (4 combos) |
| Offsuit | `AKo` | Ace-King offsuit (12 combos) |
| Plus | `TT+` | Tens or better |
| Range | `TT-77` | Tens through Sevens |
| Percentage | `top 15%` | Top 15% of hands |
| Combined | `AA,KK,AKs` | Multiple hand types |

## Card Notation

- **Ranks**: `2 3 4 5 6 7 8 9 T J Q K A`
- **Suits**: `h` (hearts), `d` (diamonds), `c` (clubs), `s` (spades)
- **Example**: `Ah` = Ace of hearts, `Ts` = Ten of spades

## Common Errors

### Invalid Cards
```
Error: Invalid card 'Xh'
```
Fix: Use valid ranks (2-9, T, J, Q, K, A) and suits (h, d, c, s).

### Duplicate Cards
```
Error: Duplicate card 'Ah'
```
Fix: Ensure no card appears in both hero hand and board.

### Invalid Range
```
Error: Invalid range notation 'AAA'
```
Fix: Use Equilab format (AA, AKs, TT+, etc.).

## Performance Tips

1. **Preflop**: Default 10,000 iterations is usually sufficient
2. **Postflop**: Consider 50,000+ iterations for complex spots
3. **River**: Fastest to solve (no future cards)
4. **Flop**: Slowest (most potential runouts)

## Next Steps

- Read the full [CLI Interface Contract](./contracts/cli-interface.md)
- See [Data Model](./data-model.md) for entity details
- Check [Research](./research.md) for algorithm background
