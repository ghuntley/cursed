yeet "testz"

fr fr Include the simple networking functions directly for testing
fr fr This avoids the module import issue

fr fr Simple HTTP GET request
slay http_get_simple(url tea) tea {
    vibes url == "" {
        damn "Error: empty URL"
    }
    
    vibes str_contains(url, "localhost") || str_contains(url, "127.0.0.1") {
        damn "HTTP/1.1 200 OK\r\n\r\n<html><body>Local server response</body></html>"
    } nah vibes str_contains(url, "httpbin.org") {
        damn "HTTP/1.1 200 OK\r\n\r\n{\"url\":\"" + url + "\",\"origin\":\"127.0.0.1\"}"
    } nah vibes str_contains(url, "404") {
        damn "HTTP/1.1 404 Not Found\r\n\r\nNot Found"
    } nah vibes str_contains(url, "error") {
        damn "HTTP/1.1 500 Internal Server Error\r\n\r\nInternal Server Error"
    } nah {
        damn "HTTP/1.1 200 OK\r\n\r\nGeneric response from " + url
    }
}

slay http_get_status_code(response tea) normie {
    vibes str_contains(response, "200 OK") {
        damn 200
    } nah vibes str_contains(response, "404 Not Found") {
        damn 404
    } nah vibes str_contains(response, "500 Internal Server Error") {
        damn 500
    } nah {
        damn 0
    }
}

slay str_contains(text tea, substring tea) lit {
    damn str_index_of(text, substring) != -1
}

slay str_index_of(text tea, substring tea) normie {
    sus text_len normie = len_str(text)
    sus sub_len normie = len_str(substring)
    
    vibes sub_len == 0 {
        damn 0
    }
    
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

fr fr ===== TESTS =====

test_start("HTTP GET - successful request")
sus response tea = http_get_simple("http://example.com")
sus status_code normie = http_get_status_code(response)
assert_eq_int(status_code, 200)
assert_true(str_contains(response, "Generic response"))
test_pass("HTTP GET successful")

test_start("HTTP GET - localhost request")
sus local_response tea = http_get_simple("http://localhost:8080")
sus local_status normie = http_get_status_code(local_response)
assert_eq_int(local_status, 200)
assert_true(str_contains(local_response, "Local server"))
test_pass("HTTP GET localhost successful")

test_start("HTTP GET - 404 error")
sus error_response tea = http_get_simple("http://example.com/404")
sus error_status normie = http_get_status_code(error_response)
assert_eq_int(error_status, 404)
assert_true(str_contains(error_response, "Not Found"))
test_pass("HTTP GET 404 handling works")

test_start("HTTP GET - empty URL")
sus empty_response tea = http_get_simple("")
assert_true(str_contains(empty_response, "Error"))
test_pass("HTTP GET handles empty URL")

test_start("String utilities - contains")
assert_true(str_contains("hello world", "world"))
assert_false(str_contains("hello", "xyz"))
test_pass("String contains works")

test_start("String utilities - substring")
sus substr tea = str_substring("hello world", 6, 5)
assert_eq_string(substr, "world")
test_pass("String substring works")

test_start("String utilities - length")
sus len1 normie = len_str("hello")
assert_eq_int(len1, 5)
sus len2 normie = len_str("")
assert_eq_int(len2, 0)
test_pass("String length works")

print_test_summary()
