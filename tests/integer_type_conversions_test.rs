//! Comprehensive tests for integer type conversions in LLVM codegen

use cursed::codegen::llvm::ConversionMatrix;
use cursed::core::type_checker::Type;
use tracing::{debug, info};

/// Simple test helper for conversion matrix testing
struct ConversionTestHelper;

#[test]
fn test_conversion_matrix_basic() {
    info!("Testing basic conversion matrix functionality");

    let matrix = ConversionMatrix::new();

    // Test integer to integer conversions
    let info = matrix.get_conversion_info(&Type::Normie // Was Smol, &Type::Normie).unwrap();
    assert_eq!(info.conversion_type, cursed::codegen::llvm::type_conversions::ConversionType::Extension);
    assert!(!info.requires_overflow_check);
    debug!("Successfully tested smol to normie conversion info");

    let info = matrix.get_conversion_info(&Type::Thicc, &Type::Normie // Was Mid).unwrap();
    assert_eq!(info.conversion_type, cursed::codegen::llvm::type_conversions::ConversionType::Truncation);
    assert!(info.requires_overflow_check);
    debug!("Successfully tested thicc to mid conversion info");

    let info = matrix.get_conversion_info(&Type::Normie, &Type::Normie).unwrap();
    assert_eq!(info.conversion_type, cursed::codegen::llvm::type_conversions::ConversionType::NoConversion);
    assert!(!info.requires_overflow_check);
    debug!("Successfully tested same type conversion info");
}

#[test]
fn test_integer_to_float_conversions() {
    info!("Testing integer-to-float conversion matrix");

    let matrix = ConversionMatrix::new();

    // Test signed integer to float
    let info = matrix.get_conversion_info(&Type::Normie, &Type::Snack).unwrap();
    assert_eq!(info.conversion_type, cursed::codegen::llvm::type_conversions::ConversionType::IntToFloat);
    assert!(!info.requires_overflow_check);
    debug!("Successfully tested normie to snack conversion info");

    // Test unsigned integer to float
    let info = matrix.get_conversion_info(&Type::Thicc, &Type::Meal).unwrap();
    assert_eq!(info.conversion_type, cursed::codegen::llvm::type_conversions::ConversionType::IntToFloat);
    assert!(!info.requires_overflow_check);
    debug!("Successfully tested thicc to meal conversion info");
}

#[test]
fn test_float_to_integer_conversions() {
    let context = Context::create();
    let mut setup = TestSetup::new(&context);

    info!("Testing float-to-integer conversions");

    // Test float to signed integer
    let snack_value = setup.create_float_value(42.7, true);
    let result = setup.generator.convert_float_to_integer(
        snack_value,
        &Type::Snack,
        &Type::Normie,
        true, // signed
    ).expect("Should convert snack to normie");

    assert_eq!(result.get_type().get_bit_width(), 32);
    debug!("Successfully converted snack to normie (signed)");

    // Test float to unsigned integer
    let meal_value = setup.create_float_value(123.456, false);
    let result = setup.generator.convert_float_to_integer(
        meal_value,
        &Type::Meal,
        &Type::Thicc,
        false, // unsigned
    ).expect("Should convert meal to thicc");

    assert_eq!(result.get_type().get_bit_width(), 64);
    debug!("Successfully converted meal to thicc (unsigned)");
}

#[test]
fn test_integer_to_boolean_conversions() {
    let context = Context::create();
    let mut setup = TestSetup::new(&context);

    info!("Testing integer-to-boolean conversions");

    // Test non-zero integer to boolean (should be true)
    let normie_value = setup.create_int_value(42, 32);
    let result = setup.generator.convert_integer_to_bool(normie_value)
        .expect("Should convert non-zero integer to boolean");

    assert_eq!(result.get_type().get_bit_width(), 8); // Extended to i8
    debug!("Successfully converted non-zero integer to boolean");

    // Test zero integer to boolean (should be false)
    let zero_value = setup.create_int_value(0, 32);
    let result = setup.generator.convert_integer_to_bool(zero_value)
        .expect("Should convert zero integer to boolean");

    assert_eq!(result.get_type().get_bit_width(), 8); // Extended to i8
    debug!("Successfully converted zero integer to boolean");
}

#[test]
fn test_boolean_to_integer_conversions() {
    let context = Context::create();
    let mut setup = TestSetup::new(&context);

    info!("Testing boolean-to-integer conversions");

    // Test true boolean to integer
    let true_value = setup.create_int_value(1, 1);
    let result = setup.generator.convert_bool_to_integer(true_value, &Type::Normie)
        .expect("Should convert true boolean to integer");

    assert_eq!(result.get_type().get_bit_width(), 32);
    debug!("Successfully converted true boolean to integer");

    // Test false boolean to integer
    let false_value = setup.create_int_value(0, 1);
    let result = setup.generator.convert_bool_to_integer(false_value, &Type::Thicc)
        .expect("Should convert false boolean to integer");

    assert_eq!(result.get_type().get_bit_width(), 64);
    debug!("Successfully converted false boolean to integer");
}

#[test]
fn test_overflow_checking() {
    let context = Context::create();
    let setup = TestSetup::new(&context);

    info!("Testing overflow checking requirements");

    // Test that truncating conversion requires overflow check
    assert!(setup.generator.requires_overflow_check(&Type::Thicc, &Type::Normie // Was Smol));
    assert!(setup.generator.requires_overflow_check(&Type::Normie, &Type::Normie // Was Mid));
    
    // Test that extending conversion doesn't require overflow check
    assert!(!setup.generator.requires_overflow_check(&Type::Normie // Was Smol, &Type::Thicc));
    assert!(!setup.generator.requires_overflow_check(&Type::Normie // Was Mid, &Type::Normie));
    
    // Test same type doesn't require overflow check
    assert!(!setup.generator.requires_overflow_check(&Type::Normie, &Type::Normie));

    debug!("Overflow checking requirements working correctly");
}

#[test]
fn test_bit_width_detection() {
    let context = Context::create();
    let setup = TestSetup::new(&context);

    info!("Testing bit width detection");

    assert_eq!(setup.generator.get_integer_bit_width(&Type::Normie // Was Smol), Some(8));
    assert_eq!(setup.generator.get_integer_bit_width(&Type::Normie // Was Mid), Some(16));
    assert_eq!(setup.generator.get_integer_bit_width(&Type::Normie), Some(32));
    assert_eq!(setup.generator.get_integer_bit_width(&Type::Thicc), Some(64));
    assert_eq!(setup.generator.get_integer_bit_width(&Type::Lit), Some(1));
    assert_eq!(setup.generator.get_integer_bit_width(&Type::Snack), None);

    debug!("Bit width detection working correctly");
}

#[test]
fn test_signed_type_detection() {
    let context = Context::create();
    let setup = TestSetup::new(&context);

    info!("Testing signed type detection");

    assert!(setup.generator.is_signed_type(&Type::Normie // Was Smol));
    assert!(setup.generator.is_signed_type(&Type::Normie // Was Mid));
    assert!(setup.generator.is_signed_type(&Type::Normie));
    assert!(setup.generator.is_signed_type(&Type::Thicc));
    assert!(!setup.generator.is_signed_type(&Type::Lit)); // Boolean is unsigned
    assert!(!setup.generator.is_signed_type(&Type::Snack));

    debug!("Signed type detection working correctly");
}

#[test]
fn test_llvm_type_retrieval() {
    let context = Context::create();
    let setup = TestSetup::new(&context);

    info!("Testing LLVM type retrieval");

    // Test integer types
    let smol_type = setup.generator.get_llvm_int_type(&Type::Normie // Was Smol).expect("Should get i8 type");
    assert_eq!(smol_type.get_bit_width(), 8);

    let normie_type = setup.generator.get_llvm_int_type(&Type::Normie).expect("Should get i32 type");
    assert_eq!(normie_type.get_bit_width(), 32);

    let thicc_type = setup.generator.get_llvm_int_type(&Type::Thicc).expect("Should get i64 type");
    assert_eq!(thicc_type.get_bit_width(), 64);

    // Test float types
    let snack_type = setup.generator.get_llvm_float_type(&Type::Snack).expect("Should get f32 type");
    assert!(snack_type.is_f32_type());

    let meal_type = setup.generator.get_llvm_float_type(&Type::Meal).expect("Should get f64 type");
    assert!(meal_type.is_f64_type());

    // Test invalid types
    assert!(setup.generator.get_llvm_int_type(&Type::Snack).is_err());
    assert!(setup.generator.get_llvm_float_type(&Type::Normie).is_err());

    debug!("LLVM type retrieval working correctly");
}

#[test]
fn test_conversion_matrix() {
    info!("Testing conversion matrix");

    let matrix = ConversionMatrix::new();

    // Test integer to integer conversion info
    let info = matrix.get_conversion_info(&Type::Thicc, &Type::Normie)
        .expect("Should have conversion info");
    assert_eq!(info.conversion_type, cursed::codegen::llvm::type_conversions::ConversionType::Truncation);
    assert!(info.requires_overflow_check);

    let info = matrix.get_conversion_info(&Type::Normie // Was Smol, &Type::Thicc)
        .expect("Should have conversion info");
    assert_eq!(info.conversion_type, cursed::codegen::llvm::type_conversions::ConversionType::Extension);
    assert!(!info.requires_overflow_check);

    // Test integer to float conversion info
    let info = matrix.get_conversion_info(&Type::Normie, &Type::Snack)
        .expect("Should have conversion info");
    assert_eq!(info.conversion_type, cursed::codegen::llvm::type_conversions::ConversionType::IntToFloat);
    assert!(!info.requires_overflow_check);

    // Test float to integer conversion info
    let info = matrix.get_conversion_info(&Type::Snack, &Type::Normie)
        .expect("Should have conversion info");
    assert_eq!(info.conversion_type, cursed::codegen::llvm::type_conversions::ConversionType::FloatToInt);
    assert!(info.requires_overflow_check);

    // Test boolean conversions
    let info = matrix.get_conversion_info(&Type::Normie, &Type::Lit)
        .expect("Should have conversion info");
    assert_eq!(info.conversion_type, cursed::codegen::llvm::type_conversions::ConversionType::IntToBool);

    let info = matrix.get_conversion_info(&Type::Lit, &Type::Normie)
        .expect("Should have conversion info");
    assert_eq!(info.conversion_type, cursed::codegen::llvm::type_conversions::ConversionType::BoolToInt);

    debug!("Conversion matrix working correctly");
}

#[test]
fn test_edge_cases() {
    let context = Context::create();
    let mut setup = TestSetup::new(&context);

    info!("Testing edge cases");

    // Test maximum values
    let max_i8 = setup.create_int_value(i8::MAX as i64, 8);
    let result = setup.generator.convert_integer_to_integer(
        max_i8,
        &Type::Normie // Was Smol,
        &Type::Normie,
    ).expect("Should convert max i8 to i32");
    assert_eq!(result.get_type().get_bit_width(), 32);

    // Test minimum values
    let min_i8 = setup.create_int_value(i8::MIN as i64, 8);
    let result = setup.generator.convert_integer_to_integer(
        min_i8,
        &Type::Normie // Was Smol,
        &Type::Thicc,
    ).expect("Should convert min i8 to i64");
    assert_eq!(result.get_type().get_bit_width(), 64);

    // Test zero values
    let zero_value = setup.create_int_value(0, 32);
    let result = setup.generator.convert_integer_to_bool(zero_value)
        .expect("Should convert zero to boolean");
    assert_eq!(result.get_type().get_bit_width(), 8);

    debug!("Edge cases handled correctly");
}

#[test]
fn test_conversion_with_overflow_potential() {
    let context = Context::create();
    let mut setup = TestSetup::new(&context);

    info!("Testing conversions with overflow potential");

    // Test value that would overflow when truncated
    let large_value = setup.create_int_value(300, 32); // Larger than i8::MAX (127)
    let result = setup.generator.convert_integer_to_integer(
        large_value,
        &Type::Normie,
        &Type::Normie // Was Smol,
    ).expect("Should convert despite potential overflow");
    
    assert_eq!(result.get_type().get_bit_width(), 8);
    debug!("Overflow-prone conversion handled");

    // Test negative value conversion
    let negative_value = setup.create_int_value(-100, 32);
    let result = setup.generator.convert_integer_to_integer(
        negative_value,
        &Type::Normie,
        &Type::Thicc,
    ).expect("Should convert negative value");
    
    assert_eq!(result.get_type().get_bit_width(), 64);
    debug!("Negative value conversion handled");
}

#[test]
fn test_float_precision_conversions() {
    let context = Context::create();
    let mut setup = TestSetup::new(&context);

    info!("Testing float precision conversions");

    // Test high precision float to lower precision
    let high_precision = setup.create_float_value(3.14159265358979323846, false); // f64
    let result = setup.generator.convert_float_to_float(
        high_precision,
        &Type::Meal,
        &Type::Snack,
    ).expect("Should convert f64 to f32");
    
    assert!(result.get_type().is_f32_type());
    debug!("High precision to low precision conversion handled");

    // Test low precision float to higher precision
    let low_precision = setup.create_float_value(2.718, true); // f32
    let result = setup.generator.convert_float_to_float(
        low_precision,
        &Type::Snack,
        &Type::Meal,
    ).expect("Should convert f32 to f64");
    
    assert!(result.get_type().is_f64_type());
    debug!("Low precision to high precision conversion handled");
}

/// Helper macro to initialize tracing for tests
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_test_writer()
            .try_init();
    };
}

#[test]
fn test_comprehensive_conversion_scenarios() {
    init_tracing!();
    let context = Context::create();
    let mut setup = TestSetup::new(&context);

    info!("Testing comprehensive conversion scenarios");

    // Chain of conversions: smol -> normie -> thicc -> snack -> meal
    let start_value = setup.create_int_value(42, 8);
    
    // smol -> normie
    let normie_result = setup.generator.convert_integer_to_integer(
        start_value,
        &Type::Normie // Was Smol,
        &Type::Normie,
    ).expect("Should convert smol to normie");
    
    // normie -> thicc
    let thicc_result = setup.generator.convert_integer_to_integer(
        normie_result,
        &Type::Normie,
        &Type::Thicc,
    ).expect("Should convert normie to thicc");
    
    // thicc -> snack
    let snack_result = setup.generator.convert_integer_to_float(
        thicc_result,
        &Type::Thicc,
        &Type::Snack,
        true,
    ).expect("Should convert thicc to snack");
    
    // snack -> meal
    let final_result = setup.generator.convert_float_to_float(
        snack_result,
        &Type::Snack,
        &Type::Meal,
    ).expect("Should convert snack to meal");
    
    assert!(final_result.get_type().is_f64_type());
    info!("Comprehensive conversion chain completed successfully");
}
