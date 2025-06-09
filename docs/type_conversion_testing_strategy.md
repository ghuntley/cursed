# CURSED Type Conversion Testing Strategy

## 1. Testing Overview

This document outlines a comprehensive testing strategy for the CURSED type conversion system. The testing approach covers unit tests, integration tests, performance tests, and edge case validation to ensure robust and reliable type conversion functionality.

## 2. Testing Architecture

### 2.1 Test Categories

```
tests/
├── type_conversion/
│   ├── unit/
│   │   ├── conversion_registry_test.rs
│   │   ├── conversion_validator_test.rs
│   │   ├── primitive_conversions_test.rs
│   │   ├── composite_conversions_test.rs
│   │   └── error_handling_test.rs
│   ├── integration/
│   │   ├── end_to_end_conversion_test.rs
│   │   ├── type_checker_integration_test.rs
│   │   ├── llvm_codegen_integration_test.rs
│   │   └── runtime_conversion_test.rs
│   ├── performance/
│   │   ├── conversion_benchmarks.rs
│   │   ├── memory_usage_test.rs
│   │   └── optimization_validation_test.rs
│   ├── edge_cases/
│   │   ├── overflow_underflow_test.rs
│   │   ├── precision_loss_test.rs
│   │   ├── boundary_values_test.rs
│   │   └── error_scenarios_test.rs
│   ├── compatibility/
│   │   ├── cross_platform_test.rs
│   │   ├── llvm_version_test.rs
│   │   └── regression_test.rs
│   └── property_based/
│       ├── conversion_properties_test.rs
│       ├── round_trip_test.rs
│       └── invariant_validation_test.rs
```

### 2.2 Test Infrastructure

```rust
// tests/type_conversion/common/mod.rs
pub mod test_utils {
    use cursed::core::type_checker::Type;
    use cursed::codegen::llvm::LlvmTypeConversionBackend;
    use inkwell::context::Context;
    
    pub struct ConversionTestHarness<'ctx> {
        pub context: &'ctx Context,
        pub backend: LlvmTypeConversionBackend<'ctx>,
        pub test_values: HashMap<Type, Vec<TestValue>>,
    }
    
    #[derive(Debug, Clone)]
    pub struct TestValue {
        pub value: ConversionTestData,
        pub expected_results: HashMap<Type, ConversionExpectation>,
        pub description: String,
    }
    
    #[derive(Debug, Clone)]
    pub enum ConversionTestData {
        Integer(i64),
        Float(f64),
        Boolean(bool),
        String(String),
        Bytes(Vec<u8>),
        Complex(f64, f64),
    }
    
    #[derive(Debug, Clone)]
    pub enum ConversionExpectation {
        Success(ConversionTestData),
        Error(String),
        Warning(ConversionTestData, String),
    }
    
    impl<'ctx> ConversionTestHarness<'ctx> {
        pub fn new(context: &'ctx Context) -> Self {
            Self {
                context,
                backend: LlvmTypeConversionBackend::new(context),
                test_values: Self::generate_test_values(),
            }
        }
        
        pub fn test_conversion(&mut self, 
                              source_type: &Type, 
                              target_type: &Type, 
                              test_value: &TestValue) -> ConversionTestResult {
            // Implementation for testing individual conversions
        }
        
        pub fn test_all_conversions(&mut self) -> Vec<ConversionTestResult> {
            // Implementation for testing all possible conversions
        }
        
        fn generate_test_values() -> HashMap<Type, Vec<TestValue>> {
            // Generate comprehensive test values for each type
        }
    }
}
```

## 3. Unit Tests

### 3.1 Conversion Registry Tests

