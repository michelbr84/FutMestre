//! Training system - handles player development, fitness, aging, and youth generation.

use crate::config::GameConfig;
use crate::state::GameState;
use cm_core::ids::{ClubId, NationId, PlayerId};
use cm_core::world::player::PreferredFoot;
use cm_core::world::{Player, Position, StaffRole, TrainingFocus, World};
use rand::Rng;
use std::collections::HashMap;

/// Training intensity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrainingIntensity {
    Low,
    Medium,
    High,
}

impl TrainingIntensity {
    /// Get attribute gain per session.
    pub fn attribute_gain(&self) -> f32 {
        match self {
            Self::Low => 0.1,
            Self::Medium => 0.2,
            Self::High => 0.3,
        }
    }

    /// Get fitness cost per session.
    pub fn fitness_cost(&self) -> i8 {
        match self {
            Self::Low => -2,
            Self::Medium => -5,
            Self::High => -10,
        }
    }

    /// Get injury risk percentage (0-100).
    pub fn injury_risk_pct(&self) -> u8 {
        match self {
            Self::Low => 1,
            Self::Medium => 3,
            Self::High => 7,
        }
    }

    // --- Keep legacy compatibility methods ---

    /// Get fitness recovery/drain factor (legacy).
    pub fn fitness_factor(&self) -> i8 {
        match self {
            Self::Low => 5,
            Self::Medium => 0,
            Self::High => -3,
        }
    }

    /// Get attribute improvement chance multiplier (legacy).
    pub fn development_multiplier(&self) -> f32 {
        match self {
            Self::Low => 0.5,
            Self::Medium => 1.0,
            Self::High => 1.5,
        }
    }

    /// Get injury risk multiplier (legacy).
    pub fn injury_risk(&self) -> f32 {
        match self {
            Self::Low => 0.25,
            Self::Medium => 1.0,
            Self::High => 2.0,
        }
    }
}

/// Age-based modifier for training.
struct AgeModifier {
    /// Multiplier applied to attribute gains (1.0 = normal).
    attribute_gain_mult: f32,
    /// Additional injury risk percentage added.
    extra_injury_risk: u8,
    /// Whether the player may experience random attribute decline.
    may_decline: bool,
}

fn age_modifier(age: u8) -> AgeModifier {
    if age < 21 {
        AgeModifier {
            attribute_gain_mult: 1.5, // +50% development bonus
            extra_injury_risk: 0,
            may_decline: false,
        }
    } else if age <= 29 {
        AgeModifier {
            attribute_gain_mult: 1.0,
            extra_injury_risk: 0,
            may_decline: false,
        }
    } else if age <= 32 {
        AgeModifier {
            attribute_gain_mult: 0.5, // -50% attribute gain
            extra_injury_risk: 50,    // +50% injury risk (applied as additive pct points scaled)
            may_decline: false,
        }
    } else {
        // 33+
        AgeModifier {
            attribute_gain_mult: 0.5,
            extra_injury_risk: 50,
            may_decline: true,
        }
    }
}

/// Apply a fractional increment to a u8 attribute, clamping to [1, 99].
/// Uses stochastic rounding: the fractional part is treated as a probability of +1.
fn apply_attr_gain(attr: &mut u8, gain: f32, rng: &mut impl Rng) {
    let whole = gain as u8;
    let frac = gain - whole as f32;
    let extra = if rng.gen::<f32>() < frac { 1u8 } else { 0u8 };
    *attr = (*attr)
        .saturating_add(whole)
        .saturating_add(extra)
        .min(99)
        .max(1);
}

/// Apply a fractional decrement to a u8 attribute, clamping to [1, 99].
fn apply_attr_decline(attr: &mut u8, loss: f32, rng: &mut impl Rng) {
    let whole = loss as u8;
    let frac = loss - whole as f32;
    let extra = if rng.gen::<f32>() < frac { 1u8 } else { 0u8 };
    *attr = (*attr).saturating_sub(whole).saturating_sub(extra).max(1);
}

/// Training system.
pub struct TrainingSystem;

