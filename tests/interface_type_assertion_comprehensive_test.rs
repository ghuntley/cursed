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
fn test_interface_assertion_with_inheritance() {
    init_tracing!();
    
    // Test type assertion with interface inheritance
    let input = r#"
        // Define interfaces
        collab Animal {
            makeSound() tea;
        }
        
        collab Mammal {
            giveBirth() lit;
        }
        
        // Define a struct that implements both interfaces
        squad Dog {
            name tea,
            age lit
        }
        
        // Implement Animal for Dog
        slay (d Dog) makeSound() tea {
            return "Woof!"
        }
        
        // Implement Mammal for Dog
        slay (d Dog) giveBirth() lit {
            return d.age > 2 ? 4 : 0
        }
        
        // Main function to test interface assertion
        slay main() tea {
            // Create a Dog
            sus dog = Dog{name: "Rex", age: 3}
            
            // Assign to Animal interface
            sus animal Animal = dog
            
            // Assert from Animal to Mammal (should fail since they're different interfaces)
            sus mammal1, ok1 = animal.(Mammal)
            
            // Assert from Animal to Dog (should succeed)
            sus dog1, ok2 = animal.(Dog)
            
            // Create result string based on assertions
            sus result tea = ""
            if ok1 {
                result = result + "Animal to Mammal: OK\n"
            } else {
                result = result + "Animal to Mammal: Failed\n"
            }
            
            if ok2 {
                result = result + "Animal to Dog: OK\n"
                result = result + "Dog name: " + dog1.name
            } else {
                result = result + "Animal to Dog: Failed"
            }
            
            return result
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            let output = result.as_string().unwrap();
            assert!(output.contains("Animal to Mammal: Failed"));
            assert!(output.contains("Animal to Dog: OK"));
            assert!(output.contains("Dog name: Rex"));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_type_assertion_with_nested_types() {
    init_tracing!();
    
    // Test type assertion with nested types
    let input = r#"
        // Define interface
        collab Container {
            getSize() lit;
        }
        
        // Define structs
        squad Box {
            width lit,
            height lit,
            depth lit
        }
        
        squad Wrapper {
            box Box
        }
        
        // Implement Container for Box
        slay (b Box) getSize() lit {
            return b.width * b.height * b.depth
        }
        
        // Implement Container for Wrapper
        slay (w Wrapper) getSize() lit {
            return w.box.getSize()
        }
        
        // Main function to test nested type assertions
        slay main() lit {
            // Create a Box
            sus b = Box{width: 2, height: 3, depth: 4}
            
            // Create a Wrapper
            sus w = Wrapper{box: b}
            
            // Assign to Container interface
            sus c Container = w
            
            // Assert back to Wrapper
            sus wrapper, ok1 = c.(Wrapper)
            
            // Try to assert directly to Box (should fail)
            sus box, ok2 = c.(Box)
            
            if ok1 && !ok2 {
                // Access the nested Box through the Wrapper
                return wrapper.box.width * 100 + wrapper.box.height * 10 + wrapper.box.depth
            } else {
                return 0
            }
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            // The result should be 234 (2*100 + 3*10 + 4)
            assert_eq!(result.as_i64(), Some(234));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_type_assertion_with_error_recovery() {
    init_tracing!();
    
    // Test type assertion with error recovery
    let input = r#"
        // Define interfaces
        collab Processor {
            process(input tea) tea;
        }
        
        collab ErrorHandler {
            handleError(err tea) tea;
        }
        
        // Define structs
        squad StandardProcessor {
            name tea
        }
        
        squad FallbackProcessor {
            defaultResult tea
        }
        
        squad DefaultErrorHandler {
            prefix tea
        }
        
        // Implement interfaces
        slay (sp StandardProcessor) process(input tea) tea {
            // Simulated processing logic
            return "Processed by " + sp.name + ": " + input
        }
        
        slay (fp FallbackProcessor) process(input tea) tea {
            // Fallback processing logic
            return fp.defaultResult
        }
        
        slay (eh DefaultErrorHandler) handleError(err tea) tea {
            return eh.prefix + ": " + err
        }
        
        // Utility function to process with error handling
        slay safeProcess(processor Processor, input tea, handler ErrorHandler) tea {
            // Try different processor types
            sus stdProc, isStd = processor.(StandardProcessor)
            sus fallbackProc, isFallback = processor.(FallbackProcessor)
            
            if isStd {
                // Use standard processor
                return stdProc.process(input)
            } else if isFallback {
                // Use fallback processor
                return fallbackProc.process(input)
            } else {
                // Unknown processor type, use error handler
                sus errHandler, hasHandler = handler.(DefaultErrorHandler)
                if hasHandler {
                    return errHandler.handleError("Unknown processor type")
                } else {
                    return "Error: No error handler available"
                }
            }
        }
        
        // Main function to test error recovery
        slay main() tea {
            // Create processors and error handler
            sus stdProc = StandardProcessor{name: "MainProcessor"}
            sus fallbackProc = FallbackProcessor{defaultResult: "Fallback result"}
            sus errHandler = DefaultErrorHandler{prefix: "ERROR"}
            
            // Test with standard processor
            sus result1 = safeProcess(stdProc, "test input", errHandler)
            
            // Test with fallback processor
            sus result2 = safeProcess(fallbackProc, "test input", errHandler)
            
            // Test with error handler (passing something that's not a processor)
            sus result3 = safeProcess(errHandler, "test input", errHandler)
            
            return result1 + " | " + result2 + " | " + result3
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            let output = result.as_string().unwrap();
            assert!(output.contains("Processed by MainProcessor"));
            assert!(output.contains("Fallback result"));
            assert!(output.contains("ERROR: Unknown processor type"));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_type_assertion_in_chain() {
    init_tracing!();
    
    // Test type assertions in a processing chain
    let input = r#"
        // Define interfaces for a processing chain
        collab Transformer {
            transform(data tea) tea;
        }
        
        collab Validator {
            validate(data tea) oof;
        }
        
        collab Encoder {
            encode(data tea) tea[]byte;
        }
        
        // Define structs
        squad UppercaseTransformer {}
        squad LengthValidator { minLength lit }
        squad Base64Encoder {}
        
        // Implement interfaces
        slay (ut UppercaseTransformer) transform(data tea) tea {
            return vibe.toUpper(data)
        }
        
        slay (lv LengthValidator) validate(data tea) oof {
            return data.length >= lv.minLength
        }
        
        slay (be Base64Encoder) encode(data tea) tea[]byte {
            // Simplified base64 encoding (just a placeholder)
            sus bytes = make(tea[]byte, data.length)
            periodt i := 0; i < data.length; i = i + 1 {
                bytes[i] = byte(data[i])
            }
            return bytes
        }
        
        // Process function that uses type assertions to create a processing pipeline
        slay process(steps tea[]any, input tea) tea {
            sus result = input
            sus valid = true
            
            periodt i := 0; i < steps.length; i = i + 1 {
                sus step = steps[i]
                
                // Try to assert step as different processor types
                sus transformer, isTransformer = step.(Transformer)
                sus validator, isValidator = step.(Validator)
                sus encoder, isEncoder = step.(Encoder)
                
                if isTransformer {
                    // Apply transformation
                    result = transformer.transform(result)
                } else if isValidator {
                    // Validate the result
                    valid = validator.validate(result)
                    if !valid {
                        return "Validation failed"
                    }
                } else if isEncoder {
                    // Encode the result (simplified)
                    sus bytes = encoder.encode(result)
                    return "Encoded (" + vibe.toString(bytes.length) + " bytes)"
                } else {
                    return "Unknown step type"
                }
            }
            
            return result
        }
        
        // Main function to test processing chain
        slay main() tea {
            // Create processors
            sus transformer = UppercaseTransformer{}
            sus validator = LengthValidator{minLength: 5}
            sus encoder = Base64Encoder{}
            
            // Create processing steps
            sus steps = make(tea[]any, 3)
            steps[0] = transformer
            steps[1] = validator
            steps[2] = encoder
            
            // Test with valid input
            sus result1 = process(steps, "hello")
            
            // Test with invalid input (too short)
            sus result2 = process(steps, "hi")
            
            return result1 + " | " + result2
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            let output = result.as_string().unwrap();
            assert!(output.contains("Encoded"));
            assert!(output.contains("Validation failed"));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}