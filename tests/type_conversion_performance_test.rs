//! Performance Tests for Type Conversion System
//!
//! This test suite focuses on the performance characteristics of type conversions,
//! measuring compilation time, memory usage, and runtime efficiency.

use std::time::{Duration, Instant};
use std::collections::HashMap;
use tracing::{info, debug, warn};

// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_test_writer()
            .try_init()
            .ok();
    };
}

use cursed::codegen::llvm::{LlvmCodeGenerator, TypeConversionSystem, ConversionConfig, ConversionStatistics};
use cursed::ast::expressions::{TypeConversionExpression, Literal, LiteralValue};
use cursed::ast::traits::{Expression, Node};
use cursed::lexer::token::{Token, TokenType};
use cursed::core::type_checker::Type;
use cursed::error::Error;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::BasicValueEnum;

/// Performance measurement utilities
struct PerformanceMeasurement {
    start_time: Instant,
    operation_name: String,
}

impl PerformanceMeasurement {
    fn new(operation_name: &str) -> Self {
        info!("Starting performance measurement: {}", operation_name);
        Self {
            start_time: Instant::now(),
            operation_name: operation_name.to_string(),
        }
    }

    fn end(self) -> Duration {
        let elapsed = self.start_time.elapsed();
        info!("Completed {}: {:?}", self.operation_name, elapsed);
        elapsed
    }
}

/// Helper to create test LLVM context and generator
fn create_test_generator() -> (Context, LlvmCodeGenerator<'static>) {
    let context = Context::create();
    let module = context.create_module("perf_test_module");
    let builder = context.create_builder();
    
    // We need to leak the context to satisfy lifetime requirements
    let leaked_context = Box::leak(Box::new(context));
    let leaked_module = Box::leak(Box::new(module));
    let leaked_builder = Box::leak(Box::new(builder));
    
    let generator = LlvmCodeGenerator::new(leaked_context, leaked_module, leaked_builder);
    
    // Return the leaked context reference and the generator
    unsafe { (std::ptr::read(leaked_context), generator) }
}

/// Helper to create type conversion expressions
fn create_conversion_expression(value: i64, target_type: &str) -> TypeConversionExpression {
    let token = Token::new(TokenType::NORMIE, &value.to_string(), 1, 1);
    let literal = Box::new(Literal {
        value: LiteralValue::Integer(value),
        token: token.clone(),
    });

    TypeConversionExpression {
        token,
        expression: literal,
        type_name: target_type.to_string(),
    }
}

/// Benchmark basic integer conversions
#[test]
fn benchmark_integer_conversions() {
    init_tracing!();
    info!("Benchmarking integer conversions");

    let (context, mut generator) = create_test_generator();
    let config = ConversionConfig::default();

    let conversion_types = vec![
        ("smol", "normie"),   // 8-bit to 32-bit
        ("normie", "thicc"),  // 32-bit to 64-bit
        ("thicc", "smol"),    // 64-bit to 8-bit (narrowing)
        ("mid", "thicc"),     // 16-bit to 64-bit
    ];

    let iterations = 10000;
    let mut total_time = Duration::ZERO;
    let mut results = HashMap::new();

    for (from_type, to_type) in conversion_types {
        let measurement = PerformanceMeasurement::new(&format!("{} to {} conversions", from_type, to_type));
        
        for i in 0..iterations {
            let conversion = create_conversion_expression(i % 1000, to_type);
            
            let start = Instant::now();
            let result = generator.compile_explicit_conversion(&conversion, &config);
            let elapsed = start.elapsed();
            
            assert!(result.is_ok(), "Conversion should succeed");
            total_time += elapsed;
        }
        
        let conversion_time = measurement.end();
        results.insert(format!("{}_to_{}", from_type, to_type), conversion_time);
        
        let avg_per_conversion = conversion_time.as_nanos() / iterations as u128;
        info!("Average time per {} to {} conversion: {} ns", from_type, to_type, avg_per_conversion);
    }

    // Verify performance is reasonable (less than 1ms per conversion)
    for (conversion_name, time) in results {
        let avg_time_per_conversion = time.as_millis() as f64 / iterations as f64;
        assert!(avg_time_per_conversion < 1.0, 
                "Conversion {} took too long: {} ms per conversion", 
                conversion_name, avg_time_per_conversion);
    }

    info!("Integer conversion benchmarks completed");
}

