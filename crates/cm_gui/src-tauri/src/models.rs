use chrono::NaiveDate;
use cm_core::economy::Money;
use cm_core::world::attributes::Attributes;
use cm_core::world::player::Player;
use serde::{Deserialize, Serialize};

// ─── Player Display ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayPlayer {
    pub id: String,
    pub name: String,
    pub position: String,
    pub age: u8,
    pub nationality: String,
    pub overall: u8,
    pub value: String,
    pub wage: String,
    pub condition: u8,
    pub morale: String,
    pub club_name: String,
}

impl DisplayPlayer {
    pub fn from_player(p: &Player, date: NaiveDate, nation_name: &str, club_name: &str) -> Self {
        Self {
            id: p.id.to_string(),
            name: p.full_name(),
            position: p.position.short_name().to_string(),
            age: p.age_on(date),
            nationality: nation_name.to_string(),
            overall: p.overall_rating(),
            value: format_money(p.value),
            wage: format!("{}/w", format_money(p.weekly_wage())),
            condition: p.fitness,
            morale: format!("{:?}", p.morale.level()),
            club_name: club_name.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayPlayerProfile {
    pub display: DisplayPlayer,
    pub attributes: DisplayAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayAttributes {
    pub technical: Vec<(String, u8)>,
    pub mental: Vec<(String, u8)>,
    pub physical: Vec<(String, u8)>,
}

impl From<&Attributes> for DisplayAttributes {
    fn from(a: &Attributes) -> Self {
        Self {
            technical: vec![
                ("Crossing".into(), a.technical.crossing),
                ("Dribbling".into(), a.technical.dribbling),
                ("Finishing".into(), a.technical.finishing),
                ("First Touch".into(), a.technical.first_touch),
                ("Free Kick".into(), a.technical.free_kick),
                ("Heading".into(), a.technical.heading),
                ("Long Shots".into(), a.technical.long_shots),
                ("Marking".into(), a.technical.marking),
                ("Passing".into(), a.technical.passing),
                ("Penalties".into(), a.technical.penalties),
                ("Tackling".into(), a.technical.tackling),
                ("Technique".into(), a.technical.technique),
            ],
            mental: vec![
                ("Aggression".into(), a.mental.aggression),
                ("Anticipation".into(), a.mental.anticipation),
                ("Bravery".into(), a.mental.bravery),
                ("Composure".into(), a.mental.composure),
                ("Concentration".into(), a.mental.concentration),
                ("Decisions".into(), a.mental.decisions),
                ("Determination".into(), a.mental.determination),
                ("Flair".into(), a.mental.flair),
                ("Leadership".into(), a.mental.leadership),
                ("Off The Ball".into(), a.mental.off_the_ball),
                ("Positioning".into(), a.mental.positioning),
                ("Teamwork".into(), a.mental.teamwork),
                ("Vision".into(), a.mental.vision),
                ("Work Rate".into(), a.mental.work_rate),
            ],
            physical: vec![
                ("Acceleration".into(), a.physical.acceleration),
                ("Agility".into(), a.physical.agility),
                ("Balance".into(), a.physical.balance),
                ("Jumping".into(), a.physical.jumping),
                ("Natural Fitness".into(), a.physical.natural_fitness),
                ("Pace".into(), a.physical.pace),
                ("Stamina".into(), a.physical.stamina),
                ("Strength".into(), a.physical.strength),
            ],
        }
    }
}

// ─── Match Display ──────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayMatchResult {
    pub home_goals: u8,
    pub away_goals: u8,
    pub home_name: String,
    pub away_name: String,
    pub highlights: Vec<String>,
    pub events: Vec<DisplayMatchEvent>,
    pub stats: DisplayMatchStats,
    pub player_ratings: Vec<DisplayPlayerRating>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayMatchEvent {
    pub minute: u32,
    pub event_type: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayMatchStats {
    pub home_possession: f64,
    pub away_possession: f64,
    pub home_shots: u32,
    pub away_shots: u32,
    pub home_shots_on_target: u32,
    pub away_shots_on_target: u32,
    pub home_fouls: u32,
    pub away_fouls: u32,
    pub home_corners: u32,
    pub away_corners: u32,
    pub home_yellow_cards: u32,
    pub away_yellow_cards: u32,
    pub home_red_cards: u32,
    pub away_red_cards: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayPlayerRating {
    pub player_id: String,
    pub team: String,
    pub rating: f32,
    pub goals: u8,
    pub assists: u8,
    pub man_of_the_match: bool,
}

// ─── League Table ───────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayLeagueRow {
    pub position: u8,
    pub club_name: String,
    pub played: u16,
    pub won: u16,
    pub drawn: u16,
    pub lost: u16,
    pub gf: u16,
    pub ga: u16,
    pub points: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayLeagueTable {
    pub name: String,
    pub rows: Vec<DisplayLeagueRow>,
}

// ─── Save Slots ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplaySaveSlot {
    pub slot_id: u32,
    pub manager_name: String,
    pub club: String,
    pub date: String,
    pub timestamp: u64,
}

// ─── Club Selection (New Game) ──────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayClubOption {
    pub id: String,
    pub nome: String,
    #[serde(rename = "corPrimaria")]
    pub cor_primaria: String,
    #[serde(rename = "corSecundaria")]
    pub cor_secundaria: String,
    pub reputation: u8,
    pub division: String,
}

// ─── Game State (HUD) ──────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayGameState {
    pub club_name: String,
    pub club_id: String,
    pub manager_name: String,
    pub date: String,
    pub season: String,
    pub balance: String,
    pub transfer_budget: String,
    pub division: String,
    pub position: String,
}

// ─── Inbox Messages ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayMessage {
    pub id: String,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub title: String,
    pub text: String,
    pub date: String,
    pub time: String,
    pub unread: bool,
    pub tags: Vec<String>,
}

// ─── Fixtures ───────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayFixture {
    pub id: String,
    pub competition: String,
    pub round: u8,
    pub date: String,
    pub home_name: String,
    pub away_name: String,
    pub home_id: String,
    pub away_id: String,
    pub result: Option<String>,
    pub played: bool,
}

// ─── Fixture Preview (check_match_today) ────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayFixturePreview {
    pub home_name: String,
    pub away_name: String,
    pub home_id: String,
    pub away_id: String,
    pub competition: String,
    pub is_home: bool,
}

// ─── Finance ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayFinances {
    pub balance: String,
    pub transfer_budget: String,
    pub wage_budget: String,
    pub wage_bill: String,
    pub wage_room: String,
}

// ─── Advance Day Result ─────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct AdvanceDayResult {
    pub game_state: DisplayGameState,
    pub user_match: Option<DisplayFixturePreview>,
    pub round_results: Vec<RoundResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundResult {
    pub home_name: String,
    pub away_name: String,
    pub home_goals: u8,
    pub away_goals: u8,
    pub competition: String,
}

// ─── Financial History ──────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayMonthlySnapshot {
    pub month: String,
    pub balance: i64,
    pub income: i64,
    pub expenses: i64,
}

// ─── Helpers ────────────────────────────────────────────────────────────────

pub fn format_money(m: Money) -> String {
    let major = m.major();
    if major.abs() >= 1_000_000 {
        format!("£{:.1}M", major as f64 / 1_000_000.0)
    } else if major.abs() >= 1_000 {
        format!("£{:.0}K", major as f64 / 1_000.0)
    } else {
        format!("£{}", major)
    }
}
