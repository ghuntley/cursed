// Network service functionality
import "../data/models.csd" as Models
import "../utils/math_utils.csd" as MathUtils
export fetch_data, post_data, NetworkError, HttpClient

// HTTP client for network operations
struct HttpClient {
    base_url: string,
    timeout: int,
    headers: Map<string, string>
}

impl HttpClient {
    func new(base_url: string) -> HttpClient {
        let mut headers = Map.new()
        headers.insert("User-Agent", "CURSED-HTTP-Client/1.0")
        headers.insert("Accept", "application/json")
        
        return HttpClient {
            base_url: base_url,
            timeout: 30000, // 30 seconds
            headers: headers
        }
    }
    
    func add_header(&mut self, key: string, value: string) {
        self.headers.insert(key, value)
    }
    
    func set_timeout(&mut self, timeout_ms: int) {
        self.timeout = timeout_ms
    }
}

// Network error types
enum NetworkError {
    ConnectionFailed(string),
    TimeoutError,
    InvalidResponse(string),
    NotFound,
    ServerError(int, string)
}

impl NetworkError {
    func to_string(&self) -> string {
        match self {
            NetworkError::ConnectionFailed(msg) => "Connection failed: " + msg,
            NetworkError::TimeoutError => "Request timed out",
            NetworkError::InvalidResponse(msg) => "Invalid response: " + msg,
            NetworkError::NotFound => "Resource not found",
            NetworkError::ServerError(code, msg) => "Server error " + code.to_string() + ": " + msg
        }
    }
}

// Public API functions
async func fetch_data(url: string) -> Result<string, NetworkError> {
    println("Fetching data from: " + url)
    
    // Simulate network request with random delay
    let delay = MathUtils.random_in_range(100, 500)
    await sleep(delay)
    
    // Simulate different response scenarios
    let rand_val = MathUtils.random_in_range(1, 100)
    
    if rand_val < 5 {
        return Err(NetworkError::ConnectionFailed("Network unreachable"))
    }
    
    if rand_val < 10 {
        return Err(NetworkError::TimeoutError)
    }
    
    if rand_val < 15 {
        return Err(NetworkError::NotFound)
    }
    
    // Simulate successful response
    let response_data = generate_mock_response(url)
    return Ok(response_data)
}

async func post_data(url: string, data: string) -> Result<string, NetworkError> {
    println("Posting data to: " + url)
    println("Data: " + data)
    
    // Simulate network request
    let delay = MathUtils.random_in_range(200, 800)
    await sleep(delay)
    
    // Simulate response
    let response = "{\"status\":\"success\",\"id\":" + MathUtils.random_in_range(1000, 9999).to_string() + "}"
    return Ok(response)
}

// Helper functions
func generate_mock_response(url: string) -> string {
    if url.contains("users") {
        return generate_users_response()
    } else if url.contains("products") {
        return generate_products_response()
    } else {
        return "{\"message\":\"Mock response\",\"timestamp\":" + get_timestamp().to_string() + "}"
    }
}

func generate_users_response() -> string {
    let users = [
        Models.User.new("Alice", 25),
        Models.User.new("Bob", 30),
        Models.User.new("Charlie", 35)
    ]
    
    let mut response = "{\"users\":["
    for (i, user) in users.iter().enumerate() {
        if i > 0 {
            response += ","
        }
        response += user.to_json()
    }
    response += "]}"
    return response
}

func generate_products_response() -> string {
    let products = [
        Models.Product.new(1, "Laptop", 999.99, "Electronics"),
        Models.Product.new(2, "Mouse", 29.99, "Electronics"),
        Models.Product.new(3, "Keyboard", 79.99, "Electronics")
    ]
    
    let mut response = "{\"products\":["
    for (i, product) in products.iter().enumerate() {
        if i > 0 {
            response += ","
        }
        response += "{\"id\":" + product.id.to_string() + ",\"name\":\"" + product.name + "\",\"price\":" + product.price.to_string() + "}"
    }
    response += "]}"
    return response
}

func get_timestamp() -> int {
    // Mock timestamp - in real implementation would use system time
    return 1640995200 + MathUtils.random_in_range(0, 86400)
}

// Async helper function
async func sleep(ms: int) {
    // Mock sleep implementation
    let start = get_timestamp()
    while get_timestamp() - start < ms / 1000 {
        // Busy wait for simulation
    }
}
