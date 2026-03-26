//! Training system.

use serde::{Deserialize, Serialize};

/// Training focus area.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrainingFocus {
    General,
    Attacking,
    Defending,
    Fitness,
    Tactics,
    SetPieces,
    Goalkeeping,
    /// Physical training: pace, stamina, strength, acceleration, agility.
    Physical,
    /// Technical training: finishing, passing, dribbling, crossing, tackling.
    Technical,
    /// Tactical training: positioning, decisions, anticipation, vision, composure.
    Tactical,
    /// Recovery: restores fitness, reduces injury risk, no attribute gains.
    Recovery,
}

impl Default for TrainingFocus {
    fn default() -> Self {
        Self::General
    }
}

/// Training intensity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrainingIntensity {
    Light,
    Medium,
    High,
    VeryHigh,
}

impl Default for TrainingIntensity {
    fn default() -> Self {
        Self::Medium
    }
}

/// Training schedule.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Training {
    pub focus: TrainingFocus,
    pub intensity: TrainingIntensity,
    pub morale_impact: i8,
    pub injury_risk: u8,
    pub fitness_gain: i8,
}

impl Training {
    /// Create training with focus.
    pub fn new(focus: TrainingFocus, intensity: TrainingIntensity) -> Self {
        let (morale_impact, injury_risk, fitness_gain) = match intensity {
            TrainingIntensity::Light => (2, 1, 1),
            TrainingIntensity::Medium => (0, 3, 2),
            TrainingIntensity::High => (-2, 6, 4),
            TrainingIntensity::VeryHigh => (-5, 10, 6),
        };

        Self {
            focus,
            intensity,
            morale_impact,
            injury_risk,
            fitness_gain,
        }
    }
}
