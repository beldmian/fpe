# Data Model: MCCFR Solver Implementation

**Feature**: 002-mccfr-engine  
**Date**: 2026-01-10

## Entity Overview

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  TrainingSession │────▶│   RegretTable   │────▶│   InfoSetKey    │
│                 │     │                 │     │                 │
│ - game_state    │     │ - regrets       │     │ - hero_hand     │
│ - config        │     │ - strategy_sum  │     │ - spr_bucket    │
│ - sampler       │     └─────────────────┘     │ - position      │
└─────────────────┘                             └─────────────────┘
        │
        ▼
┌─────────────────┐     ┌─────────────────┐
│    McSampler    │     │  Convergence    │
│                 │     │    Tracker      │
│ - rng           │     │                 │
│ - sample()      │     │ - iterations    │
└─────────────────┘     │ - metric        │
                        └─────────────────┘
```

## Entities

### TrainingSession

A single invocation of the solver; encompasses all iterations, regret updates, and final strategy computation.

| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| game_state | GameState | Input decision point (existing type) | Must have valid hero_hand, villain_range, pot_size, etc. |
| config | MccfrConfig | Solver configuration | iterations > 0, samples > 0 |
| regret_table | RegretTable | Accumulated regrets per info set | Initialized empty |
| sampler | McSampler | Monte Carlo sampler | Seeded RNG |
| iteration_count | u64 | Current iteration number | Monotonically increasing |

**State Transitions**:
- `New` → `Running` (on solve() call)
- `Running` → `Converged` (when threshold met or max iterations)
- `Running` → `Complete` (max iterations without convergence)

### InfoSetKey

Unique identifier for an information set; the fundamental unit for strategy computation.

| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| hero_hand | Hand | Hero's hole cards (canonical form) | Must be valid 2-card hand |
| spr_bucket | SprBucket | Stack-to-pot ratio bucket | Enum: Short(0-2), Medium(2-5), Deep(5-10), VeryDeep(10+) |
| position | Position | Hero's position | Enum: IP, OOP (existing type) |

**Derivation**: Created from `GameState` via `InfoSetKey::from_game_state()`

**Equality**: Two info sets are equal if all fields match (Hash + Eq derive)

### RegretTable

Storage for cumulative regrets and strategy sums across all information sets.

| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| regrets | HashMap<InfoSetKey, Vec<f64>> | Cumulative regret per action per info set | Vector length = action count |
| strategy_sum | HashMap<InfoSetKey, Vec<f64>> | Sum of strategies weighted by reach probability | Vector length = action count |

**Operations**:
- `get_strategy(key, n_actions) -> Vec<f64>`: Returns current strategy via regret matching
- `update_regrets(key, regrets, reach_prob)`: Adds regrets and updates strategy sum
- `get_average_strategy(key) -> Vec<f64>`: Returns average strategy for final output

### SprBucket

Discretized stack-to-pot ratio for info set grouping.

| Variant | SPR Range | Strategic Meaning |
|---------|-----------|-------------------|
| Short | 0 - 2 | Commitment threshold, simplified decisions |
| Medium | 2 - 5 | Standard postflop play |
| Deep | 5 - 10 | Multi-street considerations |
| VeryDeep | 10+ | Maximum flexibility |

**Derivation**: `SprBucket::from_spr(effective_stack / pot_size)`

### MccfrConfig

Configuration for solver execution.

| Field | Type | Description | Default |
|-------|------|-------------|---------|
| iterations | u32 | Number of MCCFR iterations | 10,000 |
| samples_per_iteration | usize | Villain hands sampled per iteration | 100 |
| convergence_threshold | f64 | Strategy change threshold for early stop | 0.001 |
| seed | Option<u64> | RNG seed for reproducibility | None (random) |

### McSampler

Monte Carlo sampler for External Sampling MCCFR.

| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| rng | Xoshiro256PlusPlus | Fast RNG | Seeded |

**Operations**:
- `sample_villain_hands(range, n) -> Vec<(Hand, f64)>`: Sample n hands weighted by range
- `sample_action(strategy) -> usize`: Sample action according to strategy probabilities

### ConvergenceTracker

Tracks convergence metrics during training.

| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| prev_strategies | HashMap<InfoSetKey, Vec<f64>> | Previous iteration strategies | Snapshot per check |
| max_change | f64 | Maximum strategy change this check | 0.0 - 1.0 |
| iterations_since_check | u32 | Iterations since last check | Reset on check |

**Operations**:
- `check_convergence(current_table) -> f64`: Returns max strategy change
- `is_converged(threshold) -> bool`: Returns true if below threshold

## Relationships

```
TrainingSession 1 ──── 1 GameState (input, existing type)
TrainingSession 1 ──── 1 MccfrConfig
TrainingSession 1 ──── 1 RegretTable
TrainingSession 1 ──── 1 McSampler
TrainingSession 1 ──── 1 ConvergenceTracker

RegretTable 1 ──── * InfoSetKey (keys in HashMap)
RegretTable * ──── * Action (Vec<f64> per action, existing type)

InfoSetKey * ──── 1 Hand (hero_hand, existing type)
InfoSetKey * ──── 1 Position (existing type)
InfoSetKey * ──── 1 SprBucket (derived enum)
```

## Existing Types (Unchanged)

These types from the current codebase are used directly:

- **GameState**: Input decision point (hero_hand, board, pot_size, effective_stack, to_call, position, villain_range)
- **Hand**: Two hole cards
- **Card**: Single card (rank + suit)
- **Range**: Opponent hand range with weights
- **Position**: IP / OOP enum
- **Action**: Fold, Check, Call, Bet(size), Raise(size), AllIn
- **Strategy**: Output type (actions with frequencies and EVs)
- **ActionStrategy**: Single action with frequency and EV

## Output Mapping

The existing `Strategy` type is populated from the MCCFR result:

```
RegretTable.get_average_strategy(key) → ActionStrategy.frequency
compute_ev(action, game_state)        → ActionStrategy.ev
determine_available_actions()         → Strategy.actions (list)
training_session.iteration_count      → Strategy.iterations
convergence_tracker.max_change        → Strategy.convergence
```
