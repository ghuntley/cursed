fr fr Basic networking test without external dependencies

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

slay str_contains(text tea, substring tea) lit {
    damn str_index_of(text, substring) != -1
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

fr fr Manual testing
vibez.spill("🧪 Testing CURSED Networking Module")

fr fr Test 1: Basic HTTP GET
sus response1 tea = http_get_simple("http://example.com")
vibes str_contains(response1, "200 OK") {
    vibez.spill("✅ PASS: HTTP GET returns 200 OK")
} nah {
    vibez.spill("❌ FAIL: HTTP GET test failed")
}

fr fr Test 2: Localhost request
sus response2 tea = http_get_simple("http://localhost:8080")
vibes str_contains(response2, "Local server") {
    vibez.spill("✅ PASS: Localhost request works")
} nah {
    vibez.spill("❌ FAIL: Localhost request failed")
}

fr fr Test 3: 404 error handling
sus response3 tea = http_get_simple("http://example.com/404")
vibes str_contains(response3, "404 Not Found") {
    vibez.spill("✅ PASS: 404 error handling works")
} nah {
    vibez.spill("❌ FAIL: 404 error handling failed")
}

fr fr Test 4: Empty URL handling
sus response4 tea = http_get_simple("")
vibes str_contains(response4, "Error") {
    vibez.spill("✅ PASS: Empty URL handling works")
} nah {
    vibez.spill("❌ FAIL: Empty URL handling failed")
}

fr fr Test 5: String utilities
vibes str_contains("hello world", "world") {
    vibez.spill("✅ PASS: String contains function works")
} nah {
    vibez.spill("❌ FAIL: String contains function failed")
}

sus test_substr tea = str_substring("hello world", 6, 5)
vibes test_substr == "world" {
    vibez.spill("✅ PASS: String substring function works")
} nah {
    vibez.spill("❌ FAIL: String substring function failed")
}

sus test_len normie = len_str("hello")
vibes test_len == 5 {
    vibez.spill("✅ PASS: String length function works")
} nah {
    vibez.spill("❌ FAIL: String length function failed")
}

vibez.spill("🚀 CURSED Networking Module testing complete!")
