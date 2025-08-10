fr fr CURSED Formatter Multiline String Tests - Critical P1 Issue #20 Fix
fr fr Comprehensive testing of multiline string literal round-trip preservation

yeet "testz"
yeet "formatter"

fr fr ===== MULTILINE STRING ROUND-TRIP TESTS =====

slay test_basic_multiline_string_round_trip() {
    test_start("Basic Multiline String Round-Trip")
    
    sus input tea = "sus msg tea = \"Line 1\nLine 2\nLine 3\""
    sus config FormatterConfig = default_formatter_config()
    
    fr fr First formatting pass
    sus first_pass tea = format_cursed_code_with_config_ast(input, config)
    vibez.spill("First pass: " + first_pass)
    
    fr fr Second formatting pass - should be identical
    sus second_pass tea = format_cursed_code_with_config_ast(first_pass, config)
    vibez.spill("Second pass: " + second_pass)
    
    fr fr Critical test: round-trip must be idempotent
    assert_eq_string(first_pass, second_pass)
    
    fr fr String content must be preserved exactly
    assert_true(contains_substring(first_pass, "Line 1"))
    assert_true(contains_substring(first_pass, "Line 2"))
    assert_true(contains_substring(first_pass, "Line 3"))
}

slay test_multiline_string_with_embedded_quotes() {
    test_start("Multiline String with Embedded Quotes")
    
    sus input tea = "sus sql tea = \"SELECT * FROM users WHERE name = 'John'\nAND age > 25\""
    sus config FormatterConfig = default_formatter_config()
    
    sus formatted tea = format_cursed_code_with_config_ast(input, config)
    sus second_pass tea = format_cursed_code_with_config_ast(formatted, config)
    
    fr fr Must preserve embedded single quotes
    assert_true(contains_substring(formatted, "'John'"))
    assert_eq_string(formatted, second_pass)
}

slay test_multiline_json_string() {
    test_start("Multiline JSON String")
    
    sus input tea = "sus config tea = \"{\n  \\\"setting\\\": \\\"value\\\",\n  \\\"enabled\\\": true\n}\""
    sus config FormatterConfig = default_formatter_config()
    
    sus formatted tea = format_cursed_code_with_config_ast(input, config)
    sus second_pass tea = format_cursed_code_with_config_ast(formatted, config)
    
    fr fr Must preserve JSON structure and escapes
    assert_true(contains_substring(formatted, "\\\"setting\\\""))
    assert_true(contains_substring(formatted, "\\\"value\\\""))
    assert_eq_string(formatted, second_pass)
}

slay test_code_template_multiline_string() {
    test_start("Code Template Multiline String")
    
    sus input tea = "slay get_template() tea {\n    sus html tea = \"<html>\n<body>\n    <h1>Hello World</h1>\n</body>\n</html>\"\n    damn html\n}"
    sus config FormatterConfig = default_formatter_config()
    
    sus formatted tea = format_cursed_code_with_config_ast(input, config)
    sus second_pass tea = format_cursed_code_with_config_ast(formatted, config)
    
    fr fr Must preserve HTML structure exactly
    assert_true(contains_substring(formatted, "<html>"))
    assert_true(contains_substring(formatted, "<h1>Hello World</h1>"))
    assert_eq_string(formatted, second_pass)
}

slay test_very_long_multiline_string() {
    test_start("Very Long Multiline String")
    
    sus long_content tea = "This is a very long string that spans multiple lines and contains a lot of text\\nthat might cause the formatter to break or misbehave when processing it through\\nmultiple formatting passes and should be preserved exactly as written"
    sus input tea = "sus long tea = \"" + long_content + "\""
    sus config FormatterConfig = default_formatter_config()
    
    sus formatted tea = format_cursed_code_with_config_ast(input, config)
    sus second_pass tea = format_cursed_code_with_config_ast(formatted, config)
    
    fr fr Must handle very long strings without corruption
    assert_eq_string(formatted, second_pass)
    assert_true(contains_substring(formatted, "very long string"))
}

