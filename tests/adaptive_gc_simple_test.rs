/// Comprehensive tests for adaptive garbage collection
/// Tests algorithm adaptation, behavior pattern recognition, and performance optimization

#[path = "common.rs"]
pub mod common;

use cursed::memory::{
    AdaptiveGarbageCollector, AdaptiveGcConfig, AdaptiveStrategy, BehaviorPattern,
    AdaptiveThresholds, PerformanceMetrics, TargetMetrics, CollectionAlgorithm,
    Gc, Tag, Traceable, Visitor
};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{info, debug, error};

macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

#[derive(Debug, Clone)]
struct AdaptiveTestObject {
    pub id: u32,
    pub size: usize,
    pub data: Vec<u8>,
    pub references: Vec<Gc<AdaptiveTestObject>>,
}

impl Traceable for AdaptiveTestObject {
    fn trace(&self, visitor: &mut dyn Visitor) {
        for reference in &self.references {
            reference.trace(visitor);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test adaptive GC initialization and basic configuration
    #[test]
    fn test_adaptive_gc_initialization() {
        init_tracing!();
        info!("Testing adaptive GC initialization");

        let config = AdaptiveGcConfig {
            initial_strategy: AdaptiveStrategy::Conservative,
            adaptation_window: Duration::from_millis(1000),
            performance_threshold: 0.1,
            memory_threshold: 0.8,
            adaptation_sensitivity: 0.5,
            min_adaptation_interval: Duration::from_millis(100),
            behavior_history_size: 10,
            target_metrics: TargetMetrics {
                max_pause_time: Duration::from_millis(50),
                target_throughput: 1000.0,
                max_memory_overhead: 0.2,
                target_collection_frequency: Duration::from_secs(1),
            },
            thresholds: AdaptiveThresholds {
                young_gen: 0.7,
                old_gen: 0.8,
                emergency: 0.95,
                adaptation_trigger: 0.1,
            },
        };

        let adaptive_gc = AdaptiveGarbageCollector::new(config);
        assert!(adaptive_gc.is_ok());

        let gc = adaptive_gc.unwrap();
        let stats = gc.get_stats().unwrap();
        assert_eq!(stats.collections_performed, 0);
        assert_eq!(stats.strategy_switches, 0);

        info!("Adaptive GC initialization test passed");
    } 

    /// Test algorithm adaptation based on allocation patterns
    #[test]
    fn test_algorithm_adaptation() {
        init_tracing!();
        info!("Testing algorithm adaptation");

        let config = AdaptiveGcConfig {
            initial_strategy: AdaptiveStrategy::Conservative,
            adaptation_window: Duration::from_millis(100),
            adaptation_sensitivity: 0.3,
            ..AdaptiveGcConfig::default()
        };

        let gc = AdaptiveGarbageCollector::new(config).unwrap();
        let initial_stats = gc.get_stats().unwrap();
        let initial_strategy = initial_stats.current_strategy;

        // Create allocation pattern that should trigger adaptation
        // High-frequency short-lived objects (should favor copying collector)
        for round in 0..5 {
            debug!("Adaptation test round {}", round);

            for i in 0..200 {
                let obj = AdaptiveTestObject {
                    id: i,
                    size: 64,
                    data: vec![round as u8; 64],
                    references: vec![],
                };
                let _temp = gc.allocate(obj).unwrap();
                // Objects go out of scope quickly
            }

            // Trigger collection to provide adaptation data
            let _result = gc.collect_garbage();
            
            // Check if strategy adapted
            let current_stats = gc.get_stats().unwrap();
            if current_stats.current_strategy != initial_strategy {
                debug!("Strategy adapted from {:?} to {:?}", 
                       initial_strategy, current_stats.current_strategy);
                break;
            }
        }

        let final_stats = gc.get_stats().unwrap();
        // Should have either adapted or shown adaptation consideration
        assert!(final_stats.adaptation_evaluations > 0);

        info!("Algorithm adaptation test passed");
    }

    /// Test behavior pattern recognition
    #[test]
    fn test_behavior_pattern_recognition() {
        init_tracing!();
        info!("Testing behavior pattern recognition");

        let config = AdaptiveGcConfig {
            behavior_history_size: 5,
            adaptation_window: Duration::from_millis(50),
            ..AdaptiveGcConfig::default()
        };

        let gc = AdaptiveGarbageCollector::new(config).unwrap();

        // Create consistent pattern: burst allocation followed by collection
        for cycle in 0..10 {
            debug!("Pattern recognition cycle {}", cycle);

            // Burst allocation phase
            let mut cycle_objects = Vec::new();
            for i in 0..100 {
                let obj = AdaptiveTestObject {
                    id: cycle * 100 + i,
                    size: 128,
                    data: vec![cycle as u8; 128],
                    references: vec![],
                };
                cycle_objects.push(gc.allocate(obj).unwrap());
            }

            // Collection phase
            let _result = gc.collect_garbage();

            // Let half the objects die
            if cycle % 2 == 0 {
                cycle_objects.truncate(50);
            }

            std::thread::sleep(Duration::from_millis(10));
        }

        let stats = gc.get_stats().unwrap();
        assert!(stats.pattern_detections > 0);
        assert!(stats.behavior_samples > 0);

        // Should have recognized some pattern
        assert!(stats.collections_performed >= 5);

        info!("Behavior pattern recognition test passed");
    }

    /// Test performance-based adaptation
    #[test]
    fn test_performance_adaptation() {
        init_tracing!();
        info!("Testing performance-based adaptation");

        let config = AdaptiveGcConfig {
            target_metrics: TargetMetrics {
                max_pause_time: Duration::from_millis(20), // Aggressive target
                target_throughput: 2000.0,
                max_memory_overhead: 0.15,
                target_collection_frequency: Duration::from_millis(500),
            },
            adaptation_sensitivity: 0.7, // High sensitivity
            performance_threshold: 0.05, // Low threshold for quick adaptation
            ..AdaptiveGcConfig::default()
        };

        let gc = AdaptiveGarbageCollector::new(config).unwrap();

        let performance_start = Instant::now();

        // Create workload that challenges performance targets
        for batch in 0..20 {
            let batch_start = Instant::now();

            // Allocate batch of objects
            let mut batch_objects = Vec::new();
            for i in 0..50 {
                let obj = AdaptiveTestObject {
                    id: batch * 50 + i,
                    size: 256,
                    data: vec![batch as u8; 256],
                    references: vec![],
                };
                batch_objects.push(gc.allocate(obj).unwrap());
            }

            let batch_time = batch_start.elapsed();
            debug!("Batch {} allocation time: {:?}", batch, batch_time);

            // Trigger collection
            let collection_start = Instant::now();
            let _result = gc.collect_garbage();
            let collection_time = collection_start.elapsed();
            debug!("Batch {} collection time: {:?}", batch, collection_time);

            // Performance pressure should trigger adaptations
            if collection_time > Duration::from_millis(30) {
                debug!("High collection time detected, should trigger adaptation");
            }
        }

        let total_time = performance_start.elapsed();
        let stats = gc.get_stats().unwrap();

        debug!("Total test time: {:?}", total_time);
        debug!("Performance adaptations: {}", stats.performance_adaptations);
        debug!("Strategy switches: {}", stats.strategy_switches);

        // Should have attempted performance optimizations
        assert!(stats.performance_adaptations > 0 || stats.strategy_switches > 0);

        info!("Performance-based adaptation test passed");
    }

    /// Test memory pressure adaptation
    #[test]
    fn test_memory_pressure_adaptation() {
        init_tracing!();
        info!("Testing memory pressure adaptation");

        let config = AdaptiveGcConfig {
            memory_threshold: 0.3, // Low threshold to trigger quickly
            thresholds: AdaptiveThresholds {
                emergency: 0.7, // Lower emergency threshold
                ..AdaptiveThresholds::default()
            },
            ..AdaptiveGcConfig::default()
        };

        let gc = AdaptiveGarbageCollector::new(config).unwrap();

        // Create memory pressure by allocating large objects
        let mut pressure_objects = Vec::new();
        for i in 0..100 {
            let obj = AdaptiveTestObject {
                id: i,
                size: 4096, // 4KB objects
                data: vec![i as u8; 4096],
                references: vec![],
            };
            pressure_objects.push(gc.allocate(obj).unwrap());

            // Check stats periodically
            if i % 20 == 0 {
                let stats = gc.get_stats().unwrap();
                debug!("Memory pressure cycle {}: {} bytes allocated", 
                       i, stats.total_allocated_bytes);

                // Force collection to trigger adaptation
                let _result = gc.collect_garbage();
            }
        }

        let final_stats = gc.get_stats().unwrap();
        
        // Should have responded to memory pressure
        assert!(final_stats.memory_pressure_adaptations > 0 || 
                final_stats.emergency_collections > 0);
        assert!(final_stats.collections_performed > 0);

        info!("Memory pressure adaptation test passed");
    }

    /// Test strategy switching and stability
    #[test]
    fn test_strategy_switching() {
        init_tracing!();
        info!("Testing strategy switching and stability");

        let config = AdaptiveGcConfig {
            initial_strategy: AdaptiveStrategy::Conservative,
            adaptation_window: Duration::from_millis(200),
            min_adaptation_interval: Duration::from_millis(50),
            adaptation_sensitivity: 0.4,
            ..AdaptiveGcConfig::default()
        };

        let gc = AdaptiveGarbageCollector::new(config).unwrap();

        let mut previous_strategy = AdaptiveStrategy::Conservative;
        let mut strategy_changes = 0;

        // Create varied workload patterns to trigger strategy switches
        for phase in 0..8 {
            debug!("Strategy switching phase {}", phase);

            match phase % 4 {
                0 => {
                    // Many small short-lived objects (favor copying)
                    for i in 0..150 {
                        let obj = AdaptiveTestObject {
                            id: i,
                            size: 32,
                            data: vec![phase as u8; 32],
                            references: vec![],
                        };
                        let _temp = gc.allocate(obj).unwrap();
                    }
                }
                1 => {
                    // Few large long-lived objects (favor mark-sweep)
                    let mut long_lived = Vec::new();
                    for i in 0..20 {
                        let obj = AdaptiveTestObject {
                            id: i + 1000,
                            size: 2048,
                            data: vec![phase as u8; 2048],
                            references: vec![],
                        };
                        long_lived.push(gc.allocate(obj).unwrap());
                    }
                    // Keep them alive longer
                    std::thread::sleep(Duration::from_millis(100));
                }
                2 => {
                    // Mixed size incremental pattern (favor incremental)
                    for i in 0..80 {
                        let size = if i % 3 == 0 { 1024 } else { 128 };
                        let obj = AdaptiveTestObject {
                            id: i + 2000,
                            size,
                            data: vec![phase as u8; size],
                            references: vec![],
                        };
                        let _mixed = gc.allocate(obj).unwrap();
                        
                        if i % 10 == 0 {
                            let _result = gc.collect_garbage();
                        }
                    }
                }
                _ => {
                    // High allocation rate (favor performance)
                    let rapid_start = Instant::now();
                    for i in 0..200 {
                        let obj = AdaptiveTestObject {
                            id: i + 3000,
                            size: 64,
                            data: vec![phase as u8; 64],
                            references: vec![],
                        };
                        let _rapid = gc.allocate(obj).unwrap();
                    }
                    debug!("Rapid allocation phase took: {:?}", rapid_start.elapsed());
                }
            }

            // Trigger collection and check for strategy changes
            let _result = gc.collect_garbage();
            let stats = gc.get_stats().unwrap();
            
            if stats.current_strategy != previous_strategy {
                strategy_changes += 1; 
                debug!("Strategy changed from {:?} to {:?}", 
                       previous_strategy, stats.current_strategy);
                previous_strategy = stats.current_strategy;
            }

            std::thread::sleep(Duration::from_millis(60));
        }

        let final_stats = gc.get_stats().unwrap();
        debug!("Total strategy switches: {}", final_stats.strategy_switches);
        debug!("Strategy changes detected: {}", strategy_changes);

        // Should show adaptive behavior
        assert!(final_stats.strategy_switches > 0 || strategy_changes > 0);
        assert!(final_stats.adaptation_evaluations > 0);

        info!("Strategy switching test passed");
    }

    /// Test adaptation stability and oscillation prevention
    #[test]
    fn test_adaptation_stability() {
        init_tracing!();
        info!("Testing adaptation stability");

        let config = AdaptiveGcConfig {
            min_adaptation_interval: Duration::from_millis(200), // Prevent rapid switching
            adaptation_sensitivity: 0.2, // Lower sensitivity for stability
            behavior_history_size: 15, // Larger history for stable decisions
            ..AdaptiveGcConfig::default()
        };

        let gc = AdaptiveGarbageCollector::new(config).unwrap();

        // Create consistent workload that should reach stable strategy
        for cycle in 0..20 {
            debug!("Stability test cycle {}", cycle);

            // Consistent pattern: moderate allocation with mixed sizes
            for i in 0..60 {
                let size = if i % 4 == 0 { 512 } else { 128 };
                let obj = AdaptiveTestObject {
                    id: cycle * 100 + i,
                    size,
                    data: vec![cycle as u8; size],
                    references: vec![],
                };
                let _consistent = gc.allocate(obj).unwrap();
            }

            let _result = gc.collect_garbage();
            
            // Small delay to provide consistent timing
            std::thread::sleep(Duration::from_millis(25));
        }

        let stats = gc.get_stats().unwrap();
        
        // Should reach stability (fewer switches in later cycles)
        let switch_rate = stats.strategy_switches as f64 / stats.collections_performed as f64;
        debug!("Strategy switch rate: {:.2}", switch_rate);
        
        // Switch rate should be reasonable (not oscillating rapidly)
        assert!(switch_rate < 0.5, "Too many strategy switches, may be oscillating");
        assert!(stats.collections_performed >= 15);

        info!("Adaptation stability test passed");
    }

    /// Stress test with rapid workload changes
    #[test]
    #[ignore] // Run with --ignored flag for stress tests
    fn test_adaptive_stress() {
        init_tracing!();
        info!("Testing adaptive GC under stress");

        let config = AdaptiveGcConfig {
            adaptation_window: Duration::from_millis(50),
            adaptation_sensitivity: 0.6, // High sensitivity for stress test
            target_metrics: TargetMetrics {
                max_pause_time: Duration::from_millis(30),
                target_throughput: 1500.0,
                max_memory_overhead: 0.25,
                target_collection_frequency: Duration::from_millis(200),
            },
            ..AdaptiveGcConfig::default()
        };

        let gc = AdaptiveGarbageCollector::new(config).unwrap();

        // Stress test with rapid pattern changes
        for stress_round in 0..50 {
            debug!("Stress test round {}", stress_round);

            let pattern = stress_round % 5;
            match pattern {
                0 => {
                    // Burst allocation
                    for i in 0..100 {
                        let obj = AdaptiveTestObject {
                            id: i,
                            size: 256,
                            data: vec![stress_round as u8; 256],
                            references: vec![],
                        };
                        let _burst = gc.allocate(obj).unwrap();
                    }
                }
                1 => {
                    // Large objects
                    for i in 0..20 {
                        let obj = AdaptiveTestObject {
                            id: i + 1000,
                            size: 4096,
                            data: vec![stress_round as u8; 4096],
                            references: vec![],
                        };
                        let _large = gc.allocate(obj).unwrap();
                    }
                }
                2 => {
                    // Frequent collections
                    for i in 0..40 {
                        let obj = AdaptiveTestObject {
                            id: i + 2000,
                            size: 128,
                            data: vec![stress_round as u8; 128], 
                            references: vec![],
                        };
                        let _frequent = gc.allocate(obj).unwrap();
                        
                        if i % 5 == 0 {
                            let _result = gc.collect_garbage();
                        }
                    }
                }
                3 => {
                    // Connected objects
                    let mut connected = Vec::new();
                    for i in 0..30 {
                        let obj = AdaptiveTestObject {
                            id: i + 3000,
                            size: 192,
                            data: vec![stress_round as u8; 192],
                            references: vec![],
                        };
                        let gc_obj = gc.allocate(obj).unwrap();
                        if !connected.is_empty() && i % 3 == 0 {
                            let prev_idx = connected.len() - 1;
                            connected[prev_idx].get_mut().unwrap().references.push(gc_obj.clone());
                        }
                        connected.push(gc_obj);
                    }
                }
                _ => {
                    // Mixed workload
                    for i in 0..80 {
                        let size = match i % 4 {
                            0 => 64,
                            1 => 256,
                            2 => 512,
                            _ => 1024,
                        };
                        let obj = AdaptiveTestObject {
                            id: i + 4000,
                            size,
                            data: vec![stress_round as u8; size],
                            references: vec![],
                        };
                        let _mixed = gc.allocate(obj).unwrap();
                    }
                }
            }

            // Force collection and check adaptation
            let collection_start = Instant::now();
            let _result = gc.collect_garbage();
            let collection_time = collection_start.elapsed();

            if collection_time > Duration::from_millis(50) {
                debug!("High collection time in stress round {}: {:?}", 
                       stress_round, collection_time);
            }

            // Brief pause to prevent overwhelming the system
            if stress_round % 10 == 0 {
                std::thread::sleep(Duration::from_millis(5));
            }
        }

        let final_stats = gc.get_stats().unwrap();
        
        // Should have handled stress and adapted
        assert!(final_stats.collections_performed >= 30);
        assert!(final_stats.adaptation_evaluations > 0);
        assert!(final_stats.total_allocated_bytes > 1_000_000); // At least 1MB allocated

        debug!("Stress test final stats: collections={}, adaptations={}, switches={}", 
               final_stats.collections_performed, 
               final_stats.adaptation_evaluations,
               final_stats.strategy_switches);

        info!("Adaptive GC stress test passed");
    }
}
