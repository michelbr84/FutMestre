//! Academy system - generates youth players annually for each club.

use crate::config::GameConfig;
use crate::state::GameState;
use chrono::Datelike;
use cm_core::economy::{Money, Wage};
use cm_core::ids::ClubId;
use cm_core::world::player::PreferredFoot;
use cm_core::world::{Contract, Player, Position, World};
use rand::Rng;

/// Academy system.
pub struct AcademySystem;

impl AcademySystem {
    /// Run daily academy check. On July 1st each year, generate youth players for all clubs.
    pub fn run_daily(&self, _cfg: &GameConfig, world: &mut World, state: &mut GameState) {
        let today = state.date.date();
        // Youth generation happens on July 1st
        if today.month() != 7 || today.day() != 1 {
            return;
        }

        let mut rng = rand::thread_rng();
        self.generate_all_youth(world, state, &mut rng);
    }

    /// Generate youth players for all clubs based on their academy and division level.
    pub fn generate_all_youth(&self, world: &mut World, state: &mut GameState, rng: &mut impl Rng) {
        let club_ids: Vec<ClubId> = world.clubs.keys().cloned().collect();
        let user_club_id = state.club_id.clone();

        for club_id in &club_ids {
            // Determine division level for this club (affects quality/quantity)
            let division_level = find_club_division_level(world, club_id);

            // Get academy from club or use defaults
            let (youth_recruitment, junior_coaching, academy_level) = world
                .clubs
                .get(club_id)
                .map(|club| {
                    // Use club reputation as a proxy for academy quality
                    let rep = club.reputation;
                    let level = (rep / 10).max(1).min(10);
                    let recruitment = (rep / 5).max(1).min(20);
                    let coaching = (rep / 5).max(1).min(20);
                    (recruitment, coaching, level)
                })
                .unwrap_or((10, 10, 5));

            // Number of youth players: 2-5, adjusted by academy level and division
            let base_count = match division_level {
                1 => rng.gen_range(3..=5), // Serie A: 3-5
                2 => rng.gen_range(2..=4), // Serie B: 2-4
                3 => rng.gen_range(2..=3), // Serie C: 2-3
                _ => rng.gen_range(1..=3), // Serie D or unknown: 1-3
            };
            // Academy level bonus: level 8+ can get one extra
            let bonus = if academy_level >= 8 {
                rng.gen_range(0..=1)
            } else {
                0
            };
            let count = (base_count + bonus).min(5);

            let mut generated_names = Vec::new();
            for _ in 0..count {
                let mut player =
                    generate_academy_youth(rng, youth_recruitment, junior_coaching, division_level);
                player.club_id = Some(club_id.clone());

                // Set a basic youth contract (3 years, low wage)
                let today = chrono::Utc::now().date_naive();
                let end_date =
                    chrono::NaiveDate::from_ymd_opt(today.year() + 3, 6, 30).unwrap_or(today);
                player.contract = Some(Contract::new(
                    Wage::weekly(Money::from_major(500)),
                    today,
                    end_date,
                ));

                // Set player value based on potential
                let value_base = (player.potential as i64) * 5_000;
                player.value = Money::from_major(value_base);

                generated_names.push(player.full_name());

                // Add player to world and club
                let player_id = player.id.clone();
                world.players.insert(player_id.clone(), player);
                if let Some(club) = world.clubs.get_mut(club_id) {
                    club.add_player(player_id);
                }
            }

            // Send inbox message for user's club
            if club_id == &user_club_id {
                state.add_message(format!(
                    "Academia: {} jovens promovidos para o elenco profissional. Nomes: {}.",
                    count,
                    generated_names.join(", ")
                ));
            }
        }
    }
}

/// Find the division level for a club (1 = top, 4 = lowest).
fn find_club_division_level(world: &World, club_id: &ClubId) -> u8 {
    for comp in world.competitions.values() {
        if comp.is_league() && comp.teams.contains(club_id) {
            if let Some(div) = comp.division_level {
                return div.level();
            }
        }
    }
    // Default: assume mid-level
    2
}

