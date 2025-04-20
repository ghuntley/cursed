use std::sync::Once;

// We need to call init_test_tracing only once
static INIT: Once = Once::new();

#[path = "tracing_setup.rs"]
pub mod tracing_setup;
use tracing::{debug, info, error, instrument, warn, span, Level};

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
use cursed::object::Object;
// No JitOptions needed anymore
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
fn test_runtime_type_assertion_with_errors() {
    init_tracing!();
    info!("Running test_runtime_type_assertion_with_errors");
    
    // Test type assertion with comprehensive error handling
    let input = r#"
        // Define an error wrapper for type assertions
        // It contains the actual error message and context
        squad TypeError {
            message tea,
            typeName tea,
            actualType tea
        }
        
        // Define interfaces
        collab Shape {
            area() normie;
            name() tea;
        }
        
        collab Stringer {
            toString() tea;
        }
        
        // Define structs
        squad Circle {
            radius normie
        }
        
        squad Rectangle {
            width normie,
            height normie
        }
        
        squad Message {
            text tea
        }
        
        // Implement interfaces
        slay (c Circle) area() normie {
            return 3.14159 * c.radius * c.radius
        }
        
        slay (c Circle) name() tea {
            return "Circle"
        }
        
        slay (c Circle) toString() tea {
            return "Circle with radius: " + vibe.toString(c.radius)
        }
        
        slay (r Rectangle) area() normie {
            return r.width * r.height
        }
        
        slay (r Rectangle) name() tea {
            return "Rectangle"
        }
        
        slay (m Message) toString() tea {
            return m.text
        }
        
        // Utility function to perform type assertion with detailed error reporting
        slay assertType(value any, typeName tea) (any, tea) {
            // Perform different assertions based on the requested type
            if typeName == "Circle" {
                sus result, ok = value.(Circle)
                if !ok {
                    return result, "Cannot convert to Circle"
                }
                return result, ""
            } else if typeName == "Rectangle" {
                sus result, ok = value.(Rectangle)
                if !ok {
                    return result, "Cannot convert to Rectangle"
                }
                return result, ""
            } else if typeName == "Message" {
                sus result, ok = value.(Message)
                if !ok {
                    return result, "Cannot convert to Message"
                }
                return result, ""
            }
            
            return cap, "Unknown type: " + typeName
        }
        
        // Function that safely processes shapes
        slay processShape(shape Shape) tea {
            // Try to assert as Circle
            sus circle, circleErr = assertType(shape, "Circle")
            if circleErr == "" {
                // Successfully converted to Circle
                sus c = circle.(Circle)  // We know this is safe
                return "Processed circle with area: " + vibe.toString(c.area())
            }
            
            // Try to assert as Rectangle
            sus rect, rectErr = assertType(shape, "Rectangle")
            if rectErr == "" {
                // Successfully converted to Rectangle
                sus r = rect.(Rectangle)  // We know this is safe
                return "Processed rectangle with area: " + vibe.toString(r.area())
            }
            
            // Couldn't convert to any known shape type
            return "Unknown shape: " + shape.name()
        }
        
        // Main function to test runtime type assertions
        slay main() tea {
            // Create objects
            sus circle = Circle{radius: 5.0}
            sus rect = Rectangle{width: 4.0, height: 6.0}
            sus msg = Message{text: "Hello"}
            
            // Assign to interfaces
            sus shape1 Shape = circle
            sus shape2 Shape = rect
            sus stringer1 Stringer = circle  // Circle implements both interfaces
            sus stringer2 Stringer = msg
            
            // Process shapes
            sus result1 = processShape(shape1)
            sus result2 = processShape(shape2)
            
            // Try to process a Stringer as a Shape (should fail)
            sus result3 = ""
            lowkey stringer2.(Shape) {
                // This should never execute
                result3 = "ERROR: Should have failed!"
            } highkey {
                // This branch should execute
                result3 = "Correctly failed to convert Message to Shape"
            }
            
            // Cross-interface assertions
            sus shapeFromStringer, shapeOk = stringer1.(Shape)
            sus stringerFromShape, stringerOk = shape1.(Stringer)
            
            sus result4 = ""
            if shapeOk && stringerOk {
                result4 = "Successfully performed cross-interface assertions"
            } else {
                result4 = "Failed cross-interface assertions"
            }
            
            return result1 + "\n" + result2 + "\n" + result3 + "\n" + result4
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            // Success is indicated by exit code 0
            assert_eq!(result, 0);
            info!("Test passed with exit code 0");
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_runtime_type_assertion_complex_chains() {
    init_tracing!();
    info!("Running test_runtime_type_assertion_complex_chains");
    
    // Test type assertion with complex chaining and error handling
    let input = r#"
        // Define interfaces
        collab Data {
            getData() tea;
        }
        
        collab Processor {
            process(data tea) tea;
        }
        
        collab Container {
            unwrap() any;
        }
        
        // Define structs
        squad TextData {
            content tea
        }
        
        squad JsonData {
            json tea
        }
        
        squad TextProcessor {
            prefix tea
        }
        
        squad JsonProcessor {
            indent lit
        }
        
        squad Box {
            value any
        }
        
        squad Wrapper {
            inner Container
        }
        
        // Implement interfaces
        slay (td TextData) getData() tea {
            return td.content
        }
        
        slay (jd JsonData) getData() tea {
            return jd.json
        }
        
        slay (tp TextProcessor) process(data tea) tea {
            return tp.prefix + ": " + data
        }
        
        slay (jp JsonProcessor) process(data tea) tea {
            return "JSON(" + vibe.toString(jp.indent) + "): " + data
        }
        
        slay (b Box) unwrap() any {
            return b.value
        }
        
        slay (w Wrapper) unwrap() any {
            return w.inner.unwrap()
        }
        
        // Utility function for safe type assertion
        slay tryAs(value any, typeName tea) (any, oof) {
            if typeName == "TextData" {
                sus result, ok = value.(TextData)
                return result, ok
            } else if typeName == "JsonData" {
                sus result, ok = value.(JsonData)
                return result, ok
            } else if typeName == "TextProcessor" {
                sus result, ok = value.(TextProcessor)
                return result, ok
            } else if typeName == "JsonProcessor" {
                sus result, ok = value.(JsonProcessor)
                return result, ok
            } else if typeName == "Box" {
                sus result, ok = value.(Box)
                return result, ok
            } else if typeName == "Wrapper" {
                sus result, ok = value.(Wrapper)
                return result, ok
            }
            
            return cap, false
        }
        
        // Function to process data with complex type assertion chains
        slay processComplex(container Container, processor Processor) tea {
            // First unwrap the container
            sus contents = container.unwrap()
            
            // Try different paths based on content type
            sus result tea = ""
            
            // Try contents as TextData
            sus textData, isTextData = tryAs(contents, "TextData")
            
            if isTextData {
                // Process as TextData
                sus td = textData.(TextData)  // Safe due to previous check
                sus data = td.getData()
                
                // Process with the provided processor
                result = processor.process(data)
                return result + " (from TextData)"
            }
            
            // Try contents as JsonData
            sus jsonData, isJsonData = tryAs(contents, "JsonData")
            
            if isJsonData {
                // Process as JsonData
                sus jd = jsonData.(JsonData)  // Safe due to previous check
                sus data = jd.getData()
                
                // Process with the provided processor
                result = processor.process(data)
                return result + " (from JsonData)"
            }
            
            // Try contents as nested Box
            sus box, isBox = tryAs(contents, "Box")
            
            if isBox {
                // Recursively process the box contents
                sus b = box.(Box)  // Safe due to previous check
                sus innerBox = Box{value: b.unwrap()}
                return processComplex(innerBox, processor) + " (boxed)"
            }
            
            return "Unknown data type"
        }
        
        // Main function to test complex type assertion chains
        slay main() tea {
            // Create test data
            sus textData = TextData{content: "Hello world"}
            sus jsonData = JsonData{json: "{\"message\":\"Hello\"}"}
            
            // Create processors
            sus textProcessor = TextProcessor{prefix: "TEXT"}
            sus jsonProcessor = JsonProcessor{indent: 2}
            
            // Create containers
            sus box1 = Box{value: textData}
            sus box2 = Box{value: jsonData}
            sus box3 = Box{value: Box{value: textData}}  // Nested box
            
            // Test with different combinations
            sus result1 = processComplex(box1, textProcessor)
            sus result2 = processComplex(box2, jsonProcessor)
            sus result3 = processComplex(box3, textProcessor)
            
            // Test with a wrapper
            sus wrapper = Wrapper{inner: box1}
            sus result4 = processComplex(wrapper, jsonProcessor)
            
            return result1 + "\n" + result2 + "\n" + result3 + "\n" + result4
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            // Success is indicated by exit code 0
            assert_eq!(result, 0);
            info!("Test passed with exit code 0");
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_runtime_type_assertion_errors() {
    init_tracing!();
    info!("Running test_runtime_type_assertion_errors");
    
    // Test detailed error reporting for type assertions
    let input = r#"
        // Define error handler for type assertions
        squad TypeErrorHandler {
            errors tea[]tea
        }
        
        // Define interfaces
        collab Converter {
            convert(input any) (any, tea);
        }
        
        // Define structs
        squad StringConverter {
            error_count lit
        }
        
        squad IntConverter {
            error_count lit
        }
        
        squad FloatConverter {
            error_count lit
        }
        
        // Implement interfaces
        slay (sc StringConverter) convert(input any) (any, tea) {
            // Try to convert to string
            sus str, ok = input.(tea)
            if !ok {
                sc.error_count = sc.error_count + 1
                return "", "Value is not a string"
            }
            return str, ""
        }
        
        slay (ic IntConverter) convert(input any) (any, tea) {
            // Try to convert to int
            sus intVal, ok = input.(lit)
            if !ok {
                ic.error_count = ic.error_count + 1
                return 0, "Value is not an integer"
            }
            return intVal, ""
        }
        
        slay (fc FloatConverter) convert(input any) (any, tea) {
            // Try to convert to float
            sus floatVal, ok = input.(meal)
            if !ok {
                fc.error_count = fc.error_count + 1
                return 0.0, "Value is not a float"
            }
            return floatVal, ""
        }
        
        // Error handling function for type assertions
        slay tryConvert(input any, handler TypeErrorHandler, converterName tea) (any, tea) {
            if converterName == "string" {
                sus converter = StringConverter{error_count: 0}
                sus result, err = converter.convert(input)
                if err != "" {
                    // Add error to handler
                    handler.errors = append(handler.errors, "String conversion error: " + err)
                    return "", err
                }
                return result, ""
            } else if converterName == "int" {
                sus converter = IntConverter{error_count: 0}
                sus result, err = converter.convert(input)
                if err != "" {
                    // Add error to handler
                    handler.errors = append(handler.errors, "Integer conversion error: " + err)
                    return 0, err
                }
                return result, ""
            } else if converterName == "float" {
                sus converter = FloatConverter{error_count: 0}
                sus result, err = converter.convert(input)
                if err != "" {
                    // Add error to handler
                    handler.errors = append(handler.errors, "Float conversion error: " + err)
                    return 0.0, err
                }
                return result, ""
            }
            
            handler.errors = append(handler.errors, "Unknown converter: " + converterName)
            return cap, "Unknown converter"
        }
        
        // Main function to test error handling
        slay main() tea {
            // Create an error handler
            sus handler = TypeErrorHandler{errors: make(tea[]tea, 0)}
            
            // Test values
            sus strValue tea = "Hello"
            sus intValue lit = 42
            sus floatValue meal = 3.14159
            
            // Test conversions that should work
            sus str1, strErr1 = tryConvert(strValue, handler, "string")
            sus int1, intErr1 = tryConvert(intValue, handler, "int")
            sus float1, floatErr1 = tryConvert(floatValue, handler, "float")
            
            // Test conversions that should fail with errors
            sus int2, intErr2 = tryConvert(strValue, handler, "int")
            sus float2, floatErr2 = tryConvert(strValue, handler, "float")
            sus str2, strErr2 = tryConvert(intValue, handler, "string")
            
            // Also test with unknown converter
            sus unknown, unknownErr = tryConvert(strValue, handler, "binary")
            
            // Build report based on accumulated errors
            sus report tea = "Conversion tests completed.\n"
            report = report + "Successful conversions: " + vibe.toString(3 - handler.errors.length) + "/7\n"
            report = report + "Errors: " + vibe.toString(handler.errors.length) + "\n"
            
            // List all errors
            if handler.errors.length > 0 {
                report = report + "Error details:\n"
                periodt i := 0; i < handler.errors.length; i = i + 1 {
                    report = report + "- " + handler.errors[i] + "\n"
                }
            }
            
            return report
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            // Success is indicated by exit code 0
            assert_eq!(result, 0);
            info!("Test passed with exit code 0");
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}