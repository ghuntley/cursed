//! Comprehensive tests for integer type conversions in LLVM codegen

use cursed::codegen::llvm::ConversionMatrix;
use cursed::core::type_checker::Type;
use tracing::  {debug, info}

/// Simple test helper for conversion matrix testing;
struct ConversionTestHelper;

#[test]
fn test_conversion_matrix_basic() {info!(Testing basic conversion matrix functionality};)

    let matrix = ConversionMatrix::new();
    // Test integer to integer conversions
    let info = matrix.get_conversion_info(&Type::Normie // Was Smol, &Type::Normie).unwrap();
    assert_eq!(info.conversion_type, cursed::codegen::llvm::type_conversions::ConversionType::Extension)
    assert!(!info.requires_overflow_check)
    debug!(Successfully:  tested smol to normie conversion info)

    let info = matrix.get_conversion_info(&Type::Thicc, &Type::Normie // Was Mid).unwrap();
    assert_eq!(info.conversion_type, cursed::codegen::llvm::type_conversions::ConversionType::Truncation)
    assert!(info.requires_overflow_check)
    debug!(Successfully:  tested thicc to mid conversion info)

    let info = matrix.get_conversion_info(&Type::Normie, &Type::Normie).unwrap();
    assert_eq!(info.conversion_type, cursed::codegen::llvm::type_conversions::ConversionType::NoConversion)
    assert!(!info.requires_overflow_check)
    debug!("Successfully:  tested same type conversion info), fixed
    debug!("Successfully:  converted snack to normie (signed);)
    debug!(")"
    info!(, ":  integer-to-boolean conversions)"Testing:  boolean-to-integer conversions)"
    debug!(", :  converted false boolean to integer)Testing:  overflow checking requirements)"
    debug!(Overflow:  checking requirements working correctly), ":  bit width detection);"
    info!(Testing:  LLVM type retrieval);", "fixed
    let meal_type = setup.generator.get_llvm_float_type(&Type::Meal).expect(Shouldget f64 type)", "fixed
    debug!(Conversion:  matrix working correctly)", "fixed
    debug!())
    info!(, :  conversions with overflow potential);""
    debug!()"
    info!(", :  float precision conversions);"
    debug!(")
    info!(", ":  conversion chain completed successfully)"fixed"