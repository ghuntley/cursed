/// Comprehensive Profiling Infrastructure Tests for CURSED vibecheck
/// 
/// Tests the complete profiling system including memory profiling, CPU profiling,
/// profile data structures, reporting, and integration points.

use cursed::stdlib::vibecheck::*;
use cursed::error::Error;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

#[test]
fn test_memory_profiler_basic_functionality() {
    let profiler = get_memory_profiler();
    
    // Clear any existing data
    clear_memory_profile().unwrap();
    
    // Test allocation recording
    profiler.record_allocation(0x1000, 1024, Some("TestObject".to_string())).unwrap();
    profiler.record_allocation(0x2000, 2048, Some("AnotherObject".to_string())).unwrap();
    
    // Get memory statistics
    let stats = memory_profile().unwrap();
    
    assert!(stats.heap_analysis.current_allocated >= 1024);
    assert!(stats.heap_analysis.total_allocated >= 3072);
    assert!(stats.heap_analysis.active_allocations >= 2);
    
    // Test deallocation
    profiler.record_deallocation(0x1000).unwrap();
    let stats_after = memory_profile().unwrap();
    
    assert!(stats_after.heap_analysis.current_allocated < stats.heap_analysis.current_allocated);
    assert!(stats_after.heap_analysis.total_freed >= 1024);
    
    // Test heap analysis
    let heap_analysis = profiler.analyze_heap().unwrap();
    assert!(!heap_analysis.size_distribution.is_empty());
    assert!(heap_analysis.fragmentation_ratio >= 0.0);
    
    // Test allocation patterns
    let patterns = profiler.analyze_patterns().unwrap();
    assert!(patterns.avg_allocation_size > 0.0);
    assert!(!patterns.common_sizes.is_empty());
}

#[test]
fn test_memory_leak_detection() {
    let mut config = MemoryProfilerConfig::default();
    config.leak_threshold = Duration::from_millis(100);
    configure_memory_profiler(config).unwrap();
    
    let profiler = get_memory_profiler();
    
    // Record allocation but don't deallocate
    profiler.record_allocation(0x3000, 4096, Some("LeakyObject".to_string())).unwrap();
    
    // Initially no leaks detected
    let leaks = detect_memory_leaks().unwrap();
    assert_eq!(leaks.len(), 0);
    
    // Wait for leak threshold
    thread::sleep(Duration::from_millis(150));
    
    // Now should detect leak
    let leaks = detect_memory_leaks().unwrap();
    assert!(leaks.len() > 0);
    assert_eq!(leaks[0].allocation.size, 4096);
    assert!(leaks[0].age >= Duration::from_millis(100));
}

#[test]
fn test_cpu_profiler_basic_functionality() {
    let profiler = get_cpu_profiler();
    
    // Start CPU profiling
    profiler.start().unwrap();
    
    // Record function calls
    profiler.enter_function("test_function_1".to_string(), "test_module".to_string()).unwrap();
    thread::sleep(Duration::from_millis(10));
    
    profiler.enter_function("nested_function".to_string(), "test_module".to_string()).unwrap();
    thread::sleep(Duration::from_millis(5));
    profiler.exit_function().unwrap();
    
    profiler.exit_function().unwrap();
    
    // Stop and get profile
    let profile = profiler.stop().unwrap();
    
    assert!(profile.profiling_duration > Duration::from_millis(10));
    assert!(!profile.function_calls.is_empty());
    assert!(!profile.call_graph.is_empty());
    
    // Check call graph relationships
    assert!(profile.call_graph.contains_key("test_function_1"));
    let parent_node = &profile.call_graph["test_function_1"];
    assert!(parent_node.callees.contains_key("nested_function"));
}

#[test]
fn test_cpu_profiler_sampling() {
    let mut config = CpuProfilerConfig::default();
    config.sample_rate = 1000; // High sample rate for testing
    config.function_tracing = false; // Disable function tracing to test sampling
    configure_cpu_profiler(config).unwrap();
    
    start_cpu_profiling().unwrap();
    
    // Do some work to generate samples
    for _ in 0..100 {
        thread::sleep(Duration::from_micros(100));
    }
    
    let profile = stop_cpu_profiling().unwrap();
    
    // Should have collected some samples
    assert!(profile.total_samples > 0);
    assert!(!profile.samples.is_empty());
}

