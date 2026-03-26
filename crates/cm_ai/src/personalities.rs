//! AI personality types - Affects AI decision making across all systems.

use cm_core::world::{Formation, Mentality, Tempo};
use serde::{Deserialize, Serialize};

/// Manager personality affects AI decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManagerPersonality {
    Defensive,
    Balanced,
    Attacking,
    YouthFocused,
    WinAtAllCosts,
    Financial,
}

impl Default for ManagerPersonality {
    fn default() -> Self {
        Self::Balanced
    }
}

impl ManagerPersonality {
    /// Get display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Defensive => "Defensive Manager",
            Self::Balanced => "Balanced Manager",
            Self::Attacking => "Attacking Manager",
            Self::YouthFocused => "Youth Developer",
            Self::WinAtAllCosts => "Win At All Costs",
            Self::Financial => "Financial Manager",
        }
    }

    /// Get description.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Defensive => "Prioritizes defensive stability and organization",
            Self::Balanced => "Takes a balanced approach to tactics and squad building",
            Self::Attacking => "Prefers attacking football and creative players",
            Self::YouthFocused => "Develops young talent and invests in the academy",
            Self::WinAtAllCosts => "Signs established players and prioritizes short-term success",
            Self::Financial => "Manages budget carefully and looks for bargains",
        }
    }
}

/// Get preferred squad size based on personality.
pub fn preferred_squad_size(personality: ManagerPersonality) -> usize {
    match personality {
        ManagerPersonality::Defensive => 28,
        ManagerPersonality::Balanced => 25,
        ManagerPersonality::Attacking => 22,
        ManagerPersonality::YouthFocused => 30,
        ManagerPersonality::WinAtAllCosts => 25,
        ManagerPersonality::Financial => 22,
    }
}

/// Get youth development priority (0-100).
pub fn youth_priority(personality: ManagerPersonality) -> u8 {
    match personality {
        ManagerPersonality::YouthFocused => 90,
        ManagerPersonality::Financial => 70,
        ManagerPersonality::Balanced => 50,
        ManagerPersonality::Attacking => 40,
        ManagerPersonality::Defensive => 40,
        ManagerPersonality::WinAtAllCosts => 20,
    }
}

/// Get preferred formation based on personality.
pub fn preferred_formation(personality: ManagerPersonality) -> Formation {
    match personality {
        ManagerPersonality::Defensive => Formation::F532,
        ManagerPersonality::Balanced => Formation::F442,
        ManagerPersonality::Attacking => Formation::F433,
        ManagerPersonality::YouthFocused => Formation::F442,
        ManagerPersonality::WinAtAllCosts => Formation::F4231,
        ManagerPersonality::Financial => Formation::F442,
    }
}

/// Get preferred mentality for default setup.
pub fn preferred_mentality(personality: ManagerPersonality) -> Mentality {
    match personality {
        ManagerPersonality::Defensive => Mentality::Cautious,
        ManagerPersonality::Balanced => Mentality::Balanced,
        ManagerPersonality::Attacking => Mentality::Attacking,
        ManagerPersonality::YouthFocused => Mentality::Balanced,
        ManagerPersonality::WinAtAllCosts => Mentality::Attacking,
        ManagerPersonality::Financial => Mentality::Cautious,
    }
}

/// Get preferred tempo.
pub fn preferred_tempo(personality: ManagerPersonality) -> Tempo {
    match personality {
        ManagerPersonality::Defensive => Tempo::Slow,
        ManagerPersonality::Balanced => Tempo::Normal,
        ManagerPersonality::Attacking => Tempo::Fast,
        ManagerPersonality::YouthFocused => Tempo::Normal,
        ManagerPersonality::WinAtAllCosts => Tempo::Fast,
        ManagerPersonality::Financial => Tempo::Normal,
    }
}

