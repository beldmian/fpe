//! Card representation and parsing

use crate::error::{ModelError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Card rank (2-A)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Rank {
    /// Two
    Two = 2,
    /// Three
    Three = 3,
    /// Four
    Four = 4,
    /// Five
    Five = 5,
    /// Six
    Six = 6,
    /// Seven
    Seven = 7,
    /// Eight
    Eight = 8,
    /// Nine
    Nine = 9,
    /// Ten
    Ten = 10,
    /// Jack
    Jack = 11,
    /// Queen
    Queen = 12,
    /// King
    King = 13,
    /// Ace
    Ace = 14,
}

impl From<Rank> for char {
    fn from(rank: Rank) -> char {
        match rank {
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
            Rank::Nine => '9',
            Rank::Ten => 'T',
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
            Rank::Ace => 'A',
        }
    }
}

impl FromStr for Rank {
    type Err = ModelError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            "9" => Ok(Rank::Nine),
            "T" | "t" => Ok(Rank::Ten),
            "J" | "j" => Ok(Rank::Jack),
            "Q" | "q" => Ok(Rank::Queen),
            "K" | "k" => Ok(Rank::King),
            "A" | "a" => Ok(Rank::Ace),
            _ => Err(ModelError::InvalidCard(s.to_string())),
        }
    }
}

/// Card suit
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Suit {
    /// Hearts (♥)
    Hearts,
    /// Diamonds (♦)
    Diamonds,
    /// Clubs (♣)
    Clubs,
    /// Spades (♠)
    Spades,
}

impl From<Suit> for char {
    fn from(suit: Suit) -> char {
        match suit {
            Suit::Hearts => 'h',
            Suit::Diamonds => 'd',
            Suit::Clubs => 'c',
            Suit::Spades => 's',
        }
    }
}

impl FromStr for Suit {
    type Err = ModelError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "h" | "H" => Ok(Suit::Hearts),
            "d" | "D" => Ok(Suit::Diamonds),
            "c" | "C" => Ok(Suit::Clubs),
            "s" | "S" => Ok(Suit::Spades),
            _ => Err(ModelError::InvalidCard(s.to_string())),
        }
    }
}

/// A single playing card with rank and suit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Card {
    /// Card rank
    pub rank: Rank,
    /// Card suit
    pub suit: Suit,
}

impl Card {
    /// Create a new card
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }
}

impl FromStr for Card {
    type Err = ModelError;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 2 {
            return Err(ModelError::InvalidCard(s.to_string()));
        }

        let rank_str = &s[0..1];
        let suit_str = &s[1..2];

        let rank = Rank::from_str(rank_str)?;
        let suit = Suit::from_str(suit_str)?;

        Ok(Card::new(rank, suit))
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank_char = char::from(self.rank);
        let suit_char = match self.suit {
            Suit::Hearts => '♥',
            Suit::Diamonds => '♦',
            Suit::Clubs => '♣',
            Suit::Spades => '♠',
        };
        write!(f, "{}{}", rank_char, suit_char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_parsing() {
        assert!(Card::from_str("Ah").is_ok());
        assert!(Card::from_str("Kd").is_ok());
        assert!(Card::from_str("Ts").is_ok());
        assert!(Card::from_str("2c").is_ok());
        assert!(Card::from_str("Xh").is_err());
        assert!(Card::from_str("Ax").is_err());
    }
}
