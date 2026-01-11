use fpe::models::{
    game_state::{GameState, Position},
    hand::Hand,
    range::Range,
};
use fpe::solver::info_set::{InfoSetKey, SprBucket};
use std::str::FromStr;

#[test]
fn test_spr_bucket_from_spr() {
    assert_eq!(SprBucket::from_spr(1.0), SprBucket::Short);
    assert_eq!(SprBucket::from_spr(3.0), SprBucket::Medium);
    assert_eq!(SprBucket::from_spr(7.0), SprBucket::Deep);
    assert_eq!(SprBucket::from_spr(15.0), SprBucket::VeryDeep);
}

#[test]
fn test_info_set_key_from_game_state() {
    let hand = Hand::from_str("AhKd").unwrap();
    let state = GameState::new(
        hand.clone(),
        vec![],
        10.0,
        100.0,
        0.0,
        Position::IP,
        Range::new(),
    )
    .unwrap();

    let key = InfoSetKey::from_game_state(&state);

    assert_eq!(key.hero_hand, hand);
    assert_eq!(key.position, Position::IP);
    // SPR = 100/10 = 10 -> VeryDeep
    assert_eq!(key.spr_bucket, SprBucket::VeryDeep);
}
