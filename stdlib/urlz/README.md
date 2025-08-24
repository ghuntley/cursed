# CURSED URL Parsing Package (urlz)

A comprehensive URL parsing, validation, and manipulation library for CURSED, providing robust handling of URL components, query parameters, encoding/decoding, and URL normalization.

## Overview

The `urlz` package provides:

- **URL Parsing**: Complete parsing of URL components (scheme, host, port, path, query, fragment, auth)
- **URL Building**: Construct URLs from components
- **URL Validation**: Validate URL format and components
- **URL Encoding**: Percent encoding/decoding for safe URL transmission
- **Query String Processing**: Parse and manipulate query parameters
- **URL Manipulation**: Join paths, resolve relative URLs, normalize URLs
- **Security Helpers**: Safe redirect validation and URL sanitization

## Quick Start

```cursed
yeet "urlz"

fr fr Parse a URL
sus url urlz.URL = urlz.parse_url("https://user:pass@example.com:8080/path?query=value#fragment")

vibez.spill("Scheme:", url.scheme)     fr fr "https"
vibez.spill("Host:", url.host)         fr fr "example.com"
vibez.spill("Port:", url.port)         fr fr 8080
vibez.spill("Path:", url.path)         fr fr "/path"
vibez.spill("Query:", url.query)       fr fr "query=value"
vibez.spill("Fragment:", url.fragment) fr fr "fragment"

fr fr Build URL back
sus rebuilt tea = urlz.build_url(url)
vibez.spill("Rebuilt:", rebuilt)
```

## Core Functions

### URL Parsing

#### `parse_url(url tea) URL`
Parse a complete URL string into its components.

```cursed
sus url urlz.URL = urlz.parse_url("https://api.example.com/v1/users?limit=10")
vibez.spill("Scheme:", url.scheme)  fr fr "https"
vibez.spill("Host:", url.host)      fr fr "api.example.com"
vibez.spill("Path:", url.path)      fr fr "/v1/users"
```

#### `build_url(url URL) tea`
Build a URL string from parsed components.

```cursed
sus url urlz.URL = urlz.URL{
    scheme: "https",
    host: "example.com",
    path: "/api/v1",
    query: "format=json"
}
sus result tea = urlz.build_url(url)  fr fr "https://example.com/api/v1?format=json"
```

### URL Validation

#### `is_valid_url(url tea) lit`
Check if a URL string is valid.

```cursed
vibez.spill(urlz.is_valid_url("https://example.com"))      fr fr true
vibez.spill(urlz.is_valid_url("not-a-url"))               fr fr false
```

#### `is_absolute_url(url tea) lit`
Check if URL is absolute (has scheme).

```cursed
vibez.spill(urlz.is_absolute_url("https://example.com"))   fr fr true
vibez.spill(urlz.is_absolute_url("/path/to/resource"))     fr fr false
```

#### `is_secure_url(url tea) lit`
Check if URL uses secure scheme (https, ftps, etc.).

```cursed
vibez.spill(urlz.is_secure_url("https://example.com"))     fr fr true
vibez.spill(urlz.is_secure_url("http://example.com"))      fr fr false
```

### URL Encoding

#### `url_encode(input tea) tea`
URL encode (percent encoding) for safe transmission.

```cursed
sus encoded tea = urlz.url_encode("hello world!")
vibez.spill(encoded)  fr fr "hello%20world%21"
```

#### `url_decode(input tea) tea`
URL decode (percent decoding).

```cursed
sus decoded tea = urlz.url_decode("hello%20world%21")
vibez.spill(decoded)  fr fr "hello world!"
```

### Query String Processing

#### `parse_query_string(query tea) []QueryParam`
Parse query string into key-value pairs.

```cursed
sus params []urlz.QueryParam = urlz.parse_query_string("name=John&age=30&city=NYC")
vibez.spill("First param:", params[0].key, "=", params[0].value)  fr fr "name = John"
```

#### `build_query_string(params []QueryParam) tea`
Build query string from parameters.

```cursed
sus params []urlz.QueryParam = [
    urlz.QueryParam{key: "search", value: "cursed lang"},
    urlz.QueryParam{key: "page", value: "1"}
]
sus query tea = urlz.build_query_string(params)
vibez.spill(query)  fr fr "search=cursed%20lang&page=1"
```