/// Get transfer budget spending preference (0-100, higher = more willing to spend).
pub fn transfer_spending_preference(personality: ManagerPersonality) -> u8 {
    match personality {
        ManagerPersonality::Defensive => 50,
        ManagerPersonality::Balanced => 60,
        ManagerPersonality::Attacking => 70,
        ManagerPersonality::YouthFocused => 40,
        ManagerPersonality::WinAtAllCosts => 90,
        ManagerPersonality::Financial => 30,
    }
}

/// Get risk tolerance for transfers (0-100).
pub fn risk_tolerance(personality: ManagerPersonality) -> u8 {
    match personality {
        ManagerPersonality::Defensive => 30,
        ManagerPersonality::Balanced => 50,
        ManagerPersonality::Attacking => 60,
        ManagerPersonality::YouthFocused => 70, // Willing to take risks on youth
        ManagerPersonality::WinAtAllCosts => 80,
        ManagerPersonality::Financial => 40,
    }
}

/// Get patience with young players (0-100).
pub fn youth_patience(personality: ManagerPersonality) -> u8 {
    match personality {
        ManagerPersonality::YouthFocused => 95,
        ManagerPersonality::Financial => 70,
        ManagerPersonality::Balanced => 60,
        ManagerPersonality::Defensive => 50,
        ManagerPersonality::Attacking => 50,
        ManagerPersonality::WinAtAllCosts => 20,
    }
}

/// Get press conference response style.
pub fn press_style(personality: ManagerPersonality) -> PressStyle {
    match personality {
        ManagerPersonality::Defensive => PressStyle::Cautious,
        ManagerPersonality::Balanced => PressStyle::Diplomatic,
        ManagerPersonality::Attacking => PressStyle::Confident,
        ManagerPersonality::YouthFocused => PressStyle::Optimistic,
        ManagerPersonality::WinAtAllCosts => PressStyle::Aggressive,
        ManagerPersonality::Financial => PressStyle::Pragmatic,
    }
}

/// Press conference communication style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressStyle {
    Cautious,
    Diplomatic,
    Confident,
    Optimistic,
    Aggressive,
    Pragmatic,
}

/// Get minimum acceptable player age for signings.
pub fn min_signing_age(personality: ManagerPersonality) -> u8 {
    match personality {
        ManagerPersonality::YouthFocused => 16,
        ManagerPersonality::Balanced => 18,
        ManagerPersonality::Financial => 17,
        ManagerPersonality::Attacking => 19,
        ManagerPersonality::Defensive => 20,
        ManagerPersonality::WinAtAllCosts => 22,
    }
}

/// Get maximum preferred signing age.
pub fn max_signing_age(personality: ManagerPersonality) -> u8 {
    match personality {
        ManagerPersonality::YouthFocused => 25,
        ManagerPersonality::Financial => 27,
        ManagerPersonality::Balanced => 30,
        ManagerPersonality::Attacking => 29,
        ManagerPersonality::Defensive => 32,
        ManagerPersonality::WinAtAllCosts => 35,
    }
}

/// Get preferred pressing intensity (0-100).
pub fn preferred_pressing(personality: ManagerPersonality) -> u8 {
    match personality {
        ManagerPersonality::Defensive => 35,
        ManagerPersonality::Balanced => 50,
        ManagerPersonality::Attacking => 70,
        ManagerPersonality::YouthFocused => 55,
        ManagerPersonality::WinAtAllCosts => 65,
        ManagerPersonality::Financial => 45,
    }
}

/// Get preferred defensive line height (0-100, low to high).
pub fn preferred_defensive_line(personality: ManagerPersonality) -> u8 {
    match personality {
        ManagerPersonality::Defensive => 30,
        ManagerPersonality::Balanced => 50,
        ManagerPersonality::Attacking => 65,
        ManagerPersonality::YouthFocused => 50,
        ManagerPersonality::WinAtAllCosts => 55,
        ManagerPersonality::Financial => 45,
    }
}

