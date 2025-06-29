use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let source = std::fs::read_to_string("test_cursed_function.csd")?;
    
    let mut generator = cursed::codegen::llvm::main::LlvmCodeGenerator::new()?;
    
    match generator.compile(&source) {
        Ok(ir) => {
            println!("Generated LLVM IR:");
            println!("{}", ir);
            
            // Write IR to file for inspection
            std::fs::write("test_cursed_function_compilation.ll", &ir)?;
            println!("IR written to test_cursed_function_compilation.ll");
        },
        Err(e) => {
            println!("Compilation error: {:?}", e);
        }
    }
    
    Ok(())
}
