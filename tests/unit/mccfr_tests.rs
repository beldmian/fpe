use fpe::solver::regret::regret_to_strategy;

#[test]
fn test_regret_matching_uniform() {
    let regrets = vec![0.0, 0.0, 0.0];
    let strategy = regret_to_strategy(&regrets);
    assert!((strategy[0] - 0.333).abs() < 0.01);
    assert!((strategy[1] - 0.333).abs() < 0.01);
    assert!((strategy[2] - 0.333).abs() < 0.01);
}

#[test]
fn test_regret_matching_positive() {
    let regrets = vec![1.0, 3.0, 0.0];
    let strategy = regret_to_strategy(&regrets);
    assert!((strategy[0] - 0.25).abs() < 0.01);
    assert!((strategy[1] - 0.75).abs() < 0.01);
    assert!((strategy[2] - 0.0).abs() < 0.01);
}

#[test]
fn test_regret_matching_negative() {
    let regrets = vec![10.0, -5.0, 10.0];
    let strategy = regret_to_strategy(&regrets);
    assert!((strategy[0] - 0.5).abs() < 0.01);
    assert!((strategy[1] - 0.0).abs() < 0.01);
    assert!((strategy[2] - 0.5).abs() < 0.01);
}

#[test]
fn test_convergence_tracker_check() {
    use fpe::models::{game_state::Position, hand::Hand};
    use fpe::solver::info_set::{InfoSetKey, SprBucket};
    use fpe::solver::mccfr::ConvergenceTracker;
    use fpe::solver::regret::RegretTable;
    use std::str::FromStr;

    let mut tracker = ConvergenceTracker::new();
    let mut table = RegretTable::new();

    // Create a dummy key
    let key = InfoSetKey {
        hero_hand: Hand::from_str("AhAs").unwrap(),
        spr_bucket: SprBucket::Medium,
        position: Position::IP,
    };

    // Update table with some regrets
    table.update_regrets(key.clone(), &[10.0, 10.0], 1.0);

    // First check: should establish baseline (change = 0.0 or 1.0? Usually 1.0 if no prev)
    // If we define max_change as difference from previous.
    // First iteration: no previous. So change is 0? Or we skip?
    let _change1 = tracker.check_convergence(&table);

    // Update again with different regrets (strategy changes)
    table.update_regrets(key.clone(), &[20.0, 0.0], 1.0);

    let change2 = tracker.check_convergence(&table);

    assert!(change2 > 0.0, "Strategy should have changed");
}

#[test]
fn test_convergence_tracker_is_converged() {
    use fpe::solver::mccfr::ConvergenceTracker;
    use fpe::solver::regret::RegretTable;

    let mut tracker = ConvergenceTracker::new();
    // Initially not converged (max_change = MAX)
    assert!(!tracker.is_converged(1.0));

    // Simulate a check with 0 change
    let table = RegretTable::new();
    tracker.check_convergence(&table); // Returns 0.0 (empty table)

    assert!(tracker.is_converged(0.001));
}

#[test]
fn test_mccfr_config_default() {
    use fpe::solver::mccfr::MccfrConfig;

    let config = MccfrConfig::default();
    assert_eq!(config.iterations, 10_000);
    assert_eq!(config.samples_per_iteration, 100); // Assuming 100 default
    assert_eq!(config.convergence_threshold, 0.001);
    assert!(config.seed.is_none());
}

#[test]
fn test_mccfr_config_custom() {
    use fpe::solver::mccfr::MccfrConfig;

    let config = MccfrConfig {
        iterations: 500,
        samples_per_iteration: 50,
        convergence_threshold: 0.01,
        seed: Some(12345),
    };

    assert_eq!(config.iterations, 500);
    assert_eq!(config.seed, Some(12345));
}

// US2 Tests
// Note: ConvergenceTracker is not yet implemented, so we can't import it yet.
// But we can write the test structure and comment it out or expect failure if we could import it.
// Since I can't import non-existent struct, I will implement the struct skeleton first?
// No, TDD says write test first. But in Rust, if struct doesn't exist, it's a compile error, not a test failure.
// So I should define the struct skeleton first?
// Or I can write the test and expect compilation error (which counts as failure).
// But I need to be able to run other tests.
// I will create the test file content but maybe comment out the imports until I create the file?
// No, I'll just create the test and let it fail compilation.
// Wait, if it fails compilation, I can't run the previous tests to ensure no regression.
// Standard practice in Rust TDD:
// 1. Write test.
// 2. Run test -> Compile Error.
// 3. Write minimal code to compile.
// 4. Run test -> Fail assertion.
// 5. Write code to pass.

// So I will write the test.
/*
use fpe::solver::mccfr::ConvergenceTracker;
use fpe::solver::regret::RegretTable;

#[test]
fn test_convergence_tracker_check() {
    let mut tracker = ConvergenceTracker::new();
    let mut table = RegretTable::new();
    // ... setup table ...
    let change = tracker.check_convergence(&table);
    assert!(change >= 0.0);
}
*/
// I'll add these tests after I create the struct skeleton in T038.
// But the plan says "Tests FIRST".
// I will add them now, and accept that `cargo test` will fail to compile.