/// Calculate personality modifier for a decision value.
/// Returns a multiplier based on how the personality affects the given factor.
pub fn personality_modifier(personality: ManagerPersonality, factor: DecisionFactor) -> f32 {
    match (personality, factor) {
        // Defensive personality modifiers
        (ManagerPersonality::Defensive, DecisionFactor::DefensiveStrength) => 1.3,
        (ManagerPersonality::Defensive, DecisionFactor::AttackingStrength) => 0.8,
        (ManagerPersonality::Defensive, DecisionFactor::YouthPotential) => 0.9,
        (ManagerPersonality::Defensive, DecisionFactor::Experience) => 1.2,

        // Attacking personality modifiers
        (ManagerPersonality::Attacking, DecisionFactor::DefensiveStrength) => 0.8,
        (ManagerPersonality::Attacking, DecisionFactor::AttackingStrength) => 1.3,
        (ManagerPersonality::Attacking, DecisionFactor::Creativity) => 1.2,

        // Youth focused modifiers
        (ManagerPersonality::YouthFocused, DecisionFactor::YouthPotential) => 1.5,
        (ManagerPersonality::YouthFocused, DecisionFactor::Age) => 0.7,
        (ManagerPersonality::YouthFocused, DecisionFactor::Experience) => 0.8,

        // Win at all costs modifiers
        (ManagerPersonality::WinAtAllCosts, DecisionFactor::CurrentAbility) => 1.3,
        (ManagerPersonality::WinAtAllCosts, DecisionFactor::YouthPotential) => 0.7,
        (ManagerPersonality::WinAtAllCosts, DecisionFactor::Value) => 0.8,

        // Financial modifiers
        (ManagerPersonality::Financial, DecisionFactor::Value) => 1.4,
        (ManagerPersonality::Financial, DecisionFactor::WagesCost) => 1.3,
        (ManagerPersonality::Financial, DecisionFactor::ResaleValue) => 1.3,

        // Default - no modifier
        _ => 1.0,
    }
}

