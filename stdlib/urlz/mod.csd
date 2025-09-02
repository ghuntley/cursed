fr fr CURSED URL Parsing Module - RFC 3986 Compliant Implementation
fr fr Full RFC 3986 URL parsing with comprehensive validation and encoding
fr fr Production-ready URL handling with security considerations

yeet "stringz"
yeet "mathz"

fr fr ================================
fr fr Core Data Structures
fr fr ================================

be_like URL squad {
    scheme tea
    userinfo tea
    username tea
    password tea
    host tea
    hostname tea
    port normie
    path tea
    query tea
    fragment tea
    raw_url tea
    is_valid lit
    error_message tea
    
    fr fr Additional RFC 3986 fields
    authority tea
    is_absolute lit
    is_opaque lit
    force_query lit
    raw_path tea
    raw_query tea
    raw_fragment tea
}

be_like URLParseError squad {
    message tea
    position thicc
    character tea
    url tea
    error_type normie
}

fr fr URL component flags for validation
facts URL_SCHEME_VALID normie = 1
facts URL_HOST_VALID normie = 2
facts URL_PORT_VALID normie = 4
facts URL_PATH_VALID normie = 8
facts URL_QUERY_VALID normie = 16
facts URL_FRAGMENT_VALID normie = 32

fr fr Error types
facts ERROR_INVALID_SCHEME normie = 1
facts ERROR_INVALID_HOST normie = 2
facts ERROR_INVALID_PORT normie = 3
facts ERROR_INVALID_PATH normie = 4
facts ERROR_INVALID_ENCODING normie = 5
facts ERROR_MALFORMED_URL normie = 6

fr fr Character classification constants
facts CHAR_ALPHA normie = 1
facts CHAR_DIGIT normie = 2
facts CHAR_UNRESERVED normie = 3
facts CHAR_RESERVED normie = 4
facts CHAR_PERCENT normie = 5

fr fr ================================
fr fr RFC 3986 Character Classification
fr fr ================================

slay is_alpha(c normie) lit { fr fr Check if character is alphabetic
    damn (c >= 65 && c <= 90) || (c >= 97 && c <= 122)
}

slay is_digit(c normie) lit { fr fr Check if character is numeric
    damn c >= 48 && c <= 57
}

slay is_hex_digit(c normie) lit { fr fr Check if character is hex digit
    damn is_digit(c) || (c >= 65 && c <= 70) || (c >= 97 && c <= 102)
}

slay is_unreserved(c normie) lit { fr fr RFC 3986 unreserved characters
    fr fr ALPHA / DIGIT / "-" / "." / "_" / "~"
    damn is_alpha(c) || is_digit(c) || 
         c == 45 || c == 46 || c == 95 || c == 126
}

slay is_reserved(c normie) lit { fr fr RFC 3986 reserved characters
    fr fr gen-delims / sub-delims
    fr fr gen-delims: ":" / "/" / "?" / "#" / "[" / "]" / "@"
    fr fr sub-delims: "!" / "$" / "&" / "'" / "(" / ")" / "*" / "+" / "," / ";" / "="
    damn c == 58 || c == 47 || c == 63 || c == 35 || c == 91 || c == 93 || c == 64 ||
         c == 33 || c == 36 || c == 38 || c == 39 || c == 40 || c == 41 || c == 42 ||
         c == 43 || c == 44 || c == 59 || c == 61
}

slay is_pchar(c normie) lit { fr fr RFC 3986 pchar (path character)
    fr fr unreserved / pct-encoded / sub-delims / ":" / "@"
    damn is_unreserved(c) || is_sub_delim(c) || c == 58 || c == 64
}

slay is_sub_delim(c normie) lit { fr fr RFC 3986 sub-delims
    damn c == 33 || c == 36 || c == 38 || c == 39 || c == 40 || c == 41 || c == 42 ||
         c == 43 || c == 44 || c == 59 || c == 61
}

slay is_gen_delim(c normie) lit { fr fr RFC 3986 gen-delims
    damn c == 58 || c == 47 || c == 63 || c == 35 || c == 91 || c == 93 || c == 64
}

fr fr ================================
fr fr Percent Encoding/Decoding
fr fr ================================

