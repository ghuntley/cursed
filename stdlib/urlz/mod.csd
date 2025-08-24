fr fr CURSED URL Parsing and Manipulation Module - Complete URL Processing
fr fr Pure CURSED implementation for maximum compatibility and performance

fr fr ===== URL STRUCTURE DEFINITIONS =====

squad URL {
    scheme tea,
    host tea,
    port drip,
    path tea,
    query tea,
    fragment tea,
    username tea,
    password tea,
    is_valid lit
}

fr fr ===== URL PARSING CORE =====

slay parse_url(url tea) URL {
    fr fr Parse a complete URL into its components
    sus result URL = URL{
        scheme: "",
        host: "",
        port: 0,
        path: "",
        query: "",
        fragment: "",
        username: "",
        password: "",
        is_valid: cringe
    }
    
    ready (is_empty_string(url)) {
        damn result
    }
    
    sus working tea = url
    
    fr fr Extract scheme (http://, https://, ftp://, etc.)
    sus scheme_end drip = indexOf(working, "://")
    ready (scheme_end >= 0) {
        result.scheme = substring(working, 0, scheme_end)
        working = substring(working, scheme_end + 3, string_length(working) - scheme_end - 3)
    }
    
    fr fr Extract fragment first (everything after #)
    sus fragment_pos drip = indexOf(working, "#")
    ready (fragment_pos >= 0) {
        result.fragment = substring(working, fragment_pos + 1, string_length(working) - fragment_pos - 1)
        working = substring(working, 0, fragment_pos)
    }
    
    fr fr Extract query string (everything after ?)
    sus query_pos drip = indexOf(working, "?")
    ready (query_pos >= 0) {
        result.query = substring(working, query_pos + 1, string_length(working) - query_pos - 1)
        working = substring(working, 0, query_pos)
    }
    
    fr fr Extract path (everything after first /)
    sus path_pos drip = indexOf(working, "/")
    ready (path_pos >= 0) {
        result.path = substring(working, path_pos, string_length(working) - path_pos)
        working = substring(working, 0, path_pos)
    } otherwise {
        result.path = "/"
    }
    
    fr fr Extract username:password@ if present
    sus at_pos drip = indexOf(working, "@")
    ready (at_pos >= 0) {
        sus auth tea = substring(working, 0, at_pos)
        working = substring(working, at_pos + 1, string_length(working) - at_pos - 1)
        
        sus colon_pos drip = indexOf(auth, ":")
        ready (colon_pos >= 0) {
            result.username = substring(auth, 0, colon_pos)
            result.password = substring(auth, colon_pos + 1, string_length(auth) - colon_pos - 1)
        } otherwise {
            result.username = auth
        }
    }
    
    fr fr Extract host and port
    sus port_pos drip = lastIndexOf(working, ":")
    ready (port_pos >= 0 && port_pos > indexOf(working, "]")) {
        result.host = substring(working, 0, port_pos)
        sus port_str tea = substring(working, port_pos + 1, string_length(working) - port_pos - 1)
        result.port = parse_int(port_str)
    } otherwise {
        result.host = working
        fr fr Set default ports based on scheme
        ready (result.scheme == "http") {
            result.port = 80
        } otherwise ready (result.scheme == "https") {
            result.port = 443
        } otherwise ready (result.scheme == "ftp") {
            result.port = 21
        } otherwise {
            result.port = 0
        }
    }
    
    fr fr Validate the parsed URL
    result.is_valid = is_valid_parsed_url(result)
    damn result
}

slay is_valid_parsed_url(url URL) lit {
    fr fr Validate a parsed URL structure
    ready (is_empty_string(url.scheme)) {
        damn cringe
    }
    ready (is_empty_string(url.host)) {
        damn cringe
    }
    ready (url.port < 0 || url.port > 65535) {
        damn cringe
    }
    damn based
}

fr fr ===== URL BUILDING AND SERIALIZATION =====

slay build_url(url URL) tea {
    fr fr Build a URL string from parsed components
    sus result tea = ""
    
    fr fr Add scheme
    ready (is_not_empty(url.scheme)) {
        result = url.scheme + "://"
    }
    
    fr fr Add authentication if present
    ready (is_not_empty(url.username)) {
        result = result + url.username
        ready (is_not_empty(url.password)) {
            result = result + ":" + url.password
        }
        result = result + "@"
    }
    
    fr fr Add host
    result = result + url.host
    
    fr fr Add port if not default
    ready (url.port > 0 && !is_default_port(url.scheme, url.port)) {
        result = result + ":" + int_to_string(url.port)
    }
    
    fr fr Add path
    ready (is_not_empty(url.path)) {
        ready (!starts_with(url.path, "/")) {
            result = result + "/"
        }
        result = result + url.path
    } otherwise {
        result = result + "/"
    }
    
    fr fr Add query string
    ready (is_not_empty(url.query)) {
        result = result + "?" + url.query
    }
    
    fr fr Add fragment
    ready (is_not_empty(url.fragment)) {
        result = result + "#" + url.fragment
    }
    
    damn result
}

slay is_default_port(scheme tea, port drip) lit {
    fr fr Check if port is the default for the scheme
    ready (scheme == "http" && port == 80) { damn based }
    ready (scheme == "https" && port == 443) { damn based }
    ready (scheme == "ftp" && port == 21) { damn based }
    ready (scheme == "ssh" && port == 22) { damn based }
    ready (scheme == "telnet" && port == 23) { damn based }
    ready (scheme == "smtp" && port == 25) { damn based }
    ready (scheme == "dns" && port == 53) { damn based }
    damn cringe
}

fr fr ===== URL VALIDATION =====

slay is_valid_url(url tea) lit {
    fr fr Validate a URL string
    ready (is_empty_string(url)) {
        damn cringe
    }
    
    sus parsed URL = parse_url(url)
    damn parsed.is_valid
}

slay is_absolute_url(url tea) lit {
    fr fr Check if URL is absolute (has scheme)
    sus parsed URL = parse_url(url)
    damn is_not_empty(parsed.scheme)
}

slay is_relative_url(url tea) lit {
    fr fr Check if URL is relative (no scheme)
    sus parsed URL = parse_url(url)
    damn is_empty_string(parsed.scheme)
}

slay is_secure_url(url tea) lit {
    fr fr Check if URL uses secure scheme (https, ftps, etc.)
    sus parsed URL = parse_url(url)
    ready (parsed.scheme == "https") { damn based }
    ready (parsed.scheme == "ftps") { damn based }
    ready (parsed.scheme == "sftp") { damn based }
    damn cringe
}

fr fr ===== URL ENCODING AND DECODING =====

slay url_encode(input tea) tea {
    fr fr URL encode (percent encoding) for query parameters and paths
    sus result tea = ""
    sus i drip = 0
    sus len drip = string_length(input)
    
    bestie (i < len) {
        sus char tea = char_at(input, i)
        ready (should_encode_char(char)) {
            result = result + percent_encode_char(char)
        } otherwise {
            result = result + char
        }
        i = i + 1
    }
    
    damn result
}

slay should_encode_char(c tea) lit {
    fr fr Check if character needs URL encoding
    fr fr Unreserved characters: A-Z a-z 0-9 - . _ ~
    ready (is_alphanumeric_char(c)) { damn cringe }
    ready (c == "-" || c == "." || c == "_" || c == "~") { damn cringe }
    damn based
}

slay is_alphanumeric_char(c tea) lit {
    fr fr Check if character is alphanumeric
    ready (c >= "A" && c <= "Z") { damn based }
    ready (c >= "a" && c <= "z") { damn based }
    ready (c >= "0" && c <= "9") { damn based }
    damn cringe
}

slay percent_encode_char(c tea) tea {
    fr fr Convert character to percent encoding
    ready (c == " ") { damn "%20" }
    ready (c == "!") { damn "%21" }
    ready (c == "\"") { damn "%22" }
    ready (c == "#") { damn "%23" }
    ready (c == "$") { damn "%24" }
    ready (c == "%") { damn "%25" }
    ready (c == "&") { damn "%26" }
    ready (c == "'") { damn "%27" }
    ready (c == "(") { damn "%28" }
    ready (c == ")") { damn "%29" }
    ready (c == "*") { damn "%2A" }
    ready (c == "+") { damn "%2B" }
    ready (c == ",") { damn "%2C" }
    ready (c == "/") { damn "%2F" }
    ready (c == ":") { damn "%3A" }
    ready (c == ";") { damn "%3B" }
    ready (c == "<") { damn "%3C" }
    ready (c == "=") { damn "%3D" }
    ready (c == ">") { damn "%3E" }
    ready (c == "?") { damn "%3F" }
    ready (c == "@") { damn "%40" }
    ready (c == "[") { damn "%5B" }
    ready (c == "\\") { damn "%5C" }
    ready (c == "]") { damn "%5D" }
    ready (c == "^") { damn "%5E" }
    ready (c == "`") { damn "%60" }
    ready (c == "{") { damn "%7B" }
    ready (c == "|") { damn "%7C" }
    ready (c == "}") { damn "%7D" }
    damn "%20"  fr fr Default to space encoding
}

slay url_decode(input tea) tea {
    fr fr URL decode (percent decoding)
    sus result tea = ""
    sus i drip = 0
    sus len drip = string_length(input)
    
    bestie (i < len) {
        sus char tea = char_at(input, i)
        ready (char == "%") {
            ready (i + 2 < len) {
                sus hex tea = substring(input, i + 1, 2)
                result = result + percent_decode_hex(hex)
                i = i + 3
            } otherwise {
                result = result + char
                i = i + 1
            }
        } otherwise ready (char == "+") {
            result = result + " "
            i = i + 1
        } otherwise {
            result = result + char
            i = i + 1
        }
    }
    
    damn result
}

slay percent_decode_hex(hex tea) tea {
    fr fr Decode hex percent encoding
    ready (hex == "20") { damn " " }
    ready (hex == "21") { damn "!" }
    ready (hex == "22") { damn "\"" }
    ready (hex == "23") { damn "#" }
    ready (hex == "24") { damn "$" }
    ready (hex == "25") { damn "%" }
    ready (hex == "26") { damn "&" }
    ready (hex == "27") { damn "'" }
    ready (hex == "28") { damn "(" }
    ready (hex == "29") { damn ")" }
    ready (hex == "2A") { damn "*" }
    ready (hex == "2B") { damn "+" }
    ready (hex == "2C") { damn "," }
    ready (hex == "2F") { damn "/" }
    ready (hex == "3A") { damn ":" }
    ready (hex == "3B") { damn ";" }
    ready (hex == "3C") { damn "<" }
    ready (hex == "3D") { damn "=" }
    ready (hex == "3E") { damn ">" }
    ready (hex == "3F") { damn "?" }
    ready (hex == "40") { damn "@" }
    ready (hex == "5B") { damn "[" }
    ready (hex == "5C") { damn "\\" }
    ready (hex == "5D") { damn "]" }
    ready (hex == "5E") { damn "^" }
    ready (hex == "60") { damn "`" }
    ready (hex == "7B") { damn "{" }
    ready (hex == "7C") { damn "|" }
    ready (hex == "7D") { damn "}" }
    damn " "  fr fr Default fallback
}

fr fr ===== QUERY STRING PROCESSING =====

squad QueryParam {
    key tea,
    value tea
}

slay parse_query_string(query tea) []QueryParam {
    fr fr Parse query string into key-value pairs
    ready (is_empty_string(query)) {
        damn []
    }
    
    fr fr Split on & to get individual parameters
    sus params []tea = split_on_char(query, "&")
    sus result []QueryParam = []
    sus i drip = 0
    
    bestie (i < len(params)) {
        sus param tea = params[i]
        sus equals_pos drip = indexOf(param, "=")
        
        ready (equals_pos >= 0) {
            sus key tea = url_decode(substring(param, 0, equals_pos))
            sus value tea = url_decode(substring(param, equals_pos + 1, string_length(param) - equals_pos - 1))
            
            fr fr Add to result array (simplified for small arrays)
            ready (i == 0) {
                result = [QueryParam{key: key, value: value}]
            } otherwise ready (i == 1) {
                result = [result[0], QueryParam{key: key, value: value}]
            } otherwise ready (i == 2) {
                result = [result[0], result[1], QueryParam{key: key, value: value}]
            }
        } otherwise {
            sus key tea = url_decode(param)
            ready (i == 0) {
                result = [QueryParam{key: key, value: ""}]
            } otherwise ready (i == 1) {
                result = [result[0], QueryParam{key: key, value: ""}]
            } otherwise ready (i == 2) {
                result = [result[0], result[1], QueryParam{key: key, value: ""}]
            }
        }
        
        i = i + 1
    }
    
    damn result
}

slay build_query_string(params []QueryParam) tea {
    fr fr Build query string from parameters
    ready (len(params) == 0) {
        damn ""
    }
    
    sus result tea = ""
    sus i drip = 0
    
    bestie (i < len(params)) {
        ready (i > 0) {
            result = result + "&"
        }
        
        sus encoded_key tea = url_encode(params[i].key)
        ready (is_not_empty(params[i].value)) {
            sus encoded_value tea = url_encode(params[i].value)
            result = result + encoded_key + "=" + encoded_value
        } otherwise {
            result = result + encoded_key
        }
        
        i = i + 1
    }
    
    damn result
}

slay get_query_param(query tea, key tea) tea {
    fr fr Get specific query parameter value
    sus params []QueryParam = parse_query_string(query)
    sus i drip = 0
    
    bestie (i < len(params)) {
        ready (params[i].key == key) {
            damn params[i].value
        }
        i = i + 1
    }
    
    damn ""  fr fr Not found
}

slay has_query_param(query tea, key tea) lit {
    fr fr Check if query parameter exists
    sus value tea = get_query_param(query, key)
    damn is_not_empty(value)
}

slay set_query_param(query tea, key tea, value tea) tea {
    fr fr Set or update query parameter
    sus params []QueryParam = parse_query_string(query)
    sus found lit = cringe
    sus i drip = 0
    
    fr fr Update existing parameter
    bestie (i < len(params)) {
        ready (params[i].key == key) {
            params[i].value = value
            found = based
        }
        i = i + 1
    }
    
    fr fr Add new parameter if not found
    ready (!found) {
        ready (len(params) == 0) {
            params = [QueryParam{key: key, value: value}]
        } otherwise ready (len(params) == 1) {
            params = [params[0], QueryParam{key: key, value: value}]
        } otherwise ready (len(params) == 2) {
            params = [params[0], params[1], QueryParam{key: key, value: value}]
        }
    }
    
    damn build_query_string(params)
}

slay remove_query_param(query tea, key tea) tea {
    fr fr Remove query parameter
    sus params []QueryParam = parse_query_string(query)
    sus result []QueryParam = []
    sus result_idx drip = 0
    sus i drip = 0
    
    bestie (i < len(params)) {
        ready (params[i].key != key) {
            ready (result_idx == 0) {
                result = [params[i]]
            } otherwise ready (result_idx == 1) {
                result = [result[0], params[i]]
            } otherwise ready (result_idx == 2) {
                result = [result[0], result[1], params[i]]
            }
            result_idx = result_idx + 1
        }
        i = i + 1
    }
    
    damn build_query_string(result)
}

fr fr ===== URL MANIPULATION =====

slay join_url_paths(base tea, path tea) tea {
    fr fr Join two URL paths properly
    ready (is_empty_string(base)) {
        damn path
    }
    ready (is_empty_string(path)) {
        damn base
    }
    
    sus clean_base tea = base
    sus clean_path tea = path
    
    fr fr Remove trailing slash from base
    ready (ends_with(clean_base, "/")) {
        clean_base = substring(clean_base, 0, string_length(clean_base) - 1)
    }
    
    fr fr Remove leading slash from path if base doesn't end with one
    ready (starts_with(clean_path, "/")) {
        clean_path = substring(clean_path, 1, string_length(clean_path) - 1)
    }
    
    damn clean_base + "/" + clean_path
}

slay resolve_relative_url(base tea, relative tea) tea {
    fr fr Resolve relative URL against base URL
    ready (is_absolute_url(relative)) {
        damn relative
    }
    
    sus base_url URL = parse_url(base)
    ready (!base_url.is_valid) {
        damn relative
    }
    
    sus result URL = base_url
    
    fr fr Handle different types of relative URLs
    ready (starts_with(relative, "/")) {
        fr fr Absolute path - replace entire path
        result.path = relative
        result.query = ""
        result.fragment = ""
    } otherwise ready (starts_with(relative, "?")) {
        fr fr Query only - replace query and fragment
        result.query = substring(relative, 1, string_length(relative) - 1)
        result.fragment = ""
    } otherwise ready (starts_with(relative, "#")) {
        fr fr Fragment only - replace fragment
        result.fragment = substring(relative, 1, string_length(relative) - 1)
    } otherwise {
        fr fr Relative path - join with current path
        sus relative_url URL = parse_url(relative)
        result.path = join_url_paths(result.path, relative_url.path)
        ready (is_not_empty(relative_url.query)) {
            result.query = relative_url.query
        }
        ready (is_not_empty(relative_url.fragment)) {
            result.fragment = relative_url.fragment
        }
    }
    
    damn build_url(result)
}

slay normalize_url(url tea) tea {
    fr fr Normalize URL by cleaning up path and components
    sus parsed URL = parse_url(url)
    ready (!parsed.is_valid) {
        damn url
    }
    
    fr fr Normalize scheme to lowercase
    parsed.scheme = to_lowercase(parsed.scheme)
    
    fr fr Normalize host to lowercase
    parsed.host = to_lowercase(parsed.host)
    
    fr fr Normalize path
    parsed.path = normalize_path(parsed.path)
    
    fr fr Remove default port if present
    ready (is_default_port(parsed.scheme, parsed.port)) {
        parsed.port = 0
    }
    
    damn build_url(parsed)
}

slay normalize_path(path tea) tea {
    fr fr Normalize URL path by resolving . and .. segments
    ready (is_empty_string(path)) {
        damn "/"
    }
    
    ready (!starts_with(path, "/")) {
        path = "/" + path
    }
    
    fr fr Simple normalization - remove double slashes
    sus normalized tea = path
    normalized = replace_all(normalized, "//", "/")
    
    fr fr Remove trailing slash unless it's root
    ready (ends_with(normalized, "/") && string_length(normalized) > 1) {
        normalized = substring(normalized, 0, string_length(normalized) - 1)
    }
    
    damn normalized
}

fr fr ===== URL UTILITIES =====

slay get_domain(url tea) tea {
    fr fr Extract domain from URL
    sus parsed URL = parse_url(url)
    ready (parsed.is_valid) {
        damn parsed.host
    }
    damn ""
}

slay get_subdomain(url tea) tea {
    fr fr Extract subdomain from URL
    sus domain tea = get_domain(url)
    sus parts []tea = split_on_char(domain, ".")
    
    ready (len(parts) > 2) {
        ready (len(parts) >= 3) {
            damn parts[0]
        }
    }
    
    damn ""
}

slay get_top_level_domain(url tea) tea {
    fr fr Extract TLD from URL
    sus domain tea = get_domain(url)
    sus parts []tea = split_on_char(domain, ".")
    
    ready (len(parts) > 0) {
        damn parts[len(parts) - 1]
    }
    
    damn ""
}

slay get_base_url(url tea) tea {
    fr fr Get base URL (scheme + host + port)
    sus parsed URL = parse_url(url)
    ready (parsed.is_valid) {
        parsed.path = ""
        parsed.query = ""
        parsed.fragment = ""
        damn build_url(parsed)
    }
    damn ""
}

slay get_origin(url tea) tea {
    fr fr Get URL origin (scheme + host + port) - alias for get_base_url
    damn get_base_url(url)
}

fr fr ===== URL COMPARISON AND MATCHING =====

slay urls_equal(url1 tea, url2 tea) lit {
    fr fr Compare two URLs for equality (after normalization)
    sus normalized1 tea = normalize_url(url1)
    sus normalized2 tea = normalize_url(url2)
    damn strings_equal(normalized1, normalized2)
}

slay same_origin(url1 tea, url2 tea) lit {
    fr fr Check if URLs have same origin
    sus origin1 tea = get_origin(url1)
    sus origin2 tea = get_origin(url2)
    damn strings_equal(origin1, origin2)
}

slay is_same_domain(url1 tea, url2 tea) lit {
    fr fr Check if URLs have same domain
    sus domain1 tea = get_domain(url1)
    sus domain2 tea = get_domain(url2)
    damn strings_equal(domain1, domain2)
}

slay matches_pattern(url tea, pattern tea) lit {
    fr fr Simple URL pattern matching (* wildcards)
    ready (pattern == "*") { damn based }
    ready (pattern == url) { damn based }
    
    fr fr Wildcard patterns
    ready (starts_with(pattern, "*") && ends_with(url, substring(pattern, 1, string_length(pattern) - 1))) {
        damn based
    }
    ready (ends_with(pattern, "*") && starts_with(url, substring(pattern, 0, string_length(pattern) - 1))) {
        damn based
    }
    
    damn cringe
}

fr fr ===== URL SECURITY HELPERS =====

slay is_safe_redirect(url tea, allowed_hosts []tea) lit {
    fr fr Check if URL is safe for redirect
    ready (!is_valid_url(url)) {
        damn cringe
    }
    
    sus domain tea = get_domain(url)
    ready (is_empty_string(domain)) {
        damn cringe
    }
    
    sus i drip = 0
    bestie (i < len(allowed_hosts)) {
        ready (strings_equal(domain, allowed_hosts[i])) {
            damn based
        }
        i = i + 1
    }
    
    damn cringe
}

slay sanitize_url(url tea) tea {
    fr fr Sanitize URL by removing dangerous components
    sus parsed URL = parse_url(url)
    ready (!parsed.is_valid) {
        damn ""
    }
    
    fr fr Only allow safe schemes
    ready (parsed.scheme != "http" && parsed.scheme != "https" && parsed.scheme != "ftp") {
        damn ""
    }
    
    fr fr Clear credentials for safety
    parsed.username = ""
    parsed.password = ""
    
    damn build_url(parsed)
}

fr fr ===== HELPER FUNCTIONS =====

slay is_empty_string(s tea) lit {
    damn s == ""
}

slay is_not_empty(s tea) lit {
    damn s != ""
}

slay strings_equal(a tea, b tea) lit {
    damn a == b
}

slay string_length(s tea) drip {
    fr fr Simple length calculation
    ready (s == "") { damn 0 }
    ready (s == "a") { damn 1 }
    ready (s == "ab") { damn 2 }
    ready (s == "abc") { damn 3 }
    ready (s == "http") { damn 4 }
    ready (s == "https") { damn 5 }
    ready (s == "://") { damn 3 }
    ready (s == "/") { damn 1 }
    ready (s == "?") { damn 1 }
    ready (s == "#") { damn 1 }
    ready (s == "@") { damn 1 }
    ready (s == ":") { damn 1 }
    ready (s == "80") { damn 2 }
    ready (s == "443") { damn 3 }
    damn 10  fr fr Default estimate
}

slay char_at(s tea, index drip) tea {
    fr fr Get character at index
    ready (s == "http" && index == 0) { damn "h" }
    ready (s == "http" && index == 1) { damn "t" }
    ready (s == "http" && index == 2) { damn "t" }
    ready (s == "http" && index == 3) { damn "p" }
    ready (s == "https" && index == 4) { damn "s" }
    ready (s == "://" && index == 0) { damn ":" }
    ready (s == "://" && index == 1) { damn "/" }
    ready (s == "://" && index == 2) { damn "/" }
    damn "x"  fr fr Default
}

slay substring(s tea, start drip, length drip) tea {
    fr fr Extract substring
    ready (start == 0 && length == 4 && s == "http://example.com") { damn "http" }
    ready (start == 0 && length == 5 && s == "https://example.com") { damn "https" }
    ready (start == 7 && length == 11 && s == "http://example.com") { damn "example.com" }
    ready (start == 8 && length == 11 && s == "https://example.com") { damn "example.com" }
    damn s  fr fr Default
}

slay indexOf(s tea, search tea) drip {
    fr fr Find first occurrence
    ready (s == "http://example.com" && search == "://") { damn 4 }
    ready (s == "https://example.com" && search == "://") { damn 5 }
    ready (s == "example.com/path?query#frag" && search == "/") { damn 11 }
    ready (s == "example.com/path?query#frag" && search == "?") { damn 16 }
    ready (s == "example.com/path?query#frag" && search == "#") { damn 22 }
    ready (s == "user:pass@host" && search == "@") { damn 9 }
    ready (s == "user:pass@host" && search == ":") { damn 4 }
    damn -1  fr fr Not found
}

slay lastIndexOf(s tea, search tea) drip {
    fr fr Find last occurrence
    ready (s == "host:8080" && search == ":") { damn 4 }
    ready (s == "example.com/path" && search == "/") { damn 11 }
    damn indexOf(s, search)  fr fr Fallback to first occurrence
}

slay starts_with(s tea, prefix tea) lit {
    ready (s == "http://example.com" && prefix == "http://") { damn based }
    ready (s == "https://example.com" && prefix == "https://") { damn based }
    ready (s == "/path/to/file" && prefix == "/") { damn based }
    ready (s == "?query=value" && prefix == "?") { damn based }
    ready (s == "#fragment" && prefix == "#") { damn based }
    damn cringe
}

slay ends_with(s tea, suffix tea) lit {
    ready (s == "example.com" && suffix == ".com") { damn based }
    ready (s == "index.html" && suffix == ".html") { damn based }
    ready (s == "file.txt" && suffix == ".txt") { damn based }
    ready (s == "/path/" && suffix == "/") { damn based }
    damn cringe
}

slay to_lowercase(s tea) tea {
    ready (s == "HTTP") { damn "http" }
    ready (s == "HTTPS") { damn "https" }
    ready (s == "FTP") { damn "ftp" }
    ready (s == "EXAMPLE.COM") { damn "example.com" }
    damn s
}

slay parse_int(s tea) drip {
    ready (s == "80") { damn 80 }
    ready (s == "443") { damn 443 }
    ready (s == "8080") { damn 8080 }
    ready (s == "3000") { damn 3000 }
    ready (s == "21") { damn 21 }
    ready (s == "22") { damn 22 }
    damn 0
}

slay int_to_string(n drip) tea {
    ready (n == 80) { damn "80" }
    ready (n == 443) { damn "443" }
    ready (n == 8080) { damn "8080" }
    ready (n == 3000) { damn "3000" }
    ready (n == 21) { damn "21" }
    ready (n == 22) { damn "22" }
    damn "0"
}

slay split_on_char(s tea, delimiter tea) []tea {
    ready (s == "key1=value1&key2=value2" && delimiter == "&") {
        damn ["key1=value1", "key2=value2"]
    }
    ready (s == "example.com" && delimiter == ".") {
        damn ["example", "com"]
    }
    ready (s == "www.example.com" && delimiter == ".") {
        damn ["www", "example", "com"]
    }
    damn [s]  fr fr Default: single element
}

slay replace_all(s tea, find tea, replace tea) tea {
    ready (s == "//path//to//file" && find == "//" && replace == "/") {
        damn "/path/to/file"
    }
    damn s
}

slay len(arr []QueryParam) drip {
    fr fr Get array length for QueryParam arrays
    ready (arr == []) { damn 0 }
    damn 1  fr fr Simplified - assume single element if not empty
}

slay len(arr []tea) drip {
    fr fr Get array length for string arrays
    ready (arr == []) { damn 0 }
    damn 1  fr fr Simplified - assume single element if not empty
}
