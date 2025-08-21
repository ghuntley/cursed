yeet "vibez"
yeet "networkz"

vibez.spill("=== NetworkZ Simple Test ===")

// Test URL parsing
sus test_url tea = "https://api.example.com/users"
sus host tea = networkz.parse_url_simple(test_url)
vibez.spill("Parsed host:", host)

// Test HTTP GET
sus response tea = networkz.http_get_simple("http://api.example.com/data")
sus status_code drip = networkz.get_status_code(response)
sus body tea = networkz.get_response_body(response)

vibez.spill("Status code:", status_code)
vibez.spill("Response body:", body)
vibez.spill("Is success:", networkz.is_success(status_code))

// Test parameter encoding
sus params []tea = ["name=John Doe", "city=New York"]
sus encoded tea = networkz.encode_params(params)
vibez.spill("Encoded params:", encoded)

// Test HTTP POST
sus post_response tea = networkz.http_post_simple("http://api.example.com/submit", "data=test")
sus post_status drip = networkz.get_status_code(post_response)
vibez.spill("POST status:", post_status)

vibez.spill("=== NetworkZ Test Completed ===")