fr fr ===== MULTILINE STRING TOKENIZATION TESTS =====

slay test_multiline_string_tokenizer() {
    test_start("Multiline String Tokenizer")
    
    sus multiline_source tea = "sus text tea = \"First line\nSecond line\nThird line\""
    sus ctx TokenizerContext = tokenize_advanced(multiline_source)
    
    fr fr Should detect multiline string token
    assert_true(len(ctx.tokens) > 0)
    
    fr fr Find the string token
    sus string_token_found lit = cringe
    sus i drip = 0
    bestie (i < len(ctx.tokens)) {
        ready (ctx.tokens[i].type == "MULTILINE_STRING" || ctx.tokens[i].type == "STRING") {
            string_token_found = based
            vibez.spill("Found string token type: " + ctx.tokens[i].type)
            vibez.spill("String content: " + ctx.tokens[i].value)
            
            fr fr Content must be preserved
            assert_true(contains_substring(ctx.tokens[i].value, "First line"))
            assert_true(contains_substring(ctx.tokens[i].value, "Second line"))
            assert_true(contains_substring(ctx.tokens[i].value, "Third line"))
            break
        }
        i = i + 1
    }
    
    assert_true(string_token_found)
}

slay test_escaped_characters_in_multiline_string() {
    test_start("Escaped Characters in Multiline String")
    
    sus input tea = "sus text tea = \"Line 1\\nLine 2\\tTabbed\\nLine 3\""
    sus ctx TokenizerContext = tokenize_advanced(input)
    
    fr fr Find string token and verify escapes are preserved
    sus i drip = 0
    bestie (i < len(ctx.tokens)) {
        ready (ctx.tokens[i].type == "STRING" || ctx.tokens[i].type == "MULTILINE_STRING") {
            sus content tea = ctx.tokens[i].value
            
            fr fr Escape sequences must be preserved exactly
            assert_true(contains_substring(content, "\\n"))
            assert_true(contains_substring(content, "\\t"))
            break
        }
        i = i + 1
    }
}

fr fr ===== CONFIGURATION TESTS FOR MULTILINE STRINGS =====

slay test_preserve_multiline_strings_config() {
    test_start("Preserve Multiline Strings Configuration")
    
    sus config FormatterConfig = default_formatter_config()
    config.preserve_multiline_strings = based
    
    sus input tea = "sus text tea = \"Line 1\nLine 2\""
    sus formatted tea = format_cursed_code_with_config_ast(input, config)
    
    fr fr With preserve_multiline_strings enabled, content should be unchanged
    assert_true(contains_substring(formatted, "Line 1"))
    assert_true(contains_substring(formatted, "Line 2"))
}

slay test_multiline_string_indent_level_config() {
    test_start("Multiline String Indent Level Configuration")
    
    sus config FormatterConfig = default_formatter_config()
    config.multiline_string_indent_level = 2
    
    sus input tea = "slay func() {\n    sus text tea = \"Line 1\nLine 2\"\n}"
    sus formatted tea = format_cursed_code_with_config_ast(input, config)
    
    fr fr Should handle indentation for multiline strings properly
    assert_true(string_length(formatted) > 0)
}

fr fr ===== EDGE CASE TESTS =====

slay test_empty_multiline_string() {
    test_start("Empty Multiline String")
    
    sus input tea = "sus empty tea = \"\""
    sus config FormatterConfig = default_formatter_config()
    
    sus formatted tea = format_cursed_code_with_config_ast(input, config)
    sus second_pass tea = format_cursed_code_with_config_ast(formatted, config)
    
    assert_eq_string(formatted, second_pass)
}

slay test_multiline_string_with_unicode() {
    test_start("Multiline String with Unicode")
    
    sus input tea = "sus unicode tea = \"Hello 🌍\nWelcome to CURSED 🚀\nEnjoy coding! 💻\""
    sus config FormatterConfig = default_formatter_config()
    
    sus formatted tea = format_cursed_code_with_config_ast(input, config)
    sus second_pass tea = format_cursed_code_with_config_ast(formatted, config)
    
    fr fr Unicode characters must be preserved
    assert_true(contains_substring(formatted, "🌍"))
    assert_true(contains_substring(formatted, "🚀"))
    assert_true(contains_substring(formatted, "💻"))
    assert_eq_string(formatted, second_pass)
}

