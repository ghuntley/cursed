use cursed::stdlib::web_vibez::client::{HttpClient, HttpError};
use std::collections::HashMap;

fn main() -> Result<(), HttpError> {
    println!("CURSED HTTP Client Demo");
    
    // Create a new HTTP client
    let client = HttpClient::new()
        .with_base_url("https://httpbin.org".to_string())
        .with_user_agent("CURSED-Demo/1.0".to_string());

    // Add some default headers
    let mut headers = HashMap::new();
    headers.insert("X-Custom-Header".to_string(), "CURSED-Value".to_string());
    
    let client = client.with_headers(headers);

    println!("Making GET request to /json...");
    
    // Make a GET request
    match client.get("/json").send() {
        Ok(response) => {
            println!("Response Status: {}", response.status);
            println!("Response Headers: {:?}", response.headers);
            
            if let Ok(body) = response.text() {
                println!("Response Body: {}", body);
            }
            
            if response.is_success() {
                println!("✅ Request succeeded!");
            } else {
                println!("❌ Request failed with status {}", response.status);
            }
        }
        Err(e) => {
            println!("❌ Request failed with error: {}", e);
        }
    }

    println!("\nMaking POST request with JSON body...");
    
    // Make a POST request with JSON body
    let json_data = r#"{"name": "CURSED", "type": "programming_language"}"#;
    
    match client.post("/post")
        .header("Content-Type".to_string(), "application/json".to_string())
        .json(json_data)
        .send() {
        Ok(response) => {
            println!("POST Response Status: {}", response.status);
            if let Ok(body) = response.text() {
                println!("POST Response Body: {}", body);
            }
        }
        Err(e) => {
            println!("❌ POST request failed: {}", e);
        }
    }

    println!("\nTesting form data submission...");
    
    // Test form data
    let mut form_data = HashMap::new();
    form_data.insert("language".to_string(), "CURSED".to_string());
    form_data.insert("author".to_string(), "Gen Z Developers".to_string());
    
    match client.post("/post")
        .form(&form_data)
        .send() {
        Ok(response) => {
            println!("Form POST Status: {}", response.status);
            if let Ok(body) = response.text() {
                println!("Form POST Body: {}", body);
            }
        }
        Err(e) => {
            println!("❌ Form POST failed: {}", e);
        }
    }

    println!("\nTesting authentication...");
    
    // Test authentication
    match client.get("/basic-auth/user/pass")
        .basic_auth("user", "pass")
        .send() {
        Ok(response) => {
            println!("Auth Response Status: {}", response.status);
            if response.is_success() {
                println!("✅ Authentication successful!");
            } else {
                println!("❌ Authentication failed");
            }
        }
        Err(e) => {
            println!("❌ Auth request failed: {}", e);
        }
    }

    println!("\nHTTP Client Demo completed! 🎉");
    Ok(())
}