#[test]
fn test_cpu_profiler_bottleneck_detection() {
    let profiler = get_cpu_profiler();
    profiler.start().unwrap();
    
    // Simulate CPU-intensive function with many calls
    for _ in 0..200 {
        profiler.enter_function("cpu_intensive".to_string(), "test".to_string()).unwrap();
        thread::sleep(Duration::from_micros(500));
        profiler.exit_function().unwrap();
    }
    
    let profile = profiler.stop().unwrap();
    
    // Should detect high call frequency bottleneck
    let bottlenecks = profile.bottlenecks;
    if !bottlenecks.is_empty() {
        let has_high_frequency = bottlenecks.iter().any(|b| 
            b.location.contains("cpu_intensive")
        );
        assert!(has_high_frequency);
    }
}

#[test]
fn test_profile_data_creation_and_serialization() {
    let mut profile_data = ProfileData::new("test_session".to_string(), "test_app".to_string());
    
    // Add custom metrics
    profile_data.add_custom_metric("test_counter".to_string(), MetricValue::Counter(42));
    profile_data.add_custom_metric("test_float".to_string(), MetricValue::Float(3.14));
    profile_data.add_custom_metric("test_string".to_string(), MetricValue::String("hello".to_string()));
    
    // Finalize profile
    profile_data.finalize();
    
    assert!(profile_data.metadata.end_time.is_some());
    assert!(profile_data.metadata.duration.is_some());
    assert_eq!(profile_data.custom_metrics.len(), 3);
    
    // Test JSON serialization
    let json = profile_data.to_json().unwrap();
    assert!(json.contains("test_session"));
    assert!(json.contains("test_counter"));
    
    // Test deserialization
    let restored = ProfileData::from_json(&json).unwrap();
    assert_eq!(restored.metadata.session_id, profile_data.metadata.session_id);
    assert_eq!(restored.custom_metrics.len(), 3);
}

#[test]
fn test_profile_data_merge() {
    let mut profile1 = ProfileData::new("session1".to_string(), "app".to_string());
    let mut profile2 = ProfileData::new("session2".to_string(), "app".to_string());
    
    profile1.add_custom_metric("metric1".to_string(), MetricValue::Counter(10));
    profile2.add_custom_metric("metric2".to_string(), MetricValue::Counter(20));
    
    profile1.metadata.duration = Some(Duration::from_secs(5));
    profile2.metadata.duration = Some(Duration::from_secs(3));
    
    profile1.merge(&profile2).unwrap();
    
    assert_eq!(profile1.custom_metrics.len(), 2);
    assert_eq!(profile1.metadata.duration.unwrap(), Duration::from_secs(8));
}

#[test]
fn test_profile_report_generation() {
    let mut profile_data = ProfileData::new("report_test".to_string(), "test_app".to_string());
    
    profile_data.add_custom_metric("test_metric".to_string(), MetricValue::Float(123.45));
    profile_data.finalize();
    
    // Test different report formats
    let mut config = ProfileReportConfig::default();
    
    // Text report
    config.format = ReportFormat::Text;
    let text_report = profile_data.create_report(config.clone()).generate().unwrap();
    assert!(text_report.contains("Profiling Report"));
    assert!(text_report.contains("report_test"));
    assert!(text_report.contains("test_metric"));
    
    // JSON report
    config.format = ReportFormat::Json;
    let json_report = profile_data.create_report(config.clone()).generate().unwrap();
    assert!(json_report.contains("session_id"));
    assert!(json_report.contains("report_test"));
    
    // Markdown report
    config.format = ReportFormat::Markdown;
    let md_report = profile_data.create_report(config.clone()).generate().unwrap();
    assert!(md_report.contains("# CURSED Profiling Report"));
    assert!(md_report.contains("report_test"));
    
    // HTML report
    config.format = ReportFormat::Html;
    let html_report = profile_data.create_report(config.clone()).generate().unwrap();
    assert!(html_report.contains("<html>"));
    assert!(html_report.contains("report_test"));
    
    // CSV report
    config.format = ReportFormat::Csv;
    let csv_report = profile_data.create_report(config.clone()).generate().unwrap();
    assert!(csv_report.contains("Function,Module"));
}

