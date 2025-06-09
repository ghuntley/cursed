//! Performance and optimization tests for constrained generics LLVM codegen
//!
//! This test suite focuses on:
//! - Performance characteristics of different monomorphization strategies
//! - Memory usage optimization for generic specializations
//! - Method dispatch optimization effectiveness
//! - Compilation time benchmarks
//! - Cache effectiveness for constraint validation

use cursed::ast::declarations::{FunctionStatement, SquadStatement, GenericConstraint, Parameter, TypeParameter};
use cursed::ast::expressions::{CallExpression, Identifier};
use cursed::codegen::llvm::constrained_generics::{
    ConstrainedGenericsCodegen, ConstrainedGenericConfig, MonomorphizationStrategy,
    ConstrainedGenericsExtension
};
use cursed::codegen::llvm::context::LlvmCodeGenerator;
use cursed::core::type_checker::Type;
use cursed::lexer::token::Token;
use inkwell::context::Context;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tracing::{debug, info, warn};

mod common;

/// Performance test configuration
#[derive(Debug, Clone)]
struct PerfTestConfig {
    /// Number of type combinations to test
    type_combinations: usize,
    /// Number of iterations per test
    iterations: usize,
    /// Enable detailed timing
    detailed_timing: bool,
}

impl Default for PerfTestConfig {
    fn default() -> Self {
        Self {
            type_combinations: 10,
            iterations: 5,
            detailed_timing: true,
        }
    }
}

/// Performance measurement utilities
struct PerfMeasurement {
    name: String,
    start_time: Instant,
    measurements: Vec<Duration>,
}

impl PerfMeasurement {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            start_time: Instant::now(),
            measurements: Vec::new(),
        }
    }

    fn start_iteration(&mut self) {
        self.start_time = Instant::now();
    }

    fn end_iteration(&mut self) {
        self.measurements.push(self.start_time.elapsed());
    }

    fn average(&self) -> Duration {
        if self.measurements.is_empty() {
            return Duration::from_secs(0);
        }
        let total: Duration = self.measurements.iter().sum();
        total / self.measurements.len() as u32
    }

    fn min(&self) -> Duration {
        self.measurements.iter().copied().min().unwrap_or_default()
    }

    fn max(&self) -> Duration {
        self.measurements.iter().copied().max().unwrap_or_default()
    }

    fn report(&self) {
        info!("Performance Report for {}:", self.name);
        info!("  Iterations: {}", self.measurements.len());
        info!("  Average: {:?}", self.average());
        info!("  Min: {:?}", self.min());
        info!("  Max: {:?}", self.max());
        
        if self.measurements.len() > 1 {
            let variance = self.calculate_variance();
            info!("  Variance: {:?}", variance);
        }
    }

    fn calculate_variance(&self) -> Duration {
        if self.measurements.len() <= 1 {
            return Duration::from_secs(0);
        }
        
        let avg = self.average();
        let sum_squared_diff: u128 = self.measurements.iter()
            .map(|&d| {
                let diff = if d > avg { d - avg } else { avg - d };
                diff.as_nanos() * diff.as_nanos()
            })
            .sum();
        
        let variance_nanos = sum_squared_diff / self.measurements.len() as u128;
        Duration::from_nanos((variance_nanos as f64).sqrt() as u64)
    }
}

/// Generate test type combinations for performance testing
fn generate_type_combinations(count: usize) -> Vec<Vec<Type>> {
    let mut combinations = Vec::new();
    
    let base_types = vec![
        Type::Normie,
        Type::Thicc,
        Type::Tea,
        Type::Lit,
        Type::Snack,
        Type::Meal,
    ];
    
    for i in 0..count {
        let type1 = base_types[i % base_types.len()].clone();
        let type2 = base_types[(i + 1) % base_types.len()].clone();
        combinations.push(vec![type1, type2]);
    }
    
    // Add some complex types for variety
    combinations.push(vec![Type::Array(Box::new(Type::Normie), 10), Type::Tea]);
    combinations.push(vec![Type::Slice(Box::new(Type::Thicc)), Type::Lit]);
    combinations.push(vec![Type::Pointer(Box::new(Type::Tea)), Type::Normie]);
    
    combinations
}