#### `get_query_param(query tea, key tea) tea`
Get specific query parameter value.

```cursed
sus value tea = urlz.get_query_param("name=John&age=30", "name")
vibez.spill(value)  fr fr "John"
```

#### `set_query_param(query tea, key tea, value tea) tea`
Set or update query parameter.

```cursed
sus updated tea = urlz.set_query_param("page=1", "page", "2")
vibez.spill(updated)  fr fr "page=2"
```

### URL Manipulation

#### `join_url_paths(base tea, path tea) tea`
Join two URL paths properly.

```cursed
sus joined tea = urlz.join_url_paths("/api/v1", "users")
vibez.spill(joined)  fr fr "/api/v1/users"
```

#### `resolve_relative_url(base tea, relative tea) tea`
Resolve relative URL against base URL.

```cursed
sus resolved tea = urlz.resolve_relative_url("https://example.com/api/", "../docs/guide")
vibez.spill(resolved)  fr fr "https://example.com/docs/guide"
```

#### `normalize_url(url tea) tea`
Normalize URL by cleaning up components.

```cursed
sus normalized tea = urlz.normalize_url("HTTPS://Example.COM:443//path//to//file")
vibez.spill(normalized)  fr fr "https://example.com/path/to/file"
```

### URL Component Extraction

#### `get_domain(url tea) tea`
Extract domain from URL.

```cursed
sus domain tea = urlz.get_domain("https://api.example.com/users")
vibez.spill(domain)  fr fr "api.example.com"
```

#### `get_subdomain(url tea) tea`
Extract subdomain from URL.

```cursed
sus subdomain tea = urlz.get_subdomain("https://api.example.com")
vibez.spill(subdomain)  fr fr "api"
```

#### `get_base_url(url tea) tea`
Get base URL (scheme + host + port).

```cursed
sus base tea = urlz.get_base_url("https://example.com:8080/path?query")
vibez.spill(base)  fr fr "https://example.com:8080"
```

### URL Comparison

#### `urls_equal(url1 tea, url2 tea) lit`
Compare URLs for equality (after normalization).

```cursed
vibez.spill(urlz.urls_equal("https://example.com/", "https://EXAMPLE.COM"))  fr fr true
```

#### `same_origin(url1 tea, url2 tea) lit`
Check if URLs have same origin.

```cursed
vibez.spill(urlz.same_origin("https://example.com/api", "https://example.com/docs"))  fr fr true
```

### Security Helpers

#### `is_safe_redirect(url tea, allowed_hosts []tea) lit`
Check if URL is safe for redirect.

```cursed
sus allowed []tea = ["example.com", "api.example.com"]
vibez.spill(urlz.is_safe_redirect("https://example.com/login", allowed))  fr fr true
```

#### `sanitize_url(url tea) tea`
Sanitize URL by removing dangerous components.

```cursed
sus safe tea = urlz.sanitize_url("javascript://user:pass@example.com/script")
vibez.spill(safe)  fr fr "" (empty - blocked dangerous scheme)
```

## Data Types

### URL Structure
```cursed
squad URL {
    scheme tea,      fr fr Protocol (http, https, ftp, etc.)
    host tea,        fr fr Domain name or IP address
    port drip,       fr fr Port number (0 = default for scheme)
    path tea,        fr fr URL path
    query tea,       fr fr Query string (without ?)
    fragment tea,    fr fr Fragment identifier (without #)
    username tea,    fr fr Username for authentication
    password tea,    fr fr Password for authentication
    is_valid lit     fr fr Whether the URL is valid
}
```

### Query Parameter Structure
```cursed
squad QueryParam {
    key tea,    fr fr Parameter name
    value tea   fr fr Parameter value
}
```

## Advanced Examples

### Complete URL Processing
```cursed
yeet "urlz"
yeet "vibez"

fr fr Parse complex URL
sus complex_url tea = "https://user:secret@api.example.com:8443/v2/users/123?include=profile&format=json#details"
sus url urlz.URL = urlz.parse_url(complex_url)

vibez.spill("=== URL Components ===")
vibez.spill("Scheme:", url.scheme)
vibez.spill("Username:", url.username)  fr fr Don't log passwords!
vibez.spill("Host:", url.host)
vibez.spill("Port:", url.port)
vibez.spill("Path:", url.path)
vibez.spill("Query:", url.query)
vibez.spill("Fragment:", url.fragment)
vibez.spill("Valid:", url.is_valid)
```

