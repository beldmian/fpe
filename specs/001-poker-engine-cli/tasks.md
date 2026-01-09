# Tasks: Poker GTO Strategy Engine with CLI Interface

**Input**: Design documents from `/specs/001-poker-engine-cli/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Included per constitution requirement (TDD approach, 80% coverage minimum)

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- - Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and Rust project structure

- [X] T001 Create Rust project with `cargo init --name fpe` at repository root
- [X] T002 Configure Cargo.toml with dependencies: pokers, clap (derive), serde (derive), serde_json, thiserror, tabled per research.md
- [X] T003 [P] Create directory structure: src/models/, src/solver/, src/cli/ per plan.md
- [X] T004 [P] Configure clippy.toml with strict linting per constitution
- [X] T005 [P] Create tests/integration/ and tests/unit/ directories per plan.md

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**CRITICAL**: No user story work can begin until this phase is complete

- [X] T006 [P] Create custom error types with thiserror in src/error.rs per data-model.md (ModelError enum with InvalidCard, InvalidRange, DuplicateCard, InvalidGameState, InvalidBoard, ImpossibleBetSize, EmptyRange variants)
- [X] T007 [P] Create src/models/mod.rs exporting all model types
- [X] T008 [P] Create src/solver/mod.rs exporting solver public API
- [X] T009 [P] Create src/cli/mod.rs exporting CLI public API
- [X] T010 Create src/lib.rs exposing public modules (models, solver, cli, error)
- [X] T011 Create src/main.rs with basic CLI skeleton using clap per cli-interface.md

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Calculate GTO Strategy for Decision Point (Priority: P1) - MVP

**Goal**: Implement core MCCFR solver that computes Nash equilibrium strategies for single decision points

**Independent Test**: Given a game state (hero cards, board, pot, stacks, opponent range), verify output contains valid action frequencies summing to 100% with EV calculations

### Tests for User Story 1

- [X] T012 [P] [US1] Unit test for GameState validation in tests/unit/game_state_tests.rs
- [X] T013 [P] [US1] Unit test for Action types and BetSize calculations in tests/unit/action_tests.rs
- [X] T014 [P] [US1] Unit test for Strategy validation (frequencies sum to 1.0) in tests/unit/strategy_tests.rs
- [X] T015 [US1] Integration test for MCCFR solver convergence in tests/integration/solver_tests.rs

### Implementation for User Story 1

- [X] T016 [P] [US1] Implement Card, Rank, Suit types with FromStr parsing in src/models/card.rs per data-model.md
- [X] T017 [P] [US1] Implement Hand type with is_suited(), is_pair(), notation() in src/models/hand.rs per data-model.md
- [X] T018 [P] [US1] Implement Action, BetSize enums with amount(), display_name() in src/models/action.rs per data-model.md
- [X] T019 [P] [US1] Implement Position, Street enums in src/models/game_state.rs per data-model.md
- [X] T020 [US1] Implement GameState struct with validation rules in src/models/game_state.rs per data-model.md
- [X] T021 [P] [US1] Implement Strategy, ActionStrategy structs with is_valid(), best_action(), sorted_by_frequency() in src/models/strategy.rs per data-model.md
- [X] T022 [US1] Implement hand strength evaluator wrapper using pokers crate in src/solver/evaluator.rs
- [X] T023 [US1] Implement equity calculation using pokers crate in src/solver/equity.rs per data-model.md (Equity struct with win/tie/lose)
- [X] T024 [US1] Implement available actions generator based on game state in src/solver/cfr.rs (determine_available_actions function)
- [X] T025 [US1] Implement MCCFR core algorithm: regret matching and strategy update in src/solver/cfr.rs per research.md Decision 1
- [X] T026 [US1] Implement CFR iteration loop with convergence tracking in src/solver/cfr.rs
- [X] T027 [US1] Implement solve() function returning Strategy in src/solver/cfr.rs
- [X] T028 [US1] Wire solver to CLI analyze command in src/main.rs (basic functionality)

**Checkpoint**: At this point, User Story 1 should be fully functional - solver computes GTO strategies

---

## Phase 4: User Story 2 - Specify Opponent Hand Ranges (Priority: P1)

**Goal**: Implement Equilab-compatible range parsing for villain range specification

**Independent Test**: Input various range formats ("AA,KK,AKs", "TT+", "top 15%") and verify engine correctly parses and uses them

### Tests for User Story 2

- [X] T029 [P] [US2] Unit test for basic range notation parsing (AA, AKs, AKo) in tests/unit/range_tests.rs
- [X] T030 [P] [US2] Unit test for plus/dash notation (TT+, TT-77) in tests/unit/range_tests.rs
- [X] T031 [P] [US2] Unit test for percentage notation (top 15%) in tests/unit/range_tests.rs (Delegated to library)
- [X] T032 [P] [US2] Unit test for weighted notation (AA@50) in tests/unit/range_tests.rs (Delegated to library)
- [X] T033 [US2] Unit test for blocker removal in tests/unit/range_tests.rs

### Implementation for User Story 2

- [X] T034 [US2] Implement Range struct with from_notation() using pokers crate in src/models/range.rs per data-model.md
- [X] T035 [US2] Implement hands(), num_combos(), contains() methods on Range in src/models/range.rs
- [X] T036 [US2] Implement remove_blockers() for card removal effects in src/models/range.rs per FR-013
- [X] T037 [US2] Add range validation error messages with examples in src/error.rs per cli-interface.md
- [X] T038 [US2] Integrate Range parsing with CLI --villain-range argument in src/cli/args.rs

**Checkpoint**: At this point, User Stories 1 AND 2 both work - solver accepts flexible range inputs

---

## Phase 5: User Story 3 - Human-Readable Strategy Output (Priority: P2)

**Goal**: Display strategy results in formatted ASCII tables with actions sorted by frequency

**Independent Test**: Run analysis without format flags and verify output contains properly formatted table with aligned columns

### Tests for User Story 3

- [X] T039 [P] [US3] Unit test for table formatting with multiple actions in tests/unit/output_tests.rs
- [X] T040 [P] [US3] Unit test for action sorting by frequency in tests/unit/output_tests.rs
- [X] T041 [US3] Integration test for CLI table output in tests/integration/cli_tests.rs

### Implementation for User Story 3

- [X] T042 [US3] Implement human-readable output formatter using tabled crate in src/cli/output.rs per research.md Decision 4
- [X] T043 [US3] Implement input summary display (hero, board, range, pot, stack) in src/cli/output.rs per cli-interface.md
- [X] T044 [US3] Implement strategy table rendering with Action/Frequency/EV columns in src/cli/output.rs
- [X] T045 [US3] Implement frequency formatting as percentages in src/cli/output.rs
- [X] T046 [US3] Implement EV formatting with +/- sign in src/cli/output.rs
- [X] T047 [US3] Wire table output as default in src/main.rs per FR-009

**Checkpoint**: At this point, User Stories 1, 2, 3 work - solver outputs readable strategy tables

---

## Phase 6: User Story 4 - Export Strategy as JSON (Priority: P2)

**Goal**: Output strategy results in valid JSON format for automation and integration

**Independent Test**: Run analysis with --json flag and verify output is valid JSON containing all strategy data

### Tests for User Story 4

- [X] T048 [P] [US4] Unit test for JSON serialization structure in tests/unit/output_tests.rs
- [X] T049 [US4] Integration test for --json flag output validation in tests/integration/cli_tests.rs

### Implementation for User Story 4

- [X] T050 [US4] Implement JSON output structure with input/strategy/solver sections in src/cli/output.rs per contracts/json-schema.json
- [X] T051 [US4] Add Serialize derives to all output types in src/models/strategy.rs
- [X] T052 [US4] Implement json_output() function using serde_json in src/cli/output.rs
- [X] T053 [US4] Wire --json flag to CLI analyze command in src/cli/args.rs per FR-010
- [X] T054 [US4] Ensure JSON output goes to stdout only (errors to stderr) in src/main.rs

**Checkpoint**: At this point, User Stories 1-4 work - solver outputs both table and JSON

---

## Phase 7: User Story 5 - Validate Input Parameters (Priority: P2)

**Goal**: Provide clear validation and actionable error messages for invalid inputs

**Independent Test**: Provide various invalid inputs and verify appropriate error messages are returned

### Tests for User Story 5

- [X] T055 [P] [US5] Unit test for invalid card notation validation in tests/unit/validation_tests.rs
- [X] T056 [P] [US5] Unit test for duplicate card detection in tests/unit/validation_tests.rs
- [X] T057 [P] [US5] Unit test for invalid game state validation in tests/unit/validation_tests.rs
- [X] T058 [US5] Integration test for CLI error messages in tests/integration/cli_tests.rs

### Implementation for User Story 5

- [X] T059 [US5] Implement card notation validation with detailed error messages in src/cli/validation.rs per cli-interface.md
- [X] T060 [US5] Implement duplicate card detection across hero/board/range in src/cli/validation.rs
- [X] T061 [US5] Implement game state consistency validation (pot vs stack) in src/cli/validation.rs
- [X] T062 [US5] Implement board card count validation per street in src/cli/validation.rs
- [X] T063 [US5] Wire validation to CLI with actionable error format in src/main.rs per SC-005
- [X] T064 [US5] Implement --help output per cli-interface.md Help Output section in src/cli/args.rs per FR-016
- [X] T065 [US5] Implement exit codes (0=success, 1=invalid input, 2=solver error) in src/main.rs per cli-interface.md

**Checkpoint**: All user stories functional - complete CLI with validation

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [X] T066 [P] Add doc comments to all public types and functions per constitution
- [X] T067 [P] Run cargo clippy and fix all warnings per constitution
- [X] T068 [P] Run cargo fmt for consistent formatting
- [X] T069 Run quickstart.md validation - test all example commands
- [X] T070 [P] Add additional unit tests to reach 80% coverage per constitution
- [ ] T071 Setup criterion benchmarks for solver in benches/solver_bench.rs per plan.md
- [X] T072 Final integration test: full end-to-end scenario per spec.md acceptance criteria

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-7)**: All depend on Foundational phase completion
  - US1 and US2 are both P1 and can proceed in parallel
  - US3, US4, US5 are all P2 and depend on US1 being functional
- **Polish (Phase 8)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational - Core solver, no dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational - Range parsing, independent of US1 initially
- **User Story 3 (P2)**: Depends on US1 (needs Strategy output to format)
- **User Story 4 (P2)**: Depends on US1 (needs Strategy output to serialize)
- **User Story 5 (P2)**: Can start after Foundational - Validation is independent but best after US1

### Within Each User Story

- Tests MUST be written and FAIL before implementation (TDD)
- Models before services/solver
- Solver components before CLI integration
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

**Phase 1 (Setup)**:
- T003, T004, T005 can run in parallel

**Phase 2 (Foundational)**:
- T006, T007, T008, T009 can run in parallel

**Phase 3 (User Story 1)**:
- T012, T013, T014 (tests) can run in parallel
- T016, T017, T018, T019, T021 (models) can run in parallel

**Phase 4 (User Story 2)**:
- T029, T030, T031, T032 (tests) can run in parallel

**Phase 5 (User Story 3)**:
- T039, T040 (tests) can run in parallel

**Phase 6 (User Story 4)**:
- T048 can run in parallel with previous phase completion

**Phase 7 (User Story 5)**:
- T055, T056, T057 (tests) can run in parallel

**Phase 8 (Polish)**:
- T066, T067, T068, T070, T071 can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Unit test for GameState validation in tests/unit/game_state_tests.rs"
Task: "Unit test for Action types and BetSize calculations in tests/unit/action_tests.rs"
Task: "Unit test for Strategy validation in tests/unit/strategy_tests.rs"

# Launch all model types together:
Task: "Implement Card, Rank, Suit types in src/models/card.rs"
Task: "Implement Hand type in src/models/hand.rs"
Task: "Implement Action, BetSize enums in src/models/action.rs"
Task: "Implement Position, Street enums in src/models/game_state.rs"
Task: "Implement Strategy, ActionStrategy structs in src/models/strategy.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 + 2 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (GTO Solver)
4. Complete Phase 4: User Story 2 (Range Parsing)
5. **STOP and VALIDATE**: Test basic solver with range input
6. Deploy/demo if ready (minimal viable solver)

### Incremental Delivery

1. Complete Setup + Foundational -> Foundation ready
2. Add User Story 1 + 2 -> Test independently -> Deploy/Demo (MVP!)
3. Add User Story 3 -> Test independently -> Deploy/Demo (human-readable output)
4. Add User Story 4 -> Test independently -> Deploy/Demo (JSON automation)
5. Add User Story 5 -> Test independently -> Deploy/Demo (production-ready validation)

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Solver)
   - Developer B: User Story 2 (Ranges)
3. After P1 stories complete:
   - Developer A: User Story 3 (Table output)
   - Developer B: User Story 4 (JSON output)
   - Developer C: User Story 5 (Validation)

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing (TDD per constitution)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Run `cargo clippy` and `cargo test` after each phase
