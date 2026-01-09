# Data Model: Poker GTO Strategy Engine

**Feature**: 001-poker-engine-cli
**Date**: 2026-01-09
**Status**: Complete

## Overview

This document defines the core data structures for the Poker GTO Strategy Engine. Entities are derived from the feature specification's Key Entities section and functional requirements.

---

## Entity Definitions

### Card

Represents a single playing card in a standard 52-card deck.

```rust
/// A single playing card with rank and suit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

/// Card rank (2-A)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

/// Card suit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Suit {
    Hearts,   // h
    Diamonds, // d
    Clubs,    // c
    Spades,   // s
}
```

**Validation Rules**:
- Card notation must match pattern: `[2-9TJQKA][hdcs]`
- Examples: `Ah`, `Kd`, `Ts`, `2c`
- No duplicate cards allowed in any context (board + hero cards + villain cards)

**String Representation**:
- Input: `"AhKd"` (two cards concatenated)
- Display: `A♥`, `K♦`, `T♠`, `2♣`

---

### Hand

Represents a player's hole cards (exactly 2 cards in Texas Hold'em).

```rust
/// A player's hole cards (exactly 2 cards)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hand {
    pub cards: [Card; 2],
}

impl Hand {
    /// Returns true if both cards have the same suit
    pub fn is_suited(&self) -> bool;

    /// Returns true if both cards have the same rank
    pub fn is_pair(&self) -> bool;

    /// Returns the hand in canonical notation (e.g., "AKs", "QQ", "T9o")
    pub fn notation(&self) -> String;
}
```

**Validation Rules**:
- Must contain exactly 2 cards
- Cards must be distinct (no duplicates)
- Cards cannot appear in board or opponent's holdings

---

### Range

Collection of possible hole card combinations representing a player's range.

```rust
/// A collection of possible hole card combinations with weights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    /// Map from hand combination to weight (0.0-1.0)
    hands: HashMap<Hand, f64>,
}

impl Range {
    /// Parse range from Equilab-style notation
    /// Examples: "AA,KK,QQ", "AKs", "TT+", "top 15%"
    pub fn from_notation(notation: &str) -> Result<Self, RangeError>;

    /// Returns all hands in the range
    pub fn hands(&self) -> impl Iterator<Item = (&Hand, f64)>;

    /// Returns number of hand combinations
    pub fn num_combos(&self) -> usize;

    /// Remove combos that conflict with known cards (blockers)
    pub fn remove_blockers(&mut self, cards: &[Card]);

    /// Returns true if range contains the specified hand
    pub fn contains(&self, hand: &Hand) -> bool;
}
```

**Supported Notation** (from research.md Decision 7):
| Format | Example | Combos |
|--------|---------|--------|
| Pair | `AA` | 6 |
| Suited | `AKs` | 4 |
| Offsuit | `AKo` | 12 |
| Any | `AK` | 16 |
| Plus notation | `TT+` | 60 (TT-AA) |
| Dash notation | `TT-77` | 24 |
| Percentage | `top 15%` | ~150 |
| Weighted | `AA@50` | 3 (50% of AA) |
| Combined | `AA,KK,AKs` | 22 |

**Validation Rules**:
- Notation must be valid Equilab format
- Empty range is invalid
- Weights must be in range [0.0, 1.0]

---

### GameState

Complete description of a decision point including all context needed for GTO calculation.

```rust
/// Complete game state at a decision point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    /// Hero's hole cards
    pub hero_hand: Hand,

    /// Community cards (0-5 cards)
    pub board: Vec<Card>,

    /// Current pot size in big blinds
    pub pot_size: f64,

    /// Effective stack size in big blinds
    pub effective_stack: f64,

    /// Amount hero needs to call (0 if checking)
    pub to_call: f64,

    /// Hero's position relative to opponent
    pub position: Position,

    /// Opponent's range
    pub villain_range: Range,

    /// Current street
    pub street: Street,

    /// Available actions for hero at this decision point
    pub available_actions: Vec<Action>,
}

/// Hero's position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Position {
    /// In position (acts last)
    IP,
    /// Out of position (acts first)
    OOP,
}

/// Current street in the hand
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Street {
    Preflop,
    Flop,
    Turn,
    River,
}
```

**Validation Rules**:
- `pot_size` must be > 0
- `effective_stack` must be > 0
- `to_call` must be >= 0 and <= `effective_stack`
- Board must have correct number of cards for street:
  - Preflop: 0 cards
  - Flop: 3 cards
  - Turn: 4 cards
  - River: 5 cards
- No card can appear in multiple places (hero hand, board, villain range)
- `pot_size` + `to_call` cannot exceed total chips in play

**State Transitions**:
```
Preflop (0 cards) → Flop (3 cards) → Turn (4 cards) → River (5 cards)
```

---

### Action

A possible decision at a game state node.

```rust
/// A possible action at a decision point
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    /// Surrender the pot
    Fold,

    /// Pass action (when no bet to call)
    Check,

    /// Match the current bet
    Call,

    /// Make a bet (when no bet to call)
    Bet(BetSize),

    /// Increase the current bet
    Raise(BetSize),

    /// Commit all remaining chips
    AllIn,
}

/// Bet sizing as fraction of pot or absolute amount
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BetSize {
    /// Fraction of pot (e.g., 0.33, 0.5, 0.75, 1.0)
    PotFraction(f64),

    /// Fixed amount in big blinds
    Amount(f64),
}

impl Action {
    /// Returns the amount committed by this action given pot and stack
    pub fn amount(&self, pot: f64, stack: f64, to_call: f64) -> f64;

    /// Returns display name for the action
    pub fn display_name(&self) -> String;
}
```

