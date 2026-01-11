# Quickstart: MCCFR Solver Implementation

**Feature**: 002-mccfr-engine  
**Date**: 2026-01-10

## Prerequisites

- Rust 1.75+ (stable)
- Existing fpe codebase cloned and building

## New Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
rustc-hash = "1.1"       # Fast HashMap for regret storage
rand = "0.8"             # RNG for Monte Carlo sampling
rand_xoshiro = "0.6"     # Fast RNG implementation
```

## Implementation Order

### Step 1: Add New Types (src/solver/)

1. **info_set.rs** - Information set key and SPR bucketing
2. **regret.rs** - Regret table and regret matching
3. **mccfr.rs** - Core MCCFR algorithm

### Step 2: Update Existing Files

1. **src/solver/mod.rs** - Export new modules
2. **src/solver/cfr.rs** - Replace stub with MCCFR call

### Step 3: Add Tests

1. **tests/unit/mccfr_tests.rs** - Unit tests for regret matching
2. **tests/unit/info_set_tests.rs** - Unit tests for info set creation
3. **tests/integration/solver_tests.rs** - Integration tests for known solutions

## Key Implementation Points

### Regret Matching Formula

```rust
pub fn regret_to_strategy(regrets: &[f64]) -> Vec<f64> {
    let positive: Vec<f64> = regrets.iter().map(|r| r.max(0.0)).collect();
    let sum: f64 = positive.iter().sum();
    
    if sum > 0.0 {
        positive.iter().map(|r| r / sum).collect()
    } else {
        vec![1.0 / regrets.len() as f64; regrets.len()]
    }
}
```

### External Sampling Loop

```rust
for _ in 0..config.iterations {
    // Sample villain hands
    let samples = sampler.sample_villain_hands(&range, config.samples_per_iteration);
    
    // Traverse all hero hands
    for (hero_hand, weight) in hero_hands {
        let key = InfoSetKey::from_hand(hero_hand, &game_state);
        
        // Compute counterfactual values for each action
        let cfvs = compute_action_values(hero_hand, &samples, &actions);
        
        // Update regrets
        let strategy = regret_table.get_strategy(&key, actions.len());
        let node_value: f64 = cfvs.iter().zip(&strategy).map(|(v, s)| v * s).sum();
        let regrets: Vec<f64> = cfvs.iter().map(|v| v - node_value).collect();
        
        regret_table.update_regrets(key, regrets, weight);
    }
}
```

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_regret_matching_uniform() {
    let regrets = vec![0.0, 0.0, 0.0];
    let strategy = regret_to_strategy(&regrets);
    assert!((strategy[0] - 0.333).abs() < 0.01);
}

#[test]
fn test_regret_matching_positive() {
    let regrets = vec![1.0, 3.0, 0.0];
    let strategy = regret_to_strategy(&regrets);
    assert!((strategy[0] - 0.25).abs() < 0.01);
    assert!((strategy[1] - 0.75).abs() < 0.01);
    assert!((strategy[2] - 0.0).abs() < 0.01);
}
```

### Integration Tests

```rust
#[test]
fn test_nuts_vs_air_fold_100() {
    // Hero has nuts, villain has pure bluffs
    // GTO: Hero should bet 100%
    let strategy = solve_spot(/* nuts vs air */);
    let bet_freq = strategy.actions.iter()
        .find(|a| matches!(a.action, Action::Bet(_)))
        .map(|a| a.frequency)
        .unwrap_or(0.0);
    assert!(bet_freq > 0.95);
}
```

## Verification Commands

```bash
# Build
cargo build

# Run tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Check clippy
cargo clippy

# Run benchmark (after implementation)
cargo bench --bench solver_bench
```

## Expected Behavior Changes

| Before (Stub) | After (MCCFR) |
|---------------|---------------|
| Uniform distribution | Converged Nash equilibrium |
| EV = 0.0 for all actions | Computed EV per action |
| Instant return | 1-30 seconds depending on iterations |
| No convergence metric | Reports convergence value |

## Common Issues

1. **Strategy doesn't sum to 1.0**: Check regret matching positive clipping
2. **Same output every run**: Ensure RNG is properly seeded
3. **Slow performance**: Reduce samples_per_iteration or iterations
4. **Memory issues**: Check for info set key collisions
