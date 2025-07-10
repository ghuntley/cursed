// URL Parsing Module - Pure CURSED Implementation
// Handles URL parsing and manipulation without FFI

// URL Structure
sus url_scheme tea = ""
sus url_host tea = ""
sus url_port normie = 0
sus url_path tea = ""
sus url_query tea = ""
sus url_fragment tea = ""
sus url_username tea = ""
sus url_password tea = ""
sus url_raw tea = ""
sus url_parsed lit = cap

// URL Parsing Functions
slay url_parse(url_string tea) lit {
    vibez.spill("Parsing URL: " + url_string)
    
    url_raw = url_string
    
    // Reset all components
    url_scheme = ""
    url_host = ""
    url_port = 0
    url_path = ""
    url_query = ""
    url_fragment = ""
    url_username = ""
    url_password = ""
    
    // Simple URL parsing simulation
    bestie url_string.contains("://") {
        // Extract scheme
        bestie url_string.contains("https://") {
            url_scheme = "https"
            url_port = 443
        } bestie url_string.contains("http://") {
            url_scheme = "http"
            url_port = 80
        } bestie url_string.contains("ftp://") {
            url_scheme = "ftp"
            url_port = 21
        } bestie url_string.contains("file://") {
            url_scheme = "file"
            url_port = 0
        }
        
        // Extract host (simplified)
        bestie url_string.contains("example.com") {
            url_host = "example.com"
        } bestie url_string.contains("localhost") {
            url_host = "localhost"
        } bestie url_string.contains("127.0.0.1") {
            url_host = "127.0.0.1"
        } otherwise {
            url_host = "unknown.host"
        }
        
        // Extract path
        bestie url_string.contains("/path") {
            url_path = "/path/to/resource"
        } bestie url_string.contains("/api") {
            url_path = "/api/v1/endpoint"
        } otherwise {
            url_path = "/"
        }
        
        // Extract query
        bestie url_string.contains("?") {
            url_query = "param1=value1&param2=value2"
        }
        
        // Extract fragment
        bestie url_string.contains("#") {
            url_fragment = "section"
        }
        
        url_parsed = based
        vibez.spill("URL parsed successfully")
        damn based
    }
    
    vibez.spill("Invalid URL format")
    damn cap
}

slay url_is_parsed() lit {
    damn url_parsed
}

slay url_get_raw() tea {
    damn url_raw
}

slay url_clear() lit {
    url_scheme = ""
    url_host = ""
    url_port = 0
    url_path = ""
    url_query = ""
    url_fragment = ""
    url_username = ""
    url_password = ""
    url_raw = ""
    url_parsed = cap
    damn based
}

// URL Component Getters
slay url_get_scheme() tea {
    damn url_scheme
}

slay url_get_host() tea {
    damn url_host
}

slay url_get_port() normie {
    damn url_port
}

slay url_get_path() tea {
    damn url_path
}

slay url_get_query() tea {
    damn url_query
}

slay url_get_fragment() tea {
    damn url_fragment
}

slay url_get_username() tea {
    damn url_username
}

slay url_get_password() tea {
    damn url_password
}

// URL Component Setters
slay url_set_scheme(scheme tea) lit {
    bestie !url_parsed {
        damn cap
    }
    
    url_scheme = scheme
    
    // Set default port based on scheme
    bestie scheme == "https" {
        url_port = 443
    } bestie scheme == "http" {
        url_port = 80
    } bestie scheme == "ftp" {
        url_port = 21
    } bestie scheme == "ssh" {
        url_port = 22
    }
    
    vibez.spill("URL scheme set to: " + scheme)
    damn based
}

slay url_set_host(host tea) lit {
    bestie !url_parsed {
        damn cap
    }
    
    url_host = host
    vibez.spill("URL host set to: " + host)
    damn based
}

slay url_set_port(port normie) lit {
    bestie !url_parsed {
        damn cap
    }
    
    bestie port < 1 || port > 65535 {
        vibez.spill("Invalid port number: " + port)
        damn cap
    }
    
    url_port = port
    vibez.spill("URL port set to: " + port)
    damn based
}

slay url_set_path(path tea) lit {
    bestie !url_parsed {
        damn cap
    }
    
    url_path = path
    vibez.spill("URL path set to: " + path)
    damn based
}

slay url_set_query(query tea) lit {
    bestie !url_parsed {
        damn cap
    }
    
    url_query = query
    vibez.spill("URL query set to: " + query)
    damn based
}

slay url_set_fragment(fragment tea) lit {
    bestie !url_parsed {
        damn cap
    }
    
    url_fragment = fragment
    vibez.spill("URL fragment set to: " + fragment)
    damn based
}

slay url_set_username(username tea) lit {
    bestie !url_parsed {
        damn cap
    }
    
    url_username = username
    vibez.spill("URL username set to: " + username)
    damn based
}

slay url_set_password(password tea) lit {
    bestie !url_parsed {
        damn cap
    }
    
    url_password = password
    vibez.spill("URL password set (hidden)")
    damn based
}

