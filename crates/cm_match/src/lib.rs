//! # CM Match
//!
//! Match simulation engine with probabilistic tick-by-tick simulation.

pub mod commentary;
pub mod discipline;
pub mod fatigue;
pub mod injuries;
pub mod model;
pub mod probabilistic;
pub mod ratings;
pub mod referee;
pub mod set_pieces;
pub mod tactics;
pub mod tests;

pub use model::{MatchEvent, MatchEventType, MatchInput, MatchResult, MatchStats, TeamStrength};
pub use probabilistic::{simulate_match, simulate_with_extra_time};
