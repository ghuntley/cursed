use std::sync::Once;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::path::PathBuf;
use inkwell::context::Context;
use cursed::codegen::jit::JitCompiler;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;

// Test improved error propagation for interface type assertions
// This test verifies that the enhanced error propagation mechanism works correctly
// for interface type assertions, particularly for null interfaces and other error cases.


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

// Helper function to run JIT tests on Cursed code
fn run_jit_test(input: &str) -> Result<i32, String> {
    // Create a lexer
    let mut lexer = Lexer::new(input);
    // Create a parser with a mutable reference to the lexer
    let mut parser = Parser::new(&mut lexer).map_err(|e| e.to_string())?;
    // Parse the program
    let program = parser.parse_program().map_err(|e| e.to_string())?;
    
    // Check for parser errors
    if !parser.errors().is_empty() {
        let error_msg = parser.errors().iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n");
        return Err(format!("Parser errors:\n{}", error_msg));
    }
    
    // Create LLVM context and code generator
    let context = Context::create();
    let file_path = PathBuf::from("test_program.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "main", file_path.clone());
    
    // Compile the program
    code_gen.compile(&program).map_err(|e| e.to_string())?;
    
    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(inkwell::OptimizationLevel::Default)
        .map_err(|e| e.to_string())?;
    
    // Initialize the goroutine manager
    cursed::codegen::jit::init_goroutine_manager();
    
    // Create JIT compiler
    let mut jit_compiler = JitCompiler::new(&context, execution_engine, "_main_main", file_path.clone());
    
    // Use existing code_gen to avoid recompilation
    *jit_compiler.code_generator_mut() = Some(code_gen);
    
    // Execute the program
    let result = jit_compiler.execute().map_err(|e| e.to_string())?;
    
    // Wait for any goroutines to complete (10ms timeout)
    let _remaining = cursed::codegen::jit::wait_for_goroutines(10);
    
    Ok(result)
}

#[test]
fn test_null_interface_error_handling() {
    init_tracing!();
    
    // This test verifies that null interfaces are properly handled in type assertions
    let input = r#"
        // Define an interface
        collab Logger {
            log(message tea) void;
        }
        
        // Define a struct that implements the interface
        squad ConsoleLogger {
            prefix tea
        }
        
        // Implement the interface method
        slay (c ConsoleLogger) log(message tea) void {
            vibez.spill(c.prefix + ": " + message);
        }
        
        // Function that handles null loggers gracefully
        slay logSafely(logger Logger, message tea) bool {
            // Check if logger is null first
            if logger == nil {
                vibez.spill("Error: Logger is null");
                return false;
            }
            
            // Try type assertion with null check
            sus console, ok = logger.(ConsoleLogger);
            if !ok {
                vibez.spill("Error: Not a console logger");
                return false;
            }
            
            // Log the message
            console.log(message);
            return true;
        }
        
        // Main function to test null interface handling
        slay main() lit {
            // Create a valid logger
            sus validLogger Logger = ConsoleLogger{prefix: "INFO"};
            
            // Create a null logger
            sus nullLogger Logger = nil;
            
            // Test with both loggers
            sus result1 = logSafely(validLogger, "Test message 1");
            sus result2 = logSafely(nullLogger, "Test message 2");
            
            if result1 && !result2 {
                // Expected behavior: valid logger works, null logger fails
                return 0;
            } else {
                return 1;
            }
        }
    "#;
    
    // The test should handle the null logger gracefully and return success (0)
    match run_jit_test(input) {
        Ok(result) => {
            // The test is expected to succeed with exit code 0
            assert_eq!(result, 0, "Test should return 0 for correct null interface handling");
        },
        Err(e) => panic!("Failed to run null interface test: {}", e),
    }
}

#[test]
fn test_multi_level_error_propagation() {
    init_tracing!();
    
    // This test verifies that errors propagate through multiple levels of function calls
    let input = r#"
        // Define an interface
        collab Validator {
            validate(value tea) bool;
        }
        
        // Define a struct that implements the interface
        squad StringValidator {
            minLength lit
        }
        
        // Implement the interface method
        slay (v StringValidator) validate(value tea) bool {
            return vibe.length(value) >= v.minLength;
        }
        
        // Multiple levels of function calls with error handling
        slay validateLevel3(v Validator, value tea) tea {
            sus validator, ok = v.(StringValidator);
            if !ok {
                return "L3: Invalid validator type";
            }
            
            if !validator.validate(value) {
                return "L3: Validation failed: " + value;
            }
            
            return "L3: Valid: " + value;
        }
        
        slay validateLevel2(v Validator, value tea) tea {
            return "L2: " + validateLevel3(v, value);
        }
        
        slay validateLevel1(v Validator, value tea) tea {
            return "L1: " + validateLevel2(v, value);
        }
        
        // Main function to test multi-level error propagation
        slay main() tea {
            // Create validators
            sus validValidator Validator = StringValidator{minLength: 5};
            sus nullValidator Validator = nil;
            
            // Test with both validators
            sus result1 = validateLevel1(validValidator, "ValidString");
            sus result2 = validateLevel1(nullValidator, "Test");
            
            // The results should show the proper error propagation through all levels
            return result1 + " | " + result2;
        }
    "#;
    
    // The test should successfully compile and run, demonstrating proper error propagation
    match run_jit_test(input) {
        Ok(result) => {
            // The test is expected to succeed, so we just check that the exit code is 0
            assert_eq!(result, 0, "Test should return 0 for successful multi-level error propagation");
        },
        Err(e) => panic!("Failed to run multi-level error propagation test: {}", e),
    }
}

#[test]
fn test_complex_assertion_chain() {
    init_tracing!();
    
    // This test checks a complex chain of type assertions with error recovery
    let input = r#"
        // Define interfaces
        collab Reader {
            read() tea;
        }
        
        collab Writer {
            write(data tea) bool;
        }
        
        collab Closer {
            close() bool;
        }
        
        // Define structs that implement the interfaces
        squad FileReader {
            path tea
        }
        
        squad NetworkWriter {
            url tea,
            connected bool
        }
        
        squad ResourceCloser {
            name tea
        }
        
        // Implement the interface methods
        slay (fr FileReader) read() tea {
            return "Data from file: " + fr.path;
        }
        
        slay (fr FileReader) close() bool {
            // FileReader also implements Closer
            return true;
        }
        
        slay (nw NetworkWriter) write(data tea) bool {
            if !nw.connected {
                return false;
            }
            return true;
        }
        
        slay (rc ResourceCloser) close() bool {
            return true;
        }
        
        // Function that tries multiple type assertions with recovery
        slay processResource(obj interface{}) tea {
            sus result tea = "Process result: ";
            
            // Try as Reader
            sus reader, isReader = obj.(Reader);
            if isReader {
                result = result + "Read: " + reader.read() + ", ";
            } else {
                result = result + "Not a reader, ";
            }
            
            // Try as Writer
            sus writer, isWriter = obj.(Writer);
            if isWriter {
                sus writeResult = writer.write("test data");
                result = result + "Write: " + vibe.toString(writeResult) + ", ";
            } else {
                result = result + "Not a writer, ";
            }
            
            // Try as Closer
            sus closer, isCloser = obj.(Closer);
            if isCloser {
                sus closeResult = closer.close();
                result = result + "Close: " + vibe.toString(closeResult);
            } else {
                result = result + "Not a closer";
            }
            
            return result;
        }
        
        // Main function to test complex assertion chains
        slay main() tea {
            // Create objects of different types
            sus fileReader = FileReader{path: "/data.txt"};
            sus networkWriter = NetworkWriter{url: "api.example.com", connected: true};
            sus resourceCloser = ResourceCloser{name: "resource1"};
            
            // Process each resource and collect results
            sus result1 = processResource(fileReader);
            sus result2 = processResource(networkWriter);
            sus result3 = processResource(resourceCloser);
            
            return result1 + "\n" + result2 + "\n" + result3;
        }
    "#;
    
    // The test should successfully compile and run, demonstrating complex assertion chains
    match run_jit_test(input) {
        Ok(result) => {
            // The test is expected to succeed, so we just check that the exit code is 0
            assert_eq!(result, 0, "Test should return 0 for successful complex assertion chain");
        },
        Err(e) => panic!("Failed to run complex assertion chain test: {}", e),
    }
}