# CLI Interface Contract

**Feature**: 001-poker-engine-cli
**Date**: 2026-01-09
**Version**: 1.0.0

## Overview

This document defines the CLI interface contract for the Poker GTO Strategy Engine. The CLI follows constitution requirements for UX Consistency (verb-noun naming, actionable errors, --help, --json support).

---

## Command Structure

```
fpe <COMMAND> [OPTIONS]

Commands:
  analyze    Calculate GTO strategy for a decision point
  help       Print help information
```

---

## `fpe analyze` Command

Calculate GTO strategy for a poker decision point.

### Synopsis

```
fpe analyze [OPTIONS] --hero <CARDS> --villain-range <RANGE> --pot <SIZE> --stack <SIZE>
```

### Required Arguments

| Argument | Type | Description | Example |
|----------|------|-------------|---------|
| `--hero <CARDS>` | String | Hero's hole cards in notation | `--hero AhKd` |
| `--villain-range <RANGE>` | String | Opponent's range in Equilab notation | `--villain-range "AA,KK,QQ,AKs"` |
| `--pot <SIZE>` | Float | Pot size in big blinds | `--pot 10.5` |
| `--stack <SIZE>` | Float | Effective stack size in big blinds | `--stack 100` |

### Optional Arguments

| Argument | Type | Default | Description | Example |
|----------|------|---------|-------------|---------|
| `--board <CARDS>` | String | "" (preflop) | Community cards | `--board Ts9s2h` |
| `--to-call <SIZE>` | Float | 0 | Amount to call in big blinds | `--to-call 3` |
| `--position <POS>` | Enum | IP | Hero position (IP/OOP) | `--position OOP` |
| `--iterations <N>` | Integer | 10000 | Solver iterations | `--iterations 50000` |
| `--json` | Flag | false | Output as JSON | `--json` |
| `--verbose` | Flag | false | Show solver progress | `--verbose` |
| `--help` | Flag | - | Show help | `--help` |

### Card Notation

**Single Card**: `[Rank][Suit]`
- Ranks: `2`, `3`, `4`, `5`, `6`, `7`, `8`, `9`, `T`, `J`, `Q`, `K`, `A`
- Suits: `h` (hearts), `d` (diamonds), `c` (clubs), `s` (spades)
- Examples: `Ah`, `Kd`, `Ts`, `2c`

**Multiple Cards**: Concatenated without spaces
- Hero cards: `AhKd` (Ace of hearts, King of diamonds)
- Board: `Ts9s2h` (Ten of spades, Nine of spades, Two of hearts)

### Range Notation

Supports Equilab-compatible format:

| Format | Meaning | Example |
|--------|---------|---------|
| Pair | Specific pair | `AA`, `KK`, `22` |
| Suited | Suited combo | `AKs`, `JTs` |
| Offsuit | Offsuit combo | `AKo`, `QJo` |
| Any | Any suitedness | `AK`, `QJ` |
| Plus | And higher | `TT+`, `AQs+` |
| Dash | Range | `TT-77`, `AQs-ATs` |
| Percentage | Top X% | `top 15%` |
| Weighted | Partial | `AA@50` |
| Combined | Multiple | `AA,KK,QQ,AKs` |

---

## Output Formats

### Human-Readable (Default)

```
Poker GTO Strategy Analysis
===========================

Input:
  Hero:          A♥ K♦
  Board:         T♠ 9♠ 2♥
  Villain Range: AA,KK,QQ,AKs (22 combos)
  Pot:           10.0 BB
  Stack:         100.0 BB
  Position:      IP

Strategy:
┌─────────────────┬───────────┬─────────────┐
│ Action          │ Frequency │ EV (BB)     │
├─────────────────┼───────────┼─────────────┤
│ Bet 75% pot     │ 45.2%     │ +2.34       │
│ Check           │ 32.1%     │ +1.87       │
│ Bet 33% pot     │ 22.7%     │ +1.56       │
└─────────────────┴───────────┴─────────────┘

Solver: 10000 iterations, convergence: 0.0012
```

### JSON Output (`--json`)