```rust
// tests/type_conversion/unit/conversion_registry_test.rs
use cursed::core::conversion_registry::ConversionRegistry;
use cursed::core::type_checker::Type;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_conversion_registration() {
        let registry = ConversionRegistry::new();
        
        // Test primitive type conversions are registered
        assert!(registry.has_conversion(&Type::Smol, &Type::Normie));
        assert!(registry.has_conversion(&Type::Normie, &Type::Thicc));
        assert!(registry.has_conversion(&Type::Snack, &Type::Meal));
        
        // Test invalid conversions are not registered
        assert!(!registry.has_conversion(&Type::Tea, &Type::Normie));
        assert!(!registry.has_conversion(&Type::Lit, &Type::Snack));
    }
    
    #[test]
    fn test_conversion_metadata() {
        let registry = ConversionRegistry::new();
        
        // Test safe conversions
        let metadata = registry.get_conversion_metadata(&Type::Smol, &Type::Normie).unwrap();
        assert_eq!(metadata.safety_level, SafetyLevel::Safe);
        assert_eq!(metadata.cost, ConversionCost::ZeroCost);
        
        // Test lossy conversions
        let metadata = registry.get_conversion_metadata(&Type::Thicc, &Type::Smol).unwrap();
        assert_eq!(metadata.safety_level, SafetyLevel::Lossy);
        assert_eq!(metadata.cost, ConversionCost::Low);
    }
    
    #[test]
    fn test_custom_conversion_registration() {
        let mut registry = ConversionRegistry::new();
        
        // Register custom conversion
        struct CustomIntToString;
        impl CustomConversion<i32, String> for CustomIntToString {
            fn convert(&self, value: i32) -> Result<String, ConversionError> {
                Ok(value.to_string())
            }
        }
        
        registry.register_custom_conversion(
            Type::Normie, 
            Type::Tea, 
            Box::new(CustomIntToString)
        );
        
        assert!(registry.has_conversion(&Type::Normie, &Type::Tea));
    }
    
    #[test]
    fn test_conversion_path_finding() {
        let registry = ConversionRegistry::new();
        
        // Test direct conversion path
        let path = registry.find_conversion_path(&Type::Smol, &Type::Thicc).unwrap();
        assert_eq!(path, vec![Type::Smol, Type::Thicc]);
        
        // Test multi-step conversion path
        let path = registry.find_conversion_path(&Type::Byte, &Type::Meal).unwrap();
        assert!(path.len() > 2); // Should go through intermediate types
    }
}
```

### 3.2 Primitive Conversion Tests

```rust
// tests/type_conversion/unit/primitive_conversions_test.rs
use cursed::codegen::llvm::type_conversion::LlvmTypeConversionBackend;
use inkwell::context::Context;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_integer_extension_conversions() {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        // Test smol to normie (i8 to i32)
        let smol_val = context.i8_type().const_int(42, false);
        let result = backend.convert(
            smol_val.into(), 
            &Type::Smol, 
            &Type::Normie
        ).unwrap();
        
        assert_eq!(
            result.into_int_value().get_sign_extended_constant().unwrap(), 
            42i64
        );
        
        // Test negative value extension
        let neg_val = context.i8_type().const_int((-42i8) as u64, true);
        let result = backend.convert(
            neg_val.into(), 
            &Type::Smol, 
            &Type::Normie
        ).unwrap();
        
        assert_eq!(
            result.into_int_value().get_sign_extended_constant().unwrap(), 
            -42i64
        );
    }
    
    #[test]
    fn test_integer_truncation_conversions() {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        // Test normie to smol (i32 to i8) - safe range
        let normie_val = context.i32_type().const_int(100, false);
        let result = backend.convert(
            normie_val.into(), 
            &Type::Normie, 
            &Type::Smol
        ).unwrap();
        
        assert_eq!(
            result.into_int_value().get_zero_extended_constant().unwrap(), 
            100u64
        );
        
        // Test truncation with overflow
        let large_val = context.i32_type().const_int(1000, false);
        let result = backend.convert(
            large_val.into(), 
            &Type::Normie, 
            &Type::Smol
        ).unwrap();
        
        // 1000 % 256 = 232
        assert_eq!(
            result.into_int_value().get_zero_extended_constant().unwrap(), 
            232u64
        );
    }
    
    #[test]
    fn test_float_conversions() {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        // Test snack to meal (f32 to f64)
        let snack_val = context.f32_type().const_float(3.14);
        let result = backend.convert(
            snack_val.into(), 
            &Type::Snack, 
            &Type::Meal
        ).unwrap();
        
        let result_float = result.into_float_value().get_constant().unwrap();
        assert!((result_float - 3.14).abs() < 0.001);
        
        // Test meal to snack (f64 to f32) - precision loss
        let meal_val = context.f64_type().const_float(3.14159265359);
        let result = backend.convert(
            meal_val.into(), 
            &Type::Meal, 
            &Type::Snack
        ).unwrap();
        
        let result_float = result.into_float_value().get_constant().unwrap() as f32;
        assert!((result_float - 3.1415927).abs() < 0.0001);
    }
    
    #[test]
    fn test_int_to_float_conversions() {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        // Test normie to snack (i32 to f32)
        let int_val = context.i32_type().const_int(42, false);
        let result = backend.convert(
            int_val.into(), 
            &Type::Normie, 
            &Type::Snack
        ).unwrap();
        
        let result_float = result.into_float_value().get_constant().unwrap();
        assert_eq!(result_float, 42.0);
        
        // Test large integer to float (potential precision loss)
        let large_int = context.i64_type().const_int(9007199254740993, false); // 2^53 + 1
        let result = backend.convert(
            large_int.into(), 
            &Type::Thicc, 
            &Type::Meal
        ).unwrap();
        
        // Should be approximately correct but may lose precision
        let result_float = result.into_float_value().get_constant().unwrap();
        assert!((result_float - 9007199254740992.0).abs() <= 1.0);
    }
    
    #[test]
    fn test_float_to_int_conversions() {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        // Test snack to normie (f32 to i32)
        let float_val = context.f32_type().const_float(42.7);
        let result = backend.convert(
            float_val.into(), 
            &Type::Snack, 
            &Type::Normie
        ).unwrap();
        
        assert_eq!(
            result.into_int_value().get_sign_extended_constant().unwrap(), 
            42i64
        );
        
        // Test negative float to int
        let neg_float = context.f32_type().const_float(-42.7);
        let result = backend.convert(
            neg_float.into(), 
            &Type::Snack, 
            &Type::Normie
        ).unwrap();
        
        assert_eq!(
            result.into_int_value().get_sign_extended_constant().unwrap(), 
            -42i64
        );
    }
    
    #[test]
    fn test_boolean_conversions() {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        // Test lit to normie (bool to i32)
        let true_val = context.bool_type().const_int(1, false);
        let result = backend.convert(
            true_val.into(), 
            &Type::Lit, 
            &Type::Normie
        ).unwrap();
        
        assert_eq!(
            result.into_int_value().get_zero_extended_constant().unwrap(), 
            1u64
        );
        
        let false_val = context.bool_type().const_int(0, false);
        let result = backend.convert(
            false_val.into(), 
            &Type::Lit, 
            &Type::Normie
        ).unwrap();
        
        assert_eq!(
            result.into_int_value().get_zero_extended_constant().unwrap(), 
            0u64
        );
    }
}
```

