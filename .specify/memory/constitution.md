<!--
Sync Impact Report
==================
Version change: (new) → 1.0.0
Modified principles: N/A (initial constitution)
Added sections:
  - Core Principles (4 principles: Code Quality, Testing Standards, UX Consistency, Performance)
  - Quality Gates
  - Development Workflow
  - Governance
Removed sections: N/A (initial constitution)
Templates requiring updates:
  - .specify/templates/plan-template.md ✅ (no changes needed - uses dynamic constitution reference)
  - .specify/templates/spec-template.md ✅ (no changes needed - compatible)
  - .specify/templates/tasks-template.md ✅ (no changes needed - compatible)
  - .specify/templates/checklist-template.md ✅ (no changes needed - compatible)
Follow-up TODOs: None
-->

# FPE Constitution

## Core Principles

### I. Code Quality

All code contributions MUST meet the following quality standards:

- **Linting**: Code MUST pass `cargo clippy` with zero warnings before merge
- **Documentation**: Public functions and types MUST have doc comments explaining
  purpose, parameters, return values, and potential errors
- **Examples**: Public APIs SHOULD include usage examples in documentation
- **Complexity**: Functions SHOULD maintain cyclomatic complexity below 10;
  exceptions MUST be justified in code review
- **Idioms**: Code MUST follow idiomatic Rust patterns including proper ownership,
  Result-based error handling, and appropriate use of traits

**Rationale**: Consistent code quality reduces maintenance burden, improves
onboarding, and prevents defect accumulation.

### II. Testing Standards

Testing is mandatory for all production code:

- **Coverage**: Test coverage MUST maintain a minimum of 80% line coverage for
  new code; overall project coverage SHOULD trend upward
- **Test-First**: New features SHOULD follow test-driven development; tests MUST
  exist before code is merged
- **Isolation**: Unit tests MUST be isolated with no external dependencies
  (network, filesystem, database)
- **Integration**: Integration tests MUST cover all public API entry points and
  critical user workflows
- **Determinism**: All tests MUST be deterministic and reproducible; flaky tests
  MUST be fixed or quarantined immediately

**Rationale**: Comprehensive testing catches regressions early and enables
confident refactoring.

### III. User Experience Consistency

All user-facing interfaces MUST provide consistent, predictable experiences:

- **CLI Naming**: Commands MUST follow verb-noun naming conventions
  (e.g., `fpe encode`, `fpe decode`)
- **Error Messages**: Errors MUST be actionable, including what went wrong and
  how to fix it; error codes SHOULD be documented
- **Output Formats**: Commands MUST support human-readable output by default and
  machine-parseable output (JSON) via flag (e.g., `--json`)
- **Breaking Changes**: Changes to user-facing behavior MUST be documented in
  CHANGELOG.md and follow semantic versioning
- **Help Text**: All commands MUST provide comprehensive `--help` output

**Rationale**: Predictable interfaces reduce user friction and support automation.

### IV. Performance Requirements

Performance MUST be measured, documented, and maintained:

- **Latency Budgets**: Critical operations MUST have documented latency targets;
  these targets MUST be verified by benchmarks
- **Memory Limits**: Memory usage MUST not exceed documented limits for standard
  workloads; memory leaks are treated as critical bugs
- **Regression Prevention**: Benchmark tests MUST run in CI; performance
  regressions exceeding 10% MUST block merge
- **Profiling**: Critical code paths SHOULD be profiled before optimization;
  optimizations MUST be justified by profiling data

**Rationale**: Defined performance standards prevent gradual degradation and
ensure predictable behavior under load.

## Quality Gates

All changes MUST pass these gates before merge:

- All CI checks (build, lint, test) MUST pass
- Code review by at least one team member MUST be completed
- All `cargo clippy` warnings MUST be resolved
- Test suite MUST pass with no failures or skipped tests (unless explicitly
  quarantined)
- Documentation MUST be updated for any public API changes
- CHANGELOG.md MUST be updated for user-facing changes

## Development Workflow

The following workflow standards apply to all contributions:

- **Branching**: Feature branches MUST be created from `main` for all changes
- **Commits**: Commit messages MUST follow conventional commit format
  (e.g., `feat:`, `fix:`, `docs:`, `refactor:`)
- **Pull Requests**: PRs MUST reference relevant issues or specs when applicable
- **Review**: Self-review MUST be completed before requesting team review
- **Merging**: Squash merge is RECOMMENDED for feature branches; merge commits
  for long-running branches

## Governance

This constitution is the authoritative source for FPE development standards.
All code reviews and architectural decisions MUST reference these principles.

**Amendment Process**:
1. Proposed changes MUST be documented with rationale
2. Changes MUST be reviewed by at least two team members
3. Breaking changes to principles require migration plan for existing code
4. Version MUST be incremented per semantic versioning rules

**Versioning Policy**:
- MAJOR: Backward-incompatible principle removals or redefinitions
- MINOR: New principles added or existing guidance materially expanded
- PATCH: Clarifications, wording improvements, non-semantic refinements

**Compliance**: Adherence to this constitution is verified during code review.
Exceptions MUST be documented and approved by team consensus.

**Version**: 1.0.0 | **Ratified**: 2026-01-09 | **Last Amended**: 2026-01-09
