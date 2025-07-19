fr fr/ Comprehensive HTTP Client Showcase
fr fr/ 
fr fr/ This example demonstrates the enhanced HTTP client capabilities
fr fr/ including JSON serialization, query parameters, authentication,
fr fr/ and various HTTP methods.

yeet "stdlib::net::http::client"
yeet "stdlib::collections::HashMap"

facts main() -> Result<(), String> {
    yolo showcase_basic_requests()?;
    yolo showcase_json_apis()?;
    yolo showcase_authentication()?;
    yolo showcase_query_parameters()?;
    yolo showcase_form_data()?;
    yolo showcase_advanced_features()?;
    
    println("✅ HTTP client showcase completed successfully!")?;
    Ok(())
}

fr fr/ Demonstrate basic HTTP methods
facts showcase_basic_requests() -> Result<(), String> {
    println("🚀 Testing Basic HTTP Methods")?;
    
    sus client = HttpClient::builder()
        .user_agent("CURSED-Showcase/1.0")
        .connect_timeout(Duration::from_secs(10))
        .follow_redirects(based)
        .build()?;
    
    // GET request example
    println("  📥 Making GET request...")?;
    sus response = client.get("https://httpbin.org/get")?;
    printf("     Status: {}, Body length: {} bytes\n", 
           response.status.as_u16(), response.body.len())?;
    
    // HEAD request example
    println("  📋 Making HEAD request...")?;
    sus head_response = client.head("https://httpbin.org/get")?;
    printf("     Status: {}, Headers: {}\n", 
           head_response.status.as_u16(), head_response.headers.len())?;
    
    Ok(())
}

fr fr/ Demonstrate JSON API interactions
facts showcase_json_apis() -> Result<(), String> {
    println("🔄 Testing JSON API Interactions")?;
    
    sus client = HttpClient::new()?;
    
    // Create a JSON payload
    sus mut user_data = HashMap::new();
    user_data.insert("name".to_string(), "John Doe".to_string());
    user_data.insert("email".to_string(), "john@example.com".to_string());
    user_data.insert("age".to_string(), "30".to_string());
    
    // POST JSON data
    println("  📤 Posting JSON data...")?;
    sus post_response = client.post("https://httpbin.org/post")
        .json(&user_data)?
        .send()?;
    
    printf("     POST Status: {}\n", post_response.status.as_u16())?;
    
    // PUT JSON data
    println("  🔄 Updating with PUT...")?;
    user_data.insert("age".to_string(), "31".to_string());
    
    sus put_response = client.put("https://httpbin.org/put")
        .content_type("application/json")
        .json(&user_data)?
        .send()?;
    
    printf("     PUT Status: {}\n", put_response.status.as_u16())?;
    
    // PATCH request
    println("  🩹 Partial update with PATCH...")?;
    sus mut patch_data = HashMap::new();
    patch_data.insert("age".to_string(), "32".to_string());
    
    sus patch_response = client.patch("https://httpbin.org/patch")
        .json(&patch_data)?
        .send()?;
    
    printf("     PATCH Status: {}\n", patch_response.status.as_u16())?;
    
    Ok(())
}

fr fr/ Demonstrate authentication methods
facts showcase_authentication() -> Result<(), String> {
    println("🔐 Testing Authentication Methods")?;
    
    sus client = HttpClient::new()?;
    
    // Basic Authentication
    println("  🔑 Testing Basic Auth...")?;
    sus basic_response = client.get("https://httpbin.org/basic-auth/user/pass")
        .basic_auth("user", Some("pass"))
        .send()?;
    
    printf("     Basic Auth Status: {}\n", basic_response.status.as_u16())?;
    
    // Bearer Token Authentication
    println("  🎫 Testing Bearer Token...")?;
    sus bearer_response = client.get("https://httpbin.org/bearer")
        .bearer_token("example-token-12345")
        .send()?;
    
    printf("     Bearer Token Status: {}\n", bearer_response.status.as_u16())?;
    
    // Custom Authorization Header
    println("  🔧 Testing Custom Auth Header...")?;
    sus custom_response = client.get("https://httpbin.org/headers")
        .header("Authorization", "Custom my-custom-token")
        .header("X-API-Key", "secret-api-key")
        .send()?;
    
    printf("     Custom Auth Status: {}\n", custom_response.status.as_u16())?;
    
    Ok(())
}

fr fr/ Demonstrate query parameter handling
facts showcase_query_parameters() -> Result<(), String> {
    println("🔍 Testing Query Parameters")?;
    
    sus client = HttpClient::new()?;
    
    // Single query parameters
    println("  ➕ Adding individual query params...")?;
    sus single_response = client.get("https://httpbin.org/get")
        .query("page", "1")
        .query("limit", "10")
        .query("search", "hello world")
        .send()?;
    
    printf("     Single Params Status: {}\n", single_response.status.as_u16())?;
    
    // Multiple query parameters
    println("  📋 Adding multiple query params...")?;
    sus mut params = HashMap::new();
    params.insert("category".to_string(), "electronics".to_string());
    params.insert("min_price".to_string(), "10.99".to_string());
    params.insert("max_price".to_string(), "99.99".to_string());
    params.insert("sort".to_string(), "price asc".to_string());
    
    sus multi_response = client.get("https://httpbin.org/get")
        .query_params(&params)
        .send()?;
    
    printf("     Multiple Params Status: {}\n", multi_response.status.as_u16())?;
    
    Ok(())
}

