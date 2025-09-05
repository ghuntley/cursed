yeet "url_parsing"

fr fr Comprehensive test suite for url_parsing module
fr fr Simplified tests without testz framework

vibez.spill("=== URL Parsing Module Test Suite ===")

fr fr Test 1: Basic HTTP URL parsing
vibez.spill("Test 1: Basic HTTP URL parsing")
bestie url_parse("http://example.com") {
    vibez.spill("✓ URL parsed successfully")
    vibez.spill("Scheme: " + url_get_scheme())
    vibez.spill("Host: " + url_get_host())
    vibez.spill("Port: " + url_get_port())
    vibez.spill("Path: " + url_get_path())
    
    bestie url_get_scheme() == "http" {
        vibez.spill("✓ Scheme correct")
    } otherwise {
        vibez.spill("✗ Scheme incorrect")
    }
    
    bestie url_get_host() == "example.com" {
        vibez.spill("✓ Host correct")
    } otherwise {
        vibez.spill("✗ Host incorrect")
    }
    
    bestie url_get_port() == 80 {
        vibez.spill("✓ Port correct")
    } otherwise {
        vibez.spill("✗ Port incorrect")
    }
    
    bestie url_is_valid() {
        vibez.spill("✓ URL is valid")
    } otherwise {
        vibez.spill("✗ URL is not valid")
    }
    
    bestie url_is_absolute() {
        vibez.spill("✓ URL is absolute")
    } otherwise {
        vibez.spill("✗ URL is not absolute")
    }
    
    bestie !url_is_secure() {
        vibez.spill("✓ HTTP is not secure")
    } otherwise {
        vibez.spill("✗ HTTP should not be secure")
    }
} otherwise {
    vibez.spill("✗ Failed to parse basic HTTP URL")
}

vibez.spill("")

fr fr Test 2: Basic HTTPS URL parsing
vibez.spill("Test 2: Basic HTTPS URL parsing")
bestie url_parse("https://example.com") {
    vibez.spill("✓ HTTPS URL parsed successfully")
    
    bestie url_get_scheme() == "https" {
        vibez.spill("✓ HTTPS scheme correct")
    } otherwise {
        vibez.spill("✗ HTTPS scheme incorrect")
    }
    
    bestie url_get_port() == 443 {
        vibez.spill("✓ HTTPS port correct")
    } otherwise {
        vibez.spill("✗ HTTPS port incorrect")
    }
    
    bestie url_is_secure() {
        vibez.spill("✓ HTTPS is secure")
    } otherwise {
        vibez.spill("✗ HTTPS should be secure")
    }
} otherwise {
    vibez.spill("✗ Failed to parse HTTPS URL")
}

vibez.spill("")

fr fr Test 3: Component setters
vibez.spill("Test 3: Component setters")
bestie url_parse("http://example.com") {
    bestie url_set_scheme("https") {
        vibez.spill("✓ Scheme setter works")
        
        bestie url_get_scheme() == "https" {
            vibez.spill("✓ Scheme updated correctly")
        } otherwise {
            vibez.spill("✗ Scheme not updated")
        }
    } otherwise {
        vibez.spill("✗ Scheme setter failed")
    }
    
    bestie url_set_host("newhost.com") {
        vibez.spill("✓ Host setter works")
        
        bestie url_get_host() == "newhost.com" {
            vibez.spill("✓ Host updated correctly")
        } otherwise {
            vibez.spill("✗ Host not updated")
        }
    } otherwise {
        vibez.spill("✗ Host setter failed")
    }
    
    bestie url_set_port(9000) {
        vibez.spill("✓ Port setter works")
        
        bestie url_get_port() == 9000 {
            vibez.spill("✓ Port updated correctly")
        } otherwise {
            vibez.spill("✗ Port not updated")
        }
    } otherwise {
        vibez.spill("✗ Port setter failed")
    }
} otherwise {
    vibez.spill("✗ Failed to parse URL for component setter test")
}

vibez.spill("")

fr fr Test 4: URL building
vibez.spill("Test 4: URL building")
bestie url_parse("http://example.com") {
    bestie url_set_scheme("https") {
        bestie url_set_host("api.test.com") {
            bestie url_set_port(8080) {
                sus built_url tea = url_build()
                vibez.spill("Built URL: " + built_url)
                
                bestie built_url != "" {
                    vibez.spill("✓ URL building works")
                } otherwise {
                    vibez.spill("✗ URL building failed")
                }
            }
        }
    }
} otherwise {
    vibez.spill("✗ Failed to parse URL for building test")
}

vibez.spill("")

fr fr Test 5: Query parameters
vibez.spill("Test 5: Query parameters")
bestie url_parse("https://example.com") {
    bestie url_add_query_param("name", "test") {
        vibez.spill("✓ Added query parameter")
        
        bestie url_add_query_param("value", "123") {
            vibez.spill("✓ Added second query parameter")
            
            bestie url_has_query_param("name") {
                vibez.spill("✓ Has query parameter")
                
                sus name_value tea = url_get_query_param("name")
                bestie name_value == "test" {
                    vibez.spill("✓ Query parameter value correct")
                } otherwise {
                    vibez.spill("✗ Query parameter value incorrect: " + name_value)
                }
            } otherwise {
                vibez.spill("✗ Query parameter not found")
            }
        }
    }
} otherwise {
    vibez.spill("✗ Failed to parse URL for query parameter test")
}

vibez.spill("")

fr fr Test 6: Localhost detection
vibez.spill("Test 6: Localhost detection")
bestie url_parse("http://localhost:3000") {
    bestie url_is_localhost() {
        vibez.spill("✓ Localhost detected correctly")
    } otherwise {
        vibez.spill("✗ Localhost not detected")
    }
} otherwise {
    vibez.spill("✗ Failed to parse localhost URL")
}

