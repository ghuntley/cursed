use cursed::{Lexer, Parser, ast::*};

fn test_code(name: &str, code: &str) {
    println!("\n🧪 Testing: {}", name);
    println!("Code: {}", code);
    
    let mut lexer = Lexer::new(code.to_string());
    let tokens = lexer.tokenize().unwrap();
    
    let mut parser = Parser::from_tokens(tokens);
    match parser.parse() {
        Ok(program) => {
            println!("✅ Parse successful!");
            println!("Statements: {}", program.statements.len());
            for (i, stmt) in program.statements.iter().enumerate() {
                match stmt {
                    Statement::Function(func) => {
                        println!("  Statement {}: Function '{}' with {} body statements", 
                                 i, func.name, func.body.len());
                    },
                    _ => {
                        println!("  Statement {}: {:?}", i, std::mem::discriminant(stmt));
                    }
                }
            }
        },
        Err(e) => {
            println!("❌ Parse failed: {}", e);
            let errors = parser.errors();
            for error in errors {
                println!("  Error: {}", error);
            }
        }
    }
}

fn main() {
    println!("🔍 Step-by-step debugging of demonstrateBasics...");
    
    // Step 1: Empty function
    test_code("Empty function", r#"
slay demonstrateBasics() {
}
"#);
    
    // Step 2: Function with one variable
    test_code("Function with one variable", r#"
slay demonstrateBasics() {
    sus radius snack = 5.0
}
"#);
    
    // Step 3: Function with all variables
    test_code("Function with all variables", r#"
slay demonstrateBasics() {
    sus radius snack = 5.0
    sus userName tea = "Developer"
    sus isAwesome lit = based
}
"#);
    
    // Step 4: Function with variables and function call
    test_code("Function with variables and call", r#"
slay demonstrateBasics() {
    sus radius snack = 5.0
    sus userName tea = "Developer"
    sus isAwesome lit = based
    
    sus area = calculateArea(radius)
    greetUser(userName)
}
"#);
    
    // Step 5: Function with variables, calls, and output
    test_code("Function with variables, calls, and output", r#"
slay demonstrateBasics() {
    sus radius snack = 5.0
    sus userName tea = "Developer"
    sus isAwesome lit = based
    
    sus area = calculateArea(radius)
    greetUser(userName)
    
    vibez.spill("Circle radius: " + radius)
    vibez.spill("Circle area: " + area)
}
"#);
    
    // Step 6: Function with everything including if statement
    test_code("Function with everything including if", r#"
slay demonstrateBasics() {
    sus radius snack = 5.0
    sus userName tea = "Developer"
    sus isAwesome lit = based
    
    sus area = calculateArea(radius)
    greetUser(userName)
    
    vibez.spill("Circle radius: " + radius)
    vibez.spill("Circle area: " + area)
    
    lowkey isAwesome {
        vibez.spill("This language is based!")
    } highkey {
        vibez.spill("Something is sus...")
    }
}
"#);
}
