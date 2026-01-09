//! MCCFR Solver implementation

use crate::error::Result;
use crate::models::action::{Action, BetSize};
use crate::models::game_state::GameState;
use crate::models::strategy::{ActionStrategy, Strategy};

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
        let actions = if self.game_state.available_actions.is_empty() {
            determine_available_actions(&self.game_state)
        } else {
            self.game_state.available_actions.clone()
        };

        // 2. Initialize strategy (uniform stub)
        let n_actions = actions.len();
        let uniform = if n_actions > 0 {
            1.0 / n_actions as f64
        } else {
            0.0
        };

        let action_strategies: Vec<ActionStrategy> = actions
            .into_iter()
            .map(|a| ActionStrategy {
                action: a,
                frequency: uniform,
                ev: 0.0,
            })
            .collect();

        Ok(Strategy::new(action_strategies, self.iterations, 0.0))
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