impl TrainingSystem {
    /// Run daily training updates.
    pub fn run_daily(&self, _cfg: &GameConfig, world: &mut World, _state: &mut GameState) {
        // Process training for all players with default settings
        for player in world.players.values_mut() {
            if player.is_injured() {
                continue;
            }

            let intensity = TrainingIntensity::Medium;
            let fitness_change = intensity.fitness_factor();
            player.fitness = (player.fitness as i16 + fitness_change as i16).clamp(0, 100) as u8;
        }
    }

    /// Apply focused training for a club.
    pub fn apply_club_training(
        &self,
        world: &mut World,
        club_id: &ClubId,
        _focus: TrainingFocus,
        intensity: TrainingIntensity,
    ) {
        let player_ids: Vec<_> = world
            .players
            .values()
            .filter(|p| p.club_id.as_ref() == Some(club_id) && !p.is_injured())
            .map(|p| p.id.clone())
            .collect();

        for player_id in player_ids {
            if let Some(player) = world.players.get_mut(&player_id) {
                let fitness_change = intensity.fitness_factor();
                player.fitness =
                    (player.fitness as i16 + fitness_change as i16).clamp(0, 100) as u8;
            }
        }
    }

    /// Rest players (light training/recovery).
    pub fn rest_squad(&self, world: &mut World, club_id: &ClubId) {
        self.apply_club_training(
            world,
            club_id,
            TrainingFocus::Fitness,
            TrainingIntensity::Low,
        );
    }
}

// ---------------------------------------------------------------------------
// Staff bonus calculation
// ---------------------------------------------------------------------------

/// Calculate the coaching staff bonus for a club.
///
/// Averages the `coaching` attribute of all Coach, FitnessCoach, and GoalkeeperCoach
/// staff assigned to the club. Returns a value in the range [0.0, 0.2] that acts
/// as a multiplier bonus for training gains.
pub fn calculate_staff_bonus(world: &World, club_id: &ClubId) -> f32 {
    let mut total_coaching: u32 = 0;
    let mut count: u32 = 0;

    for staff in world.staff.values() {
        if staff.club_id.as_ref() != Some(club_id) {
            continue;
        }
        match staff.role {
            StaffRole::Coach | StaffRole::FitnessCoach | StaffRole::GoalkeeperCoach => {
                total_coaching += staff.coaching as u32;
                count += 1;
            }
            _ => {}
        }
    }

    if count == 0 {
        return 0.0;
    }

    let avg = total_coaching as f32 / count as f32;
    // Scale to [0.0, 0.2] range: coaching 1-20 maps to 0.01-0.20
    (avg / 100.0).clamp(0.0, 0.2)
}

/// Calculate the goalkeeper coach bonus for a club.
///
/// Returns an additional bonus (0.0-0.2) from GoalkeeperCoach staff specifically,
/// applied on top of the general staff bonus for goalkeeper training.
pub fn calculate_gk_coach_bonus(world: &World, club_id: &ClubId) -> f32 {
    let mut total: u32 = 0;
    let mut count: u32 = 0;

    for staff in world.staff.values() {
        if staff.club_id.as_ref() != Some(club_id) {
            continue;
        }
        if staff.role == StaffRole::GoalkeeperCoach {
            total += staff.coaching as u32;
            count += 1;
        }
    }

    if count == 0 {
        return 0.0;
    }

    let avg = total as f32 / count as f32;
    (avg / 100.0).clamp(0.0, 0.2)
}

/// Pre-compute staff bonuses for all clubs in the world.
fn compute_all_staff_bonuses(world: &World) -> (HashMap<ClubId, f32>, HashMap<ClubId, f32>) {
    let mut general_bonus: HashMap<ClubId, f32> = HashMap::new();
    let mut gk_bonus: HashMap<ClubId, f32> = HashMap::new();

    for club_id in world.clubs.keys() {
        general_bonus.insert(club_id.clone(), calculate_staff_bonus(world, club_id));
        gk_bonus.insert(club_id.clone(), calculate_gk_coach_bonus(world, club_id));
    }

    (general_bonus, gk_bonus)
}

// ---------------------------------------------------------------------------
// Public free functions for the enhanced training system
// ---------------------------------------------------------------------------

