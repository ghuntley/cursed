# CURSED Type Conversion System Architecture

## 1. Executive Summary

This document defines the comprehensive type conversion system architecture for the CURSED programming language. The system enables safe and explicit type conversions between compatible types while providing clear error handling for invalid conversions.

### 1.1 Key Features
- **Explicit Type Conversions**: Using `as` syntax (e.g., `value as normie`)
- **Type Safety**: Compile-time validation of conversion validity
- **Performance**: Optimized LLVM code generation for each conversion type
- **Error Handling**: Clear error messages for invalid conversions
- **Extensibility**: Support for user-defined type conversions

### 1.2 Integration Points
- **AST Integration**: Extends existing `TypeConversionExpression`
- **Type Checker**: Validates conversion legality at compile-time
- **LLVM Codegen**: Efficient runtime conversion implementations
- **Error System**: Rich error context and reporting

## 2. System Architecture

### 2.1 Module Structure

```
src/
├── core/
│   ├── type_converter.rs           # Main conversion orchestrator
│   ├── conversion_registry.rs      # Conversion rule registry
│   └── conversion_validator.rs     # Compile-time validation
├── codegen/llvm/
│   ├── type_conversion.rs          # LLVM conversion implementations
│   ├── conversion_primitives.rs    # Basic type conversions
│   ├── conversion_composites.rs    # Complex type conversions
│   └── conversion_optimizations.rs # Performance optimizations
├── error/
│   └── type_conversion_error.rs    # Specialized error types
└── tests/
    ├── type_conversion_unit_test.rs
    ├── type_conversion_integration_test.rs
    └── type_conversion_performance_test.rs
```

### 2.2 Core Components

#### 2.2.1 TypeConverter
Main orchestrator that coordinates type conversion operations.

```rust
pub struct TypeConverter {
    registry: ConversionRegistry,
    validator: ConversionValidator,
    llvm_backend: LlvmConversionBackend,
}
```

#### 2.2.2 ConversionRegistry
Registry of all valid type conversions with metadata.

```rust
pub struct ConversionRegistry {
    conversions: HashMap<(Type, Type), ConversionRule>,
    custom_conversions: HashMap<String, Box<dyn CustomConversion>>,
}
```

#### 2.2.3 ConversionValidator
Compile-time validation of conversion operations.

```rust
pub struct ConversionValidator {
    type_checker: Arc<TypeChecker>,
    registry: Arc<ConversionRegistry>,
}
```

## 3. Type Conversion Matrix

### 3.1 Primitive Type Conversions

| From/To | lit | smol | mid | normie | thicc | snack | meal | tea | sip | byte | rune | extra |
|---------|-----|------|-----|--------|-------|-------|------|-----|-----|------|------|-------|
| **lit** | ✓   | ✓    | ✓   | ✓      | ✓     | ✗     | ✗    | ✓   | ✗   | ✗    | ✗    | ✗     |
| **smol**| ✓   | ✓    | ✓   | ✓      | ✓     | ✓     | ✓    | ✗   | ✓   | ✓    | ✓    | ✗     |
| **mid** | ✓   | ⚠    | ✓   | ✓      | ✓     | ✓     | ✓    | ✗   | ✓   | ⚠    | ✓    | ✗     |
| **normie**|✓  | ⚠    | ⚠   | ✓      | ✓     | ✓     | ✓    | ✗   | ✓   | ⚠    | ✓    | ✗     |
| **thicc**|✓   | ⚠    | ⚠   | ⚠      | ✓     | ✓     | ✓    | ✗   | ⚠   | ⚠    | ⚠    | ✗     |
| **snack**|✗   | ⚠    | ⚠   | ⚠      | ⚠     | ✓     | ✓    | ✗   | ✗   | ✗    | ✗    | ✓     |
| **meal**|✗    | ⚠    | ⚠   | ⚠      | ⚠     | ⚠     | ✓    | ✗   | ✗   | ✗    | ✗    | ✓     |
| **tea** | ✗   | ✗    | ✗   | ✗      | ✗     | ✗     | ✗    | ✓   | ✗   | ✗    | ✗    | ✗     |
| **sip** | ✗   | ⚠    | ⚠   | ✓      | ✓     | ✗     | ✗    | ✗   | ✓   | ⚠    | ✓    | ✗     |
| **byte**| ✗   | ✓    | ✓   | ✓      | ✓     | ✓     | ✓    | ✗   | ✓   | ✓    | ✓    | ✗     |
| **rune**| ✗   | ⚠    | ⚠   | ✓      | ✓     | ✗     | ✗    | ✗   | ✓   | ⚠    | ✓    | ✗     |
| **extra**|✗   | ✗    | ✗   | ✗      | ✗     | ⚠     | ⚠    | ✗   | ✗   | ✗    | ✗    | ✓     |

