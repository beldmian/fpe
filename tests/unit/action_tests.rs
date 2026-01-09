use fpe::models::action::{Action, BetSize};

#[test]
fn test_action_display_names() {
    assert_eq!(Action::Fold.display_name(), "Fold");
    assert_eq!(Action::Check.display_name(), "Check");
    assert_eq!(Action::Call.display_name(), "Call");
    assert_eq!(Action::AllIn.display_name(), "All-In");
}

#[test]
fn test_bet_size_display() {
    let bet = Action::Bet(BetSize::PotFraction(0.75));
    assert_eq!(bet.display_name(), "Bet 75% pot");

    let raise = Action::Raise(BetSize::Amount(25.0));
    assert_eq!(raise.display_name(), "Raise to 25 BB");
}
