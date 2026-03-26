//! Statistics export to plain text.

use std::fmt::Write;
use std::path::Path;

use cm_core::world::World;

use crate::errors::SaveError;
use crate::snapshot::GameStateData;

/// Export season statistics to a plain text file.
///
/// Includes league tables, top scorers, club financial summary, and squad details.
pub fn export_statistics(
    world: &World,
    state: &GameStateData,
    path: &str,
) -> Result<(), SaveError> {
    let mut output = String::with_capacity(8192);

    // Header
    writeln!(
        output,
        "===================================================="
    )
    .unwrap();
    writeln!(output, "  FutMestre - Relatorio de Estatisticas").unwrap();
    writeln!(
        output,
        "===================================================="
    )
    .unwrap();
    writeln!(output, "Manager: {}", state.manager_name).unwrap();
    writeln!(output, "Clube:   {}", state.club_id).unwrap();
    writeln!(output, "Data:    {}", state.date).unwrap();
    writeln!(output).unwrap();

    // League tables
    writeln!(
        output,
        "----------------------------------------------------"
    )
    .unwrap();
    writeln!(output, "  CLASSIFICACAO").unwrap();
    writeln!(
        output,
        "----------------------------------------------------"
    )
    .unwrap();

    let mut competitions: Vec<_> = world.competitions.values().collect();
    competitions.sort_by_key(|c| c.division_level.map(|d| d.level()).unwrap_or(99));

    for comp in &competitions {
        if !comp.is_league() {
            continue;
        }
        writeln!(output).unwrap();
        writeln!(output, "  {}", comp.name).unwrap();
        writeln!(
            output,
            "  {:<4} {:<25} {:>3} {:>3} {:>3} {:>3} {:>4} {:>4} {:>4} {:>4}",
            "#", "Clube", "J", "V", "E", "D", "GP", "GC", "SG", "Pts"
        )
        .unwrap();
        writeln!(
            output,
            "  ---- ------------------------- --- --- --- --- ---- ---- ---- ----"
        )
        .unwrap();

        let mut sorted_rows = comp.table.rows.clone();
        sorted_rows.sort_by(|a, b| {
            b.points
                .cmp(&a.points)
                .then_with(|| b.goal_difference().cmp(&a.goal_difference()))
                .then_with(|| b.goals_for.cmp(&a.goals_for))
        });

        for (i, row) in sorted_rows.iter().enumerate() {
            let club_name = world
                .clubs
                .get(&row.club_id)
                .map(|c| c.name.as_str())
                .unwrap_or(row.club_id.0.as_str());
            let club_display = if club_name.len() > 25 {
                &club_name[..25]
            } else {
                club_name
            };
            writeln!(
                output,
                "  {:<4} {:<25} {:>3} {:>3} {:>3} {:>3} {:>4} {:>4} {:>4} {:>4}",
                i + 1,
                club_display,
                row.played,
                row.won,
                row.drawn,
                row.lost,
                row.goals_for,
                row.goals_against,
                row.goal_difference(),
                row.points,
            )
            .unwrap();
        }
    }

    // Top scorers
    writeln!(output).unwrap();
    writeln!(
        output,
        "----------------------------------------------------"
    )
    .unwrap();
    writeln!(output, "  ARTILHEIROS").unwrap();
    writeln!(
        output,
        "----------------------------------------------------"
    )
    .unwrap();

    for comp in &competitions {
        if comp.top_scorers.is_empty() {
            continue;
        }
        writeln!(output).unwrap();
        writeln!(output, "  {}", comp.name).unwrap();
        writeln!(
            output,
            "  {:<4} {:<25} {:<20} {:>4} {:>4}",
            "#", "Jogador", "Clube", "Gols", "Asst"
        )
        .unwrap();

        let mut scorers = comp.top_scorers.clone();
        scorers.sort_by(|a, b| {
            b.goals
                .cmp(&a.goals)
                .then_with(|| b.assists.cmp(&a.assists))
        });

        for (i, scorer) in scorers.iter().take(20).enumerate() {
            let player_name = world
                .players
                .get(&scorer.player_id)
                .map(|p| p.full_name())
                .unwrap_or_else(|| scorer.player_id.0.clone());
            let club_name = world
                .clubs
                .get(&scorer.club_id)
                .map(|c| c.short_name.clone())
                .unwrap_or_else(|| scorer.club_id.0.clone());

            let name_display = if player_name.len() > 25 {
                player_name[..25].to_string()
            } else {
                player_name
            };
            let club_display = if club_name.len() > 20 {
                &club_name[..20]
            } else {
                &club_name
            };
            writeln!(
                output,
                "  {:<4} {:<25} {:<20} {:>4} {:>4}",
                i + 1,
                name_display,
                club_display,
                scorer.goals,
                scorer.assists,
            )
            .unwrap();
        }
    }

    // Club financial summary
    writeln!(output).unwrap();
    writeln!(
        output,
        "----------------------------------------------------"
    )
    .unwrap();
    writeln!(output, "  FINANCAS DO CLUBE").unwrap();
    writeln!(
        output,
        "----------------------------------------------------"
    )
    .unwrap();

    let user_club_id = cm_core::ids::ClubId::new(&state.club_id);
    if let Some(club) = world.clubs.get(&user_club_id) {
        writeln!(output, "  Saldo:           {}", club.budget.balance).unwrap();
        writeln!(
            output,
            "  Orc. Transferencias: {}",
            club.budget.transfer_budget
        )
        .unwrap();
        writeln!(output, "  Orc. Salarios:   {}", club.budget.wage_budget).unwrap();
        writeln!(
            output,
            "  Folha Salarial:  {} /semana",
            club.budget.wage_bill
        )
        .unwrap();
    }

    // Squad details
    writeln!(output).unwrap();
    writeln!(
        output,
        "----------------------------------------------------"
    )
    .unwrap();
    writeln!(output, "  ELENCO").unwrap();
    writeln!(
        output,
        "----------------------------------------------------"
    )
    .unwrap();

    if let Some(club) = world.clubs.get(&user_club_id) {
        writeln!(
            output,
            "  {:<4} {:<25} {:<4} {:>4} {:>4} {:>4} {:>4}",
            "#", "Jogador", "Pos", "Idade", "OVR", "Pot", "Fit"
        )
        .unwrap();
        writeln!(
            output,
            "  ---- ------------------------- ---- ---- ---- ---- ----"
        )
        .unwrap();

        let today = chrono::Utc::now().date_naive();
        let mut players: Vec<_> = club
            .player_ids
            .iter()
            .filter_map(|pid| world.players.get(pid))
            .collect();
        players.sort_by_key(|p| position_sort_key(&p.position));

        for (i, player) in players.iter().enumerate() {
            let name = player.full_name();
            let name_display = if name.len() > 25 { &name[..25] } else { &name };
            let age = player.age_on(today);
            let ovr = if player.position == cm_core::world::Position::Goalkeeper {
                player.attributes.keeper_rating()
            } else if player.position.is_defender() {
                player.attributes.defense_rating()
            } else if player.position.is_forward() {
                player.attributes.attack_rating()
            } else {
                player.attributes.midfield_rating()
            };
            writeln!(
                output,
                "  {:<4} {:<25} {:<4} {:>4} {:>4} {:>4} {:>4}",
                i + 1,
                name_display,
                player.position.short_name(),
                age,
                ovr,
                player.potential,
                player.fitness,
            )
            .unwrap();
        }
        writeln!(output).unwrap();
        writeln!(output, "  Total: {} jogadores", players.len()).unwrap();
    }

    writeln!(output).unwrap();
    writeln!(
        output,
        "===================================================="
    )
    .unwrap();
    writeln!(output, "  Gerado pelo FutMestre").unwrap();
    writeln!(
        output,
        "===================================================="
    )
    .unwrap();

    // Write to file
    let parent = Path::new(path).parent();
    if let Some(dir) = parent {
        if !dir.as_os_str().is_empty() {
            std::fs::create_dir_all(dir)?;
        }
    }
    std::fs::write(path, output)?;
    Ok(())
}

