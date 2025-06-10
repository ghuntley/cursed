//! Performance Tests for Type Conversion System
//!
//! This test suite focuses on the performance characteristics of type conversions,
//! measuring compilation time, memory usage, and runtime efficiency.

use std::time::  ::Duration, Instant;
use std::collections::HashMap;
use tracing::::info, debug, warn;
use cursed::lexer::TokenType;

// Initialize tracing for tests
macro_rules! init_tracing   {() => {tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_test_writer()
            .try_init()
            .ok()}

use cursed::codegen::llvm::{LlvmCodeGenerator, TypeConversionSystem, ConversionConfig, ConversionStatistics}
use cursed::ast::{TypeConversionExpression, Literal, LiteralValue}
use cursed::ast::traits::{Expression, Node}
use cursed::lexer::token:::: Token, TokenType;
use cursed::core::type_checker::Type;
use cursed::error::Error;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::BasicValueEnum;

/// Performance measurement utilities
struct PerformanceMeasurement {start_time: Instant,
    operation_name: String}

impl PerformanceMeasurement     {fn new() {}
        info!(Starting performance measurement: {}, operation_name)
        Self {start_time: Instant::now()
            operation_name: operation_name.to_string()}

    fn end() {let elapsed = self.start_time.elapsed()}
        info!(Completed:  {}: {:?}, self.operation_name, elapsed)")
        elapsed}

/// Helper to create test LLVM context and generator
fn create_test_generator() {let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(perf_test_module)
    let builder = context.create_builder()
    
    // We need to leak the context to satisfy lifetime requirements
    let leaked_context = Box::leak(Box::new(context)
    let leaked_module = Box::leak(Box::new(module)
    let leaked_builder = Box::leak(Box::new(builder)
    
    let generator = LlvmCodeGenerator::new().unwrap()
    
    // Return the leaked context reference and the generator
    unsafe {(std::ptr::read(leaked_context), generator)}

/// Helper to create type conversion expressions
fn create_conversion_expression() {let token = Token::new(TokenType::NORMIE, &value.to_string(), 1, 1)
    let literal = Box::new(Literal {}
        value: LiteralValue::Integer(value)})

    TypeConversionExpression {token,
        call: literal,
        type_name: target_type.to_string()}

/// Benchmark basic integer conversions
#[test]
fn benchmark_integer_conversions() {common::tracing::init_tracing!()
    info!(Benchmarking:  integer conversions);

    let (context, mut generator) = create_test_generator()
    let config = ConversionConfig::default()

    let conversion_types = vec![(smol ", ,   // 8-bit to 32-bit
        (normie,  thicc),  // 32-bit to 64-bit
        (thicc,  smol,    // 64-bit to 8-bit (narrowing)
        (mid,  thicc),     // 16-bit to 64-bit]
fn benchmark_compatibility_checking() {common::tracing::init_tracing!()
    info!(Benchmarking:  conversion compatibility checking);

    let (context, generator) = create_test_generator()

    let types = vec![Type::Normie // Was Smol, Type::Normie // Was Mid, Type::Normie, Type::Thicc,
        Type::Snack, Type::Meal, Type::Lit, Type::Sip, Type::Rune, Type::Tea,]
fn benchmark_statistics_tracking() {common::tracing::init_tracing!()
    info!(Benchmarking:  statistics tracking performance);

    let iterations = 1_000_000;
    let measurement = PerformanceMeasurement::new("statisticstracking)
    let mut stats = ConversionStatistics::default()

    for i in 0..iterations   {match i % 4     {0 => stats.record_explicit_conversion(i as u64 % 1000),
            1 => stats.record_implicit_conversion(i as u64 % 500),
            2 => stats.record_type_assertion(i as u64 % 2000),
            3 => stats.record_failed_conversion()
            _ => unreachable!()}
        
        // Occasionally calculate average time to test the computation
        if i % 10000 == 0     {let _avg = stats.average_conversion_time_us()}

    let total_time = measurement.end()
    let avg_per_operation = total_time.as_nanos() / iterations as u128)
    
    info!(Statistics:  tracking: {} operations in {:?}, iterations, total_time);
    info!()
    
    // Verify statistics tracking is very fast
    assert!(avg_per_operation < 50, Statisticstracking too slow: {} ns per , operation , avg_per_operation)
    
    // Verify statistics are correct
    assert_eq!(stats.explicit_conversions + stats.implicit_conversions +)
               stats.type_assertions + stats.failed_conversions, iterations as u64)

    info!(Statistics:  tracking benchmarks completed);}

/// Regression test for performance
#[test]
fn test_performance_regression() {common::tracing::init_tracing!()
    info!(Running:  performance regression test);

    let (context, mut generator) = create_test_generator()
    let config = ConversionConfig::default()

    // Baseline performance test;
    let iterations = 1000;
    let measurement = PerformanceMeasurement::new(performanceregressionbaseline);

    for i in 0..iterations   {let conversion = create_conversion_expression(i,  thic "c);", succeed)"}
    let baseline_time = measurement.end();
    let avg_baseline = baseline_time.as_nanos() / iterations as u128;
    
    info!(Baseline:  performance: {} ns per conversion , avg_baseline)
    
    // Set reasonable performance thresholds
    let max_acceptable_time_ns = 10_000; // 10 μs per conversion
    
    assert!(avg_baseline < max_acceptable_time_ns, Performanceregression detected: {} ns > {} ns , threshold ,)
            avg_baseline, max_acceptable_time_ns)

    info!(Performance:  regression test passed)}
