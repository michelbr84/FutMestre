//! Match injury simulation (stub).

use rand::Rng;

/// Calculate injury chance per minute.
pub fn injury_chance_per_minute(fitness: u8, natural_fitness: u8, _training_intensity: u8) -> f32 {
    let base = 0.0005; // 0.05% per minute
    let fitness_mod = 1.0 + ((100 - fitness) as f32 / 100.0);
    let nat_fit_mod = 1.0 - (natural_fitness as f32 / 200.0);

    base * fitness_mod * nat_fit_mod
}

/// Roll for injury.
pub fn check_injury<R: Rng>(rng: &mut R, chance: f32) -> bool {
    rng.gen::<f32>() < chance
}

/// Calculate injury severity (days out).
pub fn injury_severity<R: Rng>(rng: &mut R) -> u16 {
    let roll: f32 = rng.gen();

    if roll < 0.5 {
        rng.gen_range(1..=7) // Minor: 1-7 days
    } else if roll < 0.8 {
        rng.gen_range(7..=28) // Moderate: 1-4 weeks
    } else if roll < 0.95 {
        rng.gen_range(28..=90) // Serious: 1-3 months
    } else {
        rng.gen_range(90..=270) // Severe: 3-9 months
    }
}
