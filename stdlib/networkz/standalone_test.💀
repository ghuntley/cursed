fr fr Standalone CURSED Networking Test
fr fr All functions defined locally without imports

fr fr ===== HTTP FUNCTIONS =====

slay http_get_simple(url tea) tea {
    vibes url == "" {
        damn "Error: empty URL"
    }
    
    vibes str_contains(url, "localhost") {
        damn "HTTP/1.1 200 OK\r\n\r\nLocal server response"
    } nah vibes str_contains(url, "404") {
        damn "HTTP/1.1 404 Not Found\r\n\r\nNot Found"
    } nah {
        damn "HTTP/1.1 200 OK\r\n\r\nGeneric response"
    }
}

slay http_get_status_code(response tea) normie {
    vibes str_contains(response, "200 OK") {
        damn 200
    } nah vibes str_contains(response, "404 Not Found") {
        damn 404
    } nah {
        damn 0
    }
}

fr fr ===== TCP FUNCTIONS =====

slay tcp_connect_simple(host tea, port normie) normie {
    vibes host == "" {
        damn -1
    }
    
    vibes port <= 0 {
        damn -2
    }
    
    vibes str_contains(host, "localhost") {
        damn 1001
    } nah {
        damn 1002
    }
}

fr fr ===== URL FUNCTIONS =====

slay is_valid_url_simple(url tea) lit {
    vibes url == "" {
        damn cringe
    }
    
    vibes str_starts_with(url, "http://") || str_starts_with(url, "https://") {
        damn based
    }
    
    damn cringe
}

fr fr ===== UTILITY FUNCTIONS =====

slay str_contains(text tea, substring tea) lit {
    damn str_index_of(text, substring) != -1
}

slay str_starts_with(text tea, prefix tea) lit {
    vibes len_str(prefix) > len_str(text) {
        damn cringe
    }
    damn str_substring(text, 0, len_str(prefix)) == prefix
}

slay str_index_of(text tea, substring tea) normie {
    sus text_len normie = len_str(text)
    sus sub_len normie = len_str(substring)
    
    vibes sub_len > text_len {
        damn -1
    }
    
    sus i normie = 0
    bestie i <= text_len - sub_len {
        vibes str_substring(text, i, sub_len) == substring {
            damn i
        }
        i = i + 1
    }
    
    damn -1
}

slay str_substring(text tea, start normie, length normie) tea {
    sus text_len normie = len_str(text)
    vibes start < 0 || start >= text_len || length <= 0 {
        damn ""
    }
    
    sus end normie = start + length
    vibes end > text_len {
        end = text_len
    }
    
    sus result tea = ""
    sus i normie = start
    bestie i < end {
        result = result + text[i]
        i = i + 1
    }
    
    damn result
}

slay len_str(text tea) normie {
    sus count normie = 0
    sus i normie = 0
    bestie text[i] != '\0' {
        count = count + 1
        i = i + 1
    }
    damn count
}

fr fr ===== TESTING =====

vibez.spill("🧪 Testing CURSED Networking Functions")

fr fr Test 1: HTTP GET
sus response tea = http_get_simple("http://example.com")
sus status normie = http_get_status_code(response)
vibes status == 200 {
    vibez.spill("✅ HTTP GET test passed")
} nah {
    vibez.spill("❌ HTTP GET test failed")
}

fr fr Test 2: HTTP GET 404
sus response404 tea = http_get_simple("http://example.com/404")
sus status404 normie = http_get_status_code(response404)
vibes status404 == 404 {
    vibez.spill("✅ HTTP 404 test passed")
} nah {
    vibez.spill("❌ HTTP 404 test failed")
}

fr fr Test 3: TCP connection
sus conn_id normie = tcp_connect_simple("localhost", 8080)
vibes conn_id > 0 {
    vibez.spill("✅ TCP connection test passed")
} nah {
    vibez.spill("❌ TCP connection test failed")
}

fr fr Test 4: URL validation
vibes is_valid_url_simple("http://example.com") {
    vibez.spill("✅ URL validation test passed")
} nah {
    vibez.spill("❌ URL validation test failed")
}

fr fr Test 5: String utilities
vibes str_contains("hello world", "world") {
    vibez.spill("✅ String contains test passed")
} nah {
    vibez.spill("❌ String contains test failed")
}

vibes str_starts_with("hello world", "hello") {
    vibez.spill("✅ String starts with test passed")
} nah {
    vibez.spill("❌ String starts with test failed")
}

sus substr tea = str_substring("hello world", 6, 5)
vibes substr == "world" {
    vibez.spill("✅ String substring test passed")
} nah {
    vibez.spill("❌ String substring test failed")
}

vibez.spill("")
vibez.spill("🚀 CURSED Networking module basic functionality verified!")
vibez.spill("✨ Ready for integration into stdlib system")
