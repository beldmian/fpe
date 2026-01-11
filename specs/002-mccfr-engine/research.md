# Research: MCCFR Solver Implementation

**Feature**: 002-mccfr-engine  
**Date**: 2026-01-10

## 1. MCCFR Variant Selection

**Decision**: External Sampling MCCFR (ES-MCCFR)

**Rationale**:
- Optimal for single decision point analysis (the current use case)
- Samples opponent actions according to their strategy and samples chance outcomes
- Hero traverses all actions at decision points
- Lower variance than Outcome Sampling while maintaining speed
- Proven in commercial poker solvers (Libratus, etc.)

**Alternatives Considered**:
- **Vanilla CFR**: Too slow for ranges with 1326 possible holdings; O(|Game Tree|) per iteration
- **Outcome Sampling**: Higher variance, better for massive parallelization but overkill for single spots
- **CFR+**: Faster convergence but more complex implementation (requires regret truncation)
- **Chance Sampling**: Better for multi-street solving, not optimal for single spots

## 2. Regret Matching Algorithm

**Decision**: Standard regret matching with positive regret clipping

**Formula**:
```
σ(a) = R⁺(a) / Σ R⁺(a')    if Σ R⁺(a') > 0
σ(a) = 1/|A|               otherwise (uniform)

where R⁺(a) = max(R(a), 0)
```

**Rationale**:
- Simple and well-understood algorithm
- Proven convergence guarantees to Nash equilibrium
- Easy to implement and debug
- Standard in poker solver literature

**Alternatives Considered**:
- **Regret Matching+ (CFR+)**: Floor regrets at 0 cumulatively for faster convergence, but requires careful implementation
- **Discounted CFR**: Weight recent iterations more heavily for better early stopping behavior

## 3. Information Set Representation

**Decision**: Use `(Hand, SPR_bucket, Position)` as minimal info set key

**Rationale**:
- For single decision point analysis, hero hand is the primary distinguishing factor
- SPR (stack-to-pot ratio) bucketing reduces info set count while capturing strategic relevance
- Position (IP/OOP) significantly affects optimal strategy
- No need for full action history since solving single spots
- Existing `GameState` already captures all necessary information

**Buckets for SPR**:
- 0-2 (short stacked / committed)
- 2-5 (medium SPR)
- 5-10 (deep SPR)
- 10+ (very deep)

**Alternatives Considered**:
- Full action history encoding: Overkill for single spot analysis
- Card abstraction (equity bucketing): Useful for preflop ranges but loses precision for postflop

## 4. Convergence Measurement

**Decision**: Track average strategy and measure strategy stability

**Rationale**:
- True exploitability calculation requires solving a best response, which is computationally expensive
- Strategy stability (max strategy change across iterations) is a good practical proxy for convergence
- Industry standard approach: report iterations + strategy stability metric
- Users can interpret convergence value relative to iteration count

**Metric**: Maximum absolute strategy change across all info sets per iteration
- Values approaching 0 indicate convergence
- Threshold of ~0.01 (1% strategy change) indicates near-equilibrium

**Alternatives Considered**:
- Full best response calculation: Accurate exploitability but computationally expensive (doubles solve time)
- Regret sum tracking: Less intuitive than strategy change for users

## 5. Performance Optimization Strategy

**Decision**: External Sampling with 100-1000 samples per iteration

**Complexity Analysis**:
```
Vanilla CFR per iteration: O(|Hero Hands| × |Villain Hands| × |Actions|²)
= ~1000 × 1000 × 25 = 25 million nodes

External Sampling per iteration: O(|Hero Hands| × |Samples| × |Actions|)
= ~1000 × 100 × 5 = 500,000 nodes

Speedup: ~50x per iteration
```

**Rationale**:
- Monte Carlo sampling dramatically reduces tree size per iteration
- More iterations with smaller samples converges faster than fewer full iterations
- Can leverage existing `calculate_equity` infrastructure for sampling

**Optimizations Planned**:
1. Use `FxHashMap` (rustc-hash) for faster regret table lookups
2. Precompute equity between hand combinations where possible
3. Sample villain hands weighted by range probability
4. Consider `rayon` for parallel iteration over hero hands

**Alternatives Considered**:
- Full tree enumeration: Impractical for realistic ranges
- GPU acceleration: Overkill for current scope, adds complexity
- Card abstraction: Loses precision for river decisions

## 6. Rust Implementation Patterns

**Decision**: Use standard Rust patterns with performance-focused crates

**New Dependencies**:
```toml
rustc-hash = "1.1"       # FxHashMap - faster than std HashMap
rand = "0.8"             # RNG for Monte Carlo sampling
rand_xoshiro = "0.6"     # Fast RNG implementation
```

**Rationale**:
- `rustc-hash`: FxHashMap is 2-3x faster than std HashMap for small keys
- `rand`: Standard Rust randomness crate with good ergonomics
- `rand_xoshiro`: Fast, high-quality RNG suitable for Monte Carlo

**Data Structure Choices**:
- `FxHashMap<InfoSetKey, Vec<f64>>` for regret storage (fast hash, minimal allocation)
- `Vec<f64>` for per-action regrets/strategy (cache-friendly, fixed size)
- `f64` for all floating point (precision over speed for correctness)

**Alternatives Considered**:
- `rayon`: Defer parallel iteration to optimization phase; start with single-threaded
- `ndarray`: Overkill for small action counts (2-10 actions)
- Custom hash: Standard FxHash is sufficient for hand/position keys

## Summary Table

| Aspect | Decision | Key Benefit |
|--------|----------|-------------|
| Variant | External Sampling | Best balance for single spots |
| Regret Matching | Standard positive clipping | Simple, proven convergence |
| Info Set Key | (Hand, SPR, Position) | Minimal for single spot |
| Convergence | Strategy stability | Practical, no BR needed |
| Sampling | 100-1000 per iteration | ~50x speedup |
| HashMap | FxHashMap | 2-3x faster lookups |
