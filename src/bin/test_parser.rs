use cursed::parser::new_parser;
use cursed::lexer::Lexer;

fn main() {
    println!("Testing CURSED language parser...");
    
    let source = r#"
vibe main

facts greeting = "Hello from CURSED!"
facts number = 42

slay greet() {
    yolo greeting
}

slay main() {
    greet()
    yolo 0
}
"#;

    println!("Source code:");
    println!("{}", source);
    println!("\nTesting lexer...");
    
    let mut lexer = Lexer::new(source.to_string());
    match lexer.tokenize() {
        Ok(tokens) => {
            println!("Tokens generated: {}", tokens.len());
            for (i, token) in tokens.iter().enumerate() {
                println!("  {}. {:?}", i + 1, token);
            }
        }
        Err(e) => {
            println!("Lexer error: {:?}", e);
            return;
        }
    }
    
    println!("\nTesting parser...");
    
    match new_parser(source) {
        Ok(mut parser) => {
            match parser.parse_program() {
                Ok(program) => {
                    println!("Program parsed successfully!");
                    println!("Package: {:?}", program.package);
                    println!("Imports: {}", program.imports.len());
                    println!("Statements: {}", program.statements.len());
                    
                    for (i, stmt) in program.statements.iter().enumerate() {
                        println!("  Statement {}: {:?}", i + 1, stmt);
                    }
                }
                Err(e) => {
                    println!("Parser error: {:?}", e);
                    let errors = parser.errors();
                    if !errors.is_empty() {
                        println!("Parser errors:");
                        for error in errors {
                            println!("  - {}", error);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to create parser: {:?}", e);
        }
    }
}
