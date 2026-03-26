//! Main game struct and loop.

use crate::config::GameConfig;
use crate::state::GameState;
use crate::systems::*;
use cm_core::world::World;

/// Main game struct.
pub struct Game {
    cfg: GameConfig,
    world: World,
    state: GameState,
    // Systems
    time: time_manager::TimeManager,
    competitions: competition_system::CompetitionSystem,
    matches: match_system::MatchSystem,
    transfers: transfer_system::TransferSystem,
    finance: finance_system::FinanceSystem,
    ai: ai_system::AiSystem,
    morale: morale_system::MoraleSystem,
    save: save_system::SaveSystem,
    academy: academy_system::AcademySystem,
}

impl Game {
    /// Create a new game.
    pub fn new(cfg: GameConfig, world: World, state: GameState) -> Self {
        Self {
            cfg,
            world,
            state,
            time: time_manager::TimeManager,
            competitions: competition_system::CompetitionSystem,
            matches: match_system::MatchSystem,
            transfers: transfer_system::TransferSystem,
            finance: finance_system::FinanceSystem,
            ai: ai_system::AiSystem,
            morale: morale_system::MoraleSystem,
            save: save_system::SaveSystem,
            academy: academy_system::AcademySystem,
        }
    }

    /// Get config.
    pub fn cfg(&self) -> &GameConfig {
        &self.cfg
    }

    /// Get state.
    pub fn state(&self) -> &GameState {
        &self.state
    }

    /// Get mutable state.
    pub fn state_mut(&mut self) -> &mut GameState {
        &mut self.state
    }

    /// Get world.
    pub fn world(&self) -> &World {
        &self.world
    }

    /// Get mutable world.
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    /// Bootstrap initial inbox messages.
    pub fn bootstrap_inbox(&mut self) {
        let club_name = self
            .world
            .clubs
            .get(&self.state.club_id)
            .map(|c| c.name.clone())
            .unwrap_or_else(|| "seu clube".to_string());
        self.state.add_message(format!(
            "Bem-vindo ao {}! Seu desafio comeca hoje. A diretoria espera grandes resultados.",
            club_name
        ));
        self.state.add_message(
            "Revise o elenco e defina suas taticas antes da primeira partida.".to_string(),
        );
    }

    /// Process one day.
    pub fn process_day(&mut self) {
        // 1) Time management
        self.time
            .tick_day(&self.cfg, &mut self.world, &mut self.state);

        // 2) AI (pre-match decisions)
        self.ai
            .run_daily(&self.cfg, &mut self.world, &mut self.state);

        // 3) Competitions (fixtures/tables)
        self.competitions
            .run_daily(&self.cfg, &mut self.world, &mut self.state);

        // 4) Match day?
        if self.state.flags.match_day {
            self.matches
                .run_match_day(&self.cfg, &mut self.world, &mut self.state);
        }

        // 5) Transfer market
        self.transfers
            .run_daily(&self.cfg, &mut self.world, &mut self.state);

        // 6) Finances
        self.finance
            .run_daily(&self.cfg, &mut self.world, &mut self.state);

        // 7) Morale/training
        self.morale
            .run_daily(&self.cfg, &mut self.world, &mut self.state);

        // 8) Academy (youth generation on July 1st)
        self.academy
            .run_daily(&self.cfg, &mut self.world, &mut self.state);

        // 9) Save flag
        self.save.mark_dirty(&mut self.state);

        // Increment day counter
        self.state.days_played += 1;
    }
}
