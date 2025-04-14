vibe main

fr fr Test file for the web_vibez standard library package

slay main() {
    vibez.spill("Testing web_vibez (HTTP client/server) module")
    
    fr fr Test setting timeout
    timeout_ms := web_vibez.client_timeout(2000)
    vibez.spill("Set timeout to:", timeout_ms)
    
    fr fr Test getting timeout
    current := web_vibez.client_timeout()
    vibez.spill("Current timeout:", current)
    assert(current == 2000, "Expected timeout to be 2000")
    
    fr fr Test GET request with mock mode
    response := web_vibez.get("https://example.com", true)
    vibez.spill("GET mock response status:", response["status"])
    assert(response["status"] == 200, "Expected status code 200")
    
    fr fr Test POST request with mock mode
    data := {"name": "CURSED User", "message": "Hello World!"}
    post_response := web_vibez.post("https://example.com/api", data, true)
    vibez.spill("POST mock response status:", post_response["status"])
    assert(post_response["status"] == 201, "Expected status code 201")
    
    fr fr Test HEAD request with mock mode
    head_response := web_vibez.head("https://example.com", true)
    vibez.spill("HEAD mock response status:", head_response["status"])
    assert(head_response["status"] == 200, "Expected status code 200")
    assert(head_response["headers"] != null, "Expected headers to exist")
    
    fr fr Ensure HEAD has no body as expected
    has_body := "body" in head_response
    vibez.spill("HEAD response has body:", has_body)
    assert(!has_body, "HEAD response should not have a body")
    
    fr fr Test DELETE request with mock mode
    delete_response := web_vibez.delete("https://example.com/resource/123", true)
    vibez.spill("DELETE mock response status:", delete_response["status"])
    assert(delete_response["status"] == 204, "Expected status code 204")
    
    vibez.spill("All web_vibez tests passed!")
}

fr fr Assert function to make testing easier
slay assert(condition, message) {
    if !condition {
        vibez.spill("Assertion failed:", message)
        vibe_life.exit(1)
    }
}