/// Process a training session for all non-injured players in the world.
///
/// Applies attribute gains based on `training_focus` and `intensity`,
/// modified by each player's age. Also applies fitness cost and
/// stochastic injury risk.
pub fn process_training(
    world: &mut World,
    training_focus: TrainingFocus,
    intensity: TrainingIntensity,
) {
    let mut rng = rand::thread_rng();
    process_training_with_rng(world, training_focus, intensity, &mut rng);
}

/// Testable version accepting an explicit RNG.
pub fn process_training_with_rng(
    world: &mut World,
    training_focus: TrainingFocus,
    intensity: TrainingIntensity,
    rng: &mut impl Rng,
) {
    let today = chrono::Utc::now().date_naive();

    // Pre-compute staff bonuses for all clubs
    let (staff_bonuses, gk_bonuses) = compute_all_staff_bonuses(world);

    let player_ids: Vec<PlayerId> = world.players.keys().cloned().collect();

    for pid in player_ids {
        let player = match world.players.get_mut(&pid) {
            Some(p) => p,
            None => continue,
        };

        // Skip injured players
        if player.is_injured() {
            continue;
        }

        let age = player.age_on(today);
        let age_mod = age_modifier(age);

        // --- Recovery focus ---
        if training_focus == TrainingFocus::Recovery {
            // Restore fitness (+5..+15 based on intensity)
            let fitness_restore: u8 = match intensity {
                TrainingIntensity::Low => 5,
                TrainingIntensity::Medium => 10,
                TrainingIntensity::High => 15,
            };
            player.fitness = (player.fitness as u16 + fitness_restore as u16).min(100) as u8;
            // No attribute gains, no injury risk for recovery
            continue;
        }

        // --- Fitness cost ---
        let cost = intensity.fitness_cost(); // negative value
        player.fitness = (player.fitness as i16 + cost as i16).clamp(0, 100) as u8;

        // --- Injury risk ---
        let base_risk = intensity.injury_risk_pct() as u16;
        // Age 30+ adds 50% more risk: e.g. 7% -> 10%
        let adjusted_risk = if age_mod.extra_injury_risk > 0 {
            base_risk + (base_risk * age_mod.extra_injury_risk as u16) / 100
        } else {
            base_risk
        };
        let roll: u8 = rng.gen_range(1..=100);
        if roll <= adjusted_risk as u8 {
            // Player got injured during training -- mark with a minor injury
            let injury = cm_core::world::Injury::new(
                cm_core::world::InjuryType::Hamstring,
                today,
                rng.gen_range(3..=14),
            );
            player.injury = Some(injury);
            player.fitness = player.fitness.saturating_sub(10);
            continue; // No attribute gains if injured this session
        }

        // --- Staff coaching bonus ---
        let coaching_bonus = player
            .club_id
            .as_ref()
            .and_then(|cid| staff_bonuses.get(cid))
            .copied()
            .unwrap_or(0.0);

        let is_goalkeeper = player.position == Position::Goalkeeper;
        let gk_bonus = if is_goalkeeper {
            player
                .club_id
                .as_ref()
                .and_then(|cid| gk_bonuses.get(cid))
                .copied()
                .unwrap_or(0.0)
        } else {
            0.0
        };

        // --- Attribute gains ---
        let base_gain = intensity.attribute_gain();
        // training_gain = base_gain * age_modifier * (1.0 + coaching_bonus)
        let gain = base_gain * age_mod.attribute_gain_mult * (1.0 + coaching_bonus);
        // Goalkeepers get an additional boost from GK coaches
        let gk_gain = base_gain * age_mod.attribute_gain_mult * (1.0 + coaching_bonus + gk_bonus);

        match training_focus {
            TrainingFocus::Physical => {
                apply_attr_gain(&mut player.attributes.physical.pace, gain, rng);
                apply_attr_gain(&mut player.attributes.physical.stamina, gain, rng);
                apply_attr_gain(&mut player.attributes.physical.strength, gain, rng);
                apply_attr_gain(&mut player.attributes.physical.acceleration, gain, rng);
                apply_attr_gain(&mut player.attributes.physical.agility, gain, rng);
            }
            TrainingFocus::Technical => {
                apply_attr_gain(&mut player.attributes.technical.finishing, gain, rng);
                apply_attr_gain(&mut player.attributes.technical.passing, gain, rng);
                apply_attr_gain(&mut player.attributes.technical.dribbling, gain, rng);
                apply_attr_gain(&mut player.attributes.technical.crossing, gain, rng);
                apply_attr_gain(&mut player.attributes.technical.tackling, gain, rng);
            }
            TrainingFocus::Tactical => {
                apply_attr_gain(&mut player.attributes.mental.positioning, gain, rng);
                apply_attr_gain(&mut player.attributes.mental.decisions, gain, rng);
                apply_attr_gain(&mut player.attributes.mental.anticipation, gain, rng);
                apply_attr_gain(&mut player.attributes.mental.vision, gain, rng);
                apply_attr_gain(&mut player.attributes.mental.composure, gain, rng);
            }
            // Other focus types don't have specific attribute mapping yet;
            // treat them like General (small all-round gain).
            _ => {
                let small = gain * 0.5;
                apply_attr_gain(&mut player.attributes.physical.stamina, small, rng);
                apply_attr_gain(&mut player.attributes.technical.passing, small, rng);
                apply_attr_gain(&mut player.attributes.mental.decisions, small, rng);
            }
        }

        // Goalkeeper-specific training: GK coaches boost goalkeeper attributes
        if is_goalkeeper && gk_bonus > 0.0 {
            let gk_small = gk_gain * 0.3; // small bonus to GK attributes each session
            apply_attr_gain(&mut player.attributes.goalkeeper.handling, gk_small, rng);
            apply_attr_gain(&mut player.attributes.goalkeeper.reflexes, gk_small, rng);
            apply_attr_gain(&mut player.attributes.goalkeeper.positioning, gk_small, rng);
        }

        // --- 33+ random decline ---
        if age_mod.may_decline {
            // ~14% chance per session (roughly once per week if training daily)
            if rng.gen::<f32>() < 0.14 {
                // Pick a random physical attribute to decline
                let attr_idx = rng.gen_range(0..5u8);
                let decline = 0.1_f32;
                match attr_idx {
                    0 => apply_attr_decline(&mut player.attributes.physical.pace, decline, rng),
                    1 => apply_attr_decline(&mut player.attributes.physical.stamina, decline, rng),
                    2 => apply_attr_decline(&mut player.attributes.physical.strength, decline, rng),
                    3 => apply_attr_decline(
                        &mut player.attributes.physical.acceleration,
                        decline,
                        rng,
                    ),
                    _ => apply_attr_decline(&mut player.attributes.physical.agility, decline, rng),
                }
            }
        }
    }
}

