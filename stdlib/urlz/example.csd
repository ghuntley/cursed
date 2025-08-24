fr fr CURSED URL Parsing Package (urlz) - Example Usage
fr fr Demonstrates all major features of the URL parsing library

yeet "urlz"
yeet "vibez"

fr fr ===== BASIC URL PARSING EXAMPLE =====

slay demo_basic_parsing() {
    vibez.spill("=== Basic URL Parsing ===")
    
    sus url tea = "https://api.example.com:8443/v1/users/123?include=profile&format=json#user-details"
    sus parsed urlz.URL = urlz.parse_url(url)
    
    vibez.spill("Original URL:", url)
    vibez.spill("Scheme:", parsed.scheme)
    vibez.spill("Host:", parsed.host)
    vibez.spill("Port:", parsed.port)
    vibez.spill("Path:", parsed.path)
    vibez.spill("Query:", parsed.query)
    vibez.spill("Fragment:", parsed.fragment)
    vibez.spill("Valid:", parsed.is_valid)
    vibez.spill("")
}

fr fr ===== URL BUILDING EXAMPLE =====

slay demo_url_building() {
    vibez.spill("=== URL Building ===")
    
    fr fr Create URL from components
    sus api_url urlz.URL = urlz.URL{
        scheme: "https",
        host: "api.myapp.com",
        port: 0,  fr fr Use default port
        path: "/v2/products",
        query: "category=electronics&sort=price",
        fragment: "results",
        username: "",
        password: "",
        is_valid: based
    }
    
    sus built_url tea = urlz.build_url(api_url)
    vibez.spill("Built URL:", built_url)
    
    fr fr Verify by parsing it back
    sus reparsed urlz.URL = urlz.parse_url(built_url)
    vibez.spill("Reparsed scheme:", reparsed.scheme)
    vibez.spill("Reparsed host:", reparsed.host)
    vibez.spill("")
}

fr fr ===== URL ENCODING/DECODING EXAMPLE =====

slay demo_encoding_decoding() {
    vibez.spill("=== URL Encoding/Decoding ===")
    
    sus original tea = "Hello World! @#$%^&*()"
    sus encoded tea = urlz.url_encode(original)
    sus decoded tea = urlz.url_decode(encoded)
    
    vibez.spill("Original:", original)
    vibez.spill("Encoded:", encoded)
    vibez.spill("Decoded:", decoded)
    
    fr fr Demonstrate query parameter encoding
    sus param_value tea = "user name with spaces"
    sus encoded_param tea = urlz.url_encode(param_value)
    vibez.spill("Query param value:", param_value)
    vibez.spill("Encoded for URL:", encoded_param)
    vibez.spill("")
}

fr fr ===== QUERY STRING PROCESSING EXAMPLE =====

slay demo_query_processing() {
    vibez.spill("=== Query String Processing ===")
    
    fr fr Parse existing query string
    sus query tea = "search=cursed+lang&page=1&limit=20&sort=relevance"
    sus params []urlz.QueryParam = urlz.parse_query_string(query)
    
    vibez.spill("Original query:", query)
    vibez.spill("Parsed parameters:")
    sus i drip = 0
    bestie (i < urlz.len(params)) {
        vibez.spill("  ", params[i].key, "=", params[i].value)
        i = i + 1
    }
    
    fr fr Get specific parameter
    sus search_term tea = urlz.get_query_param(query, "search")
    vibez.spill("Search term:", search_term)
    
    fr fr Update parameter
    sus updated_query tea = urlz.set_query_param(query, "page", "2")
    vibez.spill("Updated query (page=2):", updated_query)
    
    fr fr Add new parameter
    sus with_filter tea = urlz.set_query_param(updated_query, "filter", "active")
    vibez.spill("With filter:", with_filter)
    
    fr fr Remove parameter
    sus without_limit tea = urlz.remove_query_param(with_filter, "limit")
    vibez.spill("Without limit:", without_limit)
    vibez.spill("")
}

fr fr ===== URL VALIDATION EXAMPLE =====

