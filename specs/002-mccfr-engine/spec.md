# Feature Specification: MCCFR Solver Implementation

**Feature Branch**: `002-mccfr-engine`  
**Created**: 2026-01-10  
**Status**: Draft  
**Input**: User description: "Implement actual MCCFR for engine"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Compute Converged GTO Strategy (Priority: P1)

As a poker analyst, I want the solver to compute mathematically accurate GTO strategies using a proper regret minimization algorithm so that the output strategies reflect true Nash equilibrium play rather than arbitrary uniform distributions.

**Why this priority**: This is the core functionality - without actual strategy computation, the engine cannot fulfill its primary purpose of providing GTO analysis. The current stub implementation returns uniform strategies which have no strategic value.

**Independent Test**: Can be fully tested by running the solver on known game scenarios and verifying that output strategies converge to exploitability levels consistent with Nash equilibrium (strategies cannot be exploited beyond a small threshold by any counter-strategy).

**Acceptance Scenarios**:

1. **Given** a valid game state with hero cards, board, pot, stacks, and opponent range, **When** I run the solver, **Then** the returned strategy frequencies reflect computed Nash equilibrium values rather than uniform distribution
2. **Given** a simple decision point (e.g., heads-up river with limited actions), **When** I run the solver, **Then** the output matches known GTO solutions for that spot within acceptable tolerance
3. **Given** a game state where one action clearly dominates (e.g., nuts vs empty range), **When** I solve, **Then** the dominant action approaches 100% frequency
4. **Given** the same game state run multiple times, **When** comparing results, **Then** strategies are consistent (converge to same equilibrium within tolerance)

---

### User Story 2 - Track Training Convergence (Priority: P2)

As a poker analyst, I want to see how well the strategy has converged so that I can assess the reliability of the computed strategy and decide whether to run more iterations.

**Why this priority**: Convergence information allows users to trade off computation time vs accuracy. Without this, users cannot know if results are trustworthy.

**Independent Test**: Can be tested by running the solver and verifying that convergence metrics are included in output and that metrics improve (decrease) with more iterations.

**Acceptance Scenarios**:

1. **Given** I run the solver, **When** results are returned, **Then** the output includes a convergence metric indicating how close the strategy is to equilibrium
2. **Given** I run the solver with N iterations then with 2N iterations, **When** comparing convergence metrics, **Then** the higher iteration count shows better (lower) exploitability
3. **Given** a well-converged strategy, **When** I examine the convergence metric, **Then** the value is below a reasonable threshold (indicating near-equilibrium)

---

### User Story 3 - Configure Training Parameters (Priority: P2)

As a poker analyst, I want to control the number of training iterations so that I can balance computation time against strategy accuracy for my use case.

**Why this priority**: Different use cases require different accuracy levels. Quick analysis may accept less converged results, while publication-quality analysis needs higher precision.

**Independent Test**: Can be tested by specifying different iteration counts and verifying the solver respects the parameter and that more iterations produce better convergence.

**Acceptance Scenarios**:

1. **Given** I specify a custom iteration count via CLI parameter, **When** the solver runs, **Then** it executes the specified number of iterations
2. **Given** I do not specify an iteration count, **When** the solver runs, **Then** a reasonable default is used that produces usable results for typical scenarios
3. **Given** I specify a very low iteration count, **When** examining output, **Then** the convergence metric reflects the limited training (higher exploitability)

---

### User Story 4 - Handle Complex Game Trees Efficiently (Priority: P3)

As a poker analyst, I want the solver to handle realistic postflop scenarios with multiple bet sizes so that I can analyze real-world decisions rather than oversimplified toy games.

**Why this priority**: Real poker analysis involves complex decision trees. An engine limited to trivial scenarios has limited practical value.

**Independent Test**: Can be tested by providing game states with multiple available actions and verifying the solver produces valid strategies without excessive computation time or memory usage.

**Acceptance Scenarios**:

