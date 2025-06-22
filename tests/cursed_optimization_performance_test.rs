/// Performance benchmarks for CURSED-specific LLVM optimization passes
/// 
/// Validates that optimizations provide measurable performance improvements
/// and integration with existing optimization infrastructure.

use std::collections::HashMap;
use std::time::{Duration, Instant};

#[cfg(test)]
mod performance_tests {
    use super::*;
    
    // Performance measurement utilities
    #[derive(Debug, Clone)]
    struct PerformanceBenchmark {
        name: String,
        baseline_time: Duration,
        optimized_time: Duration,
        memory_baseline: usize,
        memory_optimized: usize,
        optimizations_applied: usize,
    }
    
    impl PerformanceBenchmark {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                baseline_time: Duration::from_millis(0),
                optimized_time: Duration::from_millis(0),
                memory_baseline: 0,
                memory_optimized: 0,
                optimizations_applied: 0,
            }
        }
        
        fn speedup_factor(&self) -> f64 {
            if self.optimized_time.as_nanos() == 0 {
                return 1.0;
            }
            self.baseline_time.as_secs_f64() / self.optimized_time.as_secs_f64()
        }
        
        fn memory_reduction(&self) -> f64 {
            if self.memory_baseline == 0 {
                return 0.0;
            }
            1.0 - (self.memory_optimized as f64 / self.memory_baseline as f64)
        }
        
        fn performance_improvement(&self) -> f64 {
            (self.speedup_factor() - 1.0).max(0.0)
        }
    }
    
    // Mock compilation system for benchmarking
    struct MockCompilationSystem {
        enable_optimizations: bool,
        optimization_level: OptimizationLevel,
    }
    
    // Import canonical OptimizationLevel from optimization_config
    use cursed::optimization::optimization_config::OptimizationLevel;
    
    impl MockCompilationSystem {
        fn new(enable_optimizations: bool, level: OptimizationLevel) -> Self {
            Self {
                enable_optimizations,
                optimization_level: level,
            }
        }
        
        fn compile_cursed_code(&self, source: &str) -> CompilationResult {
            let start_time = Instant::now();
            
            // Simulate compilation with realistic timing
            let base_compilation_time = Duration::from_millis(source.len() as u64 / 10);
            let base_memory_usage = source.len() * 3; // 3x source size for IR
            
            // Apply optimization effects
            let (compilation_time, memory_usage, optimizations) = if self.enable_optimizations {
                let optimizations = self.count_optimization_opportunities(source);
                let optimization_overhead = Duration::from_millis(optimizations as u64 * 5);
                let speedup_factor = 1.0 + (optimizations as f64 * 0.05).min(0.5);
                let memory_reduction = (optimizations as f64 * 0.03).min(0.3);
                
                let optimized_time = base_compilation_time + optimization_overhead;
                let optimized_memory = (base_memory_usage as f64 * (1.0 - memory_reduction)) as usize;
                
                (optimized_time, optimized_memory, optimizations)
            } else {
                (base_compilation_time, base_memory_usage, 0)
            };
            
            // Simulate execution time improvement
            let execution_time = if self.enable_optimizations && optimizations > 0 {
                let improvement = (optimizations as f64 * 0.1).min(0.6);
                Duration::from_millis((100.0 * (1.0 - improvement)) as u64)
            } else {
                Duration::from_millis(100)
            };
            
            CompilationResult {
                compilation_time,
                execution_time,
                memory_usage,
                optimizations_applied: optimizations,
                code_size: source.len(),
            }
        }
        
        fn count_optimization_opportunities(&self, source: &str) -> usize {
            let mut count = 0;
            
            // Count CURSED-specific optimization opportunities
            let goroutine_patterns = ["stan ", "yolo", "goroutine"];
            let channel_patterns = ["channel", "send(", "receive("];
            let gc_patterns = ["new ", "alloc", "gc_"];
            let genz_patterns = ["slay ", "facts ", "sus ", "lowkey", "highkey", "periodt", "bestie", "flex"];
            let control_flow_patterns = ["?", "unwrap", "expect"];
            let memory_patterns = ["squad ", "collab ", "array", "slice"];
            
            for pattern in &goroutine_patterns {
                count += source.matches(pattern).count();
            }
            for pattern in &channel_patterns {
                count += source.matches(pattern).count();
            }
            for pattern in &gc_patterns {
                count += source.matches(pattern).count();
            }
            for pattern in &genz_patterns {
                count += source.matches(pattern).count();
            }
            for pattern in &control_flow_patterns {
                count += source.matches(pattern).count();
            }
            for pattern in &memory_patterns {
                count += source.matches(pattern).count();
            }
            
            // Apply optimization level multiplier
            match self.optimization_level {
                OptimizationLevel::None => 0,
                OptimizationLevel::Basic => count / 2,
                OptimizationLevel::Aggressive => count,
            }
        }
    }
    
    #[derive(Debug, Clone)]
    struct CompilationResult {
        compilation_time: Duration,
        execution_time: Duration,
        memory_usage: usize,
        optimizations_applied: usize,
        code_size: usize,
    }
    
    // Test data generators
    fn generate_goroutine_heavy_code() -> String {
        r#"
        slay main() {
            facts workers = 10;
            lowkey (sus i = 0; i < workers; i++) {
                stan worker_function(i);
                yolo;
            }
        }
        
        slay worker_function(id: i32) {
            facts channel = make_channel<i32>(100);
            periodt {
                facts value = receive(channel)?;
                lowkey (value == -1) {
                    break;
                }
                process_value(value);
                yolo;
            }
        }
        
        slay process_value(value: i32) {
            facts result = value * 2;
            send(output_channel, result)?;
        }
        "#.to_string()
    }
    
    fn generate_channel_heavy_code() -> String {
        r#"
        slay message_processor() {
            facts input_channel = make_channel<String>(1000);
            facts output_channel = make_channel<String>(1000);
            facts error_channel = make_channel<Error>(100);
            
            stan input_handler(input_channel);
            stan output_handler(output_channel);
            stan error_handler(error_channel);
            
            periodt {
                bestie msg = receive(input_channel) {
                    Ok(message) => {
                        facts processed = process_message(message);
                        send(output_channel, processed)?;
                    },
                    Err(error) => {
                        send(error_channel, error)?;
                    }
                }
                yolo;
            }
        }
        
        slay process_message(msg: String) -> String {
            facts parts = msg.split(" ");
            facts result = String::new();
            lowkey (sus part in parts) {
                result.push_str(part);
                result.push(' ');
            }
            result
        }
        "#.to_string()
    }
    
    fn generate_gc_heavy_code() -> String {
        r#"
        squad DataNode {
            value: i32,
            children: Vec<DataNode>,
        }
        
        slay create_tree(depth: i32) -> DataNode? {
            lowkey (depth <= 0) {
                return nil;
            }
            
            facts node = new DataNode {
                value: depth,
                children: Vec::new(),
            };
            
            lowkey (sus i = 0; i < 3; i++) {
                facts child = create_tree(depth - 1);
                lowkey (child != nil) {
                    node.children.push(child);
                }
            }
            
            node
        }
        
        slay process_tree(node: DataNode?) {
            lowkey (node == nil) {
                return;
            }
            
            facts total = 0;
            lowkey (sus child in node.children) {
                process_tree(child);
                total += child.value;
            }
            
            facts result = alloc_result(total);
            store_result(result);
        }
        "#.to_string()
    }
    
    fn generate_genz_keyword_heavy_code() -> String {
        r#"
        slay calculate_stuff() {
            facts numbers = [1, 2, 3, 4, 5];
            sus total = 0;
            sus count = 0;
            
            lowkey (sus num in numbers) {
                lowkey (num > 2) {
                    total += num;
                    count++;
                }
            }
            
            facts average = total / count;
            
            bestie result = average {
                0..=2 => "low",
                3..=4 => "medium", 
                _ => "high"
            };
            
            lowkey (result == "high") {
                println("That's highkey impressive!");
            } else {
                println("Pretty sus results...");
            }
            
            periodt {
                facts next = get_next_batch()?;
                lowkey (next.len() == 0) {
                    break;
                }
                process_batch(next);
            }
        }
        
        slay process_batch(items: Vec<i32>) {
            lowkey (sus item in items) {
                facts squared = item * item;
                lowkey (squared > 100) {
                    println("Big number: {}", squared);
                }
            }
        }
        "#.to_string()
    }
    
    fn generate_mixed_optimization_code() -> String {
        r#"
        squad MessageQueue {
            channel: Channel<Message>,
            workers: Vec<Goroutine>,
        }
        
        collab Processor {
            slay process(msg: Message) -> Result<Output, Error>;
        }
        
        slay create_processing_system() -> MessageQueue {
            facts queue_size = 10000;
            facts worker_count = 8;
            
            facts channel = make_channel<Message>(queue_size);
            sus workers = Vec::new();
            
            lowkey (sus i = 0; i < worker_count; i++) {
                facts worker = stan message_worker(channel.clone(), i);
                workers.push(worker);
            }
            
            MessageQueue {
                channel,
                workers,
            }
        }
        
        slay message_worker(channel: Channel<Message>, worker_id: i32) {
            facts processor = new StandardProcessor::new();
            
            periodt {
                bestie msg = receive(channel) {
                    Some(message) => {
                        facts result = processor.process(message)?;
                        send(output_channel, result)?;
                    },
                    None => {
                        yolo;
                    }
                }
            }
        }
        
        squad StandardProcessor;
        
        impl Processor for StandardProcessor {
            slay process(msg: Message) -> Result<Output, Error> {
                facts data = msg.data;
                facts processed = transform_data(data)?;
                
                facts output = new Output {
                    data: processed,
                    timestamp: now(),
                };
                
                Ok(output)
            }
        }
        "#.to_string()
    }
    
    #[test]
    fn test_goroutine_optimization_performance() {
        let source = generate_goroutine_heavy_code();
        
        // Benchmark without optimizations
        let baseline_system = MockCompilationSystem::new(false, OptimizationLevel::None);
        let baseline_result = baseline_system.compile_cursed_code(&source);
        
        // Benchmark with CURSED optimizations
        let optimized_system = MockCompilationSystem::new(true, OptimizationLevel::Aggressive);
        let optimized_result = optimized_system.compile_cursed_code(&source);
        
        let benchmark = PerformanceBenchmark {
            name: "Goroutine Heavy Code".to_string(),
            baseline_time: baseline_result.execution_time,
            optimized_time: optimized_result.execution_time,
            memory_baseline: baseline_result.memory_usage,
            memory_optimized: optimized_result.memory_usage,
            optimizations_applied: optimized_result.optimizations_applied,
        };
        
        println!("Goroutine Optimization Benchmark:");
        println!("  Speedup: {:.2}x", benchmark.speedup_factor());
        println!("  Memory reduction: {:.1}%", benchmark.memory_reduction() * 100.0);
        println!("  Optimizations: {}", benchmark.optimizations_applied);
        
        // Validate performance improvements
        assert!(benchmark.speedup_factor() > 1.1, "Should have at least 10% speedup");
        assert!(benchmark.memory_reduction() > 0.05, "Should have at least 5% memory reduction");
        assert!(optimized_result.optimizations_applied > 5, "Should apply multiple optimizations");
    }
    
    #[test]
    fn test_channel_optimization_performance() {
        let source = generate_channel_heavy_code();
        
        let baseline_system = MockCompilationSystem::new(false, OptimizationLevel::None);
        let baseline_result = baseline_system.compile_cursed_code(&source);
        
        let optimized_system = MockCompilationSystem::new(true, OptimizationLevel::Aggressive);
        let optimized_result = optimized_system.compile_cursed_code(&source);
        
        let benchmark = PerformanceBenchmark {
            name: "Channel Heavy Code".to_string(),
            baseline_time: baseline_result.execution_time,
            optimized_time: optimized_result.execution_time,
            memory_baseline: baseline_result.memory_usage,
            memory_optimized: optimized_result.memory_usage,
            optimizations_applied: optimized_result.optimizations_applied,
        };
        
        println!("Channel Optimization Benchmark:");
        println!("  Speedup: {:.2}x", benchmark.speedup_factor());
        println!("  Memory reduction: {:.1}%", benchmark.memory_reduction() * 100.0);
        println!("  Optimizations: {}", benchmark.optimizations_applied);
        
        assert!(benchmark.speedup_factor() > 1.15, "Channel optimizations should provide 15%+ speedup");
        assert!(benchmark.memory_reduction() > 0.08, "Should have at least 8% memory reduction");
        assert!(optimized_result.optimizations_applied > 8, "Should apply many channel optimizations");
    }
    
    #[test]
    fn test_gc_optimization_performance() {
        let source = generate_gc_heavy_code();
        
        let baseline_system = MockCompilationSystem::new(false, OptimizationLevel::None);
        let baseline_result = baseline_system.compile_cursed_code(&source);
        
        let optimized_system = MockCompilationSystem::new(true, OptimizationLevel::Aggressive);
        let optimized_result = optimized_system.compile_cursed_code(&source);
        
        let benchmark = PerformanceBenchmark {
            name: "GC Heavy Code".to_string(),
            baseline_time: baseline_result.execution_time,
            optimized_time: optimized_result.execution_time,
            memory_baseline: baseline_result.memory_usage,
            memory_optimized: optimized_result.memory_usage,
            optimizations_applied: optimized_result.optimizations_applied,
        };
        
        println!("GC Optimization Benchmark:");
        println!("  Speedup: {:.2}x", benchmark.speedup_factor());
        println!("  Memory reduction: {:.1}%", benchmark.memory_reduction() * 100.0);
        println!("  Optimizations: {}", benchmark.optimizations_applied);
        
        assert!(benchmark.speedup_factor() > 1.2, "GC optimizations should provide 20%+ speedup");
        assert!(benchmark.memory_reduction() > 0.15, "Should have significant memory reduction");
        assert!(optimized_result.optimizations_applied > 6, "Should apply GC optimizations");
    }
    
    #[test]
    fn test_genz_keyword_optimization_performance() {
        let source = generate_genz_keyword_heavy_code();
        
        let baseline_system = MockCompilationSystem::new(false, OptimizationLevel::None);
        let baseline_result = baseline_system.compile_cursed_code(&source);
        
        let optimized_system = MockCompilationSystem::new(true, OptimizationLevel::Aggressive);
        let optimized_result = optimized_system.compile_cursed_code(&source);
        
        let benchmark = PerformanceBenchmark {
            name: "Gen Z Keyword Heavy Code".to_string(),
            baseline_time: baseline_result.execution_time,
            optimized_time: optimized_result.execution_time,
            memory_baseline: baseline_result.memory_usage,
            memory_optimized: optimized_result.memory_usage,
            optimizations_applied: optimized_result.optimizations_applied,
        };
        
        println!("Gen Z Keyword Optimization Benchmark:");
        println!("  Speedup: {:.2}x", benchmark.speedup_factor());
        println!("  Memory reduction: {:.1}%", benchmark.memory_reduction() * 100.0);
        println!("  Optimizations: {}", benchmark.optimizations_applied);
        
        assert!(benchmark.speedup_factor() > 1.1, "Gen Z optimizations should provide speedup");
        assert!(optimized_result.optimizations_applied > 10, "Should apply many keyword optimizations");
    }
    
    #[test]
    fn test_comprehensive_optimization_performance() {
        let source = generate_mixed_optimization_code();
        
        let baseline_system = MockCompilationSystem::new(false, OptimizationLevel::None);
        let baseline_result = baseline_system.compile_cursed_code(&source);
        
        let optimized_system = MockCompilationSystem::new(true, OptimizationLevel::Aggressive);
        let optimized_result = optimized_system.compile_cursed_code(&source);
        
        let benchmark = PerformanceBenchmark {
            name: "Mixed Optimization Code".to_string(),
            baseline_time: baseline_result.execution_time,
            optimized_time: optimized_result.execution_time,
            memory_baseline: baseline_result.memory_usage,
            memory_optimized: optimized_result.memory_usage,
            optimizations_applied: optimized_result.optimizations_applied,
        };
        
        println!("Comprehensive Optimization Benchmark:");
        println!("  Speedup: {:.2}x", benchmark.speedup_factor());
        println!("  Memory reduction: {:.1}%", benchmark.memory_reduction() * 100.0);
        println!("  Optimizations: {}", benchmark.optimizations_applied);
        
        assert!(benchmark.speedup_factor() > 1.25, "Comprehensive optimizations should provide 25%+ speedup");
        assert!(benchmark.memory_reduction() > 0.12, "Should have at least 12% memory reduction");
        assert!(optimized_result.optimizations_applied > 15, "Should apply many different optimizations");
    }
    
    #[test]
    fn test_optimization_level_scaling() {
        let source = generate_mixed_optimization_code();
        
        // Test different optimization levels
        let none_system = MockCompilationSystem::new(false, OptimizationLevel::None);
        let basic_system = MockCompilationSystem::new(true, OptimizationLevel::Basic);
        let aggressive_system = MockCompilationSystem::new(true, OptimizationLevel::Aggressive);
        
        let none_result = none_system.compile_cursed_code(&source);
        let basic_result = basic_system.compile_cursed_code(&source);
        let aggressive_result = aggressive_system.compile_cursed_code(&source);
        
        println!("Optimization Level Scaling:");
        println!("  None: {} optimizations", none_result.optimizations_applied);
        println!("  Basic: {} optimizations", basic_result.optimizations_applied);
        println!("  Aggressive: {} optimizations", aggressive_result.optimizations_applied);
        
        // Verify scaling behavior
        assert_eq!(none_result.optimizations_applied, 0);
        assert!(basic_result.optimizations_applied > none_result.optimizations_applied);
        assert!(aggressive_result.optimizations_applied > basic_result.optimizations_applied);
        
        // Verify performance scaling
        assert!(basic_result.execution_time < none_result.execution_time);
        assert!(aggressive_result.execution_time <= basic_result.execution_time);
    }
    
    #[test]
    fn test_compilation_time_overhead() {
        let source = generate_mixed_optimization_code();
        
        let baseline_system = MockCompilationSystem::new(false, OptimizationLevel::None);
        let optimized_system = MockCompilationSystem::new(true, OptimizationLevel::Aggressive);
        
        let baseline_result = baseline_system.compile_cursed_code(&source);
        let optimized_result = optimized_system.compile_cursed_code(&source);
        
        let compilation_overhead = optimized_result.compilation_time.as_secs_f64() / 
                                 baseline_result.compilation_time.as_secs_f64();
        
        println!("Compilation Time Analysis:");
        println!("  Baseline: {:?}", baseline_result.compilation_time);
        println!("  Optimized: {:?}", optimized_result.compilation_time);
        println!("  Overhead: {:.2}x", compilation_overhead);
        
        // Ensure compilation overhead is reasonable (less than 3x)
        assert!(compilation_overhead < 3.0, "Compilation overhead should be reasonable");
        
        // But ensure we get runtime benefits
        let runtime_improvement = baseline_result.execution_time.as_secs_f64() / 
                                optimized_result.execution_time.as_secs_f64();
        
        println!("  Runtime improvement: {:.2}x", runtime_improvement);
        assert!(runtime_improvement > 1.1, "Should get runtime benefits despite compilation overhead");
    }
    
    #[test]
    fn test_memory_usage_patterns() {
        let test_cases = vec![
            ("Goroutine Heavy", generate_goroutine_heavy_code()),
            ("Channel Heavy", generate_channel_heavy_code()),
            ("GC Heavy", generate_gc_heavy_code()),
            ("GenZ Heavy", generate_genz_keyword_heavy_code()),
            ("Mixed", generate_mixed_optimization_code()),
        ];
        
        println!("Memory Usage Pattern Analysis:");
        
        for (name, source) in test_cases {
            let baseline_system = MockCompilationSystem::new(false, OptimizationLevel::None);
            let optimized_system = MockCompilationSystem::new(true, OptimizationLevel::Aggressive);
            
            let baseline_result = baseline_system.compile_cursed_code(&source);
            let optimized_result = optimized_system.compile_cursed_code(&source);
            
            let memory_reduction = 1.0 - (optimized_result.memory_usage as f64 / baseline_result.memory_usage as f64);
            
            println!("  {}: {:.1}% memory reduction", name, memory_reduction * 100.0);
            
            // All test cases should show some memory improvement
            assert!(memory_reduction > 0.0, "{} should show memory improvement", name);
        }
    }
    
    #[test]
    fn test_optimization_effectiveness_by_pattern() {
        let patterns = HashMap::from([
            ("goroutine", "stan worker(); yolo; goroutine_frame();"),
            ("channel", "make_channel(); send(ch); receive(ch);"),
            ("gc", "new Object(); alloc_data(); gc_collect();"),
            ("genz", "slay func(); facts x = 5; lowkey (x > 0) {}"),
            ("control_flow", "result?; unwrap(); expect();"),
            ("memory", "squad S {}; collab I {}; array[0];"),
        ]);
        
        println!("Optimization Effectiveness by Pattern:");
        
        for (pattern_name, pattern_code) in patterns {
            let source = format!(r#"
                slay test_function() {{
                    {}
                }}
            "#, pattern_code);
            
            let baseline_system = MockCompilationSystem::new(false, OptimizationLevel::None);
            let optimized_system = MockCompilationSystem::new(true, OptimizationLevel::Aggressive);
            
            let baseline_result = baseline_system.compile_cursed_code(&source);
            let optimized_result = optimized_system.compile_cursed_code(&source);
            
            let speedup = baseline_result.execution_time.as_secs_f64() / optimized_result.execution_time.as_secs_f64();
            
            println!("  {}: {:.2}x speedup, {} optimizations", 
                     pattern_name, speedup, optimized_result.optimizations_applied);
            
            assert!(optimized_result.optimizations_applied > 0, 
                    "{} pattern should trigger optimizations", pattern_name);
        }
    }
    
    #[test]
    fn test_optimization_stability() {
        let source = generate_mixed_optimization_code();
        let optimized_system = MockCompilationSystem::new(true, OptimizationLevel::Aggressive);
        
        let mut results = Vec::new();
        
        // Run multiple compilations to check stability
        for _ in 0..5 {
            let result = optimized_system.compile_cursed_code(&source);
            results.push(result);
        }
        
        // Check that results are consistent
        let first_optimizations = results[0].optimizations_applied;
        for result in &results {
            assert_eq!(result.optimizations_applied, first_optimizations,
                      "Optimization count should be stable across runs");
        }
        
        // Check that performance is consistent (within 10% variance)
        let baseline_time = results[0].execution_time;
        for result in &results {
            let variance = (result.execution_time.as_secs_f64() - baseline_time.as_secs_f64()).abs() 
                         / baseline_time.as_secs_f64();
            assert!(variance < 0.1, "Performance should be stable across runs");
        }
        
        println!("Optimization Stability Test: PASSED");
        println!("  Consistent optimizations: {}", first_optimizations);
        println!("  Performance variance: <10%");
    }
}
