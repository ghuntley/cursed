//! Demonstration of advanced function signature parsing capabilities

use cursed::lexer::Lexer;
use cursed::parser::advanced_signature_parser::AdvancedSignatureParser;

fn main() {
    println!("🚀 CURSED Advanced Function Signature Parsing Demo");
    println!("{}", "=".repeat(50));

    let examples = vec![
        // Basic function
        ("Basic function", "slay add(x normie, y normie) -> normie"),
        
        // Variadic function
        ("Variadic function", "slay printf(format tea, ...args normie)"),
        
        // Tuple return type
        ("Tuple return", "slay get_coords() -> (normie, normie)"),
        
        // Function pointer parameter
        ("Function pointer", "slay callback(handler fn(normie) -> lit)"),
        
        // Generic with bounds
        ("Generic with bounds", "slay sort<T: Clone + Debug>(items [T])"),
        
        // Async function
        ("Async function", "async slay fetch_data(url tea) -> tea"),
        
        // Unsafe function with pointer
        ("Unsafe pointer", "unsafe slay raw_access(ptr *normie) -> normie"),
        
        // Complex function with documentation
        ("Complex function", r#"/// Processes data with advanced features
pub async slay process<T: Clone>(
    mut data T,
    processor fn(T) -> T,
    options (tea, normie),
    ...extensions tea
) -> T where T: Debug"#),
    ];

    for (name, signature) in examples {
        println!("\n🔧 Testing: {}", name);
        println!("📝 Signature: {}", signature);
        
        match parse_signature(signature) {
            Ok(parsed) => {
                println!("✅ Successfully parsed!");
                println!("   Function name: {}", parsed.name);
                println!("   Parameters: {}", parsed.parameters.len());
                println!("   Type parameters: {}", parsed.type_parameters.len());
                println!("   Is async: {}", parsed.is_async);
                println!("   Is unsafe: {}", parsed.is_unsafe);
                
                // Check for advanced features
                let variadic_params = parsed.parameters.iter().filter(|p| p.is_variadic).count();
                if variadic_params > 0 {
                    println!("   🎯 Variadic parameters: {}", variadic_params);
                }
                
                if !parsed.where_clauses.is_empty() {
                    println!("   🎯 Where clauses: {}", parsed.where_clauses.len());
                }
                
                if parsed.documentation.is_some() {
                    println!("   🎯 Has documentation");
                }
            }
            Err(e) => {
                println!("❌ Parse error: {:?}", e);
            }
        }
    }

    println!("\n{}", "=".repeat(50));
    println!("🎉 Advanced signature parsing demonstration complete!");
    println!("   ✨ Supports variadic parameters (...syntax)");
    println!("   ✨ Supports complex generic bounds and where clauses");
    println!("   ✨ Supports tuple types in parameters and returns");
    println!("   ✨ Supports function pointer types");
    println!("   ✨ Supports enhanced array/slice type annotations");
    println!("   ✨ Supports async/unsafe keywords");
    println!("   ✨ Supports documentation generation");
}

fn parse_signature(input: &str) -> Result<cursed::parser::AdvancedFunctionSignature, cursed::error::CursedError> {
    let mut lexer = Lexer::new(input.to_string());
    let mut tokens = Vec::new();
    
    loop {
        match lexer.next_token() {
            Ok(token) => {
                let is_eof = token.kind == cursed::lexer::TokenKind::Eof;
                tokens.push(token);
                if is_eof { break; }
            }
            Err(_) => break,
        }
    }
    
    let mut parser = AdvancedSignatureParser::new(&tokens);
    parser.parse_advanced_function_signature()
}
