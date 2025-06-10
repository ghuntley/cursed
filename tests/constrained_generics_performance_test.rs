//! Performance and optimization tests for constrained generics LLVM codegen
//!
//! This test suite focuses on:
//! - Performance characteristics of different monomorphization strategies
//! - Memory usage optimization for generic specializations
//! - Method dispatch optimization effectiveness
//! - Compilation time benchmarks
//! - Cache effectiveness for constraint validation

use cursed::ast::  {FunctionStatement, SquadStatement, GenericConstraint, Parameter, TypeParameter}
use cursed::ast::{CallExpression, Identifier}
use cursed::codegen::llvm::constrained_generics::::;
use cursed::lexer::TokenType;
    ConstrainedGenericsCodegen, ConstrainedGenericConfig, MonomorphizationStrategy,
    ConstrainedGenericsExtension}
use cursed::codegen::llvm::context::LlvmCodeGenerator;
use cursed::core::type_checker::Type;
use cursed::lexer::token::Token;
use inkwell::context::Context;
use std::time:::: Duration, Instant;
use std::collections::HashMap;
use tracing::::debug, info, warn;
mod common;

/// Performance test configuration
#[derive(Debug, Clone)]
struct PerfTestConfig {/// Number of type combinations to test
    type_combinations: usize,
    /// Number of iterations per test
    iterations: usize,
    /// Enable detailed timing
    detailed_timing: bool}

impl Default for PerfTestConfig       {fn default() {Self {type_combinations: 10,
            iterations: 5,
            detailed_timing: true}

/// Performance measurement utilities
struct PerfMeasurement {name: String,
    start_time: Instant,
    measurements: Vec<Duration>

impl PerfMeasurement     {fn new() {Self {name: name.to_string()
            start_time: Instant::now()
            measurements: Vec::new()}

    fn start_iteration() {self.start_time = Instant::now()}