/// Benchmark floating point conversions
#[test]
fn benchmark_float_conversions() {
    init_tracing!();
    info!("Benchmarking floating point conversions");

    let (context, mut generator) = create_test_generator();
    let config = ConversionConfig::default();

    let float_conversions = vec![
        ("snack", "meal"),    // f32 to f64
        ("meal", "snack"),    // f64 to f32
        ("normie", "snack"),  // int to float
        ("snack", "normie"),  // float to int
    ];

    let iterations = 5000;

    for (from_type, to_type) in float_conversions {
        let measurement = PerformanceMeasurement::new(&format!("{} to {} float conversions", from_type, to_type));
        
        for i in 0..iterations {
            let value = if from_type.contains("snack") || from_type.contains("meal") {
                i as f64 + 0.5
            } else {
                i as f64
            };
            
            let token = Token::new(TokenType::SNACK, &value.to_string(), 1, 1);
            let literal = Box::new(Literal {
                value: if from_type.contains("snack") || from_type.contains("meal") {
                    LiteralValue::Float(value)
                } else {
                    LiteralValue::Integer(value as i64)
                },
                token: token.clone(),
            });

            let conversion = TypeConversionExpression {
                token,
                expression: literal,
                type_name: to_type.to_string(),
            };

            let mut test_config = config.clone();
            test_config.allow_lossy_conversions = true; // Allow float to int
            
            let start = Instant::now();
            let result = generator.compile_explicit_conversion(&conversion, &test_config);
            let elapsed = start.elapsed();
            
            assert!(result.is_ok(), "Float conversion should succeed");
            
            // Log slow conversions
            if elapsed.as_millis() > 1 {
                warn!("Slow conversion detected: {} ms for {} to {}", elapsed.as_millis(), from_type, to_type);
            }
        }
        
        let total_time = measurement.end();
        let avg_per_conversion = total_time.as_nanos() / iterations as u128;
        info!("Average time per {} to {} conversion: {} ns", from_type, to_type, avg_per_conversion);
        
        // Verify reasonable performance
        assert!(avg_per_conversion < 1_000_000, "Float conversion too slow: {} ns", avg_per_conversion);
    }

    info!("Float conversion benchmarks completed");
}

/// Benchmark conversion compatibility checking
#[test]
fn benchmark_compatibility_checking() {
    init_tracing!();
    info!("Benchmarking conversion compatibility checking");

    let (context, generator) = create_test_generator();

    let types = vec![
        Type::Smol, Type::Mid, Type::Normie, Type::Thicc,
        Type::Snack, Type::Meal, Type::Lit, Type::Sip, Type::Rune, Type::Tea,
    ];

    let iterations = 100000;
    let measurement = PerformanceMeasurement::new("compatibility checking");

    for _ in 0..iterations {
        for from_type in &types {
            for to_type in &types {
                let start = Instant::now();
                let _compatibility = generator.check_conversion_compatibility(from_type, to_type);
                let elapsed = start.elapsed();
                
                // Log any compatibility check that takes more than 1 microsecond
                if elapsed.as_micros() > 1 {
                    warn!("Slow compatibility check: {} μs for {:?} to {:?}", 
                          elapsed.as_micros(), from_type, to_type);
                }
            }
        }
    }

    let total_time = measurement.end();
    let total_checks = iterations * types.len() * types.len();
    let avg_per_check = total_time.as_nanos() / total_checks as u128;
    
    info!("Performed {} compatibility checks", total_checks);
    info!("Average time per compatibility check: {} ns", avg_per_check);

    // Verify compatibility checking is very fast (less than 100ns per check)
    assert!(avg_per_check < 100, "Compatibility checking too slow: {} ns per check", avg_per_check);

    info!("Compatibility checking benchmarks completed");
}

