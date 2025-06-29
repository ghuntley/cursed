use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let source = "slay test() { sus x = 5; yolo x; }";
    
    println!("Source: {}", source);
    
    // Parse first
    let mut parser = cursed::parser::new_parser(&source)?;
    let program = parser.parse_program()?;
    
    println!("Parsed program:");
    for (i, stmt) in program.statements.iter().enumerate() {
        println!("Statement {}: {:?}", i, stmt);
    }
    
    // Then compile
    let mut generator = cursed::codegen::llvm::main::LlvmCodeGenerator::new()?;
    
    match generator.compile(&source) {
        Ok(ir) => {
            println!("\nGenerated LLVM IR:");
            println!("{}", ir);
        },
        Err(e) => {
            println!("Compilation error: {:?}", e);
        }
    }
    
    Ok(())
}
