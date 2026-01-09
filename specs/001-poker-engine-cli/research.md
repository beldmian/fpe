# Research: Poker GTO Strategy Engine

**Feature**: 001-poker-engine-cli
**Date**: 2026-01-09
**Status**: Complete

## Overview

This document captures research findings and design decisions for implementing a Poker GTO Strategy Engine in Rust. All "NEEDS CLARIFICATION" items from the Technical Context have been resolved.

---

## Decision 1: GTO Solver Algorithm

**Decision**: Monte Carlo Counterfactual Regret Minimization (MCCFR)

**Rationale**:
- CFR is the proven algorithm for computing Nash equilibrium in poker (used by Libratus, Pluribus, Cepheus)
- Vanilla CFR requires full game tree traversal per iteration - too slow for complex games
- MCCFR samples single trajectories instead of exhaustive traversal, dramatically reducing computation
- For single decision point analysis, MCCFR converges in 10,000-100,000 iterations (minutes, not hours)
- Provides theoretical convergence guarantee to Nash equilibrium

**Alternatives Considered**:
| Alternative | Why Rejected |
|-------------|--------------|
| Vanilla CFR | Too slow - requires full tree traversal per iteration |
| Linear Programming | Doesn't scale well with game complexity |
| Deep CFR | Over-engineered for single decision point; requires neural network training |
| Pre-computed lookup tables | Only works for specific scenarios (e.g., push/fold); not generalizable |

**Implementation Notes**:
- Use Chance Sampling variant of MCCFR for preflop scenarios
- Implement regret matching for strategy updates
- Store cumulative regrets and strategy sums per information set
- Convergence criterion: average strategy stabilizes or max iterations reached

---

## Decision 2: Hand Evaluation Library

**Decision**: `pokers` crate (v0.7.0)

**Rationale**:
- Best combination of features: 7-card evaluation + equity calculation + range parsing
- Performance: ~20-25ns per hand evaluation (50M hands/second)
- Built-in Equilab-like range notation support ("AA,KK,AKs", "top 15%")
- Multithreaded equity calculation for range vs range
- Based on OMPEval (proven C++ poker evaluator)
- Actively maintained fork of rust_poker

**Alternatives Considered**:
| Alternative | Why Rejected |
|-------------|--------------|
| rust_poker | Less actively maintained; pokers is improved fork |
| rs_poker | Good but beta; has CFRAgent but we want custom solver |
| poker_eval | Lower-level; would need to build range parsing |
| Hand-rolled evaluator | No benefit; existing crates are highly optimized |

**Key Features Used**:
- `HandRange::from_string()` for range parsing
- `get_hand_rank()` for 7-card evaluation
- Equity calculation for heads-up scenarios
- Card/Hand types for representation

---

## Decision 3: CLI Framework

**Decision**: `clap` v4 with derive macros

**Rationale**:
- De facto standard for Rust CLI applications
- Derive macros provide ergonomic argument definition
- Automatic help generation meets FR-016
- Subcommand support for extensibility
- structopt was merged into clap v3+, so clap is the unified choice

**Alternatives Considered**:
| Alternative | Why Rejected |
|-------------|--------------|
| structopt | Deprecated; merged into clap v3+ |
| argh | Less popular; fewer features |
| pico-args | Too minimal for this use case |

**CLI Design**:
```
fpe analyze [OPTIONS]
  --hero <CARDS>         Hero hole cards (e.g., "AhKd")
  --board <CARDS>        Community cards (e.g., "Ts9s2h")
  --villain-range <RANGE> Opponent range (e.g., "AA,KK,QQ,AKs")
  --pot <SIZE>           Pot size in big blinds
  --stack <SIZE>         Effective stack size in big blinds
  --position <POS>       Hero position (IP/OOP)
  --iterations <N>       Solver iterations [default: 10000]
  --json                 Output as JSON instead of table
  --help                 Show help information
```

---

## Decision 4: Output Formatting

**Decision**: `tabled` for human-readable + `serde_json` for JSON

**Rationale**:
- Constitution requires human-readable default + JSON flag (UX Consistency III)
- tabled provides professional ASCII table output from Rust structs
- serde_json is the standard for JSON serialization
- Both integrate well with serde derive macros

**Output Format**:

Human-readable (default):
```
┌─────────────┬───────────┬─────────┐
│ Action      │ Frequency │ EV (BB) │
├─────────────┼───────────┼─────────┤
│ Raise 3x    │ 45.2%     │ +1.23   │
│ Call        │ 32.1%     │ +0.87   │
│ Fold        │ 22.7%     │ +0.00   │
└─────────────┴───────────┴─────────┘
```