/// Process aging at season end.
///
/// Increments each player's conceptual age (birth_date stays the same),
/// and applies veteran decline for players 30+.
pub fn process_aging(world: &mut World) {
    let mut rng = rand::thread_rng();
    process_aging_with_rng(world, &mut rng);
}

/// Testable version accepting an explicit RNG.
pub fn process_aging_with_rng(world: &mut World, rng: &mut impl Rng) {
    let today = chrono::Utc::now().date_naive();

    for player in world.players.values_mut() {
        let age = player.age_on(today);

        if age >= 33 {
            // Veteran decline: random -0.1 on several physical attributes
            for _ in 0..3 {
                let idx = rng.gen_range(0..5u8);
                match idx {
                    0 => apply_attr_decline(&mut player.attributes.physical.pace, 0.1, rng),
                    1 => apply_attr_decline(&mut player.attributes.physical.stamina, 0.1, rng),
                    2 => apply_attr_decline(&mut player.attributes.physical.strength, 0.1, rng),
                    3 => apply_attr_decline(&mut player.attributes.physical.acceleration, 0.1, rng),
                    _ => apply_attr_decline(&mut player.attributes.physical.agility, 0.1, rng),
                }
            }

            // Potential drops with age
            if player.potential > 1 {
                player.potential = player.potential.saturating_sub(1);
            }
        } else if age >= 30 {
            // Mild decline
            let idx = rng.gen_range(0..5u8);
            match idx {
                0 => apply_attr_decline(&mut player.attributes.physical.pace, 0.1, rng),
                1 => apply_attr_decline(&mut player.attributes.physical.stamina, 0.1, rng),
                2 => apply_attr_decline(&mut player.attributes.physical.strength, 0.1, rng),
                3 => apply_attr_decline(&mut player.attributes.physical.acceleration, 0.1, rng),
                _ => apply_attr_decline(&mut player.attributes.physical.agility, 0.1, rng),
            }
        }
    }
}

