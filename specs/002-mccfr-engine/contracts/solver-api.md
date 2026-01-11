# Solver API Contract

**Feature**: 002-mccfr-engine  
**Date**: 2026-01-10

## Overview

This document defines the API contract for the MCCFR solver. The solver is invoked via the existing CLI interface and Rust library API.

## Library API

### Primary Entry Point

```rust
/// Solve a game state using MCCFR algorithm
/// 
/// # Arguments
/// * `game_state` - The decision point to solve
/// * `iterations` - Number of MCCFR iterations (default: 10,000)
/// 
/// # Returns
/// * `Result<Strategy>` - Computed GTO strategy or error
/// 
/// # Example
/// ```rust
/// let state = GameState::new(hero_hand, board, pot, stack, to_call, position, range)?;
/// let strategy = solver::solve(state, 10000)?;
/// ```
pub fn solve(game_state: GameState, iterations: u32) -> Result<Strategy>
```

### Configuration API

```rust
/// MCCFR solver configuration
pub struct MccfrConfig {
    /// Number of iterations (default: 10,000)
    pub iterations: u32,
    /// Samples per iteration (default: 100)
    pub samples_per_iteration: usize,
    /// Convergence threshold (default: 0.001)
    pub convergence_threshold: f64,
    /// Optional RNG seed for reproducibility
    pub seed: Option<u64>,
}

impl Default for MccfrConfig {
    fn default() -> Self {
        Self {
            iterations: 10_000,
            samples_per_iteration: 100,
            convergence_threshold: 0.001,
            seed: None,
        }
    }
}

/// Solve with custom configuration
pub fn solve_with_config(game_state: GameState, config: MccfrConfig) -> Result<Strategy>
```

## Input Contract: GameState

Existing type, unchanged. Required fields:

| Field | Type | Constraint |
|-------|------|------------|
| hero_hand | Hand | Valid 2-card hand, no duplicates with board |
| board | Vec<Card> | 0, 3, 4, or 5 cards |
| pot_size | f64 | > 0.0 |
| effective_stack | f64 | > 0.0 |
| to_call | f64 | >= 0.0, <= effective_stack |
| position | Position | IP or OOP |
| villain_range | Range | At least 1 hand after blocker removal |

## Output Contract: Strategy

Existing type with guaranteed properties:

| Field | Type | Guarantee |
|-------|------|-----------|
| actions | Vec<ActionStrategy> | Non-empty, frequencies sum to 1.0 ± 0.001 |
| iterations | u32 | Equals requested iterations (or early stop count) |
| convergence | f64 | Final convergence metric (lower = better) |
| game_state | Option<GameState> | Included if requested |

### ActionStrategy

| Field | Type | Guarantee |
|-------|------|-----------|
| action | Action | Valid action for game state |
| frequency | f64 | 0.0 - 1.0, all sum to 1.0 |
| ev | f64 | Expected value in big blinds |

## Error Contract

```rust
pub enum SolverError {
    /// Invalid game state provided
    InvalidGameState(String),
    /// Range is empty after blocker removal
    EmptyRange,
    /// No valid actions available
    NoValidActions,
    /// Solver failed to converge (should not happen with valid input)
    ConvergenceFailure { iterations: u32, metric: f64 },
}
```

## CLI Contract

Existing CLI parameters unchanged. The `--iterations` flag controls MCCFR iteration count:

```bash
# Default (10,000 iterations)
fpe solve --hero AhKd --board "2s5h9c" --pot 10 --stack 100 --range "AA,KK,QQ"

# Custom iterations
fpe solve --hero AhKd --board "2s5h9c" --pot 10 --stack 100 --range "AA,KK,QQ" --iterations 50000

# JSON output with iterations
fpe solve --hero AhKd --board "2s5h9c" --pot 10 --stack 100 --range "AA,KK,QQ" --iterations 50000 --json
```

## Behavioral Guarantees

### Convergence

1. **Monotonic Improvement**: Given sufficient iterations, strategies approach Nash equilibrium
2. **Consistency**: Same input + same seed = same output
3. **Strategy Validity**: Frequencies always sum to 1.0 within tolerance
4. **EV Accuracy**: EV calculations use full equity evaluation against range

### Performance

1. **Time Bound**: 30 seconds for typical river decisions with default iterations
2. **Memory Bound**: O(|info sets| × |actions|) ≈ O(1000 × 10) for single spots
3. **Scalability**: Supports 2-10+ actions per decision

### Edge Cases

| Scenario | Behavior |
|----------|----------|
| Empty range after blockers | Return `EmptyRange` error |
| Single valid action | Return 100% frequency for that action |
| Zero/negative iterations | Return `InvalidGameState` error |
| Hero drawing dead | Strategy reflects fold or minimum loss action |
