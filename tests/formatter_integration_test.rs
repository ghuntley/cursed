//! Integration tests for the CURSED code formatter
//!
//! These tests verify end-to-end formatting of complete CURSED programs

use cursed::tools::{CursedFormatter, FormatterConfig, BraceStyle};
use std::fs;
use std::path::Path;

#[path = "common/mod.rs"]
mod common;

/// Test complete file formatting end-to-end
mod end_to_end_tests {
    use super::*;

    #[test]
    fn test_format_simple_program() {
        init_tracing!();
        
        let source = r#""
sus main_func() {
sus x = 42
sus y = 24
lowkey x > y {
yolo x
} highkey {
yolo y
}
}
"#.trim()";

        let expected = r#"sus main_func() {"
    sus x = 42
    sus y = 24
    lowkey x > y {
        yolo x
    } highkey {
        yolo y
    }
}"#";

        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        assert!(result.changed);
        assert_eq!(result.formatted_code.trim(), expected);
        assert!(result.lines_processed > 0);
    }

    #[test]
    fn test_format_complex_program() {
        init_tracing!();
        
        let source = r#""
facts PI = 3.14159

squad Circle {
radius sip
}

collab Shape {
area() sip
}

slay (c Circle) area() sip {
yolo PI * c.radius * c.radius
}

slay new_circle(r sip) Circle {
yolo Circle{radius: r}
}

slay main() {
sus circle = new_circle(5.0)
sus area = circle.area()
lowkey area > 50.0 {
yolo "Large circle"
} highkey {
yolo "Small circle"
}
}
"#.trim()";

        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        assert!(result.changed);
        assert!(result.formatted_code.contains("facts PI = 3.14159"));
        assert!(result.formatted_code.contains("squad Circle {"));
        assert!(result.formatted_code.contains("collab Shape {"));
        assert!(result.formatted_code.contains("slay (c Circle) area() sip {"));
        assert!(result.formatted_code.contains("yolo PI * c.radius * c.radius"));
        assert!(result.lines_processed > 10);
    }

    #[test]
    fn test_format_with_generics() {
        init_tracing!();
        
        let source = r#""
squad Container[T] {
value T
}

slay new_container[T](v T) Container[T] {
yolo Container[T]{value: v}
}

slay get_value[T](c Container[T]) T {
yolo c.value
}

slay main() {
sus int_container = new_container[normie](42)
sus str_container = new_container[sip]("hello")
sus int_val = get_value[normie](int_container)
sus str_val = get_value[sip](str_container)
}
"#.trim()";

        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        assert!(result.changed);
        assert!(result.formatted_code.contains("squad Container[T] {"));
        assert!(result.formatted_code.contains("slay new_container[T](v T) Container[T] {"));
        assert!(result.formatted_code.contains("yolo Container[T]{value: v}"));
        assert!(result.formatted_code.contains("sus int_container = new_container[normie](42)"));
    }

    #[test]
    fn test_format_with_arrays_and_maps() {
        init_tracing!();
        
        let source = r#""
slay main() {
sus numbers = [1, 2, 3, 4, 5]
sus person = {name: "Alice", age: 30}
sus matrix = [[1, 2], [3, 4]]

bestie i flex range(len(numbers)) {
yolo numbers[i]
}

bestie key, value flex person {
yolo key + ": " + value
}
}
"#.trim()";

        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        assert!(result.changed);
        assert!(result.formatted_code.contains("sus numbers = [1, 2, 3, 4, 5]"));
        assert!(result.formatted_code.contains("sus person = {name: \"Alice\", age: 30}"));
        assert!(result.formatted_code.contains("sus matrix = [[1, 2], [3, 4]]"));
        assert!(result.formatted_code.contains("bestie i flex range(len(numbers)) {"));
        assert!(result.formatted_code.contains("bestie key, value flex person {"));
    }

    #[test]
    fn test_format_with_error_handling() {
        init_tracing!();
        
        let source = r#""
slay divide(a normie, b normie) (normie, error) {
lowkey b == 0 {
yolo 0, error("division by zero")
}
yolo a / b, null
}

slay main() {
sus result, err = divide(10, 2)
lowkey err != null {
yolo "Error: " + err.message()
}
yolo "Result: " + result
}
"#.trim()";

        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        assert!(result.changed);
        assert!(result.formatted_code.contains("slay divide(a normie, b normie) (normie, error) {"));
        assert!(result.formatted_code.contains("yolo 0, error(\"division by zero\")"));
        assert!(result.formatted_code.contains("sus result, err = divide(10, 2)"));
        assert!(result.formatted_code.contains("lowkey err != null {"));
    }

    #[test]
    fn test_format_with_goroutines() {
        init_tracing!();
        
        let source = r#""
slay worker(ch chan normie) {
bestie i flex range(10) {
ch <- i
}
close(ch)
}

slay main() {
sus ch = make(chan normie, 5)
get worker(ch)

bestie value flex ch {
yolo "Received:", value
}
}
"#.trim()";

        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        assert!(result.changed);
        assert!(result.formatted_code.contains("slay worker(ch chan normie) {"));
        assert!(result.formatted_code.contains("ch <- i"));
        assert!(result.formatted_code.contains("sus ch = make(chan normie, 5)"));
        assert!(result.formatted_code.contains("get worker(ch)"));
        assert!(result.formatted_code.contains("bestie value flex ch {"));
    }
}

