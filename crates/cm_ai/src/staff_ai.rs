//! Staff AI - Staff hiring and management decisions.

use cm_core::economy::Money;
use cm_core::ids::{ClubId, StaffId};
use cm_core::world::{Staff, StaffRole, World};

/// Minimum recommended staff by role.
pub const MIN_COACHES: usize = 2;
pub const MIN_SCOUTS: usize = 2;
pub const MIN_PHYSIOS: usize = 1;
pub const IDEAL_COACHES: usize = 4;
pub const IDEAL_SCOUTS: usize = 4;
pub const IDEAL_PHYSIOS: usize = 2;

/// Staff need assessment.
#[derive(Debug, Clone)]
pub struct StaffNeed {
    pub role: StaffRole,
    pub priority: StaffPriority,
    pub reason: String,
}

/// Priority for hiring staff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StaffPriority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// Check if should hire staff based on current situation.
pub fn should_hire_staff(world: &World, club_id: &ClubId) -> bool {
    let needs = analyze_staff_needs(world, club_id);
    
    // Should hire if any critical or high priority needs
    needs.iter().any(|n| matches!(n.priority, StaffPriority::Critical | StaffPriority::High))
}

/// Analyze staff needs for a club.
pub fn analyze_staff_needs(world: &World, club_id: &ClubId) -> Vec<StaffNeed> {
    let club = match world.clubs.get(club_id) {
        Some(c) => c,
        None => return Vec::new(),
    };
    
    // Count staff by role
    let mut coach_count = 0;
    let mut scout_count = 0;
    let mut physio_count = 0;
    let mut fitness_count = 0;
    
    for staff_id in &club.staff_ids {
        if let Some(staff) = world.staff.get(staff_id) {
            match staff.role {
                StaffRole::AssistantManager | StaffRole::Coach | StaffRole::GoalkeeperCoach | StaffRole::YouthCoach => {
                    coach_count += 1;
                }
                StaffRole::Scout => {
                    scout_count += 1;
                }
                StaffRole::Physio => {
                    physio_count += 1;
                }
                StaffRole::FitnessCoach => {
                    fitness_count += 1;
                }
                _ => {}
            }
        }
    }
    
    let mut needs = Vec::new();
    
    // Check coaching staff
    if coach_count < MIN_COACHES {
        needs.push(StaffNeed {
            role: StaffRole::Coach,
            priority: StaffPriority::Critical,
            reason: format!("Only {} coaches, need at least {}", coach_count, MIN_COACHES),
        });
    } else if coach_count < IDEAL_COACHES {
        needs.push(StaffNeed {
            role: StaffRole::Coach,
            priority: StaffPriority::Medium,
            reason: format!("Only {} coaches, ideally need {}", coach_count, IDEAL_COACHES),
        });
    }
    
    // Check scouts
    if scout_count < MIN_SCOUTS {
        needs.push(StaffNeed {
            role: StaffRole::Scout,
            priority: StaffPriority::High,
            reason: format!("Only {} scouts, need at least {}", scout_count, MIN_SCOUTS),
        });
    } else if scout_count < IDEAL_SCOUTS {
        needs.push(StaffNeed {
            role: StaffRole::Scout,
            priority: StaffPriority::Low,
            reason: format!("Only {} scouts, ideally need {}", scout_count, IDEAL_SCOUTS),
        });
    }
    
    // Check physios
    if physio_count < MIN_PHYSIOS {
        needs.push(StaffNeed {
            role: StaffRole::Physio,
            priority: StaffPriority::High,
            reason: "No physio on staff".into(),
        });
    } else if physio_count < IDEAL_PHYSIOS {
        needs.push(StaffNeed {
            role: StaffRole::Physio,
            priority: StaffPriority::Low,
            reason: format!("Only {} physio(s), could use {}", physio_count, IDEAL_PHYSIOS),
        });
    }
    
    // Check fitness coach
    if fitness_count == 0 {
        needs.push(StaffNeed {
            role: StaffRole::FitnessCoach,
            priority: StaffPriority::Medium,
            reason: "No fitness coach".into(),
        });
    }
    
    // Sort by priority
    needs.sort_by(|a, b| b.priority.cmp(&a.priority));
    
    needs
}

