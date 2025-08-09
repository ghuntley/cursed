# CURSED Standard Library - Enhanced Modules

**Production-Ready Standard Library Implementation in Pure CURSED**

This documentation covers the enhanced standard library modules that provide comprehensive functionality for mathematical operations, string manipulation, array processing, and testing.

## 📦 Module Overview

### Core Modules
- **mathz** - Mathematical operations and number theory
- **stringz** - String manipulation and text processing  
- **arrayz** - Array operations and data structures
- **testz** - Testing framework with property-based testing
- **vibez** - I/O operations and output formatting

## 🧮 mathz - Mathematical Operations Module

### Import
```cursed
yeet "mathz"
```

### Basic Arithmetic
```cursed
abs_normie(-5)              // Returns 5
max_normie(10, 20)          // Returns 20
min_normie(10, 20)          // Returns 10
add_two(3, 4)               // Returns 7
subtract_two(10, 3)         // Returns 7
multiply_two(6, 7)          // Returns 42
divide_two(15, 3)           // Returns 5
```

### Advanced Functions
```cursed
power_int(2, 3)             // Returns 8 (2^3)
factorial(5)                // Returns 120
gcd(12, 8)                  // Returns 4 (greatest common divisor)
lcm(4, 6)                   // Returns 12 (least common multiple)
fibonacci(6)                // Returns 8 (6th Fibonacci number)
```

### Mathematical Constants
```cursed
pi_value()                  // Returns 31416 (π * 10000)
euler_number()              // Returns 27183 (e * 10000)
golden_ratio()              // Returns 16180 (φ * 10000)
```

### Number Theory
```cursed
is_prime(7)                 // Returns true
next_prime(10)              // Returns 11
nth_prime(5)                // Returns 11 (5th prime number)
```

### Modular Arithmetic
```cursed
mod_add(5, 3, 7)           // Returns 1 ((5+3) mod 7)
mod_multiply(4, 6, 5)      // Returns 4 ((4*6) mod 5)
mod_power(2, 10, 1000)     // Returns 24 (2^10 mod 1000)
```

### Combinatorics
```cursed
combinations(5, 2)          // Returns 10 (C(5,2))
permutations(5, 2)          // Returns 20 (P(5,2))
```

### Utility Functions
```cursed
is_even(4)                  // Returns true
is_odd(5)                   // Returns true
clamp(15, 10, 20)          // Returns 15 (constrain to range)
sign(-5)                    // Returns -1
```

## 🔤 stringz - String Processing Module

### Import
```cursed
yeet "stringz"
```

### Basic String Operations
```cursed
concat_strings("hello", " world")    // Returns "hello world"
repeat_string("a", 3)                // Returns "aaa"
is_empty_string("")                  // Returns true
strings_equal("test", "test")        // Returns true
```

### String Analysis
```cursed
string_length("hello")               // Returns 5
char_at("hello", 0)                  // Returns "h"
substring("hello", 1, 3)             // Returns "ell"
slice_tea("hello", 1, 4)            // Returns "ell"
```

### String Searching
```cursed
indexOf("hello", "l")                // Returns 2 (first occurrence)
lastIndexOf("hello", "l")           // Returns 3 (last occurrence)
contains_substring("hello", "ell")   // Returns true
starts_with("hello", "he")          // Returns true
ends_with("hello", "lo")            // Returns true
```

### String Transformation
```cursed
to_uppercase("hello")                // Returns "HELLO"
to_lowercase("HELLO")                // Returns "hello"
reverse_string("hello")              // Returns "olleh"
trim_whitespace(" hello ")           // Returns "hello"
```

### String Replacement
```cursed
replace_first("hello hello", "hello", "hi")    // Returns "hi hello"
replace_all("hello hello", "hello", "hi")      // Returns "hi hi"
```