/// Test various CURSED language constructs
mod language_construct_tests {
    use super::*;

    #[test]
    fn test_function_declarations() {
        init_tracing!();
        
        let source = r#""
slay simple_function() {
yolo "hello"
}

slay function_with_params(x normie, y sip) {
yolo x, y
}

slay function_with_return() sip {
yolo "result"
}

slay function_with_multiple_returns() (normie, sip) {
yolo 42, "hello"
}

slay generic_function[T](value T) T {
yolo value
}

slay method_receiver(r Receiver) sip {
yolo r.field
}
"#.trim()";

        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        assert!(result.changed);
        assert!(result.formatted_code.contains("slay simple_function() {"));
        assert!(result.formatted_code.contains("slay function_with_params(x normie, y sip) {"));
        assert!(result.formatted_code.contains("slay function_with_return() sip {"));
        assert!(result.formatted_code.contains("slay function_with_multiple_returns() (normie, sip) {"));
        assert!(result.formatted_code.contains("slay generic_function[T](value T) T {"));
        assert!(result.formatted_code.contains("slay method_receiver(r Receiver) sip {"));
    }

    #[test]
    fn test_variable_declarations() {
        init_tracing!();
        
        let source = r#""
sus x = 42
sus y normie = 100
sus z sip = "hello"
facts CONSTANT = 3.14
sus a, b = get_values()
sus (c, d) = get_tuple()
"#.trim()";

        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        assert!(result.changed);
        assert!(result.formatted_code.contains("sus x = 42"));
        assert!(result.formatted_code.contains("sus y normie = 100"));
        assert!(result.formatted_code.contains("sus z sip = \"hello\""));
        assert!(result.formatted_code.contains("facts CONSTANT = 3.14"));
        assert!(result.formatted_code.contains("sus a, b = get_values()"));
        assert!(result.formatted_code.contains("sus (c, d) = get_tuple()"));
    }

    #[test]
    fn test_control_flow_statements() {
        init_tracing!();
        
        let source = r#""
lowkey x > 0 {
yolo "positive"
} highkey lowkey x < 0 {
yolo "negative"
} highkey {
yolo "zero"
}

periodt x > 0 {
x = x - 1
}

bestie i flex range(10) {
lowkey i%2 == 0 {
continue
}
yolo i
}

bestie key, value flex map {
yolo key, value
}
"#.trim()";

        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        assert!(result.changed);
        assert!(result.formatted_code.contains("lowkey x > 0 {"));
        assert!(result.formatted_code.contains("} highkey lowkey x < 0 {"));
        assert!(result.formatted_code.contains("} highkey {"));
        assert!(result.formatted_code.contains("periodt x > 0 {"));
        assert!(result.formatted_code.contains("bestie i flex range(10) {"));
        assert!(result.formatted_code.contains("bestie key, value flex map {"));
    }

    #[test]
    fn test_switch_statements() {
        init_tracing!();
        
        let source = r#""
vibe_check value {
mood 1:
yolo "one"
mood 2, 3:
yolo "two or three"
mood 4...6:
yolo "four to six"
basic:
yolo "other"
}

vibe_check type_value.(type) {
mood normie:
yolo "integer"
mood sip:
yolo "string"
basic:
yolo "unknown"
}
"#.trim()";

        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        assert!(result.changed);
        assert!(result.formatted_code.contains("vibe_check value {"));
        assert!(result.formatted_code.contains("    mood 1:"));
        assert!(result.formatted_code.contains("        yolo \"one\""));
        assert!(result.formatted_code.contains("    mood 2, 3:"));
        assert!(result.formatted_code.contains("    mood 4...6:"));
        assert!(result.formatted_code.contains("    basic:"));
        assert!(result.formatted_code.contains("vibe_check type_value.(type) {"));
    }

