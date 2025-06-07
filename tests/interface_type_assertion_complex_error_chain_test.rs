use std::sync::Arc;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use cursed::codegen::jit::JitCompiler;
use cursed::parser::Parser;
use cursed::lexer::Lexer;
use cursed::error::Error;
use inkwell::context::Context;
use std::path::PathBuf;
use tracing::{debug, info, warn, instrument};
use common::tracing::setup as init_tracing;
use common::timing::Timer;

// # Complex Interface Type Assertion Error Chaining Test
//
// This test verifies the robustness of interface type assertions with the ? operator
// in complex error propagation scenarios, including:
// - Deep call stacks with multiple ? operators
// - Error context preservation through propagation
// - Error recovery and handling at different levels
// - Type hierarchy navigation during assertions



// Import common test utilities
#[path = "common.rs"]
pub mod common;


/// Test complex error chaining scenarios with interface type assertions and ? operator
#[test]
fn test_complex_error_chaining() {
    // Initialize tracing
    init_tracing();
    info!("Starting complex error chaining test");
    let _timer = Timer::new("complex_error_chaining_test");
    
    // Create test code with complex error chaining scenarios
    let code = r#"
    vibe error_chaining;
    
    // Error interface
    collab Error {
        slay error() tea;
    }
    
    // Custom error implementations
    squad ValidationError {
        field tea,
        reason tea
    }
    
    slay (e ValidationError) error() tea {
        return "Validation error in field '" + e.field + "': " + e.reason;
    }
    
    squad TypeAssertionError {
        expected tea,
        actual tea,
        context tea
    }
    
    slay (e TypeAssertionError) error() tea {
        return "Type assertion failed in " + e.context + ": expected " + e.expected + ", got " + e.actual;
    }
    
    squad OperationError {
        operation tea,
        details tea
    }
    
    slay (e OperationError) error() tea {
        return "Operation '" + e.operation + "' failed: " + e.details;
    }
    
    // Result type
    squad Result<T, E> {
        value T,
        err E,
        isOk lit
    }
    
    slay ok<T, E>(value T) Result<T, E> {
        return Result<T, E>{
            value: value,
            err: nofr as E,
            isOk: 1
        };
    }
    
    slay fail<T, E>(err E) Result<T, E> {
        return Result<T, E>{
            value: nofr as T,
            err: err,
            isOk: 0
        };
    }
    
    // Interface hierarchy
    collab Validator {
        slay validate() Result<tea, Error>;
    }
    
    collab Processor {
        slay process(input tea) Result<tea, Error>;
    }
    
    collab Formatter {
        slay format(data tea) Result<tea, Error>;
    }
    
    // Concrete implementations
    squad StringValidator {
        min_length normie,
        max_length normie
    }
    
    slay (v StringValidator) validate() Result<tea, Error> {
        return ok<tea, Error>("Valid string");
    }
    
    squad JsonProcessor {
        schema tea
    }
    
    slay (p JsonProcessor) process(input tea) Result<tea, Error> {
        // Simulate processing
        return ok<tea, Error>("{\"processed\": true}");
    }
    
    squad HtmlFormatter {
        pretty lit
    }
    
    slay (f HtmlFormatter) format(data tea) Result<tea, Error> {
        // Simulate formatting
        return ok<tea, Error>("<div>" + data + "</div>");
    }
    
    // Generic data object
    squad DataObject {
        type tea,
        value tea,
        metadata tea
    }
    
    // Multi-level functions with error propagation
    
    // Level 1: Validate input
    slay validateInput(obj any) Result<tea, Error> {
        // Try to assert the object as a Validator
        sus validator = obj.(Validator)?;
        
        // Call validate and propagate any errors
        sus validationResult = validator.validate()?;
        
        return ok<tea, Error>(validationResult);
    }
    
    // Level 2: Process data
    slay processData(obj any, input tea) Result<tea, Error> {
        // First validate the input
        sus validationResult = validateInput(obj)?;
        
        // Now try to process the data
        sus processor = obj.(Processor)?;
        sus processingResult = processor.process(input)?;
        
        return ok<tea, Error>(processingResult);
    }
    
    // Level 3: Format output
    slay formatOutput(obj any, input tea) Result<tea, Error> {
        // First process the data
        sus processingResult = processData(obj, input)?;
        
        // Now try to format the result
        sus formatter = obj.(Formatter)?;
        sus formattingResult = formatter.format(processingResult)?;
        
        return ok<tea, Error>(formattingResult);
    }
    
    // Complex object that implements multiple interfaces
    squad DataProcessor {
        name tea,
        config tea
    }
    
    slay (p DataProcessor) validate() Result<tea, Error> {
        return ok<tea, Error>("Data processor validation passed");
    }
    
    slay (p DataProcessor) process(input tea) Result<tea, Error> {
        return ok<tea, Error>("Processed: " + input);
    }
    
    slay (p DataProcessor) format(data tea) Result<tea, Error> {
        return ok<tea, Error>("[" + p.name + "] " + data);
    }
    
    // Object that only implements some interfaces
    squad PartialProcessor {
        mode tea
    }
    
    slay (p PartialProcessor) validate() Result<tea, Error> {
        return ok<tea, Error>("Validation passed");
    }
    
    slay (p PartialProcessor) process(input tea) Result<tea, Error> {
        return ok<tea, Error>("Partially processed: " + input);
    }
    
    // Main test function
    slay error_chaining() {
        // Create test objects
        sus processor = DataProcessor{name: "MainProcessor", config: "default"};
        sus partialProcessor = PartialProcessor{mode: "test"};
        sus validator = StringValidator{min_length: 3, max_length: 100};
        sus jsonProcessor = JsonProcessor{schema: "v1"};
        sus formatter = HtmlFormatter{pretty: 1};
        
        // Test successful chain
        vibez.spill("Testing successful error propagation chain:");
        sus fullResult = formatOutput(processor, "test data");
        lowkey fullResult.isOk {
            vibez.spill("Success: " + fullResult.value);
        } no cap {
            vibez.spill("Error: " + fullResult.err.error();
        }
        
        // Test chain breaking at formatter level
        vibez.spill("\nTesting error at formatter level:");
        sus partialResult = formatOutput(partialProcessor, "test data");
        lowkey partialResult.isOk {
            vibez.spill("Success: " + partialResult.value);
        } no cap {
            vibez.spill("Error correctly propagated: " + partialResult.err.error();
        }
        
        // Test with object that's only a validator
        vibez.spill("\nTesting with validator-only object:");
        sus validatorResult = formatOutput(validator, "test data");
        lowkey validatorResult.isOk {
            vibez.spill("Success: " + validatorResult.value);
        } no cap {
            vibez.spill("Error correctly propagated: " + validatorResult.err.error();
        }
        
        // Test with processor-only object
        vibez.spill("\nTesting with processor-only object:");
        sus processorResult = formatOutput(jsonProcessor, "test data");
        lowkey processorResult.isOk {
            vibez.spill("Success: " + processorResult.value);
        } no cap {
            vibez.spill("Error correctly propagated: " + processorResult.err.error();
        }
        
        // Test with formatter-only object
        vibez.spill("\nTesting with formatter-only object:");
        sus formatterResult = formatOutput(formatter, "test data");
        lowkey formatterResult.isOk {
            vibez.spill("Success: " + formatterResult.value);
        } no cap {
            vibez.spill("Error correctly propagated: " + formatterResult.err.error();
        }
        
        // Test error recovery
        vibez.spill("\nTesting error recovery:");
        
        // Try to process the data with validator only
        sus validatorProcessResult = processData(validator, "test data");
        // This will fail at the processor assertion
        
        // Then try again with a proper processor
        sus recoveryResult = processData(processor, "test data");
        lowkey recoveryResult.isOk {
            vibez.spill("Recovery succeeded: " + recoveryResult.value);
        } no cap {
            vibez.spill("Recovery failed: " + recoveryResult.err.error();
        }
    }
    "#;
    
    // Parse the code
    let mut lexer = Lexer::new(code);
    let mut parser = match Parser::new(&mut lexer) {
        Ok(p) => p,
        Err(e) => panic!("Failed to create parser: {}", e),
    };
    let program = match parser.parse_program() {
        Ok(prog) => prog,
        Err(e) => panic!("Failed to parse program: {}", e),
    };
    
    // Create JIT compiler
    let context = Context::create();
    let module = context.create_module("complex_error_chaining_test");
    let execution_engine = module.create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .map_err(|e| panic!("Failed to create execution engine: {:?}", e))
        .unwrap();
    
    let mut jit = JitCompiler::new(
        &context,
        execution_engine,
        "complex_error_chaining_test",
        PathBuf::from("test.csd"),
    );
    
    // Create LLVM code generator and compile the program
    use cursed::codegen::llvm::LlvmCodeGenerator;
    let mut code_gen = LlvmCodeGenerator::new(&context, "complex_error_chaining_test", &PathBuf::from("test.csd"));
    
    // Generate code for the program
    match code_gen.generate(&program) {
        Ok(_) => {
            info!("Successfully generated LLVM code for complex error chaining test program");
        },
        Err(e) => {
            panic!("Failed to generate LLVM code for complex error chaining test program: {}", e);
        }
    };
    
    // Add the code generator to the JIT compiler
    *jit.code_generator_mut() = Some(code_gen);
    
    // Execute the program
    let result = jit.execute();
    
    // Check that execution completed successfully
    assert!(result.is_ok(), "Complex error chaining test execution failed: {:?}", result.err());
    
    info!("Complex error chaining test completed successfully");
}

/// Test error context preservation through deep call stacks
#[test]
fn test_error_context_preservation() {
    // Initialize tracing
    init_tracing();
    info!("Starting error context preservation test");
    let _timer = Timer::new("error_context_preservation_test");
    
    // Create test code that verifies error context is preserved through deep call stacks
    let code = r#"
    vibe context_preservation;
    
    // Error interface
    collab Error {
        slay error() tea;
    }
    
    // Detailed error with context information
    squad ContextualError {
        message tea,
        function tea,
        line normie,
        details tea
    }
    
    slay (e ContextualError) error() tea {
        return e.message + " in " + e.function + " (line " + e.line + "): " + e.details;
    }
    
    // Result type
    squad Result<T, E> {
        value T,
        err E,
        isOk lit
    }
    
    slay ok<T, E>(value T) Result<T, E> {
        return Result<T, E>{
            value: value,
            err: nofr as E,
            isOk: 1
        };
    }
    
    slay fail<T, E>(err E) Result<T, E> {
        return Result<T, E>{
            value: nofr as T,
            err: err,
            isOk: 0
        };
    }
    
    // Interface hierarchy for testing
    collab Resource {
        slay getType() tea;
        slay getId() tea;
    }
    
    collab Database extends Resource {
        slay query(sql tea) Result<tea, Error>;
    }
    
    collab FileSystem extends Resource {
        slay readFile(path tea) Result<tea, Error>;
    }
    
    collab Network extends Resource {
        slay sendRequest(url tea) Result<tea, Error>;
    }
    
    // Helper to create contextual errors
    slay createError(message tea, function tea, line normie, details tea) ContextualError {
        return ContextualError{
            message: message,
            function: function,
            line: line,
            details: details
        };
    }
    
    // Implementations
    squad MockDatabase {
        name tea
    }
    
    slay (d MockDatabase) getType() tea {
        return "database";
    }
    
    slay (d MockDatabase) getId() tea {
        return d.name;
    }
    
    slay (d MockDatabase) query(sql tea) Result<tea, Error> {
        // Simulate database error
        sus err = createError(
            "Database error",
            "MockDatabase.query",
            42,
            "Failed to execute query: " + sql
        );
        return fail<tea, Error>(err);
    }
    
    squad MockFileSystem {
        basePath tea
    }
    
    slay (fs MockFileSystem) getType() tea {
        return "filesystem";
    }
    
    slay (fs MockFileSystem) getId() tea {
        return fs.basePath;
    }
    
    slay (fs MockFileSystem) readFile(path tea) Result<tea, Error> {
        // Simulate file not found
        sus err = createError(
            "File system error",
            "MockFileSystem.readFile",
            78,
            "File not found: " + path
        );
        return fail<tea, Error>(err);
    }
    
    // Multi-level functions with detailed error context
    
    // Level 1: Access a resource
    slay accessResource(res any) Result<tea, Error> {
        // Verify we have a resource
        sus resource = res.(Resource)?;
        
        return ok<tea, Error>("Accessed resource: " + resource.getType() + ":" + resource.getId();
    }
    
    // Level 2: Query database
    slay queryDatabase(res any, query tea) Result<tea, Error> {
        // First access the resource
        sus resourceInfo = accessResource(res)?;
        
        // Try to query the database
        sus db = res.(Database)?;
        sus queryResult = db.query(query)?;
        
        return ok<tea, Error>(queryResult);
    }
    
    // Level 3: Read file
    slay readFile(res any, path tea) Result<tea, Error> {
        // First access the resource
        sus resourceInfo = accessResource(res)?;
        
        // Try to read the file
        sus fs = res.(FileSystem)?;
        sus fileContent = fs.readFile(path)?;
        
        return ok<tea, Error>(fileContent);
    }
    
    // Main test function
    slay context_preservation() {
        // Create test resources
        sus db = MockDatabase{name: "test_db"};
        sus fs = MockFileSystem{basePath: "/tmp"};
        
        // Test database error propagation
        vibez.spill("Testing database error propagation with context:");
        sus dbResult = queryDatabase(db, "SELECT * FROM table");
        lowkey dbResult.isOk {
            vibez.spill("Success: " + dbResult.value);
        } no cap {
            vibez.spill("Error with context: " + dbResult.err.error();
            // We should see the original error from MockDatabase.query
        }
        
        // Test file system error propagation
        vibez.spill("\nTesting file system error propagation with context:");
        sus fsResult = readFile(fs, "/nonexistent.txt");
        lowkey fsResult.isOk {
            vibez.spill("Success: " + fsResult.value);
        } no cap {
            vibez.spill("Error with context: " + fsResult.err.error();
            // We should see the original error from MockFileSystem.readFile
        }
        
        // Test type assertion error
        vibez.spill("\nTesting type assertion error with context:");
        sus dbFileResult = readFile(db, "/test.txt");
        lowkey dbFileResult.isOk {
            vibez.spill("Success: " + dbFileResult.value);
        } no cap {
            vibez.spill("Error with context: " + dbFileResult.err.error();
            // We should see an error about db not implementing FileSystem
        }
    }
    "#;
    
    // Parse the code
    let mut lexer = Lexer::new(code);
    let mut parser = match Parser::new(&mut lexer) {
        Ok(p) => p,
        Err(e) => panic!("Failed to create parser: {}", e),
    };
    let program = match parser.parse_program() {
        Ok(prog) => prog,
        Err(e) => panic!("Failed to parse program: {}", e),
    };
    
    // Create JIT compiler
    let context = Context::create();
    let module = context.create_module("error_context_preservation_test");
    let execution_engine = module.create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .map_err(|e| panic!("Failed to create execution engine: {:?}", e))
        .unwrap();
    
    let mut jit = JitCompiler::new(
        &context,
        execution_engine,
        "error_context_preservation_test",
        PathBuf::from("test.csd"),
    );
    
    // Create LLVM code generator and compile the program
    use cursed::codegen::llvm::LlvmCodeGenerator;
    let mut code_gen = LlvmCodeGenerator::new(&context, "error_context_preservation_test", &PathBuf::from("test.csd"));
    
    // Generate code for the program
    match code_gen.generate(&program) {
        Ok(_) => {
            info!("Successfully generated LLVM code for error context preservation test program");
        },
        Err(e) => {
            panic!("Failed to generate LLVM code for error context preservation test program: {}", e);
        }
    };
    
    // Add the code generator to the JIT compiler
    *jit.code_generator_mut() = Some(code_gen);
    
    // Execute the program
    let result = jit.execute();
    
    // Check that execution completed successfully
    assert!(result.is_ok(), "Error context preservation test execution failed: {:?}", result.err());
    
    info!("Error context preservation test completed successfully");
}