use fpe::models::game_state::{GameState, Position, Street};
use fpe::models::hand::Hand;
use fpe::models::range::Range;
use std::str::FromStr;

#[test]
fn test_game_state_validation_external() {
    let hand = Hand::from_str("AhKd").unwrap();
    let board = vec![];

    // Valid game state
    let state = GameState::new(
        hand.clone(),
        board,
        10.0,
        100.0,
        0.0,
        Position::IP,
        Range::new(),
    );
    assert!(state.is_ok());
    let state = state.unwrap();
    assert_eq!(state.street, Street::Preflop);
}

#[test]
fn test_game_state_invalid_pot() {
    let hand = Hand::from_str("AhKd").unwrap();
    let result = GameState::new(hand, vec![], 0.0, 100.0, 0.0, Position::IP, Range::new());
    assert!(result.is_err());
}
