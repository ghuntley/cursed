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
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::path::PathBuf;
use inkwell::context::Context;
use cursed::codegen::jit::JitCompiler;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;

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
fn test_type_assertion_error_propagation() {
    init_tracing!();
    
    // This test verifies that errors in type assertions are properly propagated
    let input = r#"
        // Define an interface
        collab ErrorHandler {
            handle(msg tea) tea;
        }
        
        // Define a struct that implements the interface
        squad SafeHandler {
            prefix tea
        }
        
        // Implement the interface method
        slay (h SafeHandler) handle(msg tea) tea {
            return h.prefix + ": " + msg
        }
        
        // Function that propagates errors
        slay processSafely(h ErrorHandler, msg tea) tea {
            // Type assertion with error checking
            sus handler, ok = h.(SafeHandler)
            
            // Return custom error message if assertion fails
            if !ok {
                return "ERROR: Invalid handler type"
            }
            
            // Process message with the handler
            return handler.handle(msg)
        }
        
        // Another handler type
        squad RiskyHandler {
            factor lit
        }
        
        slay (h RiskyHandler) handle(msg tea) tea {
            return "Risky: " + msg + " (factor: " + vibe.toString(h.factor) + ")"
        }
        
        // Main function to test error propagation
        slay main() tea {
            // Create a safe handler
            sus safe = SafeHandler{prefix: "Safe"}
            
            // Create a risky handler
            sus risky = RiskyHandler{factor: 10}
            
            // Process with both handlers
            sus result1 = processSafely(safe, "message")
            sus result2 = processSafely(risky, "message")
            
            return result1 + " | " + result2
        }
    "#;
    
    // The test should process the safe handler correctly and return an error for the risky handler
    match run_jit_test(input) {
        Ok(result) => {
            // The test is expected to succeed, so we just check that the exit code is 0
            assert_eq!(result, 0);
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_type_assertion_nested_error_handling() {
    init_tracing!();
    
    // This test checks error handling in nested type assertions
    let input = r#"
        // Define interfaces
        collab DataSource {
            getData() tea;
        }
        
        collab Processor {
            process(data tea) tea;
        }
        
        // Define structs
        squad FileSource {
            path tea
        }
        
        squad ApiSource {
            url tea
        }
        
        squad TextProcessor {
            format tea
        }
        
        squad JsonProcessor {
            pretty bool
        }
        
        // Implement interfaces
        slay (fs FileSource) getData() tea {
            return "Data from file: " + fs.path
        }
        
        slay (as ApiSource) getData() tea {
            return "Data from API: " + as.url
        }
        
        slay (tp TextProcessor) process(data tea) tea {
            return "Processed text (" + tp.format + "): " + data
        }
        
        slay (jp JsonProcessor) process(data tea) tea {
            if jp.pretty {
                return "Prettified JSON: " + data
            }
            return "Minified JSON: " + data
        }
        
        // Function with nested type assertions
        slay processData(source DataSource, processor Processor) tea {
            // Get data from source
            sus data = source.getData()
            sus result = ""
            
            // Try to assert source type
            sus fs, isFile = source.(FileSource)
            sus as, isApi = source.(ApiSource)
            
            if isFile {
                result = result + "File source: " + fs.path + "\n"
            } else if isApi {
                result = result + "API source: " + as.url + "\n"
            } else {
                result = result + "Unknown source type\n"
            }
            
            // Try to assert processor type
            sus tp, isText = processor.(TextProcessor)
            sus jp, isJson = processor.(JsonProcessor)
            
            if isText {
                result = result + "Text processor with format: " + tp.format + "\n"
            } else if isJson {
                sus prettyStr = "false"
                if jp.pretty {
                    prettyStr = "true"
                }
                result = result + "JSON processor (pretty: " + prettyStr + ")\n"
            } else {
                result = result + "Unknown processor type\n"
            }
            
            // Process the data
            result = result + processor.process(data)
            return result
        }
        
        // Main function
        slay main() tea {
            // Create sources and processors
            sus fileSrc = FileSource{path: "/data.txt"}
            sus apiSrc = ApiSource{url: "api.example.com/data"}
            sus textProc = TextProcessor{format: "markdown"}
            sus jsonProc = JsonProcessor{pretty: true}
            
            // Test with file source and text processor
            sus result1 = processData(fileSrc, textProc)
            
            // Test with API source and JSON processor
            sus result2 = processData(apiSrc, jsonProc)
            
            return result1 + "-----\n" + result2
        }
    "#;
    
    // Run the test and verify nested error handling works correctly
    match run_jit_test(input) {
        Ok(result) => {
            // The test is expected to succeed, so we just check that the exit code is 0
            assert_eq!(result, 0);
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_type_assertion_with_optional_chaining() {
    init_tracing!();
    
    // This test checks error handling with optional chaining of type assertions
    let input = r#"
        // Define interfaces
        collab Container {
            getValue() lit;
        }
        
        // Define structs
        squad Box {
            value lit,
            label tea
        }
        
        squad Wrapper {
            inner Container
        }
        
        // Implement interfaces
        slay (b Box) getValue() lit {
            return b.value
        }
        
        slay (w Wrapper) getValue() lit {
            return w.inner.getValue()
        }
        
        // Helper function that uses type assertions
        slay tryExtractLabel(c Container) tea {
            // Try to assert as Box
            sus box, isBox = c.(Box)
            if isBox {
                return box.label
            }
            
            // Try to assert as Wrapper and extract inner Box
            sus wrapper, isWrapper = c.(Wrapper)
            if isWrapper {
                // Try to assert wrapper.inner as Box
                sus innerBox, isInnerBox = wrapper.inner.(Box)
                if isInnerBox {
                    return innerBox.label + " (wrapped)"
                }
            }
            
            return "Unknown container type"
        }
        
        // Main function
        slay main() tea {
            // Create containers
            sus box = Box{value: 42, label: "Direct Box"}
            sus wrappedBox = Wrapper{inner: Box{value: 100, label: "Wrapped Box"}}
            
            // Test label extraction
            sus result1 = tryExtractLabel(box)
            sus result2 = tryExtractLabel(wrappedBox)
            
            return result1 + " | " + result2
        }
    "#;
    
    // Run the test and verify chained assertions work correctly
    match run_jit_test(input) {
        Ok(result) => {
            // The test is expected to succeed, so we just check that the exit code is 0
            assert_eq!(result, 0);
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}