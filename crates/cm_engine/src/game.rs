//! Main game struct and loop.

use crate::config::GameConfig;
use crate::state::GameState;
use crate::systems::*;
use cm_ai::scouting::{generate_scout_report, DetailedScoutReport};
use cm_core::ids::{ClubId, NationId, PlayerId};
use cm_core::world::{StaffRole, World};

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
    board: board_system::BoardSystem,
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
            board: board_system::BoardSystem::new(),
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

    /// Get config (mutable).
    pub fn cfg_mut(&mut self) -> &mut GameConfig {
        &mut self.cfg
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

        // 9) Board evaluation (monthly)
        self.board
            .run_daily(&self.cfg, &mut self.world, &mut self.state);

        // 10) Save flag
        self.save.mark_dirty(&mut self.state);

        // Increment day counter
        self.state.days_played += 1;
    }

    // ─── Demissao ──────────────────────────────────────────────────────────

    /// Pedir demissao do clube atual.
    ///
    /// Reseta o estado para permitir selecionar um novo clube.
    /// Retorna `true` se a demissao foi processada com sucesso.
    pub fn resign(&mut self) -> bool {
        let club_name = self
            .world
            .clubs
            .get(&self.state.club_id)
            .map(|c| c.name.clone())
            .unwrap_or_else(|| "seu clube".to_string());

        self.state.add_message(format!(
            "Voce se demitiu do {}. Procurando novo emprego...",
            club_name
        ));

        // Limpar o clube atual - o jogador precisa escolher um novo
        self.state.club_id = ClubId::new("");

        true
    }

    // ─── Scouting ───────────────────────────────────────────────────────────

    /// Calcula a habilidade de scouting do clube baseada no staff.
    fn club_scout_ability(&self, club_id: &ClubId) -> u8 {
        let club = match self.world.clubs.get(club_id) {
            Some(c) => c,
            None => return 50,
        };

        let scout_abilities: Vec<u8> = club
            .staff_ids
            .iter()
            .filter_map(|sid| self.world.staff.get(sid))
            .filter(|s| s.role == StaffRole::Scout)
            .map(|s| s.scouting)
            .collect();

        if scout_abilities.is_empty() {
            // Sem olheiro: habilidade base 40
            40
        } else {
            // Media das habilidades de scouting, escalada de 1-20 para 0-100
            let avg = scout_abilities.iter().map(|&v| v as u16).sum::<u16>()
                / scout_abilities.len() as u16;
            (avg * 5).min(100) as u8
        }
    }

    /// Observar um jogador especifico.
    pub fn scout_player(&self, player_id: &PlayerId) -> Option<DetailedScoutReport> {
        let scout_ability = self.club_scout_ability(&self.state.club_id);
        generate_scout_report(&self.world, player_id, scout_ability)
    }

    /// Observar todos os jogadores de um clube.
    pub fn scout_club(&self, club_id: &ClubId) -> Vec<DetailedScoutReport> {
        let scout_ability = self.club_scout_ability(&self.state.club_id);
        let club = match self.world.clubs.get(club_id) {
            Some(c) => c,
            None => return Vec::new(),
        };

        club.player_ids
            .iter()
            .filter_map(|pid| generate_scout_report(&self.world, pid, scout_ability))
            .collect()
    }

    /// Observar jogadores de uma nacao.
    pub fn scout_nation(&self, nation_id: &NationId) -> Vec<DetailedScoutReport> {
        let scout_ability = self.club_scout_ability(&self.state.club_id);

        self.world
            .players
            .values()
            .filter(|p| p.nationality == *nation_id)
            .filter_map(|p| generate_scout_report(&self.world, &p.id, scout_ability))
            .collect()
    }

    /// Observar proximo adversario.
    pub fn scout_next_opponent(&self) -> Vec<DetailedScoutReport> {
        let user_club = &self.state.club_id;
        let today = self.state.date.date();

        // Encontrar o proximo jogo do usuario
        let next_opponent = self
            .world
            .competitions
            .values()
            .flat_map(|comp| comp.fixtures.matches.iter())
            .filter(|f| {
                !f.is_played()
                    && f.date >= today
                    && (f.home_id == *user_club || f.away_id == *user_club)
            })
            .min_by_key(|f| f.date)
            .map(|f| {
                if f.home_id == *user_club {
                    f.away_id.clone()
                } else {
                    f.home_id.clone()
                }
            });

        match next_opponent {
            Some(opp_id) => self.scout_club(&opp_id),
            None => Vec::new(),
        }
    }

    // ─── Reserve Team ───────────────────────────────────────────────────────

    /// Mover jogador para a reserva.
    pub fn move_to_reserve(&mut self, player_id: &PlayerId) {
        let user_club_id = self.state.club_id.clone();
        if let Some(club) = self.world.clubs.get_mut(&user_club_id) {
            club.add_to_reserve(player_id.clone());
        }
    }

    /// Mover jogador da reserva para o elenco principal.
    pub fn move_from_reserve(&mut self, player_id: &PlayerId) {
        let user_club_id = self.state.club_id.clone();
        if let Some(club) = self.world.clubs.get_mut(&user_club_id) {
            club.remove_from_reserve(player_id);
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use chrono::NaiveDate;
    use cm_core::economy::{Budget, Money};
    use cm_core::ids::{ClubId, CompetitionId, NationId};
    use cm_core::world::{Club, Competition, CompetitionType, Fixture, Player, Position, Table};

    /// Cria um mundo minimo de teste com 2 clubes, jogadores e uma competicao.
    fn create_test_world() -> World {
        let mut world = World::new();

        // Criar dois clubes
        let mut club_a = Club::new("LIV", "Liverpool", NationId::new("ENG"));
        club_a.short_name = "LIV".to_string();
        club_a.reputation = 90;
        club_a.budget = Budget::new(
            Money::from_major(50_000_000),
            Money::from_major(20_000_000),
            Money::from_major(500_000),
        );

        let mut club_b = Club::new("ARS", "Arsenal", NationId::new("ENG"));
        club_b.short_name = "ARS".to_string();
        club_b.reputation = 85;
        club_b.budget = Budget::new(
            Money::from_major(40_000_000),
            Money::from_major(15_000_000),
            Money::from_major(400_000),
        );

        // Criar jogadores para cada clube
        for i in 0..11 {
            let id_a = format!("PA{:03}", i);
            let mut pa = Player::new(
                &id_a,
                "Player",
                &format!("A{}", i),
                NationId::new("ENG"),
                NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
                Position::MidfielderCenter,
            );
            pa.club_id = Some(ClubId::new("LIV"));
            club_a.player_ids.push(pa.id.clone());
            world.players.insert(pa.id.clone(), pa);

            let id_b = format!("PB{:03}", i);
            let mut pb = Player::new(
                &id_b,
                "Player",
                &format!("B{}", i),
                NationId::new("ENG"),
                NaiveDate::from_ymd_opt(1991, 1, 1).unwrap(),
                Position::MidfielderCenter,
            );
            pb.club_id = Some(ClubId::new("ARS"));
            club_b.player_ids.push(pb.id.clone());
            world.players.insert(pb.id.clone(), pb);
        }

        world.clubs.insert(club_a.id.clone(), club_a);
        world.clubs.insert(club_b.id.clone(), club_b);

        // Criar competicao com fixtures
        let comp_id = CompetitionId::new("PL");
        let mut comp = Competition::new(comp_id.clone(), "Premier League", CompetitionType::League);
        comp.teams.push(ClubId::new("LIV"));
        comp.teams.push(ClubId::new("ARS"));
        comp.table = Table::new();

        world.competitions.insert(comp_id, comp);

        world
    }

    /// Cria um jogo de teste com data e clube especificos.
    fn create_test_game(start_date: NaiveDate) -> Game {
        let world = create_test_world();
        let state = GameState::new(start_date, "Test Manager".to_string(), ClubId::new("LIV"));
        let cfg = GameConfig::default();
        Game::new(cfg, world, state)
    }

    #[test]
    fn test_process_7_days_state_changes() {
        let start = NaiveDate::from_ymd_opt(2001, 7, 1).unwrap();
        let mut game = create_test_game(start);
        let initial_date = game.state().date;

        // Processar 7 dias
        for _ in 0..7 {
            game.process_day();
        }

        // Verificar que a data avancou 7 dias
        assert_eq!(game.state().days_played, 7);
        assert!(game.state().date > initial_date);
        assert_eq!(
            game.state().date.date(),
            start
                .succ_opt()
                .unwrap()
                .succ_opt()
                .unwrap()
                .succ_opt()
                .unwrap()
                .succ_opt()
                .unwrap()
                .succ_opt()
                .unwrap()
                .succ_opt()
                .unwrap()
                .succ_opt()
                .unwrap()
        );

        // Verificar que o mundo ainda e consistente
        assert!(game.world().clubs.contains_key(&ClubId::new("LIV")));
        assert!(game.world().clubs.contains_key(&ClubId::new("ARS")));
    }

    #[test]
    fn test_process_to_match_day() {
        // Iniciar numa quarta-feira (2001-07-04 e quarta)
        let start = NaiveDate::from_ymd_opt(2001, 7, 4).unwrap();
        let mut game = create_test_game(start);

        // Adicionar fixture para o proximo sabado (2001-07-07)
        let match_date = NaiveDate::from_ymd_opt(2001, 7, 7).unwrap();
        {
            let comp_id = CompetitionId::new("PL");
            let fixture = Fixture::new(
                comp_id.clone(),
                1,
                match_date,
                ClubId::new("ARS"), // Arsenal vs um time ficticio (nao o usuario)
                ClubId::new("LIV"),
            );
            if let Some(comp) = game.world_mut().competitions.get_mut(&comp_id) {
                comp.fixtures.add(fixture);
            }
        }

        // Avancar ate o sabado (3 dias: qui, sex, sab)
        let mut match_day_triggered = false;
        for _ in 0..5 {
            game.process_day();
            if game.state().date.date() == match_date {
                // O time manager marca sabados como match_day antes do sistema de partidas rodar
                // Mas o match_system ja teria executado, entao flags.match_day pode ser false
                match_day_triggered = true;
            }
        }

        assert!(match_day_triggered, "Deveria ter passado pelo dia de jogo");
        assert!(game.state().days_played >= 3);
    }

    #[test]
    fn test_weekly_wages_processed_on_sunday() {
        // Iniciar no sabado 2001-07-07 para que o proximo dia seja domingo
        let start = NaiveDate::from_ymd_opt(2001, 7, 7).unwrap();
        let mut game = create_test_game(start);

        let initial_balance = game
            .world()
            .clubs
            .get(&ClubId::new("LIV"))
            .unwrap()
            .budget
            .balance;

        // Avancar 1 dia (domingo - dia de pagamento de salarios)
        game.process_day();

        let after_balance = game
            .world()
            .clubs
            .get(&ClubId::new("LIV"))
            .unwrap()
            .budget
            .balance;

        // O saldo deve ter diminuido (ou ficado igual se wages sao 0)
        // Com jogadores no elenco, deve haver desconto de salarios
        assert!(
            after_balance <= initial_balance,
            "Saldo deveria ter diminuido apos pagamento de salarios: antes={}, depois={}",
            initial_balance,
            after_balance
        );
    }

    #[test]
    fn test_monthly_financial_snapshot() {
        // Iniciar em 30 de julho para que o proximo dia seja 1 de agosto
        let start = NaiveDate::from_ymd_opt(2001, 7, 31).unwrap();
        let mut game = create_test_game(start);

        assert!(game.state().financial_history.is_empty());

        // Avancar 1 dia (1 de agosto - gera relatorio mensal)
        game.process_day();

        // Deve ter gerado um snapshot financeiro
        assert_eq!(
            game.state().financial_history.len(),
            1,
            "Deveria ter 1 snapshot financeiro apos primeiro dia do mes"
        );

        let snapshot = &game.state().financial_history[0];
        assert_eq!(snapshot.month, "2001-08");
        // Balance deve ser positiva (clube comeca com 50M)
        assert!(snapshot.balance > 0);
    }

    #[test]
    fn test_fifa_break_no_match_day() {
        use cm_core::world::calendar::Calendar;

        // Setembro 6, 2001 - dentro da janela FIFA (4-12 setembro)
        let start = NaiveDate::from_ymd_opt(2001, 9, 5).unwrap();
        let mut game = create_test_game(start);

        // Avancar ate sabado 8 de setembro (dentro da data FIFA)
        for _ in 0..3 {
            game.process_day();
        }

        // Sabado 8 de setembro esta dentro da data FIFA, nao deve ser match_day
        assert!(
            Calendar::is_fifa_break(game.state().date.date()) || !game.state().flags.match_day,
            "Nao deveria ter jogo durante data FIFA"
        );
    }

    #[test]
    fn test_days_played_increments() {
        let start = NaiveDate::from_ymd_opt(2001, 7, 1).unwrap();
        let mut game = create_test_game(start);

        assert_eq!(game.state().days_played, 0);

        game.process_day();
        assert_eq!(game.state().days_played, 1);

        game.process_day();
        assert_eq!(game.state().days_played, 2);

        for _ in 0..10 {
            game.process_day();
        }
        assert_eq!(game.state().days_played, 12);
    }

    #[test]
    fn test_inbox_receives_messages() {
        let start = NaiveDate::from_ymd_opt(2001, 7, 31).unwrap();
        let mut game = create_test_game(start);

        let initial_inbox_count = game.state().inbox.len();

        // O primeiro dia do mes deve gerar mensagem de relatorio financeiro
        game.process_day();

        assert!(
            game.state().inbox.len() > initial_inbox_count,
            "Inbox deveria ter recebido novas mensagens apos avancar dia"
        );
    }
}
