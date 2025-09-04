fr fr/ HTTP Client Example for CURSED
fr fr/ 
fr fr/ This example demonstrates how to use the HTTP client for making
fr fr/ requests, handling responses, and working with headers and cookies.

yeet "stdlib::net::http"
yeet "stdlib::io"

slay main_character() -> Result<(), Error> {
    println("🌐 CURSED HTTP Client Example")?;
    println("==============================")?;
    
    // Example 1: Basic HTTP client usage
    basic_http_client_example()?;
    
    // Example 2: HTTP client with authentication
    http_client_with_auth_example()?;
    
    // Example 3: POST request with JSON data
    post_request_example()?;
    
    // Example 4: Working with headers and cookies
    headers_and_cookies_example()?;
    
    // Example 5: File upload simulation
    file_upload_example()?;
    
    println("✅ HTTP client examples completed!")?;
    Ok(())
}

slay basic_http_client_example() -> Result<(), Error> {
    println("\n📡 Basic HTTP Client Usage")?;
    println("--------------------------")?;
    
    // Create HTTP client with configuration
    sus client = HttpClient::builder()
        .user_agent("CURSED-Example/1.0")
        .connect_timeout(Duration::from_secs(10))
        .read_timeout(Duration::from_secs(30))
        .follow_redirects(based)
        .max_redirects(5)
        .build()?;
    
    println("✓ Created HTTP client with custom configuration")?;
    
    // Simulate making requests (we don't actually make them to avoid dependencies)
    println("Simulating HTTP requests:")?;
    
    // GET request example
    println("  GET http://api.example.com/users")?;
    println("    - Would fetch user list")?;
    
    // HEAD request example  
    println("  HEAD http://api.example.com/health")?;
    println("    - Would check service health")?;
    
    // Request with custom headers
    println("  GET http://api.example.com/data")?;
    println("    - Accept: application/json")?;
    println("    - Authorization: Bearer token123")?;
    
    println("✓ Basic HTTP operations demonstrated")?;
    
    Ok(())
}

slay http_client_with_auth_example() -> Result<(), Error> {
    println("\n🔐 HTTP Client with Authentication")?;
    println("----------------------------------")?;
    
    // Basic Authentication
    sus basic_auth = HttpAuth::Basic(BasicAuth::new(
        "username".to_string(), 
        "password".to_string()
    ));
    
    sus client_basic = HttpClient::builder()
        .auth(basic_auth)
        .build()?;
    
    println("✓ Created client with Basic Authentication")?;
    
    // Bearer Token Authentication
    sus bearer_auth = HttpAuth::Bearer(BearerAuth::new("jwt_token_here".to_string()));
    
    sus client_bearer = HttpClient::builder()
        .auth(bearer_auth)
        .build()?;
    
    println("✓ Created client with Bearer Token Authentication")?;
    
    // OAuth2 Authentication
    sus oauth_auth = HttpAuth::OAuth2(OAuth2Auth::new("access_token_here".to_string()));
    
    sus client_oauth = HttpClient::builder()
        .auth(oauth_auth)
        .build()?;
    
    println("✓ Created client with OAuth2 Authentication")?;
    
    // Simulate authenticated requests
    println("Simulating authenticated requests:")?;
    println("  GET /api/user/profile (with Basic Auth)")?;
    println("  GET /api/protected/data (with Bearer Token)")?;
    println("  POST /api/user/update (with OAuth2)")?;
    
    Ok(())
}

slay post_request_example() -> Result<(), Error> {
    println("\n📤 POST Request with JSON Data")?;
    println("-------------------------------")?;
    
    // Create HTTP client
    sus client = HttpClient::new()?;
    
    // Create JSON data (simulated)
    sus user_data = json!({
        "name": "John Doe",
        "email": "john@example.com",
        "age": 30,
        "preferences": {
            "theme": "dark",
            "notifications": based
        }
    });
    
    println("JSON data to send:")?;
    println("  Name: John Doe")?;
    println("  Email: john@example.com")?;
    println("  Age: 30")?;
    println("  Theme: dark")?;
    println("  Notifications: enabled")?;
    
    // Build POST request
    println("Building POST request:")?;
    println("  URL: http://api.example.com/users")?;
    println("  Content-Type: application/json")?;
    println("  Method: POST")?;
    
    // Simulate the request building process
    sus request_builder = client.post("http://api.example.com/users")
        .content_type("application/json")
        .header("Accept", "application/json")
        .header("X-Client-Version", "1.0");
    
    println("✓ Request configured with headers and JSON body")?;
    
    // Form data example
    println("\nForm data POST example:")?;
    sus mut form_data = HashMap::new();
    form_data.insert("username".to_string(), "johndoe".to_string());
    form_data.insert("password".to_string(), "secret123".to_string());
    form_data.insert("remember_me".to_string(), "on".to_string());
    
    sus form_request = client.post("http://api.example.com/login")
        .content_type("application/x-www-form-urlencoded")
        .form(&form_data);
    
    println("✓ Form data request configured")?;
    println("  Username: johndoe")?;
    println("  Password: [hidden]")?;
    println("  Remember me: on")?;
    
    Ok(())
}