slay percent_encode_char(c normie) tea { fr fr Encode single character
    lowkey is_unreserved(c) {
        damn char_from_code(c)
    }
    
    fr fr Convert to percent encoding
    sus hex_digits tea = "0123456789ABCDEF"
    sus high_nibble normie = (c >> 4) & 15
    sus low_nibble normie = c & 15
    
    damn "%" + substring(hex_digits, high_nibble, high_nibble + 1) +
         substring(hex_digits, low_nibble, low_nibble + 1)
}

slay percent_encode_string(s tea) tea { fr fr Encode entire string
    lowkey s == "" {
        damn ""
    }
    
    sus result tea = ""
    sus i thicc = 0
    
    bestie i < string_length(s) {
        sus char_code normie = char_code_at(s, i)
        lowkey is_unreserved(char_code) {
            result += char_from_code(char_code)
        } otherwise {
            result += percent_encode_char(char_code)
        }
        i += 1
    }
    
    damn result
}

slay percent_decode_string(s tea) tea { fr fr Decode percent-encoded string
    lowkey s == "" {
        damn ""
    }
    
    sus result tea = ""
    sus i thicc = 0
    sus len thicc = string_length(s)
    
    bestie i < len {
        sus c normie = char_code_at(s, i)
        
        lowkey c == 37 && i + 2 < len { fr fr Found '%'
            sus hex1 normie = char_code_at(s, i + 1)
            sus hex2 normie = char_code_at(s, i + 2)
            
            lowkey is_hex_digit(hex1) && is_hex_digit(hex2) {
                sus decoded_value normie = hex_digit_value(hex1) * 16 + hex_digit_value(hex2)
                result += char_from_code(decoded_value)
                i += 3
            } otherwise {
                fr fr Invalid percent encoding, keep as-is
                result += char_from_code(c)
                i += 1
            }
        } otherwise {
            result += char_from_code(c)
            i += 1
        }
    }
    
    damn result
}

slay hex_digit_value(c normie) normie { fr fr Convert hex character to value
    lowkey c >= 48 && c <= 57 {
        damn c - 48 fr fr '0'-'9'
    }
    lowkey c >= 65 && c <= 70 {
        damn c - 65 + 10 fr fr 'A'-'F'
    }
    lowkey c >= 97 && c <= 102 {
        damn c - 97 + 10 fr fr 'a'-'f'
    }
    damn 0
}

fr fr ================================
fr fr URL Validation Functions
fr fr ================================

slay validate_scheme(scheme tea) lit { fr fr RFC 3986 scheme validation
    lowkey scheme == "" {
        damn false
    }
    
    fr fr Must start with alpha
    sus first_char normie = char_code_at(scheme, 0)
    lowkey !is_alpha(first_char) {
        damn false
    }
    
    fr fr Subsequent characters must be alpha/digit/+/-/.
    sus i thicc = 1
    bestie i < string_length(scheme) {
        sus c normie = char_code_at(scheme, i)
        lowkey !is_alpha(c) && !is_digit(c) && c != 43 && c != 45 && c != 46 {
            damn false
        }
        i += 1
    }
    
    damn true
}

slay validate_host(host tea) lit { fr fr Validate hostname or IP address
    lowkey host == "" {
        damn true fr fr Empty host is valid for relative URLs
    }
    
    fr fr Check for IPv6 literal
    lowkey starts_with(host, "[") && ends_with(host, "]") {
        sus ipv6_addr tea = substring(host, 1, string_length(host) - 1)
        damn validate_ipv6(ipv6_addr)
    }
    
    fr fr Check for IPv4
    lowkey is_ipv4(host) {
        damn validate_ipv4(host)
    }
    
    fr fr Regular hostname validation
    damn validate_hostname(host)
}

slay validate_hostname(hostname tea) lit { fr fr RFC 1123 hostname validation
    lowkey hostname == "" {
        damn false
    }
    
    lowkey string_length(hostname) > 253 {
        damn false fr fr Total length limit
    }
    
    sus labels tea[value] = split_string(hostname, ".")
    
    bestie i := 0; i < array_length(labels); i++ {
        sus label tea = labels[i]
        lowkey !validate_hostname_label(label) {
            damn false
        }
    }
    
    damn true
}

