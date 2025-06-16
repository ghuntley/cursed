//! HTTP client demonstration using the GlowUpHTTP client

use std::collections::HashMap;
use cursed::stdlib::glowup_http::client::{VibeClient, AuthType};
use cursed::stdlib::glowup_http::error::GlowUpResult;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> GlowUpResult<()> {
    println!("GlowUpHTTP Client Demo");
    println!("=====================");

    // Create a new client
    let client = VibeClient::new()?;
    println!("✅ Created HTTP client");

    // Configure client with custom settings
    let client = client
        .timeout(std::time::Duration::from_secs(10))?
        .default_header("X-Custom-Header", "GlowUpHTTP-Demo")
        .follow_redirects(true, 5)?;
    println!("✅ Configured client settings");

    // Example 1: Simple GET request
    println!("\n1. Simple GET request:");
    match client.get("https://httpbin.org/get").send_sync() {
        Ok(response) => {
            println!("   Status: {}", response.status);
            println!("   Body length: {} bytes", response.body.len());
            if let Ok(json) = response.parse_json::<Value>() {
                if let Some(headers) = json.get("headers") {
                    println!("   Request headers seen by server: {}", headers);
                }
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    // Example 2: POST with JSON
    println!("\n2. POST request with JSON:");
    let json_data = json!({
        "name": "GlowUpHTTP",
        "version": "1.0",
        "features": ["async", "sync", "builder-pattern"]
    });

    match client.post("https://httpbin.org/post")
        .json(&json_data)
        .send_sync() {
        Ok(response) => {
            println!("   Status: {}", response.status);
            if let Ok(response_json) = response.parse_json::<Value>() {
                if let Some(sent_json) = response_json.get("json") {
                    println!("   JSON data received by server: {}", sent_json);
                }
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    // Example 3: Basic authentication
    println!("\n3. Basic authentication:");
    match client.get("https://httpbin.org/basic-auth/user/pass")
        .basic_auth("user", "pass")
        .send_sync() {
        Ok(response) => {
            println!("   Status: {} (should be 200 OK)", response.status);
            if let Ok(json) = response.parse_json::<Value>() {
                println!("   Authentication result: {}", json);
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    // Example 4: Custom headers and query parameters
    println!("\n4. Custom headers and query parameters:");
    let mut query_params = HashMap::new();
    query_params.insert("param1".to_string(), "value1".to_string());
    query_params.insert("param2".to_string(), "value2".to_string());

    match client.get("https://httpbin.org/get")
        .header("User-Agent", "GlowUpHTTP/1.0 Custom")
        .header("X-API-Key", "demo-key-12345")
        .queries(query_params)
        .send_sync() {
        Ok(response) => {
            println!("   Status: {}", response.status);
            if let Ok(json) = response.parse_json::<Value>() {
                if let Some(args) = json.get("args") {
                    println!("   Query parameters: {}", args);
                }
                if let Some(headers) = json.get("headers") {
                    println!("   Custom headers: {}", headers);
                }
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    // Example 5: Form data submission
    println!("\n5. Form data submission:");
    let mut form_data = HashMap::new();
    form_data.insert("username".to_string(), "testuser".to_string());
    form_data.insert("email".to_string(), "test@example.com".to_string());

    match client.post("https://httpbin.org/post")
        .form(form_data)
        .send_sync() {
        Ok(response) => {
            println!("   Status: {}", response.status);
            if let Ok(json) = response.parse_json::<Value>() {
                if let Some(form) = json.get("form") {
                    println!("   Form data received: {}", form);
                }
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    // Example 6: Bearer token authentication
    println!("\n6. Bearer token authentication:");
    match client.get("https://httpbin.org/bearer")
        .bearer_auth("demo-token-12345")
        .send_sync() {
        Ok(response) => {
            println!("   Status: {} (should be 200 if token is valid)", response.status);
            if let Ok(json) = response.parse_json::<Value>() {
                println!("   Token verification result: {}", json);
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    // Example 7: Timeout handling
    println!("\n7. Timeout handling:");
    match client.get("https://httpbin.org/delay/15")  // 15 second delay
        .timeout(std::time::Duration::from_secs(5))     // 5 second timeout
        .send_sync() {
        Ok(response) => {
            println!("   Unexpected success: {}", response.status);
        }
        Err(e) => {
            println!("   Expected timeout error: {}", e);
        }
    }

    // Example 8: Error handling
    println!("\n8. Error handling:");
    match client.get("https://httpbin.org/status/404").send_sync() {
        Ok(response) => {
            println!("   Status: {} (404 Not Found)", response.status);
            println!("   Body: {}", String::from_utf8_lossy(&response.body));
        }
        Err(e) => println!("   Error: {}", e),
    }

    // Example 9: Using the builder pattern
    println!("\n9. Advanced client builder:");
    let advanced_client = VibeClient::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("GlowUpHTTP-Advanced/2.0")
        .default_header("X-Client-Type", "Demo")
        .redirect_policy(true, 3)
        .gzip(true)
        .build()?;

    match advanced_client.get_simple("https://httpbin.org/gzip") {
        Ok(response) => {
            println!("   Status: {}", response.status);
            println!("   Compression handled automatically");
            if let Ok(json) = response.parse_json::<Value>() {
                if let Some(gzipped) = json.get("gzipped") {
                    println!("   Response was gzipped: {}", gzipped);
                }
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n✅ HTTP client demo completed!");
    Ok(())
}
