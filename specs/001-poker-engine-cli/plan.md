# Implementation Plan: Poker GTO Strategy Engine with CLI Interface

**Branch**: `001-poker-engine-cli` | **Date**: 2026-01-09 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-poker-engine-cli/spec.md`

## Summary

Build a Poker GTO Strategy Engine that calculates Nash equilibrium strategies for single decision points in Texas Hold'em. The engine uses Monte Carlo Counterfactual Regret Minimization (MCCFR) to compute optimal action frequencies and expected values, with a CLI interface supporting both human-readable tables and JSON output.

## Technical Context

**Language/Version**: Rust 1.75+ (stable)
**Primary Dependencies**: pokers (hand evaluation + equity), clap (CLI), serde/serde_json (serialization), tabled (table output), tokio (async runtime if needed)
**Storage**: N/A (in-memory computation only)
**Testing**: cargo test (unit + integration tests)
**Target Platform**: Cross-platform CLI (Linux, macOS, Windows)
**Project Type**: Single project
**Performance Goals**: GTO calculation within 30 seconds for typical scenarios (per SC-001), hand evaluation ~25ns per hand
**Constraints**: <1GB memory for computation, offline-capable (no network dependencies)
**Scale/Scope**: Single decision point analysis, not full game tree solving

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Pre-Phase 0 Evaluation

| Principle | Requirement | Status | Notes |
|-----------|-------------|--------|-------|
| **I. Code Quality** | cargo clippy zero warnings | ✅ WILL COMPLY | All code will pass clippy |
| | Public API doc comments | ✅ WILL COMPLY | All public functions/types documented |
| | Cyclomatic complexity <10 | ✅ WILL COMPLY | Functions kept focused |
| | Idiomatic Rust patterns | ✅ WILL COMPLY | Result-based errors, proper ownership |
| **II. Testing** | 80% line coverage minimum | ✅ WILL COMPLY | TDD approach planned |
| | Unit tests isolated | ✅ WILL COMPLY | No external dependencies |
| | Integration tests for public API | ✅ WILL COMPLY | CLI entry points tested |
| | Deterministic tests | ✅ WILL COMPLY | Seeded RNG for Monte Carlo |
| **III. UX Consistency** | CLI verb-noun naming | ✅ WILL COMPLY | `fpe analyze`, `fpe help` |
| | Actionable error messages | ✅ WILL COMPLY | Clear errors with fix suggestions |
| | Default human-readable + --json | ✅ WILL COMPLY | Per spec FR-009, FR-010 |
| | --help output | ✅ WILL COMPLY | Per spec FR-016 |
| **IV. Performance** | Documented latency targets | ✅ WILL COMPLY | 30s max for typical scenarios |
| | Memory limits documented | ✅ WILL COMPLY | <1GB for computation |
| | Benchmark tests in CI | ⚠️ DEFERRED | Initial release; benchmarks Phase 2 |

**Gate Status**: ✅ PASS - No blocking violations. Benchmark CI deferred to Phase 2 implementation.

## Project Structure

### Documentation (this feature)

```text
specs/001-poker-engine-cli/
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
├── main.rs              # CLI entry point
├── lib.rs               # Library root (public API)
├── models/
│   ├── mod.rs           # Models module
│   ├── card.rs          # Card, Rank, Suit types
│   ├── hand.rs          # Hand representation
│   ├── range.rs         # Hand range parsing/representation
│   ├── game_state.rs    # GameState (decision point context)
│   ├── action.rs        # Action types (fold/check/call/bet/raise)
│   └── strategy.rs      # Strategy output (frequencies + EVs)
├── solver/
│   ├── mod.rs           # Solver module
│   ├── cfr.rs           # MCCFR implementation
│   ├── equity.rs        # Equity calculation (uses pokers crate)
│   └── evaluator.rs     # Hand strength evaluation wrapper
├── cli/
│   ├── mod.rs           # CLI module
│   ├── args.rs          # Clap argument definitions
│   ├── output.rs        # Output formatting (table/JSON)
│   └── validation.rs    # Input validation
└── error.rs             # Custom error types

tests/
├── integration/
│   ├── cli_tests.rs     # End-to-end CLI tests
│   └── solver_tests.rs  # Solver integration tests
└── unit/
    ├── models_tests.rs  # Model unit tests
    ├── range_tests.rs   # Range parsing tests
    └── equity_tests.rs  # Equity calculation tests

benches/
└── solver_bench.rs      # Criterion benchmarks for solver
```

**Structure Decision**: Single project structure selected. This is a standalone CLI application with no web/mobile components. The structure separates concerns into models (data types), solver (GTO computation), and cli (user interface).

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Benchmark CI deferred | Initial release scope | Benchmarks added in Phase 2 once solver is stable |

No other complexity violations identified. Design follows single project, minimal dependency approach.

### Post-Phase 1 Re-Evaluation

| Principle | Requirement | Status | Design Artifact Reference |
|-----------|-------------|--------|---------------------------|
| **I. Code Quality** | cargo clippy zero warnings | ✅ CONFIRMED | All types in data-model.md derive standard traits |
| | Public API doc comments | ✅ CONFIRMED | data-model.md includes doc comments for all types |
| | Cyclomatic complexity <10 | ✅ CONFIRMED | Functions decomposed across models/solver/cli modules |
| | Idiomatic Rust patterns | ✅ CONFIRMED | Result-based errors in error.rs, ownership in data-model.md |
| **II. Testing** | 80% line coverage minimum | ✅ CONFIRMED | Test structure defined in plan, validation scenarios in data-model.md |
| | Unit tests isolated | ✅ CONFIRMED | No external dependencies in models; pokers crate for evaluation |
| | Integration tests for public API | ✅ CONFIRMED | CLI tests in tests/integration/ per structure |
| | Deterministic tests | ✅ CONFIRMED | research.md notes seeded RNG for Monte Carlo |
| **III. UX Consistency** | CLI verb-noun naming | ✅ CONFIRMED | `fpe analyze` in cli-interface.md |
| | Actionable error messages | ✅ CONFIRMED | Error examples in cli-interface.md with fix suggestions |
| | Default human-readable + --json | ✅ CONFIRMED | Both formats in contracts/cli-interface.md |
| | --help output | ✅ CONFIRMED | Help text defined in cli-interface.md |
| **IV. Performance** | Documented latency targets | ✅ CONFIRMED | 30s in Technical Context, quickstart.md performance tips |
| | Memory limits documented | ✅ CONFIRMED | <1GB in Technical Context |
| | Benchmark tests in CI | ⚠️ DEFERRED | benches/solver_bench.rs planned for Phase 2 |

**Post-Design Gate Status**: ✅ PASS - Design artifacts comply with all constitution principles. Benchmark CI remains deferred to Phase 2.

## Generated Artifacts

| Artifact | Path | Status |
|----------|------|--------|
| Research | [research.md](./research.md) | ✅ Complete |
| Data Model | [data-model.md](./data-model.md) | ✅ Complete |
| CLI Contract | [contracts/cli-interface.md](./contracts/cli-interface.md) | ✅ Complete |
| JSON Schema | [contracts/json-schema.json](./contracts/json-schema.json) | ✅ Complete |
| Quickstart | [quickstart.md](./quickstart.md) | ✅ Complete |
| Agent Context | [CLAUDE.md](../../CLAUDE.md) | ✅ Updated |