```json
{
  "input": {
    "hero": "AhKd",
    "board": "Ts9s2h",
    "villain_range": "AA,KK,QQ,AKs",
    "villain_combos": 22,
    "pot": 10.0,
    "stack": 100.0,
    "to_call": 0.0,
    "position": "IP"
  },
  "strategy": [
    {
      "action": "bet_75_pot",
      "action_display": "Bet 75% pot",
      "frequency": 0.452,
      "ev": 2.34
    },
    {
      "action": "check",
      "action_display": "Check",
      "frequency": 0.321,
      "ev": 1.87
    },
    {
      "action": "bet_33_pot",
      "action_display": "Bet 33% pot",
      "frequency": 0.227,
      "ev": 1.56
    }
  ],
  "solver": {
    "iterations": 10000,
    "convergence": 0.0012
  }
}
```

---

## Exit Codes

| Code | Meaning | When |
|------|---------|------|
| 0 | Success | Analysis completed successfully |
| 1 | Invalid input | Invalid cards, range, or parameters |
| 2 | Solver error | Solver failed to converge or internal error |

---

## Error Messages

All error messages follow the constitution requirement for actionable feedback.

### Invalid Card

```
Error: Invalid card 'Xh'

Expected format: [Rank][Suit]
  Ranks: 2, 3, 4, 5, 6, 7, 8, 9, T, J, Q, K, A
  Suits: h (hearts), d (diamonds), c (clubs), s (spades)

Example: --hero AhKd
```

### Invalid Range

```
Error: Invalid range notation 'AAA'

Expected Equilab format. Examples:
  Pairs:    AA, KK, QQ
  Suited:   AKs, JTs
  Offsuit:  AKo, QJo
  Plus:     TT+, AQs+
  Range:    TT-77
  Combined: AA,KK,QQ,AKs

Example: --villain-range "AA,KK,QQ,AKs"
```

### Duplicate Card

```
Error: Duplicate card 'Ah'

Card Ah appears in both hero hand and board.
Each card can only appear once across all inputs.
```

### Invalid Game State

```
Error: Invalid game state

Pot size (500 BB) exceeds effective stack (100 BB).
This is physically impossible in a poker game.
```

### Missing Required Parameter

```
Error: Missing required argument '--hero'

Required arguments:
  --hero <CARDS>           Hero's hole cards
  --villain-range <RANGE>  Opponent's range
  --pot <SIZE>            Pot size in BB
  --stack <SIZE>          Effective stack in BB

Run 'fpe analyze --help' for more information.
```

---

## Examples

### Basic Preflop Analysis

```bash
fpe analyze \
  --hero AhKd \
  --villain-range "TT+,AQs+,AKo" \
  --pot 1.5 \
  --stack 100 \
  --to-call 1
```

### Postflop Analysis with JSON Output

```bash
fpe analyze \
  --hero AhKd \
  --board Ts9s2h \
  --villain-range "AA,KK,QQ,JJ,TT,AKs" \
  --pot 10 \
  --stack 100 \
  --position IP \
  --json
```

### High-Precision Analysis

```bash
fpe analyze \
  --hero QsQh \
  --villain-range "top 20%" \
  --pot 3.5 \
  --stack 50 \
  --to-call 2 \
  --iterations 100000 \
  --verbose
```

---

## Help Output

```
Poker GTO Strategy Engine

Calculate Nash equilibrium strategies for poker decision points.

Usage: fpe analyze [OPTIONS] --hero <CARDS> --villain-range <RANGE> --pot <SIZE> --stack <SIZE>

Arguments:
      --hero <CARDS>           Hero's hole cards (e.g., AhKd)
      --villain-range <RANGE>  Opponent's range in Equilab notation
      --pot <SIZE>            Pot size in big blinds
      --stack <SIZE>          Effective stack size in big blinds

Options:
      --board <CARDS>         Community cards (e.g., Ts9s2h)
      --to-call <SIZE>        Amount to call in big blinds [default: 0]
      --position <POS>        Hero position: IP or OOP [default: IP]
      --iterations <N>        Solver iterations [default: 10000]
      --json                  Output as JSON
      --verbose               Show solver progress
  -h, --help                  Print help
  -V, --version               Print version

Examples:
  fpe analyze --hero AhKd --villain-range "AA,KK,AKs" --pot 10 --stack 100
  fpe analyze --hero QsQh --board Ts9s2h --villain-range "top 15%" --pot 20 --stack 80 --json

For more information, see https://github.com/fpe/poker-engine
```
