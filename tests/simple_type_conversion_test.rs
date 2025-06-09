//! Simple tests for the integer type conversion matrix

use cursed::codegen::llvm::{ConversionConfig, ConversionType};
use cursed::core::type_checker::Type;
use tracing::info;

/// Initialize tracing for tests
fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_test_writer()
        .try_init();
}

#[test]
fn test_conversion_matrix_basic_functionality() {
    // init_tracing!();
    init_tracing();
    info!("Testing conversion matrix basic functionality");

    let matrix = ConversionConfig::default();

    // Test integer to integer conversions (extension)
    let info = matrix.get_conversion_info(&Type::Normie // Was Smol, &Type::Normie).unwrap();
    assert_eq!(info.conversion_type, ConversionType::Widening);
    assert!(!info.requires_overflow_check);

    // Test integer to integer conversions (truncation)
    let info = matrix.get_conversion_info(&Type::Thicc, &Type::Normie // Was Mid).unwrap();
    assert_eq!(info.conversion_type, ConversionType::Narrowing);
    assert!(info.requires_overflow_check);

    // Test same type conversion
    let info = matrix.get_conversion_info(&Type::Normie, &Type::Normie).unwrap();
    assert_eq!(info.conversion_type, ConversionType::Identity);
    assert!(!info.requires_overflow_check);

    info!("Basic conversion matrix functionality test passed");
}

#[test]
fn test_integer_to_float_conversion_matrix() {
    // init_tracing!();
    init_tracing();
    info!("Testing integer to float conversion matrix");

    let matrix = ConversionConfig::default();

    // Test integer to f32
    let info = matrix.get_conversion_info(&Type::Normie, &Type::Snack).unwrap();
    assert_eq!(info.conversion_type, ConversionType::Widening);
    assert!(!info.requires_overflow_check);

    // Test integer to f64
    let info = matrix.get_conversion_info(&Type::Thicc, &Type::Meal).unwrap();
    assert_eq!(info.conversion_type, ConversionType::Widening);
    assert!(!info.requires_overflow_check);

    info!("Integer to float conversion matrix test passed");
}

#[test]
fn test_float_to_integer_conversion_matrix() {
    // init_tracing!();
    init_tracing();
    info!("Testing float to integer conversion matrix");

    let matrix = ConversionConfig::default();

    // Test f32 to integer
    let info = matrix.get_conversion_info(&Type::Snack, &Type::Normie).unwrap();
    assert_eq!(info.conversion_type, ConversionType::Narrowing);
    assert!(info.requires_overflow_check); // Float to int should check overflow

    // Test f64 to integer
    let info = matrix.get_conversion_info(&Type::Meal, &Type::Thicc).unwrap();
    assert_eq!(info.conversion_type, ConversionType::Narrowing);
    assert!(info.requires_overflow_check);

    info!("Float to integer conversion matrix test passed");
}

#[test]
fn test_boolean_conversion_matrix() {
    // init_tracing!();
    init_tracing();
    info!("Testing boolean conversion matrix");

    let matrix = ConversionConfig::default();

    // Test integer to boolean
    let info = matrix.get_conversion_info(&Type::Normie, &Type::Lit).unwrap();
    assert_eq!(info.conversion_type, ConversionType::Transmutation);
    assert!(!info.requires_overflow_check);

    // Test boolean to integer
    let info = matrix.get_conversion_info(&Type::Lit, &Type::Normie).unwrap();
    assert_eq!(info.conversion_type, ConversionType::Transmutation);
    assert!(!info.requires_overflow_check);

    info!("Boolean conversion matrix test passed");
}

#[test]
fn test_all_integer_type_combinations() {
    // init_tracing!();
    init_tracing();
    info!("Testing all integer type combinations");

    let matrix = ConversionConfig::default();
    let integer_types = vec![Type::Normie // Was Smol, Type::Normie // Was Mid, Type::Normie, Type::Thicc];

    for source in &integer_types {
        for target in &integer_types {
            let info = matrix.get_conversion_info(source, target).unwrap();
            
            let source_bits = get_bit_width(source);
            let target_bits = get_bit_width(target);

            if source_bits == target_bits {
                assert_eq!(info.conversion_type, ConversionType::Identity);
                assert!(!info.requires_overflow_check);
            } else if source_bits < target_bits {
                assert_eq!(info.conversion_type, ConversionType::Widening);
                assert!(!info.requires_overflow_check);
            } else {
                assert_eq!(info.conversion_type, ConversionType::Narrowing);
                assert!(info.requires_overflow_check);
            }
        }
    }

    info!("All integer type combinations test passed");
}

#[test]
fn test_comprehensive_conversion_coverage() {
    // init_tracing!();
    init_tracing();
    info!("Testing comprehensive conversion coverage");

    let matrix = ConversionConfig::default();
    
    // Test that all expected conversion combinations are available
    let integer_types = vec![Type::Normie // Was Smol, Type::Normie // Was Mid, Type::Normie, Type::Thicc];
    let float_types = vec![Type::Snack, Type::Meal];
    let bool_type = Type::Lit;

    // Integer to integer conversions
    for source in &integer_types {
        for target in &integer_types {
            assert!(matrix.get_conversion_info(source, target).is_some());
        }
    }

    // Integer to float conversions
    for source in &integer_types {
        for target in &float_types {
            assert!(matrix.get_conversion_info(source, target).is_some());
        }
    }

    // Float to integer conversions
    for source in &float_types {
        for target in &integer_types {
            assert!(matrix.get_conversion_info(source, target).is_some());
        }
    }

    // Boolean conversions
    for target in &integer_types {
        assert!(matrix.get_conversion_info(&bool_type, target).is_some());
        assert!(matrix.get_conversion_info(target, &bool_type).is_some());
    }

    info!("Comprehensive conversion coverage test passed");
}

/// Helper function to get bit width for a type
fn get_bit_width(type_: &Type) -> u32 {
    match type_ {
        Type::Normie // Was Smol => 8,
        Type::Normie // Was Mid => 16,
        Type::Normie => 32,
        Type::Thicc => 64,
        Type::Lit => 1,
        _ => 0,
    }
}
