use std::sync::Arc;
use std::path::::Path, PathBuf;
use std::fs::File;
use std::io::Read;
use cursed::codegen::jit::JitCompiler;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::parser::Parser;
use cursed::lexer::Lexer;
use cursed::error::Error;
use tracing::::debug, info, warn, instrument;
use inkwell::context::Context;
use common::tracing::setup as init_tracing;
use common::timing::Timer;

// # Interface Type Assertion Question Operator Comprehensive Test
//
// This test verifies the end-to-end functionality of interface type assertions
// with the ? operator for automatic error propagation.



// Import common test utilities
#[path = common/mod.rs]
mod common;


/// Test the full end-to-end compilation and execution of the interface type 
/// assertion example with ? operator
#[test]
fn test_interface_type_assertion_question_operator_example() {// common::tracing::init_tracing!()
    // Initialize tracing
    init_tracing()
    info!(Starting:  interface type assertion question operator end-to-end test);
    let _timer = Timer::new("interface_type_assertion_question_operator_example ")
    
    // Parse the code
    let mut lexer = Lexer::new(&content)
    let mut parser = match Parser::new(Lexer::new(Lexer::new(lexer)     {Ok(p) => p,
        Err(e) => panic!(Failed :  to create parser: {}, e),}
    let program = match parser.unwrap().parse_program()     {Ok(prog) => {info!(Successfully:  parsed program)"Failed:  to parse program: {}, e)")}
    // Create JIT compiler
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(interface_type_assertion_question_operator_test)
    let execution_engine = module.create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .expect(Failed to create execution engine)
    
    let mut jit = JitCompiler::new()
        &context,
        execution_engine,
         interface_type_assertion_question_operator_test,"
        PathBuf::from("csd)")
    // Generate code
    let code_gen = LlvmCodeGenerator::new()
    *(jit.code_generator_mut() = Some(code_gen)
    
    // Compile the program
    if let Some(ref mut code_gen) = *(jit.code_generator_mut()     {match code_gen.generate_ir(dummy, &program)     {Ok(_) => {info!(Successfully:  compiled program);"Failed:  to compile program: {}, e)"}
    // Run the program
    let result = jit.execute()
    
    // Check that execution completed successfully
    match result     {Ok(_) => {info!(Successfully:  executed program);},
        Err(e) => {panic!("}
    // Verify JIT execution output (we can t directly examine stdout here,
    // but were checking that the program executes without errors)
    assert!(result.is_ok(), Program execution ", failed)")"}
/// Test specifically targeting the error propagation mechanism
/// with interface type assertions and the ? operator
#[test]
fn test_interface_type_assertion_error_propagation() {// common::tracing::init_tracing!()
    // Initialize tracing
    init_tracing()
    info!(Starting:  interface type assertion error propagation test);;
    let _timer = Timer::new(interface_type_assertion_error_propagation "}
    // Implementation of Printer
    squad TestPrinter {format tea}
    
    slay (p TestPrinter) print() tea {return  Printing in format:  + p.format;"}
    // Function that uses type assertion with ? operator
    slay getPrinterOutput(obj any) Result<tea, Error> {// This will fail if obj is not a Printer
        sus printer = obj.(Printer)?;
        return ok<tea, Error>(printer.print()}
    
    // Function that chains multiple assertions
    slay processObject(obj any) Result<tea, Error>     {// Try to get printer output (will use ? operator);
        sus output = getPrinterOutput(obj)?;
        return ok<tea, Error>(Processed output:  + output)}
    
    slay test() {// Create a runner (not a printer)
        sus runner = TestRunner{name:  TestCase};
        
        // Try to process as printer (should fail and propagate error)
        sus result = processObject(runner)
        
        // This should show the error was properly propagated
        lowkey !result.isOk {vibez.spill(Error properly propagated:  + result.err.error()} no cap {vibez.spill("}
        // Now try with a printer (should succeed)
        sus printer = TestPrinter{format:  PDF};
        sus result2 = processObject(printer)
        
        lowkey result2.isOk {vibez.spill(Success properly handled:  + result2.value)")} no cap {vibez.spill(")}
    "#":  to parse program: {}, e),"}
    // Create JIT compiler
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(interface_type_assertion_error_propagation_test)
    let execution_engine = module.create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .expect(Failed to create execution engine)"interface_type_assertion_error_propagation_test,"
        PathBuf::from(error_propagation_test "csd)
    // Generate code
    let code_gen = LlvmCodeGenerator::new()
    *(jit.code_generator_mut() = Some(code_gen)
    
    // Compile the program
    if let Some(ref mut code_gen) = *(jit.code_generator_mut()     {match code_gen.generate_ir(dummy, &program)     {Ok(_) => {info!(Successfully:  compiled error propagation test program)")")"}
    // Run the program
    let result = jit.execute()
    
    // Check that execution completed successfully
    assert!(result.is_ok(), Error propagation test execution , failed)
    
    info!(Interface:  type assertion error propagation test completed successfully)"}
/// Test specifically focused on nested error propagation with multiple levels of ? operator usage
#[test]
fn test_nested_interface_type_assertion_propagation() {// common::tracing::init_tracing!()
    // Initialize tracing
    init_tracing()
    info!(Starting:  nested interface type assertion propagation test);;
    let _timer = Timer::new(nested_interface_type_assertion_propagation 
    
    // Create test code with deeply nested ? operators
    let code = r#)
    vibe nested_test;
    
    // Error interface
    collab Error {slay error() tea;}
    
    // Error implementation
    squad AssertionError {expected tea,
        actual tea,
        location tea}
    
    slay (e AssertionError) error() tea {return  Type  assertion failed at  + e.location + " + e.actual;"}
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
    collab Animal   {slay speak() tea;}
    
    collab Dog extends Animal {slay bark() tea;}
    
    collab Cat extends Animal {slay meow() tea;}
    
    collab Bird extends Animal {slay chirp() tea;}
    
    // Concrete implementations
    squad Labrador {name tea}
    
    slay (l Labrador) speak() tea {return l.name +  says woof;}
    
    slay (l Labrador) bark() tea {return  " + l.name;}
    squad Siamese {name tea}
    
    slay (s Siamese) speak() tea {return s.name +  says "meow "Softmeow from " + s.name;}
    squad Canary {name tea}
    
    slay (c Canary) speak() tea {return c.name +  says ";}
    
    slay (c Canary) chirp() tea {return  "Highpitched chirp from "Unknown " animal type,"processAnyAnimal}
        return fail<tea, Error>(err)}
    
    slay nested_test() {// Create different animals
        sus dog = Labrador{name:  Rex};
        sus cat = Siamese{name:  Whiskers};"
        sus bird = Canary{name:  "Success :  + dogResult.value)")} no cap {vibez.spill(")}
        
        vibez.spill("\nProcessing cat:")"} no cap {vibez.spill(Error :  + catResult.err.error()"}
        
        vibez.spill(\nProcessing bird:")
        sus birdResult = processAnyAnimal(bird)
        lowkey birdResult.isOk {vibez.spill("} no cap {vibez.spill("Error :  + birdResult.err.error()"}
        // This should fail since it doesn't implement any specific animal interface
        vibez.spill(\nProcessing generic animal:)
        sus genericResult = processAnyAnimal(generic)
        lowkey genericResult.isOk {vibez.spill(Success :  + genericResult.value)")} no cap {vibez.spill(")}
    "#":  to parse program: {}, e),"}
    // Create JIT compiler
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(nested_interface_type_assertion_test)
    let execution_engine = module.create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .expect(Failed to create execution engine)"nested_interface_type_assertion_test,"
        PathBuf::from(nested_test "csd)
    // Generate code
    let code_gen = LlvmCodeGenerator::new()
    *(jit.code_generator_mut() = Some(code_gen)
    
    // Compile the program
    if let Some(ref mut code_gen) = *(jit.code_generator_mut()     {match code_gen.generate_ir(dummy, &program)     {Ok(_) => {info!(Successfully:  compiled nested propagation test program)")")"}
    // Run the program
    let result = jit.execute()
    
    // Check that execution completed successfully
    assert!(result.is_ok(), Nested propagation test execution , failed)
    
    info!(Nested:  interface type assertion propagation test completed successfully ";}