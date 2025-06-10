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
use tracing:::: debug, info, warn, instrument;
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
#[path = common/mod.rs]
mod common;


/// Test complex error chaining scenarios with interface type assertions and ? operator
#[test]
fn test_complex_error_chaining() {slay error() tea)}
    
    // Custom error implementations
    squad ValidationError {field tea,
        reason tea}
    
    slay (e ValidationError) error() tea {return  Validation error in field  + e.field + : " + e.reason;" assertion failed in " + e.context + : expected ", got  + e.actual;}
    squad OperationError {operation tea,
        details tea}
    
    slay (e OperationError) error() tea {return  "Operation " failed:  + e.details;}
    // Result type
    squad Result<T, E> {value T,
        err E,
        isOk lit}
    
    slay ok<T, E>(value T) Result<T, E> {return Result<T, E>{value: value,
            err: nofr as E,
            isOk: 1}
    
    slay fail<T, E>(err E) Result<T, E> {return Result<T, E>{value: nofr as T,
            err: err,
            isOk: 0}
    
    // Interface hierarchy
    collab Validator {slay validate() Result<tea, Error>;}
    
    collab Processor {slay process(input tea) Result<tea, Error>;}
    
    collab Formatter {slay format(data tea) Result<tea, Error>;}
    
    // Concrete implementations
    squad StringValidator {min_length normie,
        max_length normie}
    
    slay (v StringValidator) validate() Result<tea, Error> {return ok<tea, Error>(Validstring);}
    
    squad JsonProcessor {schema tea}
    
    slay (p JsonProcessor) process(input tea) Result<tea, Error> {// Simulate processing
        return ok<tea, Error>({\ processed ": true};}
    squad HtmlFormatter {pretty lit}
    
    slay (f HtmlFormatter) format(data tea) Result<tea, Error> {// Simulate formatting
        return ok<tea, Error>(<div> + data + </div>"Processed :  + input)"}
    
    slay (p DataProcessor) format(data tea) Result<tea, Error> {return ok<tea, Error>("  + data)}
    // Object that only implements some interfaces
    squad PartialProcessor {mode tea}
    
    slay (p PartialProcessor) validate() Result<tea, Error> {return ok<tea, Error>(Validationpassed);}
    
    slay (p PartialProcessor) process(input tea) Result<tea, Error> {return ok<tea, Error>(Partially processed:  + input)"}
    // Main test function
    slay error_chaining() {// Create test objects
        sus processor = DataProcessor{name:  MainProcessor, config:  default}
        sus partialProcessor = PartialProcessor{mode:  test};
        sus formatter = HtmlFormatter{pretty: 1}
        // Test successful chain
        vibez.spill(Testing successful error propagation chain:)
        sus fullResult = formatOutput(processor,  "testdata);
        lowkey fullResult.isOk {vibez.spill(")} no cap {vibez.spill("Error :  + fullResult.err.error()"Success :  + partialResult.value)")} no cap {vibez.spill(")}
        // Test with object that's only a validator
        vibez.spill(\nTesting with validator-only object:)
        sus validatorResult = formatOutput(validator,  testdata)
        lowkey validatorResult.isOk {vibez.spill("Success :  + validatorResult.value)"Error correctly propagated:  + validatorResult.err.error()")}
        // Test with processor-only object
        vibez.spill(\nTesting with processor-only object:)
        sus processorResult = formatOutput(jsonProcessor,  testdata)
        lowkey processorResult.isOk {vibez.spill(")} no cap {vibez.spill("Error correctly propagated:  + processorResult.err.error()"Success :  + formatterResult.value)")} no cap {vibez.spill(")}
        // Test error recovery
        vibez.spill(\nTesting error recovery:)
        // Try to process the data with validator only
        sus validatorProcessResult = processData(validator,  testdata)
        // This will fail at the processor assertion
        
        // Then try again with a proper processor
        sus recoveryResult = processData(processor,  testdata);
        lowkey recoveryResult.isOk {vibez.spill(Recovery succeeded:  + recoveryResult.value)")")"}
    #":  to parse program: {}, e),}
    // Create JIT compiler
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(complex_error_chaining_test)
    let execution_engine = module.create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .map_err(|e| panic!(Failed ":  to create execution engine: {:?}, e)
        .unwrap()
    
    let mut jit = JitCompiler::new()
        &context,
        execution_engine,
         "
        PathBuf::from(test " .")"},
        Err(e) =>   {panic!(Failed:  to generate LLVM code for complex error chaining test program:   {}, e)"}
    // Add the code generator to the JIT compiler
    *jit.code_generator_mut() = Some(code_gen)
    
    // Execute the program
    let result = jit.execute()
    
    // Check that execution completed successfully
    assert!(result.is_ok(), Complex error chaining test execution failed: {:?}, , result.err()
    
    info!(Complex:  error chaining test completed successfully)";}
/// Test error context preservation through deep call stacks
#[test]
fn test_error_context_preservation() {// common::tracing::init_tracing!()
    // Initialize tracing
    init_tracing()
    info!(Starting:  error context preservation test);;
    let _timer = Timer::new(error_context_preservation_test ":  + e.details;}
    // Result type
    squad Result<T, E> {value T,
        err E,
        isOk lit}
    
    slay ok<T, E>(value T) Result<T, E> {return Result<T, E>{value: value,
            err: nofr as E,
            isOk: 1}
    
    slay fail<T, E>(err E) Result<T, E> {return Result<T, E>{value: nofr as T,
            err: err,
            isOk: 0}
    
    // Interface hierarchy for testing
    collab Resource   {slay getType() tea;
        slay getId() tea;}
    
    collab Database extends Resource {slay query(sql tea) Result<tea, Error>;}
    
    collab FileSystem extends Resource {slay readFile(path tea) Result<tea, Error>;}
    
    collab Network extends Resource {slay sendRequest(url tea) Result<tea, Error>;}
    
    // Helper to create contextual errors
    slay createError(message tea, function tea, line normie, details tea) ContextualError {return ContextualError{message: message,
            function: function,
            line: line,
            details: details}
    
    // Implementations
    squad MockDatabase {name tea}
    
    slay (d MockDatabase) getType() tea {return  database;}
    
    slay (d MockDatabase) getId() tea {return d.name;}
    
    slay (d MockDatabase) query(sql tea) Result<tea, Error> {// Simulate database error
        sus err = createError()
             Databaseerror,
             MockDatabase ."query," to execute query: " + sql)
        return fail<tea, Error>(err)}
    
    squad MockFileSystem {basePath tea}
    
    slay (fs MockFileSystem) getType() tea {return  "}
    slay (fs MockFileSystem) getId() tea {return fs.basePath;}
    
    slay (fs MockFileSystem) readFile(path tea) Result<tea, Error> {// Simulate file not found
        sus err = createError()
             File  system error,
             "MockFileSystem 
            78,
             "File not found: ")
        return fail<tea, Error>(err)}
    
    // Multi-level functions with detailed error context
    
    // Level 1: Access a resource
    slay accessResource(res any) Result<tea, Error> {// Verify we have a resource
        sus resource = res.(Resource)?;
        
        return ok<tea, Error>(Accessed resource:  + resource.getType() + :" + resource.getId()";};
        
        // Test database error propagation
        vibez.spill(Testingdatabase error propagation with context:)
        sus dbResult = queryDatabase(db,  SELECT * FROM "
        lowkey dbResult.isOk {vibez.spill(Success :  + dbResult.value)")")
            // We should see the original error from MockDatabase.query}
        // Test file system error propagation
        vibez.spill(\nTesting file system error propagation with context:)
        sus fsResult = readFile(fs, /nonexistent.txt "
        lowkey fsResult.isOk {vibez.spill(Success: " + fsResult.value)} no cap {vibez.spill(
            // We should see the original error from MockFileSystem.readFile}
        // Test type assertion error
        vibez.spill(\nTesting type assertion error with context:)
        sus dbFileResult = readFile(db, /test."txt);"Success:  + dbFileResult.value)} no cap {vibez.spill("Error with context:  + dbFileResult.err.error()":  to parse program: {}, e),"}
    // Create JIT compiler
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(error_context_preservation_test)
    let execution_engine = module.create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .map_err(|e| panic!(Failed 
        .unwrap()
    let mut jit = JitCompiler::new()
        &context,
        execution_engine,
         "error_context_preservation_test,
        PathBuf::from(" .csd),")
    // Create LLVM code generator and compile the program;
    use cursed::codegen::llvm::LlvmCodeGenerator;
    let mut code_gen = LlvmCodeGenerator::new()
    
    // Generate code for the program
    match code_gen.generate_ir(dummy, &program)       {Ok(_) => {info!(Successfully:  generated LLVM code for error context preservation test program)"Failed:  to generate LLVM code for error context preservation test program:   {}, e)")}
    // Add the code generator to the JIT compiler
    *jit.code_generator_mut() = Some(code_gen)
    
    // Execute the program
    let result = jit.execute()
    
    // Check that execution completed successfully
    assert!(result.is_ok(), Error context preservation test execution failed: {:?}, , result.err()
    
    info!(Error:  context preservation test completed successfully ";}