/// Generate a random youth player aged 16-19 with random attributes and high potential.
pub fn generate_youth_player(rng: &mut impl Rng) -> Player {
    let age: u8 = rng.gen_range(16..=19);
    let birth_year = chrono::Utc::now().date_naive().year_ce().1 as i32 - age as i32;
    let birth_month: u32 = rng.gen_range(1..=12);
    let birth_day: u32 = rng.gen_range(1..=28);
    let birth_date = chrono::NaiveDate::from_ymd_opt(birth_year, birth_month, birth_day)
        .unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(birth_year, 1, 1).unwrap());

    let id_num: u32 = rng.gen_range(100_000..999_999);
    let player_id = format!("YTH{id_num}");

    let first_names = [
        "Lucas", "Gabriel", "Matheus", "Pedro", "Rafael", "Bruno", "Felipe", "Thiago", "Vinicius",
        "Arthur", "Carlos", "Diego", "Eduardo", "Gustavo", "Henrique",
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
        NationId::new("BRA"),
        birth_date,
        position,
    );

    // Random attributes in the 20-55 range for youth players
    fn rand_attr(rng: &mut impl Rng) -> u8 {
        rng.gen_range(20..=55)
    }

    // Technical
    player.attributes.technical.crossing = rand_attr(rng);
    player.attributes.technical.dribbling = rand_attr(rng);
    player.attributes.technical.finishing = rand_attr(rng);
    player.attributes.technical.first_touch = rand_attr(rng);
    player.attributes.technical.free_kick = rand_attr(rng);
    player.attributes.technical.heading = rand_attr(rng);
    player.attributes.technical.long_shots = rand_attr(rng);
    player.attributes.technical.marking = rand_attr(rng);
    player.attributes.technical.passing = rand_attr(rng);
    player.attributes.technical.penalties = rand_attr(rng);
    player.attributes.technical.tackling = rand_attr(rng);
    player.attributes.technical.technique = rand_attr(rng);

    // Mental
    player.attributes.mental.aggression = rand_attr(rng);
    player.attributes.mental.anticipation = rand_attr(rng);
    player.attributes.mental.bravery = rand_attr(rng);
    player.attributes.mental.composure = rand_attr(rng);
    player.attributes.mental.concentration = rand_attr(rng);
    player.attributes.mental.decisions = rand_attr(rng);
    player.attributes.mental.determination = rand_attr(rng);
    player.attributes.mental.flair = rand_attr(rng);
    player.attributes.mental.leadership = rand_attr(rng);
    player.attributes.mental.off_the_ball = rand_attr(rng);
    player.attributes.mental.positioning = rand_attr(rng);
    player.attributes.mental.teamwork = rand_attr(rng);
    player.attributes.mental.vision = rand_attr(rng);
    player.attributes.mental.work_rate = rand_attr(rng);

    // Physical
    player.attributes.physical.acceleration = rand_attr(rng);
    player.attributes.physical.agility = rand_attr(rng);
    player.attributes.physical.balance = rand_attr(rng);
    player.attributes.physical.jumping = rand_attr(rng);
    player.attributes.physical.natural_fitness = rand_attr(rng);
    player.attributes.physical.pace = rand_attr(rng);
    player.attributes.physical.stamina = rand_attr(rng);
    player.attributes.physical.strength = rand_attr(rng);

    // Goalkeeper (low unless GK)
    if position == Position::Goalkeeper {
        player.attributes.goalkeeper.aerial_ability = rand_attr(rng);
        player.attributes.goalkeeper.command_of_area = rand_attr(rng);
        player.attributes.goalkeeper.communication = rand_attr(rng);
        player.attributes.goalkeeper.handling = rand_attr(rng);
        player.attributes.goalkeeper.kicking = rand_attr(rng);
        player.attributes.goalkeeper.one_on_ones = rand_attr(rng);
        player.attributes.goalkeeper.positioning = rand_attr(rng);
        player.attributes.goalkeeper.reflexes = rand_attr(rng);
        player.attributes.goalkeeper.throwing = rand_attr(rng);
    }

    // High potential for youth players (60-90)
    player.potential = rng.gen_range(60..=90);
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

