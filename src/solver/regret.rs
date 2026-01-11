//! Regret tracking and matching.
//!
//! This module defines the `RegretTable` struct for storing cumulative regrets
//! and the `regret_to_strategy` function for converting regrets to strategy probabilities.

use crate::solver::info_set::InfoSetKey;
use rustc_hash::FxHashMap;

/// Convert cumulative regrets to a strategy using regret matching.
///
/// Any negative regrets are treated as zero. The resulting strategy is normalized
/// so that probabilities sum to 1.0. If all positive regrets are zero, a uniform
/// strategy is returned.
pub fn regret_to_strategy(regrets: &[f64]) -> Vec<f64> {
    let positive_regrets: Vec<f64> = regrets.iter().map(|&r| r.max(0.0)).collect();
    let sum_positive_regret: f64 = positive_regrets.iter().sum();

    if sum_positive_regret > 0.0 {
        positive_regrets
            .into_iter()
            .map(|r| r / sum_positive_regret)
            .collect()
    } else {
        let count = regrets.len();
        if count == 0 {
            return Vec::new();
        }
        let uniform_prob = 1.0 / count as f64;
        vec![uniform_prob; count]
    }
}

/// Storage for cumulative regrets and strategy sums across all information sets.
pub struct RegretTable {
    /// Cumulative regret per action per info set
    regrets: FxHashMap<InfoSetKey, Vec<f64>>,
    /// Sum of strategies weighted by reach probability (for average strategy)
    strategy_sum: FxHashMap<InfoSetKey, Vec<f64>>,
}

impl Default for RegretTable {
    fn default() -> Self {
        Self::new()
    }
}

impl RegretTable {
    /// Create a new empty regret table.
    pub fn new() -> Self {
        Self {
            regrets: FxHashMap::default(),
            strategy_sum: FxHashMap::default(),
        }
    }

    /// Get the current strategy for an info set using regret matching.
    ///
    /// If the info set doesn't exist, it initializes it with zero regrets.
    pub fn get_strategy(&mut self, key: &InfoSetKey, n_actions: usize) -> Vec<f64> {
        let regrets = self
            .regrets
            .entry(key.clone())
            .or_insert_with(|| vec![0.0; n_actions]);
        regret_to_strategy(regrets)
    }

    /// Update regrets and strategy sum for an info set.
    pub fn update_regrets(&mut self, key: InfoSetKey, new_regrets: &[f64], reach_prob: f64) {
        let n_actions = new_regrets.len();

        // Update cumulative regrets
        let regrets = self
            .regrets
            .entry(key.clone())
            .or_insert_with(|| vec![0.0; n_actions]);
        for (i, &r) in new_regrets.iter().enumerate() {
            regrets[i] += r;
        }

        // Update strategy sum (using current strategy * reach_prob)
        // Note: In standard CFR, we update strategy sum based on the strategy used in this iteration.
        // We need to re-calculate the strategy used to update the sum.
        // Optimization: Pass the strategy used in this iteration to this function to avoid re-calculation?
        // For now, let's re-calculate it or assume the caller handles it.
        // Actually, the standard way is to update strategy sum with σ(a) * π_{-i}.
        // But here we are doing External Sampling.
        // In External Sampling, we update the average strategy by adding the current strategy to the sum.
        // Since we sample one history, the reach prob is effectively 1 for the sampled path (conceptually).
        // But wait, for the average strategy to converge to Nash, we usually weight by hero's reach prob?
        // In External Sampling MCCFR, the average strategy is updated by adding the current strategy (unweighted)
        // if we update all hero hands. But we are iterating over all hero hands.
        // So we should weight by the probability of the hero hand?
        // Let's stick to the plan: "Sum of strategies weighted by reach probability".
        // Since we iterate all hero hands, the "reach prob" is the probability of having that hand (range weight).

        let current_strategy = regret_to_strategy(regrets);
        let strategy_sum = self
            .strategy_sum
            .entry(key)
            .or_insert_with(|| vec![0.0; n_actions]);

        for (i, &prob) in current_strategy.iter().enumerate() {
            strategy_sum[i] += prob * reach_prob;
        }
    }

    /// Get the average strategy for an info set (converged strategy).
    pub fn get_average_strategy(&self, key: &InfoSetKey) -> Option<Vec<f64>> {
        self.strategy_sum.get(key).map(|sum| {
            let total: f64 = sum.iter().sum();
            if total > 0.0 {
                sum.iter().map(|&s| s / total).collect()
            } else {
                // Fallback to uniform if no strategy sum accumulated
                let count = sum.len();
                vec![1.0 / count as f64; count]
            }
        })
    }

    /// Get all info set keys.
    pub fn keys(&self) -> impl Iterator<Item = &InfoSetKey> {
        self.strategy_sum.keys()
    }
}