**Legend:**
- ✓ = Always safe conversion
- ⚠ = Potentially lossy conversion (truncation/overflow warnings)
- ✗ = Invalid conversion (compile error)

### 3.2 Composite Type Conversions

#### 3.2.1 Pointer Conversions
- `*T` ↔ `*U` where `T` and `U` are compatible
- `*T` ↔ `uintptr` (platform-specific integer type)
- `*void` ↔ `*T` (unsafe pointer casting)

#### 3.2.2 Array/Slice Conversions
- `[n]T` → `[]T` (array to slice)
- `[]T` ↔ `*T` (slice to pointer, first element)
- `[n]byte` ↔ `tea` (byte array to string)

#### 3.2.3 Interface Conversions
- `T` → `Interface` where `T` implements `Interface`
- `Interface` → `T` via type assertion (runtime check)

## 4. LLVM Implementation Strategy

### 4.1 Conversion Categories

#### 4.1.1 Zero-Cost Conversions
Conversions that require no runtime operations:
- `byte` → `smol` (same bit representation)
- `sip` → `rune` (aliases)
- Array to slice conversions

```rust
impl ZeroCostConversion for LlvmConversionBackend {
    fn convert_zero_cost(&self, value: BasicValueEnum, target_type: BasicTypeEnum) -> BasicValueEnum {
        // Bitcast or direct return
        if value.get_type() == target_type {
            value
        } else {
            self.builder.build_bitcast(value, target_type, "zero_cost_cast")
        }
    }
}
```

#### 4.1.2 Truncation Conversions
Conversions that may lose precision:
- Larger integers → smaller integers
- Float → integer conversions

```rust
impl TruncationConversion for LlvmConversionBackend {
    fn convert_with_truncation(&self, value: IntValue, target_type: IntType) -> IntValue {
        if value.get_type().get_bit_width() > target_type.get_bit_width() {
            self.builder.build_int_truncate(value, target_type, "truncate")
        } else {
            self.builder.build_int_s_extend(value, target_type, "extend")
        }
    }
}
```

#### 4.1.3 Extension Conversions
Conversions that preserve all information:
- Smaller integers → larger integers
- Integer → float conversions

```rust
impl ExtensionConversion for LlvmConversionBackend {
    fn convert_with_extension(&self, value: IntValue, target_type: IntType, signed: bool) -> IntValue {
        if signed {
            self.builder.build_int_s_extend(value, target_type, "sext")
        } else {
            self.builder.build_int_z_extend(value, target_type, "zext")
        }
    }
}
```

#### 4.1.4 Complex Conversions
Conversions requiring runtime support:
- String conversions
- Interface conversions
- Custom type conversions

```rust
impl ComplexConversion for LlvmConversionBackend {
    fn convert_complex(&self, value: BasicValueEnum, source_type: &Type, target_type: &Type) -> Result<BasicValueEnum, String> {
        match (source_type, target_type) {
            (Type::Tea, Type::Array(box Type::Byte, _)) => self.string_to_byte_array(value),
            (Type::Array(box Type::Byte, _), Type::Tea) => self.byte_array_to_string(value),
            _ => self.call_runtime_conversion(value, source_type, target_type),
        }
    }
}
```

### 4.2 Performance Optimizations

#### 4.2.1 Conversion Inlining
- Inline simple conversions to avoid function call overhead
- Use LLVM intrinsics for common conversions

#### 4.2.2 Conversion Caching
- Cache conversion function addresses for repeated conversions
- Pre-generate conversion functions during module initialization

#### 4.2.3 Dead Code Elimination
- Only generate conversion functions for actually used conversions
- Remove unused conversion paths during optimization

## 5. Integration with Existing Components

### 5.1 Type Checker Integration

```rust
impl TypeChecker {
    pub fn validate_type_conversion(&mut self, expr: &TypeConversionExpression) -> Result<Type, Error> {
        let source_type = self.check_expression(&expr.expression)?;
        let target_type = Type::new_basic(&expr.type_name);
        
        self.type_converter.validate_conversion(&source_type, &target_type)
            .map_err(|e| Error::type_conversion_error(e, expr.token.location()))?;
            
        Ok(target_type)
    }
}
```

### 5.2 LLVM Codegen Integration

