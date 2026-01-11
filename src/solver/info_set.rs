//! Information set abstraction for MCCFR.
//!
//! This module defines the `InfoSetKey` struct and `SprBucket` enum used to
//! group similar game states into information sets for strategy computation.

use crate::models::{game_state::GameState, game_state::Position, hand::Hand};

/// Discretized stack-to-pot ratio for info set grouping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SprBucket {
    /// SPR 0-2: Commitment threshold, simplified decisions
    Short,
    /// SPR 2-5: Standard postflop play
    Medium,
    /// SPR 5-10: Deep stack considerations
    Deep,
    /// SPR 10+: Maximum flexibility
    VeryDeep,
}

impl SprBucket {
    /// Create a bucket from a raw SPR value.
    pub fn from_spr(spr: f64) -> Self {
        if spr < 2.0 {
            SprBucket::Short
        } else if spr < 5.0 {
            SprBucket::Medium
        } else if spr < 10.0 {
            SprBucket::Deep
        } else {
            SprBucket::VeryDeep
        }
    }
}

/// Unique identifier for an information set.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InfoSetKey {
    /// Hero's hole cards
    pub hero_hand: Hand,
    /// Stack-to-pot ratio bucket
    pub spr_bucket: SprBucket,
    /// Hero's position
    pub position: Position,
}

impl InfoSetKey {
    /// Create an info set key from a game state.
    pub fn from_game_state(state: &GameState) -> Self {
        let spr = if state.pot_size > 0.0 {
            state.effective_stack / state.pot_size
        } else {
            0.0
        };

        Self {
            hero_hand: state.hero_hand.clone(),
            spr_bucket: SprBucket::from_spr(spr),
            position: state.position,
        }
    }
}