1. **Given** a flop scenario with check, bet 33%, bet 75%, and all-in options, **When** I run the solver, **Then** it produces valid mixed strategy frequencies across all actions
2. **Given** a scenario with both hero and opponent having multiple actions, **When** solving, **Then** the solver considers opponent responses when computing hero's strategy
3. **Given** a moderately complex scenario, **When** I run the solver, **Then** results are returned within reasonable time (not hanging indefinitely)

---

### Edge Cases

- What happens when the opponent range is empty after accounting for blockers?
- How does the solver handle scenarios with only one valid action available?
- What happens if the requested iteration count is zero or negative?
- How does the solver handle scenarios where hero is drawing dead against all opponent holdings?
- What happens if the game state represents an impossible situation (e.g., negative pot)?
- How does the system handle very deep stack scenarios with many possible raise sizes?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST compute strategy frequencies using regret minimization rather than returning uniform/arbitrary distributions
- **FR-002**: System MUST track cumulative regrets for each decision point to compute strategies
- **FR-003**: System MUST iterate through the game tree multiple times, updating regrets each iteration
- **FR-004**: System MUST compute strategy from regrets using regret matching (positive regrets normalized to probabilities)
- **FR-005**: System MUST handle both hero and opponent decision points when computing equilibrium
- **FR-006**: System MUST account for opponent range when computing expected values
- **FR-007**: System MUST support configurable iteration count via existing CLI parameter
- **FR-008**: System MUST use a reasonable default iteration count when not specified (e.g., 10,000 iterations)
- **FR-009**: System MUST report a convergence metric in the strategy output
- **FR-010**: System MUST produce consistent results when run multiple times on the same input
- **FR-011**: System MUST handle game states with varying numbers of available actions (2-10+ actions)
- **FR-012**: System MUST correctly handle terminal states (showdown, fold) with proper payoff calculation
- **FR-013**: System MUST account for card removal effects when traversing opponent hands
- **FR-014**: System MUST use Monte Carlo sampling to efficiently traverse large game trees
- **FR-015**: System MUST compute expected value for each action in the returned strategy

### Key Entities

- **Training Session**: A single invocation of the solver; encompasses all iterations, regret updates, and final strategy computation for a given game state
- **Information Set**: A grouping of game states indistinguishable to a player; the fundamental unit for strategy computation where regrets are tracked
- **Regret**: Cumulative counterfactual regret for not taking a specific action; used to compute strategy via regret matching
- **Strategy Profile**: Complete strategy specification for all information sets; maps each decision point to action probabilities
- **Convergence Metric**: Numerical measure of how close current strategy is to Nash equilibrium; lower values indicate better convergence (e.g., exploitability)
- **Counterfactual Value**: Expected payoff for a player at a decision point, assuming they played to reach that point; used to compute regrets

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Computed strategies are exploitable by less than 1% of the pot per decision for typical scenarios after default iterations
- **SC-002**: Strategy frequencies sum to 100% (within 0.1% rounding tolerance) for every decision point
- **SC-003**: Running the solver twice on identical input produces strategies within 5% frequency difference for each action
- **SC-004**: Typical single-street scenarios (river decisions) complete within 30 seconds using default iteration count
- **SC-005**: Multi-action scenarios (5+ actions) produce valid mixed strategies without timeout or error
- **SC-006**: Convergence metric improves (decreases) monotonically as iteration count increases (averaged over multiple runs)
- **SC-007**: Known solved toy games produce strategies matching theoretical GTO within 5% frequency per action

## Assumptions

- The existing CLI interface and game state model remain unchanged; MCCFR implementation integrates with current solver API
- Texas Hold'em No-Limit rules apply (hand rankings, betting rules)
- Single decision point analysis is the primary use case (not full hand tree solving)
- Users understand that more iterations = better accuracy but longer computation time
- The opponent is assumed to play GTO (no exploitative adjustments)
- Bet sizing options are discrete (limited to a set of specified sizes, not continuous)
- Memory constraints allow storing regrets for all information sets in the analyzed subtree
- Default parameters produce "good enough" results for most practical use cases
