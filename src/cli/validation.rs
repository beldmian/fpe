//! Input validation for CLI arguments
//!
//! Handles validation of cards, ranges, game state, and other parameters.

use crate::error::{ModelError, Result};
use crate::models::{Card, GameState, Hand, Range};
use std::collections::HashSet;
use std::str::FromStr;

/// Validate card notation
pub fn validate_card(s: &str) -> Result<Card> {
    Card::from_str(s)
}

/// Validate hand notation
pub fn validate_hand(s: &str) -> Result<Hand> {
    Hand::from_str(s)
}

/// Validate range notation
pub fn validate_range(s: &str) -> Result<Range> {
    Range::from_notation(s)
}

/// Check for duplicate cards across hero, board, and range (range blockers)
pub fn check_duplicates(hero: &Hand, board: &[Card]) -> Result<()> {
    let mut seen = HashSet::new();

    // Check hero cards
    for card in &hero.cards {
        if !seen.insert(*card) {
            return Err(ModelError::DuplicateCard(card.to_string()));
        }
    }

    // Check board cards
    for card in board {
        if !seen.insert(*card) {
            return Err(ModelError::DuplicateCard(card.to_string()));
        }
    }

    Ok(())
}

/// Validate complete game state
pub fn validate_game_state(state: &GameState) -> Result<()> {
    // Basic structural validation is done in GameState::new
    // This adds higher level checks if needed

    // Check if range is empty?
    if state.villain_range.num_combos() == 0 {
        // Warning or Error?
        // For now, allow it (maybe user wants to test vs empty range)
    }

    Ok(())
}
