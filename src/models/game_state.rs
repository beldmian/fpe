//! Game state representation

use crate::error::{ModelError, Result};
use crate::models::action::Action;
use crate::models::card::Card;
use crate::models::hand::Hand;
use crate::models::range::Range;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Hero's position relative to opponent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Position {
    /// In position (acts last)
    IP,
    /// Out of position (acts first)
    OOP,
}

impl FromStr for Position {
    type Err = ModelError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "IP" => Ok(Position::IP),
            "OOP" => Ok(Position::OOP),
            _ => Err(ModelError::InvalidGameState(format!(
                "Invalid position '{}', expected 'IP' or 'OOP'",
                s
            ))),
        }
    }
}

/// Current street in the hand
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Street {
    /// Preflop (no community cards)
    Preflop,
    /// Flop (3 community cards)
    Flop,
    /// Turn (4 community cards)
    Turn,
    /// River (5 community cards)
    River,
}

impl Street {
    /// Returns the expected number of community cards for this street
    pub fn expected_cards(&self) -> usize {
        match self {
            Street::Preflop => 0,
            Street::Flop => 3,
            Street::Turn => 4,
            Street::River => 5,
        }
    }

    /// Determines the street based on the number of board cards
    pub fn from_board_size(size: usize) -> Result<Self> {
        match size {
            0 => Ok(Street::Preflop),
            3 => Ok(Street::Flop),
            4 => Ok(Street::Turn),
            5 => Ok(Street::River),
            _ => Err(ModelError::InvalidBoard {
                street: "Unknown".to_string(),
                expected: 0,
                actual: size,
            }),
        }
    }
}

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

impl GameState {
    /// Create a new game state with validation
    pub fn new(
        hero_hand: Hand,
        board: Vec<Card>,
        pot_size: f64,
        effective_stack: f64,
        to_call: f64,
        position: Position,
        villain_range: Range,
    ) -> Result<Self> {
        // Validate pot size
        if pot_size <= 0.0 {
            return Err(ModelError::InvalidGameState(
                "Pot size must be greater than 0".to_string(),
            ));
        }

        // Validate effective stack
        if effective_stack <= 0.0 {
            return Err(ModelError::InvalidGameState(
                "Effective stack must be greater than 0".to_string(),
            ));
        }

        // Validate to_call
        if to_call < 0.0 || to_call > effective_stack {
            return Err(ModelError::InvalidGameState(format!(
                "to_call ({}) must be between 0 and effective_stack ({})",
                to_call, effective_stack
            )));
        }

        // Determine street from board size
        let street = Street::from_board_size(board.len())?;

        // Validate board size matches street
        if board.len() != street.expected_cards() {
            return Err(ModelError::InvalidBoard {
                street: format!("{:?}", street),
                expected: street.expected_cards(),
                actual: board.len(),
            });
        }

        // Check for duplicate cards
        let mut all_cards = vec![hero_hand.cards[0], hero_hand.cards[1]];
        all_cards.extend(&board);
        for i in 0..all_cards.len() {
            for j in (i + 1)..all_cards.len() {
                if all_cards[i] == all_cards[j] {
                    return Err(ModelError::DuplicateCard(format!("{}", all_cards[i])));
                }
            }
        }

        Ok(Self {
            hero_hand,
            board,
            pot_size,
            effective_stack,
            to_call,
            position,
            villain_range,
            street,
            available_actions: Vec::new(), // Will be populated by solver
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::range::Range;
    use std::str::FromStr;

    #[test]
    fn test_street_from_board_size() {
        assert_eq!(Street::from_board_size(0).unwrap(), Street::Preflop);
        assert_eq!(Street::from_board_size(3).unwrap(), Street::Flop);
        assert_eq!(Street::from_board_size(4).unwrap(), Street::Turn);
        assert_eq!(Street::from_board_size(5).unwrap(), Street::River);
        assert!(Street::from_board_size(2).is_err());
    }

    #[test]
    fn test_position_parsing() {
        assert_eq!(Position::from_str("IP").unwrap(), Position::IP);
        assert_eq!(Position::from_str("ip").unwrap(), Position::IP);
        assert_eq!(Position::from_str("OOP").unwrap(), Position::OOP);
        assert!(Position::from_str("invalid").is_err());
    }

    #[test]
    fn test_game_state_validation() {
        let hand = Hand::from_str("AhKd").unwrap();
        let board = vec![];

        // Valid game state
        let state = GameState::new(
            hand.clone(),
            board.clone(),
            10.0,
            100.0,
            0.0,
            Position::IP,
            Range::new(),
        );
        assert!(state.is_ok());

        // Invalid pot size
        let state = GameState::new(
            hand.clone(),
            vec![],
            -1.0,
            100.0,
            0.0,
            Position::IP,
            Range::new(),
        );
        assert!(state.is_err());

        // Invalid stack size
        let state = GameState::new(
            hand.clone(),
            vec![],
            10.0,
            0.0,
            0.0,
            Position::IP,
            Range::new(),
        );
        assert!(state.is_err());
    }
}
