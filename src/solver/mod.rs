//! GTO solver implementation using MCCFR

// Re-export solver components
pub mod cfr;
// pub mod equity;
pub mod evaluator;
pub mod info_set;
pub mod mccfr;
pub mod regret;

pub use cfr::solve;
pub use mccfr::{solve_with_config, MccfrConfig};
