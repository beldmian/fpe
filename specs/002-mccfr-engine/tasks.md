# Tasks: MCCFR Solver Implementation

**Input**: Design documents from `/specs/002-mccfr-engine/`  
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Tests**: Tests are included per constitution requirement (Testing Standards: "Tests MUST exist before code is merged")

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3, US4)
- All file paths are relative to repository root

---

## Phase 1: Setup

**Purpose**: Add new dependencies and create module structure

- [x] T001 Add rustc-hash, rand, rand_xoshiro dependencies to Cargo.toml
- [x] T002 [P] Create empty src/solver/info_set.rs with module doc comment
- [x] T003 [P] Create empty src/solver/regret.rs with module doc comment
- [x] T004 [P] Create empty src/solver/mccfr.rs with module doc comment
- [x] T005 Update src/solver/mod.rs to export new modules (info_set, regret, mccfr)
- [x] T006 Run `cargo check` to verify module structure compiles

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core types that MUST be complete before ANY user story implementation

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T007 Implement SprBucket enum with Short/Medium/Deep/VeryDeep variants in src/solver/info_set.rs
- [x] T008 Implement SprBucket::from_spr(f64) conversion function in src/solver/info_set.rs
- [x] T009 Implement InfoSetKey struct with hero_hand, spr_bucket, position fields in src/solver/info_set.rs
- [x] T010 Derive Hash, Eq, PartialEq, Clone for InfoSetKey in src/solver/info_set.rs
- [x] T011 Implement InfoSetKey::from_game_state(&GameState) in src/solver/info_set.rs
- [x] T012 Implement regret_to_strategy(regrets: &[f64]) -> Vec<f64> function in src/solver/regret.rs
- [x] T013 Run `cargo test` to verify foundational types compile

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Compute Converged GTO Strategy (Priority: P1) 🎯 MVP

**Goal**: Replace stub solver with actual MCCFR algorithm that computes Nash equilibrium strategies

**Independent Test**: Run solver on known game scenarios and verify output strategies are non-uniform and converge to exploitability below 1% of pot

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T014 [P] [US1] Unit test for regret_to_strategy with zero regrets returns uniform in tests/unit/mccfr_tests.rs
- [x] T015 [P] [US1] Unit test for regret_to_strategy with positive regrets returns proportional in tests/unit/mccfr_tests.rs
- [x] T016 [P] [US1] Unit test for regret_to_strategy with negative regrets clips to zero in tests/unit/mccfr_tests.rs
- [x] T017 [P] [US1] Unit test for InfoSetKey::from_game_state creates correct key in tests/unit/info_set_tests.rs
- [x] T018 [P] [US1] Unit test for SprBucket::from_spr bucketing logic in tests/unit/info_set_tests.rs
- [x] T019 [US1] Integration test: solver returns non-uniform strategy in tests/integration/solver_tests.rs
- [x] T020 [US1] Integration test: nuts vs air returns bet ~100% in tests/integration/solver_tests.rs

### Implementation for User Story 1

- [x] T021 [P] [US1] Implement RegretTable struct with FxHashMap<InfoSetKey, Vec<f64>> in src/solver/regret.rs
- [x] T022 [P] [US1] Implement McSampler struct with Xoshiro256PlusPlus RNG in src/solver/mccfr.rs
- [x] T023 [US1] Implement RegretTable::new() constructor in src/solver/regret.rs
- [x] T024 [US1] Implement RegretTable::get_strategy(&InfoSetKey, n_actions) in src/solver/regret.rs
- [x] T025 [US1] Implement RegretTable::update_regrets(key, regrets, reach_prob) in src/solver/regret.rs
- [x] T026 [US1] Implement RegretTable::get_average_strategy(&InfoSetKey) in src/solver/regret.rs
- [x] T027 [US1] Implement McSampler::new(seed: Option<u64>) in src/solver/mccfr.rs
- [x] T028 [US1] Implement McSampler::sample_villain_hands(&Range, n) in src/solver/mccfr.rs
- [x] T029 [US1] Implement compute_action_values() for counterfactual value calculation in src/solver/mccfr.rs
- [x] T030 [US1] Implement core MCCFR iteration loop in src/solver/mccfr.rs
- [x] T031 [US1] Implement extract_strategy() to convert RegretTable to Strategy output in src/solver/mccfr.rs
- [x] T032 [US1] Update src/solver/cfr.rs solve() to call MCCFR implementation instead of stub
- [x] T033 [US1] Run all US1 tests and verify they pass

