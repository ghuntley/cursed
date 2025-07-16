yeet "testz"
yeet "security_testing"

test_start("Security Testing Framework Tests")

# Test security_test function
slay test_security_test_basic() {
    sus result lit = security_test("test_function", 3)
    assert_true(result)
    vibez.spill("✅ security_test function works")
}

# Test injection_test function
slay test_injection_test_basic() {
    sus result lit = injection_test("user_input_function", 2)
    assert_true(result)
    vibez.spill("✅ injection_test function works")
}

# Test privilege escalation
slay test_privilege_escalation() {
    sus result lit = privilege_escalation_test("admin_function")
    assert_true(result)
    vibez.spill("✅ privilege_escalation_test works")
}

# Test timing attack
slay test_timing_attack() {
    sus result lit = timing_attack_test("auth_function")
    assert_true(result)
    vibez.spill("✅ timing_attack_test works")
}

# Test SQL injection detection
slay test_sql_injection_detection() {
    assert_true(contains_sql_injection("' OR '1'='1"))
    assert_false(contains_sql_injection("normal input"))
    vibez.spill("✅ SQL injection detection works")
}

# Test script injection detection
slay test_script_injection_detection() {
    assert_true(contains_script_injection("<script>alert('xss')</script>"))
    assert_false(contains_script_injection("normal text"))
    vibez.spill("✅ Script injection detection works")
}

# Test command injection detection
slay test_command_injection_detection() {
    assert_true(contains_command_injection("; rm -rf /"))
    assert_false(contains_command_injection("normal command"))
    vibez.spill("✅ Command injection detection works")
}

# Test input safety
slay test_input_safety() {
    assert_false(is_safe_input("' OR '1'='1"))
    assert_true(is_safe_input("safe input"))
    vibez.spill("✅ Input safety validation works")
}

# Test escaping mechanisms
slay test_escaping() {
    sus sql_safe lit = test_sql_escaping("test_function", "normal_input")
    assert_true(sql_safe)
    
    sus script_safe lit = test_script_sanitization("test_function", "normal_input")
    assert_true(script_safe)
    
    vibez.spill("✅ Escaping mechanisms work")
}

# Run all tests
test_security_test_basic()
test_injection_test_basic()
test_privilege_escalation()
test_timing_attack()
test_sql_injection_detection()
test_script_injection_detection()
test_command_injection_detection()
test_input_safety()
test_escaping()

print_test_summary()
