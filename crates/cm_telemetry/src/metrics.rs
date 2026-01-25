//! Metrics collection for game telemetry.
//!
//! Provides counters, gauges, histograms, and a metrics registry for tracking
//! game performance and statistics.

use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// Simple counter metric that only goes up.
#[derive(Debug)]
pub struct Counter {
    value: AtomicU64,
    name: String,
    description: String,
}

impl Counter {
    /// Create a new counter with name and description.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            value: AtomicU64::new(0),
            name: name.into(),
            description: description.into(),
        }
    }

    /// Create a new counter with just a name.
    pub fn named(name: impl Into<String>) -> Self {
        Self::new(name, "")
    }

    /// Increment the counter by 1.
    pub fn inc(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }

    /// Add to the counter.
    pub fn add(&self, n: u64) {
        self.value.fetch_add(n, Ordering::Relaxed);
    }

    /// Get the current value.
    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }

    /// Reset the counter to zero.
    pub fn reset(&self) {
        self.value.store(0, Ordering::Relaxed);
    }

    /// Get the counter name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the counter description.
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new("counter", "")
    }
}

/// Gauge metric that can go up or down.
#[derive(Debug)]
pub struct Gauge {
    value: AtomicI64,
    name: String,
    description: String,
}

impl Gauge {
    /// Create a new gauge with name and description.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            value: AtomicI64::new(0),
            name: name.into(),
            description: description.into(),
        }
    }

    /// Create a new gauge with just a name.
    pub fn named(name: impl Into<String>) -> Self {
        Self::new(name, "")
    }

    /// Set the gauge to a specific value.
    pub fn set(&self, val: i64) {
        self.value.store(val, Ordering::Relaxed);
    }

    /// Increment the gauge by 1.
    pub fn inc(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrement the gauge by 1.
    pub fn dec(&self) {
        self.value.fetch_sub(1, Ordering::Relaxed);
    }

    /// Add to the gauge.
    pub fn add(&self, n: i64) {
        self.value.fetch_add(n, Ordering::Relaxed);
    }

    /// Subtract from the gauge.
    pub fn sub(&self, n: i64) {
        self.value.fetch_sub(n, Ordering::Relaxed);
    }

    /// Get the current value.
    pub fn get(&self) -> i64 {
        self.value.load(Ordering::Relaxed)
    }

    /// Get the gauge name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the gauge description.
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl Default for Gauge {
    fn default() -> Self {
        Self::new("gauge", "")
    }
}

/// Histogram for tracking distributions of values.
#[derive(Debug)]
pub struct Histogram {
    name: String,
    description: String,
    buckets: Vec<f64>,
    counts: Vec<AtomicU64>,
    sum: AtomicU64,
    count: AtomicU64,
}

impl Histogram {
    /// Create a new histogram with custom buckets.
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        buckets: Vec<f64>,
    ) -> Self {
        let bucket_count = buckets.len();
        let counts = (0..=bucket_count).map(|_| AtomicU64::new(0)).collect();
        Self {
            name: name.into(),
            description: description.into(),
            buckets,
            counts,
            sum: AtomicU64::new(0),
            count: AtomicU64::new(0),
        }
    }

    /// Create a histogram with default buckets for timing (in milliseconds).
    pub fn with_default_buckets(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(
            name,
            description,
            vec![1.0, 5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0, 5000.0],
        )
    }

    /// Record a value.
    pub fn observe(&self, value: f64) {
        // Find the appropriate bucket
        for (i, &bound) in self.buckets.iter().enumerate() {
            if value <= bound {
                self.counts[i].fetch_add(1, Ordering::Relaxed);
            }
        }
        // Always increment the +Inf bucket (last one)
        self.counts[self.buckets.len()].fetch_add(1, Ordering::Relaxed);

        // Update sum and count (using fixed-point for sum)
        self.sum
            .fetch_add((value * 1000.0) as u64, Ordering::Relaxed);
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    /// Get the total count of observations.
    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }

    /// Get the sum of all observations.
    pub fn sum(&self) -> f64 {
        self.sum.load(Ordering::Relaxed) as f64 / 1000.0
    }

    /// Get the mean of all observations.
    pub fn mean(&self) -> f64 {
        let count = self.count();
        if count == 0 {
            0.0
        } else {
            self.sum() / count as f64
        }
    }

    /// Get bucket counts.
    pub fn bucket_counts(&self) -> Vec<(f64, u64)> {
        self.buckets
            .iter()
            .enumerate()
            .map(|(i, &bound)| (bound, self.counts[i].load(Ordering::Relaxed)))
            .collect()
    }

    /// Get the histogram name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the histogram description.
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// Timer for measuring operation duration.
pub struct Timer {
    start: Instant,
    histogram: Arc<Histogram>,
}

impl Timer {
    /// Start a new timer.
    pub fn start(histogram: Arc<Histogram>) -> Self {
        Self {
            start: Instant::now(),
            histogram,
        }
    }

    /// Stop the timer and record the duration.
    pub fn stop(self) -> Duration {
        let elapsed = self.start.elapsed();
        self.histogram.observe(elapsed.as_secs_f64() * 1000.0);
        elapsed
    }

    /// Get elapsed time without stopping.
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

/// Metrics registry for managing all metrics.
#[derive(Default)]
pub struct MetricsRegistry {
    counters: RwLock<HashMap<String, Arc<Counter>>>,
    gauges: RwLock<HashMap<String, Arc<Gauge>>>,
    histograms: RwLock<HashMap<String, Arc<Histogram>>>,
}

impl MetricsRegistry {
    /// Create a new metrics registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register or get a counter.
    pub fn counter(&self, name: &str, description: &str) -> Arc<Counter> {
        let mut counters = self.counters.write().unwrap();
        counters
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Counter::new(name, description)))
            .clone()
    }

    /// Register or get a gauge.
    pub fn gauge(&self, name: &str, description: &str) -> Arc<Gauge> {
        let mut gauges = self.gauges.write().unwrap();
        gauges
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Gauge::new(name, description)))
            .clone()
    }

    /// Register or get a histogram.
    pub fn histogram(&self, name: &str, description: &str) -> Arc<Histogram> {
        let mut histograms = self.histograms.write().unwrap();
        histograms
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Histogram::with_default_buckets(name, description)))
            .clone()
    }

    /// Get all counter values as a map.
    pub fn counter_values(&self) -> HashMap<String, u64> {
        let counters = self.counters.read().unwrap();
        counters.iter().map(|(k, v)| (k.clone(), v.get())).collect()
    }

    /// Get all gauge values as a map.
    pub fn gauge_values(&self) -> HashMap<String, i64> {
        let gauges = self.gauges.read().unwrap();
        gauges.iter().map(|(k, v)| (k.clone(), v.get())).collect()
    }

    /// Reset all metrics.
    pub fn reset(&self) {
        let counters = self.counters.read().unwrap();
        for counter in counters.values() {
            counter.reset();
        }
        let gauges = self.gauges.read().unwrap();
        for gauge in gauges.values() {
            gauge.set(0);
        }
    }

    /// Get metrics summary as formatted string.
    pub fn summary(&self) -> String {
        let mut output = String::new();
        output.push_str("=== Metrics Summary ===\n");

        let counters = self.counters.read().unwrap();
        if !counters.is_empty() {
            output.push_str("\nCounters:\n");
            for (name, counter) in counters.iter() {
                output.push_str(&format!("  {}: {}\n", name, counter.get()));
            }
        }

        let gauges = self.gauges.read().unwrap();
        if !gauges.is_empty() {
            output.push_str("\nGauges:\n");
            for (name, gauge) in gauges.iter() {
                output.push_str(&format!("  {}: {}\n", name, gauge.get()));
            }
        }

        let histograms = self.histograms.read().unwrap();
        if !histograms.is_empty() {
            output.push_str("\nHistograms:\n");
            for (name, histogram) in histograms.iter() {
                output.push_str(&format!(
                    "  {}: count={}, mean={:.2}ms\n",
                    name,
                    histogram.count(),
                    histogram.mean()
                ));
            }
        }

        output
    }
}

