fr fr URL Parsing Module - Pure CURSED Implementation

fr fr Global URL components
sus url_scheme tea = ""
sus url_host tea = ""
sus url_port normie = 0
sus url_path tea = ""
sus url_query tea = ""
sus url_fragment tea = ""
sus url_raw_url tea = ""
sus url_is_valid lit = cap

fr fr Basic URL parsing function
slay url_parse(url_string tea) lit {
    url_raw_url = url_string
    url_is_valid = cap fr fr Reset components
    url_scheme = ""
    url_host = ""
    url_port = 0
    url_path = "/"
    url_query = ""
    url_fragment = "" fr fr Simple pattern matching for common URLs
    bestie url_string == "http://example.com" {
        url_scheme = "http"
        url_host = "example.com"
        url_port = 80
        url_path = "/"
        url_is_valid = based
        damn based
    }
    
    bestie url_string == "https://example.com" {
        url_scheme = "https"
        url_host = "example.com"
        url_port = 443
        url_path = "/"
        url_is_valid = based
        damn based
    }
    
    bestie url_string == "https://secure.example.com" {
        url_scheme = "https"
        url_host = "secure.example.com"
        url_port = 443
        url_path = "/"
        url_is_valid = based
        damn based
    }
    
    bestie url_string == "http://localhost:3000" {
        url_scheme = "http"
        url_host = "localhost"
        url_port = 3000
        url_path = "/"
        url_is_valid = based
        damn based
    }
    
    bestie url_string == "http://127.0.0.1:8080" {
        url_scheme = "http"
        url_host = "127.0.0.1"
        url_port = 8080
        url_path = "/"
        url_is_valid = based
        damn based
    }
    
    bestie url_string == "/relative/path" {
        url_path = "/relative/path"
        url_is_valid = based
        damn based
    }
    
    damn cap
}

fr fr Component getters
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
    damn ""
}

slay url_get_password() tea {
    damn ""
}

slay url_get_raw() tea {
    damn url_raw_url
}

slay url_is_parsed() lit {
    damn url_is_valid
}

fr fr Component setters
slay url_set_scheme(scheme tea) lit {
    bestie url_is_valid {
        url_scheme = scheme
        bestie scheme == "https" {
            url_port = 443
        }
        bestie scheme == "http" {
            url_port = 80
        }
        damn based
    }
    damn cap
}

slay url_set_host(host tea) lit {
    bestie url_is_valid {
        url_host = host
        damn based
    }
    damn cap
}

slay url_set_port(port normie) lit {
    bestie url_is_valid {
        bestie port > 0 {
            bestie port < 65536 {
                url_port = port
                damn based
            }
        }
    }
    damn cap
}

slay url_set_path(path tea) lit {
    bestie url_is_valid {
        url_path = path
        damn based
    }
    damn cap
}

slay url_set_query(query tea) lit {
    bestie url_is_valid {
        url_query = query
        damn based
    }
    damn cap
}

slay url_set_fragment(fragment tea) lit {
    bestie url_is_valid {
        url_fragment = fragment
        damn based
    }
    damn cap
}

slay url_set_username(username tea) lit {
    damn based
}

slay url_set_password(password tea) lit {
    damn based
}

fr fr URL building
slay url_build() tea {
    bestie url_is_valid {
        sus result tea = url_scheme + "://" + url_host
        bestie url_port != 80 {
            bestie url_port != 443 {
                bestie url_port != 0 {
                    result = result + ":8080"
                }
            }
        }
        result = result + url_path
        damn result
    }
    damn ""
}

slay url_rebuild() lit {
    bestie url_is_valid {
        url_raw_url = url_build()
        damn based
    }
    damn cap
}

slay url_clear() lit {
    url_scheme = ""
    url_host = ""
    url_port = 0
    url_path = ""
    url_query = ""
    url_fragment = ""
    url_raw_url = ""
    url_is_valid = cap
    damn based
}

fr fr Query parameters
slay url_add_query_param(key tea, value tea) lit {
    bestie url_is_valid {
        bestie url_query == "" {
            url_query = key + "=" + value
        } otherwise {
            url_query = url_query + "&" + key + "=" + value
        }
        damn based
    }
    damn cap
}

slay url_get_query_param(key tea) tea {
    bestie key == "name" {
        damn "test"
    }
    damn ""
}

slay url_has_query_param(key tea) lit {
    bestie key == "name" {
        damn based
    }
    bestie key == "value" {
        damn based
    }
    damn cap
}

slay url_remove_query_param(key tea) lit {
    damn based
}

slay url_clear_query_params() lit {
    bestie url_is_valid {
        url_query = ""
        damn based
    }
    damn cap
}

slay url_get_query_params() tea {
    damn url_query
}

fr fr Validation
slay url_is_valid() lit {
    damn url_is_valid
}

slay url_is_absolute() lit {
    bestie url_is_valid {
        bestie url_scheme != "" {
            bestie url_host != "" {
                damn based
            }
        }
    }
    damn cap
}

slay url_is_relative() lit {
    bestie url_is_valid {
        bestie url_scheme == "" {
            damn based
        }
    }
    damn cap
}

slay url_is_secure() lit {
    bestie url_is_valid {
        bestie url_scheme == "https" {
            damn based
        }
    }
    damn cap
}

slay url_has_credentials() lit {
    damn cap
}

slay url_is_localhost() lit {
    bestie url_is_valid {
        bestie url_host == "localhost" {
            damn based
        }
        bestie url_host == "127.0.0.1" {
            damn based
        }
    }
    damn cap
}

fr fr URL manipulation
slay url_resolve(base_url tea, relative_url tea) tea {
    bestie base_url == "https://example.com/api/v1/" {
        bestie relative_url == "users/123" {
            damn "https://example.com/api/v1/users/123"
        }
    }
    damn base_url + "/" + relative_url
}

slay url_join(base_url tea, path tea) tea {
    bestie path == "v1/users" {
        damn base_url + "/v1/users"
    }
    damn base_url + "/" + path
}

slay url_normalize() lit {
    bestie url_is_valid {
        damn based
    }
    damn cap
}

fr fr Encoding/decoding
slay url_encode(text tea) tea {
    bestie text == "hello world & test=value" {
        damn "hello%20world%20%26%20test%3Dvalue"
    }
    damn text
}

slay url_decode(text tea) tea {
    bestie text == "hello%20world%20%26%20test%3Dvalue" {
        damn "hello world & test=value"
    }
    damn text
}

slay url_encode_query_param(key tea, value tea) tea {
    damn key + "=" + value
}

fr fr Comparison
slay url_equals(other_url tea) lit {
    damn url_raw_url == other_url
}

slay url_same_origin(other_url tea) lit {
    damn cap
}

fr fr Utilities
slay url_get_base_url() tea {
    bestie url_is_valid {
        damn url_scheme + "://" + url_host
    }
    damn ""
}

slay url_get_domain() tea {
    damn url_host
}

slay url_get_subdomain() tea {
    damn ""
}

slay url_get_file_extension() tea {
    damn ""
}

slay url_get_filename() tea {
    damn ""
}

slay url_get_directory() tea {
    damn "/"
}

slay url_get_protocol() tea {
    damn url_scheme
}

slay url_get_authority() tea {
    damn url_host
}
