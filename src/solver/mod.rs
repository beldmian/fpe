//! GTO solver implementation using MCCFR

// Re-export solver components
pub mod cfr;
// pub mod equity;
pub mod evaluator;

pub use cfr::solve;
// pub use equity::Equity;
