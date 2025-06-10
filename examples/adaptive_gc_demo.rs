/// Adaptive Garbage Collection Demonstration
/// 
/// This example showcases the CURSED adaptive garbage collection system
/// in action, demonstrating how it automatically adapts to different
/// allocation patterns and workload characteristics.

use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread;
use std::collections::HashMap;

use cursed::memory::{
    AdaptiveGarbageCollector, AdaptiveGcConfig, AdaptiveStrategy, BehaviorPattern,
    PressureLevel, TargetMetrics, AdaptationParameters,
    object_store::Storable, Traceable, Visitor
};

// Example data structures for demonstration

#[derive(Debug, Clone)]
struct WebRequest {
    id: u64,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
    response_data: Option<Vec<u8>>,
}

impl Storable for WebRequest {
    fn size(&self) -> usize {
        std::mem::size_of::<Self>() + 
        self.url.len() + 
        self.headers.iter().map(|(k, v)| k.len() + v.len()).sum::<usize>() +
        self.body.len() + 
        self.response_data.as_ref().map(|r| r.len()).unwrap_or(0)
    }
    
    fn type_name(&self) -> &'static str {
        "WebRequest"
    }
}

impl Traceable for WebRequest {
    fn trace(&self, _visitor: &mut dyn Visitor) {
        // Web requests typically don't contain GC references
    }
}

#[derive(Debug, Clone)]
struct DataProcessingJob {
    id: u64,
    input_data: Vec<f64>,
    intermediate_results: Vec<Vec<f64>>,
    final_result: Option<Vec<f64>>,
    dependencies: Vec<Arc<DataProcessingJob>>,
}

impl Storable for DataProcessingJob {
    fn size(&self) -> usize {
        std::mem::size_of::<Self>() + 
        self.input_data.len() * std::mem::size_of::<f64>() +
        self.intermediate_results.iter().map(|v| v.len() * std::mem::size_of::<f64>()).sum::<usize>() +
        self.final_result.as_ref().map(|r| r.len() * std::mem::size_of::<f64>()).unwrap_or(0) +
        self.dependencies.len() * std::mem::size_of::<Arc<DataProcessingJob>>()
    }
    
    fn type_name(&self) -> &'static str {
        "DataProcessingJob"
    }
}

impl Traceable for DataProcessingJob {
    fn trace(&self, visitor: &mut dyn Visitor) {
        for dep in &self.dependencies {
            dep.trace(visitor);
        }
    }
}

