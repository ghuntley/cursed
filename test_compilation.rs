use cursed::codegen::LlvmCodeGenerator;

fn test_basic_compilation() {
    println!("Testing basic CURSED compilation...");
    let source = r#"
    slay hello() {
        yolo 42;
    }
    "#;
    
    let mut codegen = LlvmCodeGenerator::new().unwrap();
    match codegen.compile(source) {
        Ok(ir) => {
            println!("Successfully compiled to LLVM IR:");
            println!("{}", ir);
        }
        Err(e) => println!("Compilation error: {:?}", e),
    }
}

fn main() {
    test_basic_compilation();
}
