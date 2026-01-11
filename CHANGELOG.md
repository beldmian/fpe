# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added - MCCFR Solver Implementation

#### Core Algorithm
- Implemented Monte Carlo Counterfactual Regret Minimization (MCCFR) solver using External Sampling
- Added `MccfrConfig` struct for configurable solver parameters (iterations, samples_per_iteration, convergence_threshold, seed)
- Implemented `solve_with_config()` function for advanced solver configuration
- Added regret matching algorithm with positive regret clipping for strategy computation
- Implemented information set abstraction using hand, SPR bucket, and position

#### New Modules
- `src/solver/mccfr.rs`: Core MCCFR algorithm implementation with:
  - `MccfrConfig`: Configuration struct with default values (10,000 iterations, 100 samples)
  - `ConvergenceTracker`: Tracks strategy convergence across iterations
  - `McSampler`: Monte Carlo sampler for External Sampling MCCFR
  - `solve_mccfr()`: Main solver entry point
  - `solve_with_config()`: Advanced solver with custom configuration

- `src/solver/info_set.rs`: Information set abstraction with:
  - `SprBucket`: Discretized stack-to-pot ratio bucketing (Short, Medium, Deep, VeryDeep)
  - `InfoSetKey`: Unique identifier for information sets based on hand, SPR bucket, and position
  - `from_game_state()`: Converts GameState to InfoSetKey

- `src/solver/regret.rs`: Regret tracking and strategy computation with:
  - `regret_to_strategy()`: Converts cumulative regrets to strategy probabilities
  - `RegretTable`: Storage for regrets and strategy sums across all information sets
  - Support for average strategy computation for final output

#### Dependencies
- Added `rustc-hash = "1.1"` for fast HashMap implementation (FxHashMap)
- Added `rand = "0.8"` for random number generation
- Added `rand_xoshiro = "0.6"` for fast, high-quality RNG (Xoshiro256PlusPlus)

#### Features
- Convergence tracking with automatic early stopping support
- Configurable iteration count, samples per iteration, and convergence threshold
- Deterministic solving with optional RNG seed for reproducibility
- Strategy stability metric reported in solver output
- Support for complex game trees with 5+ actions per decision point

#### Testing
- Added comprehensive unit tests for:
  - Regret matching algorithm (zero, positive, and negative regrets)
  - Information set key creation and SPR bucketing
  - Convergence tracking and threshold detection
  - MCCFR configuration (default and custom)

- Added integration tests for:
  - Non-uniform strategy generation (vs. previous stub behavior)
  - Nuts vs air scenarios (bet frequency validation)
  - Convergence metric reporting
  - Iteration count scaling
  - Complex game tree handling (5+ actions)
  - Configuration parameter respect

#### Performance
- Added benchmarks in `benches/solver_bench.rs`:
  - River decision benchmarks (100 and 1000 iterations)
  - Polarized range scenarios
  - Flop decision with medium SPR
  - Iteration count scaling analysis (100-5000 iterations)

- Optimizations:
  - FxHashMap for ~2-3x faster lookups vs standard HashMap
  - Monte Carlo sampling reduces tree size by ~50x per iteration
  - External Sampling for efficient single-spot solving

#### Documentation
- Added comprehensive doc comments to all public functions and types
- Module-level documentation for mccfr, info_set, and regret modules
- Updated project structure documentation

#### Breaking Changes
- Replaced stub solver that returned uniform strategies with actual MCCFR implementation
- Solver now computes real Nash equilibrium strategies (non-uniform)
- Strategy output now includes convergence metric
- Solving now takes 1-30 seconds depending on iterations (vs instant for stub)

### Changed
- Updated `Strategy` struct to include convergence metric
- Modified solver output to report actual computed strategies instead of uniform distribution
- Improved strategy EV calculation accuracy

### Fixed
- Fixed all clippy warnings:
  - Removed unused imports in `src/solver/cfr.rs`
  - Added Default implementations for `ConvergenceTracker` and `RegretTable`
  - Fixed field reassignment patterns in `solve_mccfr()`
  - Fixed collapsible if-else blocks
  - Added missing doc comments for public methods
  - Fixed manual is_multiple_of implementation in `src/main.rs`

## [0.1.0] - Initial Release

### Added
- Basic poker engine CLI implementation
- Hand evaluation using pokers crate
- Range parsing and notation support
- Game state modeling (positions, actions, board states)
- CLI argument parsing with clap
- JSON and table output formats
- Stub solver (uniform strategy distribution)
- Basic validation and error handling