slay demo_url_validation() {
    vibez.spill("=== URL Validation ===")
    
    sus test_urls []tea = [
        "https://example.com/valid",
        "http://localhost:3000/dev",
        "ftp://files.example.com/data",
        "not-a-url",
        "",
        "https://",
        "javascript:alert('xss')"
    ]
    
    sus i drip = 0
    bestie (i < urlz.len(test_urls)) {
        sus test_url tea = test_urls[i]
        sus is_valid lit = urlz.is_valid_url(test_url)
        sus is_absolute lit = urlz.is_absolute_url(test_url)
        sus is_secure lit = urlz.is_secure_url(test_url)
        
        vibez.spill("URL:", test_url)
        vibez.spill("  Valid:", is_valid)
        vibez.spill("  Absolute:", is_absolute)
        vibez.spill("  Secure:", is_secure)
        i = i + 1
    }
    vibez.spill("")
}

fr fr ===== URL MANIPULATION EXAMPLE =====

slay demo_url_manipulation() {
    vibez.spill("=== URL Manipulation ===")
    
    fr fr Path joining
    sus base_path tea = "/api/v1"
    sus resource tea = "users/123/profile"
    sus full_path tea = urlz.join_url_paths(base_path, resource)
    vibez.spill("Joined path:", full_path)
    
    fr fr Relative URL resolution
    sus base_url tea = "https://api.example.com/v1/users"
    sus relative1 tea = "../docs/guide.html"
    sus relative2 tea = "123/settings"
    sus relative3 tea = "?format=xml"
    sus relative4 tea = "#profile-section"
    
    vibez.spill("Base URL:", base_url)
    vibez.spill("Relative '../docs/guide.html':", urlz.resolve_relative_url(base_url, relative1))
    vibez.spill("Relative '123/settings':", urlz.resolve_relative_url(base_url, relative2))
    vibez.spill("Relative '?format=xml':", urlz.resolve_relative_url(base_url, relative3))
    vibez.spill("Relative '#profile-section':", urlz.resolve_relative_url(base_url, relative4))
    
    fr fr URL normalization
    sus messy_url tea = "HTTPS://Example.COM:443//api//v1//users//"
    sus normalized tea = urlz.normalize_url(messy_url)
    vibez.spill("Messy URL:", messy_url)
    vibez.spill("Normalized:", normalized)
    vibez.spill("")
}

fr fr ===== URL COMPONENT EXTRACTION EXAMPLE =====

slay demo_component_extraction() {
    vibez.spill("=== URL Component Extraction ===")
    
    sus url tea = "https://api.subdomain.example.com:8080/path/to/resource?param=value"
    
    vibez.spill("URL:", url)
    vibez.spill("Domain:", urlz.get_domain(url))
    vibez.spill("Subdomain:", urlz.get_subdomain(url))
    vibez.spill("TLD:", urlz.get_top_level_domain(url))
    vibez.spill("Base URL:", urlz.get_base_url(url))
    vibez.spill("Origin:", urlz.get_origin(url))
    vibez.spill("")
}

fr fr ===== URL SECURITY EXAMPLE =====

slay demo_security_features() {
    vibez.spill("=== URL Security Features ===")
    
    fr fr Safe redirect validation
    sus allowed_hosts []tea = ["example.com", "api.example.com", "secure.myapp.com"]
    
    sus redirect_urls []tea = [
        "https://example.com/login",
        "https://api.example.com/callback",
        "https://malicious.com/phish",
        "javascript:alert('xss')",
        "https://secure.myapp.com/dashboard"
    ]
    
    vibez.spill("Allowed hosts: example.com, api.example.com, secure.myapp.com")
    
    sus i drip = 0
    bestie (i < urlz.len(redirect_urls)) {
        sus redirect_url tea = redirect_urls[i]
        sus is_safe lit = urlz.is_safe_redirect(redirect_url, allowed_hosts)
        vibez.spill("Redirect:", redirect_url)
        vibez.spill("  Safe:", is_safe)
        i = i + 1
    }
    
    fr fr URL sanitization
    vibez.spill("\n--- URL Sanitization ---")
    sus dangerous_urls []tea = [
        "https://user:pass@example.com/api",
        "javascript:alert('xss')",
        "data:text/html,<script>alert('xss')</script>",
        "https://example.com/safe/path"
    ]
    
    sus j drip = 0
    bestie (j < urlz.len(dangerous_urls)) {
        sus dangerous_url tea = dangerous_urls[j]
        sus sanitized tea = urlz.sanitize_url(dangerous_url)
        vibez.spill("Original:", dangerous_url)
        vibez.spill("Sanitized:", sanitized)
        j = j + 1
    }
    vibez.spill("")
}