### 3.3 Error Handling Tests

```rust
// tests/type_conversion/unit/error_handling_test.rs
use cursed::error::type_conversion_error::{TypeConversionError, ConversionFailureReason};
use cursed::core::conversion_validator::ConversionValidator;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_invalid_conversion_detection() {
        let validator = ConversionValidator::new();
        
        // Test invalid string to number conversion
        let result = validator.validate_conversion(&Type::Tea, &Type::Normie);
        assert!(result.is_err());
        
        if let Err(error) = result {
            assert_eq!(error.reason, ConversionFailureReason::IncompatibleTypes);
        }
    }
    
    #[test]
    fn test_lossy_conversion_warnings() {
        let validator = ConversionValidator::new();
        
        // Test lossy conversion from thicc to smol
        let result = validator.validate_conversion(&Type::Thicc, &Type::Smol);
        assert!(result.is_ok());
        
        if let Ok(metadata) = result {
            assert_eq!(metadata.safety_level, SafetyLevel::Lossy);
            assert!(metadata.warnings.contains(&"Potential data loss".to_string()));
        }
    }
    
    #[test]
    fn test_error_message_quality() {
        let error = TypeConversionError::new(
            Type::Tea,
            Type::Normie,
            ConversionFailureReason::IncompatibleTypes,
            SourceLocation::new(10, 5),
        );
        
        let message = error.to_string();
        assert!(message.contains("Cannot convert"));
        assert!(message.contains("tea"));
        assert!(message.contains("normie"));
        assert!(message.contains("line 10"));
    }
    
    #[test]
    fn test_error_suggestions() {
        let error = TypeConversionError::incompatible_types(&Type::Tea, &Type::Normie)
            .with_suggestion("Use parsing functions like str.parse() for string to number conversion");
        
        assert!(error.suggestion.is_some());
        assert!(error.suggestion.unwrap().contains("str.parse()"));
    }
}
```

## 4. Integration Tests

### 4.1 End-to-End Conversion Tests