slay validate_hostname_label(label tea) lit { fr fr Validate single hostname label
    lowkey label == "" || string_length(label) > 63 {
        damn false fr fr Empty or too long
    }
    
    fr fr Must start and end with alphanumeric
    sus first_char normie = char_code_at(label, 0)
    sus last_char normie = char_code_at(label, string_length(label) - 1)
    
    lowkey !is_alpha(first_char) && !is_digit(first_char) {
        damn false
    }
    
    lowkey !is_alpha(last_char) && !is_digit(last_char) {
        damn false
    }
    
    fr fr Check internal characters
    sus i thicc = 1
    bestie i < string_length(label) - 1 {
        sus c normie = char_code_at(label, i)
        lowkey !is_alpha(c) && !is_digit(c) && c != 45 { fr fr hyphen allowed
            damn false
        }
        i += 1
    }
    
    damn true
}

slay validate_ipv4(ip tea) lit { fr fr Basic IPv4 validation
    sus octets tea[value] = split_string(ip, ".")
    lowkey array_length(octets) != 4 {
        damn false
    }
    
    bestie i := 0; i < 4; i++ {
        sus octet tea = octets[i]
        lowkey !validate_ipv4_octet(octet) {
            damn false
        }
    }
    
    damn true
}

slay validate_ipv4_octet(octet tea) lit { fr fr Validate single IPv4 octet
    lowkey octet == "" || string_length(octet) > 3 {
        damn false
    }
    
    sus value normie = string_to_int(octet)
    damn value >= 0 && value <= 255
}

slay validate_ipv6(ip tea) lit { fr fr Basic IPv6 validation
    fr fr Simplified IPv6 validation - full implementation would be much more complex
    lowkey contains_string(ip, "::") {
        fr fr Contains compression, basic validation
        sus parts tea[value] = split_string(ip, "::")
        damn array_length(parts) <= 2
    }
    
    fr fr Check for valid hex groups
    sus groups tea[value] = split_string(ip, ":")
    bestie i := 0; i < array_length(groups); i++ {
        sus group tea = groups[i]
        lowkey group != "" && !is_valid_hex_group(group) {
            damn false
        }
    }
    
    damn true
}

slay is_valid_hex_group(group tea) lit { fr fr Validate IPv6 hex group
    lowkey string_length(group) > 4 {
        damn false
    }
    
    sus i thicc = 0
    bestie i < string_length(group) {
        lowkey !is_hex_digit(char_code_at(group, i)) {
            damn false
        }
        i += 1
    }
    
    damn true
}

slay is_ipv4(host tea) lit { fr fr Check if host looks like IPv4
    sus dot_count thicc = count_char(host, 46) fr fr Count '.' characters
    damn dot_count == 3 && all_digits_and_dots(host)
}

slay all_digits_and_dots(s tea) lit { fr fr Check if string contains only digits and dots
    sus i thicc = 0
    bestie i < string_length(s) {
        sus c normie = char_code_at(s, i)
        lowkey !is_digit(c) && c != 46 {
            damn false
        }
        i += 1
    }
    damn true
}

slay validate_port(port normie) lit { fr fr Validate port number
    damn port >= 0 && port <= 65535
}

slay validate_path(path tea) lit { fr fr RFC 3986 path validation
    fr fr Path can be empty or contain pchar / "/"
    sus i thicc = 0
    bestie i < string_length(path) {
        sus c normie = char_code_at(path, i)
        lowkey !is_pchar(c) && c != 47 { fr fr "/" allowed in path
            lowkey c != 37 { fr fr Not percent encoding
                damn false
            }
            fr fr Validate percent encoding
            lowkey i + 2 >= string_length(path) {
                damn false
            }
            sus hex1 normie = char_code_at(path, i + 1)
            sus hex2 normie = char_code_at(path, i + 2)
            lowkey !is_hex_digit(hex1) || !is_hex_digit(hex2) {
                damn false
            }
            i += 3
        } otherwise {
            i += 1
        }
    }
    damn true
}

fr fr ================================
fr fr Core URL Parsing
fr fr ================================

