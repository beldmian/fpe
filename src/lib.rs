//! Poker GTO Strategy Engine
//!
//! A library for computing Game Theory Optimal (GTO) strategies for
//! Texas Hold'em poker using Monte Carlo Counterfactual Regret Minimization (MCCFR).

#![warn(missing_docs)]

/// Data models for poker game states and strategies
pub mod models;

/// GTO solver implementation
pub mod solver;

/// CLI interface components
pub mod cli;

/// Error types
pub mod error;

// Re-export commonly used types
pub use error::{ModelError, Result};