```rust
impl LlvmCodeGenerator {
    pub fn compile_type_conversion(&mut self, type_conv: &TypeConversionExpression) -> Result<BasicValueEnum<'ctx>, String> {
        let source_value = self.compile_expression(type_conv.expression.as_ref())?;
        let source_type = self.get_expression_type(type_conv.expression.as_ref())?;
        let target_type = Type::new_basic(&type_conv.type_name);
        
        self.type_conversion_backend.convert(source_value, &source_type, &target_type)
    }
}
```

### 5.3 Error System Integration

```rust
#[derive(Debug, Clone)]
pub struct TypeConversionError {
    pub source_type: Type,
    pub target_type: Type,
    pub reason: ConversionFailureReason,
    pub location: SourceLocation,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ConversionFailureReason {
    IncompatibleTypes,
    LossyConversion,
    UnsafeConversion,
    RuntimeConversionRequired,
    CustomConversionNotFound,
}
```

## 6. Error Handling Strategy

### 6.1 Compile-Time Validation

```rust
impl ConversionValidator {
    pub fn validate_conversion(&self, source: &Type, target: &Type) -> Result<ConversionMetadata, TypeConversionError> {
        // Check if conversion is registered
        if let Some(rule) = self.registry.get_conversion(source, target) {
            match rule.safety_level {
                SafetyLevel::Safe => Ok(ConversionMetadata::safe(rule)),
                SafetyLevel::Lossy => self.validate_lossy_conversion(source, target, rule),
                SafetyLevel::Unsafe => self.validate_unsafe_conversion(source, target, rule),
            }
        } else {
            Err(TypeConversionError::incompatible_types(source, target))
        }
    }
}
```

### 6.2 Runtime Error Handling

```rust
impl RuntimeConversionHandler {
    pub fn handle_conversion_failure(&self, error: ConversionRuntimeError) -> ! {
        match error.error_type {
            RuntimeErrorType::Overflow => panic!("Integer overflow during conversion"),
            RuntimeErrorType::InvalidValue => panic!("Invalid value for target type"),
            RuntimeErrorType::NullPointer => panic!("Null pointer in conversion"),
        }
    }
}
```

### 6.3 Error Recovery Strategies

1. **Fallback Conversions**: Automatic fallback to safe alternatives
2. **Default Values**: Use type-appropriate default values for failed conversions
3. **Optional Returns**: Return `Option<T>` for potentially failing conversions

## 7. Testing Strategy

### 7.1 Unit Tests

#### 7.1.1 Conversion Registry Tests
```rust
#[test]
fn test_basic_type_conversion_registration() {
    let registry = ConversionRegistry::new();
    assert!(registry.has_conversion(&Type::Smol, &Type::Normie));
    assert!(!registry.has_conversion(&Type::Tea, &Type::Normie));
}
```

#### 7.1.2 Validation Tests
```rust
#[test]
fn test_lossy_conversion_validation() {
    let validator = ConversionValidator::new();
    let result = validator.validate_conversion(&Type::Thicc, &Type::Smol);
    assert!(matches!(result, Ok(ConversionMetadata { safety_level: SafetyLevel::Lossy, .. })));
}
```

#### 7.1.3 LLVM Generation Tests
```rust
#[test]
fn test_integer_truncation_llvm() {
    let backend = LlvmConversionBackend::new();
    let i64_val = backend.context.i64_type().const_int(1000, false);
    let result = backend.convert_int_truncate(i64_val.into(), &Type::Smol);
    assert_eq!(result.into_int_value().get_zero_extended_constant(), Some(232)); // 1000 % 256
}
```

### 7.2 Integration Tests

#### 7.2.1 End-to-End Conversion Tests
```rust
#[test]
fn test_complete_conversion_pipeline() {
    let source = "42 as smol";
    let result = compile_and_execute(source);
    assert_eq!(result.unwrap(), Value::Smol(42));
}
```

#### 7.2.2 Error Propagation Tests
```rust
#[test]
fn test_invalid_conversion_error() {
    let source = r#""hello" as normie"#;
    let error = compile(source).unwrap_err();
    assert!(matches!(error, Error::TypeConversion { .. }));
}
```

### 7.3 Performance Tests

#### 7.3.1 Conversion Benchmarks
```rust
#[bench]
fn bench_integer_conversions(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..1000 {
            let _: i8 = (i as i64) as i8;
        }
    });
}
```

#### 7.3.2 Complex Conversion Benchmarks
```rust
#[bench]
fn bench_string_to_bytes_conversion(b: &mut Bencher) {
    let test_string = "Hello, World!".repeat(100);
    b.iter(|| {
        let _bytes: Vec<u8> = test_string.as_bytes().to_vec();
    });
}
```

### 7.4 Fuzzing Tests

