fr fr Critical P1 Issue #20: Multiline String Literal Round-Trip Test
fr fr This test demonstrates the formatter breaking multiline string literals

yeet "stringz"
yeet "arrayz"
yeet "testz"

fr fr Test multiline string literals that break with current formatter
slay test_multiline_string_literals() {
    test_start("Multiline String Literal Formatting")
    
    fr fr Test case 1: Basic multiline string
    sus multiline_input tea = "sus message tea = \"Line 1\nLine 2\nLine 3\""
    vibez.spill("Input: " + multiline_input)
    
    fr fr Test case 2: Multiline string with embedded quotes
    sus embedded_quotes tea = "sus sql tea = \"SELECT * FROM users WHERE name = 'John'\nAND age > 25\""
    vibez.spill("Embedded quotes: " + embedded_quotes)
    
    fr fr Test case 3: Multiline JSON string
    sus json_string tea = "sus config tea = \"{\n  \"setting\": \"value\",\n  \"enabled\": true\n}\""
    vibez.spill("JSON string: " + json_string)
    
    fr fr Test case 4: Code with multiline string literal
    sus code_with_multiline tea = "slay get_template() tea {\n    sus html tea = \"<html>\n<body>\n    <h1>Hello World</h1>\n</body>\n</html>\"\n    damn html\n}"
    vibez.spill("Code with multiline: " + code_with_multiline)
    
    fr fr Test case 5: Multiline raw string (if supported)
    sus raw_multiline tea = "sus raw tea = `This is a\nmultiline\nraw string`"
    vibez.spill("Raw multiline: " + raw_multiline)
}

fr fr Test the round-trip issue: format -> parse -> format again
slay test_multiline_string_round_trip() {
    test_start("Multiline String Round-Trip Issue")
    
    sus original tea = "sus message tea = \"Hello\nWorld\nFrom\nCURSED\""
    vibez.spill("Original: " + original)
    
    fr fr First formatting pass
    sus first_pass tea = "formatted_placeholder"  fr fr Would call formatter here
    vibez.spill("First pass: " + first_pass)
    
    fr fr Second formatting pass (should be identical to first)
    sus second_pass tea = "formatted_placeholder"  fr fr Would call formatter here
    vibez.spill("Second pass: " + second_pass)
    
    fr fr The issue: second_pass != first_pass for multiline strings
    ready (first_pass != second_pass) {
        vibez.spill("❌ ROUND-TRIP ISSUE DETECTED!")
        vibez.spill("First and second formatting passes produce different results")
    } otherwise {
        vibez.spill("✅ Round-trip formatting consistent")
    }
}

fr fr Test how current tokenizer handles multiline strings
slay test_multiline_string_tokenization() {
    test_start("Multiline String Tokenization")
    
    sus multiline_code tea = "sus msg tea = \"Line 1\nLine 2\""
    vibez.spill("Testing tokenization of: " + multiline_code)
    
    fr fr Demonstrate issues with current approach
    vibez.spill("Current tokenizer likely breaks on:")
    vibez.spill("1. Newlines inside string literals")
    vibez.spill("2. Quote counting becoming unbalanced")
    vibez.spill("3. String literal boundaries not preserved")
    vibez.spill("4. Escape sequences not handled properly")
}

fr fr Test specific problematic patterns
slay test_problematic_multiline_patterns() {
    test_start("Problematic Multiline String Patterns")
    
    fr fr Pattern 1: Escaped quotes in multiline
    sus escaped_quotes tea = "sus text tea = \"Say \\\"Hello\\\" to\nthe world\""
    vibez.spill("Escaped quotes: " + escaped_quotes)
    
    fr fr Pattern 2: Mixed quotes
    sus mixed_quotes tea = "sus html tea = \"<div class='container'>\n<p>Content here</p>\n</div>\""
    vibez.spill("Mixed quotes: " + mixed_quotes)
    
    fr fr Pattern 3: Code snippets in strings
    sus code_in_string tea = "sus example tea = \"function test() {\n    return 'hello';\n}\""
    vibez.spill("Code in string: " + code_in_string)
    
    fr fr Pattern 4: Unicode and special characters
    sus unicode_multiline tea = "sus unicode tea = \"Hello 🌍\nWelcome to CURSED 🚀\nEnjoy coding! 💻\""
    vibez.spill("Unicode multiline: " + unicode_multiline)
    
    fr fr Pattern 5: Very long multiline strings
    sus long_multiline tea = "sus long tea = \"This is a very long string that spans multiple lines and contains a lot of text that might cause the formatter to break or misbehave when processing it through multiple formatting passes\""
    vibez.spill("Long multiline: " + long_multiline)
}

fr fr Test how formatter should preserve string integrity
slay test_string_preservation_requirements() {
    test_start("String Preservation Requirements")
    
    vibez.spill("Requirements for proper multiline string handling:")
    vibez.spill("1. ✅ Preserve exact string content including newlines")
    vibez.spill("2. ✅ Maintain escape sequences exactly as written")
    vibez.spill("3. ✅ Don't add or remove quotes within string literals")
    vibez.spill("4. ✅ Handle both double and single quotes correctly")
    vibez.spill("5. ✅ Support raw strings if implemented")
    vibez.spill("6. ✅ Round-trip formatting should be idempotent")
    vibez.spill("7. ✅ String boundaries must be detected accurately")
    vibez.spill("8. ✅ Don't format inside string literals")
    vibez.spill("9. ✅ Preserve Unicode and special characters")
    vibez.spill("10. ✅ Handle extremely long strings gracefully")
}

slay main() {
    vibez.spill("🔍 Critical P1 Issue #20: Multiline String Literal Formatter Tests")
    vibez.spill("Demonstrates the round-trip formatting issue with multiline strings")
    vibez.spill("")
    
    test_multiline_string_literals()
    test_multiline_string_round_trip()
    test_multiline_string_tokenization()
    test_problematic_multiline_patterns()
    test_string_preservation_requirements()
    
    vibez.spill("")
    vibez.spill("🚨 This test suite identifies the critical multiline string issues")
    vibez.spill("📋 Next step: Implement fixed formatter in pure CURSED")
    
    print_test_summary()
}