```rust
// tests/type_conversion/integration/end_to_end_conversion_test.rs
use cursed::compile_and_execute;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_integer_conversion_syntax() {
        let source = r#"
            sus x = 42
            sus y = x as thicc
            facts result = y == 42
        "#;
        
        let result = compile_and_execute(source).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }
    
    #[test]
    fn test_float_conversion_syntax() {
        let source = r#"
            sus x = 3.14 as snack
            sus y = x as meal
            facts result = y > 3.0
        "#;
        
        let result = compile_and_execute(source).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }
    
    #[test]
    fn test_chained_conversions() {
        let source = r#"
            sus x = 1000 as thicc
            sus y = x as normie
            sus z = y as smol
            facts result = z == -24  // 1000 % 256 = 232, but signed byte = -24
        "#;
        
        let result = compile_and_execute(source).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }
    
    #[test]
    fn test_invalid_conversion_compile_error() {
        let source = r#"
            sus x = "hello"
            sus y = x as normie  // Should fail at compile time
        "#;
        
        let result = compile_and_execute(source);
        assert!(result.is_err());
        
        if let Err(error) = result {
            assert!(error.to_string().contains("Cannot convert"));
            assert!(error.to_string().contains("tea"));
            assert!(error.to_string().contains("normie"));
        }
    }
    
    #[test]
    fn test_conversion_in_expressions() {
        let source = r#"
            sus x = 42 as thicc
            sus y = 3.14 as meal
            sus result = (x as meal) + y
        "#;
        
        let result = compile_and_execute(source).unwrap();
        if let Value::Float(f) = result {
            assert!((f - 45.14).abs() < 0.001);
        } else {
            panic!("Expected float result");
        }
    }
    
    #[test]
    fn test_conversion_in_function_calls() {
        let source = r#"
            yolo process_number(x: meal) -> normie {
                yolo x as normie
            }
            
            sus input = 42 as snack
            sus result = process_number(input as meal)
        "#;
        
        let result = compile_and_execute(source).unwrap();
        assert_eq!(result, Value::Integer(42));
    }
}
```

### 4.2 Type Checker Integration Tests

```rust
// tests/type_conversion/integration/type_checker_integration_test.rs
use cursed::core::type_checker::TypeChecker;
use cursed::parser::Parser;
use cursed::lexer::Lexer;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_type_checker_validates_conversions() {
        let source = "42 as normie";
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check_program(&program);
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_type_checker_rejects_invalid_conversions() {
        let source = r#""hello" as normie"#;
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check_program(&program);
        
        assert!(result.is_err());
        
        if let Err(errors) = result {
            assert!(!errors.is_empty());
            assert!(errors[0].to_string().contains("type conversion"));
        }
    }
    
    #[test]
    fn test_type_inference_with_conversions() {
        let source = r#"
            sus x = 42
            sus y = x as meal
            sus z = y + 3.14
        "#;
        
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check_program(&program);
        
        assert!(result.is_ok());
        
        // Verify inferred types
        let var_z_type = type_checker.get_variable_type("z").unwrap();
        assert_eq!(var_z_type, Type::Meal);
    }
}
```

## 5. Performance Tests

### 5.1 Conversion Benchmarks

