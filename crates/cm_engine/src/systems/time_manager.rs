//! Time management system.

use crate::config::GameConfig;
use crate::state::GameState;
use cm_core::world::calendar::Calendar;
use cm_core::world::World;

/// Time manager system.
pub struct TimeManager;

impl TimeManager {
    /// Advance one day.
    pub fn tick_day(&self, _cfg: &GameConfig, world: &mut World, state: &mut GameState) {
        state.date.advance_day();

        // Check for special dates — but NOT during FIFA international breaks
        let today = state.date.date();
        let is_fifa_break =
            Calendar::is_fifa_break(today) || world.calendar.is_international_break(today);

        if is_fifa_break {
            // No matches during international breaks
            state.flags.match_day = false;
        } else {
            state.flags.match_day = state.date.is_saturday();
        }

        // First of month for finances
        if state.date.is_first_of_month() {
            state.add_message("Relatorio financeiro mensal disponivel.");
        }

        // Notify player about international break start
        if is_fifa_break && !Calendar::is_fifa_break(today.pred_opt().unwrap_or(today)) {
            state.add_message(
                "Data FIFA: pausa internacional em andamento. Nenhuma partida sera disputada."
                    .to_string(),
            );
        }
    }
}
