//! Equity calculation

use crate::models::card::Card;
use crate::models::hand::Hand;
use crate::models::range::Range;
use crate::solver::evaluator::evaluate_hand;
use serde::{Deserialize, Serialize};

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
    /// Create new equity result
    pub fn new(win: f64, tie: f64, lose: f64) -> Self {
        Self { win, tie, lose }
    }
}

/// Calculate equity of hero hand vs villain range on board
pub fn calculate_equity(hero_hand: &Hand, villain_range: &Range, board: &[Card]) -> Equity {
    let mut wins = 0.0;
    let mut ties = 0.0;
    let mut losses = 0.0;
    let mut total_weight = 0.0;

    let hero_score = evaluate_hand(hero_hand, board);

    for (villain_hand, weight) in villain_range.hands() {
        if shares_cards(hero_hand, villain_hand) || shares_board(villain_hand, board) {
            continue;
        }

        let villain_score = evaluate_hand(villain_hand, board);

        if hero_score > villain_score {
            wins += weight;
        } else if hero_score < villain_score {
            losses += weight;
        } else {
            ties += weight;
        }
        total_weight += weight;
    }

    if total_weight == 0.0 {
        return Equity::new(0.0, 0.0, 0.0);
    }

    Equity::new(
        wins / total_weight,
        ties / total_weight,
        losses / total_weight,
    )
}

fn shares_cards(h1: &Hand, h2: &Hand) -> bool {
    h1.cards[0] == h2.cards[0]
        || h1.cards[0] == h2.cards[1]
        || h1.cards[1] == h2.cards[0]
        || h1.cards[1] == h2.cards[1]
}

fn shares_board(h: &Hand, board: &[Card]) -> bool {
    for card in board {
        if h.cards[0] == *card || h.cards[1] == *card {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_equity_calculation() {
        let hero = Hand::from_str("AhAs").unwrap();
        let board = vec![
            Card::from_str("Ks").unwrap(),
            Card::from_str("Qh").unwrap(),
            Card::from_str("Jd").unwrap(),
        ];

        // Villain range: KhKd (Set), 2c3c (Miss)
        let mut range = Range::new();
        range.hands.insert(Hand::from_str("KhKd").unwrap(), 1.0); // Set of Ks (beats AA)
        range.hands.insert(Hand::from_str("2c3c").unwrap(), 1.0); // High Card (loses to AA)

        let equity = calculate_equity(&hero, &range, &board);

        // Should be 50% win (vs 2c3c), 50% lose (vs KhKd)
        assert!((equity.win - 0.5).abs() < 0.001);
        assert!((equity.lose - 0.5).abs() < 0.001);
    }
}