/// Generate a youth player with attributes influenced by academy quality and division.
///
/// - `youth_recruitment` (1-20): affects potential range
/// - `junior_coaching` (1-20): affects base attributes
/// - `division_level` (1-4): higher division = better base quality
fn generate_academy_youth(
    rng: &mut impl Rng,
    youth_recruitment: u8,
    junior_coaching: u8,
    division_level: u8,
) -> Player {
    let age: u8 = rng.gen_range(16..=18);
    let birth_year = chrono::Utc::now().date_naive().year_ce().1 as i32 - age as i32;
    let birth_month: u32 = rng.gen_range(1..=12);
    let birth_day: u32 = rng.gen_range(1..=28);
    let birth_date = chrono::NaiveDate::from_ymd_opt(birth_year, birth_month, birth_day)
        .unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(birth_year, 1, 1).unwrap());

    let id_num: u32 = rng.gen_range(100_000..999_999);
    let player_id = format!("YTH{id_num}");

    // Brazilian first and last names
    let first_names = [
        "Lucas",
        "Gabriel",
        "Matheus",
        "Pedro",
        "Rafael",
        "Bruno",
        "Felipe",
        "Thiago",
        "Vinicius",
        "Arthur",
        "Carlos",
        "Diego",
        "Eduardo",
        "Gustavo",
        "Henrique",
        "Leonardo",
        "Andre",
        "Joao",
        "Caio",
        "Enzo",
        "Davi",
        "Bernardo",
        "Luan",
        "Miguel",
        "Samuel",
        "Daniel",
        "Renan",
        "Igor",
        "Guilherme",
        "Marcos",
    ];
    let last_names = [
        "Silva",
        "Santos",
        "Oliveira",
        "Souza",
        "Lima",
        "Pereira",
        "Costa",
        "Almeida",
        "Ferreira",
        "Rodrigues",
        "Gomes",
        "Martins",
        "Araujo",
        "Ribeiro",
        "Carvalho",
        "Barbosa",
        "Nascimento",
        "Moura",
        "Teixeira",
        "Vieira",
        "Campos",
        "Monteiro",
        "Nunes",
        "Pinto",
        "Mendes",
    ];

    let first = first_names[rng.gen_range(0..first_names.len())];
    let last = last_names[rng.gen_range(0..last_names.len())];

    let positions = [
        Position::Goalkeeper,
        Position::DefenderCenter,
        Position::DefenderLeft,
        Position::DefenderRight,
        Position::MidfielderCenter,
        Position::MidfielderLeft,
        Position::MidfielderRight,
        Position::MidfielderDefensive,
        Position::MidfielderAttacking,
        Position::ForwardCenter,
        Position::ForwardLeft,
        Position::ForwardRight,
    ];
    let position = positions[rng.gen_range(0..positions.len())];

    let mut player = Player::new(
        player_id,
        first,
        last,
        cm_core::ids::NationId::new("BRA"),
        birth_date,
        position,
    );

    // Attribute range: influenced by junior_coaching (1-20) and division_level (1-4)
    // Higher coaching and higher division -> better base attributes
    let coaching_bonus = (junior_coaching as i16 - 10).max(0) as u8; // 0-10 bonus
    let division_bonus = match division_level {
        1 => 8, // Top division clubs produce slightly better youth
        2 => 4,
        3 => 2,
        _ => 0,
    };
    let attr_min: u8 = (20 + coaching_bonus / 2 + division_bonus).min(45);
    let attr_max: u8 = (55 + coaching_bonus + division_bonus).min(70);

    macro_rules! ra {
        ($r:expr) => {
            $r.gen_range(attr_min..=attr_max)
        };
    }

    // Technical
    player.attributes.technical.crossing = ra!(rng);
    player.attributes.technical.dribbling = ra!(rng);
    player.attributes.technical.finishing = ra!(rng);
    player.attributes.technical.first_touch = ra!(rng);
    player.attributes.technical.free_kick = ra!(rng);
    player.attributes.technical.heading = ra!(rng);
    player.attributes.technical.long_shots = ra!(rng);
    player.attributes.technical.marking = ra!(rng);
    player.attributes.technical.passing = ra!(rng);
    player.attributes.technical.penalties = ra!(rng);
    player.attributes.technical.tackling = ra!(rng);
    player.attributes.technical.technique = ra!(rng);

    // Mental
    player.attributes.mental.aggression = ra!(rng);
    player.attributes.mental.anticipation = ra!(rng);
    player.attributes.mental.bravery = ra!(rng);
    player.attributes.mental.composure = ra!(rng);
    player.attributes.mental.concentration = ra!(rng);
    player.attributes.mental.decisions = ra!(rng);
    player.attributes.mental.determination = ra!(rng);
    player.attributes.mental.flair = ra!(rng);
    player.attributes.mental.leadership = ra!(rng);
    player.attributes.mental.off_the_ball = ra!(rng);
    player.attributes.mental.positioning = ra!(rng);
    player.attributes.mental.teamwork = ra!(rng);
    player.attributes.mental.vision = ra!(rng);
    player.attributes.mental.work_rate = ra!(rng);

    // Physical
    player.attributes.physical.acceleration = ra!(rng);
    player.attributes.physical.agility = ra!(rng);
    player.attributes.physical.balance = ra!(rng);
    player.attributes.physical.jumping = ra!(rng);
    player.attributes.physical.natural_fitness = ra!(rng);
    player.attributes.physical.pace = ra!(rng);
    player.attributes.physical.stamina = ra!(rng);
    player.attributes.physical.strength = ra!(rng);

    // Goalkeeper (high only for GK position)
    if position == Position::Goalkeeper {
        player.attributes.goalkeeper.aerial_ability = ra!(rng);
        player.attributes.goalkeeper.command_of_area = ra!(rng);
        player.attributes.goalkeeper.communication = ra!(rng);
        player.attributes.goalkeeper.handling = ra!(rng);
        player.attributes.goalkeeper.kicking = ra!(rng);
        player.attributes.goalkeeper.one_on_ones = ra!(rng);
        player.attributes.goalkeeper.positioning = ra!(rng);
        player.attributes.goalkeeper.reflexes = ra!(rng);
        player.attributes.goalkeeper.throwing = ra!(rng);
    }

    // Potential: influenced by youth_recruitment (1-20) and division
    // Range: 60-95, with better recruitment yielding higher potential
    let recruitment_bonus = (youth_recruitment as i16 - 10).max(0) as u8; // 0-10
    let pot_min: u8 = (60 + recruitment_bonus / 2 + division_bonus / 2).min(80);
    let pot_max: u8 = (85 + recruitment_bonus + division_bonus / 2).min(95);
    player.potential = rng.gen_range(pot_min..=pot_max);

    player.fitness = rng.gen_range(75..=95);
    player.form = rng.gen_range(40..=60);

    // Preferred foot
    let foot_roll: u8 = rng.gen_range(0..100);
    player.preferred_foot = if foot_roll < 70 {
        PreferredFoot::Right
    } else if foot_roll < 95 {
        PreferredFoot::Left
    } else {
        PreferredFoot::Either
    };

    player
}

