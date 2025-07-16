# Property-Based Testing Framework for CURSED

A comprehensive QuickCheck-style property-based testing framework that allows you to test properties of your code by generating random inputs and automatically shrinking failing cases to find minimal counterexamples.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Generators](#generators)
- [Properties](#properties)
- [Shrinking](#shrinking)
- [Configuration](#configuration)
- [Advanced Features](#advanced-features)
- [Examples](#examples)
- [Integration with testz](#integration-with-testz)
- [API Reference](#api-reference)

## Overview

Property-based testing is a testing methodology where you specify properties that your code should satisfy, and the framework automatically generates test cases to verify these properties. Instead of writing specific test cases, you write general properties that should hold for all inputs.

### Key Features

- **Automatic Test Case Generation**: Generate random inputs for your tests
- **Intelligent Shrinking**: When a test fails, automatically find the smallest failing case
- **Multiple Data Type Generators**: Built-in generators for integers, strings, lists, and more
- **Property Combinators**: Compose properties using logical operators
- **Statistical Testing**: Verify distribution properties of your generators
- **Integration with testz**: Seamless integration with CURSED's existing test framework

## Quick Start

```cursed
yeet "property_test"
yeet "testz"

# Define a property: addition is commutative
slay addition_commutative(a normie, b normie) lit {
    damn a + b == b + a
}

# Test the property
slay test_addition() {
    forall(
        slay() { damn [gen_int(-100, 100), gen_int(-100, 100)] },
        slay(inputs []) { 
            damn addition_commutative(inputs[0], inputs[1]) 
        },
        "Addition is commutative"
    )
}

test_addition()
```

## Generators

Generators produce random values for testing. The framework provides built-in generators for common types:

### Basic Generators

```cursed
# Integer generators
sus small_int normie = gen_int(-100, 100)
sus positive normie = gen_positive_int()
sus large_int normie = gen_large_int()

# Boolean generator
sus random_bool lit = gen_boolean()

# Character and string generators
sus random_char sip = gen_char()
sus short_string tea = gen_string(10)
sus ascii_string tea = gen_ascii_string(15)
sus non_empty tea = gen_non_empty_string()

# List generator
sus int_list [] = gen_list_int(20)
```

### Composite Generators

```cursed
# Sorted list generator
sus sorted [] = gen_sorted_list(10)

# Email generator
sus email tea = gen_email()
```

### Custom Generators

Create your own generators by combining existing ones:

```cursed
slay gen_even_int() normie {
    sus base normie = gen_int(0, 50)
    damn base * 2
}

slay gen_coordinate() [] {
    damn [gen_int(-100, 100), gen_int(-100, 100)]
}

slay gen_person() [] {
    sus name tea = gen_ascii_string(10)
    sus age normie = gen_int(0, 120)
    sus email tea = gen_email()
    damn [name, age, email]
}
```

## Properties

Properties are predicates that should hold for all generated inputs. The framework provides several ways to define and combine properties:

### Basic Property Definition

```cursed
# Simple property
slay list_length_property(list []) lit {
    sus reversed [] = reverse_list(list)
    damn size(list) == size(reversed)
}

# Property with multiple inputs
slay string_concat_property(s1 tea, s2 tea) lit {
    sus concat tea = s1 + s2
    damn len(concat) == len(s1) + len(s2)
}
```

### Property Combinators

```cursed
# Idempotent property: f(f(x)) == f(x)
sus abs_idempotent slay = prop_idempotent(abs_function)

# Commutative property: f(a, b) == f(b, a)
sus add_commutative slay = prop_commutative(add_function)

# Associative property: f(f(a, b), c) == f(a, f(b, c))
sus add_associative slay = prop_associative(add_function)

# Reversible property: inverse(f(x)) == x
sus encode_decode slay = prop_reversible(encode_function, decode_function)
```

### Conditional Properties

```cursed
slay division_property(a normie, b normie) lit {
    # Only test when divisor is non-zero
    vibes b == 0 {
        damn based  # Vacuously true
    }
    sus result normie = a / b
    damn result * b == a
}
```

### Property Composition

```cursed
# Combine properties with logical operators
slay combined_property(x normie) lit {
    sus prop1 slay = slay(val) { damn val >= 0 }
    sus prop2 slay = slay(val) { damn val <= 1000 }
    sus combined slay = conjoin(prop1, prop2)
    damn combined(x)
}
```

## Shrinking

When a property fails, the framework automatically attempts to find the smallest failing input through a process called shrinking:

### How Shrinking Works

1. **Initial Failure**: A generated input causes the property to fail
2. **Generate Candidates**: Create smaller versions of the failing input
3. **Test Candidates**: Check if smaller inputs also fail
4. **Iterate**: Continue shrinking until no smaller failing input is found

### Shrinking Strategies

```cursed
# Integer shrinking: moves towards zero
shrink_int(42)  # Produces: [0, 21, 41]

# String shrinking: reduces length and removes characters
shrink_string("hello")  # Produces: ["", "he", "llo", "ell", "hell"]

# List shrinking: removes elements and reduces size
shrink_list([1, 2, 3, 4])  # Produces: [[], [1, 2], [3, 4], [2, 3, 4], [1, 2, 3]]
```

### Custom Shrinking

```cursed
slay shrink_coordinate(coord []) [] {
    sus x normie = coord[0]
    sus y normie = coord[1]
    sus shrunk [] = []
    
    # Shrink towards origin
    shrunk = shrunk + [[0, 0]]
    shrunk = shrunk + [[x / 2, y / 2]]
    shrunk = shrunk + [[x, 0]]
    shrunk = shrunk + [[0, y]]
    
    damn shrunk
}
```

## Configuration

Customize the behavior of property tests:

```cursed
# Set number of test cases to run
set_test_count(200)

# Set maximum shrinking attempts
set_max_shrinks(50)

# Set random seed for reproducible tests
set_seed(12345)
```

## Advanced Features

### Statistical Testing

Test the distribution properties of your generators:

```cursed
# Test that boolean generator produces roughly 50% true values
slay is_true(x lit) lit {
    damn x
}

sus distribution_ok lit = prop_distribution_test(
    gen_boolean,
    is_true,
    0.5  # Expected ratio
)
```

### Performance Testing

Combine property testing with performance requirements:

```cursed
slay sorting_performance_property(list []) lit {
    sus start_time normie = current_time()
    sus sorted [] = sort_function(list)
    sus end_time normie = current_time()
    sus duration normie = end_time - start_time
    
    # Property: sorting should complete in reasonable time
    damn duration < size(list) * size(list)  # O(n²) bound
}
```

### State-Based Testing

Test stateful systems by generating sequences of operations:

```cursed
slay gen_stack_operation() [] {
    sus operation normie = gen_int(0, 2)
    vibes operation == 0 {
        damn ["push", gen_int(1, 100)]
    } mil operation == 1 {
        damn ["pop"]
    } nah {
        damn ["peek"]
    }
}

slay stack_property(operations []) lit {
    sus stack [] = []
    sus i normie = 0
    bestie i < size(operations) {
        sus op [] = operations[i]
        # Apply operation to stack and verify invariants
        i = i + 1
    }
    damn based  # All operations maintained stack invariants
}
```

## Examples

### Testing Mathematical Functions

```cursed
yeet "property_test"
yeet "testz"

# Test that square root is inverse of squaring (for positive numbers)
slay sqrt_square_property(x drip) lit {
    vibes x < 0.0 {
        damn based  # Skip negative numbers
    }
    sus squared drip = x * x
    sus root drip = sqrt(squared)
    damn abs(root - x) < 0.001  # Within tolerance
}

slay test_sqrt() {
    forall(
        slay() { damn gen_int(0, 1000) },
        sqrt_square_property,
        "Square root is inverse of squaring"
    )
}
```

### Testing String Operations

```cursed
# Test string concatenation properties
slay string_properties() {
    # Concatenation is associative
    forall(
        slay() { damn [gen_string(10), gen_string(10), gen_string(10)] },
        slay(strings []) {
            sus a tea = strings[0]
            sus b tea = strings[1] 
            sus c tea = strings[2]
            damn (a + b) + c == a + (b + c)
        },
        "String concatenation is associative"
    )
    
    # Empty string is identity
    forall(
        slay() { damn gen_string(20) },
        slay(s tea) {
            damn s + "" == s && "" + s == s
        },
        "Empty string is concatenation identity"
    )
}
```

### Testing Data Structures

```cursed
# Test list operations
slay list_properties() {
    # Appending preserves order
    forall(
        slay() { damn [gen_list_int(10), gen_int(-100, 100)] },
        slay(data []) {
            sus list [] = data[0]
            sus element normie = data[1]
            sus new_list [] = append(list, element)
            damn size(new_list) == size(list) + 1 &&
                 last_element(new_list) == element
        },
        "Append preserves order and increases size"
    )
}
```

### Testing Parsing and Serialization

```cursed
# Test that parse/serialize are inverses
slay json_roundtrip_property(data []) lit {
    sus json_string tea = serialize_to_json(data)
    sus parsed_data [] = parse_from_json(json_string)
    damn deep_equals(data, parsed_data)
}

slay test_json_roundtrip() {
    forall(
        slay() { damn gen_json_compatible_data() },
        json_roundtrip_property,
        "JSON serialize/parse roundtrip"
    )
}
```

## Integration with testz

The property testing framework integrates seamlessly with CURSED's existing testz framework:

```cursed
yeet "testz"
yeet "property_test"

# Regular unit test
test_start("Basic addition test")
assert_eq_int(2 + 2, 4)

# Property-based test
forall(
    slay() { damn gen_int(-100, 100) },
    slay(x normie) { damn x + 0 == x },
    "Zero is additive identity"
)

# Another unit test
test_start("String length test")
assert_eq_int(len("hello"), 5)

print_test_summary()
```

## API Reference

### Generators

| Function | Description | Example |
|----------|-------------|---------|
| `gen_int(min, max)` | Generate integer in range | `gen_int(-10, 10)` |
| `gen_positive_int()` | Generate positive integer | `gen_positive_int()` |
| `gen_boolean()` | Generate random boolean | `gen_boolean()` |
| `gen_char()` | Generate printable ASCII character | `gen_char()` |
| `gen_string(max_len)` | Generate string up to max length | `gen_string(20)` |
| `gen_list_int(max_len)` | Generate list of integers | `gen_list_int(10)` |
| `gen_email()` | Generate email-like string | `gen_email()` |

### Property Combinators

| Function | Description | Example |
|----------|-------------|---------|
| `prop_idempotent(fn)` | Test f(f(x)) == f(x) | `prop_idempotent(abs)` |
| `prop_commutative(fn)` | Test f(a,b) == f(b,a) | `prop_commutative(add)` |
| `prop_associative(fn)` | Test f(f(a,b),c) == f(a,f(b,c)) | `prop_associative(mul)` |
| `prop_reversible(fn, inv)` | Test inv(fn(x)) == x | `prop_reversible(encode, decode)` |
| `conjoin(p1, p2)` | Logical AND of properties | `conjoin(prop1, prop2)` |
| `disjoin(p1, p2)` | Logical OR of properties | `disjoin(prop1, prop2)` |

### Test Execution

| Function | Description | Example |
|----------|-------------|---------|
| `forall(gen, prop, desc)` | Run property test | `forall(gen_int, prop_fn, "test")` |
| `run_property_test(prop, gen, desc)` | Manual property test execution | `run_property_test(prop, gen, "test")` |

### Configuration

| Function | Description | Default |
|----------|-------------|---------|
| `set_test_count(n)` | Set number of test cases | 100 |
| `set_max_shrinks(n)` | Set maximum shrinking attempts | 100 |
| `set_seed(n)` | Set random seed | 42 |

### Shrinking

| Function | Description | Return Type |
|----------|-------------|-------------|
| `shrink_int(x)` | Shrink integer towards zero | `[]` |
| `shrink_string(s)` | Shrink string by reducing length | `[]` |
| `shrink_list(list)` | Shrink list by removing elements | `[]` |

## Best Practices

1. **Start Simple**: Begin with basic properties before testing complex scenarios
2. **Use Appropriate Generators**: Choose generators that produce realistic test data
3. **Handle Edge Cases**: Use conditional properties to handle division by zero, etc.
4. **Combine with Unit Tests**: Use property tests alongside traditional unit tests
5. **Set Appropriate Test Counts**: Higher counts for critical properties, lower for expensive tests
6. **Use Descriptive Names**: Clear property descriptions help with debugging
7. **Test Your Tests**: Verify that properties can actually fail by testing with known bad implementations

## Common Patterns

### Testing Invariants

```cursed
# Data structure invariants
slay heap_property(heap []) lit {
    # Parent is always >= children
    damn check_heap_invariant(heap)
}

# Business rule invariants  
slay account_balance_property(transactions []) lit {
    sus balance normie = apply_transactions(transactions)
    damn balance >= 0  # Never go negative
}
```

### Testing Equivalence

```cursed
# Two implementations should be equivalent
slay sort_equivalence(list []) lit {
    sus result1 [] = quicksort(list)
    sus result2 [] = mergesort(list)
    damn lists_equal(result1, result2)
}
```

### Testing Optimization

```cursed
# Optimized version should produce same result
slay optimization_correctness(input []) lit {
    sus normal_result = slow_function(input)
    sus optimized_result = fast_function(input)
    damn normal_result == optimized_result
}
```

This comprehensive property-based testing framework provides powerful tools for testing your CURSED code through automated test case generation, intelligent shrinking, and flexible property composition.
