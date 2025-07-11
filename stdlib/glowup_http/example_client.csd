yeet "glowup_http"

# Example HTTP Client using glowup_http framework

# Initialize the framework
glowup_http_main()

# Create client configuration
sus client_config ClientConfig
client_config.timeout = 30
client_config.max_redirects = 5
client_config.user_agent = "glowup_http_client/1.0"
client_config.follow_redirects = based
client_config.verify_ssl = based

vibez.spill("HTTP Client Examples")
vibez.spill("==================")

# Test GET request
vibez.spill("Testing GET request...")
sus get_response HttpResponse = http_client_get("https://api.example.com/users")
vibez.spill("GET Response Status: " + http_int_to_string(get_response.status_code))
vibez.spill("GET Response Body: " + get_response.body)
vibez.spill("GET Response Content-Type: " + get_response.content_type)

# Test POST request
vibez.spill("Testing POST request...")
sus post_data tea = "{\"name\": \"John Doe\", \"email\": \"john@example.com\", \"age\": 30}"
sus post_response HttpResponse = http_client_post("https://api.example.com/users", post_data)
vibez.spill("POST Response Status: " + http_int_to_string(post_response.status_code))
vibez.spill("POST Response Body: " + post_response.body)

# Test PUT request
vibez.spill("Testing PUT request...")
sus put_data tea = "{\"name\": \"Jane Smith\", \"email\": \"jane@example.com\", \"age\": 25}"
sus put_response HttpResponse = http_client_put("https://api.example.com/users/1", put_data)
vibez.spill("PUT Response Status: " + http_int_to_string(put_response.status_code))
vibez.spill("PUT Response Body: " + put_response.body)

# Test DELETE request
vibez.spill("Testing DELETE request...")
sus delete_response HttpResponse = http_client_delete("https://api.example.com/users/1")
vibez.spill("DELETE Response Status: " + http_int_to_string(delete_response.status_code))
vibez.spill("DELETE Response Body: " + delete_response.body)

# Test JSON utilities
vibez.spill("Testing JSON utilities...")
sus json_data tea = "{\"message\": \"Hello World\", \"count\": 42}"
sus parsed_json tea = json_parse(json_data)
vibez.spill("Parsed JSON: " + parsed_json)

sus json_string tea = json_stringify("test_object")
vibez.spill("JSON String: " + json_string)

# Test URL utilities
vibez.spill("Testing URL utilities...")
sus test_url tea = "https://example.com/api/v1/users?page=1&limit=10"
sus parsed_url tea = url_parse(test_url)
vibez.spill("Parsed URL: " + parsed_url)

sus encoded_text tea = url_encode("hello world & special chars")
vibez.spill("URL Encoded: " + encoded_text)

sus decoded_text tea = url_decode("hello%20world")
vibez.spill("URL Decoded: " + decoded_text)

# Test multiple requests
vibez.spill("Testing multiple API calls...")
bestie i := 0; i < 3; i++ {
    sus api_url tea = "https://api.example.com/data/" + http_int_to_string(i)
    sus api_response HttpResponse = http_client_get(api_url)
    vibez.spill("API Call " + http_int_to_string(i) + " Status: " + http_int_to_string(api_response.status_code))
}

# Test error handling
vibez.spill("Testing error responses...")
sus error_response HttpResponse = http_client_get("https://api.example.com/nonexistent")
vibez.spill("Error Response Status: " + http_int_to_string(error_response.status_code))

# Test different content types
vibez.spill("Testing content types...")
sus xml_response HttpResponse = http_client_get("https://api.example.com/xml")
sus json_response HttpResponse = http_client_get("https://api.example.com/json")
sus text_response HttpResponse = http_client_get("https://api.example.com/text")

vibez.spill("XML Response: " + xml_response.body)
vibez.spill("JSON Response: " + json_response.body)
vibez.spill("Text Response: " + text_response.body)

vibez.spill("HTTP Client examples completed successfully!")