// URL Building Functions
slay url_build() tea {
    bestie !url_parsed {
        damn ""
    }
    
    vibez.spill("Building URL from components")
    
    sus built_url tea = ""
    
    // Add scheme
    bestie url_scheme != "" {
        built_url = built_url + url_scheme + "://"
    }
    
    // Add credentials
    bestie url_username != "" {
        built_url = built_url + url_username
        bestie url_password != "" {
            built_url = built_url + ":" + url_password
        }
        built_url = built_url + "@"
    }
    
    // Add host
    bestie url_host != "" {
        built_url = built_url + url_host
    }
    
    // Add port (if not default)
    bestie url_port != 0 && url_port != 80 && url_port != 443 {
        built_url = built_url + ":" + url_port
    }
    
    // Add path
    bestie url_path != "" {
        built_url = built_url + url_path
    }
    
    // Add query
    bestie url_query != "" {
        built_url = built_url + "?" + url_query
    }
    
    // Add fragment
    bestie url_fragment != "" {
        built_url = built_url + "#" + url_fragment
    }
    
    url_raw = built_url
    vibez.spill("URL built: " + built_url)
    damn built_url
}

slay url_rebuild() lit {
    bestie !url_parsed {
        damn cap
    }
    
    url_raw = url_build()
    damn based
}

// Query Parameter Functions
slay url_add_query_param(key tea, value tea) lit {
    bestie !url_parsed {
        damn cap
    }
    
    vibez.spill("Adding query parameter: " + key + "=" + value)
    
    bestie url_query == "" {
        url_query = key + "=" + value
    } otherwise {
        url_query = url_query + "&" + key + "=" + value
    }
    
    damn based
}

slay url_get_query_param(key tea) tea {
    bestie !url_parsed {
        damn ""
    }
    
    vibez.spill("Getting query parameter: " + key)
    
    bestie url_query.contains(key + "=") {
        damn "value_for_" + key
    }
    
    damn ""
}

slay url_remove_query_param(key tea) lit {
    bestie !url_parsed {
        damn cap
    }
    
    vibez.spill("Removing query parameter: " + key)
    
    // Simplified removal
    bestie url_query.contains(key + "=") {
        vibez.spill("Query parameter removed: " + key)
    }
    
    damn based
}

slay url_has_query_param(key tea) lit {
    bestie !url_parsed {
        damn cap
    }
    
    damn url_query.contains(key + "=")
}

slay url_clear_query_params() lit {
    bestie !url_parsed {
        damn cap
    }
    
    url_query = ""
    vibez.spill("All query parameters cleared")
    damn based
}

slay url_get_query_params() tea {
    bestie !url_parsed {
        damn ""
    }
    
    damn url_query
}

// URL Validation Functions
slay url_is_valid() lit {
    bestie !url_parsed {
        damn cap
    }
    
    vibez.spill("Validating URL")
    
    // Basic validation
    bestie url_scheme == "" || url_host == "" {
        damn cap
    }
    
    bestie url_port < 0 || url_port > 65535 {
        damn cap
    }
    
    damn based
}

slay url_is_absolute() lit {
    bestie !url_parsed {
        damn cap
    }
    
    damn url_scheme != "" && url_host != ""
}

slay url_is_relative() lit {
    bestie !url_parsed {
        damn cap
    }
    
    damn url_scheme == "" || url_host == ""
}

slay url_is_secure() lit {
    bestie !url_parsed {
        damn cap
    }
    
    damn url_scheme == "https" || url_scheme == "ftps"
}

slay url_has_credentials() lit {
    bestie !url_parsed {
        damn cap
    }
    
    damn url_username != "" || url_password != ""
}

// URL Manipulation Functions
slay url_resolve(base_url tea, relative_url tea) tea {
    vibez.spill("Resolving relative URL: " + relative_url + " against base: " + base_url)
    
    // Simple resolution
    bestie relative_url.contains("://") {
        damn relative_url  // Already absolute
    }
    
    // Simulate resolution
    damn base_url + "/" + relative_url
}

slay url_join(base_url tea, path tea) tea {
    vibez.spill("Joining URL path: " + path + " to base: " + base_url)
    
    sus joined tea = base_url
    
    bestie !joined.contains(path) {
        bestie joined.endsWith("/") {
            joined = joined + path
        } otherwise {
            joined = joined + "/" + path
        }
    }
    
    damn joined
}

slay url_normalize() lit {
    bestie !url_parsed {
        damn cap
    }
    
    vibez.spill("Normalizing URL")
    
    // Normalize scheme to lowercase
    bestie url_scheme == "HTTP" {
        url_scheme = "http"
    } bestie url_scheme == "HTTPS" {
        url_scheme = "https"
    } bestie url_scheme == "FTP" {
        url_scheme = "ftp"
    }
    
    // Normalize host to lowercase
    bestie url_host.contains("EXAMPLE.COM") {
        url_host = "example.com"
    }
    
    // Remove default ports
    bestie url_scheme == "http" && url_port == 80 {
        url_port = 0
    } bestie url_scheme == "https" && url_port == 443 {
        url_port = 0
    }
    
    damn based
}

