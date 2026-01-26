use serde::{Serialize, Deserialize};
use cm_core::world::player::{Player};
use cm_core::world::attributes::Attributes;

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
}

impl From<&Player> for DisplayPlayer {
    fn from(p: &Player) -> Self {
        // Simplified mapping for display
        // In a real scenario, we'd lookup nation name from ID, etc.
        Self {
            id: p.id.to_string(),
            name: p.full_name(),
            position: p.position.short_name().to_string(),
            age: p.age_on(chrono::Local::now().date_naive()), // Determine date from game state ideally
            nationality: format!("Nation {}", p.nationality.0), // Placeholder until we have Nation DB access here
            overall: p.overall_rating(),
            value: format!("£{:.1}k", p.value.major() as f64 / 1000.0),
            wage: format!("£{:.1}k/w", p.weekly_wage().major() as f64 / 1000.0),
            condition: p.fitness,
            morale: format!("{:?}", p.morale), // Debug formatting for now
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
                ("Crossing".to_string(), a.technical.crossing),
                ("Dribbling".to_string(), a.technical.dribbling),
                ("Finishing".to_string(), a.technical.finishing),
                ("Passing".to_string(), a.technical.passing),
                ("Tackling".to_string(), a.technical.tackling),
                ("Technique".to_string(), a.technical.technique),
            ],
            mental: vec![
                ("Aggression".to_string(), a.mental.aggression),
                ("Decisions".to_string(), a.mental.decisions),
                ("Determination".to_string(), a.mental.determination),
                ("Leadership".to_string(), a.mental.leadership),
                ("Positioning".to_string(), a.mental.positioning),
                ("Work Rate".to_string(), a.mental.work_rate),
            ],
            physical: vec![
                ("Acceleration".to_string(), a.physical.acceleration),
                ("Agility".to_string(), a.physical.agility),
                ("Balance".to_string(), a.physical.balance),
                ("Pace".to_string(), a.physical.pace),
                ("Stamina".to_string(), a.physical.stamina),
                ("Strength".to_string(), a.physical.strength),
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayMatchResult {
    pub home_goals: u8,
    pub away_goals: u8,
    pub highlights: Vec<String>,
}

impl From<cm_match::MatchResult> for DisplayMatchResult {
    fn from(r: cm_match::MatchResult) -> Self {
        Self {
            home_goals: r.home_goals,
            away_goals: r.away_goals,
            highlights: r.highlights,
        }
    }
}
