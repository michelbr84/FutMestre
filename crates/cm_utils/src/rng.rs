//! Random number generation utilities.

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Create a seeded RNG for deterministic simulation.
pub fn seeded_rng(seed: u64) -> ChaCha8Rng {
    ChaCha8Rng::seed_from_u64(seed)
}

/// Create an entropy-based RNG.
pub fn entropy_rng() -> ChaCha8Rng {
    ChaCha8Rng::from_entropy()
}

/// Generate a random value in range [min, max).
pub fn random_range<R: Rng>(rng: &mut R, min: i32, max: i32) -> i32 {
    rng.gen_range(min..max)
}

/// Generate a random float in range [0.0, 1.0).
pub fn random_float<R: Rng>(rng: &mut R) -> f32 {
    rng.gen()
}

/// Roll a percentage chance (0-100).
pub fn roll_chance<R: Rng>(rng: &mut R, percent: u8) -> bool {
    rng.gen_range(0..100) < percent
}

/// Pick a random element from a slice.
pub fn pick_random<'a, T, R: Rng>(rng: &mut R, items: &'a [T]) -> Option<&'a T> {
    if items.is_empty() {
        None
    } else {
        Some(&items[rng.gen_range(0..items.len())])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seeded_rng_deterministic() {
        let mut rng1 = seeded_rng(42);
        let mut rng2 = seeded_rng(42);

        for _ in 0..100 {
            assert_eq!(random_range(&mut rng1, 0, 1000), random_range(&mut rng2, 0, 1000));
        }
    }

    #[test]
    fn test_seeded_rng_different_seeds() {
        let mut rng1 = seeded_rng(42);
        let mut rng2 = seeded_rng(43);

        // Very unlikely to be equal with different seeds
        let vals1: Vec<i32> = (0..10).map(|_| random_range(&mut rng1, 0, 1000)).collect();
        let vals2: Vec<i32> = (0..10).map(|_| random_range(&mut rng2, 0, 1000)).collect();
        assert_ne!(vals1, vals2);
    }

    #[test]
    fn test_entropy_rng_different_each_time() {
        let mut rng1 = entropy_rng();
        let mut rng2 = entropy_rng();

        // Very unlikely to produce same sequence
        let vals1: Vec<i32> = (0..10).map(|_| random_range(&mut rng1, 0, i32::MAX)).collect();
        let vals2: Vec<i32> = (0..10).map(|_| random_range(&mut rng2, 0, i32::MAX)).collect();
        assert_ne!(vals1, vals2);
    }

    #[test]
    fn test_random_range_bounds() {
        let mut rng = seeded_rng(42);
        for _ in 0..1000 {
            let val = random_range(&mut rng, 10, 20);
            assert!(val >= 10 && val < 20);
        }
    }

    #[test]
    fn test_random_range_single_value() {
        let mut rng = seeded_rng(42);
        for _ in 0..100 {
            let val = random_range(&mut rng, 5, 6);
            assert_eq!(val, 5);
        }
    }

    #[test]
    fn test_random_float_bounds() {
        let mut rng = seeded_rng(42);
        for _ in 0..1000 {
            let val = random_float(&mut rng);
            assert!(val >= 0.0 && val < 1.0);
        }
    }

    #[test]
    fn test_roll_chance_zero() {
        let mut rng = seeded_rng(42);
        for _ in 0..100 {
            assert!(!roll_chance(&mut rng, 0));
        }
    }

    #[test]
    fn test_roll_chance_hundred() {
        let mut rng = seeded_rng(42);
        for _ in 0..100 {
            assert!(roll_chance(&mut rng, 100));
        }
    }

    #[test]
    fn test_roll_chance_distribution() {
        let mut rng = seeded_rng(42);
        let successes = (0..10000).filter(|_| roll_chance(&mut rng, 50)).count();
        // Should be roughly 50% (within reasonable margin)
        assert!(successes > 4500 && successes < 5500);
    }

    #[test]
    fn test_pick_random_empty() {
        let mut rng = seeded_rng(42);
        let items: Vec<i32> = vec![];
        assert!(pick_random(&mut rng, &items).is_none());
    }

    #[test]
    fn test_pick_random_single() {
        let mut rng = seeded_rng(42);
        let items = vec![42];
        assert_eq!(pick_random(&mut rng, &items), Some(&42));
    }

    #[test]
    fn test_pick_random_multiple() {
        let mut rng = seeded_rng(42);
        let items = vec![1, 2, 3, 4, 5];
        for _ in 0..100 {
            let picked = pick_random(&mut rng, &items);
            assert!(picked.is_some());
            assert!(items.contains(picked.unwrap()));
        }
    }

    #[test]
    fn test_pick_random_all_items_possible() {
        let mut rng = seeded_rng(42);
        let items = vec![1, 2, 3];
        let mut seen = std::collections::HashSet::new();
        for _ in 0..1000 {
            if let Some(&val) = pick_random(&mut rng, &items) {
                seen.insert(val);
            }
        }
        // All items should have been picked at least once
        assert_eq!(seen.len(), 3);
    }
}