/// Helper: sort key for positions (GK first, then DEF, MID, FWD).
fn position_sort_key(pos: &cm_core::world::Position) -> u8 {
    use cm_core::world::Position;
    match pos {
        Position::Goalkeeper => 0,
        Position::DefenderLeft => 1,
        Position::DefenderCenter => 2,
        Position::DefenderRight => 3,
        Position::MidfielderDefensive => 4,
        Position::MidfielderLeft => 5,
        Position::MidfielderCenter => 6,
        Position::MidfielderRight => 7,
        Position::MidfielderAttacking => 8,
        Position::ForwardLeft => 9,
        Position::ForwardCenter => 10,
        Position::ForwardRight => 11,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cm_core::ids::{ClubId, CompetitionId, NationId};
    use cm_core::world::{Club, Competition, DivisionLevel, Player, Position};

    #[test]
    fn test_export_statistics_basic() {
        let mut world = World::new();

        // Add a club
        let mut club = Club::new("TST", "Teste FC", NationId::new("BRA"));
        club.reputation = 70;

        // Add a player
        let mut player = Player::new(
            "P001",
            "Joao",
            "Silva",
            NationId::new("BRA"),
            chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            Position::MidfielderCenter,
        );
        player.club_id = Some(ClubId::new("TST"));
        player.fitness = 85;
        player.potential = 75;
        club.player_ids.push(player.id.clone());
        world.players.insert(player.id.clone(), player);
        world.clubs.insert(club.id.clone(), club);

        // Add a competition
        let mut comp = Competition::new_league("LIGA", "Serie A Test", DivisionLevel::SerieA);
        comp.add_team(ClubId::new("TST"));
        world.competitions.insert(CompetitionId::new("LIGA"), comp);

        let state = GameStateData {
            date: "01 Jul 2026".to_string(),
            manager_name: "Manager Teste".to_string(),
            club_id: "TST".to_string(),
            inbox: Vec::new(),
        };

        let tmp_dir = std::env::temp_dir().join(format!("cm_export_test_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&tmp_dir);
        let path = tmp_dir.join("stats.txt");
        let path_str = path.to_str().unwrap();

        let result = export_statistics(&world, &state, path_str);
        assert!(result.is_ok(), "export should succeed");

        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("FutMestre"));
        assert!(content.contains("Manager Teste"));
        assert!(content.contains("TST"));
        assert!(content.contains("Joao Silva"));
        assert!(content.contains("Serie A Test"));

        let _ = std::fs::remove_dir_all(&tmp_dir);
    }
}
