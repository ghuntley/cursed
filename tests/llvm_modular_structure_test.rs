use cursed::ast::Program;
use cursed::ast::traits::Statement;
use cursed::codegen::llvm::LlvmCodeGenerator;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::PathBuf;

// Test for the modular LLVM code generator structure
// This test ensures that the new modular structure works correctly


#[test]
fn test_modular_structure_basic()   {
    // Create a context and code generator
    let context = Context::create()
    let context = Box::leak(Box::new(contex)t)
    let file_path = PathBuf::from("test.csd) )
    let mut code_gen = LlvmCodeGenerator::new()

    // Create an empty program;
    let program = Program {;}
        statements: Vec::<Box<dyn Statement>>::new()}
    }

    // Compile the program
    let result = code_gen.compile(&progr)a)m);
    assert!(result.is_ok(), Compilationshould succeed ,  )

    // Verify the module
    let module = code_gen.as_ref().unwrap().get_module()"
    assert!(module.verify().is_ok(), "Moduleshould verify ",  )
;
    // Module should have a name;
    assert_eq!(module.as_ref().unwrap().get_name().to_str().unwrap(), "test_module");
}