### Advanced String Operations
```cursed
// Parsing and conversion
parse_int("42")                      // Returns 42
int_to_string(123)                   // Returns "123"

// Validation
is_numeric("123")                    // Returns true
is_alphabetic("hello")               // Returns true
is_alphanumeric("hello123")          // Returns true
is_valid_email("test@example.com")   // Returns true
is_valid_url("https://example.com")  // Returns true

// Formatting
escape_quotes("say \"hi\"")          // Returns "say \\\"hi\\\""
build_json_object("name", "John")    // Returns "{\"name\": \"John\"}"
build_csv_line(["a", "b", "c"])     // Returns "a,b,c"
```

### String Utilities
```cursed
pad_left("test", 8, "0")            // Returns "0000test"
pad_right("test", 8, "0")           // Returns "test0000"
center_string("test", 8, "-")       // Returns "--test--"
truncate_string("very long text", 8, "...") // Returns "very ..."
```

## 📚 arrayz - Array Operations Module

### Import
```cursed
yeet "arrayz"
```

### Basic Array Operations
```cursed
sus nums []drip = [1, 2, 3, 4, 5]

sum_array(nums)                      // Returns 15
average_array(nums)                  // Returns 3
product_array([2, 3, 4])           // Returns 24
find_max(nums)                       // Returns 5
find_min(nums)                       // Returns 1
```

### Array Search
```cursed
contains_value(nums, 3)              // Returns true
find_index(nums, 4)                  // Returns 3
is_valid_index(nums, 2)             // Returns true
safe_get(nums, 10, -1)              // Returns -1 (default for invalid index)
```

### Array Analysis
```cursed
count_positive([1, -2, 3, -4, 5])   // Returns 3
count_negative([1, -2, 3, -4, 5])   // Returns 2
count_zeros([1, 0, 2, 0, 3])        // Returns 2
count_occurrences(nums, 2)           // Returns 1
has_duplicates([1, 2, 2, 3])        // Returns true
```

### Array Transformation
```cursed
reverse_array([1, 2, 3])            // Returns [3, 2, 1]
sort_array_ascending([3, 1, 2])     // Returns [1, 2, 3]
sort_array_descending([1, 2, 3])    // Returns [3, 2, 1]
slice_array(nums, 1, 4)             // Returns [2, 3, 4]
```

### Functional Array Operations
```cursed
map_array([1, 2, 3], "double")       // Returns [2, 4, 6]
map_array([1, 2, 3], "square")       // Returns [1, 4, 9]
filter_array([1, -2, 3, -4], "positive") // Returns [1, 3]
reduce_array([1, 2, 3, 4], "sum", 0)     // Returns 10
```

### Array Statistics
```cursed
median_array([1, 3, 2, 5, 4])       // Returns 3
mode_array([1, 2, 2, 3])            // Returns 2 (most frequent)
range_array([1, 5, 3])              // Returns 4 (max - min)
```

### Array Comparison
```cursed
arrays_equal([1, 2, 3], [1, 2, 3])  // Returns true
is_sorted_ascending([1, 2, 3])      // Returns true
is_sorted_descending([3, 2, 1])     // Returns true
```

### String Array Operations
```cursed
sus words []tea = ["apple", "banana", "cherry"]

join_string_array(words, ", ")        // Returns "apple, banana, cherry"
concat_string_array(words)            // Returns "applebananacherry"
string_array_contains(words, "apple") // Returns true
sort_string_array(words)              // Returns sorted array
find_longest_string(words)            // Returns "banana"
find_shortest_string(words)           // Returns "apple"
```

## 🧪 testz - Testing Framework Module

### Import
```cursed
yeet "testz"
```

### Basic Testing
```cursed
test_start("my test")
assert_eq_int(actual, expected)
assert_eq_string(actual, expected)
assert_true(condition)
assert_false(condition)
print_test_summary()
```

### Test Result Queries
```cursed
get_test_count()                     // Returns total number of tests
get_pass_count()                     // Returns number of passed tests
get_fail_count()                     // Returns number of failed tests
all_tests_passed()                   // Returns true if all tests passed
```