/// Get staff ability score based on their role.
fn get_staff_ability(staff: &Staff) -> u8 {
    match staff.role {
        StaffRole::Coach | StaffRole::AssistantManager | StaffRole::GoalkeeperCoach | StaffRole::YouthCoach => {
            // Coaching roles use coaching and man_management
            ((staff.coaching as u16 + staff.man_management as u16 + staff.tactics as u16) / 3) as u8
        }
        StaffRole::Scout => {
            staff.scouting
        }
        StaffRole::Physio => {
            staff.physiotherapy
        }
        StaffRole::FitnessCoach => {
            staff.fitness
        }
        StaffRole::Manager => {
            ((staff.coaching as u16 + staff.man_management as u16 + staff.tactics as u16) / 3) as u8
        }
        StaffRole::DataAnalyst => {
            staff.tactics
        }
    }
}

/// Evaluate a potential staff hire.
pub fn evaluate_staff_hire(
    world: &World,
    club_id: &ClubId,
    staff: &Staff,
    salary: Money,
) -> HireDecision {
    let club = match world.clubs.get(club_id) {
        Some(c) => c,
        None => return HireDecision::Reject { reason: "Club not found".into() },
    };
    
    // Check budget
    let weekly_wage_room = club.budget.wage_budget.minor() - club.budget.wage_bill.minor();
    let weekly_salary = salary.minor() / 52; // Assume annual salary
    
    if weekly_salary > weekly_wage_room {
        return HireDecision::Reject {
            reason: "Cannot afford salary within wage budget".into(),
        };
    }
    
    // Check if we need this role
    let needs = analyze_staff_needs(world, club_id);
    let role_needed = needs.iter()
        .find(|n| matches_role(&n.role, &staff.role));
    
    let ability = get_staff_ability(staff);
    
    match role_needed {
        Some(need) if need.priority >= StaffPriority::High => {
            HireDecision::Hire {
                reason: format!("High priority need for {}", staff.role.display_name()),
            }
        }
        Some(need) if need.priority >= StaffPriority::Medium => {
            // Check if quality justifies hire
            if ability >= 12 { // Good ability (12+ out of 20)
                HireDecision::Hire {
                    reason: "Good quality candidate for needed role".into(),
                }
            } else {
                HireDecision::Consider {
                    reason: "Needed role but candidate quality is moderate".into(),
                }
            }
        }
        Some(_) => HireDecision::Consider {
            reason: "Low priority need".into(),
        },
        None => HireDecision::Reject {
            reason: "No current need for this role".into(),
        },
    }
}

fn matches_role(need: &StaffRole, candidate: &StaffRole) -> bool {
    match (need, candidate) {
        (StaffRole::Coach, StaffRole::Coach) => true,
        (StaffRole::Coach, StaffRole::AssistantManager) => true,
        (StaffRole::Coach, StaffRole::GoalkeeperCoach) => true,
        (StaffRole::Scout, StaffRole::Scout) => true,
        (StaffRole::Physio, StaffRole::Physio) => true,
        (StaffRole::FitnessCoach, StaffRole::FitnessCoach) => true,
        _ => need == candidate,
    }
}

/// Decision on staff hire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HireDecision {
    Hire { reason: String },
    Consider { reason: String },
    Reject { reason: String },
}

