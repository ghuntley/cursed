use cursed::{run, check, format};

fn test_basic_execution() {
    println!("=== Testing Basic CURSED Execution ===");
    
    let simple_code = r#"
    slay hello() {
        yolo 42;
    }
    "#;
    
    println!("Source code:");
    println!("{}", simple_code);
    
    match check(simple_code) {
        Ok(_) => println!("✓ Code check passed"),
        Err(e) => println!("✗ Code check failed: {:?}", e),
    }
    
    match format(simple_code) {
        Ok(formatted) => {
            println!("✓ Code formatting successful:");
            println!("{}", formatted);
        }
        Err(e) => println!("✗ Code formatting failed: {:?}", e),
    }
    
    match run(simple_code) {
        Ok(_) => println!("✓ Code execution successful"),
        Err(e) => println!("✗ Code execution failed: {:?}", e),
    }
}

fn test_cursed_keywords() {
    println!("\n=== Testing CURSED Gen Z Keywords ===");
    
    let cursed_code = r#"
    facts meaning_of_life = 42;
    sus vibe_count = 0;
    
    slay check_vibes(count) {
        lowkey count > 0 {
            yolo "good vibes";
        } highkey {
            yolo "no vibes";
        }
    }
    "#;
    
    println!("CURSED code with Gen Z syntax:");
    println!("{}", cursed_code);
    
    match check(cursed_code) {
        Ok(_) => println!("✓ CURSED keyword parsing successful"),
        Err(e) => println!("✗ CURSED keyword parsing failed: {:?}", e),
    }
}

fn test_member_access() {
    println!("\n=== Testing Member Access (vibez.spill()) ===");
    
    let member_code = r#"
    vibez.spill("hello world");
    "#;
    
    println!("Member access code:");
    println!("{}", member_code);
    
    match check(member_code) {
        Ok(_) => println!("✓ Member access parsing successful"),
        Err(e) => println!("✗ Member access parsing failed: {:?}", e),
    }
}

fn main() {
    test_basic_execution();
    test_cursed_keywords();
    test_member_access();
    
    println!("\n=== Summary ===");
    println!("Core functionality tests completed.");
    println!("Check the output above for detailed results.");
}