/// Create a mock generic function for performance testing
fn create_perf_test_function(name: &str) -> FunctionStatement {
    let token = Token::Identifier(name.to_string());
    
    FunctionStatement {
        token: token.clone(),
        name: Identifier {
            token: token.clone(),
            value: name.to_string(),
        },
        type_parameters: vec![
            TypeParameter {
                token: token.clone(),
                value: "T".to_string(),
            },
            TypeParameter {
                token: token.clone(),
                value: "U".to_string(),
            },
        ],
        generic_constraints: vec![
            GenericConstraint::new(
                token.clone(),
                "T".to_string(),
                "Stringer".to_string(),
            ),
            GenericConstraint::new(
                token.clone(),
                "U".to_string(),
                "Comparable".to_string(),
            ),
        ],
        parameters: vec![
            Parameter {
                token: token.clone(),
                name: Identifier {
                    token: token.clone(),
                    value: "x".to_string(),
                },
                type_name: Identifier {
                    token: token.clone(),
                    value: "T".to_string(),
                },
            },
            Parameter {
                token: token.clone(),
                name: Identifier {
                    token: token.clone(),
                    value: "y".to_string(),
                },
                type_name: Identifier {
                    token: token.clone(),
                    value: "U".to_string(),
                },
            },
        ],
        return_type: Some(Box::new(Identifier {
            token: token.clone(),
            value: "Lit".to_string(),
        })),
        body: Box::new(cursed::ast::statements::BlockStatement {
            token: token.clone(),
            statements: vec![],
        }),
    }
}

/// Setup function for performance testing
fn setup_perf_llvm_generator() -> (Context, LlvmCodeGenerator<'static>) {
    let context = Box::leak(Box::new(Context::create()));
    let module = context.create_module("perf_test_module");
    let builder = context.create_builder();
    
    let mut generator = LlvmCodeGenerator::new(context, module, builder, "perf_test".to_string());
    
    // Initialize required components
    generator.mono_manager = cursed::codegen::monomorphization::MonomorphizationManager::new();
    generator.llvm_mono_manager = cursed::codegen::llvm::monomorphization::MonomorphizationManager::new();
    
    (*context, generator)
}

#[test]
fn test_full_specialization_performance() {
    common::tracing::setup();
    info!("Testing full specialization performance");

    let (_context, mut generator) = setup_perf_llvm_generator();
    let config = ConstrainedGenericConfig {
        strategy: MonomorphizationStrategy::FullSpecialization,
        optimize_dispatch: true,
        debug_generics: false,
        max_recursion_depth: 32,
        cache_constraints: true,
    };

    let function = create_perf_test_function("perf_full_spec");
    let type_combinations = generate_type_combinations(20);
    
    let mut measurement = PerfMeasurement::new("Full Specialization");
    
    for type_args in &type_combinations {
        measurement.start_iteration();
        
        let result = generator.generate_constrained_function_specialization(&function, type_args, &config);
        
        measurement.end_iteration();
        
        debug!("Specialization for {:?}: {:?}", type_args, result.is_ok());
    }
    
    measurement.report();
}

#[test]
fn test_type_erasure_performance() {
    common::tracing::setup();
    info!("Testing type erasure performance");

    let (_context, mut generator) = setup_perf_llvm_generator();
    let config = ConstrainedGenericConfig {
        strategy: MonomorphizationStrategy::TypeErasure,
        optimize_dispatch: false,
        debug_generics: false,
        max_recursion_depth: 32,
        cache_constraints: true,
    };

    let function = create_perf_test_function("perf_type_erasure");
    let type_combinations = generate_type_combinations(20);
    
    let mut measurement = PerfMeasurement::new("Type Erasure");
    
    for type_args in &type_combinations {
        measurement.start_iteration();
        
        let result = generator.generate_constrained_function_specialization(&function, type_args, &config);
        
        measurement.end_iteration();
        
        debug!("Type erasure for {:?}: {:?}", type_args, result.is_ok());
    }
    
    measurement.report();
}