bestie url_parse("http://127.0.0.1:8080") {
    bestie url_is_localhost() {
        vibez.spill("✓ 127.0.0.1 detected as localhost")
    } otherwise {
        vibez.spill("✗ 127.0.0.1 not detected as localhost")
    }
} otherwise {
    vibez.spill("✗ Failed to parse 127.0.0.1 URL")
}

vibez.spill("")

fr fr Test 7: URL encoding/decoding
vibez.spill("Test 7: URL encoding/decoding")
sus original tea = "hello world & test=value"
sus encoded tea = url_encode(original)
vibez.spill("Original: " + original)
vibez.spill("Encoded: " + encoded)

sus decoded tea = url_decode(encoded)
vibez.spill("Decoded: " + decoded)

bestie decoded == original {
    vibez.spill("✓ Encoding/decoding works")
} otherwise {
    vibez.spill("✗ Encoding/decoding failed")
}

vibez.spill("")

fr fr Test 8: URL normalization
vibez.spill("Test 8: URL normalization")
bestie url_parse("http://example.com") {
    bestie url_normalize() {
        vibez.spill("✓ URL normalization works")
    } otherwise {
        vibez.spill("✗ URL normalization failed")
    }
} otherwise {
    vibez.spill("✗ Failed to parse URL for normalization test")
}

vibez.spill("")

fr fr Test 9: URL comparison
vibez.spill("Test 9: URL comparison")
bestie url_parse("https://example.com/path") {
    bestie url_equals("https://example.com/path") {
        vibez.spill("✓ URL equality works")
    } otherwise {
        vibez.spill("✗ URL equality failed")
    }
} otherwise {
    vibez.spill("✗ Failed to parse URL for comparison test")
}

vibez.spill("")

fr fr Test 10: URL resolution
vibez.spill("Test 10: URL resolution")
sus base tea = "https://example.com/api/v1/"
sus relative tea = "users/123"
sus resolved tea = url_resolve(base, relative)
vibez.spill("Base: " + base)
vibez.spill("Relative: " + relative)
vibez.spill("Resolved: " + resolved)

bestie resolved != "" {
    vibez.spill("✓ URL resolution works")
} otherwise {
    vibez.spill("✗ URL resolution failed")
}

vibez.spill("")

fr fr Test 11: URL joining
vibez.spill("Test 11: URL joining")
sus base_url tea = "https://example.com/api"
sus path tea = "v1/users"
sus joined tea = url_join(base_url, path)
vibez.spill("Base URL: " + base_url)
vibez.spill("Path: " + path)
vibez.spill("Joined: " + joined)

bestie joined != "" {
    vibez.spill("✓ URL joining works")
} otherwise {
    vibez.spill("✗ URL joining failed")
}

vibez.spill("")

fr fr Test 12: Relative URL parsing
vibez.spill("Test 12: Relative URL parsing")
bestie url_parse("/relative/path") {
    vibez.spill("✓ Relative URL parsed")
    
    bestie url_is_relative() {
        vibez.spill("✓ Relative URL detected correctly")
    } otherwise {
        vibez.spill("✗ Relative URL not detected")
    }
} otherwise {
    vibez.spill("✗ Failed to parse relative URL")
}

vibez.spill("")

fr fr Test 13: Clear functionality
vibez.spill("Test 13: Clear functionality")
bestie url_parse("https://example.com") {
    bestie url_is_parsed() {
        vibez.spill("✓ URL is parsed")
        
        bestie url_clear() {
            bestie !url_is_parsed() {
                vibez.spill("✓ URL cleared successfully")
                
                bestie url_get_raw() == "" {
                    vibez.spill("✓ Raw URL cleared")
                } otherwise {
                    vibez.spill("✗ Raw URL not cleared")
                }
            } otherwise {
                vibez.spill("✗ URL not cleared")
            }
        } otherwise {
            vibez.spill("✗ Clear function failed")
        }
    } otherwise {
        vibez.spill("✗ URL not parsed initially")
    }
} otherwise {
    vibez.spill("✗ Failed to parse URL for clear test")
}

vibez.spill("")

fr fr Test 14: Invalid port handling
vibez.spill("Test 14: Invalid port handling")
bestie url_parse("https://example.com") {
    bestie !url_set_port(-1) {
        vibez.spill("✓ Invalid negative port rejected")
    } otherwise {
        vibez.spill("✗ Invalid negative port accepted")
    }
    
    bestie !url_set_port(65536) {
        vibez.spill("✓ Invalid high port rejected")
    } otherwise {
        vibez.spill("✗ Invalid high port accepted")
    }
    
    bestie url_set_port(8080) {
        vibez.spill("✓ Valid port accepted")
    } otherwise {
        vibez.spill("✗ Valid port rejected")
    }
} otherwise {
    vibez.spill("✗ Failed to parse URL for port validation test")
}

vibez.spill("")

fr fr Test 15: Utility functions
vibez.spill("Test 15: Utility functions")
bestie url_parse("https://example.com") {
    sus base_url tea = url_get_base_url()
    vibez.spill("Base URL: " + base_url)
    
    sus domain tea = url_get_domain()
    vibez.spill("Domain: " + domain)
    
    sus protocol tea = url_get_protocol()
    vibez.spill("Protocol: " + protocol)
    
    sus authority tea = url_get_authority()
    vibez.spill("Authority: " + authority)
    
    bestie base_url != "" {
        vibez.spill("✓ Utility functions work")
    } otherwise {
        vibez.spill("✗ Utility functions failed")
    }
} otherwise {
    vibez.spill("✗ Failed to parse URL for utility function test")
}

vibez.spill("")
vibez.spill("=== URL Parsing Module Test Suite Complete ===")