**Checkpoint**: User Story 1 complete - solver computes actual GTO strategies

---

## Phase 4: User Story 2 - Track Training Convergence (Priority: P2)

**Goal**: Report convergence metric in strategy output so users can assess reliability

**Independent Test**: Run solver with N iterations, then 2N iterations, verify convergence metric decreases

### Tests for User Story 2

- [x] T034 [P] [US2] Unit test for ConvergenceTracker::check_convergence returns max change in tests/unit/mccfr_tests.rs
- [x] T035 [P] [US2] Unit test for ConvergenceTracker::is_converged with threshold in tests/unit/mccfr_tests.rs
- [x] T036 [US2] Integration test: solver output includes convergence metric in tests/integration/solver_tests.rs
- [x] T037 [US2] Integration test: more iterations produces lower convergence in tests/integration/solver_tests.rs

### Implementation for User Story 2

- [x] T038 [P] [US2] Implement ConvergenceTracker struct in src/solver/mccfr.rs
- [x] T039 [US2] Implement ConvergenceTracker::new() in src/solver/mccfr.rs
- [x] T040 [US2] Implement ConvergenceTracker::check_convergence(&RegretTable) in src/solver/mccfr.rs
- [x] T041 [US2] Implement ConvergenceTracker::is_converged(threshold) in src/solver/mccfr.rs
- [x] T042 [US2] Integrate ConvergenceTracker into MCCFR iteration loop in src/solver/mccfr.rs
- [x] T043 [US2] Update extract_strategy() to include convergence metric in Strategy.convergence in src/solver/mccfr.rs
- [x] T044 [US2] Run all US2 tests and verify they pass

**Checkpoint**: User Story 2 complete - convergence is tracked and reported

---

## Phase 5: User Story 3 - Configure Training Parameters (Priority: P2)

**Goal**: Allow users to control iteration count via CLI and config struct

**Independent Test**: Specify custom iteration count, verify solver executes that many iterations

### Tests for User Story 3

- [x] T045 [P] [US3] Unit test for MccfrConfig::default() values in tests/unit/mccfr_tests.rs
- [x] T046 [P] [US3] Unit test for MccfrConfig with custom iterations in tests/unit/mccfr_tests.rs
- [x] T047 [US3] Integration test: solve_with_config respects iteration count in tests/integration/solver_tests.rs
- [x] T048 [US3] Integration test: default iterations produces usable convergence in tests/integration/solver_tests.rs

### Implementation for User Story 3

- [x] T049 [P] [US3] Implement MccfrConfig struct with iterations, samples_per_iteration, convergence_threshold, seed in src/solver/mccfr.rs
- [x] T050 [US3] Implement Default for MccfrConfig with 10,000 iterations, 100 samples in src/solver/mccfr.rs
- [x] T051 [US3] Implement solve_with_config(GameState, MccfrConfig) -> Result<Strategy> in src/solver/mccfr.rs
- [x] T052 [US3] Update solve() to use MccfrConfig::default() in src/solver/cfr.rs
- [x] T053 [US3] Export MccfrConfig from src/solver/mod.rs and src/lib.rs
- [x] T054 [US3] Run all US3 tests and verify they pass

**Checkpoint**: User Story 3 complete - training parameters are configurable

---

## Phase 6: User Story 4 - Handle Complex Game Trees (Priority: P3)

**Goal**: Support realistic scenarios with multiple bet sizes (5+ actions)

**Independent Test**: Provide flop scenario with check, bet 33%, bet 75%, all-in; verify valid mixed strategy

### Tests for User Story 4

- [x] T055 [P] [US4] Integration test: 5+ action scenario returns valid frequencies in tests/integration/solver_tests.rs
- [x] T056 [P] [US4] Integration test: multi-action scenario completes within 60 seconds in tests/integration/solver_tests.rs
- [x] T057 [US4] Integration test: complex scenario strategy frequencies sum to 1.0 in tests/integration/solver_tests.rs

### Implementation for User Story 4

