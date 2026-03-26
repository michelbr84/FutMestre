//! JSON world importer.

use std::path::Path;

use chrono::NaiveDate;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::Deserialize;

use cm_core::economy::{Budget, Money};
use cm_core::ids::*;
use cm_core::world::*;

use crate::errors::DataError;

/// Brazilian first names for procedural generation.
const FIRST_NAMES: &[&str] = &[
    "João",
    "Pedro",
    "Lucas",
    "Gabriel",
    "Matheus",
    "Rafael",
    "Bruno",
    "Felipe",
    "Gustavo",
    "Leonardo",
    "Ricardo",
    "Carlos",
    "André",
    "Paulo",
    "Marcos",
    "Eduardo",
    "Fernando",
    "Diego",
    "Thiago",
    "Rodrigo",
    "Daniel",
    "Alexandre",
    "Vinicius",
    "Henrique",
    "Arthur",
    "Caio",
    "Leandro",
    "Marcelo",
    "Fabio",
    "Sergio",
    "Renato",
    "Willian",
    "Luiz",
    "Danilo",
    "Igor",
    "Renan",
    "Hugo",
    "Victor",
    "Otávio",
    "Wesley",
    "Luan",
    "Yuri",
    "Raul",
    "Enzo",
    "Murilo",
];

/// Brazilian last names for procedural generation.
const LAST_NAMES: &[&str] = &[
    "Silva",
    "Santos",
    "Oliveira",
    "Souza",
    "Lima",
    "Pereira",
    "Costa",
    "Ferreira",
    "Rodrigues",
    "Almeida",
    "Nascimento",
    "Carvalho",
    "Gomes",
    "Martins",
    "Araújo",
    "Ribeiro",
    "Barbosa",
    "Rocha",
    "Dias",
    "Moreira",
    "Mendes",
    "Nunes",
    "Correia",
    "Vieira",
    "Lopes",
    "Monteiro",
    "Batista",
    "Cardoso",
    "Teixeira",
    "Freitas",
    "Pinto",
    "Melo",
    "Cunha",
    "Andrade",
    "Barros",
    "Campos",
    "Rezende",
    "Machado",
    "Ramos",
    "Fonseca",
];

/// Schema version information loaded from schema_version.json.
#[derive(Debug, Clone, Deserialize)]
pub struct SchemaVersion {
    pub version: String,
    pub min_compatible: String,
}

/// Current schema version expected by this build.
pub const CURRENT_SCHEMA_VERSION: &str = "1.0.0";

/// Check the data schema version.
///
/// Reads `schema_version.json` from the data directory and validates
/// that the data files are compatible with this build.
/// Returns the version string if compatible.
pub fn check_schema_version(data_dir: &str) -> Result<String, DataError> {
    let path = Path::new(data_dir).join("schema_version.json");
    if !path.exists() {
        // No version file means default/embedded data - always compatible
        return Ok(CURRENT_SCHEMA_VERSION.to_string());
    }

    let content = std::fs::read_to_string(&path)?;
    let schema: SchemaVersion = serde_json::from_str(&content)
        .map_err(|e| DataError::Validation(format!("Invalid schema_version.json: {}", e)))?;

    // Parse versions for comparison (simple semver: major.minor.patch)
    let current_parts = parse_semver(CURRENT_SCHEMA_VERSION);
    let min_parts = parse_semver(&schema.min_compatible);

    if current_parts < min_parts {
        return Err(DataError::Validation(format!(
            "Data schema version {} requires minimum compatible {}, but current build supports {}",
            schema.version, schema.min_compatible, CURRENT_SCHEMA_VERSION
        )));
    }

    Ok(schema.version)
}

/// Parse a semver string into (major, minor, patch) for comparison.
fn parse_semver(version: &str) -> (u32, u32, u32) {
    let parts: Vec<u32> = version.split('.').filter_map(|p| p.parse().ok()).collect();
    (
        parts.first().copied().unwrap_or(0),
        parts.get(1).copied().unwrap_or(0),
        parts.get(2).copied().unwrap_or(0),
    )
}

