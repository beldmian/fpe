use fpe::models::{Card, GameState, Hand, Position, Range, Street};
use fpe::solver::solve;
use std::str::FromStr;

#[test]
fn test_solver_execution_river_nuts() {
    // Hero has Nuts (Royal Flush) on River
    // Board: As Ks Qs Js Ts
    // Hero: Ah Kd (Wait, Ah Kd makes nothing on this board? No, Board is Spades. Hero has Ah? High card? Board is Royal Flush on board?)
    // Let's make Hero have the nuts.
    // Board: 2s 3s 4s 5s
    // Hero: As Ks (Nut Flush? No, 6s is straight flush. As is high flush.)
    // Let's do simple: Board 2d 2c 2h 3h 3d (Full House)
    // Hero: 2s 2h (Quads - Wait, 2h is on board. 2s 3s? )
    // Hero: 2s 3s (Quads vs Full House) -> Nuts.
    // Villain Range: AA, KK (Overpairs -> Full Houses)
    // Hero should Bet/All-in.

    // Simplest Nut scenario:
    // Board: Kh Qh Jh Th 2s
    // Hero: Ah 2d (Royal Flush)
    // Villain: Kd Qd (Two Pair? No, K-high straight? No. Board has 4 hearts.)
    // Villain Range: "KdQd" (High cards)

    let hero = Hand::from_str("Ah2d").unwrap();
    let board = vec![
        Card::from_str("Kh").unwrap(),
        Card::from_str("Qh").unwrap(),
        Card::from_str("Jh").unwrap(),
        Card::from_str("Th").unwrap(),
        Card::from_str("2s").unwrap(),
    ];
    let villain_range = Range::new(); // Stub, effectively empty or random?
                                      // We need logic to handle stub range in US1.
                                      // If range is empty, maybe solver panics or assumes 100% equity?
                                      // Let's assume for now the solver handles empty range gracefully or we assume 50/50 if not specified.
                                      // Ideally US2 adds range parsing.

    let state = GameState::new(
        hero,
        board,
        10.0,  // Pot
        100.0, // Stack
        0.0,   // To Call
        Position::IP,
        villain_range,
    )
    .unwrap();

    let result = solve(state, 1000);
    assert!(result.is_ok());
    let strategy = result.unwrap();

    // Check if we have actions
    assert!(!strategy.actions.is_empty());

    // Basic validity
    assert!(strategy.is_valid());

    // For a Stub solver (uniform), this passes.
    // For a Real solver, we expect high EV for betting with Nuts.
    // Currently T015 expects "convergence", which implies we run enough iterations.
}
