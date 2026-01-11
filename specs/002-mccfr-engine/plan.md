# Implementation Plan: MCCFR Solver Implementation

**Branch**: `002-mccfr-engine` | **Date**: 2026-01-10 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-mccfr-engine/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement Monte Carlo Counterfactual Regret Minimization (MCCFR) algorithm to replace the current stub solver that returns uniform strategies. The implementation will compute Nash equilibrium strategies through iterative regret minimization, using Monte Carlo sampling for efficient game tree traversal.

## Technical Context

**Language/Version**: Rust 1.75+ (stable)  
**Primary Dependencies**: pokers 0.7 (hand evaluation), clap 4 (CLI), serde/serde_json (serialization), tabled 0.15 (output formatting)  
**Storage**: N/A (in-memory computation, no persistence required)  
**Testing**: cargo test + criterion 0.5 (benchmarks)  
**Target Platform**: CLI application (Linux, macOS, Windows)  
**Project Type**: single (CLI tool)  
**Performance Goals**: Single-street (river) decisions complete within 30 seconds with 10,000 iterations  
**Constraints**: Memory usage must fit typical workloads in RAM; no external dependencies beyond existing crates  
**Scale/Scope**: Single decision point analysis with 2-10+ actions per node; opponent range up to 1326 hand combinations

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Code Quality
- [ ] Code passes `cargo clippy` with zero warnings
- [ ] Public functions have doc comments
- [ ] Functions maintain cyclomatic complexity below 10
- [ ] Code follows idiomatic Rust patterns (ownership, Result-based errors, traits)

### II. Testing Standards
- [ ] Minimum 80% line coverage for new code
- [ ] Tests exist before code is merged (TDD)
- [ ] Unit tests are isolated (no external dependencies)
- [ ] Integration tests cover solver API entry points
- [ ] All tests are deterministic

### III. User Experience Consistency
- [ ] CLI parameter `--iterations` follows existing conventions
- [ ] Error messages are actionable
- [ ] Output formats support both human-readable and JSON (`--json`)
- [ ] Breaking changes documented in CHANGELOG.md

### IV. Performance Requirements
- [ ] Latency target: 30 seconds for typical river decisions documented
- [ ] Benchmark tests run in CI
- [ ] Performance regressions >10% block merge

**Initial Gate Status**: PASS - No violations identified. Design will integrate with existing solver API.

## Project Structure

### Documentation (this feature)

```text
specs/002-mccfr-engine/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── models/
│   ├── action.rs        # Existing: Action, BetSize types
│   ├── card.rs          # Existing: Card representation
│   ├── game_state.rs    # Existing: GameState for decision points
│   ├── hand.rs          # Existing: Hand (hole cards)
│   ├── range.rs         # Existing: Opponent range handling
│   └── strategy.rs      # Existing: Strategy output (to be updated)
├── solver/
│   ├── mod.rs           # Existing: Solver module exports
│   ├── cfr.rs           # MODIFY: Implement actual MCCFR algorithm
│   ├── evaluator.rs     # Existing: Hand evaluation wrapper
│   ├── mccfr.rs         # NEW: MCCFR core algorithm implementation
│   ├── info_set.rs      # NEW: Information set abstraction
│   └── regret.rs        # NEW: Regret tracking and matching
├── cli/
│   ├── args.rs          # Existing: CLI argument parsing
│   └── output.rs        # Existing: Output formatting
├── error.rs             # Existing: Error types
├── lib.rs               # Existing: Library exports
└── main.rs              # Existing: CLI entry point

tests/
├── unit/
│   ├── mccfr_tests.rs   # NEW: Unit tests for MCCFR algorithm
│   └── info_set_tests.rs # NEW: Unit tests for information sets
├── integration/
│   └── solver_tests.rs  # MODIFY: Add MCCFR integration tests
└── contract/            # NEW: Contract tests for solver API
```

**Structure Decision**: Single project structure. MCCFR implementation extends existing `solver/` module with new files for algorithm components. Existing models and CLI remain unchanged.

## Complexity Tracking

> No constitution violations requiring justification. Design follows existing patterns.

## Post-Design Constitution Check

*Re-evaluation after Phase 1 design completion.*

### I. Code Quality
- [x] **Compliant**: New modules (mccfr.rs, info_set.rs, regret.rs) will have doc comments
- [x] **Compliant**: Algorithm functions (regret_to_strategy, compute_cfv) stay under complexity 10
- [x] **Compliant**: Design uses Result-based error handling via existing error.rs

### II. Testing Standards
- [x] **Compliant**: Test plan includes unit tests for regret matching, info set creation
- [x] **Compliant**: Integration tests cover solver API (solve function)
- [x] **Compliant**: Tests use deterministic RNG seed for reproducibility

### III. User Experience Consistency
- [x] **Compliant**: Uses existing --iterations CLI parameter
- [x] **Compliant**: Strategy output unchanged; same JSON structure
- [x] **Compliant**: No breaking CLI changes

### IV. Performance Requirements
- [x] **Compliant**: 30-second target documented for river decisions
- [x] **Compliant**: Will add MCCFR benchmark to existing solver_bench.rs

**Post-Design Gate Status**: PASS - Design satisfies all constitution requirements.

## New Dependencies

```toml
# Add to Cargo.toml
rustc-hash = "1.1"       # Fast HashMap for regret storage
rand = "0.8"             # RNG for Monte Carlo sampling  
rand_xoshiro = "0.6"     # Fast RNG implementation
```

## Phase 1 Artifacts

| Artifact | Path | Status |
|----------|------|--------|
| Research | specs/002-mccfr-engine/research.md | Complete |
| Data Model | specs/002-mccfr-engine/data-model.md | Complete |
| API Contract | specs/002-mccfr-engine/contracts/solver-api.md | Complete |
| Quickstart | specs/002-mccfr-engine/quickstart.md | Complete |

## Next Steps

Run `/speckit.tasks` to generate implementation tasks from this plan.
