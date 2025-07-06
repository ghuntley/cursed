use cursed::codegen::llvm::main::LlvmCodeGenerator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = r#"sus x normie = 42
sus s tea = "hello"
sus b lit = based

vibez.spill(x)
vibez.spill(s)
vibez.spill(b)"#;

    let mut codegen = LlvmCodeGenerator::new()?;
    let ir = codegen.compile(source)?;
    
    println!("Generated IR:");
    println!("{}", ir);
    
    Ok(())
}