**Standard Bet Sizes** (from research.md Decision 6):
| Context | Available Sizes |
|---------|-----------------|
| Preflop open | 2.5x, 3x, All-in |
| Postflop bet | 33%, 50%, 75%, 100% pot |
| Facing bet | Min-raise, 50% pot raise, Pot raise, All-in |

**Validation Rules**:
- Bet/Raise size must be >= minimum raise
- Bet/Raise size cannot exceed effective stack
- Check only valid when to_call == 0
- Call only valid when to_call > 0

---

### Strategy

Output of GTO calculation mapping actions to frequencies and expected values.

```rust
/// GTO strategy output for a decision point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Strategy {
    /// Input game state
    pub game_state: GameState,

    /// Strategy for each action
    pub actions: Vec<ActionStrategy>,

    /// Number of CFR iterations run
    pub iterations: u32,

    /// Convergence metric (Nash distance approximation)
    pub convergence: f64,
}

/// Strategy for a single action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionStrategy {
    /// The action
    pub action: Action,

    /// Frequency to take this action (0.0-1.0)
    pub frequency: f64,

    /// Expected value in big blinds
    pub ev: f64,
}

impl Strategy {
    /// Returns true if frequencies sum to 1.0 (within tolerance)
    pub fn is_valid(&self) -> bool;

    /// Returns action with highest EV
    pub fn best_action(&self) -> &ActionStrategy;

    /// Returns actions sorted by frequency (highest first)
    pub fn sorted_by_frequency(&self) -> Vec<&ActionStrategy>;
}
```

**Validation Rules**:
- Frequencies must sum to 1.0 (within ε = 0.001 tolerance)
- Each frequency must be in range [0.0, 1.0]
- At least one action must have frequency > 0

**Invariants**:
- `actions` contains all available actions from `game_state`
- Actions are mutually exclusive (exactly one chosen per iteration)

---

### Equity

Probability of winning/tying against opponent range.

```rust
/// Equity calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equity {
    /// Probability of winning (0.0-1.0)
    pub win: f64,

    /// Probability of tying (0.0-1.0)
    pub tie: f64,

    /// Probability of losing (0.0-1.0)
    pub lose: f64,
}

impl Equity {
    /// Returns total equity (win + tie/2)
    pub fn total(&self) -> f64 {
        self.win + self.tie / 2.0
    }
}
```

**Invariants**:
- `win + tie + lose == 1.0` (within tolerance)
- All values in range [0.0, 1.0]

---

## Entity Relationships

```
┌─────────────┐     ┌─────────────┐
│    Card     │────▶│    Hand     │ (2 cards)
└─────────────┘     └─────────────┘
       │                   │
       │                   ▼
       │            ┌─────────────┐
       └───────────▶│    Range    │ (many hands with weights)
                    └─────────────┘
                           │
                           ▼
┌─────────────┐     ┌─────────────┐
│   Action    │────▶│  GameState  │
└─────────────┘     └─────────────┘
       │                   │
       │                   ▼
       │            ┌─────────────┐
       └───────────▶│  Strategy   │
                    └─────────────┘
                           │
                           ▼
                    ┌─────────────┐
                    │   Equity    │ (computed per action)
                    └─────────────┘
```

---

## Error Types

```rust
/// Errors that can occur during model operations
#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("Invalid card '{0}': expected format like 'Ah', 'Kd', 'Ts'")]
    InvalidCard(String),

    #[error("Invalid range notation '{0}': {1}")]
    InvalidRange(String, String),

    #[error("Duplicate card '{0}' appears in multiple places")]
    DuplicateCard(Card),

    #[error("Invalid game state: {0}")]
    InvalidGameState(String),

    #[error("Invalid board: expected {expected} cards for {street}, got {actual}")]
    InvalidBoard {
        street: Street,
        expected: usize,
        actual: usize,
    },

    #[error("Impossible bet size: {0} exceeds effective stack {1}")]
    ImpossibleBetSize(f64, f64),

    #[error("Empty range after removing blockers")]
    EmptyRange,
}
```

---

## JSON Schema

Output JSON schema for `--json` flag (from Strategy):

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "GTO Strategy Output",
  "type": "object",
  "required": ["input", "strategy", "iterations", "convergence"],
  "properties": {
    "input": {
      "type": "object",
      "properties": {
        "hero": { "type": "string", "pattern": "^[2-9TJQKA][hdcs]{2}$" },
        "board": { "type": "string" },
        "villain_range": { "type": "string" },
        "pot": { "type": "number", "minimum": 0 },
        "stack": { "type": "number", "minimum": 0 },
        "to_call": { "type": "number", "minimum": 0 },
        "position": { "type": "string", "enum": ["IP", "OOP"] }
      }
    },
    "strategy": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["action", "frequency", "ev"],
        "properties": {
          "action": { "type": "string" },
          "frequency": { "type": "number", "minimum": 0, "maximum": 1 },
          "ev": { "type": "number" }
        }
      }
    },
    "iterations": { "type": "integer", "minimum": 1 },
    "convergence": { "type": "number", "minimum": 0 }
  }
}
```