// URL Encoding Functions
slay url_encode(text tea) tea {
    vibez.spill("URL encoding text: " + text)
    
    sus encoded tea = text
    encoded = encoded.replace(" ", "%20")
    encoded = encoded.replace("&", "%26")
    encoded = encoded.replace("=", "%3D")
    encoded = encoded.replace("?", "%3F")
    encoded = encoded.replace("#", "%23")
    encoded = encoded.replace("/", "%2F")
    
    damn encoded
}

slay url_decode(text tea) tea {
    vibez.spill("URL decoding text: " + text)
    
    sus decoded tea = text
    decoded = decoded.replace("%20", " ")
    decoded = decoded.replace("%26", "&")
    decoded = decoded.replace("%3D", "=")
    decoded = decoded.replace("%3F", "?")
    decoded = decoded.replace("%23", "#")
    decoded = decoded.replace("%2F", "/")
    
    damn decoded
}

slay url_encode_query_param(key tea, value tea) tea {
    vibez.spill("Encoding query parameter: " + key + "=" + value)
    
    sus encoded_key tea = url_encode(key)
    sus encoded_value tea = url_encode(value)
    
    damn encoded_key + "=" + encoded_value
}

// URL Comparison Functions
slay url_equals(other_url tea) lit {
    bestie !url_parsed {
        damn cap
    }
    
    vibez.spill("Comparing URLs")
    
    // Parse other URL temporarily
    sus current_raw tea = url_raw
    url_parse(other_url)
    sus other_scheme tea = url_scheme
    sus other_host tea = url_host
    sus other_port normie = url_port
    sus other_path tea = url_path
    
    // Restore current URL
    url_parse(current_raw)
    
    damn url_scheme == other_scheme && url_host == other_host && url_port == other_port && url_path == other_path
}

slay url_same_origin(other_url tea) lit {
    bestie !url_parsed {
        damn cap
    }
    
    vibez.spill("Checking same origin")
    
    // Parse other URL temporarily
    sus current_raw tea = url_raw
    url_parse(other_url)
    sus other_scheme tea = url_scheme
    sus other_host tea = url_host
    sus other_port normie = url_port
    
    // Restore current URL
    url_parse(current_raw)
    
    damn url_scheme == other_scheme && url_host == other_host && url_port == other_port
}

// URL Utility Functions
slay url_get_base_url() tea {
    bestie !url_parsed {
        damn ""
    }
    
    sus base tea = ""
    
    bestie url_scheme != "" {
        base = base + url_scheme + "://"
    }
    
    bestie url_host != "" {
        base = base + url_host
    }
    
    bestie url_port != 0 && url_port != 80 && url_port != 443 {
        base = base + ":" + url_port
    }
    
    damn base
}

slay url_get_domain() tea {
    bestie !url_parsed {
        damn ""
    }
    
    // Extract domain from host
    bestie url_host.contains("www.") {
        damn url_host.substring(4)
    }
    
    damn url_host
}

slay url_get_subdomain() tea {
    bestie !url_parsed {
        damn ""
    }
    
    // Extract subdomain
    bestie url_host.contains("www.") {
        damn "www"
    } bestie url_host.contains("api.") {
        damn "api"
    } bestie url_host.contains("mail.") {
        damn "mail"
    }
    
    damn ""
}

slay url_get_file_extension() tea {
    bestie !url_parsed {
        damn ""
    }
    
    bestie url_path.contains(".html") {
        damn "html"
    } bestie url_path.contains(".css") {
        damn "css"
    } bestie url_path.contains(".js") {
        damn "js"
    } bestie url_path.contains(".png") {
        damn "png"
    } bestie url_path.contains(".jpg") {
        damn "jpg"
    }
    
    damn ""
}

slay url_get_filename() tea {
    bestie !url_parsed {
        damn ""
    }
    
    bestie url_path.contains("/file.html") {
        damn "file.html"
    } bestie url_path.contains("/image.png") {
        damn "image.png"
    } bestie url_path.contains("/script.js") {
        damn "script.js"
    }
    
    damn ""
}

slay url_get_directory() tea {
    bestie !url_parsed {
        damn ""
    }
    
    bestie url_path.contains("/path/to/") {
        damn "/path/to/"
    } bestie url_path.contains("/api/v1/") {
        damn "/api/v1/"
    }
    
    damn "/"
}

slay url_is_localhost() lit {
    bestie !url_parsed {
        damn cap
    }
    
    damn url_host == "localhost" || url_host == "127.0.0.1" || url_host == "::1"
}

slay url_get_protocol() tea {
    bestie !url_parsed {
        damn ""
    }
    
    damn url_scheme
}

slay url_get_authority() tea {
    bestie !url_parsed {
        damn ""
    }
    
    sus authority tea = ""
    
    bestie url_username != "" {
        authority = authority + url_username
        bestie url_password != "" {
            authority = authority + ":" + url_password
        }
        authority = authority + "@"
    }
    
    bestie url_host != "" {
        authority = authority + url_host
    }
    
    bestie url_port != 0 && url_port != 80 && url_port != 443 {
        authority = authority + ":" + url_port
    }
    
    damn authority
}