/// Game-specific metrics.
pub struct GameMetrics {
    pub registry: MetricsRegistry,
    pub matches_simulated: Arc<Counter>,
    pub days_advanced: Arc<Counter>,
    pub saves_written: Arc<Counter>,
    pub saves_loaded: Arc<Counter>,
    pub active_players: Arc<Gauge>,
    pub active_clubs: Arc<Gauge>,
    pub transfer_count: Arc<Counter>,
    pub match_duration: Arc<Histogram>,
    pub save_duration: Arc<Histogram>,
}

impl GameMetrics {
    /// Create new game metrics.
    pub fn new() -> Self {
        let registry = MetricsRegistry::new();
        Self {
            matches_simulated: registry.counter("matches_simulated", "Total matches simulated"),
            days_advanced: registry.counter("days_advanced", "Total days advanced in game"),
            saves_written: registry.counter("saves_written", "Total saves written"),
            saves_loaded: registry.counter("saves_loaded", "Total saves loaded"),
            active_players: registry.gauge("active_players", "Current active players in game"),
            active_clubs: registry.gauge("active_clubs", "Current active clubs in game"),
            transfer_count: registry.counter("transfer_count", "Total transfers completed"),
            match_duration: registry.histogram("match_duration_ms", "Match simulation duration"),
            save_duration: registry.histogram("save_duration_ms", "Save operation duration"),
            registry,
        }
    }