```rust
// tests/type_conversion/performance/conversion_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use cursed::codegen::llvm::type_conversion::LlvmTypeConversionBackend;

fn bench_integer_conversions(c: &mut Criterion) {
    let context = Context::create();
    let mut backend = LlvmTypeConversionBackend::new(&context);
    
    let mut group = c.benchmark_group("integer_conversions");
    
    // Benchmark different conversion types
    let conversions = vec![
        ("smol_to_normie", Type::Smol, Type::Normie),
        ("normie_to_thicc", Type::Normie, Type::Thicc),
        ("thicc_to_smol", Type::Thicc, Type::Smol),
        ("normie_to_mid", Type::Normie, Type::Mid),
    ];
    
    for (name, source_type, target_type) in conversions {
        group.bench_with_input(
            BenchmarkId::new("conversion", name),
            &(source_type, target_type),
            |b, (src, tgt)| {
                let test_value = generate_test_value(src);
                b.iter(|| {
                    backend.convert(
                        black_box(test_value), 
                        black_box(src), 
                        black_box(tgt)
                    ).unwrap()
                });
            },
        );
    }
    
    group.finish();
}

fn bench_float_conversions(c: &mut Criterion) {
    let context = Context::create();
    let mut backend = LlvmTypeConversionBackend::new(&context);
    
    let mut group = c.benchmark_group("float_conversions");
    
    let conversions = vec![
        ("snack_to_meal", Type::Snack, Type::Meal),
        ("meal_to_snack", Type::Meal, Type::Snack),
        ("normie_to_snack", Type::Normie, Type::Snack),
        ("snack_to_normie", Type::Snack, Type::Normie),
    ];
    
    for (name, source_type, target_type) in conversions {
        group.bench_with_input(
            BenchmarkId::new("conversion", name),
            &(source_type, target_type),
            |b, (src, tgt)| {
                let test_value = generate_test_value(src);
                b.iter(|| {
                    backend.convert(
                        black_box(test_value), 
                        black_box(src), 
                        black_box(tgt)
                    ).unwrap()
                });
            },
        );
    }
    
    group.finish();
}

fn bench_batch_conversions(c: &mut Criterion) {
    let context = Context::create();
    let mut backend = LlvmTypeConversionBackend::new(&context);
    
    let mut group = c.benchmark_group("batch_conversions");
    
    let sizes = vec![10, 100, 1000, 10000];
    
    for size in sizes {
        group.bench_with_input(
            BenchmarkId::new("batch_int_conversion", size),
            &size,
            |b, &size| {
                let values: Vec<_> = (0..size)
                    .map(|i| context.i32_type().const_int(i as u64, false).into())
                    .collect();
                
                b.iter(|| {
                    backend.vectorized_conversion(
                        black_box(&values),
                        black_box(&Type::Normie),
                        black_box(&Type::Thicc),
                    ).unwrap()
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_integer_conversions, bench_float_conversions, bench_batch_conversions);
criterion_main!(benches);
```

### 5.2 Memory Usage Tests

```rust
// tests/type_conversion/performance/memory_usage_test.rs
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct TrackingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_conversion_memory_usage() {
        let initial_memory = ALLOCATED.load(Ordering::SeqCst);
        
        // Perform many conversions
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        for i in 0..1000 {
            let value = context.i32_type().const_int(i, false);
            let _result = backend.convert(
                value.into(),
                &Type::Normie,
                &Type::Thicc,
            ).unwrap();
        }
        
        let final_memory = ALLOCATED.load(Ordering::SeqCst);
        let memory_used = final_memory - initial_memory;
        
        // Ensure memory usage is reasonable (less than 1MB for 1000 conversions)
        assert!(memory_used < 1024 * 1024, "Memory usage too high: {} bytes", memory_used);
    }
    
    #[test]
    fn test_no_memory_leaks_in_conversions() {
        let initial_memory = ALLOCATED.load(Ordering::SeqCst);
        
        {
            let context = Context::create();
            let mut backend = LlvmTypeConversionBackend::new(&context);
            
            // Perform conversions and let everything go out of scope
            for i in 0..100 {
                let value = context.f32_type().const_float(i as f64);
                let _result = backend.convert(
                    value.into(),
                    &Type::Snack,
                    &Type::Meal,
                ).unwrap();
            }
        } // Context and backend destroyed here
        
        // Allow some time for cleanup
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        let final_memory = ALLOCATED.load(Ordering::SeqCst);
        
        // Memory should be back to initial level (with some tolerance)
        assert!(
            final_memory <= initial_memory + 1024, 
            "Memory leak detected: {} bytes not freed", 
            final_memory - initial_memory
        );
    }
}
```

## 6. Edge Case Tests

### 6.1 Boundary Value Tests