#[test]
fn test_hybrid_strategy_performance() {
    common::tracing::setup();
    info!("Testing hybrid strategy performance");

    let (_context, mut generator) = setup_perf_llvm_generator();
    let config = ConstrainedGenericConfig {
        strategy: MonomorphizationStrategy::Hybrid,
        optimize_dispatch: true,
        debug_generics: false,
        max_recursion_depth: 32,
        cache_constraints: true,
    };

    let function = create_perf_test_function("perf_hybrid");
    let type_combinations = generate_type_combinations(20);
    
    let mut measurement = PerfMeasurement::new("Hybrid Strategy");
    
    for type_args in &type_combinations {
        measurement.start_iteration();
        
        let result = generator.generate_constrained_function_specialization(&function, type_args, &config);
        
        measurement.end_iteration();
        
        debug!("Hybrid strategy for {:?}: {:?}", type_args, result.is_ok());
    }
    
    measurement.report();
}

#[test]
fn test_constraint_validation_cache_effectiveness() {
    common::tracing::setup();
    info!("Testing constraint validation cache effectiveness");

    let (_context, generator) = setup_perf_llvm_generator();
    
    let constraints = vec![
        GenericConstraint::new(
            Token::Identifier("constraint".to_string()),
            "T".to_string(),
            "Stringer".to_string(),
        ),
    ];
    
    let type_args = vec![Type::Tea];
    let type_params = vec!["T".to_string()];
    
    // First run - cache miss
    let mut first_run = PerfMeasurement::new("First Validation (Cache Miss)");
    for _ in 0..10 {
        first_run.start_iteration();
        let _ = generator.validate_generic_constraints(&constraints, &type_args, &type_params);
        first_run.end_iteration();
    }
    first_run.report();
    
    // Second run - cache hit (in a real implementation)
    let mut second_run = PerfMeasurement::new("Second Validation (Cache Hit)");
    for _ in 0..10 {
        second_run.start_iteration();
        let _ = generator.validate_generic_constraints(&constraints, &type_args, &type_params);
        second_run.end_iteration();
    }
    second_run.report();
    
    // In a real implementation with caching, second run should be faster
    info!("Cache effectiveness: First avg: {:?}, Second avg: {:?}", 
          first_run.average(), second_run.average());
}

#[test]
fn test_specialization_name_generation_performance() {
    common::tracing::setup();
    info!("Testing specialization name generation performance");

    let (_context, generator) = setup_perf_llvm_generator();
    
    let base_name = "complex_generic_function_with_long_name";
    let type_combinations = generate_type_combinations(100);
    
    let mut measurement = PerfMeasurement::new("Name Generation");
    
    for type_args in &type_combinations {
        measurement.start_iteration();
        
        let _specialized_name = generator.generate_specialized_function_name(base_name, type_args);
        
        measurement.end_iteration();
    }
    
    measurement.report();
}

#[test]
fn test_type_mangling_performance() {
    common::tracing::setup();
    info!("Testing type mangling performance");

    let (_context, generator) = setup_perf_llvm_generator();
    
    let test_types = vec![
        Type::Normie,
        Type::Tea,
        Type::Array(Box::new(Type::Normie), 100),
        Type::Slice(Box::new(Type::Tea)),
        Type::Pointer(Box::new(Type::Thicc)),
        Type::Struct("VeryLongStructName".to_string(), vec![]),
        Type::Array(Box::new(Type::Array(Box::new(Type::Normie), 10)), 5), // Nested array
    ];
    
    let mut measurement = PerfMeasurement::new("Type Mangling");
    
    for _ in 0..1000 {
        measurement.start_iteration();
        
        for typ in &test_types {
            let _mangled = generator.type_to_mangled_name(typ);
        }
        
        measurement.end_iteration();
    }
    
    measurement.report();
}

