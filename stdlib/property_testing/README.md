# Property Testing Module

Comprehensive property-based testing framework for CURSED that enables testing code properties across many randomly generated inputs.

## Overview

Property-based testing is a testing methodology where you specify properties (invariants) that should hold for your code, and the framework automatically generates many test cases to verify these properties. When a property fails, the framework uses shrinking to find the minimal failing case.

## Features

### Random Value Generators
- `generate_random_int(min, max)` - Generate random integers within range
- `generate_random_string(length)` - Generate random strings of specified length  
- `generate_random_boolean()` - Generate random boolean values

### Property Test Runners
- `run_property_test(name, iterations, property_func)` - Run property with multiple test cases
- `forall_property(name, iterations, generator, property)` - Test universal quantification
- `exists_property(name, iterations, generator, property)` - Test existential quantification

### Shrinking Functionality
- `shrink_string(input)` - Reduce string to find minimal failing case
- `shrink_int(input)` - Reduce integer towards zero
- `shrink_boolean(input)` - Prefer false for boolean shrinking

### Mathematical Property Testing
- `test_reflexivity(name, iterations, generator, relation)` - Test reflexive properties
- `test_symmetry(name, iterations, generator, relation)` - Test symmetric properties  
- `test_transitivity(name, iterations, generator, relation)` - Test transitive properties

### Property Assertions
- `property_assert_true(condition, message)` - Assert condition with custom message
- `property_assert_equal_int(actual, expected, message)` - Assert integer equality
- `property_assert_equal_string(actual, expected, message)` - Assert string equality

## Usage Examples

### Basic Property Testing
```cursed
yeet "property_testing"

# Test that addition is commutative
slay addition_commutative(str tea, a normie, b normie) lit {
    damn (a + b) == (b + a)
}

sus result lit = run_property_test("Addition commutativity", 100, addition_commutative)
```

### Forall Properties
```cursed
# Test that all strings have non-negative length
slay string_length_property(str tea, val normie, bool lit) lit {
    damn string.length(str) >= 0
}

slay string_generator() (tea, normie, lit) {
    sus test_str tea = generate_random_string(10)
    damn (test_str, 0, based)
}

sus result lit = forall_property("String length property", 50, string_generator, string_length_property)
```

### Mathematical Relations
```cursed
# Test that equality is reflexive
slay int_generator() normie {
    damn generate_random_int(1, 100)
}

slay equality_relation(a normie, b normie) lit {
    damn a == b
}

sus result lit = test_reflexivity("Equality reflexivity", 20, int_generator, equality_relation)
```

## Advanced Features

### Custom Generators
Create specialized generators for domain-specific testing:

```cursed
slay email_generator() tea {
    sus username tea = generate_random_string(5)
    sus domain tea = generate_random_string(8)
    damn username + "@" + domain + ".com"
}
```

### Property Composition
Combine multiple properties for comprehensive testing:

```cursed
slay complex_property(str tea, val normie, bool lit) lit {
    lowkey val > 0 && bool {
        damn (val * 2) > val && (val + 1) > val
    }
    damn based
}
```

### Error Analysis
Use shrinking to identify minimal failing cases:

```cursed
# When a property fails, the framework automatically:
# 1. Identifies the failing input
# 2. Attempts to shrink it to find minimal case
# 3. Reports both original and shrunk inputs
```

## Integration with testz

The property testing framework integrates seamlessly with the existing testz testing framework:

```cursed
yeet "testz"
yeet "property_testing"

test_start("My property test")
sus result lit = run_property_test("Property name", 100, my_property)
assert_true(result)
print_test_summary()
```

## Best Practices

### Property Design
1. **Start Simple**: Begin with basic properties like reflexivity
2. **Think Invariants**: Focus on what should always be true
3. **Use Preconditions**: Guard properties with appropriate conditions
4. **Test Edge Cases**: Include boundary conditions in generators

### Generator Strategy  
1. **Comprehensive Coverage**: Ensure generators cover the full input space
2. **Realistic Data**: Generate data that reflects real usage patterns
3. **Edge Cases**: Include special values (empty strings, zero, negative numbers)
4. **Controlled Randomness**: Use deterministic seeds for reproducible tests

### Performance Considerations
1. **Iteration Count**: Balance thoroughness with execution time
2. **Generator Efficiency**: Optimize generators for fast execution
3. **Early Termination**: Use appropriate loop breaks for efficiency
4. **Memory Management**: Be mindful of memory usage in large test runs

## Testing the Framework

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/property_testing/test_property_testing.csd

# Test compilation mode  
cargo run --bin cursed -- compile stdlib/property_testing/test_property_testing.csd
./test_property_testing

# Both-mode verification
test_both_modes() {
    cargo run --bin cursed stdlib/property_testing/test_property_testing.csd > interp_output.txt
    cargo run --bin cursed -- compile stdlib/property_testing/test_property_testing.csd
    ./test_property_testing > comp_output.txt
    diff interp_output.txt comp_output.txt
}
```

## Implementation Notes

This is a pure CURSED implementation with:
- Zero FFI dependencies
- Full integration with testz framework
- Comprehensive error handling
- Support for both interpretation and compilation modes
- Enterprise-grade reliability and performance

The framework follows CURSED specification standards and integrates with the existing development toolchain including LSP server, build system, and coverage analysis tools.