```rust
// tests/type_conversion/edge_cases/boundary_values_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_integer_boundary_conversions() {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        // Test maximum values
        let max_smol = context.i8_type().const_int(i8::MAX as u64, false);
        let result = backend.convert(
            max_smol.into(),
            &Type::Smol,
            &Type::Normie,
        ).unwrap();
        assert_eq!(
            result.into_int_value().get_sign_extended_constant().unwrap(),
            i8::MAX as i64
        );
        
        // Test minimum values
        let min_smol = context.i8_type().const_int(i8::MIN as u64, true);
        let result = backend.convert(
            min_smol.into(),
            &Type::Smol,
            &Type::Normie,
        ).unwrap();
        assert_eq!(
            result.into_int_value().get_sign_extended_constant().unwrap(),
            i8::MIN as i64
        );
        
        // Test overflow scenarios
        let overflow_val = context.i32_type().const_int(300, false); // > i8::MAX
        let result = backend.convert(
            overflow_val.into(),
            &Type::Normie,
            &Type::Smol,
        ).unwrap();
        assert_eq!(
            result.into_int_value().get_zero_extended_constant().unwrap(),
            (300u32 % 256) as u64
        );
    }
    
    #[test]
    fn test_float_special_values() {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        // Test infinity
        let infinity = context.f32_type().const_float(f32::INFINITY as f64);
        let result = backend.convert(
            infinity.into(),
            &Type::Snack,
            &Type::Meal,
        ).unwrap();
        assert!(result.into_float_value().get_constant().unwrap().is_infinite());
        
        // Test negative infinity
        let neg_infinity = context.f32_type().const_float(f32::NEG_INFINITY as f64);
        let result = backend.convert(
            neg_infinity.into(),
            &Type::Snack,
            &Type::Meal,
        ).unwrap();
        assert!(result.into_float_value().get_constant().unwrap().is_infinite());
        assert!(result.into_float_value().get_constant().unwrap().is_sign_negative());
        
        // Test NaN
        let nan = context.f32_type().const_float(f32::NAN as f64);
        let result = backend.convert(
            nan.into(),
            &Type::Snack,
            &Type::Meal,
        ).unwrap();
        assert!(result.into_float_value().get_constant().unwrap().is_nan());
    }
    
    #[test]
    fn test_zero_values() {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        // Test positive zero
        let pos_zero = context.f32_type().const_float(0.0);
        let result = backend.convert(
            pos_zero.into(),
            &Type::Snack,
            &Type::Normie,
        ).unwrap();
        assert_eq!(
            result.into_int_value().get_zero_extended_constant().unwrap(),
            0u64
        );
        
        // Test negative zero
        let neg_zero = context.f32_type().const_float(-0.0);
        let result = backend.convert(
            neg_zero.into(),
            &Type::Snack,
            &Type::Normie,
        ).unwrap();
        assert_eq!(
            result.into_int_value().get_zero_extended_constant().unwrap(),
            0u64
        );
    }
}
```

### 6.2 Precision Loss Tests

```rust
// tests/type_conversion/edge_cases/precision_loss_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_float_precision_loss() {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        // Test meal to snack precision loss
        let precise_val = context.f64_type().const_float(1.23456789012345);
        let result = backend.convert(
            precise_val.into(),
            &Type::Meal,
            &Type::Snack,
        ).unwrap();
        
        let result_f32 = result.into_float_value().get_constant().unwrap() as f32;
        let expected_f32 = 1.23456789012345f64 as f32;
        
        assert!((result_f32 - expected_f32).abs() < 1e-6);
        assert_ne!(result_f32 as f64, 1.23456789012345); // Precision should be lost
    }
    
    #[test]
    fn test_large_int_to_float_precision() {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        // Test large integer that cannot be exactly represented in f32
        let large_int = context.i64_type().const_int(16777217, false); // 2^24 + 1
        let result = backend.convert(
            large_int.into(),
            &Type::Thicc,
            &Type::Snack,
        ).unwrap();
        
        let result_f32 = result.into_float_value().get_constant().unwrap() as f32;
        
        // f32 should round to nearest representable value
        assert_eq!(result_f32, 16777216.0f32); // Rounded down to 2^24
    }
    
    #[test]
    fn test_round_trip_precision_loss() {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        // Test round-trip conversion that should lose precision
        let original = context.i64_type().const_int(9007199254740993, false); // 2^53 + 1
        
        // Convert to f64 and back
        let to_float = backend.convert(
            original.into(),
            &Type::Thicc,
            &Type::Meal,
        ).unwrap();
        
        let back_to_int = backend.convert(
            to_float,
            &Type::Meal,
            &Type::Thicc,
        ).unwrap();
        
        let final_value = back_to_int.into_int_value().get_zero_extended_constant().unwrap();
        
        // Should be rounded to 2^53 (nearest representable in f64)
        assert_eq!(final_value, 9007199254740992);
        assert_ne!(final_value, 9007199254740993); // Original value lost
    }
}
```

## 7. Property-Based Tests

### 7.1 Conversion Properties

