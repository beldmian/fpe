//! Error types for the Poker GTO Strategy Engine

use thiserror::Error;

/// Errors that can occur during model operations
#[derive(Debug, Error)]
pub enum ModelError {
    /// Invalid card notation provided
    #[error("Invalid card '{0}': expected format like 'Ah', 'Kd', 'Ts'")]
    InvalidCard(String),

    /// Invalid range notation provided
    #[error("Invalid range notation '{0}': {1}")]
    InvalidRange(String, String),

    /// Duplicate card detected across hero/board/range
    #[error("Duplicate card '{0}' appears in multiple places")]
    DuplicateCard(String),

    /// Invalid game state detected
    #[error("Invalid game state: {0}")]
    InvalidGameState(String),

    /// Invalid board card count for the street
    #[error("Invalid board: expected {expected} cards for {street:?}, got {actual}")]
    InvalidBoard {
        /// The current street
        street: String,
        /// Expected number of cards
        expected: usize,
        /// Actual number of cards provided
        actual: usize,
    },

    /// Bet size exceeds available stack
    #[error("Impossible bet size: {0} exceeds effective stack {1}")]
    ImpossibleBetSize(f64, f64),

    /// Range is empty after blocker removal
    #[error("Empty range after removing blockers")]
    EmptyRange,
}

/// Result type for model operations
pub type Result<T> = std::result::Result<T, ModelError>;