#[test]
fn test_memory_usage_different_strategies() {
    common::tracing::setup();
    info!("Testing memory usage characteristics of different strategies");

    // This test measures relative memory usage patterns
    // In a real implementation, this would use actual memory profiling

    let (_context, mut generator) = setup_perf_llvm_generator();
    let function = create_perf_test_function("memory_test");
    let type_combinations = generate_type_combinations(50);
    
    // Test full specialization "memory usage"
    let full_config = ConstrainedGenericConfig {
        strategy: MonomorphizationStrategy::FullSpecialization,
        ..Default::default()
    };
    
    let mut full_spec_count = 0;
    for type_args in &type_combinations {
        if generator.generate_constrained_function_specialization(&function, type_args, &full_config).is_ok() {
            full_spec_count += 1;
        }
    }
    
    info!("Full specialization would generate {} functions", full_spec_count);
    
    // Test type erasure "memory usage"
    let erasure_config = ConstrainedGenericConfig {
        strategy: MonomorphizationStrategy::TypeErasure,
        ..Default::default()
    };
    
    let mut erasure_count = 0;
    for type_args in &type_combinations {
        if generator.generate_constrained_function_specialization(&function, type_args, &erasure_config).is_ok() {
            erasure_count += 1;
        }
    }
    
    info!("Type erasure would generate {} functions", erasure_count);
    
    // Hybrid should be somewhere in between
    let hybrid_config = ConstrainedGenericConfig {
        strategy: MonomorphizationStrategy::Hybrid,
        ..Default::default()
    };
    
    let mut hybrid_count = 0;
    for type_args in &type_combinations {
        if generator.generate_constrained_function_specialization(&function, type_args, &hybrid_config).is_ok() {
            hybrid_count += 1;
        }
    }
    
    info!("Hybrid strategy would generate {} functions", hybrid_count);
}

#[test]
fn test_constraint_checking_scalability() {
    common::tracing::setup();
    info!("Testing constraint checking scalability");

    let (_context, generator) = setup_perf_llvm_generator();
    
    // Test with increasing numbers of constraints
    for constraint_count in [1, 5, 10, 20, 50] {
        let mut constraints = Vec::new();
        let mut type_args = Vec::new();
        let mut type_params = Vec::new();
        
        for i in 0..constraint_count {
            constraints.push(GenericConstraint::new(
                Token::Identifier(format!("constraint_{}", i)),
                format!("T{}", i),
                format!("Interface{}", i % 3), // Cycle through 3 interface names
            ));
            type_args.push(Type::Tea); // Use consistent type
            type_params.push(format!("T{}", i));
        }
        
        let mut measurement = PerfMeasurement::new(&format!("Constraint Checking ({})", constraint_count));
        
        for _ in 0..10 {
            measurement.start_iteration();
            let _ = generator.validate_generic_constraints(&constraints, &type_args, &type_params);
            measurement.end_iteration();
        }
        
        measurement.report();
    }
}

#[test]
fn test_cache_key_generation_performance() {
    common::tracing::setup();
    info!("Testing cache key generation performance under load");

    let (_context, generator) = setup_perf_llvm_generator();
    
    let function_names = vec![
        "short_func",
        "medium_length_function_name",
        "very_long_function_name_that_might_be_used_in_real_code",
    ];
    
    let type_combinations = generate_type_combinations(100);
    
    for func_name in &function_names {
        let mut measurement = PerfMeasurement::new(&format!("Cache Key Gen ({})", func_name));
        
        for type_args in &type_combinations {
            measurement.start_iteration();
            let _cache_key = generator.generate_specialization_cache_key(func_name, type_args);
            measurement.end_iteration();
        }
        
        measurement.report();
    }
}