#[cfg(test)]
mod tests {
    use super::*;
    use cm_core::ids::{ClubId, CompetitionId};
    use cm_core::world::{Club, Competition, DivisionLevel};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn setup_world_with_clubs() -> World {
        let mut world = World::new();

        // Create two clubs
        let mut club_a = Club::new("CLUBA", "Clube A FC", cm_core::ids::NationId::new("BRA"));
        club_a.reputation = 80;
        world.clubs.insert(club_a.id.clone(), club_a);

        let mut club_b = Club::new("CLUBB", "Clube B FC", cm_core::ids::NationId::new("BRA"));
        club_b.reputation = 40;
        world.clubs.insert(club_b.id.clone(), club_b);

        // Create a league with both clubs
        let mut comp = Competition::new_league("LIGA1", "Serie A", DivisionLevel::SerieA);
        comp.add_team(ClubId::new("CLUBA"));
        comp.add_team(ClubId::new("CLUBB"));
        world.competitions.insert(CompetitionId::new("LIGA1"), comp);

        world
    }

    #[test]
    fn test_generate_academy_youth_basic() {
        let mut rng = StdRng::seed_from_u64(42);
        let player = generate_academy_youth(&mut rng, 15, 15, 1);

        let today = chrono::Utc::now().date_naive();
        let age = player.age_on(today);
        assert!(age >= 15 && age <= 18, "youth should be 15-18, got {age}");
        assert!(
            player.potential >= 60,
            "potential should be >= 60, got {}",
            player.potential
        );
        assert!(
            player.potential <= 95,
            "potential should be <= 95, got {}",
            player.potential
        );
        assert!(player.attributes.technical.passing >= 20);
    }

