//! Strategy output representation

use crate::models::action::Action;
use crate::models::game_state::GameState;
use serde::{Deserialize, Serialize};

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

/// GTO strategy output for a decision point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Strategy {
    /// Input game state (omitted in some serializations to reduce size)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_state: Option<GameState>,

    /// Strategy for each action
    pub actions: Vec<ActionStrategy>,

    /// Number of CFR iterations run
    pub iterations: u32,

    /// Convergence metric (Nash distance approximation)
    pub convergence: f64,
}

impl Strategy {
    /// Create a new strategy
    pub fn new(actions: Vec<ActionStrategy>, iterations: u32, convergence: f64) -> Self {
        Self {
            game_state: None,
            actions,
            iterations,
            convergence,
        }
    }

    /// Returns true if frequencies sum to 1.0 (within tolerance)
    pub fn is_valid(&self) -> bool {
        let sum: f64 = self.actions.iter().map(|a| a.frequency).sum();
        (sum - 1.0).abs() < 0.001
    }

    /// Returns action with highest EV
    pub fn best_action(&self) -> Option<&ActionStrategy> {
        self.actions
            .iter()
            .max_by(|a, b| a.ev.partial_cmp(&b.ev).unwrap())
    }

    /// Returns actions sorted by frequency (highest first)
    pub fn sorted_by_frequency(&self) -> Vec<&ActionStrategy> {
        let mut sorted: Vec<&ActionStrategy> = self.actions.iter().collect();
        sorted.sort_by(|a, b| b.frequency.partial_cmp(&a.frequency).unwrap());
        sorted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::action::{Action, BetSize};

    #[test]
    fn test_strategy_validity() {
        let actions = vec![
            ActionStrategy {
                action: Action::Fold,
                frequency: 0.5,
                ev: 0.0,
            },
            ActionStrategy {
                action: Action::Call,
                frequency: 0.5,
                ev: 1.0,
            },
        ];

        let strategy = Strategy::new(actions, 10000, 0.001);
        assert!(strategy.is_valid());
    }

    #[test]
    fn test_best_action() {
        let actions = vec![
            ActionStrategy {
                action: Action::Fold,
                frequency: 0.3,
                ev: 0.0,
            },
            ActionStrategy {
                action: Action::Call,
                frequency: 0.7,
                ev: 2.5,
            },
        ];

        let strategy = Strategy::new(actions, 10000, 0.001);
        let best = strategy.best_action().unwrap();
        assert_eq!(best.ev, 2.5);
    }

    #[test]
    fn test_sorted_by_frequency() {
        let actions = vec![
            ActionStrategy {
                action: Action::Fold,
                frequency: 0.2,
                ev: 0.0,
            },
            ActionStrategy {
                action: Action::Call,
                frequency: 0.5,
                ev: 1.0,
            },
            ActionStrategy {
                action: Action::Bet(BetSize::PotFraction(0.5)),
                frequency: 0.3,
                ev: 1.5,
            },
        ];

        let strategy = Strategy::new(actions, 10000, 0.001);
        let sorted = strategy.sorted_by_frequency();
        assert_eq!(sorted[0].frequency, 0.5);
        assert_eq!(sorted[1].frequency, 0.3);
        assert_eq!(sorted[2].frequency, 0.2);
    }
}
