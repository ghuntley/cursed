use cursed::execution::CursedExecutionEngine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = CursedExecutionEngine::new()?;
    
    // Test simple variable assignment and arithmetic
    let program1 = r#"
let x = 10;
let y = 20;
let sum = x + y;
vibez.spill(sum);
"#;
    
    println!("Testing program 1:");
    let result1 = engine.execute(program1)?;
    println!("Result: {:?}\n", result1);
    
    // Test conditional logic
    let program2 = r#"
let a = 15;
let b = 10;
if a > b {
    vibez.spill("a is greater than b");
} else {
    vibez.spill("b is greater than or equal to a");
}
"#;
    
    println!("Testing program 2:");
    let result2 = engine.execute(program2)?;
    println!("Result: {:?}\n", result2);
    
    // Test loops
    let program3 = r#"
let i = 0;
while i < 3 {
    vibez.spill(i);
    i = i + 1;
}
"#;
    
    println!("Testing program 3:");
    let result3 = engine.execute(program3)?;
    println!("Result: {:?}\n", result3);
    
    Ok(())
}