/// JSON importer for world data.
pub struct JsonWorldImporter {
    data_dir: String,
}

impl JsonWorldImporter {
    /// Create a new importer.
    pub fn new(data_dir: impl Into<String>) -> Self {
        Self {
            data_dir: data_dir.into(),
        }
    }

    /// Load the complete world.
    /// Checks schema version before loading data.
    pub fn load_world(&self) -> Result<World, DataError> {
        // Validate schema version before loading
        check_schema_version(&self.data_dir)?;

        let mut world = World::new();

        // Load nations
        self.load_nations(&mut world)?;

        // Load clubs
        self.load_clubs(&mut world)?;

        // Load players
        self.load_players(&mut world)?;

        // Load competitions
        self.load_competitions(&mut world)?;

        Ok(world)
    }

    fn load_nations(&self, world: &mut World) -> Result<(), DataError> {
        let path = Path::new(&self.data_dir).join("nations.json");
        if !path.exists() {
            let nations = vec![
                ("BRA", "Brasil", "América do Sul"),
                ("ARG", "Argentina", "América do Sul"),
                ("ENG", "Inglaterra", "Europa"),
                ("ESP", "Espanha", "Europa"),
                ("GER", "Alemanha", "Europa"),
                ("ITA", "Itália", "Europa"),
                ("FRA", "França", "Europa"),
                ("POR", "Portugal", "Europa"),
            ];

            for (id, name, continent) in nations {
                let mut nation = Nation::new(id, name);
                nation.continent = continent.to_string();
                nation.reputation = 80;
                world.nations.insert(NationId::new(id), nation);
            }
            return Ok(());
        }

        let content = std::fs::read_to_string(&path)?;
        let raw: Vec<RawNation> = serde_json::from_str(&content)?;

        for n in raw {
            let nation = Nation {
                id: NationId::new(&n.id),
                name: n.name,
                short_name: n.short_name.unwrap_or_default(),
                continent: n.continent.unwrap_or_default(),
                reputation: n.reputation.unwrap_or(50),
                youth_rating: n.youth_rating.unwrap_or(50),
            };
            world.nations.insert(nation.id.clone(), nation);
        }

        Ok(())
    }

    fn load_clubs(&self, world: &mut World) -> Result<(), DataError> {
        let path = Path::new(&self.data_dir).join("clubs.json");
        if !path.exists() {
            let clubs = vec![
                ("FLA", "Flamengo", "BRA", 92, 100_000_000),
                ("PAL", "Palmeiras", "BRA", 90, 90_000_000),
                ("SAO", "São Paulo", "BRA", 88, 80_000_000),
                ("COR", "Corinthians", "BRA", 88, 80_000_000),
            ];

            for (id, name, nation, rep, budget) in clubs {
                let mut club = Club::new(id, name, NationId::new(nation));
                club.short_name = id.to_string();
                club.reputation = rep;
                club.budget = Budget::new(
                    Money::from_major(budget),
                    Money::from_major(budget / 2),
                    Money::from_major(500_000),
                );
                world.clubs.insert(ClubId::new(id), club);
            }
            return Ok(());
        }

        let content = std::fs::read_to_string(&path)?;
        let raw: Vec<RawClub> = serde_json::from_str(&content)?;

        for c in raw {
            let budget = Budget::new(
                Money::from_major(c.balance.unwrap_or(1_000_000)),
                Money::from_major(c.transfer_budget.unwrap_or(500_000)),
                Money::from_major(c.wage_budget.unwrap_or(100_000)),
            );

            let club = Club {
                id: ClubId::new(&c.id),
                name: c.name,
                short_name: c.short_name.unwrap_or_default(),
                nation_id: NationId::new(c.nation_id.unwrap_or_default()),
                stadium_id: c.stadium_id.map(StadiumId::new),
                reputation: c.reputation.unwrap_or(50),
                budget,
                board: Board::default(),
                tactics: Tactics::default(),
                player_ids: Vec::new(),
                staff_ids: Vec::new(),
                reserve_ids: Vec::new(),
                primary_color: c.primary_color.unwrap_or_else(|| "#FF0000".into()),
                secondary_color: c.secondary_color.unwrap_or_else(|| "#FFFFFF".into()),
                history: Default::default(),
            };
            world.clubs.insert(club.id.clone(), club);
        }

        Ok(())
    }