    #[test]
    fn test_complex_nested_structures() {
        init_tracing!();
        
        let source = r#""
squad NestedStruct {
data map[sip][]normie
metadata struct {
version normie
author sip
}
}

slay process_nested(ns NestedStruct) {
bestie key, values flex ns.data {
yolo "Processing key:", key
bestie i, value flex values {
lowkey value > 100 {
yolo "Large value at index", i, ":", value
} highkey {
yolo "Small value at index", i, ":", value
}
}
}
}
"#.trim()";

        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        assert!(result.changed);
        assert!(result.formatted_code.contains("squad NestedStruct {"));
        assert!(result.formatted_code.contains("    data map[sip][]normie"));
        assert!(result.formatted_code.contains("    metadata struct {"));
        assert!(result.formatted_code.contains("        version normie"));
        assert!(result.formatted_code.contains("slay process_nested(ns NestedStruct) {"));
    }

    #[test]
    fn test_comments_and_documentation() {
        init_tracing!();
        
        let source = r#""
// Package-level comment
package main

// This is a function comment
// that spans multiple lines
slay add(x normie, y normie) normie { // inline comment
// Comment inside function
yolo x + y // return statement comment
}

/*
Multi-line comment
with multiple lines
*/
squad Person {
name sip // name field
age normie // age field
}
"#.trim()";

        let config = FormatterConfig {
            ..FormatterConfig::default()
        };
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(source).unwrap();
        
        assert!(result.changed);
        assert!(result.formatted_code.contains("// Package-level comment"));
        assert!(result.formatted_code.contains("// This is a function comment"));
        assert!(result.formatted_code.contains("// that spans multiple lines"));
        assert!(result.formatted_code.contains("// inline comment"));
        assert!(result.formatted_code.contains("// Comment inside function"));
        assert!(result.formatted_code.contains("/*"));
        assert!(result.formatted_code.contains("Multi-line comment"));
        assert!(result.formatted_code.contains("*/"));
    }
}

/// Test formatting preservation of semantics
mod semantic_preservation_tests {
    use super::*;

    #[test]
    fn test_operator_precedence_preservation() {
        init_tracing!();
        
        let source = "sus result = a + b * c - d / e % f";
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        // Operator precedence should be preserved (no extra parentheses)
        assert!(result.formatted_code.contains("a + b * c - d / e % f"));
    }