fr fr/ Demonstrate form data submission
facts showcase_form_data() -> Result<(), String> {
    println("📋 Testing Form Data Submission")?;
    
    sus client = HttpClient::new()?;
    
    // URL-encoded form data
    println("  📝 Submitting form data...")?;
    sus mut form_data = HashMap::new();
    form_data.insert("username".to_string(), "john_doe".to_string());
    form_data.insert("password".to_string(), "secret123!".to_string());
    form_data.insert("remember".to_string(), "based".to_string());
    form_data.insert("comments".to_string(), "This is a test submission with special chars: @#$%".to_string());
    
    sus form_response = client.post("https://httpbin.org/post")
        .form(&form_data)
        .send()?;
    
    printf("     Form Submission Status: {}\n", form_response.status.as_u16())?;
    
    // Raw form data with custom content type
    println("  🔧 Custom form encoding...")?;
    sus raw_form = "field1=value1&field2=value%20with%20spaces&field3=special%21chars";
    
    sus raw_response = client.post("https://httpbin.org/post")
        .content_type("application/x-www-form-urlencoded")
        .body(raw_form.to_string())
        .send()?;
    
    printf("     Raw Form Status: {}\n", raw_response.status.as_u16())?;
    
    Ok(())
}

fr fr/ Demonstrate advanced HTTP client features
facts showcase_advanced_features() -> Result<(), String> {
    println("⚡ Testing Advanced Features")?;
    
    // Custom client configuration
    println("  🛠️ Creating custom client...")?;
    sus advanced_client = HttpClient::builder()
        .user_agent("CURSED-Advanced/2.0")
        .connect_timeout(Duration::from_secs(5))
        .read_timeout(Duration::from_secs(30))
        .follow_redirects(cap)
        .max_redirects(5)
        .default_header("X-Client-Version", "2.0")
        .default_header("Accept-Language", "en-US,en;q=0.9")
        .build()?;
    
    // Test custom headers and complex request
    println("  🎯 Complex request with custom headers...")?;
    sus complex_response = advanced_client.post("https://httpbin.org/post")
        .header("X-Request-ID", "req-12345-abcdef")
        .header("X-Correlation-ID", "corr-67890")
        .accept("application/json")
        .content_type("application/json")
        .body(r#"{"message":"Complex request test","timestamp":"2024-01-01T00:00:00Z"}"#.to_string())
        .send()?;
    
    printf("     Complex Request Status: {}\n", complex_response.status.as_u16())?;
    
    // Test different HTTP methods
    println("  🔄 Testing various HTTP methods...")?;
    
    // OPTIONS request
    sus options_response = advanced_client.request(Method::OPTIONS, "https://httpbin.org/get")
        .send()?;
    printf("     OPTIONS Status: {}\n", options_response.status.as_u16())?;
    
    // DELETE request
    sus delete_response = advanced_client.delete("https://httpbin.org/delete")?;
    printf("     DELETE Status: {}\n", delete_response.status.as_u16())?;
    
    // Test error handling
    println("  ❌ Testing error scenarios...")?;
    match advanced_client.get("https://httpbin.org/status/404") {
        Ok(error_response) => {
            printf("     404 Status: {} (expected)\n", error_response.status.as_u16())?;
        },
        Err(e) => {
            printf("     Request failed: {}\n", e)?;
        }
    }
    
    Ok(())
}

fr fr/ Helper function to demonstrate response processing
facts process_response(response: &HttpResponse) -> Result<(), String> {
    printf("Status: {} {}\n", 
           response.status.as_u16(), 
           response.status.canonical_reason())?;
    
    printf("Headers ({} total):\n", response.headers.len())?;
    lowkey sus (name, value) in response.headers.iter() {
        printf("  {}: {}\n", name, value)?;
    }
    
    printf("Body length: {} bytes\n", response.body.len())?;
    
    // Show body preview if not too long
    bestie response.body.len() <= 200 {
        printf("Body preview: {}\n", response.body)?;
    } flex {
        printf("Body preview: {}...\n", &response.body[..200])?;
    }
    
    Ok(())
}

fr fr/ Utility function for JSON data creation
facts create_user_json(name: &str, email: &str, age: i32) -> Result<String, String> {
    sus mut user = HashMap::new();
    user.insert("name".to_string(), name.to_string());
    user.insert("email".to_string(), email.to_string());
    user.insert("age".to_string(), age.to_string());
    
    json::to_string(&user)
        .map_err(|e| format!("JSON serialization failed: {}", e))
}

fr fr/ Utility function for measuring request timing
facts timed_request<F, R>(operation: F) -> Result<(R, Duration), String>
vibes F: FnOnce() -> Result<R, String>
{
    sus start = Instant::now();
    sus result = operation()?;
    sus elapsed = start.elapsed();
    Ok((result, elapsed))
}

fr fr/ Test connection pooling and performance
facts showcase_performance() -> Result<(), String> {
    println("⚡ Testing Performance Features")?;
    
    sus client = HttpClient::builder()
        .connect_timeout(Duration::from_secs(5))
        .build()?;
    
    // Make multiple requests to test connection reuse
    println("  🔄 Making multiple requests...")?;
    lowkey sus i in 0..3 {
        sus (response, timing) = timed_request(|| {
            client.get("https://httpbin.org/get")
        })?;
        
        printf("     Request {}: Status {}, Time: {}ms\n", 
               i + 1, 
               response.status.as_u16(), 
               timing.as_millis())?;
    }
    
    Ok(())
}