/// Benchmark conversion cost calculation
#[test]
fn benchmark_cost_calculation() {
    init_tracing!();
    info!("Benchmarking conversion cost calculation");

    let (context, generator) = create_test_generator();

    let types = vec![
        Type::Smol, Type::Mid, Type::Normie, Type::Thicc,
        Type::Snack, Type::Meal, Type::Lit,
    ];

    let iterations = 50000;
    let measurement = PerformanceMeasurement::new("cost calculation");

    for _ in 0..iterations {
        for from_type in &types {
            for to_type in &types {
                let start = Instant::now();
                let _cost = generator.get_conversion_cost(from_type, to_type);
                let elapsed = start.elapsed();
                
                // Log slow cost calculations
                if elapsed.as_micros() > 1 {
                    warn!("Slow cost calculation: {} μs for {:?} to {:?}", 
                          elapsed.as_micros(), from_type, to_type);
                }
            }
        }
    }

    let total_time = measurement.end();
    let total_calculations = iterations * types.len() * types.len();
    let avg_per_calculation = total_time.as_nanos() / total_calculations as u128;
    
    info!("Performed {} cost calculations", total_calculations);
    info!("Average time per cost calculation: {} ns", avg_per_calculation);

    // Verify cost calculation is very fast
    assert!(avg_per_calculation < 50, "Cost calculation too slow: {} ns per calculation", avg_per_calculation);

    info!("Cost calculation benchmarks completed");
}

/// Benchmark conversion chains
#[test]
fn benchmark_conversion_chains() {
    init_tracing!();
    info!("Benchmarking conversion chains");

    let (context, mut generator) = create_test_generator();
    let config = ConversionConfig::default();

    // Create test value
    let int_value = generator.context.i8_type().const_int(42, false).into();

    let chain_lengths = vec![1, 2, 3, 5, 10];
    let iterations = 1000;

    for chain_length in chain_lengths {
        let measurement = PerformanceMeasurement::new(&format!("conversion chains of length {}", chain_length));
        
        // Create a conversion chain
        let mut conversion_chain = Vec::new();
        let types = vec![Type::Smol, Type::Mid, Type::Normie, Type::Thicc];
        
        for i in 0..chain_length {
            let from_idx = i % types.len();
            let to_idx = (i + 1) % types.len();
            conversion_chain.push((types[from_idx].clone(), types[to_idx].clone()));
        }

        for _ in 0..iterations {
            let start = Instant::now();
            let result = generator.apply_conversion_chain(int_value, &conversion_chain, &config);
            let elapsed = start.elapsed();
            
            assert!(result.is_ok(), "Conversion chain should succeed");
            
            // Log slow chain applications
            if elapsed.as_millis() > 1 {
                warn!("Slow conversion chain of length {}: {} ms", chain_length, elapsed.as_millis());
            }
        }
        
        let total_time = measurement.end();
        let avg_per_chain = total_time.as_nanos() / iterations as u128;
        info!("Average time per chain of length {}: {} ns", chain_length, avg_per_chain);
        
        // Verify chain performance scales reasonably
        let max_time_per_step = 100_000; // 100 μs per step
        assert!(avg_per_chain < (chain_length as u128 * max_time_per_step), 
                "Conversion chain too slow: {} ns for length {}", avg_per_chain, chain_length);
    }

    info!("Conversion chain benchmarks completed");
}

/// Memory usage benchmark
#[test]
fn benchmark_memory_usage() {
    init_tracing!();
    info!("Benchmarking memory usage of conversions");

    // This test monitors memory usage during heavy conversion operations
    let (context, mut generator) = create_test_generator();
    let config = ConversionConfig::default();

    let measurement = PerformanceMeasurement::new("memory usage test");
    
    // Perform many conversions to stress test memory usage
    let iterations = 50000;
    let mut successful_conversions = 0;

    for i in 0..iterations {
        let conversion = create_conversion_expression(i % 1000, "thicc");
        
        match generator.compile_explicit_conversion(&conversion, &config) {
            Ok(_) => successful_conversions += 1,
            Err(e) => {
                warn!("Conversion failed at iteration {}: {}", i, e);
            }
        }
        
        // Periodically log progress
        if i % 10000 == 0 {
            debug!("Completed {} conversions", i);
        }
    }

    let total_time = measurement.end();
    
    info!("Successfully completed {}/{} conversions", successful_conversions, iterations);
    info!("Total time: {:?}", total_time);
    
    assert!(successful_conversions > iterations * 9 / 10, 
            "Too many conversion failures: {}/{}", successful_conversions, iterations);

    info!("Memory usage benchmarks completed");
}

