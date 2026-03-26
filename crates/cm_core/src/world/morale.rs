//! Morale system.

use serde::{Deserialize, Serialize};

/// Morale levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoraleLevel {
    Superb,
    Good,
    Okay,
    Poor,
    VeryPoor,
}

impl std::fmt::Display for MoraleLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Superb => write!(f, "Excelente"),
            Self::Good => write!(f, "Bom"),
            Self::Okay => write!(f, "Normal"),
            Self::Poor => write!(f, "Ruim"),
            Self::VeryPoor => write!(f, "Pessimo"),
        }
    }
}

impl MoraleLevel {
    /// Get performance modifier.
    pub fn modifier(&self) -> f32 {
        match self {
            Self::Superb => 1.15,
            Self::Good => 1.05,
            Self::Okay => 1.0,
            Self::Poor => 0.90,
            Self::VeryPoor => 0.80,
        }
    }
}

/// Player morale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Morale {
    pub value: u8, // 0-100
}

impl Morale {
    /// Create new morale.
    pub fn new(value: u8) -> Self {
        Self { value: value.min(100) }
    }

    /// Get morale level.
    pub fn level(&self) -> MoraleLevel {
        match self.value {
            80..=100 => MoraleLevel::Superb,
            60..=79 => MoraleLevel::Good,
            40..=59 => MoraleLevel::Okay,
            20..=39 => MoraleLevel::Poor,
            _ => MoraleLevel::VeryPoor,
        }
    }

    /// Adjust morale.
    pub fn adjust(&mut self, delta: i8) {
        let new_value = (self.value as i16 + delta as i16).clamp(0, 100);
        self.value = new_value as u8;
    }

    /// Get performance modifier.
    pub fn modifier(&self) -> f32 {
        self.level().modifier()
    }
}

impl Default for Morale {
    fn default() -> Self {
        Self::new(50)
    }
}