    fn load_players(&self, world: &mut World) -> Result<(), DataError> {
        let path = Path::new(&self.data_dir).join("players.json");
        if !path.exists() {
            // Generate players with real names for each club
            let mut rng = ChaCha8Rng::seed_from_u64(42);
            let mut player_id = 1;

            for club_id in world.clubs.keys().cloned().collect::<Vec<_>>() {
                let club_rep = world
                    .clubs
                    .get(&club_id)
                    .map(|c| c.reputation)
                    .unwrap_or(50);

                // Squad of 22 players with balanced positions
                let positions = vec![
                    Position::Goalkeeper,
                    Position::Goalkeeper,
                    Position::DefenderLeft,
                    Position::DefenderCenter,
                    Position::DefenderCenter,
                    Position::DefenderCenter,
                    Position::DefenderRight,
                    Position::MidfielderDefensive,
                    Position::MidfielderLeft,
                    Position::MidfielderCenter,
                    Position::MidfielderCenter,
                    Position::MidfielderRight,
                    Position::MidfielderAttacking,
                    Position::MidfielderAttacking,
                    Position::ForwardLeft,
                    Position::ForwardCenter,
                    Position::ForwardCenter,
                    Position::ForwardRight,
                    // Subs
                    Position::DefenderCenter,
                    Position::MidfielderCenter,
                    Position::ForwardCenter,
                    Position::Goalkeeper,
                ];

                for (i, pos) in positions.iter().enumerate() {
                    let id = format!("P{:04}", player_id);
                    let first = FIRST_NAMES[rng.gen_range(0..FIRST_NAMES.len())];
                    let last = LAST_NAMES[rng.gen_range(0..LAST_NAMES.len())];

                    // Age distribution: mostly 20-32
                    let age: i32 = if i < 18 {
                        rng.gen_range(19..32)
                    } else {
                        rng.gen_range(17..22) // younger subs
                    };
                    let birth_year = 2001 - age;
                    let birth_month = rng.gen_range(1..=12);
                    let birth_day = rng.gen_range(1..=28);

                    let mut player = Player::new(
                        &id,
                        first,
                        last,
                        NationId::new("BRA"),
                        NaiveDate::from_ymd_opt(birth_year, birth_month, birth_day).unwrap(),
                        *pos,
                    );
                    player.club_id = Some(club_id.clone());

                    // Attributes based on club reputation + randomness
                    let base = (club_rep as i32 - 20).max(20) as u8;
                    let var = 15u8;

                    player.attributes.technical.passing =
                        base.saturating_add(rng.gen_range(0..var)).min(95);
                    player.attributes.technical.finishing = base
                        .saturating_sub(5)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.technical.dribbling = base
                        .saturating_sub(3)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.technical.crossing = base
                        .saturating_sub(5)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.technical.tackling = base
                        .saturating_sub(5)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.technical.heading = base
                        .saturating_sub(5)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.technical.first_touch = base
                        .saturating_sub(3)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.technical.technique = base
                        .saturating_sub(3)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.technical.long_shots = base
                        .saturating_sub(8)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.technical.marking = base
                        .saturating_sub(5)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.technical.penalties = base
                        .saturating_sub(10)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.technical.free_kick = base
                        .saturating_sub(10)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);

                    player.attributes.mental.decisions =
                        base.saturating_add(rng.gen_range(0..var)).min(95);
                    player.attributes.mental.positioning =
                        base.saturating_add(rng.gen_range(0..var)).min(95);
                    player.attributes.mental.composure = base
                        .saturating_sub(3)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.mental.vision = base
                        .saturating_sub(5)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.mental.anticipation = base
                        .saturating_sub(3)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.mental.determination = base
                        .saturating_sub(3)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.mental.teamwork = base
                        .saturating_sub(3)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.mental.work_rate = base
                        .saturating_sub(3)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.mental.concentration = base
                        .saturating_sub(5)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.mental.leadership = base
                        .saturating_sub(10)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.mental.bravery = base
                        .saturating_sub(5)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.mental.aggression = base
                        .saturating_sub(10)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.mental.flair = base
                        .saturating_sub(8)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.mental.off_the_ball = base
                        .saturating_sub(5)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);

                    player.attributes.physical.pace =
                        base.saturating_add(rng.gen_range(0..var)).min(95);
                    player.attributes.physical.stamina =
                        base.saturating_add(rng.gen_range(0..var)).min(95);
                    player.attributes.physical.strength = base
                        .saturating_sub(5)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.physical.acceleration = base
                        .saturating_sub(3)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.physical.agility = base
                        .saturating_sub(3)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.physical.balance = base
                        .saturating_sub(5)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.physical.jumping = base
                        .saturating_sub(5)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);
                    player.attributes.physical.natural_fitness = base
                        .saturating_sub(3)
                        .saturating_add(rng.gen_range(0..var))
                        .min(95);

                    if *pos == Position::Goalkeeper {
                        player.attributes.goalkeeper.handling =
                            base.saturating_add(rng.gen_range(0..var)).min(95);
                        player.attributes.goalkeeper.reflexes =
                            base.saturating_add(rng.gen_range(0..var)).min(95);
                        player.attributes.goalkeeper.positioning =
                            base.saturating_add(rng.gen_range(0..var)).min(95);
                        player.attributes.goalkeeper.aerial_ability = base
                            .saturating_sub(5)
                            .saturating_add(rng.gen_range(0..var))
                            .min(95);
                        player.attributes.goalkeeper.command_of_area = base
                            .saturating_sub(5)
                            .saturating_add(rng.gen_range(0..var))
                            .min(95);
                        player.attributes.goalkeeper.communication = base
                            .saturating_sub(5)
                            .saturating_add(rng.gen_range(0..var))
                            .min(95);
                        player.attributes.goalkeeper.kicking = base
                            .saturating_sub(8)
                            .saturating_add(rng.gen_range(0..var))
                            .min(95);
                        player.attributes.goalkeeper.one_on_ones = base
                            .saturating_sub(3)
                            .saturating_add(rng.gen_range(0..var))
                            .min(95);
                        player.attributes.goalkeeper.throwing = base
                            .saturating_sub(5)
                            .saturating_add(rng.gen_range(0..var))
                            .min(95);
                    }

                    // Value based on reputation
                    let value_base = (club_rep as i64) * 50_000;
                    player.value = Money::from_major(value_base + rng.gen_range(0..value_base));

                    player.potential = (base + rng.gen_range(0..20)).min(99);
                    player.fitness = (80 + rng.gen_range(0..20)).min(100);
                    player.form = (40 + rng.gen_range(0..30)) as u8;

                    world.players.insert(PlayerId::new(&id), player);

                    if let Some(club) = world.clubs.get_mut(&club_id) {
                        club.player_ids.push(PlayerId::new(&id));
                    }

                    player_id += 1;
                }
            }
            return Ok(());
        }

        let content = std::fs::read_to_string(&path)?;
        let raw: Vec<RawPlayer> = serde_json::from_str(&content)?;

        for p in raw {
            let birth_date = NaiveDate::parse_from_str(&p.birth_date, "%Y-%m-%d")
                .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1990, 1, 1).unwrap());

            let position = parse_position(&p.position);

            let mut player = Player::new(
                &p.id,
                &p.first_name,
                &p.last_name,
                NationId::new(&p.nationality),
                birth_date,
                position,
            );

            if let Some(club) = &p.club_id {
                player.club_id = Some(ClubId::new(club));
                if let Some(c) = world.clubs.get_mut(&ClubId::new(club)) {
                    c.player_ids.push(PlayerId::new(&p.id));
                }
            }

            player.value = Money::from_major(p.value.unwrap_or(100_000));

            world.players.insert(PlayerId::new(&p.id), player);
        }

        Ok(())
    }

    fn load_competitions(&self, world: &mut World) -> Result<(), DataError> {
        let path = Path::new(&self.data_dir).join("competitions.json");
        if !path.exists() {
            let mut league = Competition::new("BRA1", "Serie A", CompetitionType::League);
            league.short_name = "Serie A".into();
            league.nation_id = Some(NationId::new("BRA"));
            league.reputation = 90;
            league.division_level = Some(DivisionLevel::SerieA);

            for (club_id, club) in &world.clubs {
                if club.nation_id.as_str() == "BRA" {
                    league.add_team(club_id.clone());
                }
            }

            world
                .competitions
                .insert(CompetitionId::new("BRA1"), league);
            return Ok(());
        }

        let content = std::fs::read_to_string(&path)?;
        let raw: Vec<RawCompetition> = serde_json::from_str(&content)?;

        for c in raw {
            let comp_type = match c.competition_type.as_deref() {
                Some("cup") => CompetitionType::Cup,
                Some("international") => CompetitionType::International,
                _ => CompetitionType::League,
            };

            let mut comp = Competition::new(&c.id, &c.name, comp_type);
            comp.short_name = c.short_name.unwrap_or_default();
            comp.nation_id = c.nation_id.as_ref().map(|s| NationId::new(s));
            comp.reputation = c.reputation.unwrap_or(50);

            // Parse division_level
            comp.division_level = c.division_level.and_then(|lvl| match lvl {
                1 => Some(DivisionLevel::SerieA),
                2 => Some(DivisionLevel::SerieB),
                3 => Some(DivisionLevel::SerieC),
                4 => Some(DivisionLevel::SerieD),
                _ => None,
            });

            for team_id in c.teams.unwrap_or_default() {
                comp.add_team(ClubId::new(&team_id));
            }

            world.competitions.insert(CompetitionId::new(&c.id), comp);
        }

        Ok(())
    }
}