/// Factors that personalities can influence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionFactor {
    CurrentAbility,
    YouthPotential,
    DefensiveStrength,
    AttackingStrength,
    Creativity,
    Experience,
    Age,
    Value,
    WagesCost,
    ResaleValue,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_personality() {
        assert_eq!(ManagerPersonality::default(), ManagerPersonality::Balanced);
    }

    #[test]
    fn test_preferred_squad_size() {
        assert!(
            preferred_squad_size(ManagerPersonality::YouthFocused)
                > preferred_squad_size(ManagerPersonality::Financial)
        );
    }

    #[test]
    fn test_youth_priority() {
        assert_eq!(youth_priority(ManagerPersonality::YouthFocused), 90);
        assert_eq!(youth_priority(ManagerPersonality::WinAtAllCosts), 20);
    }

    #[test]
    fn test_preferred_formations() {
        assert_eq!(
            preferred_formation(ManagerPersonality::Defensive),
            Formation::F532
        );
        assert_eq!(
            preferred_formation(ManagerPersonality::Attacking),
            Formation::F433
        );
    }

    #[test]
    fn test_preferred_mentality() {
        assert_eq!(
            preferred_mentality(ManagerPersonality::Defensive),
            Mentality::Cautious
        );
        assert_eq!(
            preferred_mentality(ManagerPersonality::Attacking),
            Mentality::Attacking
        );
    }

    #[test]
    fn test_transfer_spending() {
        // Win at all costs should spend the most
        assert!(
            transfer_spending_preference(ManagerPersonality::WinAtAllCosts)
                > transfer_spending_preference(ManagerPersonality::Balanced)
        );

        // Financial should spend the least
        assert!(
            transfer_spending_preference(ManagerPersonality::Financial)
                < transfer_spending_preference(ManagerPersonality::Balanced)
        );
    }

    #[test]
    fn test_risk_tolerance() {
        // Win at all costs = high risk
        assert!(risk_tolerance(ManagerPersonality::WinAtAllCosts) >= 80);

        // Defensive = low risk
        assert!(risk_tolerance(ManagerPersonality::Defensive) <= 40);
    }

    #[test]
    fn test_youth_patience() {
        assert_eq!(youth_patience(ManagerPersonality::YouthFocused), 95);
        assert_eq!(youth_patience(ManagerPersonality::WinAtAllCosts), 20);
    }

    #[test]
    fn test_signing_age_ranges() {
        // Youth focused wants young players
        assert!(max_signing_age(ManagerPersonality::YouthFocused) <= 25);
        assert!(min_signing_age(ManagerPersonality::YouthFocused) <= 18);

        // Win at all costs accepts older players
        assert!(max_signing_age(ManagerPersonality::WinAtAllCosts) >= 30);
    }

    #[test]
    fn test_pressing_and_line() {
        // Attacking should press higher
        assert!(
            preferred_pressing(ManagerPersonality::Attacking)
                > preferred_pressing(ManagerPersonality::Defensive)
        );

        // Attacking should have higher line
        assert!(
            preferred_defensive_line(ManagerPersonality::Attacking)
                > preferred_defensive_line(ManagerPersonality::Defensive)
        );
    }

    #[test]
    fn test_personality_modifier_defensive() {
        let def_mod = personality_modifier(
            ManagerPersonality::Defensive,
            DecisionFactor::DefensiveStrength,
        );
        let att_mod = personality_modifier(
            ManagerPersonality::Defensive,
            DecisionFactor::AttackingStrength,
        );

        assert!(def_mod > 1.0);
        assert!(att_mod < 1.0);
    }

    #[test]
    fn test_personality_modifier_youth() {
        let youth_mod = personality_modifier(
            ManagerPersonality::YouthFocused,
            DecisionFactor::YouthPotential,
        );
        let exp_mod =
            personality_modifier(ManagerPersonality::YouthFocused, DecisionFactor::Experience);

        assert!(youth_mod > 1.0);
        assert!(exp_mod < 1.0);
    }

    #[test]
    fn test_personality_modifier_financial() {
        let value_mod = personality_modifier(ManagerPersonality::Financial, DecisionFactor::Value);
        assert!(value_mod > 1.0);
    }

    #[test]
    fn test_personality_modifier_default() {
        // Unknown combinations should return 1.0
        let default_mod =
            personality_modifier(ManagerPersonality::Balanced, DecisionFactor::Creativity);
        assert_eq!(default_mod, 1.0);
    }

    #[test]
    fn test_display_name() {
        assert_eq!(
            ManagerPersonality::Balanced.display_name(),
            "Balanced Manager"
        );
        assert_eq!(
            ManagerPersonality::YouthFocused.display_name(),
            "Youth Developer"
        );
    }

    #[test]
    fn test_description() {
        let desc = ManagerPersonality::WinAtAllCosts.description();
        assert!(desc.contains("short-term"));
    }

    #[test]
    fn test_press_style() {
        assert_eq!(
            press_style(ManagerPersonality::Defensive),
            PressStyle::Cautious
        );
        assert_eq!(
            press_style(ManagerPersonality::WinAtAllCosts),
            PressStyle::Aggressive
        );
    }

    #[test]
    fn test_all_personalities_have_values() {
        let personalities = [
            ManagerPersonality::Defensive,
            ManagerPersonality::Balanced,
            ManagerPersonality::Attacking,
            ManagerPersonality::YouthFocused,
            ManagerPersonality::WinAtAllCosts,
            ManagerPersonality::Financial,
        ];

        for p in personalities {
            // Ensure all functions return valid values
            assert!(preferred_squad_size(p) >= 20 && preferred_squad_size(p) <= 35);
            assert!(youth_priority(p) <= 100);
            assert!(transfer_spending_preference(p) <= 100);
            assert!(risk_tolerance(p) <= 100);
            assert!(youth_patience(p) <= 100);
            assert!(min_signing_age(p) < max_signing_age(p));
        }
    }
}
