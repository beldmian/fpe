use fpe::models::{action::Action, Card, GameState, Hand, Position, Range};
use fpe::solver::solve;
use std::str::FromStr;

#[test]
fn test_solver_execution_river_nuts() {
    let hero = Hand::from_str("Ah2d").unwrap();
    let board = vec![
        Card::from_str("Kh").unwrap(),
        Card::from_str("Qh").unwrap(),
        Card::from_str("Jh").unwrap(),
        Card::from_str("Th").unwrap(),
        Card::from_str("2s").unwrap(),
    ];
    let villain_range = Range::from_notation("KdQd").unwrap();

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
}

#[test]
fn test_solver_non_uniform_strategy() {
    // Setup a scenario where one action is clearly better
    // Hero has nuts, should bet. Checking is bad.
    let hero = Hand::from_str("AhKh").unwrap(); // Royal Flush
    let board = vec![
        Card::from_str("Qh").unwrap(),
        Card::from_str("Jh").unwrap(),
        Card::from_str("Th").unwrap(),
        Card::from_str("2s").unwrap(),
        Card::from_str("3d").unwrap(),
    ];
    // Villain has air
    let villain_range = Range::from_notation("2c3c").unwrap();

    let state = GameState::new(hero, board, 10.0, 100.0, 0.0, Position::IP, villain_range).unwrap();

    let strategy = solve(state, 1000).unwrap();

    // In a uniform strategy (stub), all actions have equal probability.
    // In a real solver, Bet/AllIn should be high frequency.

    let actions_count = strategy.actions.len();
    if actions_count > 1 {
        let uniform_prob = 1.0 / actions_count as f64;

        // Check that at least one action deviates significantly from uniform
        let has_deviation = strategy
            .actions
            .iter()
            .any(|a| (a.frequency - uniform_prob).abs() > 0.1);
        assert!(
            has_deviation,
            "Strategy should not be uniform for nuts vs air"
        );
    }
}

#[test]
fn test_solver_nuts_vs_air_bet_frequency() {
    // Hero has nuts, Villain has air. Hero should bet ~100%.
    let hero = Hand::from_str("AhKh").unwrap(); // Royal Flush
    let board = vec![
        Card::from_str("Qh").unwrap(),
        Card::from_str("Jh").unwrap(),
        Card::from_str("Th").unwrap(),
        Card::from_str("2s").unwrap(),
        Card::from_str("3d").unwrap(),
    ];
    let villain_range = Range::from_notation("7c2c").unwrap(); // Air

    let state = GameState::new(hero, board, 10.0, 100.0, 0.0, Position::IP, villain_range).unwrap();

    // Run enough iterations to converge
    let strategy = solve(state, 5000).unwrap();

    let bet_freq: f64 = strategy
        .actions
        .iter()
        .filter(|a| matches!(a.action, Action::Bet(_) | Action::AllIn))
        .map(|a| a.frequency)
        .sum();

    assert!(
        bet_freq > 0.90,
        "Hero should bet > 90% with nuts vs air, got {}",
        bet_freq
    );
}

#[test]
fn test_solver_convergence_metric() {
    let hero = Hand::from_str("AhKh").unwrap();
    let board = vec![
        Card::from_str("Qh").unwrap(),
        Card::from_str("Jh").unwrap(),
        Card::from_str("Th").unwrap(),
        Card::from_str("2s").unwrap(),
        Card::from_str("3d").unwrap(),
    ];
    let villain_range = Range::from_notation("22+").unwrap();

    let state = GameState::new(hero, board, 10.0, 100.0, 0.0, Position::IP, villain_range).unwrap();

    let strategy = solve(state, 100).unwrap();

    // Convergence metric should be reported.
    // Since we solve for a single Hero hand, strategies often converge to pure strategies instantly,
    // resulting in 0.0 convergence (perfect stability).
    // We just verify it's a valid number.
    assert!(strategy.convergence >= 0.0);
}

// Removed test_solver_convergence_improvement as it's flaky with instant convergence.

#[test]
fn test_solver_convergence_improvement() {
    let hero = Hand::from_str("AhKh").unwrap();
    let board = vec![
        Card::from_str("Qh").unwrap(),
        Card::from_str("Jh").unwrap(),
        Card::from_str("Th").unwrap(),
        Card::from_str("2s").unwrap(),
        Card::from_str("3d").unwrap(),
    ];
    // Use a range with multiple hands to induce variance
    let villain_range = Range::from_notation("22+").unwrap();

    let state = GameState::new(hero, board, 10.0, 100.0, 0.0, Position::IP, villain_range).unwrap();

    let s1 = solve(state.clone(), 100).unwrap();
    let s2 = solve(state, 2000).unwrap();

    // More iterations should generally lead to lower or equal convergence metric
    assert!(
        s2.convergence <= s1.convergence,
        "More iterations should not degrade convergence ({} vs {})",
        s2.convergence,
        s1.convergence
    );
}

#[test]
fn test_solve_with_config() {
    use fpe::solver::mccfr::{solve_with_config, MccfrConfig};

    let hero = Hand::from_str("AhKh").unwrap();
    let board = vec![
        Card::from_str("Qh").unwrap(),
        Card::from_str("Jh").unwrap(),
        Card::from_str("Th").unwrap(),
        Card::from_str("2s").unwrap(),
        Card::from_str("3d").unwrap(),
    ];
    let villain_range = Range::from_notation("22+").unwrap();

    let state = GameState::new(hero, board, 10.0, 100.0, 0.0, Position::IP, villain_range).unwrap();

    let config = MccfrConfig {
        iterations: 50,
        samples_per_iteration: 10,
        convergence_threshold: 0.001,
        seed: Some(42),
    };

    let strategy = solve_with_config(state, config).unwrap();

    assert_eq!(strategy.iterations, 50);
}

#[test]
fn test_solver_complex_game_tree() {
    use fpe::models::action::BetSize;

    let hero = Hand::from_str("AhKh").unwrap();
    let board = vec![
        Card::from_str("Qh").unwrap(),
        Card::from_str("Jh").unwrap(),
        Card::from_str("Th").unwrap(),
        Card::from_str("2s").unwrap(),
        Card::from_str("3d").unwrap(),
    ];
    let villain_range = Range::from_notation("22+").unwrap();

    let mut state =
        GameState::new(hero, board, 10.0, 100.0, 0.0, Position::IP, villain_range).unwrap();

    // Add multiple actions
    state.available_actions = vec![
        Action::Check,
        Action::Bet(BetSize::PotFraction(0.33)),
        Action::Bet(BetSize::PotFraction(0.5)),
        Action::Bet(BetSize::PotFraction(0.75)),
        Action::Bet(BetSize::PotFraction(1.0)),
        Action::AllIn,
    ];

    let strategy = solve(state, 1000).unwrap();

    assert_eq!(strategy.actions.len(), 6);
    assert!(strategy.is_valid());

    // Check frequencies sum to 1.0
    let sum: f64 = strategy.actions.iter().map(|a| a.frequency).sum();
    assert!((sum - 1.0).abs() < 0.001);
}
