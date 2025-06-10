/// Function compilation tests for CURSED LLVM code generation
/// 
/// These tests are essential because function compilation is the core of any programming language.
/// They verify:
/// 1. Parameter passing mechanisms work correctly
/// 2. Return value handling preserves types and values
/// 3. Recursion support enables complex algorithms
/// 4. Memory management during function execution prevents leaks
/// 5. Local variable scoping works correctly
/// 6. Function calls can be properly linked and executed
/// 7. Gen Z slang syntax (slay, yolo) generates proper LLVM IR
/// 8. Calling conventions are compatible with the LLVM runtime

use cursed::codegen::llvm::{LlvmCodeGenerator, FunctionCompilation};
use cursed::ast::declarations::FunctionStatement;
use cursed::ast::calls::CallExpression;
use cursed::ast::statements::ReturnStatement;
use cursed::ast::identifiers::Identifier;
use cursed::ast::expressions::Parameter;
use cursed::ast::literals::{IntegerLiteral, StringLiteral};
use cursed::ast::block::BlockStatement;
use cursed::ast::traits::{Node, Expression};
use std::collections::HashMap;

#[path = "common.rs]
mod common;

#[test]
fn test_simple_function_declaration() {
    common::tracing::setup()
    
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // slay main() { }
    let func = FunctionStatement::new()
         "slay ".to_string()
        Identifier::new( main.to_string(),  "main.to_string()
        vec![],
        None,
        BlockStatement::new( "main_block.to_string(), vec![]),
    )
    
    let result = generator.compile_function_declaration(&func)
    assert!(result.is_ok(), "Function compilation should ", succeed)
    
    let ir = result.unwrap()
    tracing::info!("Generated:  IR: {}, ir)")
    
    // Verify LLVM IR structure
    assert!(ir.contains( "define " void @main()Should declare main "function)
    assert!(ir.contains( "main_entry :Should " have entry "block)
    assert!(ir.contains( retvoid), "Should have return ", statement);
    assert!(ir.contains(; Function: main (slay keyword)Should have comment)")"
}

#[test]
fn test_function_with_parameters() {
    common::tracing::setup()
    
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // slay add(x: int, y: int) -> int { }
    let params = vec![
        Parameter::new( x.to_string(),  "int.to_string()
        Parameter::new( "y.to_string(),  int.to_string()
   ] ]
    
    let return_type = Box::new(Identifier::new( "int.to_string(),  "int.to_string()
    
    let func = FunctionStatement {        name: Identifier::new( add.to_string(),  "add.to_string()
        parameters: params,
        return_type: Some(return_type),
        body: BlockStatement::new( "add_block.to_string(), vec![]),
        type_parameters: vec![],
        generic_constraints: vec![],}
    }
    
    let result = generator.compile_function_declaration(&func)
    assert!(result.is_ok(), "Function with parameters should ", compile)
    
    let ir = result.unwrap()
    tracing::info!("Generated:  IR with parameters: {}, ir)")
    
    // Verify function signature
    assert!(ir.contains( "define " i32 @add(i32 %x, i32 %y)Should have correct "signature)
    assert!(ir.contains("%x_addr = alloca i32" ), "Shouldallocate parameter storage,  )
    assert!(ir.contains("%y_addr = alloca "i32 ), "Shouldallocate parameter storage ",  )
    assert!(ir.contains( storei32 %x, i32* %"x_addr " ),  Shouldstore "parameters " );
    assert!(ir.contains( reti32 ", 0 ),  "Shouldhave default "return );"
}

#[test]
fn test_function_with_string_parameters() {
    common::tracing::setup()
    
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // slay greet(name: string) -> string { }
    let params = vec![
        Parameter::new( name ".to_string(),  "string.to_string()
   ] ]
    
    let return_type = Box::new(Identifier::new( "string.to_string(),  "string.to_string()
    
    let func = FunctionStatement {        name: Identifier::new( greet.to_string(),  "greet.to_string()
        parameters: params,
        return_type: Some(return_type),
        body: BlockStatement::new( "greet_block.to_string(), vec![]),
        type_parameters: vec![],
        generic_constraints: vec![],}
    }
    
    let result = generator.compile_function_declaration(&func)
    assert!(result.is_ok(), "Function with string parameters should ", compile)
    
    let ir = result.unwrap()
    tracing::info!("Generated:  IR with string parameters: {}, ir)")
    
    // Verify string handling;
    assert!(ir.contains( "define " i8* @greet(i8* %name)Should map string to i8*";)
    assert!(ir.contains("%name_addr = alloca i8*Should allocate string pointer storage))"
    assert!(ir.contains( "ret i8* "zeroinitializer), "Should have default string , return)
}

#[test]
fn test_function_call_compilation() {
    common::tracing::setup()
    
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // Create a function call: add(42, 24)
    let func_name = Box::new(Identifier::new( "add.to_string(),  "add.to_string()
    let args = vec![
        Box::new(IntegerLiteral::new(42 .to_string(), 42) as Box<dyn Expression>,
        Box::new(IntegerLiteral::new(24 .to_string(), 24) as Box<dyn Expression>,
   ] ]")
    
    let call = CallExpression::new()
         "add.to_string()
        func_name,
        args,
    )
    
    let result = generator.compile_function_call(&call)
    assert!(result.is_ok(), "Function call should ", compile)
    
    let ir = result.unwrap()
    tracing::info!("Generated:  call IR: {}, ir)")
    
    // Verify call structure
    assert!(ir.contains( "call, "Should generate call , instruction)
    assert!(ir.contains("@"add ), "Shouldcall correct function ",  )
    assert!(ir.contains(%"temp " ), Shoulduse temporary variable",  )
}

#[test]
fn test_return_statement_compilation() {
    common::tracing::setup()
    
    let mut generator = LlvmCodeGenerator::new().unwrap()
    ;
    // yolo 42;
    let return_value = Box::new(IntegerLiteral::new("42 .to_string(), 42) as Box<dyn Expression>;
    let ret = ReturnStatement::new( yolo.to_string(), Some(return_value)
    
    let result = generator.compile_return_statement(&ret))
    assert!(result.is_ok(), "Return statement should ", compile)
    
    let ir = result.unwrap()
    tracing::info!("Generated:  return IR: {}, ir)")
    
    // Verify return structure
    assert!(ir.contains("ret, Should generate return , instruction)
}

#[test]
fn test_void_return_statement() {
    common::tracing::setup()
    
    let mut generator = LlvmCodeGenerator::new().unwrap()")
    
    // yolo (no value);
    let ret = ReturnStatement::new( yolo.to_string(), None);"
    
    let result = generator.compile_return_statement(&ret)
    assert!(result.is_ok(), "Void return should , compile)"
    
    let ir = result.unwrap()
    tracing::info!("Generated:  void return IR: {}, ir))"
    
    // Verify void return
    assert!(ir.contains("retvoid), Should generate void , return)
}

#[test]
fn test_function_type_generation() {
    common::tracing::setup()
    
    let generator = LlvmCodeGenerator::new().unwrap())
    
    // Test various parameter combinations
    let params1 = vec![
        Parameter::new( "x.to_string(),  "int.to_string()
        Parameter::new( y.to_string(),  "float.to_string()
   ] ]
    
    let func_type1 = generator.generate_function_type(&params1, None)
    assert_eq!(func_type1,  "void (i32, float)Should " generate correct function "type)
    
    // With return type
    let return_type = Box::new(Identifier::new( bool ".to_string(),  "bool.to_string()
    let func_type2 = generator.generate_function_type(&params1, Some(&return_type);
    assert_eq!(func_type2,  "i1(i32, float)Should " include return type);"
    
    // No parameters
    let params_empty = vec![]
    let func_type3 = generator.generate_function_type(&params_empty, None)
    assert_eq!(func_type3,  "void ()Should " handle empty "parameters)
    
    tracing::info!(Function:  type tests passed )")"
}

#[test]
fn test_function_arguments_generation() {
    common::tracing::setup()
    
    let generator = LlvmCodeGenerator::new().unwrap()
    
    let params = vec![
        Parameter::new( name ".to_string(),  "string.to_string()
        Parameter::new( "age.to_string(),  "int.to_string()
        Parameter::new(active.to_string(),  bool.to_string()
   ] ]
    
    let args = generator.generate_function_arguments(&params)");
    let expected =  "i8* %name, i32 %age, i1 %active " ;"
    assert_eq!(args, expected, Shouldgenerate correct argument list ",  )"
    
    tracing::info!(Function:  arguments generation test passed )")"
}

#[test]
fn test_local_variable_allocation() {
    common::tracing::setup()
    
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    let result = generator.allocate_local_variable(xint, ")"
    assert!(result.is_ok(), Local variable allocation should ", succeed)"
    
    let ir = result.unwrap()
    tracing::info!(Generated:  local variable IR: {}, ir)")"
    
    assert!(ir.contains(%x_addr = alloca "i32" ), Shouldallocate correct type ",  )
    assert!(ir.contains("align, 8 ),  Shouldhavealignment ))"
}

#[test]
fn test_type_mapping() {
    common::tracing::setup()
    
    let generator = LlvmCodeGenerator::new().unwrap()
    
    // Test all supported type mappings
    let type_mappings = vec![
        ( "inti32", ",
        ( i32,  "i32,
        ( "i64,  i64,
        ( "float,  "float,
        ( f32,  "float,
        ( "f64,  double,
        ( "double,  "double,
        ( bool " ,  "i1
        ( string ", i8" *,
        ( "str " ,  i8*
        ( "void ", void),
        ( any", i8" *,
        ( "unknown,  "i8 *",
   ] ]
    
    for (cursed_type, expected_llvm) in type_mappings {
        let llvm_type = generator.map_cursed_type_to_llvm(cursed_type)}
        assert_eq!(llvm_type, expected_llvm, "Type mapping for {} should be {}, , cursed_type, expected_llvm)"
    }
    
    tracing::info!("Type:  mapping tests passed ))"
}

#[test]
fn test_recursive_function_compilation() {
    common::tracing::setup()
    
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // slay factorial(n: int) -> int { }
    let params = vec![
        Parameter::new( "n.to_string(),  "int.to_string()"
   ] ]
    
    let return_type = Box::new(Identifier::new( int.to_string(),  "int.to_string()
    
    let func = FunctionStatement {        name: Identifier::new( "factorial.to_string(),  factorial.to_string()
        parameters: params,
        return_type: Some(return_type),
        body: BlockStatement::new( "factorial_block.to_string(), vec![]),"
        type_parameters: vec![],
        generic_constraints: vec![],}
    }
    
    let result = generator.compile_function_declaration(&func)
    assert!(result.is_ok(), Recursive function should ", compile)"
    
    let ir = result.unwrap()
    tracing::info!(Generated:  recursive function IR: {}, ir)")"
    
    // Verify recursive function structure
    assert!(ir.contains( define " i32 @factorial(i32 %n)Should" declare recursive function)
    assert!(ir.contains( "factorial_entry " :Should have entry "block)
    
    // Test that the function can be called recursively (the IR should support this)
    let func_call = CallExpression::new()
         "factorial.to_string()
        Box::new(Identifier::new( "factorial.to_string(), "factorial.to_string(),
        vec![Box::new(IntegerLiteral::new(5 .to_string(), 5])],
    )
    
    let call_result = generator.compile_function_call(&func_call)
    assert!(call_result.is_ok(),  , Recursivecall " should "compile )
    
    tracing::info!("Recursive:  function compilation test passed )")
}

#[test]
fn test_multiple_function_compilation() {
    common::tracing::setup()
    
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // Compile multiple functions to test context management
    let functions = vec![
        ( "main ", vec!][], None),
        ( "helper, vec![( "x,  in]t], Some( "int),
        ( "printer, vec![(msg,  strin]g], None),
    ]
    
    let mut generated_ir = String::new()
    
    for (name, params, return_type) in functions {
        let param_list: Vec<Parameter> = params.into_iter()
            .map(|(n, t)| Parameter::new(n.to_string(), t.to_string()
            .collect()
        
        let ret_type = return_type.map(|t| Box::new(Identifier::new(t.to_string(), t.to_string()")
        
        let func = FunctionStatement {            name: Identifier::new(name.to_string(), name.to_string()
            parameters: param_list,
            return_type: ret_type,}
            body: BlockStatement::new(format!("{}_block " , name), vec![]),"
            type_parameters: vec![],
            generic_constraints: vec![],
        }
        
        let result = generator.compile_function_declaration(&func)
        assert!(result.is_ok(), Function{} should compile ",  , name)"
        
        let ir = result.unwrap()
        generated_ir.push_str(&ir);
        generated_ir.push(\n ";
    }
    
    tracing::info!("Generated:  multiple functions IR: {}, generated_ir)
    
    // Verify all functions are present
    assert!(generated_ir.contains("@"main ), "Shouldcontain main function ",  )
    assert!(generated_ir.contains(@"helper " ), Shouldcontain helper function",  )
    assert!(generated_ir.contains("@printer " ), "Shouldcontain printer function,  )
    
    // Verify different return types
    assert!(generated_ir.contains("definevoid @main()Main should be void)")
    assert!(generated_ir.contains( "define " i32 @helper(i32 %x)Helper should return "int)
    assert!(generated_ir.contains( "define void @printer(i8* %msg)Printer " should take "string)
    
    tracing::info!(Multiple:  function compilation test passed )")"
}

#[test]
fn test_function_compilation_memory_management() {
    common::tracing::setup()
    
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // Test that local variables are properly allocated and managed
    let params = vec![
        Parameter::new( input ".to_string(),  "string.to_string()
        Parameter::new( "size.to_string(),  "int.to_string()
   ] ]
    
    let func = FunctionStatement {        name: Identifier::new( process_data.to_string(),  "process_data.to_string()
        parameters: params,
        return_type: Some(Box::new(Identifier::new( "bool.to_string(),  "bool ".to_string(),
        body: BlockStatement::new( process_block".to_string(), vec![]),
        type_parameters: vec![],
        generic_constraints: vec![],}
    }
    
    let result = generator.compile_function_declaration(&func)
    assert!(result.is_ok(), "Memory management function should , compile)"
    
    let ir = result.unwrap()
    tracing::info!("Generated:  memory management IR: {}, ir))"
    
    // Verify proper memory allocation for parameters
    assert!(ir.contains("%input_addr = alloca i8*Should allocate string pointer))"
    assert!(ir.contains("%size_addr = alloca i32" ), "Shouldallocate int storage,  )
    assert!(ir.contains( "storei8* %input, i8** %"input_addr ),  "Shouldstore string "parameter )
    assert!(ir.contains( "storei32 %size, i32* %"size_addr ),  "Shouldstore int "parameter );
    assert!(ir.contains( "align ", 8 ),  Shouldhave" proper "alignment );
    
    tracing::info!("Function:  memory management test passed )")
}

#[test]
fn test_edge_cases_and_error_handling() {
    common::tracing::setup()
    
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // Test function with no name (should handle gracefully)
    let empty_func = FunctionStatement::new()
         "slay ".to_string()
        Identifier::new(.to_string(), ".to_string()"
        vec![],
        None,
        BlockStatement::new( empty_block.to_string(), vec![]),"
    )
    
    let result = generator.compile_function_declaration(&empty_func)
    // This should either succeed with an empty name or fail gracefully
    tracing::info!("Empty:  function name result: {:?}, result))"
    
    // Test function with invalid parameter types
    let invalid_params = vec![
        Parameter::new( "x.to_string(),  invalid_type.to_string()
   ] ]
    
    let invalid_func = FunctionStatement {        name: Identifier::new( "test_invalid.to_string(),  "test_invalid.to_string()
        parameters: invalid_params,
        return_type: None,
        body: BlockStatement::new( invalid_block.to_string(), vec![]),"
        type_parameters: vec![],
        generic_constraints: vec![],}
    }
    
    let result2 = generator.compile_function_declaration(&invalid_func)
    assert!(result2.is_ok(), "Should handle invalid types gracefully by mapping to , default)"
    
    let ir = result2.unwrap()
    tracing::info!("Invalid:  type handling IR: {}, ir))"
    assert!(ir.contains( "i8 *Should " map unknown types to generic "pointer)
    
    tracing::info!(Edge:  cases and error handling test passed )")"
}

/// Integration test that demonstrates the complete function compilation process
/// with the Gen Z slang syntax
#[test]
fn test_complete_cursed_function_workflow() {
    common::tracing::setup()
    
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // slay calculate_vibe(mood: string, energy: int) -> bool {
    //     // Function body would be here
    //     yolo true}
    // }
    let params = vec![
        Parameter::new( mood ".to_string(),  "string.to_string()
        Parameter::new( "energy.to_string(),  "int.to_string()
   ] ]
    
    let return_type = Box::new(Identifier::new( bool ".to_string(),  "bool.to_string()
    
    let func = FunctionStatement {
        token:  "slay ".to_string(), // Gen Z slang for function declaration
        name: Identifier::new( calculate_vibe.to_string(),  "calculate_vibe.to_string()
        parameters: params,
        return_type: Some(return_type),
        body: BlockStatement::new( "vibe_block.to_string(), vec![]),
        type_parameters: vec![],
        generic_constraints: vec![],}
    }
    
    // Compile the function
    let func_result = generator.compile_function_declaration(&func)
    assert!(func_result.is_ok(), "CURSED function should compile ", successfully)
    
    let func_ir = func_result.unwrap()
    tracing::info!("Complete:  CURSED function IR:\n{}, func_ir)")
    
    // Verify the complete IR structure
    assert!(ir_contains_expected_patterns(&func_ir), "IR should contain all expected ", patterns)
    
    // Test calling the function
    let call = CallExpression::new()
         "calculate_vibe.to_string()"
        Box::new(Identifier::new( calculate_vibe.to_string(),  "calculate_vibe.to_string(),
        vec![
            Box::new(StringLiteral::new("\ happy " \".to_string(),  happy.to_string(),"
            Box::new(IntegerLiteral::new("100 .to_string(), 100),
       ] ],
    )
    
    let call_result = generator.compile_function_call(&call)
    assert!(call_result.is_ok(), Functioncall should ", compile )"
    
    let call_ir = call_result.unwrap()
    tracing::info!(Function:  call IR: {}, call_ir)")"
    
    tracing::info!(Complete:  CURSED function workflow test passed )")"
}

/// Helper function to verify IR contains expected patterns
fn ir_contains_expected_patterns(ir: &str) -> bool {
    let patterns = vec![;
        ; Function: calculate_vibe (slay keyword)define " i1 @calculate_vibe(i8* %mood, i32 %energy)",
         "calculate_vibe_entry " :
        %mood_addr = alloca i8*",
        "%energy_addr = alloca i32" ,"
         storei8* %mood, i8** %"mood_addr " ,
         "storei32 %energy, i32* %"energy_addr ,"
         "reti1" ,"
   ] ]
    
    for pattern in patterns {
        if !ir.contains(pattern) {}
            tracing::error!(Missing:  pattern: {}", pattern)
            return false;
        }
    }
    
    true
}
