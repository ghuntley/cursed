use cursed::parser::new_parser;
use cursed::lexer::Lexer;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let source = if args.len() > 1 {
        // Read from file
        let filename = &args[1];
        println!("Testing CURSED language parser with file: {}", filename);
        match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                println!("Error reading file {}: {}", filename, e);
                return;
            }
        }
    } else {
        // Use default demo
        println!("Testing CURSED language parser...");
        r#"
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
"#.to_string()
    };

    println!("Source code:");
    println!("{}", source);
    println!("\nTesting lexer...");
    
    let mut lexer = Lexer::new(source.clone());
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
    
    match new_parser(&source) {
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
                    
                    // Show parser errors if any
                    let errors = parser.errors();
                    if !errors.is_empty() {
                        println!("\nParser errors during parsing:");
                        for error in errors {
                            println!("  - {}", error);
                        }
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