    #[test]
    fn test_division_affects_quality() {
        let mut rng1 = StdRng::seed_from_u64(100);
        let mut rng2 = StdRng::seed_from_u64(100);

        // Generate many players to compare averages
        let mut top_div_sum = 0u32;
        let mut low_div_sum = 0u32;
        for _ in 0..50 {
            let p1 = generate_academy_youth(&mut rng1, 15, 15, 1);
            let p2 = generate_academy_youth(&mut rng2, 15, 15, 4);
            top_div_sum += p1.potential as u32;
            low_div_sum += p2.potential as u32;
        }
        // Top division should produce higher average potential
        assert!(
            top_div_sum > low_div_sum,
            "top division avg potential ({}) should exceed lower division ({})",
            top_div_sum / 50,
            low_div_sum / 50,
        );
    }

    #[test]
    fn test_generate_all_youth_adds_players() {
        let mut world = setup_world_with_clubs();
        let mut state = GameState::new(
            chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
            "Teste".to_string(),
            ClubId::new("CLUBA"),
        );
        let mut rng = StdRng::seed_from_u64(42);

        let initial_player_count = world.players.len();
        let system = AcademySystem;
        system.generate_all_youth(&mut world, &mut state, &mut rng);

        assert!(
            world.players.len() > initial_player_count,
            "should have generated new players"
        );

        // Check that user club got an inbox message
        assert!(
            state.inbox.iter().any(|m| m.contains("Academia")),
            "should have academy inbox message"
        );

        // Check that generated players have club assignments
        for player in world.players.values() {
            if player.id.0.starts_with("YTH") {
                assert!(
                    player.club_id.is_some(),
                    "youth player should be assigned to a club"
                );
                assert!(
                    player.contract.is_some(),
                    "youth player should have a contract"
                );
            }
        }
    }

    #[test]
    fn test_academy_only_triggers_july_1() {
        let mut world = setup_world_with_clubs();
        let system = AcademySystem;
        let cfg = GameConfig::default();

        // Not July 1st - should not generate
        let mut state = GameState::new(
            chrono::NaiveDate::from_ymd_opt(2026, 3, 15).unwrap(),
            "Teste".to_string(),
            ClubId::new("CLUBA"),
        );
        system.run_daily(&cfg, &mut world, &mut state);
        assert_eq!(world.players.len(), 0, "should not generate outside July 1");

        // July 1st - should generate
        let mut state = GameState::new(
            chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
            "Teste".to_string(),
            ClubId::new("CLUBA"),
        );
        system.run_daily(&cfg, &mut world, &mut state);
        assert!(world.players.len() > 0, "should generate on July 1st");
    }
}