JSON (`--json` flag):
```json
{
  "input": {
    "hero": "AhKd",
    "board": "Ts9s2h",
    "villain_range": "AA,KK,QQ,AKs",
    "pot": 10,
    "stack": 100
  },
  "strategy": [
    {"action": "raise_3x", "frequency": 0.452, "ev": 1.23},
    {"action": "call", "frequency": 0.321, "ev": 0.87},
    {"action": "fold", "frequency": 0.227, "ev": 0.0}
  ],
  "iterations": 10000,
  "convergence": 0.001
}
```

---

## Decision 5: Error Handling

**Decision**: Custom error enum with `thiserror` + actionable messages

**Rationale**:
- Constitution requires actionable error messages (UX Consistency III)
- thiserror provides ergonomic error enum definition
- Errors must identify specific invalid parameter and expected format (SC-005)

**Error Categories**:
- `InvalidCard` - Invalid card notation (expected format: Ah, Kd, Ts)
- `InvalidRange` - Invalid range syntax (expected format: AA,KK,AKs)
- `InvalidGameState` - Impossible game state (e.g., pot > total chips)
- `MissingParameter` - Required parameter not provided
- `SolverError` - Internal solver failure

**Example Error Output**:
```
Error: Invalid card 'XY'
Expected format: Rank + Suit (e.g., Ah, Kd, Ts, 2c)
Valid ranks: 2-9, T, J, Q, K, A
Valid suits: h (hearts), d (diamonds), c (clubs), s (spades)
```

---

## Decision 6: Bet Sizing Model

**Decision**: Discrete bet sizing options (33%, 50%, 75%, 100% pot)

**Rationale**:
- Spec assumption states bet sizings are simplified (not continuous)
- Matches real-world solver conventions (GTO Wizard, PioSOLVER)
- Reduces game tree complexity significantly
- Allows clear strategy representation

**Available Actions**:
| Street | Available Actions |
|--------|-------------------|
| Preflop | Fold, Call, Raise (2.5x, 3x, All-in) |
| Postflop facing bet | Fold, Call, Raise (min, 50%, pot, all-in) |
| Postflop to act | Check, Bet (33%, 50%, 75%, 100% pot) |

---

## Decision 7: Range Notation Support

**Decision**: Equilab-compatible notation

**Rationale**:
- Industry standard notation (Equilab, Flopzilla, GTO Wizard)
- pokers crate has built-in support
- Users are familiar with this format (spec assumption)

**Supported Formats**:
| Format | Meaning |
|--------|---------|
| `AA` | Pair of aces (6 combos) |
| `AKs` | Ace-king suited (4 combos) |
| `AKo` | Ace-king offsuit (12 combos) |
| `AK` | Ace-king any suitedness (16 combos) |
| `TT+` | Tens or better (60 combos) |
| `TT-77` | Tens through sevens (24 combos) |
| `AQs+` | AKs and AQs (8 combos) |
| `top 15%` | Top 15% of hands by equity |
| `AA@50` | 50% of AA combos (weighted) |
| `AA,KK,AKs` | Multiple ranges combined |

---

## Decision 8: Async Runtime

**Decision**: Tokio (optional, only if needed for parallelization)

**Rationale**:
- User specified Tokio as preferred async runtime
- Main computation is CPU-bound (MCCFR iterations)
- May use for parallel equity calculations across threads
- Not required for initial MVP; can add later if performance needs it

**Initial Approach**: Synchronous single-threaded implementation
**Future Enhancement**: Rayon for CPU parallelism or Tokio for async I/O if needed

---

## Dependencies Summary

```toml
[dependencies]
# Core
pokers = "0.7"            # Hand evaluation + equity + ranges
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
tabled = "0.15"

# Optional (add if needed)
# tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
# rayon = "1"  # For CPU parallelism

[dev-dependencies]
criterion = "0.5"  # Benchmarking
```

---

## Open Questions (None)

All technical decisions have been resolved. No outstanding clarifications needed.

---

## References

### GTO/CFR Algorithms
- [CFR Original Paper - University of Alberta](https://poker.cs.ualberta.ca/publications/NIPS07-cfr.pdf)
- [Int8 - CFR for Poker AI](https://int8.io/counterfactual-regret-minimization-for-poker-ai/)
- [GTO Wizard - How Solvers Work](https://blog.gtowizard.com/how-solvers-work/)

### Rust Crates
- [pokers crate](https://crates.io/crates/pokers)
- [clap documentation](https://docs.rs/clap/latest/clap/)
- [tabled - Pretty Print Tables](https://github.com/zhiburt/tabled)

### Poker Hand Evaluation
- [PokerHandEvaluator Algorithm](https://github.com/HenryRLee/PokerHandEvaluator/blob/master/Documentation/Algorithm.md)
- [OMPEval GitHub](https://github.com/zekyll/OMPEval)