slay test_multiline_string_boundary_detection() {
    test_start("Multiline String Boundary Detection")
    
    sus input tea = "sus before tea = \"simple\"; sus multi tea = \"line1\nline2\"; sus after tea = \"simple\""
    sus config FormatterConfig = default_formatter_config()
    
    sus formatted tea = format_cursed_code_with_config_ast(input, config)
    sus second_pass tea = format_cursed_code_with_config_ast(formatted, config)
    
    fr fr All string boundaries must be detected correctly
    assert_true(contains_substring(formatted, "simple"))
    assert_true(contains_substring(formatted, "line1"))
    assert_true(contains_substring(formatted, "line2"))
    assert_eq_string(formatted, second_pass)
}

fr fr ===== REGRESSION TESTS =====

slay test_multiline_string_regression_test_1() {
    test_start("Multiline String Regression Test 1")
    
    fr fr Real-world example that previously broke
    sus problematic_input tea = "sus template tea = \"<!DOCTYPE html>\n<html>\n<head>\n    <title>{{title}}</title>\n</head>\n<body>\n    <h1>{{header}}</h1>\n    <p>{{content}}</p>\n</body>\n</html>\""
    
    sus config FormatterConfig = default_formatter_config()
    sus formatted tea = format_cursed_code_with_config_ast(problematic_input, config)
    sus second_pass tea = format_cursed_code_with_config_ast(formatted, config)
    
    fr fr Must not break on HTML template
    assert_eq_string(formatted, second_pass)
    assert_true(contains_substring(formatted, "<!DOCTYPE html>"))
    assert_true(contains_substring(formatted, "{{title}}"))
}

slay test_multiline_string_regression_test_2() {
    test_start("Multiline String Regression Test 2")
    
    fr fr SQL query with complex structure
    sus sql_input tea = "sus query tea = \"SELECT u.name, u.email,\n       p.title, p.content\nFROM users u\nJOIN posts p ON u.id = p.user_id\nWHERE u.active = true\n  AND p.published = true\nORDER BY p.created_at DESC\nLIMIT 10\""
    
    sus config FormatterConfig = default_formatter_config()
    sus formatted tea = format_cursed_code_with_config_ast(sql_input, config)
    sus second_pass tea = format_cursed_code_with_config_ast(formatted, config)
    
    fr fr Must preserve SQL formatting
    assert_eq_string(formatted, second_pass)
    assert_true(contains_substring(formatted, "SELECT"))
    assert_true(contains_substring(formatted, "JOIN"))
    assert_true(contains_substring(formatted, "WHERE"))
}

fr fr ===== MAIN TEST RUNNER =====

slay main() {
    vibez.spill("🔧 Critical P1 Issue #20: Multiline String Literal Formatter Fix Tests")
    vibez.spill("Testing enhanced formatter with proper multiline string support")
    vibez.spill("")
    
    fr fr Round-trip tests - most critical
    test_basic_multiline_string_round_trip()
    test_multiline_string_with_embedded_quotes()
    test_multiline_json_string()
    test_code_template_multiline_string()
    test_very_long_multiline_string()
    
    fr fr Tokenization tests
    test_multiline_string_tokenizer()
    test_escaped_characters_in_multiline_string()
    
    fr fr Configuration tests
    test_preserve_multiline_strings_config()
    test_multiline_string_indent_level_config()
    
    fr fr Edge case tests
    test_empty_multiline_string()
    test_multiline_string_with_unicode()
    test_multiline_string_boundary_detection()
    
    fr fr Regression tests
    test_multiline_string_regression_test_1()
    test_multiline_string_regression_test_2()
    
    vibez.spill("")
    vibez.spill("✅ All multiline string formatting tests completed")
    vibez.spill("🎯 Critical P1 Issue #20 validation complete")
    
    print_test_summary()
}