#[test]
fn test_main_profiler_interface() {
    let mut config = ProfilerConfig::default();
    config.memory.sample_rate = 1;
    config.cpu.sample_rate = 100;
    config.export_formats = vec![ReportFormat::Text];
    config.session_name = "integration_test".to_string();
    
    configure_profiler(config).unwrap();
    
    // Start profiling
    start_profiling().unwrap();
    
    let profiler = get_profiler();
    
    // Test memory recording
    profiler.record_allocation(0x4000, 1024, Some("IntegrationTest".to_string())).unwrap();
    profiler.record_deallocation(0x4000).unwrap();
    
    // Test CPU recording
    profiler.record_function_entry("integration_function".to_string(), "test".to_string()).unwrap();
    thread::sleep(Duration::from_millis(5));
    profiler.record_function_exit().unwrap();
    
    // Add custom metric
    profiler.add_custom_metric("integration_test".to_string(), MetricValue::Integer(42)).unwrap();
    
    // Get statistics
    let stats = profiling_stats().unwrap();
    assert!(stats.start_time.is_some());
    assert!(stats.duration > Duration::from_millis(0));
    
    // Generate report
    let config = ProfileReportConfig::default();
    let report = generate_profiling_report(config).unwrap();
    assert!(report.contains("integration_test"));
    
    // Stop profiling
    let profile_data = stop_profiling().unwrap();
    assert!(profile_data.metadata.duration.is_some());
    assert!(profile_data.custom_metrics.contains_key("integration_test"));
}

#[test]
fn test_profile_scope_raii() {
    {
        let _scope = ProfileScope::new().unwrap();
        
        // Do some work while profiling is active
        thread::sleep(Duration::from_millis(10));
        
        // Verify profiling is active
        let stats = profiling_stats().unwrap();
        assert!(stats.start_time.is_some());
        
    } // Scope drops here, automatically stopping profiling
    
    // Verify profiling has stopped by trying to start again
    let result = ProfileScope::new();
    assert!(result.is_ok());
}

#[test]
fn test_profiler_hooks() {
    let start_called = Arc::new(AtomicUsize::new(0));
    let stop_called = Arc::new(AtomicUsize::new(0));
    let update_called = Arc::new(AtomicUsize::new(0));
    
    let start_called_clone = start_called.clone();
    let stop_called_clone = stop_called.clone();
    let update_called_clone = update_called.clone();
    
    let profiler = get_profiler();
    
    profiler.set_hooks(
        Some(Box::new(move || {
            start_called_clone.fetch_add(1, Ordering::SeqCst);
        })),
        Some(Box::new(move |_profile_data| {
            stop_called_clone.fetch_add(1, Ordering::SeqCst);
        })),
        Some(Box::new(move |_stats| {
            update_called_clone.fetch_add(1, Ordering::SeqCst);
        })),
    ).unwrap();
    
    profiler.start().unwrap();
    assert_eq!(start_called.load(Ordering::SeqCst), 1);
    
    thread::sleep(Duration::from_millis(10));
    
    profiler.stop().unwrap();
    assert_eq!(stop_called.load(Ordering::SeqCst), 1);
}

