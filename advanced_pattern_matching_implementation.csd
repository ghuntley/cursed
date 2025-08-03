yeet "testz"

fr fr Advanced Pattern Matching Implementation Test
fr fr Tests all enhanced pattern matching features including:
fr fr - Complex destructuring patterns
fr fr - Guards with complex conditions
fr fr - Exhaustiveness checking
fr fr - Nested pattern matching
fr fr - Performance optimizations

test_start("Advanced Pattern Matching - Complex Destructuring")

fr fr Test tuple destructuring with guards
slay test_tuple_destructuring() normie {
    sus data := (42, "hello", based)
    
    sus result := match data {
        (x, y, z) when x > 40 && y == "hello" -> "complex match"
        (x, _, _) when x < 20 -> "small number"
        (_, y, _) when y == "world" -> "world greeting"
        _ -> "no match"
    }
    
    damn result
}

assert_eq_string(test_tuple_destructuring(), "complex match")

fr fr Test struct destructuring patterns
struct Person {
    name tea
    age drip
    active lit
}

slay test_struct_destructuring() normie {
    sus person := Person { name: "Alice", age: 25, active: based }
    
    sus result := match person {
        Person { name: "Alice", age: x, .. } when x >= 18 -> "adult Alice"
        Person { name: n, age: x, active: based } when x < 18 -> "young active person"
        Person { active: cringe, .. } -> "inactive person"
        _ -> "unknown person"
    }
    
    damn result
}

assert_eq_string(test_struct_destructuring(), "adult Alice")

fr fr Test array destructuring with rest patterns
slay test_array_destructuring() normie {
    sus numbers := [1, 2, 3, 4, 5]
    
    sus result := match numbers {
        [first, second, ...rest] when first == 1 && second == 2 -> "starts with 1,2"
        [x] when x > 10 -> "single large number"
        [x, y] when x + y == 10 -> "sum is 10"
        [] -> "empty array"
        _ -> "other pattern"
    }
    
    damn result
}

assert_eq_string(test_array_destructuring(), "starts with 1,2")

fr fr Test nested pattern matching
enum Result[T, E] {
    Ok(T)
    Err(E)
}

enum Option[T] {
    Some(T)
    None
}

slay test_nested_patterns() normie {
    sus nested_data := Ok(Some(42))
    
    sus result := match nested_data {
        Ok(Some(x)) when x > 40 -> "success with large value"
        Ok(Some(x)) when x <= 40 -> "success with small value"
        Ok(None) -> "success but empty"
        Err(e) -> "error occurred"
    }
    
    damn result
}

assert_eq_string(test_nested_patterns(), "success with large value")

fr fr Test or patterns with complex conditions
slay test_or_patterns() normie {
    sus value := 15
    
    sus result := match value {
        1 | 2 | 3 when value < 2 -> "very small"
        4 | 5 | 6 -> "small range"
        10 | 15 | 20 when value % 5 == 0 -> "divisible by 5"
        x when x > 50 -> "large number"
        _ -> "other"
    }
    
    damn result
}

assert_eq_string(test_or_patterns(), "divisible by 5")

fr fr Test range patterns
slay test_range_patterns() normie {
    sus score := 85
    
    sus grade := match score {
        90..100 -> "A"
        80..89 -> "B"
        70..79 -> "C"
        60..69 -> "D"
        0..59 -> "F"
        _ -> "Invalid"
    }
    
    damn grade
}

assert_eq_string(test_range_patterns(), "B")

fr fr Test exhaustiveness checking
enum Color {
    Red
    Green
    Blue
    Yellow
}

slay test_exhaustiveness() normie {
    sus color := Color::Red
    
    fr fr This should compile and work correctly
    sus description := match color {
        Color::Red -> "red color"
        Color::Green -> "green color"
        Color::Blue -> "blue color"
        Color::Yellow -> "yellow color"
    }
    
    damn description
}

assert_eq_string(test_exhaustiveness(), "red color")

fr fr Test pattern matching performance optimization
slay test_pattern_performance() normie {
    sus large_number := 1000000
    
    fr fr Test that pattern matching is optimized for large numbers
    sus category := match large_number {
        0..999 -> "small"
        1000..99999 -> "medium"
        100000..999999 -> "large"
        1000000..9999999 -> "very large"
        _ -> "huge"
    }
    
    damn category
}

assert_eq_string(test_pattern_performance(), "very large")

fr fr Test guard expressions with complex logic
slay test_complex_guards() normie {
    sus data := (25, "developer", based)
    
    sus result := match data {
        (age, profession, active) when active && age >= 18 && profession == "developer" -> "qualified developer"
        (age, profession, _) when age >= 65 && profession == "teacher" -> "retired teacher"
        (age, _, active) when !active -> "inactive person"
        _ -> "other"
    }
    
    damn result
}

assert_eq_string(test_complex_guards(), "qualified developer")

fr fr Test type patterns for dynamic dispatch
trait Display {
    slay show() tea
}

struct Number {
    value drip
}

impl Number : Display {
    slay show() tea {
        damn format("Number: {}", value)
    }
}

struct Text {
    content tea
}

impl Text : Display {
    slay show() tea {
        damn format("Text: {}", content)
    }
}

slay test_type_patterns(obj Display) normie {
    sus result := match obj {
        Number { value: x } when x > 100 -> "large number object"
        Number { .. } -> "number object"
        Text { content: c } when c.len() > 10 -> "long text object"
        Text { .. } -> "text object"
        _ -> "unknown object"
    }
    
    damn result
}

sus number := Number { value: 150 }
assert_eq_string(test_type_patterns(number), "large number object")

print_test_summary()