```rust
// tests/type_conversion/property_based/conversion_properties_test.rs
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_extension_never_changes_value(
        value in any::<i8>()
    ) {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        let smol_val = context.i8_type().const_int(value as u64, true);
        let result = backend.convert(
            smol_val.into(),
            &Type::Smol,
            &Type::Normie,
        ).unwrap();
        
        let result_value = result.into_int_value().get_sign_extended_constant().unwrap();
        prop_assert_eq!(result_value, value as i64);
    }
    
    #[test]
    fn prop_truncation_preserves_lower_bits(
        value in any::<i32>()
    ) {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        let normie_val = context.i32_type().const_int(value as u64, true);
        let result = backend.convert(
            normie_val.into(),
            &Type::Normie,
            &Type::Smol,
        ).unwrap();
        
        let result_value = result.into_int_value().get_zero_extended_constant().unwrap() as i8;
        let expected = value as i8; // Truncation to i8
        
        prop_assert_eq!(result_value, expected);
    }
    
    #[test]
    fn prop_float_extension_preserves_value(
        value in any::<f32>().prop_filter("finite", |f| f.is_finite())
    ) {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        let snack_val = context.f32_type().const_float(value as f64);
        let result = backend.convert(
            snack_val.into(),
            &Type::Snack,
            &Type::Meal,
        ).unwrap();
        
        let result_value = result.into_float_value().get_constant().unwrap();
        prop_assert!((result_value - value as f64).abs() < 1e-6);
    }
    
    #[test]
    fn prop_identity_conversions(
        value in any::<i32>()
    ) {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        let normie_val = context.i32_type().const_int(value as u64, true);
        let result = backend.convert(
            normie_val.into(),
            &Type::Normie,
            &Type::Normie,
        ).unwrap();
        
        let result_value = result.into_int_value().get_sign_extended_constant().unwrap();
        prop_assert_eq!(result_value, value as i64);
    }
}
```

### 7.2 Round-Trip Tests

```rust
// tests/type_conversion/property_based/round_trip_test.rs
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_safe_round_trip_conversions(
        value in any::<i8>()
    ) {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        // smol -> normie -> smol should preserve value
        let original = context.i8_type().const_int(value as u64, true);
        
        let to_normie = backend.convert(
            original.into(),
            &Type::Smol,
            &Type::Normie,
        ).unwrap();
        
        let back_to_smol = backend.convert(
            to_normie,
            &Type::Normie,
            &Type::Smol,
        ).unwrap();
        
        let final_value = back_to_smol.into_int_value().get_sign_extended_constant().unwrap() as i8;
        prop_assert_eq!(final_value, value);
    }
    
    #[test]
    fn prop_lossy_round_trip_bounds(
        value in any::<i32>()
    ) {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        // normie -> smol -> normie may lose information
        let original = context.i32_type().const_int(value as u64, true);
        
        let to_smol = backend.convert(
            original.into(),
            &Type::Normie,
            &Type::Smol,
        ).unwrap();
        
        let back_to_normie = backend.convert(
            to_smol,
            &Type::Smol,
            &Type::Normie,
        ).unwrap();
        
        let final_value = back_to_normie.into_int_value().get_sign_extended_constant().unwrap() as i32;
        
        // The final value should be within the range of i8 when converted
        prop_assert!(final_value >= i8::MIN as i32);
        prop_assert!(final_value <= i8::MAX as i32);
        
        // If original was in i8 range, it should be preserved
        if value >= i8::MIN as i32 && value <= i8::MAX as i32 {
            prop_assert_eq!(final_value, value);
        }
    }
}
```

## 8. Cross-Platform Tests

### 8.1 Platform Compatibility Tests

```rust
// tests/type_conversion/compatibility/cross_platform_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pointer_size_conversions() {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        // Test pointer to uintptr conversion
        let null_ptr = context.i8_type().ptr_type(inkwell::AddressSpace::default()).const_null();
        
        // This should work regardless of platform pointer size
        let result = backend.convert_pointer_to_int(null_ptr.into());
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_endianness_independence() {
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        // Test byte order doesn't affect basic conversions
        let test_val = context.i32_type().const_int(0x12345678, false);
        let result = backend.convert(
            test_val.into(),
            &Type::Normie,
            &Type::Thicc,
        ).unwrap();
        
        assert_eq!(
            result.into_int_value().get_zero_extended_constant().unwrap(),
            0x12345678u64
        );
    }
    
    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_64bit_specific_conversions() {
        // Tests specific to 64-bit platforms
        let context = Context::create();
        let mut backend = LlvmTypeConversionBackend::new(&context);
        
        let large_val = context.i64_type().const_int(u64::MAX, false);
        let result = backend.convert(
            large_val.into(),
            &Type::Thicc,
            &Type::Thicc,
        ).unwrap();
        
        assert_eq!(
            result.into_int_value().get_zero_extended_constant().unwrap(),
            u64::MAX
        );
    }
    
    #[cfg(target_pointer_width = "32")]
    #[test]
    fn test_32bit_specific_conversions() {
        // Tests specific to 32-bit platforms
        // (Similar tests adapted for 32-bit constraints)
    }
}
```