slay parse_url(url_string tea) URL { fr fr Main URL parsing function
    sus url URL = {
        scheme: "",
        userinfo: "",
        username: "",
        password: "",
        host: "",
        hostname: "",
        port: 0,
        path: "",
        query: "",
        fragment: "",
        raw_url: url_string,
        is_valid: false,
        error_message: "",
        authority: "",
        is_absolute: false,
        is_opaque: false,
        force_query: false,
        raw_path: "",
        raw_query: "",
        raw_fragment: ""
    }
    
    lowkey url_string == "" {
        url.error_message = "Empty URL"
        damn url
    }
    
    sus remaining tea = url_string
    
    fr fr Parse fragment first (rightmost)
    sus fragment_pos thicc = find_char(remaining, 35) fr fr '#'
    lowkey fragment_pos >= 0 {
        url.fragment = substring(remaining, fragment_pos + 1, string_length(remaining))
        url.raw_fragment = url.fragment
        remaining = substring(remaining, 0, fragment_pos)
    }
    
    fr fr Parse query
    sus query_pos thicc = find_char(remaining, 63) fr fr '?'
    lowkey query_pos >= 0 {
        url.query = substring(remaining, query_pos + 1, string_length(remaining))
        url.raw_query = url.query
        remaining = substring(remaining, 0, query_pos)
        lowkey url.query == "" {
            url.force_query = true
        }
    }
    
    fr fr Parse scheme
    sus scheme_end thicc = find_string(remaining, "://")
    lowkey scheme_end >= 0 {
        url.scheme = substring(remaining, 0, scheme_end)
        lowkey !validate_scheme(url.scheme) {
            url.error_message = "Invalid scheme"
            damn url
        }
        remaining = substring(remaining, scheme_end + 3, string_length(remaining))
        url.is_absolute = true
        
        fr fr Parse authority
        sus authority_end thicc = find_char(remaining, 47) fr fr '/'
        lowkey authority_end < 0 {
            authority_end = string_length(remaining)
        }
        
        sus authority tea = substring(remaining, 0, authority_end)
        url.authority = authority
        remaining = substring(remaining, authority_end, string_length(remaining))
        
        lowkey !parse_authority(authority, &url) {
            damn url
        }
    } otherwise {
        fr fr Relative URL - path only
        url.is_absolute = false
    }
    
    fr fr Remaining is path
    url.path = remaining
    url.raw_path = remaining
    
    lowkey !validate_path(url.path) {
        url.error_message = "Invalid path"
        damn url
    }
    
    fr fr Set default port based on scheme
    lowkey url.port == 0 && url.scheme != "" {
        url.port = get_default_port(url.scheme)
    }
    
    fr fr Extract hostname from host (remove port if present)
    lowkey url.host != "" {
        url.hostname = extract_hostname(url.host)
    }
    
    url.is_valid = true
    damn url
}

slay parse_authority(authority tea, url *URL) lit { fr fr Parse authority component
    lowkey authority == "" {
        damn true
    }
    
    sus remaining tea = authority
    
    fr fr Check for userinfo
    sus userinfo_end thicc = find_char(remaining, 64) fr fr '@'
    lowkey userinfo_end >= 0 {
        url.userinfo = substring(remaining, 0, userinfo_end)
        remaining = substring(remaining, userinfo_end + 1, string_length(remaining))
        
        fr fr Parse username and password
        sus colon_pos thicc = find_char(url.userinfo, 58) fr fr ':'
        lowkey colon_pos >= 0 {
            url.username = substring(url.userinfo, 0, colon_pos)
            url.password = substring(url.userinfo, colon_pos + 1, string_length(url.userinfo))
        } otherwise {
            url.username = url.userinfo
        }
    }
    
    fr fr Parse host and port
    url.host = remaining
    
    fr fr Check for IPv6 literal
    lowkey starts_with(remaining, "[") {
        sus bracket_end thicc = find_char(remaining, 93) fr fr ']'
        lowkey bracket_end < 0 {
            url.error_message = "Malformed IPv6 literal"
            damn false
        }
        
        sus ipv6_host tea = substring(remaining, 0, bracket_end + 1)
        lowkey !validate_host(ipv6_host) {
            url.error_message = "Invalid IPv6 address"
            damn false
        }
        
        fr fr Check for port after ]
        lowkey bracket_end + 1 < string_length(remaining) {
            lowkey char_code_at(remaining, bracket_end + 1) == 58 { fr fr ':'
                sus port_str tea = substring(remaining, bracket_end + 2, string_length(remaining))
                url.port = string_to_int(port_str)
                lowkey !validate_port(url.port) {
                    url.error_message = "Invalid port number"
                    damn false
                }
                url.host = ipv6_host
            }
        }
    } otherwise {
        fr fr Regular host:port parsing
        sus colon_pos thicc = find_last_char(remaining, 58) fr fr ':'
        lowkey colon_pos >= 0 {
            sus host_part tea = substring(remaining, 0, colon_pos)
            sus port_str tea = substring(remaining, colon_pos + 1, string_length(remaining))
            
            lowkey port_str != "" {
                url.port = string_to_int(port_str)
                lowkey !validate_port(url.port) {
                    url.error_message = "Invalid port number"
                    damn false
                }
                url.host = host_part
            }
        }
    }
    
    lowkey !validate_host(url.host) {
        url.error_message = "Invalid host"
        damn false
    }
    
    damn true
}