use chrono::Datelike;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use cm_core::ids::NationId;
    use cm_core::world::Player;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn setup_test() -> (World, GameState, TrainingSystem) {
        let mut world = World::new();

        let mut player = Player::new(
            "P001",
            "Test",
            "Player",
            NationId::new("ENG"),
            NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            Position::MidfielderCenter,
        );
        player.fitness = 80;
        player.club_id = Some(ClubId::new("LIV"));
        world.players.insert(player.id.clone(), player);

        let state = GameState::default();
        let system = TrainingSystem;

        (world, state, system)
    }

    fn make_seeded_rng() -> StdRng {
        StdRng::seed_from_u64(42)
    }

    // --- Legacy tests ---

    #[test]
    fn test_intensity_fitness_factors() {
        assert_eq!(TrainingIntensity::Low.fitness_factor(), 5);
        assert_eq!(TrainingIntensity::Medium.fitness_factor(), 0);
        assert!(TrainingIntensity::High.fitness_factor() < 0);
    }

    #[test]
    fn test_intensity_development() {
        assert!(
            TrainingIntensity::High.development_multiplier()
                > TrainingIntensity::Medium.development_multiplier()
        );
    }

    #[test]
    fn test_intensity_injury_risk() {
        assert!(TrainingIntensity::High.injury_risk() > TrainingIntensity::Low.injury_risk());
    }

    #[test]
    fn test_daily_training() {
        let (mut world, mut state, system) = setup_test();
        let config = GameConfig::default();

        let _initial_fitness = world.players.get(&PlayerId::new("P001")).unwrap().fitness;
        system.run_daily(&config, &mut world, &mut state);

        let player = world.players.get(&PlayerId::new("P001")).unwrap();
        assert!(player.fitness <= 100);
    }

    #[test]
    fn test_rest_squad() {
        let (mut world, _, system) = setup_test();
        let club_id = ClubId::new("LIV");

        if let Some(player) = world.players.get_mut(&PlayerId::new("P001")) {
            player.fitness = 70;
        }

        system.rest_squad(&mut world, &club_id);

        let player = world.players.get(&PlayerId::new("P001")).unwrap();
        assert!(player.fitness >= 70);
    }

    // --- New training system tests ---

    #[test]
    fn test_process_training_physical_gains() {
        let mut world = World::new();
        let mut player = Player::new(
            "P100",
            "Young",
            "Star",
            NationId::new("BRA"),
            NaiveDate::from_ymd_opt(2008, 6, 15).unwrap(), // ~17-18 years old
            Position::ForwardCenter,
        );
        player.fitness = 90;
        player.attributes.physical.pace = 40;
        player.attributes.physical.stamina = 40;
        player.attributes.physical.strength = 40;
        player.attributes.physical.acceleration = 40;
        player.attributes.physical.agility = 40;
        world.players.insert(player.id.clone(), player);

        let mut rng = make_seeded_rng();

        // Run many sessions to ensure measurable gain
        for _ in 0..50 {
            process_training_with_rng(
                &mut world,
                TrainingFocus::Physical,
                TrainingIntensity::High,
                &mut rng,
            );
            // Restore fitness so they keep training
            if let Some(p) = world.players.get_mut(&PlayerId::new("P100")) {
                p.fitness = 90;
                p.injury = None; // clear any training injury for test purposes
            }
        }

        let p = world.players.get(&PlayerId::new("P100")).unwrap();
        // After 50 high-intensity sessions with youth bonus, attributes should have improved
        assert!(
            p.attributes.physical.pace > 40,
            "pace should improve, got {}",
            p.attributes.physical.pace
        );
        assert!(p.attributes.physical.stamina > 40, "stamina should improve");
    }

    #[test]
    fn test_process_training_technical_gains() {
        let mut world = World::new();
        let mut player = Player::new(
            "P200",
            "Tech",
            "Wizard",
            NationId::new("ESP"),
            NaiveDate::from_ymd_opt(2001, 3, 10).unwrap(),
            Position::MidfielderAttacking,
        );
        player.fitness = 90;
        player.attributes.technical.passing = 50;
        player.attributes.technical.dribbling = 50;
        world.players.insert(player.id.clone(), player);

        let mut rng = make_seeded_rng();

        for _ in 0..30 {
            process_training_with_rng(
                &mut world,
                TrainingFocus::Technical,
                TrainingIntensity::Medium,
                &mut rng,
            );
            if let Some(p) = world.players.get_mut(&PlayerId::new("P200")) {
                p.fitness = 90;
                p.injury = None;
            }
        }

        let p = world.players.get(&PlayerId::new("P200")).unwrap();
        assert!(
            p.attributes.technical.passing > 50,
            "passing should improve"
        );
        assert!(
            p.attributes.technical.dribbling > 50,
            "dribbling should improve"
        );
    }

    #[test]
    fn test_process_training_tactical_gains() {
        let mut world = World::new();
        let mut player = Player::new(
            "P300",
            "Tact",
            "Mind",
            NationId::new("GER"),
            NaiveDate::from_ymd_opt(1999, 7, 20).unwrap(),
            Position::MidfielderDefensive,
        );
        player.fitness = 90;
        player.attributes.mental.positioning = 50;
        player.attributes.mental.decisions = 50;
        player.attributes.mental.vision = 50;
        world.players.insert(player.id.clone(), player);

        let mut rng = make_seeded_rng();

        for _ in 0..30 {
            process_training_with_rng(
                &mut world,
                TrainingFocus::Tactical,
                TrainingIntensity::Medium,
                &mut rng,
            );
            if let Some(p) = world.players.get_mut(&PlayerId::new("P300")) {
                p.fitness = 90;
                p.injury = None;
            }
        }

        let p = world.players.get(&PlayerId::new("P300")).unwrap();
        assert!(
            p.attributes.mental.positioning > 50,
            "positioning should improve"
        );
        assert!(
            p.attributes.mental.decisions > 50,
            "decisions should improve"
        );
    }

    #[test]
    fn test_recovery_restores_fitness() {
        let mut world = World::new();
        let mut player = Player::new(
            "P400",
            "Tired",
            "Runner",
            NationId::new("KEN"),
            NaiveDate::from_ymd_opt(1997, 2, 5).unwrap(),
            Position::MidfielderCenter,
        );
        player.fitness = 50;
        player.attributes.physical.pace = 60;
        world.players.insert(player.id.clone(), player);

        let mut rng = make_seeded_rng();
        process_training_with_rng(
            &mut world,
            TrainingFocus::Recovery,
            TrainingIntensity::High,
            &mut rng,
        );

        let p = world.players.get(&PlayerId::new("P400")).unwrap();
        assert!(
            p.fitness > 50,
            "fitness should be restored, got {}",
            p.fitness
        );
        // Attributes should not change
        assert_eq!(p.attributes.physical.pace, 60);
    }

    #[test]
    fn test_fitness_cost_applied() {
        let mut world = World::new();
        let mut player = Player::new(
            "P500",
            "Fit",
            "Player",
            NationId::new("ENG"),
            NaiveDate::from_ymd_opt(1998, 8, 12).unwrap(),
            Position::DefenderCenter,
        );
        player.fitness = 80;
        world.players.insert(player.id.clone(), player);

        // Use a deterministic RNG that won't cause injury
        let mut rng = StdRng::seed_from_u64(999);
        process_training_with_rng(
            &mut world,
            TrainingFocus::Physical,
            TrainingIntensity::High,
            &mut rng,
        );

        let p = world.players.get(&PlayerId::new("P500")).unwrap();
        // Fitness should decrease (unless player got injured which also decreases it)
        assert!(
            p.fitness < 80,
            "fitness should decrease from training, got {}",
            p.fitness
        );
    }

    #[test]
    fn test_age_modifier_young() {
        let m = age_modifier(18);
        assert!((m.attribute_gain_mult - 1.5).abs() < f32::EPSILON);
        assert_eq!(m.extra_injury_risk, 0);
        assert!(!m.may_decline);
    }

    #[test]
    fn test_age_modifier_prime() {
        let m = age_modifier(25);
        assert!((m.attribute_gain_mult - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_age_modifier_veteran() {
        let m = age_modifier(34);
        assert!((m.attribute_gain_mult - 0.5).abs() < f32::EPSILON);
        assert!(m.may_decline);
    }

    #[test]
    fn test_process_aging_veteran_decline() {
        let mut world = World::new();
        let mut player = Player::new(
            "P600",
            "Old",
            "Legend",
            NationId::new("ITA"),
            NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(), // ~36 years old
            Position::DefenderCenter,
        );
        player.attributes.physical.pace = 70;
        player.attributes.physical.stamina = 70;
        player.attributes.physical.strength = 70;
        player.attributes.physical.acceleration = 70;
        player.attributes.physical.agility = 70;
        player.potential = 80;
        world.players.insert(player.id.clone(), player);

        let mut rng = make_seeded_rng();

        // Run aging multiple times to observe decline
        for _ in 0..5 {
            process_aging_with_rng(&mut world, &mut rng);
        }

        let p = world.players.get(&PlayerId::new("P600")).unwrap();
        let total_phys = p.attributes.physical.pace as u32
            + p.attributes.physical.stamina as u32
            + p.attributes.physical.strength as u32
            + p.attributes.physical.acceleration as u32
            + p.attributes.physical.agility as u32;
        // Should have declined from 350 total
        assert!(
            total_phys < 350,
            "physical attributes should decline with aging, total={total_phys}"
        );
        assert!(p.potential < 80, "potential should decrease");
    }

    #[test]
    fn test_generate_youth_player() {
        let mut rng = make_seeded_rng();
        let player = generate_youth_player(&mut rng);

        let today = chrono::Utc::now().date_naive();
        let age = player.age_on(today);

        // Age can be 15 if birthday hasn't passed yet this year
        assert!(age >= 15 && age <= 19, "youth should be 15-19, got {age}");
        assert!(
            player.potential >= 60,
            "youth should have high potential, got {}",
            player.potential
        );
        assert!(
            player.attributes.technical.passing >= 20,
            "attributes should be initialized"
        );
        assert!(
            player.attributes.physical.pace >= 20,
            "physical attrs should be initialized"
        );
        assert!(!player.first_name.is_empty());
        assert!(!player.last_name.is_empty());
    }

    #[test]
    fn test_generate_youth_player_variety() {
        let mut rng = make_seeded_rng();
        let p1 = generate_youth_player(&mut rng);
        let p2 = generate_youth_player(&mut rng);

        // Two generated players should have different IDs
        assert_ne!(p1.id, p2.id);
    }

    #[test]
    fn test_injured_players_skip_training() {
        let mut world = World::new();
        let mut player = Player::new(
            "P700",
            "Injured",
            "Player",
            NationId::new("FRA"),
            NaiveDate::from_ymd_opt(2000, 5, 5).unwrap(),
            Position::ForwardCenter,
        );
        player.fitness = 80;
        player.attributes.technical.finishing = 50;
        let injury = cm_core::world::Injury::new(
            cm_core::world::InjuryType::Hamstring,
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            30,
        );
        player.injury = Some(injury);
        world.players.insert(player.id.clone(), player);

        let mut rng = make_seeded_rng();
        process_training_with_rng(
            &mut world,
            TrainingFocus::Technical,
            TrainingIntensity::High,
            &mut rng,
        );

        let p = world.players.get(&PlayerId::new("P700")).unwrap();
        assert_eq!(
            p.attributes.technical.finishing, 50,
            "injured player should not improve"
        );
        assert_eq!(p.fitness, 80, "injured player fitness should not change");
    }
}