## 9. Test Automation and CI Integration

### 9.1 Test Runner Script

```bash
#!/bin/bash
# tests/run_conversion_tests.sh

set -e

echo "Running CURSED Type Conversion Test Suite"
echo "========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test categories
UNIT_TESTS="conversion_registry primitive_conversions error_handling"
INTEGRATION_TESTS="end_to_end type_checker_integration llvm_codegen_integration"
PERFORMANCE_TESTS="conversion_benchmarks memory_usage"
EDGE_CASE_TESTS="boundary_values precision_loss overflow_underflow"
PROPERTY_TESTS="conversion_properties round_trip_test"

run_test_category() {
    local category=$1
    local tests=$2
    
    echo -e "${YELLOW}Running $category tests...${NC}"
    
    for test in $tests; do
        echo "  - $test"
        if cargo test --test "${test}" --release; then
            echo -e "    ${GREEN}✓ PASSED${NC}"
        else
            echo -e "    ${RED}✗ FAILED${NC}"
            exit 1
        fi
    done
    
    echo -e "${GREEN}$category tests completed successfully${NC}"
    echo
}

# Run all test categories
run_test_category "Unit" "$UNIT_TESTS"
run_test_category "Integration" "$INTEGRATION_TESTS" 
run_test_category "Performance" "$PERFORMANCE_TESTS"
run_test_category "Edge Case" "$EDGE_CASE_TESTS"
run_test_category "Property-based" "$PROPERTY_TESTS"

# Run cross-platform tests if specified
if [[ "$1" == "--cross-platform" ]]; then
    echo -e "${YELLOW}Running cross-platform tests...${NC}"
    cargo test --test cross_platform_test --release
    echo -e "${GREEN}Cross-platform tests completed${NC}"
fi

# Generate coverage report if requested
if [[ "$1" == "--coverage" ]]; then
    echo -e "${YELLOW}Generating coverage report...${NC}"
    cargo tarpaulin --out Html --output-dir coverage/
    echo -e "${GREEN}Coverage report generated in coverage/tarpaulin-report.html${NC}"
fi

echo -e "${GREEN}All type conversion tests passed!${NC}"
```

### 9.2 CI Configuration

```yaml
# .github/workflows/type_conversion_tests.yml
name: Type Conversion Tests

on:
  push:
    paths:
      - 'src/core/type_converter.rs'
      - 'src/core/conversion_registry.rs'
      - 'src/codegen/llvm/type_conversion.rs'
      - 'tests/type_conversion/**'
  pull_request:
    paths:
      - 'src/core/type_converter.rs'
      - 'src/core/conversion_registry.rs'
      - 'src/codegen/llvm/type_conversion.rs'
      - 'tests/type_conversion/**'

jobs:
  unit-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run unit tests
        run: |
          cd tests/type_conversion
          ./run_conversion_tests.sh unit

  integration-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run integration tests
        run: |
          cd tests/type_conversion
          ./run_conversion_tests.sh integration

  cross-platform-tests:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run cross-platform tests
        run: |
          cd tests/type_conversion
          ./run_conversion_tests.sh --cross-platform

  performance-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Install criterion
        run: cargo install cargo-criterion
      - name: Run performance benchmarks
        run: |
          cd tests/type_conversion/performance
          cargo criterion --bench conversion_benchmarks
      - name: Upload performance results
        uses: actions/upload-artifact@v3
        with:
          name: performance-results
          path: target/criterion/

  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin
      - name: Generate coverage
        run: |
          cd tests/type_conversion
          ./run_conversion_tests.sh --coverage
      - name: Upload coverage to Codecov
        uses: codecov/codecov-action@v3
        with:
          file: ./coverage/cobertura.xml
```

This comprehensive testing strategy ensures the CURSED type conversion system is robust, performant, and reliable across all supported platforms and use cases.
