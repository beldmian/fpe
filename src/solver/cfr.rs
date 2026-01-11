//! MCCFR Solver implementation

use crate::error::Result;
use crate::models::action::{Action, BetSize};
use crate::models::game_state::GameState;
use crate::models::strategy::Strategy;

use crate::solver::mccfr::solve_mccfr;

/// GTO Solver engine
pub struct Solver {
    game_state: GameState,
    iterations: u32,
}

impl Solver {
    /// Create a new solver instance
    pub fn new(game_state: GameState, iterations: u32) -> Self {
        Self {
            game_state,
            iterations,
        }
    }

    /// Execute the solver and return the strategy
    pub fn solve(&self) -> Result<Strategy> {
        // 1. Determine available actions (if not already set in game_state)
        let mut state = self.game_state.clone();
        if state.available_actions.is_empty() {
            state.available_actions = determine_available_actions(&state);
        }

        // 2. Run MCCFR
        let strategy = solve_mccfr(&state, self.iterations);

        Ok(strategy)
    }
}

/// Determine valid actions for the current game state
pub fn determine_available_actions(state: &GameState) -> Vec<Action> {
    let mut actions = Vec::new();

    // Basic logic
    if state.to_call > 0.0 {
        actions.push(Action::Fold);
        actions.push(Action::Call);

        // Raises
        if state.effective_stack > state.to_call {
            actions.push(Action::Raise(BetSize::PotFraction(1.0)));
            actions.push(Action::AllIn);
        }
    } else {
        actions.push(Action::Check);

        // Bets
        if state.effective_stack > 0.0 {
            actions.push(Action::Bet(BetSize::PotFraction(0.5)));
            actions.push(Action::Bet(BetSize::PotFraction(1.0)));
            actions.push(Action::AllIn);
        }
    }

    actions
}

/// Helper to run solver in one step
pub fn solve(game_state: GameState, iterations: u32) -> Result<Strategy> {
    let solver = Solver::new(game_state, iterations);
    solver.solve()
}