fn parse_position(s: &str) -> Position {
    match s.to_uppercase().as_str() {
        "GK" => Position::Goalkeeper,
        "DC" | "CB" => Position::DefenderCenter,
        "DL" | "LB" => Position::DefenderLeft,
        "DR" | "RB" => Position::DefenderRight,
        "MC" | "CM" => Position::MidfielderCenter,
        "ML" | "LM" => Position::MidfielderLeft,
        "MR" | "RM" => Position::MidfielderRight,
        "DM" | "DMC" => Position::MidfielderDefensive,
        "AM" | "AMC" => Position::MidfielderAttacking,
        "FC" | "ST" | "CF" => Position::ForwardCenter,
        "FL" | "LW" => Position::ForwardLeft,
        "FR" | "RW" => Position::ForwardRight,
        _ => Position::MidfielderCenter,
    }
}

// Raw JSON structures for deserialization
#[derive(Deserialize)]
struct RawNation {
    id: String,
    name: String,
    short_name: Option<String>,
    continent: Option<String>,
    reputation: Option<u8>,
    youth_rating: Option<u8>,
}

#[derive(Deserialize)]
struct RawClub {
    id: String,
    name: String,
    short_name: Option<String>,
    nation_id: Option<String>,
    stadium_id: Option<String>,
    reputation: Option<u8>,
    balance: Option<i64>,
    transfer_budget: Option<i64>,
    wage_budget: Option<i64>,
    primary_color: Option<String>,
    secondary_color: Option<String>,
}

