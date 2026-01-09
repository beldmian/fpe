# Feature Specification: Poker GTO Strategy Engine with CLI Interface

**Feature Branch**: `001-poker-engine-cli`  
**Created**: 2026-01-09  
**Status**: Draft  
**Input**: User description: "Create a poker engine with simple CLI interface for math modeling of optimal strategy"

## Clarifications

### Session 2026-01-09

- Q: What should be the primary calculation output? → A: Full Game Theory Optimal (GTO) strategy with mixed frequencies
- Q: What game tree scope should the engine handle? → A: Single decision point analysis (given current state, compute optimal strategy for one spot)
- Q: How should users specify the game situation to analyze? → A: Structured CLI parameters
- Q: How should the engine handle opponent ranges? → A: User-specified opponent ranges
- Q: How should the GTO strategy results be presented? → A: Both formats available (default human-readable, flag for JSON output)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Calculate GTO Strategy for a Decision Point (Priority: P1)

As a poker analyst, I want to input a specific game situation and receive the Game Theory Optimal strategy with action frequencies so that I can understand the mathematically correct play.

**Why this priority**: This is the core value proposition - without GTO calculation, the engine has no purpose. All other features depend on this working correctly.

**Independent Test**: Can be fully tested by providing a game state (hero cards, board, pot, stacks, opponent range) and verifying the output contains valid action frequencies that sum to 100% with corresponding EV calculations.

**Acceptance Scenarios**:

1. **Given** a valid game state with hero cards, board, pot size, stack sizes, and opponent range, **When** I run the analysis command, **Then** I receive a GTO strategy showing each possible action with its frequency and expected value
2. **Given** a preflop scenario with positions and stack depths, **When** I specify the opponent's opening range, **Then** the engine calculates optimal response frequencies (fold/call/raise percentages)
3. **Given** a postflop scenario, **When** I run the analysis, **Then** the output includes optimal bet sizing frequencies where applicable (e.g., "bet 33% pot: 45%, bet 75% pot: 30%, check: 25%")

---

### User Story 2 - Specify Opponent Hand Ranges (Priority: P1)

As a poker analyst, I want to define opponent hand ranges precisely so that I can model realistic scenarios against specific player types.

**Why this priority**: GTO calculations require opponent range assumptions. Without flexible range input, analysis would be limited to unrealistic scenarios.

**Independent Test**: Can be tested by inputting various range formats and verifying the engine correctly interprets and uses them in calculations.

**Acceptance Scenarios**:

1. **Given** I want to specify a range, **When** I use standard poker notation (e.g., "AA,KK,QQ,AKs,AKo"), **Then** the engine correctly parses and applies that range
2. **Given** I want to specify a percentage-based range, **When** I input "top 15%", **Then** the engine expands this to the corresponding hands
3. **Given** I input an invalid range syntax, **When** I run the command, **Then** I receive a clear error message explaining the correct format

---

### User Story 3 - Get Human-Readable Strategy Output (Priority: P2)

As a poker analyst, I want to see results in a clear, formatted table so that I can quickly understand the optimal strategy.

**Why this priority**: Human readability is essential for interactive analysis and learning. This is the default output mode.

**Independent Test**: Can be tested by running an analysis and verifying output contains properly formatted tables with aligned columns showing actions, frequencies, and EVs.

**Acceptance Scenarios**:

1. **Given** I run an analysis without format flags, **When** results are displayed, **Then** I see a formatted ASCII table with actions, frequencies (as percentages), and EV values
2. **Given** a complex strategy with multiple actions, **When** results display, **Then** actions are sorted by frequency (highest first) with clear labels
3. **Given** the strategy includes bet sizing options, **When** results display, **Then** each bet size is shown as a separate row with its frequency and EV

---

### User Story 4 - Export Strategy as JSON (Priority: P2)

As a poker analyst, I want to export results in JSON format so that I can integrate with other analysis tools or build automation pipelines.

**Why this priority**: Machine-readable output enables integration with databases, visualization tools, and batch processing workflows.

**Independent Test**: Can be tested by running analysis with JSON flag and verifying output is valid JSON containing all strategy data.

**Acceptance Scenarios**:

1. **Given** I run analysis with `--json` flag, **When** results are returned, **Then** output is valid JSON with action frequencies, EVs, and input parameters
2. **Given** I want to process multiple scenarios, **When** I script multiple CLI calls with JSON output, **Then** each result can be parsed and aggregated programmatically