/// Identify staff who might be released.
pub fn identify_releasable_staff(world: &World, club_id: &ClubId) -> Vec<(StaffId, ReleaseReason)> {
    let club = match world.clubs.get(club_id) {
        Some(c) => c,
        None => return Vec::new(),
    };
    
    let mut releasable = Vec::new();
    
    // Count by role to avoid releasing critical staff
    let mut role_counts: std::collections::HashMap<StaffRole, usize> = std::collections::HashMap::new();
    
    for staff_id in &club.staff_ids {
        if let Some(staff) = world.staff.get(staff_id) {
            *role_counts.entry(staff.role).or_insert(0) += 1;
        }
    }
    
    for staff_id in &club.staff_ids {
        if let Some(staff) = world.staff.get(staff_id) {
            let count = role_counts.get(&staff.role).copied().unwrap_or(0);
            
            // Don't release if only one at this role
            if count <= 1 {
                continue;
            }
            
            // Check for low quality
            let ability = get_staff_ability(staff);
            if ability < 8 { // Below average (8 out of 20)
                releasable.push((staff_id.clone(), ReleaseReason::LowQuality));
            }
        }
    }
    
    releasable
}

/// Reason for considering releasing staff.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseReason {
    LowQuality,
    HighCost,
    Redundant,
    Replacement,
}

/// Calculate staff effectiveness for training.
pub fn calculate_coaching_quality(world: &World, club_id: &ClubId) -> u8 {
    let club = match world.clubs.get(club_id) {
        Some(c) => c,
        None => return 50,
    };
    
    let mut total_coaching = 0u32;
    let mut coach_count = 0u32;
    
    for staff_id in &club.staff_ids {
        if let Some(staff) = world.staff.get(staff_id) {
            if matches!(staff.role, 
                StaffRole::Coach | StaffRole::AssistantManager | 
                StaffRole::GoalkeeperCoach | StaffRole::YouthCoach
            ) {
                total_coaching += staff.coaching as u32;
                coach_count += 1;
            }
        }
    }
    
    if coach_count == 0 {
        return 40; // No coaches is bad
    }
    
    // Convert 1-20 scale to percentage, capped at 95
    let avg = (total_coaching / coach_count) as u8;
    ((avg as u16 * 100) / 20).min(95) as u8
}

/// Calculate scouting network quality.
pub fn calculate_scouting_quality(world: &World, club_id: &ClubId) -> u8 {
    let club = match world.clubs.get(club_id) {
        Some(c) => c,
        None => return 50,
    };
    
    let mut total_scouting = 0u32;
    let mut scout_count = 0u32;
    
    for staff_id in &club.staff_ids {
        if let Some(staff) = world.staff.get(staff_id) {
            if matches!(staff.role, StaffRole::Scout) {
                total_scouting += staff.scouting as u32;
                scout_count += 1;
            }
        }
    }
    
    if scout_count == 0 {
        return 40;
    }
    
    // Convert to percentage and add coverage bonus
    let base = ((total_scouting / scout_count) as u16 * 100 / 20) as u8;
    let coverage_bonus = (scout_count as u8).min(5) * 2;
    
    (base + coverage_bonus).min(95)
}