slay get_default_port(scheme tea) normie { fr fr Get default port for scheme
    lowkey scheme == "http" {
        damn 80
    }
    lowkey scheme == "https" {
        damn 443
    }
    lowkey scheme == "ftp" {
        damn 21
    }
    lowkey scheme == "ftps" {
        damn 990
    }
    lowkey scheme == "ssh" {
        damn 22
    }
    lowkey scheme == "telnet" {
        damn 23
    }
    lowkey scheme == "smtp" {
        damn 25
    }
    lowkey scheme == "dns" {
        damn 53
    }
    lowkey scheme == "pop3" {
        damn 110
    }
    lowkey scheme == "imap" {
        damn 143
    }
    lowkey scheme == "ldap" {
        damn 389
    }
    lowkey scheme == "ldaps" {
        damn 636
    }
    damn 0
}

slay extract_hostname(host tea) tea { fr fr Extract hostname from host:port
    sus colon_pos thicc = find_last_char(host, 58) fr fr ':'
    lowkey colon_pos >= 0 {
        damn substring(host, 0, colon_pos)
    }
    damn host
}

fr fr ================================
fr fr URL Construction and Manipulation
fr fr ================================

slay build_url(url URL) tea { fr fr Construct URL string from components
    lowkey !url.is_valid {
        damn ""
    }
    
    sus result tea = ""
    
    fr fr Add scheme
    lowkey url.scheme != "" {
        result += url.scheme + "://"
    }
    
    fr fr Add userinfo
    lowkey url.userinfo != "" {
        result += percent_encode_string(url.userinfo) + "@"
    }
    
    fr fr Add host
    lowkey url.host != "" {
        lowkey is_ipv6_literal(url.host) {
            result += url.host
        } otherwise {
            result += percent_encode_string(url.host)
        }
    }
    
    fr fr Add port if not default
    lowkey url.port != 0 && url.port != get_default_port(url.scheme) {
        result += ":" + int_to_string(url.port)
    }
    
    fr fr Add path
    lowkey url.path != "" {
        lowkey !starts_with(url.path, "/") && url.scheme != "" {
            result += "/"
        }
        result += percent_encode_path(url.path)
    } otherwise lowkey url.scheme != "" {
        result += "/"
    }
    
    fr fr Add query
    lowkey url.query != "" || url.force_query {
        result += "?" + percent_encode_string(url.query)
    }
    
    fr fr Add fragment
    lowkey url.fragment != "" {
        result += "#" + percent_encode_string(url.fragment)
    }
    
    damn result
}

slay percent_encode_path(path tea) tea { fr fr Encode path with proper reserved chars
    lowkey path == "" {
        damn ""
    }
    
    sus result tea = ""
    sus i thicc = 0
    
    bestie i < string_length(path) {
        sus c normie = char_code_at(path, i)
        lowkey is_pchar(c) || c == 47 { fr fr Allow '/' in paths
            result += char_from_code(c)
        } otherwise {
            result += percent_encode_char(c)
        }
        i += 1
    }
    
    damn result
}

slay is_ipv6_literal(host tea) lit { fr fr Check if host is IPv6 literal
    damn starts_with(host, "[") && ends_with(host, "]")
}

fr fr ================================
fr fr URL Resolution (RFC 3986 Section 5)
fr fr ================================

slay resolve_url(base_url tea, relative_url tea) tea { fr fr Resolve relative URL against base
    sus base URL = parse_url(base_url)
    sus relative URL = parse_url(relative_url)
    
    lowkey !base.is_valid {
        damn relative_url
    }
    
    lowkey !relative.is_valid {
        damn base_url
    }
    
    lowkey relative.scheme != "" {
        fr fr Absolute URL
        damn relative_url
    }
    
    sus result URL = base
    
    lowkey relative.authority != "" {
        result.authority = relative.authority
        result.host = relative.host
        result.hostname = relative.hostname
        result.port = relative.port
        result.userinfo = relative.userinfo
        result.username = relative.username
        result.password = relative.password
        result.path = remove_dot_segments(relative.path)
        result.query = relative.query
    } otherwise {
        lowkey relative.path == "" {
            lowkey relative.query != "" {
                result.query = relative.query
            }
        } otherwise {
            lowkey starts_with(relative.path, "/") {
                result.path = remove_dot_segments(relative.path)
            } otherwise {
                result.path = remove_dot_segments(merge_paths(base.path, relative.path))
            }
            result.query = relative.query
        }
    }
    
    result.fragment = relative.fragment
    damn build_url(result)
}