#### 7.4.1 Conversion Input Fuzzing
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn fuzz_integer_conversions(value in any::<i64>()) {
        // Test all integer type conversions with random inputs
        let small: i8 = value as i8;
        let medium: i16 = value as i16;
        let normal: i32 = value as i32;
        // Verify no panics occur
    }
}
```

### 7.5 Compatibility Tests

#### 7.5.1 Cross-Platform Tests
Test conversions work identically across different platforms:
- Linux x86_64
- Windows x86_64
- macOS ARM64

#### 7.5.2 LLVM Version Compatibility
Test with different LLVM versions to ensure consistent behavior.

## 8. Performance Considerations

### 8.1 Optimization Opportunities

#### 8.1.1 Compile-Time Optimizations
- **Constant Folding**: Evaluate constant conversions at compile time
- **Conversion Elimination**: Remove redundant conversions (T → U → T)
- **Conversion Combining**: Combine multiple conversions into single operations

#### 8.1.2 Runtime Optimizations
- **Vectorization**: Use SIMD instructions for bulk conversions
- **Branch Prediction**: Optimize common conversion paths
- **Cache Locality**: Minimize memory access patterns

### 8.2 Memory Management

#### 8.2.1 Temporary Value Management
- Minimize allocation of temporary values during conversions
- Use stack allocation where possible
- Implement conversion in-place when safe

#### 8.2.2 String Conversion Optimization
- Reuse string buffers for repeated conversions
- Implement copy-on-write semantics for string-to-string conversions
- Use memory mapping for large string operations

### 8.3 Performance Monitoring

#### 8.3.1 Conversion Metrics
Track and optimize:
- Conversion frequency by type pair
- Conversion latency distribution
- Memory allocation patterns
- Cache miss rates

#### 8.3.2 Performance Regression Detection
- Automated benchmarking in CI/CD
- Performance baseline tracking
- Alert system for significant regressions

## 9. Future Enhancements

### 9.1 Advanced Features

#### 9.1.1 Custom Conversion Traits
```rust
trait CustomConversion<T, U> {
    fn convert(&self, value: T) -> Result<U, ConversionError>;
    fn is_safe(&self) -> bool;
    fn cost_estimate(&self) -> ConversionCost;
}
```

#### 9.1.2 Automatic Conversion Discovery
- Analyze type relationships to suggest valid conversions
- Generate conversion paths through intermediate types
- Optimize multi-step conversion chains

#### 9.1.3 Conversion Optimization Passes
- LLVM-level optimization passes specific to type conversions
- Profile-guided optimization for common conversion patterns
- Auto-vectorization of conversion operations

### 9.2 Language Integration

#### 9.2.1 Implicit Conversions
Limited implicit conversions for enhanced ergonomics:
- Numeric widening (smol → normie → thicc)
- Array to slice promotion
- Interface implementation promotion

#### 9.2.2 Conversion Operators
Additional syntax for specialized conversions:
- `value as? Type` - Optional conversion (returns Option<Type>)
- `value as! Type` - Forced conversion (panics on failure)
- `value into Type` - Move conversion consuming the source

### 9.3 Tooling Support

#### 9.3.1 IDE Integration
- Conversion suggestion in IDEs
- Conversion safety warnings
- Performance impact indicators

#### 9.3.2 Static Analysis Tools
- Conversion safety analyzer
- Performance impact analyzer
- Conversion pattern recommendations

## 10. Implementation Roadmap

### Phase 1: Core Infrastructure (Week 1-2)
1. Implement `ConversionRegistry` with basic type mappings
2. Create `ConversionValidator` for compile-time checking
3. Enhance `TypeConversionError` with rich context
4. Basic LLVM conversion implementations

### Phase 2: Primitive Conversions (Week 3-4)
1. Implement all numeric type conversions
2. Add character and byte conversions
3. Boolean conversion support
4. Comprehensive unit testing

### Phase 3: Composite Conversions (Week 5-6)
1. Pointer conversion implementations
2. Array and slice conversions
3. String conversion optimizations
4. Interface conversion support

### Phase 4: Integration and Optimization (Week 7-8)
1. Integrate with existing type checker
2. Enhance LLVM codegen integration
3. Performance optimization passes
4. Integration testing

### Phase 5: Advanced Features (Week 9-10)
1. Custom conversion support
2. Runtime conversion handling
3. Error recovery mechanisms
4. Performance monitoring

### Phase 6: Testing and Documentation (Week 11-12)
1. Comprehensive test suite completion
2. Performance benchmarking
3. Documentation finalization
4. User guide creation

This architecture provides a solid foundation for implementing a robust, performant, and extensible type conversion system for the CURSED programming language.