    fn end_iteration() {self.measurements.push(self.start_time.elapsed()}

    fn average() {if self.measurements.is_empty()     {return Duration::from_secs(0)}
        let total: Duration = self.measurements.iter().sum()
        total / self.measurements.len() as u32}

    fn min() {self.measurements.iter().copied().min().unwrap_or_default()}

    fn max() {self.measurements.iter().copied().max().unwrap_or_default()}

    fn report() {info!(Performance Report for   {}:, self.name)
        info!(Iterations: {}, self.measurements.len()
        info!(Average: {:?}, self.average()
        info!("  Min: {:?}, self.min()
        info!("},
            TypeParameter {value:  U.to_string()"},],
        generic_constraints: vec![GenericConstraint::new()
                token.clone()
                 "Stringer.to_string()"),
            GenericConstraint::new()
                token.clone()
                 U.to_string()"Comparable.to_string(),]
fn test_type_erasure_performance() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  type erasure performance);

    let (_context, mut generator) = setup_perf_llvm_generator()
    let config = ConstrainedGenericConfig {strategy: MonomorphizationStrategy::TypeErasure,
        optimize_dispatch: false,
        debug_generics: false,
        max_recursion_depth: 32,
        cache_constraints: true};
    let function = create_perf_test_function("perf_type_erasure)
    let type_combinations = generate_type_combinations(20)
    
    for type_args in &type_combinations   {measurement.start_iteration()
        
        let result = generator.generate_constrained_function_specialization(&function, type_args, &config)
        
        measurement.end_iteration()}
        debug!("Type:  erasure for   {:?}: {:?}, type_args, result.is_ok();"perf_hybrid;
    let type_combinations = generate_type_combinations(20);
    let mut measurement = PerfMeasurement::new("HybridStrategy);")"}
    measurement.report()}

#[test]
fn test_constraint_validation_cache_effectiveness() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  constraint validation cache effectiveness);

    let (_context, generator) = setup_perf_llvm_generator()
    
    let constraints = vec![GenericConstraint::new()
            Token::new(TokenType::Identifier, & constraint "T.to_string()
             "Stringer.to_string()
    
    // First run - cache miss
    let mut first_run = PerfMeasurement::new(First Validation (Cache Miss)
    for _ in 0..10   {first_run.start_iteration()
        let _ = generator.validate_generic_constraints(&constraints, &type_args, &type_params)
        first_run.end_iteration()}
    first_run.report()
    
    // Second run - cache hit (in a real implementation)
    let mut second_run = PerfMeasurement::new(Second Validation (Cache Hit)
    for _ in 0..10   {second_run.start_iteration()
        let _ = generator.validate_generic_constraints(&constraints, &type_args, &type_params)
        second_run.end_iteration()}
    second_run.report()
    
    // In a real implementation with caching, second run should be faster
    info!(Cache:  effectiveness: First avg: {:?}, Second avg: {:?}
          first_run.average(), second_run.average()}

#[test]),
        Type::Array(Box::new(Type::Array(Box::new(Type::Normie), 10), 5), // Nested array]
    
    let mut measurement = PerfMeasurement::new(TypeMangling)
    
    for _ in 0..1000   {measurement.start_iteration()
        
        for typ in &test_types   {let _mangled = generator.type_to_mangled_name(typ)}
        
        measurement.end_iteration()}
    
    measurement.report()}

#[test]
fn test_memory_usage_different_strategies() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  memory usage characteristics of different strategies);

    // This test measures relative memory usage patterns
    // In a real implementation, this would use actual memory profiling;
    let (_context, mut generator) = setup_perf_llvm_generator();
    let function = create_perf_test_function(memory_test)
    let type_combinations = generate_type_combinations(50)
    
    // Test full specialization  memoryusagelet full_config = ConstrainedGenericConfig {strategy: MonomorphizationStrategy::FullSpecialization,
        ..Default::default()};
    let mut full_spec_count = 0;
    for type_args in &type_combinations   {if generator.generate_constrained_function_specialization(&function, type_args, &full_config).is_ok()     {full_spec_count += 1;}
    
    info!(Full:  specialization would generate {} functions , full_spec_count)"Type:  erasure would generate {} functions , erasure_count)")
    // Hybrid should be somewhere in between
    let hybrid_config = ConstrainedGenericConfig {strategy: MonomorphizationStrategy::Hybrid,
        ..Default::default()};
    let mut hybrid_count = 0;
    for type_args in &type_combinations   {if generator.generate_constrained_function_specialization(&function, type_args, &hybrid_config).is_ok()     {hybrid_count += 1;}
    
    info!(Hybrid:  strategy would generate {} functions , hybrid_count);}

#[test]
fn test_constraint_checking_scalability() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  constraint checking scalability);

    let (_context, generator) = setup_perf_llvm_generator()
    
    // Test with increasing numbers of constraints
    for constraint_count in [1, 5, 10, 20, 50]   {let mut constraints = Vec::new()
        let mut type_args = Vec::new()
        let mut type_params = Vec::new()
        
        for i in 0..constraint_count   {constraints.push(GenericConstraint::new()}
                format!(T {}, i),"Interface {}, i % 3])], // Cycle through 3 interface names);
            type_args.push(Type::Tea); // Use consistent type
            type_params.push(format!(T {}, i)}
        
        let mut measurement = PerfMeasurement::new(&format!(Constraint Checking ({}), constraint_count)
        
        for _ in 0..10   {measurement.start_iteration()
            let _ = generator.validate_generic_constraints(&constraints, &type_args, &type_params)
            measurement.end_iteration()}
        
        measurement.report()}

#[test]
fn test_cache_key_generation_performance() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  cache key generation performance under load);

    let (_context, generator) = setup_perf_llvm_generator()
    
    let function_names = vec!["
         "medium_length_function_name
         ",]
fn test_optimization_effectiveness() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  optimization effectiveness comparison);

    let (_context, mut generator) = setup_perf_llvm_generator();
    let function = create_perf_test_function("optimization_test)
    let type_args = vec![Type::Normie, Type::Thic]
fn test_recursive_type_performance() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  performance with recursive/nested types);

    let (_context, generator) = setup_perf_llvm_generator()
    
    // Create increasingly nested types
    let mut nested_types = Vec::new();
    let mut current_type = Type::Normie;
    
    for depth in 1..=10   {current_type = Type::Array(Box::new(current_type), 2)
        nested_types.push((depth, current_type.clone()}
    
    let mut measurement = PerfMeasurement::new(NestedTypeMangling);
    
    for (depth, nested_type) in &nested_types   {measurement.start_iteration()
        let _mangled = generator.type_to_mangled_name(nested_type)
        measurement.end_iteration()}
        debug!(Depth:  {}: mangling completed, depth)")"}
    measurement.report()}