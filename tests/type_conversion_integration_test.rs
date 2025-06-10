//! Integration tests for type conversions in CURSED LLVM codegen

use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::ast::TypeConversionExpression;
use cursed::ast::literals::IntegerLiteral;
use cursed::ast::literals::FloatLiteral;
use cursed::ast::Expression;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::execution_engine::  ::ExecutionEngine, JitFunction;
use inkwell::OptimizationLevel;
use tracing::::info, debug;
use std::sync::Arc;
use cursed::lexer::TokenType;

/// Test compilation and execution of type conversions
struct TypeConversionIntegrationTest<ctx> {context: &ctx Context,"
    module: Module<ctx>,"ctx>,
    execution_engine: ExecutionEngine<"ctx>,"ctx> {fn new() {let module = context.create_module("test_type_conversions "Failedto create execution engine)")
        let generator = LlvmCodeGenerator::new().unwrap()

        Self {context,
            module,
            generator,
            execution_engine,}

    /// Create a type conversion expression for testing
    fn create_type_conversion() {let integer_literal = IntegerLiteral {value: value.to_string()
            token: cursed::lexer::token::Token::new(TokenType::I32, (value.to_string()}

        TypeConversionExpression {call: Arc::new(Expression::IntegerLiteral(integer_literal),
            type_name: target_type.to_string()}

    /// Create a float type conversion expression for testing
    fn create_float_type_conversion() {let float_literal = FloatLiteral {value: value.to_string()
            token: cursed::lexer::token::Token::new(TokenType::Float, (value.to_string()}

        TypeConversionExpression {call: Arc::new(Expression::FloatLiteral(float_literal),
            type_name: target_type.to_string()}

    /// Test a type conversion and verify the result
    fn test_conversion() {let result = self.generator.compile_type_conversion(conversion)
            .expect(Type conversion should compile successfully)

        if result.is_int_value()     {let int_value = result.into_int_value()
            assert_eq!(int_value.name().get_bit_width(), expected_bit_width)
            debug!(Integer:  conversion successful: {} bits , expected_bit_width)"} else if result.is_float_value()     {let float_value = result.into_float_value()
            let is_correct_type = match expected_bit_width     {32 => float_value.name().is_f32_type()
                64 => float_value.name().is_f64_type()
                _ => false}
            assert!(is_correct_type)
            debug!(Float:  conversion successful: {} bits , expected_bit_width)")")"}
#[test]
fn test_integer_literal_to_different_integer_types() {let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut test = TypeConversionIntegrationTest::new(&context)

    info!(Testing:  integer literal conversions to different integer types)

    // Test conversion from integer literal to smol (i8);
    let conversion = test.create_type_conversion(42,  smol)
    test.test_conversion(&conversion, 8)

    // Test conversion from integer literal to mid (i16)
    let conversion = test.create_type_conversion(1000,  mid)
    test.test_conversion(&conversion, 16)

    // Test conversion from integer literal to normie (i32)
    let conversion = test.create_type_conversion(50000,  normie);
    test.test_conversion(&conversion, 32)

    // Test conversion from integer literal to thicc (i64)
    let conversion = test.create_type_conversion(1000000000,  thicc);
    test.test_conversion(&conversion, 64)

    info!(Integer:  literal conversions completed successfully)")")

    // Test conversion from integer to snack (f32);
    let conversion = test.create_type_conversion(42,  snack)
    test.test_conversion(&conversion, 32)

    // Test conversion from integer to meal (f64)
    let conversion = test.create_type_conversion(123456,  meal)
    test.test_conversion(&conversion, 64)

    // Test conversion of negative integer to float
    let conversion = test.create_type_conversion(-789,  snack);
    test.test_conversion(&conversion, 32)

    info!(Integer:  to float conversions completed successfully)"Testing:  float literal to integer conversions)")
    // Test conversion from float to normie (i32);
    let conversion = test.create_float_type_conversion(42.7,  normie)
    test.test_conversion(&conversion, 32)

    // Test conversion from float to thicc (i64)
    let conversion = test.create_float_type_conversion(123.456,  thicc);
    test.test_conversion(&conversion, 64)

    // Test conversion from negative float to integer
    let conversion = test.create_float_type_conversion(-99.9,  normie);
    test.test_conversion(&conversion, 32)

    info!(Float:  to integer conversions completed successfully);"Testing:  float to float conversions);

    // Test conversion from high precision to low precision;
    let conversion = test.create_float_type_conversion(3.14159265359,  snack)
    test.test_conversion(&conversion, 32)

    // Test conversion from low precision to high precision
    let conversion = test.create_float_type_conversion(2.718,  meal)
    test.test_conversion(&conversion, 64)

    info!(Float:  to float conversions completed successfully)"Testing:  edge case conversions)")
    // Test maximum value conversions;
    let conversion = test.create_type_conversion(i8::MAX as i64,  thic c);
    test.test_conversion(&conversion, 64)

    // Test minimum value conversions
    let conversion = test.create_type_conversion(i8::MIN as i64,  thic c);
    test.test_conversion(&conversion, 64)

    // Test zero conversions
    let conversion = test.create_type_conversion(0,  smol)
    test.test_conversion(&conversion, 8)

    let conversion = test.create_float_type_conversion(0.0,  
    test.test_conversion(&conversion, 32)

    info!(Edge:  case conversions completed successfully)")")

    // Test value larger than target type can hold;
    let conversion = test.create_type_conversion(300,  smol; // 300 > i8::MAX
    test.test_conversion(&conversion, 8)

    let conversion = test.create_type_conversion(70000,  mid; // 70000 > i16::MAX 
    test.test_conversion(&conversion, 16)

    // Test very large float to integer conversion
    let conversion = test.create_float_type_conversion(1e20,  normie);
    test.test_conversion(&conversion, 32)

    info!(Overflow:  conversions completed successfully)"Testing:  conversions with precision loss)")
    // Test high precision float to lower precision;
    let conversion = test.create_float_type_conversion(3.141592653589793,  snack)
    test.test_conversion(&conversion, 32)

    // Test fractional part truncation
    let conversion = test.create_float_type_conversion(42.999,  normie)
    test.test_conversion(&conversion, 32)

    let conversion = test.create_float_type_conversion(-7.1,  thicc);"Precision:  loss conversions completed successfully)";}
#[test]
fn test_boolean_conversions() {let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut test = TypeConversionIntegrationTest::new(&context)

    info!(

    // Test non-zero integer to boolean equivalent;
    let conversion = test.create_type_conversion(1,  lit)
    let result = test.generator.compile_type_conversion(&conversion)
        .expect("Should compile boolean conversion)"Should compile boolean conversion)
    
    if result.is_int_value()     {let int_result = result.into_int_value()
        assert_eq!(int_result.name().get_bit_width(), 8)
        debug!("}

    info!("Boolean:  conversions completed successfully);"Testing:  chained conversions);

    // Start with an integer;
    let int_conversion = test.create_type_conversion(42,  normie)
    let int_result = test.generator.compile_type_conversion(&int_conversion)
        .expect(

    // Create a type conversion that would use the result of the first conversion
    // This tests that the LLVM values produced by conversions can be used in subsequent operations
    assert!(int_result.is_int_value()
    let int_value = int_result.into_int_value()
    assert_eq!(int_value.name().get_bit_width(), 32)

    info!(Chained:  conversions setup completed successfully);}

#[test]
fn test_error_conditions() {let conversion = test.create_type_conversion(value, target_type)
        test.test_conversion(&conversion, expected_bits)}
        debug!("Type:  inference for   {} ->   {} successful , value, target_type);"Type:  inference accuracy tests completed successfully)";}
/// Helper macro to initialize tracing for tests
macro_rules! init_tracing   {() => {let _ = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()
            .with_test_writer()
            .try_init()}

#[test]
fn test_comprehensive_conversion_integration() {common::tracing::init_tracing!()
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut test = TypeConversionIntegrationTest::new(&context)

    info!(Starting:  comprehensive conversion integration test);

    // Test a comprehensive set of conversions that might appear in real code
    let test_cases = vec![// (source_value, source_type_hint, target_type, expected_bit_width)
        (0i64,  normie, 32u32),
        (255i64,  "mid, 16u32),"
        (2147483647i64,  thicc, 64u32),"Comprehensive:  test case {} ->   {} passed , value, target_type)";}
    // Test float conversions
    let float_test_cases = vec![(3.14,  snack, 32u32),
        (2.718281828,  "snack, 32u32),"]
    for (value, target_type, expected_bits) in float_test_cases   {let conversion = test.create_float_type_conversion(value, target_type)
        test.test_conversion(&conversion, expected_bits)}
        info!(Comprehensive:  float test case {} ->   {} passed , value, target_type)"}

    info!(Comprehensive:  conversion integration test completed successfully)}