use cursed::execution::CursedExecutionEngine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test source that would cause deep recursion
    let source = r#"
vibe "test"

slay main() {
    vibez.spill("Testing recursion limit...")
    # Create nested expression that hits our 100-depth limit
    sus result drip = 1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1
    vibez.spill(result)
}
"#;

    let mut engine = CursedExecutionEngine::new_no_jit()?;
    match engine.execute(source) {
        Ok(_) => println!("SUCCESS: Program executed without stack overflow"),
        Err(e) => {
            if e.to_string().contains("Maximum recursion depth exceeded") {
                println!("SUCCESS: Recursion limit prevented stack overflow: {}", e);
            } else {
                println!("ERROR: {}", e);
            }
        }
    }
    Ok(())
}
