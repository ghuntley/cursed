# quick_test Module

A pure CURSED implementation of property-based testing framework for discovering edge cases and bugs through automatic test input generation.

## Overview

The `quick_test` module provides tools for property-based testing, allowing you to test code against a wide range of automatically generated inputs. This approach helps discover edge cases and bugs that might be missed by traditional unit tests.

## Features

- **Property-based testing**: Test functions with automatically generated inputs
- **Input shrinking**: Automatically find minimal failing cases
- **Configurable test parameters**: Control test iterations, input ranges, and behavior
- **Multiple generators**: Built-in generators for various data types
- **Test statistics**: Collect and analyze test execution statistics
- **Pure CURSED implementation**: No external dependencies or FFI usage

## Core Types

### Config
Configuration structure for controlling test execution:
```cursed
Config{
    MaxCount: 100,        # Maximum number of test iterations
    MaxSize: 100,         # Maximum size of generated values
    MinSize: 1,           # Minimum size of generated values
    ExpectFailure: cap,   # Whether the test should fail
    MaxFailures: 1,       # Maximum failures before stopping
    MaxShrinkCount: 10,   # Maximum shrink iterations
    Quiet: cap           # Suppress output during testing
}
```

### Result
Structure containing test execution results:
```cursed
Result{
    Passed: based,        # Did the test pass?
    Count: 50,           # Number of iterations performed
    FailedAfter: 0,      # Iteration that caused failure
    Input: 123,          # Input that caused failure
    ShrunkInput: 45,     # Shrunk version of failing input
    ShrinkCount: 3       # Number of shrink iterations
}
```

## Core Functions

### Configuration Functions
- `default_config()` - Create default configuration
- `config_with_max_count(max_count normie)` - Create config with custom iteration count
- `config_with_range(min_size normie, max_size normie)` - Create config with custom range

### Property Testing Functions
- `check_property(test_func tea, config Config)` - Run property test with configuration
- `test_property_with_gen(property_name tea, gen_func tea, max_count normie)` - Test property with specific generator
- `quick_check_positive(max_count normie)` - Quick test for positive numbers
- `quick_check_even(max_count normie)` - Quick test for even numbers
- `quick_check_abs_positive(max_count normie)` - Quick test for absolute value positivity

### Generator Functions
- `gen_int()` - Generate random integer (0-999)
- `gen_small_int()` - Generate small random integer (0-99)
- `gen_positive_int()` - Generate positive integer (1-999)
- `gen_negative_int()` - Generate negative integer (-1000 to -1)
- `gen_bool()` - Generate random boolean
- `int_generator(min normie, max normie)` - Generate integer in range
- `random_int_range(min normie, max normie)` - Generate random integer in range
- `random_bool()` - Generate random boolean

### Property Functions
- `is_positive_property(x normie)` - Test if number is positive
- `is_even_property(x normie)` - Test if number is even
- `abs_non_negative_property(x normie)` - Test if absolute value is non-negative

### Utility Functions
- `set_seed(seed normie)` - Set random seed for reproducible tests
- `shrink_input(input normie, test_func tea)` - Shrink failing input to minimal case
- `collect_test_stats(property_name tea, iterations normie)` - Collect test statistics
- `generate_test_report(property_name tea, iterations normie)` - Generate test report
- `run_comprehensive_tests()` - Run comprehensive test suite

## Usage Examples

### Basic Property Testing
```cursed
yeet "quick_test"

# Test that absolute value is always non-negative
sus config Config = default_config()
sus result Result = check_property("abs_positive", config)

vibes result.Passed {
    vibez.spill("✅ Test passed after " + result.Count + " iterations")
} nah {
    vibez.spill("❌ Test failed at iteration " + result.FailedAfter)
    vibez.spill("   Failing input: " + result.Input)
    vibez.spill("   Shrunk input: " + result.ShrunkInput)
}
```

### Testing with Custom Configuration
```cursed
# Create custom configuration
sus config Config = config_with_max_count(50)
config.Quiet = based  # Suppress output

# Test positive number property
sus result Result = check_property("is_positive", config)
```

