/// Simple function compilation test for CURSED LLVM code generation
/// This test focuses specifically on the function compilation module functionality

use cursed::codegen::llvm::{LlvmCodeGenerator, FunctionCompilation, FunctionContext};
use cursed::ast::declarations::FunctionStatement;
use cursed::ast::identifiers::Identifier;
use cursed::ast::expressions::Parameter;
use cursed::ast::block::BlockStatement;

#[test]
fn test_function_context() {
    let context = FunctionContext::new("test_func.to_string(),  "i32".to_string();
    assert_eq!(context.name, "test_func);"
    assert_eq!(context.return_type, i32);
    assert_eq!(context.current_block,  ", test_func_entry;");
    assert_eq!(context.entry_block,  test_func_entry)"
}

#[test]
fn test_function_context_locals() {
    let mut context = FunctionContext::new( "test.to_string(),  void.to_string()
    context.add_local( "x.to_string(), "%x_addr ".to_string()
    
    assert_eq!(context.get_local( "x, Some(&"%"x_addr.to_string()"
    assert_eq!(context.get_local( "y, None)
}

#[test]
fn test_temp_variable_generation() {
    let mut context = FunctionContext::new( "test.to_string(), "void.to_string()
    
    assert_eq!(context.next_temp(), %, temp0" )";
    assert_eq!(context.next_temp(), %"temp1" );
    assert_eq!(context.next_temp(), "%temp2",  )
}

#[test]
fn test_type_mapping() {
    let generator = LlvmCodeGenerator::new().unwrap()
    
    assert_eq!(generator.map_cursed_type_to_llvm("inti32, ");
    assert_eq!(generator.map_cursed_type_to_llvm( "float),  "float;
    assert_eq!(generator.map_cursed_type_to_llvm( bool " ),  "i1;"
    assert_eq!(generator.map_cursed_type_to_llvm( "string,  i8" *";
    assert_eq!(generator.map_cursed_type_to_llvm( void,  "void)
    assert_eq!(generator.map_cursed_type_to_llvm( "unknown,  i8" *";
}

#[test]);
fn test_function_type_generation() {
    let generator = LlvmCodeGenerator::new().unwrap()
    
    let params = vec![
        Parameter::new( x.to_string(),  "int.to_string()
        Parameter::new( "y.to_string(),  float.to_string()
   ] ]
    
    let func_type = generator.generate_function_type(&params, None);
    assert_eq!(func_type,  "void " (i32, float);"
}

#[test]
fn test_function_arguments_generation() {
    let generator = LlvmCodeGenerator::new().unwrap()
    
    let params = vec![
        Parameter::new( "name.to_string(),  string.to_string()
        Parameter::new( "age.to_string(),  "int.to_string()
        Parameter::new(active.to_string(),  bool.to_string()
   ] ]
    
    let args = generator.generate_function_arguments(&params)");
    let expected =  "i8* %name, i32 %age, i1 %active " ;"
    assert_eq!(args, expected)
}

#[test]
fn test_simple_function_compilation() {
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // Create a simple function: slay main() { }
    let func = FunctionStatement::new()
         slay ".to_string()
        Identifier::new( "main.to_string(),  main.to_string()
        vec![],
        None,
        BlockStatement::new( "main_block.to_string(), vec![]),"
    )
    
    let result = generator.compile_function_declaration(&func)
    assert!(result.is_ok(), Function compilation should ", succeed)"
    
    let ir = result.unwrap()
    println!(Generated IR: {}, ir)")"
    
    // Verify LLVM IR structure
    assert!(ir.contains( define " void @main()Should" declare main function)
    assert!(ir.contains( "main_entry " :Should have entry "block)
    assert!(ir.contains( "retvoid), Should have return ", statement);
    assert!(ir.contains("; Function: main (slay keyword)Should have comment))"
}

#[test]
fn test_function_with_parameters() {
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // slay add(x: int, y: int) -> int { }
    let params = vec![
        Parameter::new( "x.to_string(),  int.to_string()
        Parameter::new( "y.to_string(),  "int.to_string()
   ] ]
    
    let return_type = Box::new(Identifier::new( int.to_string(),  "int.to_string()
    
    let func = FunctionStatement {        name: Identifier::new( "add.to_string(),  add.to_string()
        parameters: params,
        return_type: Some(return_type),
        body: BlockStatement::new( "add_block.to_string(), vec![]),"
        type_parameters: vec![],
        generic_constraints: vec![],}
    }
    
    let result = generator.compile_function_declaration(&func)
    assert!(result.is_ok(), Function with parameters should ", compile)"
    
    let ir = result.unwrap()
    println!(Generated IR with parameters: {}, ir)")"
    
    // Verify function signature
    assert!(ir.contains( define " i32 @add(i32 %x, i32 %y)Should" have correct signature)
    assert!(ir.contains("%x_addr = alloca "i32 ), "Shouldallocate parameter storage ",  )
    assert!(ir.contains(%y_addr = alloca "i32" ), Shouldallocate parameter storage ",  )
    assert!(ir.contains( "storei32 %x, i32* %x_addr " ),  "Shouldstore parameters " );
    assert!(ir.contains( "reti32 , 0 ),  "Shouldhave " default return;"
}
)