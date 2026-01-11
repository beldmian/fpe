//! Monte Carlo Counterfactual Regret Minimization (MCCFR) algorithm.
//!
//! This module implements the core MCCFR algorithm using External Sampling.

use crate::models::{
    action::{Action, BetSize},
    game_state::{GameState, Position},
    hand::Hand,
    range::Range,
    strategy::{ActionStrategy, Strategy},
};
use crate::solver::{evaluator::evaluate_hand, info_set::InfoSetKey, regret::RegretTable};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use rustc_hash::FxHashMap;

/// Configuration for solver execution.
#[derive(Debug, Clone)]
pub struct MccfrConfig {
    /// Number of MCCFR iterations
    pub iterations: u32,
    /// Villain hands sampled per iteration
    pub samples_per_iteration: usize,
    /// Strategy change threshold for early stop
    pub convergence_threshold: f64,
    /// RNG seed for reproducibility
    pub seed: Option<u64>,
}

impl Default for MccfrConfig {
    fn default() -> Self {
        Self {
            iterations: 10_000,
            samples_per_iteration: 100,
            convergence_threshold: 0.001,
            seed: None,
        }
    }
}

/// Tracks convergence metrics during training.
pub struct ConvergenceTracker {
    /// Previous iteration strategies
    prev_strategies: FxHashMap<InfoSetKey, Vec<f64>>,
    /// Maximum strategy change this check
    max_change: f64,
    /// Iterations since last check
    iterations_since_check: u32,
}

impl Default for ConvergenceTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl ConvergenceTracker {
    /// Create a new convergence tracker.
    pub fn new() -> Self {
        Self {
            prev_strategies: FxHashMap::default(),
            max_change: f64::MAX, // Start high (not converged)
            iterations_since_check: 0,
        }
    }

    /// Check convergence by comparing current average strategy to previous check.
    /// Returns the maximum strategy change across all info sets.
    pub fn check_convergence(&mut self, regret_table: &RegretTable) -> f64 {
        let mut max_diff = 0.0;

        for key in regret_table.keys() {
            if let Some(current_strategy) = regret_table.get_average_strategy(key) {
                if let Some(prev_strategy) = self.prev_strategies.get(key) {
                    // Calculate diff
                    for (s1, s2) in current_strategy.iter().zip(prev_strategy.iter()) {
                        let diff = (s1 - s2).abs();
                        if diff > max_diff {
                            max_diff = diff;
                        }
                    }
                }
                // Update previous strategy
                self.prev_strategies.insert(key.clone(), current_strategy);
            }
        }

        self.max_change = max_diff;
        self.iterations_since_check = 0;
        max_diff
    }

    /// Check if convergence is below the threshold.
    pub fn is_converged(&self, threshold: f64) -> bool {
        self.max_change < threshold
    }
}

/// Monte Carlo sampler for External Sampling MCCFR.
pub struct McSampler {
    rng: Xoshiro256PlusPlus,
}

impl McSampler {
    /// Create a new sampler with optional seed.
    pub fn new(seed: Option<u64>) -> Self {
        let rng = if let Some(s) = seed {
            Xoshiro256PlusPlus::seed_from_u64(s)
        } else {
            Xoshiro256PlusPlus::from_entropy()
        };
        Self { rng }
    }

    /// Sample a single hand from the range based on weights.
    pub fn sample_hand(&mut self, range: &Range) -> Option<(Hand, f64)> {
        if range.hands.is_empty() {
            return None;
        }

        let total_weight: f64 = range.hands.values().sum();
        let mut r = self.rng.gen::<f64>() * total_weight;

        for (hand, &weight) in &range.hands {
            r -= weight;
            if r <= 0.0 {
                return Some((hand.clone(), weight));
            }
        }

        // Fallback to any hand if rounding errors
        range.hands.iter().next().map(|(h, &w)| (h.clone(), w))
    }
}

