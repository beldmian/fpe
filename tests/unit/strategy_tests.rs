use fpe::models::action::Action;
use fpe::models::strategy::{ActionStrategy, Strategy};

#[test]
fn test_strategy_frequency_sum() {
    let actions = vec![
        ActionStrategy {
            action: Action::Fold,
            frequency: 0.25,
            ev: 0.0,
        },
        ActionStrategy {
            action: Action::Call,
            frequency: 0.75,
            ev: 1.5,
        },
    ];

    let strategy = Strategy::new(actions, 1000, 0.001);
    assert!(strategy.is_valid());
}

#[test]
fn test_strategy_invalid_sum() {
    let actions = vec![
        ActionStrategy {
            action: Action::Fold,
            frequency: 0.25,
            ev: 0.0,
        },
        ActionStrategy {
            action: Action::Call,
            frequency: 0.80, // Sums to 1.05
            ev: 1.5,
        },
    ];

    let strategy = Strategy::new(actions, 1000, 0.001);
    assert!(!strategy.is_valid());
}