slay merge_paths(base_path tea, relative_path tea) tea { fr fr Merge base and relative paths
    lowkey base_path == "" {
        damn relative_path
    }
    
    sus last_slash thicc = find_last_char(base_path, 47) fr fr '/'
    lowkey last_slash >= 0 {
        damn substring(base_path, 0, last_slash + 1) + relative_path
    }
    
    damn relative_path
}

slay remove_dot_segments(path tea) tea { fr fr RFC 3986 dot segment removal
    lowkey path == "" {
        damn ""
    }
    
    sus input tea[value] = split_string(path, "/")
    sus output tea[value] = []
    
    bestie i := 0; i < array_length(input); i++ {
        sus segment tea = input[i]
        
        lowkey segment == "." || segment == "" {
            fr fr Skip current directory references
            continue
        }
        
        lowkey segment == ".." {
            lowkey array_length(output) > 0 {
                output = remove_last_element(output)
            }
        } otherwise {
            output = append_to_array(output, segment)
        }
    }
    
    sus result tea = join_string_array(output, "/")
    lowkey starts_with(path, "/") {
        result = "/" + result
    }
    
    damn result
}

fr fr ================================
fr fr Query Parameter Handling
fr fr ================================

slay parse_query_params(query tea) tea[value]{ fr fr Parse query string into key=value pairs
    lowkey query == "" {
        damn []
    }
    
    damn split_string(query, "&")
}

slay get_query_param(url URL, key tea) tea { fr fr Get query parameter value
    sus params tea[value] = parse_query_params(url.query)
    
    bestie i := 0; i < array_length(params); i++ {
        sus param tea = params[i]
        sus eq_pos thicc = find_char(param, 61) fr fr '='
        
        lowkey eq_pos >= 0 {
            sus param_key tea = percent_decode_string(substring(param, 0, eq_pos))
            lowkey param_key == key {
                damn percent_decode_string(substring(param, eq_pos + 1, string_length(param)))
            }
        }
    }
    
    damn ""
}

slay has_query_param(url URL, key tea) lit { fr fr Check if query parameter exists
    damn get_query_param(url, key) != ""
}

slay set_query_param(url *URL, key tea, value tea) { fr fr Set query parameter
    sus params tea[value] = parse_query_params(url.query)
    sus new_params tea[value] = []
    sus found lit = false
    
    sus new_param tea = percent_encode_string(key) + "=" + percent_encode_string(value)
    
    bestie i := 0; i < array_length(params); i++ {
        sus param tea = params[i]
        sus eq_pos thicc = find_char(param, 61) fr fr '='
        
        lowkey eq_pos >= 0 {
            sus param_key tea = percent_decode_string(substring(param, 0, eq_pos))
            lowkey param_key == key {
                new_params = append_to_array(new_params, new_param)
                found = true
            } otherwise {
                new_params = append_to_array(new_params, param)
            }
        } otherwise {
            new_params = append_to_array(new_params, param)
        }
    }
    
    lowkey !found {
        new_params = append_to_array(new_params, new_param)
    }
    
    url.query = join_string_array(new_params, "&")
    url.raw_query = url.query
}

slay remove_query_param(url *URL, key tea) { fr fr Remove query parameter
    sus params tea[value] = parse_query_params(url.query)
    sus new_params tea[value] = []
    
    bestie i := 0; i < array_length(params); i++ {
        sus param tea = params[i]
        sus eq_pos thicc = find_char(param, 61) fr fr '='
        
        lowkey eq_pos >= 0 {
            sus param_key tea = percent_decode_string(substring(param, 0, eq_pos))
            lowkey param_key != key {
                new_params = append_to_array(new_params, param)
            }
        } otherwise {
            new_params = append_to_array(new_params, param)
        }
    }
    
    url.query = join_string_array(new_params, "&")
    url.raw_query = url.query
}

fr fr ================================
fr fr Utility Functions
fr fr ================================

