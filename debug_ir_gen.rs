use cursed::codegen::llvm::main::LlvmCodeGenerator;
use cursed::parser::new_parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = r#"
sus x normie = 1
sus y normie = 2
"#;
    
    let mut parser = new_parser(source)?;
    let program = parser.parse_program()?;
    
    let mut codegen = LlvmCodeGenerator::new()?;
    let ir = codegen.generate_ir(&program)?;
    
    println!("Generated LLVM IR:");
    println!("{}", ir);
    
    Ok(())
}
