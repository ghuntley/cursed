use cursed::compile_to_ir;

fn main() {
    let cursed_code = r#"
slay main() {
    yolo "Hello, World!"
}
"#;
    
    println!("Testing compile_to_ir function...");
    
    match compile_to_ir(cursed_code) {
        Ok(ir) => {
            println!("✅ SUCCESS: compile_to_ir returned IR!");
            println!("IR length: {} characters", ir.len());
            println!("IR preview: {}", &ir[..ir.len().min(200)]);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
        }
    }
}