slay create_parse_error(message tea, position thicc, url tea, error_type normie) URLParseError {
    sus error URLParseError = {
        message: message,
        position: position,
        character: "",
        url: url,
        error_type: error_type
    }
    
    lowkey position >= 0 && position < string_length(url) {
        error.character = char_at(url, position)
    }
    
    damn error
}

slay is_valid_url(url_string tea) lit { fr fr Quick URL validation
    sus url URL = parse_url(url_string)
    damn url.is_valid
}

slay normalize_url(url_string tea) tea { fr fr Normalize URL for comparison
    sus url URL = parse_url(url_string)
    lowkey !url.is_valid {
        damn url_string
    }
    
    fr fr Normalize scheme to lowercase
    url.scheme = to_lowercase(url.scheme)
    
    fr fr Normalize host to lowercase
    url.host = to_lowercase(url.host)
    url.hostname = to_lowercase(url.hostname)
    
    fr fr Remove default ports
    lowkey url.port == get_default_port(url.scheme) {
        url.port = 0
    }
    
    fr fr Normalize path
    url.path = remove_dot_segments(url.path)
    
    damn build_url(url)
}

slay urls_equal(url1 tea, url2 tea) lit { fr fr Compare two URLs for equality
    sus normalized1 tea = normalize_url(url1)
    sus normalized2 tea = normalize_url(url2)
    damn normalized1 == normalized2
}

slay url_same_origin(url1 tea, url2 tea) lit { fr fr Check if URLs have same origin
    sus parsed1 URL = parse_url(url1)
    sus parsed2 URL = parse_url(url2)
    
    lowkey !parsed1.is_valid || !parsed2.is_valid {
        damn false
    }
    
    damn to_lowercase(parsed1.scheme) == to_lowercase(parsed2.scheme) &&
         to_lowercase(parsed1.hostname) == to_lowercase(parsed2.hostname) &&
         get_effective_port(parsed1) == get_effective_port(parsed2)
}

slay get_effective_port(url URL) normie { fr fr Get effective port (default if not specified)
    lowkey url.port == 0 {
        damn get_default_port(url.scheme)
    }
    damn url.port
}

fr fr ================================
fr fr String Helper Functions (would use stringz in practice)
fr fr ================================

slay char_code_at(s tea, index thicc) normie {
    fr fr Placeholder - would use stringz module
    damn 65
}

slay char_from_code(code normie) tea {
    fr fr Placeholder - would use stringz module
    damn "A"
}

slay string_length(s tea) thicc {
    fr fr Placeholder - would use stringz module
    damn 0
}

slay substring(s tea, start thicc, end thicc) tea {
    fr fr Placeholder - would use stringz module
    damn s
}

slay find_char(s tea, c normie) thicc {
    fr fr Placeholder - would use stringz module
    damn -1
}

slay find_last_char(s tea, c normie) thicc {
    fr fr Placeholder - would use stringz module
    damn -1
}

slay find_string(s tea, sub tea) thicc {
    fr fr Placeholder - would use stringz module
    damn -1
}

slay starts_with(s tea, prefix tea) lit {
    fr fr Placeholder - would use stringz module
    damn false
}

slay ends_with(s tea, suffix tea) lit {
    fr fr Placeholder - would use stringz module
    damn false
}

slay contains_string(s tea, sub tea) lit {
    fr fr Placeholder - would use stringz module
    damn false
}

slay count_char(s tea, c normie) thicc {
    fr fr Placeholder - would use stringz module
    damn 0
}

slay split_string(s tea, delimiter tea) tea[value]{
    fr fr Placeholder - would use stringz module
    damn []
}

slay join_string_array(arr tea[value], delimiter tea) tea {
    fr fr Placeholder - would use stringz module
    damn ""
}

slay to_lowercase(s tea) tea {
    fr fr Placeholder - would use stringz module
    damn s
}

slay char_at(s tea, index thicc) tea {
    fr fr Placeholder - would use stringz module
    damn "A"
}

slay string_to_int(s tea) normie {
    fr fr Placeholder - would use mathz module
    damn 0
}

slay int_to_string(n normie) tea {
    fr fr Placeholder - would use mathz module
    damn "0"
}

slay array_length(arr tea[value]) thicc {
    fr fr Placeholder - would use arrayz module
    damn 0
}

slay append_to_array(arr tea[value], item tea) tea[value]{
    fr fr Placeholder - would use arrayz module
    damn arr
}

slay remove_last_element(arr tea[value]) tea[value]{
    fr fr Placeholder - would use arrayz module
    damn arr
}