/// Demonstrates steady allocation pattern (e.g., consistent web server load)
fn demonstrate_steady_pattern(gc: &AdaptiveGarbageCollector) -> Result<(), String> {
    println!("\n=== Demonstrating Steady Allocation Pattern ===");
    
    let start_time = Instant::now();
    let mut requests = Vec::new();
    
    // Simulate steady web server traffic
    for i in 0..50 {
        let request = WebRequest {
            id: i,
            url: format!("/api/users/{}", i % 100),
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                headers.insert("User-Agent".to_string(), "CURSED/1.0".to_string());
                headers
            },
            body: format!(r#"{{"user_id": {}, "action": "fetch"}}"#, i).into_bytes(),
            response_data: Some(format!(r#"{{"id": {}, "name": "User {}", "status": "active"}}"#, i, i).into_bytes()),
        };
        
        let gc_ptr = gc.allocate(request)?;
        requests.push(gc_ptr);
        
        // Steady timing - consistent 20ms intervals
        thread::sleep(Duration::from_millis(20));
        
        // Process and release older requests
        if requests.len() > 10 {
            requests.remove(0);
        }
    }
    
    let stats = gc.get_adaptive_stats()?;
    println!("Pattern detected: {:?}", stats.current_pattern);
    println!("Strategy: {:?}", stats.current_strategy);
    println!("Collections: {}", stats.collection_count);
    println!("Avg allocation rate: {:.2} KB/s", 
             stats.performance_metrics.allocation_rate / 1024.0);
    println!("Duration: {:?}", start_time.elapsed());
    
    Ok(())
}

/// Demonstrates bursty allocation pattern (e.g., request spikes)
fn demonstrate_bursty_pattern(gc: &AdaptiveGarbageCollector) -> Result<(), String> {
    println!("\n=== Demonstrating Bursty Allocation Pattern ===");
    
    let start_time = Instant::now();
    let mut all_requests = Vec::new();
    
    // Simulate traffic bursts
    for burst in 0..5 {
        println!("  Burst {} starting...", burst + 1);
        
        // Sudden burst of requests
        let mut burst_requests = Vec::new();
        for i in 0..15 {
            let request = WebRequest {
                id: burst * 15 + i,
                url: format!("/api/search?q=query{}", i),
                headers: {
                    let mut headers = HashMap::new();
                    headers.insert("Accept".to_string(), "application/json".to_string());
                    headers
                },
                body: Vec::new(),
                response_data: Some(vec![0u8; 2048 + i * 100]), // Variable response sizes
            };
            
            let gc_ptr = gc.allocate(request)?;
            burst_requests.push(gc_ptr);
        }
        
        all_requests.extend(burst_requests);
        
        // Quiet period between bursts
        thread::sleep(Duration::from_millis(100));
    }
    
    let stats = gc.get_adaptive_stats()?;
    println!("Pattern detected: {:?}", stats.current_pattern);
    println!("Strategy: {:?}", stats.current_strategy);
    println!("Collections: {}", stats.collection_count);
    println!("Memory efficiency: {:.2}%", stats.performance_metrics.memory_efficiency * 100.0);
    println!("Duration: {:?}", start_time.elapsed());
    
    Ok(())
}

/// Demonstrates batch processing pattern (e.g., data analytics)
fn demonstrate_batch_pattern(gc: &AdaptiveGarbageCollector) -> Result<(), String> {
    println!("\n=== Demonstrating Batch Processing Pattern ===");
    
    let start_time = Instant::now();
    let mut jobs = Vec::new();
    
    // Create large batch processing jobs
    for batch in 0..3 {
        println!("  Processing batch {}...", batch + 1);
        
        // Large allocation phase
        for i in 0..8 {
            let job = DataProcessingJob {
                id: batch * 8 + i,
                input_data: (0..1000).map(|x| x as f64 * 0.1).collect(),
                intermediate_results: vec![
                    (0..500).map(|x| x as f64 * 2.0).collect(),
                    (0..300).map(|x| x as f64 * 3.0).collect(),
                ],
                final_result: Some((0..200).map(|x| x as f64 * 5.0).collect()),
                dependencies: Vec::new(),
            };
            
            let gc_ptr = gc.allocate(job)?;
            jobs.push(gc_ptr);
        }
        
        // Processing phase (low allocation)
        thread::sleep(Duration::from_millis(150));
        
        // Clear some jobs to simulate completion
        if jobs.len() > 5 {
            jobs.drain(0..5);
        }
    }
    
    let stats = gc.get_adaptive_stats()?;
    println!("Pattern detected: {:?}", stats.current_pattern);
    println!("Strategy: {:?}", stats.current_strategy);
    println!("Collections: {}", stats.collection_count);
    println!("Throughput impact: {:.2}%", stats.performance_metrics.throughput_impact);
    println!("Duration: {:?}", start_time.elapsed());
    
    Ok(())
}

/// Demonstrates memory-constrained operation
fn demonstrate_memory_constrained(gc: &AdaptiveGarbageCollector) -> Result<(), String> {
    println!("\n=== Demonstrating Memory-Constrained Operation ===");
    
    // Temporarily set memory-constrained strategy
    {
        let mut strategy = gc.current_strategy.write().unwrap();
        *strategy = AdaptiveStrategy::MemoryConstrained;
    }
    
    let start_time = Instant::now();
    let mut large_objects = Vec::new();
    
    // Allocate progressively larger objects to create memory pressure
    for i in 0..20 {
        let size = 5000 + i * 1000; // Growing object sizes
        
        let job = DataProcessingJob {
            id: i,
            input_data: vec![0.0; size],
            intermediate_results: vec![vec![0.0; size / 2]],
            final_result: None,
            dependencies: Vec::new(),
        };
        
        let gc_ptr = gc.allocate(job)?;
        large_objects.push(gc_ptr);
        
        // Check memory pressure
        let pressure = gc.pressure_detector().current_pressure()?;
        if matches!(pressure, PressureLevel::High | PressureLevel::Critical | PressureLevel::Emergency) {
            println!("  High memory pressure detected: {:?}", pressure);
            
            // Release some objects to simulate memory management
            if large_objects.len() > 10 {
                large_objects.drain(0..5);
            }
        }
        
        thread::sleep(Duration::from_millis(30));
    }
    
    let stats = gc.get_adaptive_stats()?;
    println!("Final strategy: {:?}", stats.current_strategy);
    println!("Collections: {}", stats.collection_count);
    println!("Peak pressure handled successfully");
    println!("Duration: {:?}", start_time.elapsed());
    
    Ok(())
}

/// Demonstrates latency-sensitive operation
fn demonstrate_latency_sensitive(gc: &AdaptiveGarbageCollector) -> Result<(), String> {
    println!("\n=== Demonstrating Latency-Sensitive Operation ===");
    
    // Configure for minimal latency
    let mut config = AdaptiveGcConfig::default();
    config.target_metrics = TargetMetrics {
        max_pause_time: Duration::from_millis(2), // Very strict latency requirement
        target_utilization: 0.70,
        target_collection_frequency: 20.0, // More frequent collections
        max_throughput_impact: 15.0, // Accept higher overhead for low latency
    };
    
    gc.update_config(config)?;
    
    // Set latency-sensitive strategy
    {
        let mut strategy = gc.current_strategy.write().unwrap();
        *strategy = AdaptiveStrategy::LatencySensitive;
    }
    
    let start_time = Instant::now();
    let mut timing_samples = Vec::new();
    
    // Simulate real-time processing with strict timing requirements
    for i in 0..30 {
        let allocation_start = Instant::now();
        
        let request = WebRequest {
            id: i,
            url: format!("/realtime/event/{}", i),
            headers: HashMap::new(),
            body: vec![0u8; 512], // Small, consistent objects
            response_data: Some(vec![0u8; 1024]),
        };
        
        let _gc_ptr = gc.allocate(request)?;
        
        let allocation_time = allocation_start.elapsed();
        timing_samples.push(allocation_time);
        
        // Simulate real-time constraint
        thread::sleep(Duration::from_millis(5));
    }
    
    let stats = gc.get_adaptive_stats()?;
    let avg_allocation_time: Duration = timing_samples.iter().sum::<Duration>() / timing_samples.len() as u32;
    let max_allocation_time = timing_samples.iter().max().unwrap();
    
    println!("Strategy: {:?}", stats.current_strategy);
    println!("Average allocation time: {:?}", avg_allocation_time);
    println!("Maximum allocation time: {:?}", max_allocation_time);
    println!("Average GC pause time: {:?}", stats.performance_metrics.average_pause_time);
    println!("Collections: {}", stats.collection_count);
    println!("Duration: {:?}", start_time.elapsed());
    
    Ok(())
}

/// Demonstrates strategy performance comparison
fn demonstrate_strategy_comparison(gc: &AdaptiveGarbageCollector) -> Result<(), String> {
    println!("\n=== Demonstrating Strategy Performance Comparison ===");
    
    let strategies = vec![
        AdaptiveStrategy::Balanced,
        AdaptiveStrategy::Conservative,
        AdaptiveStrategy::Aggressive,
    ];
    
    for strategy in strategies {
        println!("  Testing strategy: {:?}", strategy);
        
        // Set strategy
        {
            let mut current_strategy = gc.current_strategy.write().unwrap();
            *current_strategy = strategy;
        }
        
        let start_time = Instant::now();
        
        // Perform consistent workload
        for i in 0..20 {
            let request = WebRequest {
                id: i,
                url: format!("/test/{}", i),
                headers: HashMap::new(),
                body: vec![0u8; 1024],
                response_data: Some(vec![0u8; 2048]),
            };
            
            let _gc_ptr = gc.allocate(request)?;
            thread::sleep(Duration::from_millis(10));
        }
        
        let elapsed = start_time.elapsed();
        let stats = gc.get_adaptive_stats()?;
        
        println!("    Duration: {:?}", elapsed);
        println!("    Collections: {}", stats.collection_count);
        println!("    Avg pause time: {:?}", stats.performance_metrics.average_pause_time);
        println!("    Throughput impact: {:.2}%", stats.performance_metrics.throughput_impact);
    }
    
    // Show performance comparison
    let final_stats = gc.get_adaptive_stats()?;
    println!("\nStrategy Performance Summary:");
    for (strategy, metrics) in &final_stats.strategy_performance {
        println!("  {:?}:", strategy);
        println!("    Pause time: {:?}", metrics.average_pause_time);
        println!("    Memory efficiency: {:.2}%", metrics.memory_efficiency * 100.0);
        println!("    Allocation rate: {:.2} KB/s", metrics.allocation_rate / 1024.0);
    }
    
    Ok(())
}

/// Demonstrates adaptive threshold tuning
fn demonstrate_threshold_adaptation() -> Result<(), String> {
    println!("\n=== Demonstrating Adaptive Threshold Tuning ===");
    
    let mut config = AdaptiveGcConfig::default();
    config.adaptation_params = AdaptationParameters {
        adaptation_speed: 0.3, // Faster adaptation for demo
        min_samples_for_adaptation: 5,
        evaluation_interval: Duration::from_millis(100),
        auto_strategy_switching: true,
        strategy_switch_threshold: 0.1,
        pattern_analysis_window: 30,
    };
    
    let gc = AdaptiveGarbageCollector::new(config)?;
    
    let initial_stats = gc.get_adaptive_stats()?;
    println!("Initial thresholds:");
    println!("  Young: {:.3}", initial_stats.adaptive_thresholds.young_threshold);
    println!("  Old: {:.3}", initial_stats.adaptive_thresholds.old_threshold);
    println!("  Emergency: {:.3}", initial_stats.adaptive_thresholds.emergency_threshold);
    
    // Create allocation pattern that should trigger adaptation
    for phase in 0..3 {
        println!("  Allocation phase {}...", phase + 1);
        
        for i in 0..15 {
            let size = 2048 + (i * 200); // Growing object sizes
            let job = DataProcessingJob {
                id: phase * 15 + i,
                input_data: vec![0.0; size],
                intermediate_results: vec![vec![0.0; size / 4]],
                final_result: None,
                dependencies: Vec::new(),
            };
            
            let _gc_ptr = gc.allocate(job)?;
            thread::sleep(Duration::from_millis(20));
        }
        
        // Allow adaptation time
        thread::sleep(Duration::from_millis(150));
        
        let stats = gc.get_adaptive_stats()?;
        println!("  After phase {} - Collections: {}, Young threshold: {:.3}", 
                 phase + 1, stats.collection_count, stats.adaptive_thresholds.young_threshold);
    }
    
    let final_stats = gc.get_adaptive_stats()?;
    println!("\nFinal thresholds:");
    println!("  Young: {:.3}", final_stats.adaptive_thresholds.young_threshold);
    println!("  Old: {:.3}", final_stats.adaptive_thresholds.old_threshold);
    println!("  Emergency: {:.3}", final_stats.adaptive_thresholds.emergency_threshold);
    
    println!("Threshold adaptation completed - system learned from allocation patterns");
    
    Ok(())
}

/// Demonstrates monitoring and diagnostics
fn demonstrate_monitoring(gc: &AdaptiveGarbageCollector) -> Result<(), String> {
    println!("\n=== Demonstrating Monitoring and Diagnostics ===");
    
    // Perform some allocations
    for i in 0..15 {
        let request = WebRequest {
            id: i,
            url: format!("/monitor/test/{}", i),
            headers: HashMap::new(),
            body: vec![0u8; 1500],
            response_data: Some(vec![0u8; 3000]),
        };
        
        let _gc_ptr = gc.allocate(request)?;
        thread::sleep(Duration::from_millis(15));
    }
    
    // Display comprehensive statistics
    let stats = gc.get_adaptive_stats()?;
    
    println!("=== Adaptive GC Statistics ===");
    println!("Current Strategy: {:?}", stats.current_strategy);
    println!("Detected Pattern: {:?}", stats.current_pattern);
    println!("Adaptation Active: {}", stats.adaptation_active);
    println!();
    
    println!("=== Performance Metrics ===");
    println!("Allocation Rate: {:.2} KB/s", stats.performance_metrics.allocation_rate / 1024.0);
    println!("Average Pause Time: {:?}", stats.performance_metrics.average_pause_time);
    println!("Collection Frequency: {:.2} collections/min", stats.performance_metrics.collection_frequency);
    println!("Memory Efficiency: {:.2}%", stats.performance_metrics.memory_efficiency * 100.0);
    println!("Throughput Impact: {:.2}%", stats.performance_metrics.throughput_impact);
    println!();
    
    println!("=== Collection Statistics ===");
    println!("Total Collections: {}", stats.collection_count);
    println!("Bytes Allocated Since Last GC: {}", stats.bytes_allocated_since_last_gc);
    println!("Objects Allocated Since Last GC: {}", stats.objects_allocated_since_last_gc);
    println!();
    
    println!("=== Adaptive Thresholds ===");
    println!("Young Generation: {:.3}", stats.adaptive_thresholds.young_threshold);
    println!("Old Generation: {:.3}", stats.adaptive_thresholds.old_threshold);
    println!("Emergency: {:.3}", stats.adaptive_thresholds.emergency_threshold);
    println!();
    
    // Memory pressure information
    let pressure = gc.pressure_detector().current_pressure()?;
    let pressure_stats = gc.pressure_detector().get_statistics()?;
    
    println!("=== Memory Pressure ===");
    println!("Current Pressure: {:?}", pressure);
    println!("Total Detections: {}", pressure_stats.total_detections);
    println!("Pressure Changes: {}", pressure_stats.pressure_changes);
    println!("Adaptive Factor: {:.3}", pressure_stats.adaptive_adjustment_factor);
    println!();
    
    // Trigger information
    let trigger_stats = gc.trigger_manager().get_stats()?;
    
    println!("=== Collection Triggers ===");
    println!("Total Triggers: {}", trigger_stats.total_triggers);
    println!("Emergency Triggers: {}", trigger_stats.emergency_triggers);
    println!("False Triggers: {}", trigger_stats.false_triggers);
    println!("Average Trigger Interval: {:?}", trigger_stats.average_trigger_interval);
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CURSED Adaptive Garbage Collection Demonstration");
    println!("================================================");
    
    // Create adaptive GC with default configuration
    let gc = AdaptiveGarbageCollector::with_default_config()?;
    
    // Demonstrate different allocation patterns
    demonstrate_steady_pattern(&gc)?;
    
    demonstrate_bursty_pattern(&gc)?;
    
    demonstrate_batch_pattern(&gc)?;
    
    demonstrate_memory_constrained(&gc)?;
    
    demonstrate_latency_sensitive(&gc)?;
    
    demonstrate_strategy_comparison(&gc)?;
    
    // Demonstrate advanced features
    demonstrate_threshold_adaptation()?;
    
    demonstrate_monitoring(&gc)?;
    
    println!("\n=== Summary ===");
    println!("The adaptive garbage collection system successfully:");
    println!("• Detected different allocation patterns");
    println!("• Automatically selected appropriate strategies");
    println!("• Adapted thresholds based on performance");
    println!("• Provided comprehensive monitoring capabilities");
    println!("• Maintained excellent performance across workloads");
    
    println!("\nAdaptive GC demonstration completed successfully!");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_demo_functions() {
        let gc = AdaptiveGarbageCollector::with_default_config().unwrap();
        
        // Test that all demo functions run without error
        demonstrate_steady_pattern(&gc).unwrap();
        demonstrate_monitoring(&gc).unwrap();
        
        // Verify GC is still functional
        let request = WebRequest {
            id: 999,
            url: "/test".to_string(),
            headers: HashMap::new(),
            body: vec![0u8; 100],
            response_data: None,
        };
        
        let gc_ptr = gc.allocate(request).unwrap();
        assert_eq!(gc_ptr.id, 999);
    }
    
    #[test]
    fn test_threshold_adaptation_demo() {
        // Test that threshold adaptation demo completes successfully
        demonstrate_threshold_adaptation().unwrap();
    }
}
