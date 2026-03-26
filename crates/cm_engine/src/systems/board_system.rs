//! Board system - avaliacao mensal da diretoria.

use crate::config::GameConfig;
use crate::state::GameState;
use chrono::Datelike;
use cm_ai::board_ai::{
    calculate_board_satisfaction, evaluate_board_action, generate_expectations, BoardDecision,
    JobRisk,
};
use cm_core::ids::ClubId;
use cm_core::world::World;

/// Board system - avalia satisfacao da diretoria mensalmente.
pub struct BoardSystem {
    /// Meses desde a ultima acao da diretoria.
    months_since_action: u8,
    /// Ultimo mes em que a avaliacao foi feita.
    last_evaluated_month: Option<u32>,
}

impl Default for BoardSystem {
    fn default() -> Self {
        Self {
            months_since_action: 3, // Permite acao no inicio
            last_evaluated_month: None,
        }
    }
}

impl BoardSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run_daily(&mut self, _cfg: &GameConfig, world: &mut World, state: &mut GameState) {
        let date = state.date.date();
        let day = date.day();
        let month = date.month();
        let year = date.year() as u32;

        // Avaliacao mensal no primeiro dia do mes
        if day != 1 {
            return;
        }

        // Evitar avaliar o mesmo mes duas vezes
        let month_key = year * 100 + month;
        if self.last_evaluated_month == Some(month_key) {
            return;
        }
        self.last_evaluated_month = Some(month_key);
        self.months_since_action += 1;

        let club_id = state.club_id.clone();
        let club = match world.clubs.get(&club_id) {
            Some(c) => c,
            None => return,
        };

        // Gerar expectativas baseadas na reputacao do clube
        let expectations = generate_expectations(club.reputation, club.budget.balance);

        // Determinar posicao na liga (buscar na primeira competicao)
        let league_position = find_league_position(world, &club_id);

        // Calcular satisfacao
        let satisfaction = calculate_board_satisfaction(
            &expectations,
            league_position,
            club.budget.balance,
            0, // cup_round_reached - simplificado por enquanto
        );

        // Avaliar acao da diretoria
        let action = evaluate_board_action(&satisfaction, self.months_since_action);

        // Gerar mensagens baseadas na satisfacao e acao
        match satisfaction.risk_level {
            JobRisk::Imminent => {
                // Confianca muito baixa (< 20)
                state.add_message(format!(
                    "[DIRETORIA] A diretoria esta extremamente insatisfeita com os resultados. \
                     Ha rumores de que a diretoria considera a demissao do tecnico. \
                     Satisfacao: {}%",
                    satisfaction.overall
                ));
            }
            JobRisk::AtRisk => {
                // Confianca baixa (20-34)
                state.add_message(format!(
                    "[DIRETORIA] AVISO: A diretoria esta insatisfeita com o desempenho. \
                     Melhore os resultados ou seu cargo estara em risco. \
                     Satisfacao: {}%",
                    satisfaction.overall
                ));
            }
            JobRisk::Warning => {
                // Confianca moderada-baixa (35-49)
                state.add_message(format!(
                    "[DIRETORIA] A diretoria espera uma melhora nos resultados. \
                     Satisfacao: {}%",
                    satisfaction.overall
                ));
            }
            JobRisk::Secure => {
                // Sem mensagem para satisfacao normal
            }
            JobRisk::Safe => {
                if satisfaction.overall >= 85 {
                    state.add_message(format!(
                        "[DIRETORIA] A diretoria esta muito satisfeita com seu trabalho! \
                         Satisfacao: {}%",
                        satisfaction.overall
                    ));
                }
            }
        }

        // Processar acao da diretoria
        match action {
            BoardDecision::IncreaseBudget { amount, reason } => {
                if let Some(club) = world.clubs.get_mut(&club_id) {
                    club.budget.transfer_budget += amount;
                }
                state.add_message(format!(
                    "[DIRETORIA] Orcamento de transferencias aumentado em {}. Motivo: {}",
                    amount, reason
                ));
                self.months_since_action = 0;
            }
            BoardDecision::DecreaseBudget { amount, reason } => {
                if let Some(club) = world.clubs.get_mut(&club_id) {
                    let current = club.budget.transfer_budget;
                    if current > amount {
                        club.budget.transfer_budget -= amount;
                    } else {
                        club.budget.transfer_budget = cm_core::economy::Money::ZERO;
                    }
                }
                state.add_message(format!(
                    "[DIRETORIA] Orcamento de transferencias reduzido em {}. Motivo: {}",
                    amount, reason
                ));
                self.months_since_action = 0;
            }
            BoardDecision::ExtendContract => {
                state.add_message(
                    "[DIRETORIA] A diretoria oferece uma extensao de contrato pelo excelente trabalho!"
                        .to_string(),
                );
                self.months_since_action = 0;
            }
            BoardDecision::Warning => {
                // Mensagem ja enviada acima baseada no risk_level
                self.months_since_action = 0;
            }
            BoardDecision::Termination => {
                state.add_message(
                    "[DIRETORIA] A diretoria decidiu encerrar seu contrato devido aos maus resultados."
                        .to_string(),
                );
                self.months_since_action = 0;
            }
            BoardDecision::NoAction => {}
        }
    }
}

/// Encontra a posicao do clube na liga principal.
fn find_league_position(world: &World, club_id: &ClubId) -> u8 {
    for comp in world.competitions.values() {
        if comp.teams.contains(club_id) {
            // Buscar posicao na tabela
            for (i, row) in comp.table.rows.iter().enumerate() {
                if row.club_id == *club_id {
                    return (i + 1) as u8;
                }
            }
            // Clube na competicao mas nao na tabela: posicao padrao
            return (comp.teams.len() as u8).max(1);
        }
    }
    10 // Padrao se nao encontrar
}
