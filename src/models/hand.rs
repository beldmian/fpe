//! Hand representation (2-card poker hands)

use crate::error::{ModelError, Result};
use crate::models::card::Card;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// A player's hole cards (exactly 2 cards in Texas Hold'em)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hand {
    /// The two cards in the hand
    pub cards: [Card; 2],
}

impl Hand {
    /// Create a new hand from two cards
    pub fn new(card1: Card, card2: Card) -> Self {
        Self {
            cards: [card1, card2],
        }
    }

    /// Returns true if both cards have the same suit
    pub fn is_suited(&self) -> bool {
        self.cards[0].suit == self.cards[1].suit
    }

    /// Returns true if both cards have the same rank
    pub fn is_pair(&self) -> bool {
        self.cards[0].rank == self.cards[1].rank
    }

    /// Returns the hand in canonical notation (e.g., "AKs", "QQ", "T9o")
    pub fn notation(&self) -> String {
        let mut ranks = [self.cards[0].rank, self.cards[1].rank];
        ranks.sort_by(|a, b| b.cmp(a)); // Sort descending

        let rank1_char = char::from(ranks[0]);
        let rank2_char = char::from(ranks[1]);

        if self.is_pair() {
            format!("{}{}", rank1_char, rank2_char)
        } else if self.is_suited() {
            format!("{}{}s", rank1_char, rank2_char)
        } else {
            format!("{}{}o", rank1_char, rank2_char)
        }
    }
}

impl FromStr for Hand {
    type Err = ModelError;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 4 {
            return Err(ModelError::InvalidCard(format!(
                "Hand must be 4 characters (e.g., 'AhKd'), got '{}'",
                s
            )));
        }

        let card1 = Card::from_str(&s[0..2])?;
        let card2 = Card::from_str(&s[2..4])?;

        if card1 == card2 {
            return Err(ModelError::DuplicateCard(s.to_string()));
        }

        Ok(Hand::new(card1, card2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_parsing() {
        let hand = Hand::from_str("AhKd").unwrap();
        assert_eq!(hand.cards.len(), 2);
        assert!(!hand.is_suited());
        assert!(!hand.is_pair());
    }

    #[test]
    fn test_suited() {
        let hand = Hand::from_str("AhKh").unwrap();
        assert!(hand.is_suited());
    }

    #[test]
    fn test_pair() {
        let hand = Hand::from_str("AhAd").unwrap();
        assert!(hand.is_pair());
    }

    #[test]
    fn test_notation() {
        assert_eq!(Hand::from_str("AhKd").unwrap().notation(), "AKo");
        assert_eq!(Hand::from_str("AhKh").unwrap().notation(), "AKs");
        assert_eq!(Hand::from_str("AhAd").unwrap().notation(), "AA");
    }
}