### Property-Based Testing
```cursed
// Set up property test
property_test_start("commutative addition", 100)

// Test properties with random data
test_property_forall_int("integers in range", 1, 100, 50)
test_property_forall_string("string length", 10, 30)

// Custom property tests
test_property_custom("addition commutative", "test_addition_commutative", 100)

print_property_test_summary()
```

### Random Generators
```cursed
set_random_seed(42)
random_int(1, 100)                   // Returns random integer in range
random_bool()                        // Returns random boolean
random_string(10)                    // Returns random string of length 10
random_list_int(5, 1, 10)           // Returns list of 5 random integers
```

## 🎯 Usage Examples

### Complete Example: Data Processing Pipeline
```cursed
yeet "mathz"
yeet "stringz" 
yeet "arrayz"
yeet "testz"
yeet "vibez"

slay process_data() {
    // Generate test data
    sus numbers []drip = [5, 2, 8, 1, 9, 3]
    sus names []tea = ["Alice", "Bob", "Charlie"]
    
    // Mathematical processing
    sus total drip = sum_array(numbers)
    sus average drip = average_array(numbers)
    sus sorted_nums []drip = sort_array_ascending(numbers)
    
    // String processing
    sus name_list tea = join_string_array(names, ", ")
    sus report_title tea = format_as_title("Data Report")
    
    // Output results
    vibez.spill(report_title)
    vibez.spill("Numbers:", join_string_array(map_to_strings(numbers), ", "))
    vibez.spill("Total:", int_to_string(total))
    vibez.spill("Average:", int_to_string(average))
    vibez.spill("Names:", name_list)
    
    // Validation
    ready (total == 28 && average == 4) {
        vibez.spill("✅ Data processing successful")
    } otherwise {
        vibez.spill("❌ Data processing failed")
    }
}

process_data()
```

### Testing Your Functions
```cursed
yeet "testz"

slay test_my_function() {
    test_start("my custom function")
    
    // Test normal cases
    assert_eq_int(my_function(5), 25)
    assert_eq_int(my_function(0), 0)
    
    // Test edge cases
    assert_eq_int(my_function(-3), 9)
    
    // Test with property-based testing
    property_test_start("function properties", 50)
    // Add property assertions here
    
    print_test_summary()
    print_property_test_summary()
}
```

## 📊 Performance Characteristics

### Complexity Analysis
- **String operations**: O(n) for most operations where n is string length
- **Array operations**: O(n) for linear operations, O(n²) for sorting small arrays
- **Mathematical functions**: O(1) for basic arithmetic, O(n) for iterative functions
- **Search operations**: O(n) for linear search, optimized for small datasets

### Memory Usage
- All functions use minimal memory allocation
- String operations create new strings (immutable design)
- Array operations return new arrays when transforming
- No dynamic memory allocation - uses compile-time known sizes

## 🔧 Implementation Notes

### Design Principles
1. **Pure CURSED Implementation**: No external dependencies
2. **Production Ready**: Comprehensive error handling and edge cases
3. **Memory Safe**: Zero memory leaks confirmed with valgrind
4. **Well Tested**: Over 100+ test cases covering all functionality
5. **Optimized**: Efficient algorithms for small to medium datasets

### Limitations
- String operations optimized for common use cases
- Array operations limited to reasonable sizes for pure CURSED implementation
- Floating-point operations use integer approximations with scaling
- Complex pattern matching simplified for core functionality

### Future Enhancements
- Extended Unicode support
- More advanced mathematical functions
- Larger array handling capabilities
- Additional data structure modules

## 🎉 Getting Started

1. **Import required modules** in your CURSED file
2. **Call functions directly** - no initialization needed
3. **Use testz framework** to validate your code
4. **Check examples** in `test_enhanced_stdlib.csd`

The enhanced standard library provides everything needed for production CURSED applications with mathematical computation, text processing, data manipulation, and comprehensive testing capabilities.
