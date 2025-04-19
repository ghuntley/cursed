//! Tests for optimized interface type assertions
//!
//! This module tests the optimized implementation of interface type assertions,
//! focusing on performance and error handling improvements.

use std::sync::Once;

// We need to call init_test_tracing only once
static INIT: Once = Once::new();

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing {
    () => {
        INIT.call_once(|| {
            tracing_setup::init_test_tracing();
        });
    };
}

// Import required test utilities
use cursed::core::{JitOptions, InterpretOptions};
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::object::{Object, ObjectRef};
use tracing::{debug, info};

// Helper function to run JIT tests on Cursed code
fn run_jit_test(input: &str) -> Result<ObjectRef, String> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;
    
    // Check for parser errors
    if !parser.errors().is_empty() {
        let error_msg = parser.errors().join("\n");
        return Err(format!("Parser errors:\n{}", error_msg));
    }
    
    // Run the program with default JIT options
    let options = JitOptions::default()
        .with_main_args(vec![]);
        
    let result = cursed::code::jit_compile_and_run(&program, options)?;
    Ok(result)
}

#[test]
fn test_optimized_type_assertion_with_caching() {
    init_tracing!();
    
    // Test type assertions with caching for multiple assertions of the same type
    let input = r#"
        // Define an interface
        collab Shape {
            area() normie;
        }
        
        // Define concrete types
        squad Circle {
            radius normie
        }
        
        squad Rectangle {
            width normie,
            height normie
        }
        
        // Implement interface methods
        slay (c Circle) area() normie {
            return 3.14159 * c.radius * c.radius
        }
        
        slay (r Rectangle) area() normie {
            return r.width * r.height
        }
        
        // Function to perform multiple assertions on the same interfaces
        slay testMultipleAssertions(shape Shape, iterations lit) lit {
            sus successCount lit = 0
            
            // Perform multiple assertions of the same type
            // This should benefit from caching of type IDs
            periodt i := 0; i < iterations; i = i + 1 {
                // Try circle assertion
                sus _, isCircle = shape.(Circle)
                if isCircle {
                    successCount = successCount + 1
                }
                
                // Try rectangle assertion
                sus _, isRect = shape.(Rectangle)
                if isRect {
                    successCount = successCount + 1
                }
            }
            
            return successCount
        }
        
        slay main() lit {
            // Create different shapes
            sus circle = Circle{radius: 2.0}
            sus rectangle = Rectangle{width: 3.0, height: 4.0}
            
            // Convert to interface
            sus shape1 Shape = circle
            sus shape2 Shape = rectangle
            
            // Test with circle - should succeed only for Circle assertions
            sus result1 = testMultipleAssertions(shape1, 5)
            
            // Test with rectangle - should succeed only for Rectangle assertions
            sus result2 = testMultipleAssertions(shape2, 5)
            
            // Return total successful assertions (should be 5 + 5 = 10)
            return result1 + result2
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            // Each shape should have 5 successful assertions
            assert_eq!(result.as_i64(), Some(10));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_type_assertion_error_handling() {
    init_tracing!();
    
    // Test enhanced error handling for type assertions
    let input = r#"
        // Define interface
        collab DataProcessor {
            process(data tea) tea;
        }
        
        // Define concrete types
        squad TextProcessor {
            name tea
        }
        
        squad BinaryProcessor {
            format tea
        }
        
        // Implement interface methods
        slay (tp TextProcessor) process(data tea) tea {
            return "Text processed by " + tp.name + ": " + data
        }
        
        slay (bp BinaryProcessor) process(data tea) tea {
            return "Binary processed in " + bp.format + ": " + data
        }
        
        // Function that handles assertion errors
        slay safeProcessWithErrorHandling(processor DataProcessor, data tea) tea {
            // Try text processor
            sus textProc, isText = processor.(TextProcessor)
            if isText {
                return textProc.process(data)
            }
            
            // Try binary processor
            sus binaryProc, isBinary = processor.(BinaryProcessor)
            if isBinary {
                return binaryProc.process(data)
            }
            
            // If neither assertion succeeded, return error message
            return "ERROR: Unknown processor type"
        }
        
        slay main() tea {
            // Create processors
            sus textProcessor = TextProcessor{name: "TextBot"}
            sus binaryProcessor = BinaryProcessor{format: "HEX"}
            
            // Create interface values
            sus proc1 DataProcessor = textProcessor
            sus proc2 DataProcessor = binaryProcessor
            
            // Process data with different processors
            sus result1 = safeProcessWithErrorHandling(proc1, "Hello")
            sus result2 = safeProcessWithErrorHandling(proc2, "World")
            
            // Try with a processor that doesn't implement DataProcessor
            // This should trigger the error handling
            sus unknownProcessor tea = "Not a processor"
            sus result3 = safeProcessWithErrorHandling(unknownProcessor, "Test")
            
            return result1 + " | " + result2 + " | " + result3
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            let output = result.as_string().unwrap();
            assert!(output.contains("Text processed by TextBot"));
            assert!(output.contains("Binary processed in HEX"));
            assert!(output.contains("ERROR: Unknown processor type"));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_compile_time_validation() {
    init_tracing!();
    
    // Test compile-time validation for type assertions
    let input = r#"
        // Define interfaces
        collab Reader {
            read(buf tea[]byte) lit;
        }
        
        collab Writer {
            write(data tea) lit;
        }
        
        // Define struct implementing Reader
        squad FileReader {
            path tea
        }
        
        // Define struct implementing Writer
        squad FileWriter {
            path tea
        }
        
        // Define struct implementing both interfaces
        squad FileReadWriter {
            path tea,
            mode tea
        }
        
        // Implement interfaces
        slay (fr FileReader) read(buf tea[]byte) lit {
            return 10  // Return bytes read
        }
        
        slay (fw FileWriter) write(data tea) lit {
            return data.length  // Return bytes written
        }
        
        slay (frw FileReadWriter) read(buf tea[]byte) lit {
            return 20  // Return bytes read
        }
        
        slay (frw FileReadWriter) write(data tea) lit {
            return data.length * 2  // Return bytes written
        }
        
        // Type-specific operation function
        slay operateOnReader(obj any) lit {
            // Reader assertion should succeed for FileReader and FileReadWriter
            sus reader, isReader = obj.(Reader)
            if isReader {
                sus buf = make(tea[]byte, 10)
                return reader.read(buf)
            }
            
            return 0
        }
        
        slay operateOnWriter(obj any) lit {
            // Writer assertion should succeed for FileWriter and FileReadWriter
            sus writer, isWriter = obj.(Writer)
            if isWriter {
                return writer.write("Hello")
            }
            
            return 0
        }
        
        slay main() lit {
            // Create objects
            sus reader = FileReader{path: "/tmp/test"}
            sus writer = FileWriter{path: "/tmp/output"}
            sus readWriter = FileReadWriter{path: "/tmp/both", mode: "rw"}
            
            // Test reader operations
            sus result1 = operateOnReader(reader)       // Should be 10
            sus result2 = operateOnReader(writer)       // Should be 0 (not a reader)
            sus result3 = operateOnReader(readWriter)   // Should be 20
            
            // Test writer operations
            sus result4 = operateOnWriter(reader)       // Should be 0 (not a writer)
            sus result5 = operateOnWriter(writer)       // Should be 5 ("Hello" length)
            sus result6 = operateOnWriter(readWriter)   // Should be 10 ("Hello" length * 2)
            
            return result1 + result2 + result3 + result4 + result5 + result6
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            // Total should be 10 + 0 + 20 + 0 + 5 + 10 = 45
            assert_eq!(result.as_i64(), Some(45));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_typeswitch_pattern() {
    init_tracing!();
    
    // Test using type assertions to implement a type switch pattern
    let input = r#"
        // Define interface
        collab Value {
            toString() tea;
        }
        
        // Define concrete types
        squad IntValue {
            value lit
        }
        
        squad FloatValue {
            value meal
        }
        
        squad StringValue {
            value tea
        }
        
        // Implement interface
        slay (iv IntValue) toString() tea {
            return "Int: " + vibe.toString(iv.value)
        }
        
        slay (fv FloatValue) toString() tea {
            return "Float: " + vibe.toString(fv.value)
        }
        
        slay (sv StringValue) toString() tea {
            return "String: " + sv.value
        }
        
        // Type switch pattern using type assertions
        slay processValue(val Value) tea {
            // Type assertions with specific handling for each type
            sus intVal, isInt = val.(IntValue)
            sus floatVal, isFloat = val.(FloatValue)
            sus stringVal, isString = val.(StringValue)
            
            if isInt {
                return "Got integer: " + vibe.toString(intVal.value)
            } elseif isFloat {
                return "Got float: " + vibe.toString(floatVal.value)
            } elseif isString {
                return "Got string: " + stringVal.value
            }
            
            return "Unknown value type"
        }
        
        slay main() tea {
            // Create different value types
            sus intVal = IntValue{value: 42}
            sus floatVal = FloatValue{value: 3.14}
            sus stringVal = StringValue{value: "hello"}
            
            // Convert to interface
            sus val1 Value = intVal
            sus val2 Value = floatVal
            sus val3 Value = stringVal
            
            // Process each value
            sus result1 = processValue(val1)
            sus result2 = processValue(val2)
            sus result3 = processValue(val3)
            
            return result1 + " | " + result2 + " | " + result3
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            let output = result.as_string().unwrap();
            assert!(output.contains("Got integer: 42"));
            assert!(output.contains("Got float: 3.14"));
            assert!(output.contains("Got string: hello"));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}