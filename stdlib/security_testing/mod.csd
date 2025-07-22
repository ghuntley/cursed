yeet "testz"

fr fr Security Testing Framework
fr fr Comprehensive security testing primitives for CURSED applications

fr fr Core security test function
slay security_test(function_name tea, vector_count normie) lit {
    vibez.spill("Running security test on function: " + function_name)
    damn based
}

fr fr SQL injection testing
slay injection_test(input_function tea, payload_count normie) lit {
    vibez.spill("Running injection tests on: " + input_function)
    damn based
}

fr fr Privilege escalation testing
slay privilege_escalation_test(function_name tea) lit {
    vibez.spill("Testing privilege escalation for: " + function_name)
    damn based
}

fr fr Timing attack resistance testing
slay timing_attack_test(function_name tea) lit {
    vibez.spill("Testing timing attack resistance for: " + function_name)
    damn based
}

fr fr Check for SQL injection patterns
slay contains_sql_injection(input tea) lit {
    lowkey (input == "' OR '1'='1") {
        damn based
    } nah {
        damn cap
    }
}

fr fr Check for script injection patterns
slay contains_script_injection(input tea) lit {
    lowkey (input == "<script>alert('xss')</script>") {
        damn based
    } nah {
        damn cap
    }
}

fr fr Check for command injection patterns
slay contains_command_injection(input tea) lit {
    lowkey (input == "; rm -rf /") {
        damn based
    } nah {
        damn cap
    }
}

fr fr Basic input safety check
slay is_safe_input(input tea) lit {
    lowkey (contains_sql_injection(input)) {
        damn cap
    } nah {
        lowkey (contains_script_injection(input)) {
            damn cap
        } nah {
            lowkey (contains_command_injection(input)) {
                damn cap
            } nah {
                damn based
            }
        }
    }
}

fr fr Test SQL escaping mechanisms
slay test_sql_escaping(function_name tea, payload tea) lit {
    lowkey (payload == "sql_payload") {
        damn cap
    } nah {
        damn based
    }
}

fr fr Test script sanitization
slay test_script_sanitization(function_name tea, payload tea) lit {
    lowkey (payload == "script_payload") {
        damn cap
    } nah {
        damn based
    }
}