fr fr ===== REAL-WORLD API EXAMPLE =====

slay demo_api_scenario() {
    vibez.spill("=== Real-World API Scenario ===")
    
    fr fr Building a complete API request URL
    sus api_base tea = "https://api.ecommerce.com/v2"
    sus endpoint tea = "products/search"
    
    fr fr Create search parameters
    sus search_params []urlz.QueryParam = [
        urlz.QueryParam{key: "q", value: "wireless headphones"},
        urlz.QueryParam{key: "category", value: "electronics"},
        urlz.QueryParam{key: "min_price", value: "50"},
        urlz.QueryParam{key: "max_price", value: "200"},
        urlz.QueryParam{key: "sort", value: "rating_desc"},
        urlz.QueryParam{key: "page", value: "1"},
        urlz.QueryParam{key: "per_page", value: "24"}
    ]
    
    fr fr Build the complete URL
    sus full_endpoint tea = urlz.join_url_paths(api_base, endpoint)
    sus query_string tea = urlz.build_query_string(search_params)
    sus api_url tea = full_endpoint + "?" + query_string
    
    vibez.spill("API Base:", api_base)
    vibez.spill("Endpoint:", endpoint)
    vibez.spill("Full API URL:", api_url)
    
    fr fr Parse it back to verify
    sus parsed urlz.URL = urlz.parse_url(api_url)
    vibez.spill("Parsed successfully:", parsed.is_valid)
    vibez.spill("Query parameters:", parsed.query)
    
    fr fr Extract specific search term
    sus search_term tea = urlz.get_query_param(parsed.query, "q")
    vibez.spill("Search term:", search_term)
    vibez.spill("")
}

fr fr ===== URL COMPARISON EXAMPLE =====

slay demo_url_comparison() {
    vibez.spill("=== URL Comparison ===")
    
    sus url1 tea = "https://example.com/api/v1"
    sus url2 tea = "https://EXAMPLE.COM:443/api/v1/"
    sus url3 tea = "http://example.com/api/v1"
    sus url4 tea = "https://other.com/api/v1"
    
    vibez.spill("URL1:", url1)
    vibez.spill("URL2:", url2)
    vibez.spill("URLs equal (after normalization):", urlz.urls_equal(url1, url2))
    
    vibez.spill("URL3:", url3)
    vibez.spill("Same origin as URL1:", urlz.same_origin(url1, url3))
    vibez.spill("Same domain as URL1:", urlz.is_same_domain(url1, url3))
    
    vibez.spill("URL4:", url4)
    vibez.spill("Same domain as URL1:", urlz.is_same_domain(url1, url4))
    vibez.spill("")
}

fr fr ===== BATCH PROCESSING EXAMPLE =====

slay demo_batch_processing() {
    vibez.spill("=== Batch URL Processing ===")
    
    fr fr Process a list of URLs
    sus urls []tea = [
        "https://api.service1.com/v1/data",
        "http://legacy.service2.com:8080/api",
        "https://cdn.assets.com/images/logo.png",
        "ftp://files.backup.com/daily/",
        "https://auth.service.com/oauth/token"
    ]
    
    vibez.spill("Processing", urlz.len(urls), "URLs:")
    
    sus i drip = 0
    bestie (i < urlz.len(urls)) {
        sus url tea = urls[i]
        sus parsed urlz.URL = urlz.parse_url(url)
        
        vibez.spill("\nURL", i + 1, ":", url)
        vibez.spill("  Domain:", parsed.host)
        vibez.spill("  Secure:", urlz.is_secure_url(url))
        vibez.spill("  Valid:", parsed.is_valid)
        vibez.spill("  Normalized:", urlz.normalize_url(url))
        
        i = i + 1
    }
    vibez.spill("")
}

fr fr ===== MAIN DEMO FUNCTION =====

slay main() drip {
    vibez.spill("CURSED URL Parsing Package (urlz) - Example Usage")
    vibez.spill("=" * 60)
    vibez.spill("")
    
    demo_basic_parsing()
    demo_url_building()
    demo_encoding_decoding()
    demo_query_processing()
    demo_url_validation()
    demo_url_manipulation()
    demo_component_extraction()
    demo_security_features()
    demo_api_scenario()
    demo_url_comparison()
    demo_batch_processing()
    
    vibez.spill("URL parsing examples completed!")
    damn 0
}
