fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("debug"))
        .init();
    
    tracing::info!("Starting preprocessor tests");
    println!("Running manual test for preprocessor...");

    // Create an example with generics
    let input = "be_like Box[T] squad { value normie }";
    tracing::debug!(test_case = "generic_type", input = input, "Testing generic type declaration");
    
    let mut lexer = cursed::lexer::Lexer::new(input);
    let mut preprocessor = cursed::parser::preprocessor::Preprocessor::new(&mut lexer);
    
    // Process tokens
    let tokens_result = preprocessor.process();
    if let Ok(tokens) = tokens_result {
        println!("Tokens processed successfully");
        if tokens.contains_generic_type_declaration("Box", &["T"]) {
            tracing::info!("Generic type declaration test passed");
            println!("✅ Generic type declaration found");
        } else {
            tracing::error!("Generic type declaration test failed");
            println!("❌ Generic type declaration NOT found");
        }
    } else {
        let err = tokens_result.err().unwrap();
        tracing::error!(error = ?err, "Token processing failed");
        println!("❌ Error processing tokens: {:?}", err);
    }

    // Test function declarations
    let input = "slay foo[T](x normie) T { yolo x }";
    let mut lexer = cursed::lexer::Lexer::new(input);
    let mut preprocessor = cursed::parser::preprocessor::Preprocessor::new(&mut lexer);
    
    // Process tokens
    let tokens_result = preprocessor.process();
    if let Ok(tokens) = tokens_result {
        println!("Tokens processed successfully");
        if tokens.contains_generic_function_declaration("foo", &["T"]) {
            println!("✅ Generic function declaration found");
        } else {
            println!("❌ Generic function declaration NOT found");
        }
    } else {
        println!("❌ Error processing tokens: {:?}", tokens_result.err());
    }

    // Test function calls
    let input = "foo[normie](42)";
    let mut lexer = cursed::lexer::Lexer::new(input);
    let mut preprocessor = cursed::parser::preprocessor::Preprocessor::new(&mut lexer);
    
    // Process tokens
    let tokens_result = preprocessor.process();
    if let Ok(tokens) = tokens_result {
        println!("Tokens processed successfully");
        if tokens.contains_generic_function_call("foo", &["normie"]) {
            println!("✅ Generic function call found");
        } else {
            println!("❌ Generic function call NOT found");
        }
    } else {
        println!("❌ Error processing tokens: {:?}", tokens_result.err());
    }

    // Test malformed syntax
    let input = "be_like Box[T squad { value normie }"; // Missing closing bracket
    let mut lexer = cursed::lexer::Lexer::new(input);
    let mut preprocessor = cursed::parser::preprocessor::Preprocessor::new(&mut lexer);
    
    // Process tokens should return an error
    let result = preprocessor.process();
    if let Err(err) = result {
        println!("✅ Correctly detected error: {}", err);
    } else {
        println!("❌ Failed to detect malformed syntax");
    }

    println!("\nPreprocessor tests completed.");
}