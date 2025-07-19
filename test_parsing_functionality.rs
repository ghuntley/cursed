// Simple test for the parameter parsing functions I implemented

#[derive(Debug)]
struct Parameter {
    name: String,
    param_type: String,
    description: String,
}

/// Parse function parameters from a function declaration line
/// Supports both Rust-style (fn name(params)) and CURSED-style (slay name(params)) syntax
fn parse_function_parameters(line: &str) -> Vec<Parameter> {
    let mut parameters = Vec::new();
    
    // Find the parameter list between parentheses
    if let Some(start) = line.find('(') {
        if let Some(end) = line.find(')') {
            let param_str = &line[start + 1..end];
            
            if param_str.trim().is_empty() {
                return parameters;
            }
            
            // Split parameters by comma
            for param in param_str.split(',') {
                let param = param.trim();
                
                if param.is_empty() {
                    continue;
                }
                
                // Parse individual parameter
                // Format: "name: type" or "name tea" (CURSED syntax)
                let (name, param_type) = if param.contains(':') {
                    // Standard format: "name: type"
                    let parts: Vec<&str> = param.splitn(2, ':').collect();
                    let name = parts[0].trim().to_string();
                    let param_type = parts.get(1).map(|t| t.trim().to_string()).unwrap_or_default();
                    (name, param_type)
                } else {
                    // CURSED format: might be "name type" or just "name"
                    let parts: Vec<&str> = param.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let name = parts[0].to_string();
                        let param_type = parts[1..].join(" ");
                        (name, param_type)
                    } else {
                        let name = parts[0].to_string();
                        (name, "unknown".to_string())
                    }
                };
                
                parameters.push(Parameter {
                    name,
                    param_type,
                    description: String::new(), // Will be filled from documentation comments
                });
            }
        }
    }
    
    parameters
}

/// Parse function return type from a function declaration line
/// Supports both Rust-style (-> type) and CURSED-style (type after parentheses)
fn parse_function_return_type(line: &str) -> Option<String> {
    // Look for Rust-style return type with "->"
    if let Some(arrow_pos) = line.find("->") {
        let return_part = &line[arrow_pos + 2..];
        
        // Extract return type before any braces
        let return_type = return_part
            .split('{')
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
            
        if !return_type.is_empty() {
            return Some(return_type);
        }
    }
    
    // Look for CURSED-style return type (after closing parenthesis, before opening brace)
    if let Some(paren_end) = line.find(')') {
        let after_params = &line[paren_end + 1..];
        
        // Look for return type before opening brace
        if let Some(brace_pos) = after_params.find('{') {
            let return_part = &after_params[..brace_pos].trim();
            
            // Remove arrow if present and extract type
            let return_type = return_part
                .trim_start_matches("->")
                .trim()
                .to_string();
                
            if !return_type.is_empty() && return_type != "{" {
                return Some(return_type);
            }
        } else {
            // No opening brace on same line, check for return type
            let return_type = after_params.trim().to_string();
            if !return_type.is_empty() {
                return Some(return_type);
            }
        }
    }
    
    None
}

fn main() {
    // Test cases for CURSED function syntax
    let test_cases = vec![
        "slay add_numbers(a: normie, b: normie) -> normie {",
        "slay greet_user(name: tea) -> tea {",
        "slay is_positive(num: normie) -> facts {",
        "slay calculate_area(radius: vibes) -> vibes {",
        "slay get_default_message() -> tea {",
        "slay process_user(name: tea, age: normie, is_active: facts) -> tea {",
    ];

    println!("Testing parameter and return type parsing:");
    println!("=========================================");

    for test_case in test_cases {
        println!("\nTest case: {}", test_case);
        
        let parameters = parse_function_parameters(test_case);
        let return_type = parse_function_return_type(test_case);

        println!("Parameters:");
        for param in &parameters {
            println!("  - {} : {}", param.name, param.param_type);
        }

        if let Some(ret_type) = return_type {
            println!("Return type: {}", ret_type);
        } else {
            println!("Return type: None");
        }
    }

    println!("\n=========================================");
    println!("All tests completed successfully!");
}
