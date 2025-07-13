# URL Parsing Module

A comprehensive RFC 3986 compliant URL parsing and manipulation library for the CURSED language. This module provides pure CURSED implementation for URL parsing, validation, encoding/decoding, and manipulation without any FFI dependencies.

## Features

- **RFC 3986 Compliant**: Full compliance with URL specification standards
- **Pure CURSED Implementation**: No external dependencies or FFI bridges
- **Comprehensive Parsing**: Supports all URL components (scheme, host, port, path, query, fragment, credentials)
- **URL Validation**: Validates URLs according to RFC standards
- **URL Manipulation**: Build, modify, and normalize URLs
- **Query Parameter Management**: Add, remove, and retrieve query parameters
- **URL Encoding/Decoding**: Proper percent-encoding for URL safety
- **URL Resolution**: Resolve relative URLs against base URLs
- **Security Features**: Detect secure schemes and localhost URLs

## Supported URL Schemes

- `http://` - HTTP protocol (default port 80)
- `https://` - Secure HTTP protocol (default port 443)
- `ftp://` - File Transfer Protocol (default port 21)
- `ftps://` - Secure FTP protocol
- `file://` - Local file system
- `ssh://` - Secure Shell (default port 22)
- `ws://` - WebSocket protocol
- `wss://` - Secure WebSocket protocol

## Basic Usage

### Parsing URLs

```cursed
yeet "url_parsing"

# Parse a complete URL
url_parse("https://user:pass@api.example.com:8080/v1/users?limit=10&sort=name#results")

# Access components
vibez.spill("Scheme: " + url_get_scheme())     # "https"
vibez.spill("Host: " + url_get_host())         # "api.example.com"
vibez.spill("Port: " + url_get_port())         # 8080
vibez.spill("Path: " + url_get_path())         # "/v1/users"
vibez.spill("Query: " + url_get_query())       # "limit=10&sort=name"
vibez.spill("Fragment: " + url_get_fragment()) # "results"
vibez.spill("Username: " + url_get_username()) # "user"
vibez.spill("Password: " + url_get_password()) # "pass"
```

### Building URLs

```cursed
# Start with basic URL
url_parse("http://example.com")

# Modify components
url_set_scheme("https")
url_set_host("api.example.com")
url_set_port(8443)
url_set_path("/v2/endpoint")

# Add query parameters
url_add_query_param("api_key", "secret123")
url_add_query_param("format", "json")

# Build final URL
sus final_url tea = url_build()
vibez.spill("Built URL: " + final_url)
# Output: "https://api.example.com:8443/v2/endpoint?api_key=secret123&format=json"
```

### URL Validation

```cursed
url_parse("https://secure.example.com/api")

bestie url_is_valid() {
    vibez.spill("URL is valid")
}

bestie url_is_absolute() {
    vibez.spill("URL is absolute")
}

bestie url_is_secure() {
    vibez.spill("URL uses secure protocol")
}

bestie url_is_localhost() {
    vibez.spill("URL points to localhost")
}
```

### Query Parameter Management

```cursed
url_parse("https://search.example.com")

# Add parameters
url_add_query_param("q", "search term")
url_add_query_param("limit", "50")
url_add_query_param("category", "books & media")  # Will be properly encoded

# Check and retrieve parameters
bestie url_has_query_param("q") {
    sus search_term tea = url_get_query_param("q")
    vibez.spill("Search term: " + search_term)
}

# Remove parameter
url_remove_query_param("limit")

# Clear all parameters
url_clear_query_params()
```

### URL Encoding and Decoding

```cursed
# Encode unsafe characters
sus unsafe_text tea = "hello world & special=chars"
sus encoded tea = url_encode(unsafe_text)
vibez.spill("Encoded: " + encoded)  # "hello%20world%20%26%20special%3Dchars"

# Decode back
sus decoded tea = url_decode(encoded)
vibez.spill("Decoded: " + decoded)  # "hello world & special=chars"

# Encode query parameters properly
sus param tea = url_encode_query_param("search query", "books & movies")
vibez.spill("Encoded param: " + param)  # "search%20query=books%20%26%20movies"
```

### URL Resolution and Joining

```cursed
# Resolve relative URLs
sus base tea = "https://example.com/api/v1/"
sus relative tea = "users/123/profile"
sus resolved tea = url_resolve(base, relative)
vibez.spill("Resolved: " + resolved)  # "https://example.com/api/v1/users/123/profile"

# Join URL with path
sus joined tea = url_join("https://cdn.example.com", "assets/images/logo.png")
vibez.spill("Joined: " + joined)  # "https://cdn.example.com/assets/images/logo.png"
```