/// Apply an action to a game state to get the next state.
/// Returns None if the action results in a terminal state (Fold, Showdown).
/// Returns (NewState, IsTerminal, PayoffForActor)
/// Payoff is only relevant if IsTerminal is true.
fn apply_action(state: &GameState, action: &Action) -> (Option<GameState>, bool, f64) {
    let mut next = state.clone();
    next.available_actions.clear(); // Clear actions for the next state

    match action {
        Action::Fold => (None, true, 0.0),
        Action::Check => {
            if state.position == Position::OOP {
                next.position = Position::IP;
                (Some(next), false, 0.0)
            } else {
                (None, true, 0.0)
            }
        }
        Action::Call => {
            next.pot_size += state.to_call;
            next.effective_stack -= state.to_call;
            next.to_call = 0.0;
            (None, true, 0.0)
        }
        Action::Bet(_) | Action::Raise(_) | Action::AllIn => {
            let amount = action.amount(state.pot_size, state.effective_stack, state.to_call);
            next.pot_size += amount;
            next.effective_stack -= amount;
            next.to_call = amount;
            next.position = if state.position == Position::IP {
                Position::OOP
            } else {
                Position::IP
            };
            (Some(next), false, 0.0)
        }
    }
}

/// Recursive MCCFR traversal.
/// Returns the utility for the *traverser*.
fn traverse(
    state: &GameState,
    traverser: Position,
    hero_hand: &Hand,
    villain_hand: &Hand,
    regret_table: &mut RegretTable,
    sampler: &mut McSampler,
) -> f64 {
    // Determine whose turn it is
    let actor = state.position;
    let is_traverser = actor == traverser;

    let actor_hand = if is_traverser {
        hero_hand
    } else {
        villain_hand
    };

    // Get available actions
    let actions = if state.available_actions.is_empty() {
        if state.to_call > 0.0 {
            vec![Action::Fold, Action::Call]
        } else {
            vec![Action::Check, Action::Bet(BetSize::PotFraction(0.5))]
        }
    } else {
        state.available_actions.clone()
    };

    if actions.is_empty() {
        return evaluate_showdown(state, hero_hand, villain_hand, traverser);
    }

    // Get Strategy
    let mut state_for_key = state.clone();
    state_for_key.hero_hand = actor_hand.clone();
    let key = InfoSetKey::from_game_state(&state_for_key);

    let strategy = regret_table.get_strategy(&key, actions.len());

    if is_traverser {
        // Traverser: Iterate all actions
        let mut node_util = 0.0;
        let mut action_utils = vec![0.0; actions.len()];

        for (i, action) in actions.iter().enumerate() {
            let (next_state_opt, is_terminal, payoff) = apply_action(state, action);

            let util = if is_terminal {
                if payoff != 0.0 {
                    payoff // Fold payoff
                } else {
                    evaluate_showdown(state, hero_hand, villain_hand, traverser)
                }
            } else if let Some(next) = next_state_opt {
                traverse(
                    &next,
                    traverser,
                    hero_hand,
                    villain_hand,
                    regret_table,
                    sampler,
                )
            } else {
                0.0
            };

            action_utils[i] = util;
            node_util += strategy[i] * util;
        }

        // Update Regrets
        let regrets: Vec<f64> = action_utils.iter().map(|u| u - node_util).collect();
        regret_table.update_regrets(key, &regrets, 1.0);

        node_util
    } else {
        // Opponent: Sample one action
        let mut r = sampler.rng.gen::<f64>();
        let mut chosen_idx = 0;
        for (i, &prob) in strategy.iter().enumerate() {
            r -= prob;
            if r <= 0.0 {
                chosen_idx = i;
                break;
            }
        }
        if chosen_idx >= actions.len() {
            chosen_idx = actions.len() - 1;
        }

        let action = &actions[chosen_idx];
        let (next_state_opt, is_terminal, payoff) = apply_action(state, action);

        if is_terminal {
            if payoff != 0.0 {
                if matches!(action, Action::Fold) {
                    state.pot_size // Traverser wins pot
                } else {
                    evaluate_showdown(state, hero_hand, villain_hand, traverser)
                }
            } else {
                evaluate_showdown(state, hero_hand, villain_hand, traverser)
            }
        } else if let Some(next) = next_state_opt {
            traverse(
                &next,
                traverser,
                hero_hand,
                villain_hand,
                regret_table,
                sampler,
            )
        } else {
            0.0
        }
    }
}

