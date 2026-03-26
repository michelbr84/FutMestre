//! # CM Core
//!
//! Core domain models for the football manager simulation.
//! Contains world entities, economy models, and simulation primitives.

pub mod competitions;
pub mod economy;
pub mod errors;
pub mod ids;
pub mod prelude;
pub mod sim;
pub mod world;
pub use errors::CoreError;
pub use prelude::*;
