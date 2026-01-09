//! Output formatting for CLI

use crate::models::strategy::Strategy;
use tabled::{Table, Tabled};

/// Row structure for the strategy table
#[derive(Tabled)]
struct StrategyRow {
    #[tabled(rename = "Action")]
    action: String,

    #[tabled(rename = "Frequency")]
    frequency: String,

    #[tabled(rename = "EV (BB)")]
    ev: String,
}

/// Format strategy as an ASCII table
pub fn format_strategy_table(strategy: &Strategy) -> String {
    let sorted_actions = strategy.sorted_by_frequency();

    let rows: Vec<StrategyRow> = sorted_actions
        .into_iter()
        .map(|a| StrategyRow {
            action: a.action.display_name(),
            frequency: format!("{:.1}%", a.frequency * 100.0),
            ev: format!("{:+6.2}", a.ev), // Always sign, width 6
        })
        .collect();

    Table::new(rows).to_string()
}
