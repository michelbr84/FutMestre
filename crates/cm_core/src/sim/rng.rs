//! Seeded RNG for deterministic simulation.

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

/// Simulation RNG wrapper for reproducible results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimRng {
    seed: u64,
    #[serde(skip)]
    rng: Option<StdRng>,
}

impl SimRng {
    /// Create with a specific seed.
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            rng: Some(StdRng::seed_from_u64(seed)),
        }
    }

    /// Create from entropy.
    pub fn from_entropy() -> Self {
        let mut rng = StdRng::from_entropy();
        let seed = rng.gen();
        Self {
            seed,
            rng: Some(rng),
        }
    }

    /// Get the seed.
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Ensure RNG is initialized (needed after deserialization).
    fn ensure_init(&mut self) {
        if self.rng.is_none() {
            self.rng = Some(StdRng::seed_from_u64(self.seed));
        }
    }

    /// Generate a random float in [0, 1).
    pub fn random(&mut self) -> f32 {
        self.ensure_init();
        self.rng.as_mut().unwrap().gen()
    }

    /// Generate a random integer in range [min, max).
    pub fn range(&mut self, min: i32, max: i32) -> i32 {
        self.ensure_init();
        self.rng.as_mut().unwrap().gen_range(min..max)
    }

    /// Roll a percentage chance (0-100).
    pub fn roll(&mut self, percent: u8) -> bool {
        self.range(0, 100) < percent as i32
    }

    /// Pick a random index for a slice of given length.
    pub fn index(&mut self, len: usize) -> usize {
        if len == 0 {
            0
        } else {
            self.range(0, len as i32) as usize
        }
    }
}

impl Default for SimRng {
    fn default() -> Self {
        Self::from_entropy()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seeded_rng_deterministic() {
        let mut rng1 = SimRng::new(42);
        let mut rng2 = SimRng::new(42);

        for _ in 0..100 {
            assert_eq!(rng1.range(0, 1000), rng2.range(0, 1000));
        }
    }

    #[test]
    fn test_different_seeds_different_output() {
        let mut rng1 = SimRng::new(42);
        let mut rng2 = SimRng::new(43);

        let vals1: Vec<i32> = (0..10).map(|_| rng1.range(0, 1000)).collect();
        let vals2: Vec<i32> = (0..10).map(|_| rng2.range(0, 1000)).collect();

        assert_ne!(vals1, vals2);
    }

    #[test]
    fn test_seed_retrieval() {
        let rng = SimRng::new(12345);
        assert_eq!(rng.seed(), 12345);
    }

    #[test]
    fn test_random_in_range() {
        let mut rng = SimRng::new(42);
        for _ in 0..1000 {
            let val = rng.random();
            assert!(val >= 0.0 && val < 1.0);
        }
    }

    #[test]
    fn test_range_bounds() {
        let mut rng = SimRng::new(42);
        for _ in 0..1000 {
            let val = rng.range(10, 20);
            assert!(val >= 10 && val < 20);
        }
    }

    #[test]
    fn test_roll_zero_percent() {
        let mut rng = SimRng::new(42);
        for _ in 0..100 {
            assert!(!rng.roll(0));
        }
    }

    #[test]
    fn test_roll_hundred_percent() {
        let mut rng = SimRng::new(42);
        for _ in 0..100 {
            assert!(rng.roll(100));
        }
    }

    #[test]
    fn test_roll_distribution() {
        let mut rng = SimRng::new(42);
        let successes = (0..10000).filter(|_| rng.roll(50)).count();
        // Should be roughly 50% (within reasonable margin)
        assert!(successes > 4500 && successes < 5500);
    }

    #[test]
    fn test_index_empty() {
        let mut rng = SimRng::new(42);
        assert_eq!(rng.index(0), 0);
    }

    #[test]
    fn test_index_single() {
        let mut rng = SimRng::new(42);
        for _ in 0..100 {
            assert_eq!(rng.index(1), 0);
        }
    }

    #[test]
    fn test_index_bounds() {
        let mut rng = SimRng::new(42);
        for _ in 0..1000 {
            let idx = rng.index(10);
            assert!(idx < 10);
        }
    }

    #[test]
    fn test_serialization_preserves_seed() {
        let rng = SimRng::new(12345);
        let json = serde_json::to_string(&rng).unwrap();
        let parsed: SimRng = serde_json::from_str(&json).unwrap();

        assert_eq!(rng.seed(), parsed.seed());
    }

    #[test]
    fn test_deserialized_rng_works() {
        let original = SimRng::new(42);
        let json = serde_json::to_string(&original).unwrap();
        let mut parsed: SimRng = serde_json::from_str(&json).unwrap();

        // RNG should be reinitialized and work
        let val = parsed.range(0, 100);
        assert!(val >= 0 && val < 100);
    }

    #[test]
    fn test_from_entropy_different() {
        let rng1 = SimRng::from_entropy();
        let rng2 = SimRng::from_entropy();

        // Seeds should be different (extremely unlikely to be equal)
        assert_ne!(rng1.seed(), rng2.seed());
    }
}