#[derive(Deserialize)]
struct RawPlayer {
    id: String,
    first_name: String,
    last_name: String,
    nationality: String,
    birth_date: String,
    position: String,
    club_id: Option<String>,
    value: Option<i64>,
}

#[derive(Deserialize)]
struct RawCompetition {
    id: String,
    name: String,
    short_name: Option<String>,
    nation_id: Option<String>,
    competition_type: Option<String>,
    reputation: Option<u8>,
    division_level: Option<u8>,
    teams: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_check_schema_version_no_file() {
        // Non-existent directory should return default version
        let result = check_schema_version("/nonexistent/path");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), CURRENT_SCHEMA_VERSION);
    }

    #[test]
    fn test_check_schema_version_valid() {
        let temp_dir = std::env::temp_dir().join("cm_test_schema_valid");
        std::fs::create_dir_all(&temp_dir).unwrap();

        let schema_path = temp_dir.join("schema_version.json");
        let mut file = std::fs::File::create(&schema_path).unwrap();
        write!(file, r#"{{"version": "1.0.0", "min_compatible": "1.0.0"}}"#).unwrap();

        let result = check_schema_version(temp_dir.to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1.0.0");

        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_check_schema_version_incompatible() {
        let temp_dir = std::env::temp_dir().join("cm_test_schema_incompat");
        std::fs::create_dir_all(&temp_dir).unwrap();

        let schema_path = temp_dir.join("schema_version.json");
        let mut file = std::fs::File::create(&schema_path).unwrap();
        // min_compatible is higher than current
        write!(
            file,
            r#"{{"version": "99.0.0", "min_compatible": "99.0.0"}}"#
        )
        .unwrap();

        let result = check_schema_version(temp_dir.to_str().unwrap());
        assert!(result.is_err());

        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_check_schema_version_invalid_json() {
        let temp_dir = std::env::temp_dir().join("cm_test_schema_invalid");
        std::fs::create_dir_all(&temp_dir).unwrap();

        let schema_path = temp_dir.join("schema_version.json");
        let mut file = std::fs::File::create(&schema_path).unwrap();
        write!(file, "not json at all").unwrap();

        let result = check_schema_version(temp_dir.to_str().unwrap());
        assert!(result.is_err());

        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_parse_semver() {
        assert_eq!(parse_semver("1.0.0"), (1, 0, 0));
        assert_eq!(parse_semver("2.3.4"), (2, 3, 4));
        assert_eq!(parse_semver("0.1.0"), (0, 1, 0));
        assert_eq!(parse_semver(""), (0, 0, 0));
    }

    #[test]
    fn test_parse_position_mapping() {
        assert_eq!(parse_position("GK"), Position::Goalkeeper);
        assert_eq!(parse_position("DC"), Position::DefenderCenter);
        assert_eq!(parse_position("CB"), Position::DefenderCenter);
        assert_eq!(parse_position("LB"), Position::DefenderLeft);
        assert_eq!(parse_position("RB"), Position::DefenderRight);
        assert_eq!(parse_position("CM"), Position::MidfielderCenter);
        assert_eq!(parse_position("DM"), Position::MidfielderDefensive);
        assert_eq!(parse_position("AM"), Position::MidfielderAttacking);
        assert_eq!(parse_position("ST"), Position::ForwardCenter);
        assert_eq!(parse_position("LW"), Position::ForwardLeft);
        assert_eq!(parse_position("RW"), Position::ForwardRight);
        // Unknown defaults to MidfielderCenter
        assert_eq!(parse_position("XYZ"), Position::MidfielderCenter);
    }
}