#[test]
fn test_gc_metadata_registration_performance() {
    common::tracing::setup();
    info!("Testing GC metadata registration performance");

    let (_context, mut generator) = setup_perf_llvm_generator();
    
    let struct_names = (0..100).map(|i| format!("TestStruct{}", i)).collect::<Vec<_>>();
    let type_combinations = generate_type_combinations(20);
    
    let mut measurement = PerfMeasurement::new("GC Metadata Registration");
    
    for (i, struct_name) in struct_names.iter().enumerate() {
        let type_args = &type_combinations[i % type_combinations.len()];
        let specialized_name = format!("{}__specialized", struct_name);
        
        measurement.start_iteration();
        let _ = generator.register_gc_metadata_for_specialization(struct_name, type_args, &specialized_name);
        measurement.end_iteration();
    }
    
    measurement.report();
    
    info!("Registered {} GC metadata entries", generator.gc_metadata.len());
}

#[test]
fn test_optimization_effectiveness() {
    common::tracing::setup();
    info!("Testing optimization effectiveness comparison");

    let (_context, mut generator) = setup_perf_llvm_generator();
    let function = create_perf_test_function("optimization_test");
    let type_args = vec![Type::Normie, Type::Thicc]; // Simple types for optimization
    
    // Test with optimizations enabled
    let optimized_config = ConstrainedGenericConfig {
        strategy: MonomorphizationStrategy::Hybrid,
        optimize_dispatch: true,
        debug_generics: false,
        max_recursion_depth: 32,
        cache_constraints: true,
    };
    
    let mut optimized_measurement = PerfMeasurement::new("Optimized Generation");
    for _ in 0..20 {
        optimized_measurement.start_iteration();
        let _ = generator.generate_constrained_function_specialization(&function, &type_args, &optimized_config);
        optimized_measurement.end_iteration();
    }
    optimized_measurement.report();
    
    // Test with optimizations disabled
    let unoptimized_config = ConstrainedGenericConfig {
        strategy: MonomorphizationStrategy::FullSpecialization,
        optimize_dispatch: false,
        debug_generics: true,
        max_recursion_depth: 16,
        cache_constraints: false,
    };
    
    let mut unoptimized_measurement = PerfMeasurement::new("Unoptimized Generation");
    for _ in 0..20 {
        unoptimized_measurement.start_iteration();
        let _ = generator.generate_constrained_function_specialization(&function, &type_args, &unoptimized_config);
        unoptimized_measurement.end_iteration();
    }
    unoptimized_measurement.report();
    
    // Compare results
    let optimized_avg = optimized_measurement.average();
    let unoptimized_avg = unoptimized_measurement.average();
    
    info!("Optimization comparison:");
    info!("  Optimized: {:?}", optimized_avg);
    info!("  Unoptimized: {:?}", unoptimized_avg);
    
    if optimized_avg < unoptimized_avg {
        let improvement = ((unoptimized_avg.as_nanos() - optimized_avg.as_nanos()) as f64 / unoptimized_avg.as_nanos() as f64) * 100.0;
        info!("  Improvement: {:.2}%", improvement);
    }
}

#[test]
fn test_recursive_type_performance() {
    common::tracing::setup();
    info!("Testing performance with recursive/nested types");

    let (_context, generator) = setup_perf_llvm_generator();
    
    // Create increasingly nested types
    let mut nested_types = Vec::new();
    let mut current_type = Type::Normie;
    
    for depth in 1..=10 {
        current_type = Type::Array(Box::new(current_type), 2);
        nested_types.push((depth, current_type.clone()));
    }
    
    let mut measurement = PerfMeasurement::new("Nested Type Mangling");
    
    for (depth, nested_type) in &nested_types {
        measurement.start_iteration();
        let _mangled = generator.type_to_mangled_name(nested_type);
        measurement.end_iteration();
        
        debug!("Depth {}: mangling completed", depth);
    }
    
    measurement.report();
}
