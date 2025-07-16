use cursed::codegen::llvm::LlvmCodeGenerator;

fn main() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    let source = r#"vibez.spill("Hello World")"#;
    
    match generator.compile(source) {
        Ok(ir) => {
            println!("Generated LLVM IR:");
            println!("{}", ir);
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