- [x] T058 [US4] Verify RegretTable handles variable action counts in src/solver/regret.rs
- [x] T059 [US4] Optimize compute_action_values for 5+ actions in src/solver/mccfr.rs
- [x] T060 [US4] Add timeout/max iteration check to prevent infinite loops in src/solver/mccfr.rs
- [x] T061 [US4] Run all US4 tests and verify they pass

**Checkpoint**: User Story 4 complete - complex game trees are handled efficiently

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, benchmarks, and quality improvements

- [x] T062 [P] Add doc comments to all public functions in src/solver/info_set.rs
- [x] T063 [P] Add doc comments to all public functions in src/solver/regret.rs
- [x] T064 [P] Add doc comments to all public functions in src/solver/mccfr.rs
- [x] T065 [P] Add MCCFR benchmark to benches/solver_bench.rs
- [x] T066 Run `cargo clippy` and fix any warnings
- [x] T067 Run `cargo test` to verify all tests pass
- [x] T068 Run quickstart.md verification commands (build, test, clippy)
- [x] T069 Update CHANGELOG.md with MCCFR implementation details

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1 (Setup)
    │
    ▼
Phase 2 (Foundational) ──── BLOCKS ALL USER STORIES
    │
    ├───────────────┬───────────────┬───────────────┐
    ▼               ▼               ▼               ▼
Phase 3 (US1)   Phase 4 (US2)   Phase 5 (US3)   Phase 6 (US4)
    │               │               │               │
    │               │               │               │
    └───────────────┴───────────────┴───────────────┘
                    │
                    ▼
            Phase 7 (Polish)
```

### User Story Dependencies

- **User Story 1 (P1)**: After Foundational - No dependencies on other stories - **MVP**
- **User Story 2 (P2)**: After Foundational - Uses RegretTable from US1 (can parallelize model work)
- **User Story 3 (P2)**: After Foundational - Independent config, integrates with US1 solve()
- **User Story 4 (P3)**: After Foundational - Extends US1 for multi-action support

### Within Each User Story

1. Tests FIRST (write, ensure they FAIL)
2. Models/structs (can be parallel)
3. Core implementation
4. Integration with existing code
5. Run tests (ensure they PASS)

### Parallel Opportunities

**Phase 1 (Setup)**:
```
T002, T003, T004 can run in parallel (different files)
```

**Phase 3 (US1)**:
```
T014, T015, T016, T017, T018 can run in parallel (test files)
T021, T022 can run in parallel (different files)
```

**Phase 4 (US2)**:
```
T034, T035 can run in parallel (same test file, different functions)
```

**Phase 5 (US3)**:
```
T045, T046 can run in parallel (same test file, different functions)
```

**Phase 6 (US4)**:
```
T055, T056 can run in parallel (same test file, different functions)
```

**Phase 7 (Polish)**:
```
T062, T063, T064, T065 can run in parallel (different files)
```

---

## Parallel Example: User Story 1 Tests

```bash
# Launch all US1 unit tests in parallel:
Task: "Unit test for regret_to_strategy with zero regrets in tests/unit/mccfr_tests.rs"
Task: "Unit test for regret_to_strategy with positive regrets in tests/unit/mccfr_tests.rs"
Task: "Unit test for regret_to_strategy with negative regrets in tests/unit/mccfr_tests.rs"
Task: "Unit test for InfoSetKey::from_game_state in tests/unit/info_set_tests.rs"
Task: "Unit test for SprBucket::from_spr in tests/unit/info_set_tests.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Run solver, verify non-uniform strategies
5. This is a shippable MVP that replaces the stub with real MCCFR

### Incremental Delivery

1. **Setup + Foundational** → Module structure ready
2. **Add User Story 1** → Working MCCFR (MVP!)
3. **Add User Story 2** → Convergence tracking
4. **Add User Story 3** → Configurable parameters
5. **Add User Story 4** → Complex game support
6. **Polish** → Documentation, benchmarks

### Test-First Execution

Within each user story phase:
1. Write all tests first (they will fail)
2. Implement code to make tests pass
3. Verify all tests pass before moving on

---

## Notes

- Constitution requires TDD: Write tests FIRST, ensure they FAIL
- [P] tasks = different files, no dependencies on incomplete tasks
- [Story] label maps task to user story for traceability
- Each user story is independently testable after Foundational phase
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
