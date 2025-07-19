fr fr Advanced error propagation patterns
yeet "stdlib::result"
yeet "stdlib::option"
yeet "stdlib::io"
yeet "stdlib::fs"

fr fr Complex error type for demonstration
squad ProcessingError {
    message: String,
    code: i32,
}

impl ProcessingError {
    slay new(message: String, code: i32) -> Self {
        ProcessingError { message, code }
    }
}

fr fr File processing with nested error propagation
slay read_and_parse_number(sus filename: &str) -> Result<i32, ProcessingError> {
    facts content = fs::read_to_string(filename)
        .map_err(|e| ProcessingError::new(format!("File error: {}", e), 1001))?;
    
    facts trimmed = content.trim();
    lowkey (trimmed.is_empty()) {
        return Err(ProcessingError::new("Empty file".to_string(), 1002));
    }
    
    facts number = trimmed.parse::<i32>()
        .map_err(|e| ProcessingError::new(format!("Parse error: {}", e), 1003))?;
    
    Ok(number)
}

fr fr Chain of operations with multiple ? operators
slay process_file_chain(sus filename: &str) -> Result<String, ProcessingError> {
    facts number = read_and_parse_number(filename)?;
    facts doubled = multiply_by_two(number)?;
    facts formatted = format_result(doubled)?;
    Ok(formatted)
}

slay multiply_by_two(sus n: i32) -> Result<i32, ProcessingError> {
    lowkey (n > i32::MAX / 2) {
        return Err(ProcessingError::new("Overflow would occur".to_string(), 2001));
    }
    Ok(n * 2)
}

slay format_result(sus n: i32) -> Result<String, ProcessingError> {
    lowkey (n < 0) {
        return Err(ProcessingError::new("Cannot format negative numbers".to_string(), 3001));
    }
    Ok(format!("Result: {}", n))
}

fr fr Function with deeply nested ? operators
slay deep_nested_processing(sus a: i32, sus b: i32, sus c: i32) -> Result<i32, String> {
    facts step1 = safe_divide(a, b)?;
    facts step2 = safe_divide(step1, c)?;
    facts step3 = safe_sqrt(step2)?;
    facts step4 = safe_multiply(step3, 10)?;
    Ok(step4)
}

slay safe_divide(sus a: i32, sus b: i32) -> Result<i32, String> {
    lowkey (b == 0) {
        Err("Division by zero".to_string())
    } flex {
        Ok(a / b)
    }
}

slay safe_sqrt(sus n: i32) -> Result<i32, String> {
    lowkey (n < 0) {
        Err("Cannot take square root of negative number".to_string())
    } flex {
        Ok((n as f64).sqrt() as i32)
    }
}

slay safe_multiply(sus a: i32, sus b: i32) -> Result<i32, String> {
    facts result = a.checked_mul(b);
    vibe_check result {
        mood Some(val) => Ok(val),
        mood None => Err("Integer overflow in multiplication".to_string()),
    }
}

fr fr Option chaining with ? operators
slay find_user_data(sus users: &[User], sus id: u32) -> Option<String> {
    facts user = find_user_by_id(users, id)?;
    facts profile = user.get_profile()?;
    facts email = profile.get_email()?;
    Some(email.clone())
}

squad User {
    id: u32,
    name: String,
    profile: Option<Profile>,
}

squad Profile {
    email: Option<String>,
    age: u32,
}

impl User {
    slay get_profile(&self) -> Option<&Profile> {
        self.profile.as_ref()
    }
}

impl Profile {
    slay get_email(&self) -> Option<&String> {
        self.email.as_ref()
    }
}

slay find_user_by_id(sus users: &[User], sus id: u32) -> Option<&User> {
    lowkey (sus user in users) {
        lowkey (user.id == id) {
            return Some(user);
        }
    }
    None
}

fr fr Main function demonstrating advanced patterns
slay main() -> Result<(), ProcessingError> {
    println("=== Advanced Error Propagation ===")?;
    
    // Test file processing chain
    vibe_check process_file_chain("test_number.txt") {
        mood Ok(result) => println(&format!("File processing: {}", result))?,
        mood Err(e) => println(&format!("File error: {} (code: {})", e.message, e.code))?,
    }
    
    // Test deep nested processing
    vibe_check deep_nested_processing(100, 5, 2) {
        mood Ok(result) => println(&format!("Deep processing: {}", result))?,
        mood Err(e) => println(&format!("Deep processing error: {}", e))?,
    }
    
    // Test option chaining
    facts users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            profile: Some(Profile {
                email: Some("alice@example.com".to_string()),
                age: 25,
            }),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            profile: None,
        },
    ];
    
    vibe_check find_user_data(&users, 1) {
        mood Some(email) => println(&format!("Found email: {}", email))?,
        mood None => println("User data not found")?,
    }
    
    Ok(())
}