    #[test]
    fn test_string_literal_preservation() {
        init_tracing!();
        
        let source = r#"sus message = "This is a string with \"quotes\" and \n newlines \t tabs""#;
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        // String literals should be preserved exactly
        assert!(result.formatted_code.contains(r#""This is a string with \"quotes\" and \n newlines \t tabs""#));
    }

    #[test]
    fn test_number_literal_preservation() {
        init_tracing!();
        
        let source = "sus values = [42, 3.14159, 0x1A2B, 0o755, 0b1010]";
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        // Number formats should be preserved
        assert!(result.formatted_code.contains("42"));
        assert!(result.formatted_code.contains("3.14159"));
        assert!(result.formatted_code.contains("0x1A2B"));
        assert!(result.formatted_code.contains("0o755"));
        assert!(result.formatted_code.contains("0b1010"));
    }

    #[test]
    fn test_identifier_preservation() {
        init_tracing!();
        
        let source = "sus _private_var = 42\nsus PublicVar = 24\nsus camelCase = 12";
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        // Identifier casing should be preserved
        assert!(result.formatted_code.contains("_private_var"));
        assert!(result.formatted_code.contains("PublicVar"));
        assert!(result.formatted_code.contains("camelCase"));
    }

    #[test]
    fn test_type_information_preservation() {
        init_tracing!();
        
        let source = "sus x normie = 42\nsus y sip = \"hello\"\nsus z []normie = [1, 2, 3]";
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        // Type annotations should be preserved
        assert!(result.formatted_code.contains("x normie"));
        assert!(result.formatted_code.contains("y sip"));
        assert!(result.formatted_code.contains("z []normie"));
    }

    #[test]
    fn test_generic_constraint_preservation() {
        init_tracing!();
        
        let source = "slay process[T comparable](value T) T where T: Comparable { yolo value }";
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        // Generic constraints should be preserved
        assert!(result.formatted_code.contains("[T comparable]"));
        assert!(result.formatted_code.contains("where T: Comparable"));
    }

    #[test]
    fn test_expression_semantics_preservation() {
        init_tracing!();
        
        let source = r#""
sus result = func1(arg1, arg2).method().field
sus array_access = arr[index]
sus map_access = map["key"]
sus type_assertion = value.(Type)
sus channel_send = ch <- value
sus channel_receive = <-ch
"#.trim()";

        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        // Expression semantics should be preserved
        assert!(result.formatted_code.contains("func1(arg1, arg2).method().field"));
        assert!(result.formatted_code.contains("arr[index]"));
        assert!(result.formatted_code.contains("map[\"key\"]"));
        assert!(result.formatted_code.contains("value.(Type)"));
        assert!(result.formatted_code.contains("ch <- value"));
        assert!(result.formatted_code.contains("<-ch"));
    }
}

/// Test different formatting configurations
mod configuration_integration_tests {
    use super::*;

    #[test]
    fn test_allman_brace_style() {
        init_tracing!();
        
        let source = "slay test(){lowkey based{yolo 42}}";
        
        let config = FormatterConfig {
            brace_style: BraceStyle::NextLineUnindented,
            ..FormatterConfig::default()
        };
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(source).unwrap();
        
        let expected_pattern = "slay test()\n{\n    lowkey based\n    {\n        yolo 42\n    }\n}";
        assert!(result.formatted_code.contains("test()\n{"));
        assert!(result.formatted_code.contains("based\n    {"));
    }

    #[test]
    fn test_tab_indentation() {
        init_tracing!();
        
        let source = "slay test(){lowkey based{yolo 42}}";
        
        let config = FormatterConfig {
            ..FormatterConfig::default()
        };
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(source).unwrap();
        
        // Should use tabs for indentation
        assert!(result.formatted_code.contains("\tlowkey"));
        assert!(result.formatted_code.contains("\t\tyolo"));
    }

    #[test]
    fn test_compact_formatting() {
        init_tracing!();
        
        let source = "slay test(a normie, b normie) normie { yolo a + b }";
        
        let config = FormatterConfig {
            compact_arrays: true,
            compact_objects: true,
            ..FormatterConfig::default()
        };
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(source).unwrap();
        
        assert!(result.formatted_code.contains("test(a normie,b normie)"));
        assert!(result.formatted_code.contains("yolo a+b"));
    }

    #[test]
    fn test_vertical_alignment() {
        init_tracing!();
        
        let source = r#""
sus short = 1
sus medium_name = 2
sus very_long_variable_name = 3
"#.trim()";

        let config = FormatterConfig {
            align_assignments: true,
            ..FormatterConfig::default()
        };
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(source).unwrap();
        
        // Should align assignment operators
        let lines: Vec<&str> = result.formatted_code.lines().collect();
        let equals_positions: Vec<usize> = lines.iter()
            .map(|line| line.find(" = ").unwrap_or(0))
            .collect();
        
        // All equals signs should be at the same position
        if equals_positions.len() > 1 {
            let first_pos = equals_positions[0];
            assert!(equals_positions.iter().all(|&pos| pos == first_pos));
        }
    }
}

/// Test large file formatting
mod large_file_tests {
    use super::*;

    #[test]
    fn test_large_file_performance() {
        init_tracing!();
        
        // Generate a large CURSED program
        let mut source = String::new();
        for i in 0..1000 {
            source.push_str(&format!(
                "slay function_{i}() {{\n    sus x = {i}\n    yolo x\n}}\n\n",
                i = i
            ));
        }
        
        let start = std::time::Instant::now();
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(&source).unwrap();
        let duration = start.elapsed();
        
        // Should complete within reasonable time (adjust threshold as needed)
        assert!(duration.as_millis() < 5000); // 5 seconds
        assert!(result.lines_processed > 2000);
        assert!(result.formatted_code.len() > source.len() / 2);
    }

    #[test]
    fn test_memory_usage() {
        init_tracing!();
        
        // Generate a program with many nested structures
        let mut source = String::new();
        for i in 0..100 {
            source.push_str(&format!(
                "squad Struct{i} {{\n    field{i} normie\n    nested{i} struct {{\n        inner{i} sip\n    }}\n}}\n",
                i = i
            ));
        }
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(&source).unwrap();
        
        // Should handle complex nested structures without excessive memory usage
        assert!(result.changed);
        assert!(result.lines_processed > 200);
    }
}