/// Calculate medical department quality.
pub fn calculate_medical_quality(world: &World, club_id: &ClubId) -> u8 {
    let club = match world.clubs.get(club_id) {
        Some(c) => c,
        None => return 50,
    };
    
    let mut has_physio = false;
    let mut has_fitness = false;
    let mut quality = 0u32;
    let mut count = 0u32;
    
    for staff_id in &club.staff_ids {
        if let Some(staff) = world.staff.get(staff_id) {
            match staff.role {
                StaffRole::Physio => {
                    has_physio = true;
                    quality += staff.physiotherapy as u32;
                    count += 1;
                }
                StaffRole::FitnessCoach => {
                    has_fitness = true;
                    quality += staff.fitness as u32;
                    count += 1;
                }
                _ => {}
            }
        }
    }
    
    let base = if count > 0 { 
        ((quality / count) as u16 * 100 / 20) as u8
    } else { 
        40 
    };
    
    // Bonuses for having required staff
    let mut score = base;
    if has_physio {
        score = score.saturating_add(10);
    }
    if has_fitness {
        score = score.saturating_add(10);
    }
    
    score.min(95)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cm_core::ids::NationId;
    use cm_core::world::Club;

    fn create_test_staff(id: &str, role: StaffRole, quality: u8) -> Staff {
        let mut staff = Staff::new(
            id,
            "Test",
            id,
            NationId::new("test"),
            role,
        );
        staff.coaching = quality;
        staff.man_management = quality;
        staff.tactics = quality;
        staff.scouting = quality;
        staff.youth_development = quality;
        staff.physiotherapy = quality;
        staff.fitness = quality;
        staff
    }

    fn setup_test_world() -> (World, ClubId) {
        let mut world = World::new();
        let club_id = ClubId::new("test_club");
        
        let mut club = Club::new("test_club", "Test FC", NationId::new("test"));
        club.budget.wage_budget = Money::from_major(500_000);
        club.budget.wage_bill = Money::from_major(200_000);
        
        // Add minimal staff
        let staff = vec![
            create_test_staff("coach1", StaffRole::Coach, 14),
            create_test_staff("scout1", StaffRole::Scout, 13),
        ];
        
        for s in staff {
            club.staff_ids.push(s.id.clone());
            world.staff.insert(s.id.clone(), s);
        }
        
        world.clubs.insert(club_id.clone(), club);
        
        (world, club_id)
    }

    fn setup_full_staff_world() -> (World, ClubId) {
        let mut world = World::new();
        let club_id = ClubId::new("full_club");
        
        let mut club = Club::new("full_club", "Full FC", NationId::new("test"));
        club.budget.wage_budget = Money::from_major(1_000_000);
        club.budget.wage_bill = Money::from_major(300_000);
        
        let staff = vec![
            create_test_staff("coach1", StaffRole::Coach, 15),
            create_test_staff("coach2", StaffRole::Coach, 14),
            create_test_staff("coach3", StaffRole::AssistantManager, 14),
            create_test_staff("coach4", StaffRole::GoalkeeperCoach, 13),
            create_test_staff("scout1", StaffRole::Scout, 16),
            create_test_staff("scout2", StaffRole::Scout, 14),
            create_test_staff("scout3", StaffRole::Scout, 13),
            create_test_staff("scout4", StaffRole::Scout, 12),
            create_test_staff("physio1", StaffRole::Physio, 14),
            create_test_staff("physio2", StaffRole::Physio, 13),
            create_test_staff("fitness1", StaffRole::FitnessCoach, 14),
        ];
        
        for s in staff {
            club.staff_ids.push(s.id.clone());
            world.staff.insert(s.id.clone(), s);
        }
        
        world.clubs.insert(club_id.clone(), club);
        
        (world, club_id)
    }

    #[test]
    fn test_should_hire_staff_minimal() {
        let (world, club_id) = setup_test_world();
        
        let should = should_hire_staff(&world, &club_id);
        assert!(should, "Club with minimal staff should need more");
    }

    #[test]
    fn test_should_not_hire_full_staff() {
        let (world, club_id) = setup_full_staff_world();
        
        let should = should_hire_staff(&world, &club_id);
        assert!(!should, "Fully staffed club should not urgently need more");
    }

    #[test]
    fn test_analyze_staff_needs() {
        let (world, club_id) = setup_test_world();
        
        let needs = analyze_staff_needs(&world, &club_id);
        
        assert!(!needs.is_empty());
        
        // Should need more coaches (only 1)
        let coach_need = needs.iter().find(|n| n.role == StaffRole::Coach);
        assert!(coach_need.is_some());
        assert!(matches!(coach_need.unwrap().priority, StaffPriority::Critical));
        
        // Should need physio (0)
        let physio_need = needs.iter().find(|n| n.role == StaffRole::Physio);
        assert!(physio_need.is_some());
    }

    #[test]
    fn test_evaluate_staff_hire_needed() {
        let (world, club_id) = setup_test_world();
        
        let candidate = create_test_staff("new_coach", StaffRole::Coach, 14);
        let salary = Money::from_major(60_000);
        
        let decision = evaluate_staff_hire(&world, &club_id, &candidate, salary);
        
        assert!(matches!(decision, HireDecision::Hire { .. }));
    }

    #[test]
    fn test_evaluate_staff_hire_not_needed() {
        let (world, club_id) = setup_full_staff_world();
        
        let candidate = create_test_staff("extra_scout", StaffRole::Scout, 10);
        let salary = Money::from_major(40_000);
        
        let decision = evaluate_staff_hire(&world, &club_id, &candidate, salary);
        
        // Already have enough scouts
        assert!(matches!(decision, HireDecision::Consider { .. } | HireDecision::Reject { .. }));
    }

    #[test]
    fn test_evaluate_staff_hire_too_expensive() {
        let (mut world, club_id) = setup_test_world();
        
        // Set wage_bill to nearly max out the weekly wage_budget
        // wage_budget is 500k weekly, set wage_bill to 499k so only 1k room
        world.clubs.get_mut(&club_id).unwrap().budget.wage_bill = Money::from_major(499_000);
        
        let candidate = create_test_staff("expensive", StaffRole::Coach, 17);
        // Salary is annual - 100k annual = ~1923 weekly, which exceeds 1k room
        let salary = Money::from_major(100_000);
        
        let decision = evaluate_staff_hire(&world, &club_id, &candidate, salary);
        
        assert!(matches!(decision, HireDecision::Reject { .. }));
    }

    #[test]
    fn test_identify_releasable_staff() {
        let (mut world, club_id) = setup_full_staff_world();
        
        // Add a low quality coach
        let mut bad_coach = create_test_staff("bad_coach", StaffRole::Coach, 5);
        
        world.clubs.get_mut(&club_id).unwrap().staff_ids.push(bad_coach.id.clone());
        world.staff.insert(bad_coach.id.clone(), bad_coach);
        
        let releasable = identify_releasable_staff(&world, &club_id);
        
        let has_bad_coach = releasable.iter()
            .any(|(id, reason)| *id == StaffId::new("bad_coach") && *reason == ReleaseReason::LowQuality);
        
        assert!(has_bad_coach, "Low quality coach should be releasable");
    }

    #[test]
    fn test_calculate_coaching_quality() {
        let (world, club_id) = setup_full_staff_world();
        
        let quality = calculate_coaching_quality(&world, &club_id);
        
        assert!(quality >= 60, "Full coaching staff should have good quality: {}", quality);
    }

    #[test]
    fn test_calculate_coaching_quality_no_coaches() {
        let mut world = World::new();
        let club_id = ClubId::new("empty");
        let club = Club::new("empty", "Empty FC", NationId::new("test"));
        world.clubs.insert(club_id.clone(), club);
        
        let quality = calculate_coaching_quality(&world, &club_id);
        
        assert_eq!(quality, 40, "No coaches should give low quality");
    }

    #[test]
    fn test_calculate_scouting_quality() {
        let (world, club_id) = setup_full_staff_world();
        
        let quality = calculate_scouting_quality(&world, &club_id);
        
        assert!(quality >= 60, "Full scouting network should have good quality: {}", quality);
    }

    #[test]
    fn test_calculate_medical_quality() {
        let (world, club_id) = setup_full_staff_world();
        
        let quality = calculate_medical_quality(&world, &club_id);
        
        assert!(quality >= 70, "Full medical staff should have good quality: {}", quality);
    }

    #[test]
    fn test_staff_priority_ordering() {
        assert!(StaffPriority::Critical > StaffPriority::High);
        assert!(StaffPriority::High > StaffPriority::Medium);
        assert!(StaffPriority::Medium > StaffPriority::Low);
    }

    #[test]
    fn test_needs_sorted_by_priority() {
        let (world, club_id) = setup_test_world();
        
        let needs = analyze_staff_needs(&world, &club_id);
        
        for i in 1..needs.len() {
            assert!(needs[i-1].priority >= needs[i].priority, "Should be sorted by priority");
        }
    }
}