### URL Normalization

```cursed
# Normalize URLs for consistency
url_parse("HTTP://WWW.EXAMPLE.COM:80/Path/../Other")
url_normalize()

vibez.spill("Scheme: " + url_get_scheme())  # "http" (lowercase)
vibez.spill("Host: " + url_get_host())      # "www.example.com" (lowercase)
vibez.spill("Port: " + url_get_port())      # 0 (default port removed)
```

### URL Comparison

```cursed
url_parse("https://example.com/page")

# Compare complete URLs
bestie url_equals("https://example.com/page") {
    vibez.spill("URLs are identical")
}

# Check same origin (scheme + host + port)
bestie url_same_origin("https://example.com/other-page") {
    vibez.spill("URLs have same origin")
}
```

### Utility Functions

```cursed
url_parse("https://api.subdomain.example.com:8080/v1/users/profile.json?active=true")

# Extract various parts
vibez.spill("Base URL: " + url_get_base_url())        # "https://api.subdomain.example.com:8080"
vibez.spill("Domain: " + url_get_domain())            # "subdomain.example.com"
vibez.spill("Subdomain: " + url_get_subdomain())      # "api"
vibez.spill("Filename: " + url_get_filename())        # "profile.json"
vibez.spill("Extension: " + url_get_file_extension()) # "json"
vibez.spill("Directory: " + url_get_directory())      # "/v1/users/"
vibez.spill("Protocol: " + url_get_protocol())        # "https"
vibez.spill("Authority: " + url_get_authority())      # "api.subdomain.example.com:8080"
```

## Advanced Examples

### Building API URLs

```cursed
slay build_api_url(base_url tea, endpoint tea, params tea) tea {
    url_parse(base_url)
    url_set_path("/api/v1/" + endpoint)
    
    # Add common parameters
    url_add_query_param("format", "json")
    url_add_query_param("timestamp", get_current_timestamp())
    
    # Add custom parameters
    bestie params != "" {
        url_add_query_param("custom", params)
    }
    
    damn url_build()
}

sus api_url tea = build_api_url("https://service.example.com", "users", "active=true")
vibez.spill("API URL: " + api_url)
```

### URL Validation Function

```cursed
slay validate_url_security(url_string tea) lit {
    bestie !url_parse(url_string) {
        vibez.spill("Invalid URL format")
        damn cap
    }
    
    bestie !url_is_secure() {
        vibez.spill("URL must use secure protocol (HTTPS)")
        damn cap
    }
    
    bestie url_has_credentials() {
        vibez.spill("URLs with embedded credentials are not allowed")
        damn cap
    }
    
    vibez.spill("URL passed security validation")
    damn based
}
```

### Cleaning and Normalizing URLs

```cursed
slay clean_url(dirty_url tea) tea {
    bestie !url_parse(dirty_url) {
        damn ""
    }
    
    # Normalize the URL
    url_normalize()
    
    # Remove sensitive information
    url_set_username("")
    url_set_password("")
    
    # Remove tracking parameters
    url_remove_query_param("utm_source")
    url_remove_query_param("utm_medium")
    url_remove_query_param("utm_campaign")
    
    damn url_build()
}
```

## API Reference

### Core Functions

| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `url_parse(url_string tea)` | URL string | `lit` | Parse URL string into components |
| `url_build()` | None | `tea` | Build URL from current components |
| `url_rebuild()` | None | `lit` | Rebuild internal URL from components |
| `url_clear()` | None | `lit` | Clear all URL components |
| `url_is_parsed()` | None | `lit` | Check if URL is currently parsed |
| `url_get_raw()` | None | `tea` | Get original/built URL string |

### Component Getters

| Function | Returns | Description |
|----------|---------|-------------|
| `url_get_scheme()` | `tea` | Get URL scheme (http, https, etc.) |
| `url_get_host()` | `tea` | Get hostname |
| `url_get_port()` | `normie` | Get port number |
| `url_get_path()` | `tea` | Get URL path |
| `url_get_query()` | `tea` | Get query string |
| `url_get_fragment()` | `tea` | Get URL fragment |
| `url_get_username()` | `tea` | Get username from URL |
| `url_get_password()` | `tea` | Get password from URL |

### Component Setters

| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `url_set_scheme(scheme tea)` | Scheme name | `lit` | Set URL scheme |
| `url_set_host(host tea)` | Hostname | `lit` | Set hostname |
| `url_set_port(port normie)` | Port number | `lit` | Set port number |
| `url_set_path(path tea)` | URL path | `lit` | Set URL path |
| `url_set_query(query tea)` | Query string | `lit` | Set query string |
| `url_set_fragment(fragment tea)` | Fragment | `lit` | Set URL fragment |
| `url_set_username(username tea)` | Username | `lit` | Set username |
| `url_set_password(password tea)` | Password | `lit` | Set password |

### Query Parameters

| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `url_add_query_param(key tea, value tea)` | Key, Value | `lit` | Add query parameter |
| `url_get_query_param(key tea)` | Parameter key | `tea` | Get parameter value |
| `url_has_query_param(key tea)` | Parameter key | `lit` | Check if parameter exists |
| `url_remove_query_param(key tea)` | Parameter key | `lit` | Remove parameter |
| `url_clear_query_params()` | None | `lit` | Remove all parameters |
| `url_get_query_params()` | None | `tea` | Get all parameters string |

### Validation

| Function | Returns | Description |
|----------|---------|-------------|
| `url_is_valid()` | `lit` | Check if URL is valid |
| `url_is_absolute()` | `lit` | Check if URL is absolute |
| `url_is_relative()` | `lit` | Check if URL is relative |
| `url_is_secure()` | `lit` | Check if URL uses secure protocol |
| `url_has_credentials()` | `lit` | Check if URL has credentials |
| `url_is_localhost()` | `lit` | Check if URL points to localhost |

### Manipulation

| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `url_resolve(base tea, relative tea)` | Base URL, Relative URL | `tea` | Resolve relative URL |
| `url_join(base tea, path tea)` | Base URL, Path | `tea` | Join URL with path |
| `url_normalize()` | None | `lit` | Normalize URL components |

### Encoding/Decoding

| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `url_encode(text tea)` | Text to encode | `tea` | URL encode text |
| `url_decode(text tea)` | Text to decode | `tea` | URL decode text |
| `url_encode_query_param(key tea, value tea)` | Key, Value | `tea` | Encode query parameter |

### Comparison

| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `url_equals(other_url tea)` | URL to compare | `lit` | Check if URLs are equal |
| `url_same_origin(other_url tea)` | URL to compare | `lit` | Check if URLs have same origin |

### Utilities

| Function | Returns | Description |
|----------|---------|-------------|
| `url_get_base_url()` | `tea` | Get base URL (scheme + host + port) |
| `url_get_domain()` | `tea` | Get domain (host without subdomain) |
| `url_get_subdomain()` | `tea` | Get subdomain |
| `url_get_file_extension()` | `tea` | Get file extension from path |
| `url_get_filename()` | `tea` | Get filename from path |
| `url_get_directory()` | `tea` | Get directory from path |
| `url_get_protocol()` | `tea` | Get protocol (alias for scheme) |
| `url_get_authority()` | `tea` | Get authority (user:pass@host:port) |

## Error Handling

The module uses boolean return values for operations that can fail:

- `url_parse()` returns `cap` for invalid URLs
- Setter functions return `cap` if no URL is currently parsed
- `url_set_port()` returns `cap` for invalid port numbers (< 1 or > 65535)

Always check return values before proceeding with operations:

```cursed
bestie url_parse("https://example.com") {
    # URL parsing successful, safe to use other functions
    vibez.spill("Host: " + url_get_host())
} otherwise {
    vibez.spill("Failed to parse URL")
}
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/url_parsing/test_url_parsing.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/url_parsing/test_url_parsing.csd
./test_url_parsing
```

The test suite includes:
- Basic URL parsing for all supported schemes
- Component manipulation and validation
- Query parameter management
- URL encoding/decoding
- URL resolution and joining
- Edge cases and error conditions
- Performance and stress tests
- Integration scenarios

## Implementation Notes

- **Pure CURSED**: No FFI dependencies, completely implemented in CURSED language
- **RFC 3986 Compliant**: Follows URL specification standards
- **Stateful Design**: Uses global URL instance for simplicity
- **Gen Z Naming**: Consistent with CURSED language style using slang function names
- **Error Resilient**: Graceful handling of malformed URLs and invalid operations
- **Performance Optimized**: Efficient string operations and minimal memory allocation

## Contributing

When adding new features:
1. Follow RFC 3986 specifications
2. Maintain Gen Z slang naming conventions
3. Add comprehensive tests for new functionality
4. Update documentation with examples
5. Ensure pure CURSED implementation (no FFI)

## License

This module is part of the CURSED language standard library and follows the same licensing terms.