### Using Different Generators
```cursed
# Test property with specific generator
sus result Result = test_property_with_gen("is_positive", "gen_positive_int", 100)

# Test with small integers
sus result2 Result = test_property_with_gen("is_even", "gen_small_int", 50)
```

### Custom Range Testing
```cursed
# Test with custom range
sus config Config = config_with_range(10, 100)
sus result Result = check_property("is_positive", config)
```

### Statistics Collection
```cursed
# Collect test statistics
sus passed_count normie = collect_test_stats("abs_non_negative", 200)
vibez.spill("Passed: " + passed_count + " out of 200 tests")

# Generate detailed report
generate_test_report("is_positive", 100)
```

### Reproducible Testing
```cursed
# Set seed for reproducible results
set_seed(42)
sus result Result = check_property("is_even", default_config())

# Reset seed to get same results
set_seed(42)
sus result2 Result = check_property("is_even", default_config())
```

## Property Examples

### Testing Mathematical Properties
```cursed
# Test that max(x, y) >= x for all x, y
slay test_max_property() {
    sus config Config = config_with_max_count(100)
    
    # Generate two numbers and test max property
    sus i normie = 0
    bestie i < config.MaxCount {
        sus x normie = gen_int()
        sus y normie = gen_int()
        sus max_val normie = x
        vibes y > x {
            max_val = y
        }
        
        vibes max_val < x || max_val < y {
            vibez.spill("❌ Max property failed: max(" + x + ", " + y + ") = " + max_val)
            damn cap
        }
        
        i = i + 1
    }
    
    vibez.spill("✅ Max property passed for " + config.MaxCount + " cases")
    damn based
}
```

### Testing Data Structure Properties
```cursed
# Test that reversing a list twice gives original
slay test_reverse_twice_property() {
    sus config Config = config_with_max_count(50)
    
    sus i normie = 0
    bestie i < config.MaxCount {
        sus original normie = gen_int()
        
        # Simple test: reversing a single element is itself
        sus reversed normie = original
        sus double_reversed normie = reversed
        
        vibes original != double_reversed {
            vibez.spill("❌ Reverse twice property failed")
            damn cap
        }
        
        i = i + 1
    }
    
    vibez.spill("✅ Reverse twice property passed")
    damn based
}
```

## Advanced Usage

### Comprehensive Test Suite
```cursed
# Run comprehensive property tests
sus success lit = run_comprehensive_tests()
vibes success {
    vibez.spill("🎉 All comprehensive tests passed")
} nah {
    vibez.spill("❌ Some comprehensive tests failed")
}
```

### Custom Property Testing
```cursed
# Define custom property
slay my_custom_property(x normie) lit {
    # Test that x^2 >= 0
    sus squared normie = x * x
    damn squared >= 0
}

# Test with different generators
sus result1 Result = test_property_with_gen("custom", "gen_int", 50)
sus result2 Result = test_property_with_gen("custom", "gen_small_int", 30)
```

## Implementation Details

- **Random Number Generation**: Uses linear congruential generator for reproducible results
- **Shrinking Strategy**: Iteratively reduces failing inputs to find minimal cases
- **Memory Efficient**: Pure CURSED implementation without external dependencies
- **Configurable**: Flexible configuration options for different testing scenarios
- **Statistics**: Built-in statistics collection and reporting capabilities

## Testing

Run the test suite with:
```bash
cargo run --bin cursed stdlib/quick_test/test_quick_test.csd
```

For compiled testing:
```bash
cargo run --bin cursed -- compile stdlib/quick_test/test_quick_test.csd
./test_quick_test
```

## Performance Notes

- **Generator Efficiency**: Simple generators optimized for speed
- **Shrinking Performance**: Limited shrinking iterations to balance thoroughness with speed
- **Memory Usage**: Minimal memory footprint through efficient data structures
- **Scalability**: Configurable iteration counts for different testing needs

## Integration

The quick_test module integrates seamlessly with the testz testing framework and can be used alongside other stdlib modules for comprehensive testing strategies.