    /// Record a match simulation.
    pub fn record_match(&self, duration_ms: f64) {
        self.matches_simulated.inc();
        self.match_duration.observe(duration_ms);
    }

    /// Record a save operation.
    pub fn record_save(&self, duration_ms: f64) {
        self.saves_written.inc();
        self.save_duration.observe(duration_ms);
    }

    /// Start a timer for match simulation.
    pub fn time_match(&self) -> Timer {
        Timer::start(self.match_duration.clone())
    }

    /// Start a timer for save operation.
    pub fn time_save(&self) -> Timer {
        Timer::start(self.save_duration.clone())
    }

    /// Get a summary of all metrics.
    pub fn summary(&self) -> String {
        self.registry.summary()
    }
}

impl Default for GameMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Global metrics instance (lazy static alternative).
pub mod global {
    use super::*;
    use std::sync::OnceLock;

    static METRICS: OnceLock<GameMetrics> = OnceLock::new();

    /// Get or initialize the global metrics.
    pub fn metrics() -> &'static GameMetrics {
        METRICS.get_or_init(GameMetrics::new)
    }

    /// Record a match simulation.
    pub fn record_match(duration_ms: f64) {
        metrics().record_match(duration_ms);
    }

    /// Record a save operation.
    pub fn record_save(duration_ms: f64) {
        metrics().record_save(duration_ms);
    }

    /// Increment days advanced.
    pub fn advance_day() {
        metrics().days_advanced.inc();
    }

    /// Get metrics summary.
    pub fn summary() -> String {
        metrics().summary()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_basic() {
        let counter = Counter::new("test_counter", "A test counter");
        assert_eq!(counter.get(), 0);
        counter.inc();
        assert_eq!(counter.get(), 1);
        counter.add(10);
        assert_eq!(counter.get(), 11);
    }

    #[test]
    fn test_counter_reset() {
        let counter = Counter::named("test");
        counter.add(100);
        assert_eq!(counter.get(), 100);
        counter.reset();
        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn test_counter_name_description() {
        let counter = Counter::new("my_counter", "Counts things");
        assert_eq!(counter.name(), "my_counter");
        assert_eq!(counter.description(), "Counts things");
    }

    #[test]
    fn test_gauge_basic() {
        let gauge = Gauge::new("test_gauge", "A test gauge");
        assert_eq!(gauge.get(), 0);
        gauge.set(50);
        assert_eq!(gauge.get(), 50);
        gauge.inc();
        assert_eq!(gauge.get(), 51);
        gauge.dec();
        assert_eq!(gauge.get(), 50);
    }

    #[test]
    fn test_gauge_add_sub() {
        let gauge = Gauge::named("test");
        gauge.add(100);
        assert_eq!(gauge.get(), 100);
        gauge.sub(30);
        assert_eq!(gauge.get(), 70);
    }

    #[test]
    fn test_gauge_negative() {
        let gauge = Gauge::named("test");
        gauge.set(-10);
        assert_eq!(gauge.get(), -10);
        gauge.add(5);
        assert_eq!(gauge.get(), -5);
    }

    #[test]
    fn test_histogram_basic() {
        let histogram = Histogram::with_default_buckets("test_hist", "A test histogram");
        assert_eq!(histogram.count(), 0);
        assert_eq!(histogram.sum(), 0.0);

        histogram.observe(10.0);
        histogram.observe(20.0);
        histogram.observe(30.0);

        assert_eq!(histogram.count(), 3);
        assert!((histogram.sum() - 60.0).abs() < 0.01);
        assert!((histogram.mean() - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_histogram_buckets() {
        let histogram = Histogram::new("test", "", vec![10.0, 50.0, 100.0]);
        histogram.observe(5.0);
        histogram.observe(25.0);
        histogram.observe(75.0);
        histogram.observe(150.0);

        let buckets = histogram.bucket_counts();
        assert_eq!(buckets.len(), 3);
        // 5.0 falls into bucket <=10
        assert!(buckets[0].1 >= 1);
    }

    #[test]
    fn test_metrics_registry() {
        let registry = MetricsRegistry::new();

        let counter1 = registry.counter("my_counter", "A counter");
        let counter2 = registry.counter("my_counter", "A counter");

        counter1.inc();
        assert_eq!(counter2.get(), 1); // Same counter

        let gauge = registry.gauge("my_gauge", "A gauge");
        gauge.set(42);

        let values = registry.counter_values();
        assert_eq!(values.get("my_counter"), Some(&1));

        let gauge_values = registry.gauge_values();
        assert_eq!(gauge_values.get("my_gauge"), Some(&42));
    }

    #[test]
    fn test_metrics_registry_summary() {
        let registry = MetricsRegistry::new();
        let counter = registry.counter("test_counter", "desc");
        counter.add(5);
        let gauge = registry.gauge("test_gauge", "desc");
        gauge.set(10);

        let summary = registry.summary();
        assert!(summary.contains("test_counter"));
        assert!(summary.contains("test_gauge"));
    }

    #[test]
    fn test_game_metrics() {
        let metrics = GameMetrics::new();

        metrics.matches_simulated.add(5);
        metrics.days_advanced.add(100);
        metrics.active_players.set(500);
        metrics.active_clubs.set(20);

        assert_eq!(metrics.matches_simulated.get(), 5);
        assert_eq!(metrics.days_advanced.get(), 100);
        assert_eq!(metrics.active_players.get(), 500);
        assert_eq!(metrics.active_clubs.get(), 20);
    }

    #[test]
    fn test_game_metrics_record_match() {
        let metrics = GameMetrics::new();
        metrics.record_match(50.0);
        metrics.record_match(75.0);

        assert_eq!(metrics.matches_simulated.get(), 2);
        assert_eq!(metrics.match_duration.count(), 2);
    }

    #[test]
    fn test_timer() {
        let histogram = Arc::new(Histogram::with_default_buckets("timer_test", ""));
        let timer = Timer::start(histogram.clone());
        std::thread::sleep(std::time::Duration::from_millis(10));
        let duration = timer.stop();

        assert!(duration.as_millis() >= 10);
        assert_eq!(histogram.count(), 1);
    }

    #[test]
    fn test_global_metrics() {
        global::advance_day();
        global::advance_day();
        global::record_match(100.0);

        // Just verify it doesn't panic
        let _ = global::summary();
    }

    #[test]
    fn test_counter_thread_safety() {
        use std::thread;

        let counter = Arc::new(Counter::named("thread_test"));
        let mut handles = vec![];

        for _ in 0..10 {
            let c = counter.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..1000 {
                    c.inc();
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.get(), 10000);
    }
}