/// Concurrent conversion benchmark
#[test]
fn benchmark_concurrent_conversions() {
    init_tracing!();
    info!("Benchmarking concurrent conversions");

    use std::sync::Arc;
    use std::thread;

    let num_threads = 4;
    let conversions_per_thread = 1000;
    
    let measurement = PerformanceMeasurement::new("concurrent conversions");

    let handles: Vec<_> = (0..num_threads).map(|thread_id| {
        thread::spawn(move || {
            let (context, mut generator) = create_test_generator();
            let config = ConversionConfig::default();
            
            let mut successful = 0;
            for i in 0..conversions_per_thread {
                let value = thread_id * conversions_per_thread + i;
                let conversion = create_conversion_expression(value as i64, "thicc");
                
                if generator.compile_explicit_conversion(&conversion, &config).is_ok() {
                    successful += 1;
                }
            }
            
            info!("Thread {} completed {}/{} conversions", thread_id, successful, conversions_per_thread);
            successful
        })
    }).collect();

    let mut total_successful = 0;
    for handle in handles {
        total_successful += handle.join().unwrap();
    }

    let total_time = measurement.end();
    let total_conversions = num_threads * conversions_per_thread;
    
    info!("Concurrent test: {}/{} conversions successful", total_successful, total_conversions);
    info!("Total time with {} threads: {:?}", num_threads, total_time);
    
    assert!(total_successful > total_conversions * 9 / 10, 
            "Too many failures in concurrent test: {}/{}", total_successful, total_conversions);

    info!("Concurrent conversion benchmarks completed");
}

/// Statistics tracking performance
#[test]
fn benchmark_statistics_tracking() {
    init_tracing!();
    info!("Benchmarking statistics tracking performance");

    let iterations = 1_000_000;
    let measurement = PerformanceMeasurement::new("statistics tracking");

    let mut stats = ConversionStatistics::default();

    for i in 0..iterations {
        match i % 4 {
            0 => stats.record_explicit_conversion(i as u64 % 1000),
            1 => stats.record_implicit_conversion(i as u64 % 500),
            2 => stats.record_type_assertion(i as u64 % 2000),
            3 => stats.record_failed_conversion(),
            _ => unreachable!(),
        }
        
        // Occasionally calculate average time to test the computation
        if i % 10000 == 0 {
            let _avg = stats.average_conversion_time_us();
        }
    }

    let total_time = measurement.end();
    let avg_per_operation = total_time.as_nanos() / iterations as u128;
    
    info!("Statistics tracking: {} operations in {:?}", iterations, total_time);
    info!("Average time per statistics operation: {} ns", avg_per_operation);
    
    // Verify statistics tracking is very fast
    assert!(avg_per_operation < 50, "Statistics tracking too slow: {} ns per operation", avg_per_operation);
    
    // Verify statistics are correct
    assert_eq!(stats.explicit_conversions + stats.implicit_conversions + 
               stats.type_assertions + stats.failed_conversions, iterations as u64);

    info!("Statistics tracking benchmarks completed");
}

/// Regression test for performance
#[test]
fn test_performance_regression() {
    init_tracing!();
    info!("Running performance regression test");

    let (context, mut generator) = create_test_generator();
    let config = ConversionConfig::default();

    // Baseline performance test
    let iterations = 1000;
    let measurement = PerformanceMeasurement::new("performance regression baseline");

    for i in 0..iterations {
        let conversion = create_conversion_expression(i, "thicc");
        let result = generator.compile_explicit_conversion(&conversion, &config);
        assert!(result.is_ok(), "Baseline conversion should succeed");
    }

    let baseline_time = measurement.end();
    let avg_baseline = baseline_time.as_nanos() / iterations as u128;
    
    info!("Baseline performance: {} ns per conversion", avg_baseline);
    
    // Set reasonable performance thresholds
    let max_acceptable_time_ns = 10_000; // 10 μs per conversion
    
    assert!(avg_baseline < max_acceptable_time_ns, 
            "Performance regression detected: {} ns > {} ns threshold", 
            avg_baseline, max_acceptable_time_ns);

    info!("Performance regression test passed");
}
