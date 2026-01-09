//! Poker action types

use serde::{Deserialize, Serialize};

/// Bet sizing as fraction of pot or absolute amount
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BetSize {
    /// Fraction of pot (e.g., 0.33, 0.5, 0.75, 1.0)
    PotFraction(f64),
    /// Fixed amount in big blinds
    Amount(f64),
}

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

impl Action {
    /// Returns the amount committed by this action given pot and stack
    pub fn amount(&self, pot: f64, stack: f64, to_call: f64) -> f64 {
        match self {
            Action::Fold => 0.0,
            Action::Check => 0.0,
            Action::Call => to_call,
            Action::Bet(size) => match size {
                BetSize::PotFraction(fraction) => (pot * fraction).min(stack),
                BetSize::Amount(amt) => amt.min(stack),
            },
            Action::Raise(size) => match size {
                BetSize::PotFraction(fraction) => (to_call + pot * fraction).min(stack),
                BetSize::Amount(amt) => amt.min(stack),
            },
            Action::AllIn => stack,
        }
    }

    /// Returns display name for the action
    pub fn display_name(&self) -> String {
        match self {
            Action::Fold => "Fold".to_string(),
            Action::Check => "Check".to_string(),
            Action::Call => "Call".to_string(),
            Action::Bet(size) => match size {
                BetSize::PotFraction(f) => format!("Bet {:.0}% pot", f * 100.0),
                BetSize::Amount(a) => format!("Bet {} BB", a),
            },
            Action::Raise(size) => match size {
                BetSize::PotFraction(f) => format!("Raise {:.0}% pot", f * 100.0),
                BetSize::Amount(a) => format!("Raise to {} BB", a),
            },
            Action::AllIn => "All-In".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_amounts() {
        let pot = 10.0;
        let stack = 100.0;
        let to_call = 5.0;

        assert_eq!(Action::Fold.amount(pot, stack, to_call), 0.0);
        assert_eq!(Action::Check.amount(pot, stack, to_call), 0.0);
        assert_eq!(Action::Call.amount(pot, stack, to_call), 5.0);
        assert_eq!(
            Action::Bet(BetSize::PotFraction(0.5)).amount(pot, stack, to_call),
            5.0
        );
        assert_eq!(Action::AllIn.amount(pot, stack, to_call), 100.0);
    }

    #[test]
    fn test_display_names() {
        assert_eq!(Action::Fold.display_name(), "Fold");
        assert_eq!(Action::Check.display_name(), "Check");
        assert_eq!(
            Action::Bet(BetSize::PotFraction(0.5)).display_name(),
            "Bet 50% pot"
        );
    }
}