#[test]
fn test_concurrent_profiling() {
    let profiler = get_profiler();
    profiler.start().unwrap();
    
    let handles: Vec<_> = (0..4).map(|thread_id| {
        thread::spawn(move || {
            for i in 0..10 {
                // Memory allocations
                let address = (thread_id * 1000 + i) as usize;
                profiler.record_allocation(address, 512, Some(format!("Thread{}", thread_id))).unwrap();
                
                // Function calls
                profiler.record_function_entry(
                    format!("thread_function_{}", thread_id), 
                    "concurrent_test".to_string()
                ).unwrap();
                
                thread::sleep(Duration::from_millis(1));
                
                profiler.record_function_exit().unwrap();
                profiler.record_deallocation(address).unwrap();
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let profile_data = profiler.stop().unwrap();
    
    // Should have recorded activities from all threads
    if let Some(ref memory) = profile_data.memory {
        assert!(memory.total_allocated > 0);
    }
    
    if let Some(ref cpu) = profile_data.cpu {
        assert!(cpu.total_samples > 0 || !cpu.function_stats.is_empty());
    }
}

#[test] 
fn test_memory_profiler_configuration() {
    let config = MemoryProfilerConfig {
        stack_traces: false,
        sample_rate: 2,
        min_tracked_size: 1024,
        max_allocation_records: 100,
        leak_detection: false,
        leak_threshold: Duration::from_secs(30),
    };
    
    configure_memory_profiler(config).unwrap();
    let profiler = get_memory_profiler();
    
    // Test sampling (should only record every 2nd allocation)
    profiler.record_allocation(0x5000, 2048, None).unwrap(); // Not sampled
    profiler.record_allocation(0x6000, 2048, None).unwrap(); // Sampled
    profiler.record_allocation(0x7000, 2048, None).unwrap(); // Not sampled
    profiler.record_allocation(0x8000, 2048, None).unwrap(); // Sampled
    
    let stats = profiler.get_memory_stats().unwrap();
    // Should have recorded 2 allocations due to sampling
    assert_eq!(stats.heap_analysis.active_allocations, 2);
    
    // Test minimum size filtering
    profiler.record_allocation(0x9000, 512, None).unwrap(); // Below min size, should be ignored
    let stats_after = profiler.get_memory_stats().unwrap();
    assert_eq!(stats_after.heap_analysis.active_allocations, 2); // Should still be 2
}

#[test]
fn test_cpu_profiler_configuration() {
    let config = CpuProfilerConfig {
        sample_rate: 50,
        function_tracing: true,
        max_stack_depth: 16,
        min_function_duration: 1000, // 1ms minimum
        max_samples: 1000,
        call_graph: true,
        per_thread_profiling: true,
    };
    
    configure_cpu_profiler(config).unwrap();
    let profiler = get_cpu_profiler();
    
    profiler.start().unwrap();
    
    // Test function duration filtering
    profiler.enter_function("short_function".to_string(), "test".to_string()).unwrap();
    thread::sleep(Duration::from_micros(100)); // Very short duration
    profiler.exit_function().unwrap();
    
    profiler.enter_function("long_function".to_string(), "test".to_string()).unwrap();
    thread::sleep(Duration::from_millis(2)); // Above minimum
    profiler.exit_function().unwrap();
    
    let profile = profiler.stop().unwrap();
    
    // Should only have recorded the long function
    let long_function_found = profile.function_calls.iter()
        .any(|call| call.name == "long_function");
    assert!(long_function_found);
}

#[test]
fn test_system_info_collection() {
    let system_info = SystemInfo::current();
    
    assert!(!system_info.os.is_empty());
    assert!(!system_info.arch.is_empty());
    assert!(system_info.cpu_cores > 0);
    assert!(system_info.total_memory > 0);
    assert!(!system_info.hostname.is_empty());
}

#[test]
fn test_profiler_state_management() {
    let profiler = get_profiler();
    
    // Initially stopped
    let state = profiler.get_state().unwrap();
    assert_eq!(state, ProfilerState::Stopped);
    
    // Start profiling
    profiler.start().unwrap();
    let state = profiler.get_state().unwrap();
    assert_eq!(state, ProfilerState::Running);
    
    // Cannot start while running
    let result = profiler.start();
    assert!(result.is_err());
    
    // Stop profiling
    profiler.stop().unwrap();
    let state = profiler.get_state().unwrap();
    assert_eq!(state, ProfilerState::Stopped);
    
    // Cannot stop while stopped
    let result = profiler.stop();
    assert!(result.is_err());
}

#[test]
fn test_metric_value_types() {
    let mut profile_data = ProfileData::new("metrics_test".to_string(), "test".to_string());
    
    // Test all metric value types
    profile_data.add_custom_metric("integer_metric".to_string(), MetricValue::Integer(-42));
    profile_data.add_custom_metric("float_metric".to_string(), MetricValue::Float(3.14159));
    profile_data.add_custom_metric("string_metric".to_string(), MetricValue::String("test_value".to_string()));
    profile_data.add_custom_metric("duration_metric".to_string(), MetricValue::Duration(Duration::from_secs(10)));
    profile_data.add_custom_metric("counter_metric".to_string(), MetricValue::Counter(12345));
    profile_data.add_custom_metric("histogram_metric".to_string(), 
        MetricValue::Histogram(vec![(1.0, 10), (2.0, 20), (3.0, 15)]));
    
    assert_eq!(profile_data.custom_metrics.len(), 6);
    
    // Test serialization with all metric types
    let json = profile_data.to_json().unwrap();
    let restored = ProfileData::from_json(&json).unwrap();
    assert_eq!(restored.custom_metrics.len(), 6);
}

#[test]
fn test_profiler_error_handling() {
    let profiler = get_profiler();
    
    // Test recording without starting profiler
    let result = profiler.record_allocation(0x1000, 1024, None);
    // Should not error, but should not record anything
    assert!(result.is_ok());
    
    // Test stopping without starting
    let result = profiler.stop();
    assert!(result.is_err());
    
    // Test double start
    profiler.start().unwrap();
    let result = profiler.start();
    assert!(result.is_err());
    
    profiler.stop().unwrap();
}