fn evaluate_showdown(
    state: &GameState,
    hero_hand: &Hand,
    villain_hand: &Hand,
    traverser: Position,
) -> f64 {
    let t_score = evaluate_hand(hero_hand, &state.board);
    let o_score = evaluate_hand(villain_hand, &state.board);

    let hero_is_traverser = if state.position == Position::IP {
        traverser == Position::IP
    } else {
        traverser == Position::OOP
    };

    if t_score > o_score {
        // Hero wins
        if hero_is_traverser {
            state.pot_size
        } else {
            0.0
        }
    } else if t_score < o_score {
        // Villain wins
        if hero_is_traverser {
            0.0
        } else {
            state.pot_size
        }
    } else {
        state.pot_size / 2.0
    }
}

/// Solve the game state using MCCFR with default configuration.
pub fn solve_mccfr(state: &GameState, iterations: u32) -> Strategy {
    let config = MccfrConfig {
        iterations,
        ..Default::default()
    };
    solve_with_config(state.clone(), config).unwrap() // Unwrap safe as we control inputs
}

/// Solve with custom configuration.
pub fn solve_with_config(state: GameState, config: MccfrConfig) -> Result<Strategy, String> {
    let mut regret_table = RegretTable::new();
    let mut sampler = McSampler::new(config.seed);
    let mut convergence_tracker = ConvergenceTracker::new();

    let mut root = state.clone();
    if root.available_actions.is_empty() {
        if root.to_call > 0.0 {
            root.available_actions = vec![Action::Fold, Action::Call];
        } else {
            root.available_actions = vec![Action::Check, Action::Bet(BetSize::PotFraction(0.5))];
        }
    }

    let check_interval = if config.iterations <= 100 {
        (config.iterations / 2).max(1)
    } else {
        (config.iterations / 10).max(100)
    };

    for i in 0..config.iterations {
        // Check convergence
        if i > 0 && i % check_interval == 0 {
            convergence_tracker.check_convergence(&regret_table);
            if convergence_tracker.is_converged(config.convergence_threshold) {
                // Early stop?
                // For now, we just track.
            }
        }

        let traverser = if i % 2 == 0 {
            state.position
        } else if state.position == Position::IP {
            Position::OOP
        } else {
            Position::IP
        };

        let is_hero_traverser = traverser == state.position;

        if is_hero_traverser {
            for _ in 0..config.samples_per_iteration {
                if let Some((villain_hand, _)) = sampler.sample_hand(&state.villain_range) {
                    traverse(
                        &root,
                        traverser,
                        &state.hero_hand,
                        &villain_hand,
                        &mut regret_table,
                        &mut sampler,
                    );
                }
            }
        } else {
            let hero_sample = state.hero_hand.clone();
            for _ in 0..config.samples_per_iteration {
                if let Some((villain_hand, _)) = sampler.sample_hand(&state.villain_range) {
                    traverse(
                        &root,
                        traverser,
                        &hero_sample,
                        &villain_hand,
                        &mut regret_table,
                        &mut sampler,
                    );
                }
            }
        }
    }

    // Final convergence check
    convergence_tracker.check_convergence(&regret_table);

    Ok(extract_strategy(
        &root,
        &regret_table,
        config.iterations,
        convergence_tracker.max_change,
    ))
}

fn extract_strategy(
    state: &GameState,
    regret_table: &RegretTable,
    iterations: u32,
    convergence: f64,
) -> Strategy {
    let key = InfoSetKey::from_game_state(state);
    let avg_strategy = regret_table.get_average_strategy(&key).unwrap_or_else(|| {
        let n = if state.available_actions.is_empty() {
            2
        } else {
            state.available_actions.len()
        };
        vec![1.0 / n as f64; n]
    });

    let actions = if state.available_actions.is_empty() {
        if state.to_call > 0.0 {
            vec![Action::Fold, Action::Call]
        } else {
            vec![Action::Check, Action::Bet(BetSize::PotFraction(0.5))]
        }
    } else {
        state.available_actions.clone()
    };

    let action_strategies = actions
        .iter()
        .zip(avg_strategy.iter())
        .map(|(action, &freq)| {
            ActionStrategy {
                action: action.clone(),
                frequency: freq,
                ev: 0.0, // TODO: Compute EV
            }
        })
        .collect();

    Strategy {
        actions: action_strategies,
        iterations,
        convergence,
        game_state: Some(state.clone()),
    }
}