---

### User Story 5 - Validate Input Parameters (Priority: P2)

As a poker analyst, I want clear validation and error messages so that I can quickly correct invalid inputs.

**Why this priority**: Invalid inputs should fail fast with helpful messages rather than producing incorrect results.

**Independent Test**: Can be tested by providing various invalid inputs and verifying appropriate error messages are returned.

**Acceptance Scenarios**:

1. **Given** I input invalid card notation (e.g., "XX" or duplicate cards), **When** I run the command, **Then** I receive a specific error identifying the invalid card
2. **Given** I input an impossible game state (e.g., pot larger than total chips), **When** I run the command, **Then** I receive an error explaining the inconsistency
3. **Given** I omit required parameters, **When** I run the command, **Then** I receive a usage message listing required parameters

---

### Edge Cases

- What happens when the opponent range is empty or contains no valid hands?
- How does the system handle scenarios where only one action is mathematically valid (e.g., must fold or must call)?
- What happens if hero's hand blocks significant portions of villain's range?
- How does the engine handle all-in situations where no further decisions exist?
- What happens when stack sizes create non-standard bet sizing options?
- How does the system handle calculations that would take excessive time?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST calculate Nash equilibrium strategies for single decision points in Texas Hold'em
- **FR-002**: System MUST output action frequencies as percentages that sum to 100% for each decision
- **FR-003**: System MUST calculate Expected Value (EV) for each available action
- **FR-004**: System MUST accept hero hole cards via CLI parameter in standard notation (e.g., AhKd, AsKs)
- **FR-005**: System MUST accept community cards (board) via CLI parameter for postflop analysis
- **FR-006**: System MUST accept pot size and effective stack sizes as numeric CLI parameters
- **FR-007**: System MUST accept opponent hand ranges in standard notation (e.g., "AA,KK,AKs,AKo" or "top 10%")
- **FR-008**: System MUST support preflop scenarios (no board) and postflop scenarios (flop, turn, river)
- **FR-009**: System MUST output human-readable formatted tables by default
- **FR-010**: System MUST output valid JSON when `--json` flag is provided
- **FR-011**: System MUST validate all inputs and return clear error messages for invalid parameters
- **FR-012**: System MUST correctly evaluate hand equity using all possible remaining card runouts
- **FR-013**: System MUST account for card removal effects (blockers) when calculating ranges
- **FR-014**: System MUST support standard poker actions: fold, check, call, bet (with sizing), raise (with sizing)
- **FR-015**: System MUST handle all-in scenarios where stack sizes limit available actions
- **FR-016**: System MUST provide usage/help information when run without parameters or with `--help`

### Key Entities

- **GameState**: Complete description of a decision point; includes hero cards, board cards, pot size, stack sizes, position, and betting history context
- **Range**: Collection of possible hole card combinations with optional weights; represents opponent's possible holdings
- **Strategy**: Output of GTO calculation; maps each available action to a frequency (0-100%) and expected value
- **Action**: A possible decision at a node; includes action type (fold/check/call/bet/raise) and sizing where applicable
- **Card**: Individual playing card with suit (h/d/c/s) and rank (2-9, T, J, Q, K, A)
- **Equity**: Probability of winning/tying against opponent range given current cards and possible runouts

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can input a complete game state and receive GTO strategy output within 30 seconds for typical scenarios
- **SC-002**: Strategy frequencies are mathematically valid (sum to 100% within rounding tolerance)
- **SC-003**: All valid poker hand notations are correctly parsed without errors
- **SC-004**: JSON output is valid and parseable by standard JSON libraries
- **SC-005**: Error messages identify the specific invalid parameter and expected format
- **SC-006**: Calculated equities match established poker equity calculators within 0.1% tolerance
- **SC-007**: Users can specify any opponent range using standard poker range notation

## Assumptions

- The poker variant is No-Limit Texas Hold'em
- Standard poker hand rankings are used (high card through royal flush)
- The CLI operates in a terminal/console environment with text input/output
- Single decision point analysis (not full game tree solving)
- Users have poker knowledge and understand range notation, GTO concepts, and EV
- Calculations assume rational, GTO-playing opponents (no exploitative adjustments)
- Bet sizing options are simplified (e.g., 33%, 50%, 75%, 100% pot) rather than continuous
- The engine runs locally without network dependencies