### Query Parameter Manipulation
```cursed
sus original_query tea = "search=cursed&page=1&limit=10"
sus params []urlz.QueryParam = urlz.parse_query_string(original_query)

vibez.spill("=== Original Parameters ===")
sus i drip = 0
bestie (i < urlz.len(params)) {
    vibez.spill(params[i].key, "=", params[i].value)
    i = i + 1
}

fr fr Modify parameters
sus updated_query tea = urlz.set_query_param(original_query, "page", "2")
updated_query = urlz.set_query_param(updated_query, "sort", "name")
updated_query = urlz.remove_query_param(updated_query, "limit")

vibez.spill("Updated Query:", updated_query)
```

### URL Building and Manipulation
```cursed
fr fr Build API URL
sus api_url urlz.URL = urlz.URL{
    scheme: "https",
    host: "api.myapp.com",
    path: "/v1/users",
    query: "active=true&sort=created"
}

sus base_url tea = urlz.build_url(api_url)
vibez.spill("API Base:", base_url)

fr fr Add specific user path
sus user_url tea = urlz.join_url_paths(base_url, "123")
vibez.spill("User URL:", user_url)

fr fr Resolve relative documentation link
sus docs_url tea = urlz.resolve_relative_url(base_url, "../docs/api-guide.html")
vibez.spill("Docs URL:", docs_url)
```

### URL Validation and Security
```cursed
sus test_urls []tea = [
    "https://example.com/safe",
    "javascript:alert('xss')",
    "http://malicious.com/phish",
    "https://trusted.myapp.com/api"
]

sus allowed_hosts []tea = ["example.com", "trusted.myapp.com"]

sus i drip = 0
bestie (i < urlz.len(test_urls)) {
    sus test_url tea = test_urls[i]
    sus is_valid lit = urlz.is_valid_url(test_url)
    sus is_safe lit = urlz.is_safe_redirect(test_url, allowed_hosts)
    sus is_secure lit = urlz.is_secure_url(test_url)
    
    vibez.spill("URL:", test_url)
    vibez.spill("  Valid:", is_valid)
    vibez.spill("  Safe:", is_safe)
    vibez.spill("  Secure:", is_secure)
    
    i = i + 1
}
```

## Error Handling

The urlz package uses CURSED's built-in error handling patterns:

- Invalid URLs return empty results or `cringe` for boolean functions
- The `URL.is_valid` field indicates parsing success
- Empty strings are returned for missing components
- Array operations handle bounds safely

```cursed
sus url urlz.URL = urlz.parse_url("not-a-valid-url")
ready (!url.is_valid) {
    vibez.spill("Invalid URL provided")
    damn
}

fr fr Safe to use parsed components
vibez.spill("Host:", url.host)
```

## Performance Considerations

- URL parsing is optimized for common URL patterns
- String operations use efficient CURSED stdlib functions
- Query parameter arrays are optimized for typical sizes (1-10 parameters)
- Encoding/decoding tables are hardcoded for maximum speed

## Integration with Other Modules

The urlz package works seamlessly with other CURSED stdlib modules:

```cursed
yeet "urlz"
yeet "networkz"  fr fr For HTTP requests
yeet "jsonz"     fr fr For API responses

fr fr Build API request URL
sus api_url tea = urlz.build_url(urlz.URL{
    scheme: "https",
    host: "api.example.com",
    path: "/v1/users",
    query: urlz.build_query_string([
        urlz.QueryParam{key: "format", value: "json"},
        urlz.QueryParam{key: "limit", value: "50"}
    ])
})

fr fr Use with HTTP client
sus response tea = networkz.get(api_url)
sus data tea = jsonz.parse(response)
```

## Security Best Practices

1. **Always validate URLs** before using them in redirects or requests
2. **Use allowed host lists** for redirect validation
3. **Sanitize URLs** when accepting user input
4. **Prefer HTTPS** URLs for sensitive operations
5. **URL encode** all user input in query parameters

## Compatibility

- Pure CURSED implementation - no external dependencies
- Works with all CURSED runtime modes (interpreter and compiled)
- Thread-safe operations
- Memory efficient with automatic cleanup
- Cross-platform support (Linux, macOS, Windows, WASM)
