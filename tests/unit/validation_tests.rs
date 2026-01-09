use fpe::cli::validation;
use fpe::models::{Card, GameState, Hand, Position, Range};
use std::str::FromStr;

#[test]
fn test_invalid_card_notation() {
    assert!(validation::validate_card("Ah").is_ok());
    assert!(validation::validate_card("Xh").is_err());
    assert!(validation::validate_card("Ax").is_err());
    assert!(validation::validate_card("A").is_err());
    assert!(validation::validate_card("AhK").is_err());
}

#[test]
fn test_duplicate_card_detection() {
    let hero = Hand::from_str("AhKd").unwrap();
    let board = vec![
        Card::from_str("Ah").unwrap(), // Duplicate Ah
        Card::from_str("Ts").unwrap(),
        Card::from_str("2c").unwrap(),
    ];

    let result = validation::check_duplicates(&hero, &board);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Duplicate card"), "Msg: {}", err_msg);
    assert!(
        err_msg.contains("A♥") || err_msg.contains("Ah"),
        "Msg: {}",
        err_msg
    );
}

#[test]
fn test_duplicate_board_cards() {
    let hero = Hand::from_str("AhKd").unwrap();
    let board = vec![
        Card::from_str("Ts").unwrap(),
        Card::from_str("Ts").unwrap(), // Duplicate Ts
        Card::from_str("2c").unwrap(),
    ];

    let result = validation::check_duplicates(&hero, &board);
    assert!(result.is_err());
}

#[test]
fn test_invalid_game_state_validation() {
    let h = Hand::from_str("AhKd").unwrap();
    let b = vec![];
    let r = Range::new();

    assert!(GameState::new(
        h.clone(),
        b.clone(),
        -1.0,
        100.0,
        0.0,
        Position::IP,
        r.clone()
    )
    .is_err());
    assert!(GameState::new(
        h.clone(),
        b.clone(),
        10.0,
        0.0,
        0.0,
        Position::IP,
        r.clone()
    )
    .is_err());
    assert!(GameState::new(
        h.clone(),
        b.clone(),
        10.0,
        100.0,
        200.0,
        Position::IP,
        r.clone()
    )
    .is_err());
}

#[test]
fn test_board_street_mismatch() {
    let h = Hand::from_str("AhKd").unwrap();
    let r = Range::new();

    // 2 cards -> Invalid street
    let board_2 = vec![Card::from_str("Ah").unwrap(), Card::from_str("Kd").unwrap()];
    assert!(GameState::new(
        h.clone(),
        board_2,
        10.0,
        100.0,
        0.0,
        Position::IP,
        r.clone()
    )
    .is_err());
}
