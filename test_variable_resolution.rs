use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let source = "slay test() { sus x = 5; yolo x; }";
    
    let mut generator = cursed::codegen::llvm::main::LlvmCodeGenerator::new()?;
    
    match generator.compile(&source) {
        Ok(ir) => {
            println!("Generated LLVM IR:");
            println!("{}", ir);
        },
        Err(e) => {
            println!("Compilation error: {:?}", e);
        }
    }
    
    Ok(())
}