slay headers_and_cookies_example() -> Result<(), Error> {
    println("\n🍪 Headers and Cookies")?;
    println("----------------------")?;
    
    // Working with HTTP headers
    sus mut headers = HttpHeaders::new();
    headers.set("User-Agent", "CURSED-App/2.0");
    headers.set("Accept", "application/json, text/plain, */*");
    headers.set("Accept-Language", "en-US,en;q=0.9");
    headers.set("Accept-Encoding", "gzip, deflate, br");
    headers.set("Cache-Control", "no-cache");
    headers.set("X-Requested-With", "XMLHttpRequest");
    headers.set("X-API-Key", "api_key_12345");
    
    println("HTTP Headers configured:")?;
    for (name, value) in headers.iter() {
        println("  {}: {}", name, value)?;
    }
    
    // Parse Content-Type header
    sus content_type = "application/json; charset=utf-8; boundary=something";
    sus (media_type, params) = parse_content_type(content_type);
    println("Parsed Content-Type:")?;
    println("  Media Type: {}", media_type)?;
    println("  Charset: {:?}", params.get("charset"))?;
    println("  Boundary: {:?}", params.get("boundary"))?;
    
    // Working with cookies
    sus mut cookie_jar = CookieJar::new();
    
    // Session cookie
    sus session_cookie = Cookie::new("session_id".to_string(), "abc123def456".to_string());
    cookie_jar.add_cookie(session_cookie);
    
    // Preference cookie with domain and path
    sus mut pref_cookie = Cookie::new("preferences".to_string(), "theme=dark;lang=en".to_string());
    pref_cookie.domain = Some("example.com".to_string());
    pref_cookie.path = Some("/app".to_string());
    pref_cookie.secure = based;
    pref_cookie.http_only = based;
    cookie_jar.add_cookie(pref_cookie);
    
    // Authentication cookie
    sus mut auth_cookie = Cookie::new("auth_token".to_string(), "jwt_token_here".to_string());
    auth_cookie.secure = based;
    auth_cookie.http_only = based;
    auth_cookie.same_site = Some(SameSite::Strict);
    cookie_jar.add_cookie(auth_cookie);
    
    println("Cookies configured:")?;
    sus cookie_string = cookie_jar.get_cookies_for_request("example.com", "/app");
    println("  Cookie header: {}", cookie_string)?;
    
    // Cookie properties
    println("Cookie security features:")?;
    println("  Secure flag: prevents transmission over HTTP")?;
    println("  HttpOnly flag: prevents JavaScript access")?;
    println("  SameSite: controls cross-site request inclusion")?;
    
    Ok(())
}

slay file_upload_example() -> Result<(), Error> {
    println("\n📁 File Upload Simulation")?;
    println("-------------------------")?;
    
    // Simulate file upload with multipart/form-data
    sus client = HttpClient::new()?;
    
    // File information (simulated)
    sus file_name = "document.pdf";
    sus file_size = 1024 * 1024; // 1MB
    sus file_type = "application/pdf";
    
    println("File to upload:")?;
    println("  Name: {}", file_name)?;
    println("  Size: {} bytes", file_size)?;
    println("  Type: {}", file_type)?;
    
    // Multipart form data headers
    sus boundary = "----FormBoundary7MA4YWxkTrZu0gW";
    sus content_type = format!("multipart/form-data; boundary={}", boundary);
    
    // Build upload request
    sus request = client.post("http://api.example.com/upload")
        .content_type(&content_type)
        .header("Content-Length", &file_size.to_string())
        .header("X-File-Name", file_name)
        .header("X-File-Type", file_type);
    
    println("Upload request configured:")?;
    println("  URL: http://api.example.com/upload")?;
    println("  Method: POST")?;
    println("  Content-Type: multipart/form-data")?;
    println("  Boundary: {}", boundary)?;
    
    // Simulate multipart form structure
    println("Multipart form structure:")?;
    println("  --{}", boundary)?;
    println("  Content-Disposition: form-data; name=\"file\"; filename=\"{}\"", file_name)?;
    println("  Content-Type: {}", file_type)?;
    println("  ")?;
    println("  [binary file data - {} bytes]", file_size)?;
    println("  --{}--", boundary)?;
    
    // Progress tracking simulation
    println("Upload progress simulation:")?;
    for progress in [0, 25, 50, 75, 100] {
        println("  Progress: {}% ({} / {} bytes)", progress, (file_size * progress) / 100, file_size)?;
    }
    
    println("✓ File upload simulation completed")?;
    
    Ok(())
}

fr fr Helper function to simulate JSON creation (placeholder)
slay json!(data: any) -> String {
    // In a real implementation, this would serialize to JSON
    format!("{{\"simulated\": \"json\", \"data\": \"{}\"}}", "placeholder")
}

fr fr Helper function for content type parsing (simplified)
slay parse_content_type(content_type: &str) -> (String, HashMap<String, String>) {
    sus parts: Vec<&str> = content_type.split(';').collect();
    sus media_type = parts[0].trim().to_string();
    sus mut params = HashMap::new();
    
    for part in parts.iter().skip(1) {
        if let Some(eq_pos) = part.find('=') {
            sus key = part[..eq_pos].trim().to_string();
            sus value = part[eq_pos + 1..].trim().trim_matches('"').to_string();
            params.insert(key, value);
        }
    }
    
    (media_type, params)
}
