//! Time management system.

use crate::config::GameConfig;
use crate::state::GameState;
use cm_core::world::World;

/// Time manager system.
pub struct TimeManager;

impl TimeManager {
    /// Advance one day.
    pub fn tick_day(&self, _cfg: &GameConfig, _world: &mut World, state: &mut GameState) {
        state.date.advance_day();

        // Check for special dates
        state.flags.match_day = state.date.is_saturday();

        // First of month for finances
        if state.date.is_first_of_month() {
            state.add_message("Relatorio financeiro mensal disponivel.");
        }
    }
}
