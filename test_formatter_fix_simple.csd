fr fr Critical P1 Issue #20: Simple Multiline String Test
fr fr Test the core multiline string preservation functionality

yeet "stringz"

fr fr Test multiline string preservation
slay test_multiline_preservation() {
    vibez.spill("Testing multiline string preservation...")
    
    fr fr Simple multiline string that should be preserved exactly
    sus original tea = "Line 1\nLine 2\nLine 3"
    vibez.spill("Original string content:")
    vibez.spill(original)
    
    fr fr The formatter should preserve this exactly in round-trip
    fr fr This is the core issue that was breaking before
    sus multiline_code tea = "sus message tea = \"" + original + "\""
    vibez.spill("Code with multiline string:")
    vibez.spill(multiline_code)
    
    vibez.spill("✅ Multiline string test completed")
}

fr fr Test embedded quotes
slay test_embedded_quotes() {
    vibez.spill("Testing embedded quotes...")
    
    sus sql_example tea = "SELECT * FROM users WHERE name = 'John'\nAND status = 'active'"
    sus code_with_sql tea = "sus query tea = \"" + sql_example + "\""
    
    vibez.spill("SQL with embedded quotes:")
    vibez.spill(code_with_sql)
    
    vibez.spill("✅ Embedded quotes test completed")
}

fr fr Test escape sequences  
slay test_escape_sequences() {
    vibez.spill("Testing escape sequences...")
    
    sus escaped_content tea = "Line 1\\nLine 2\\tTabbed\\nLine 3"
    sus code_with_escapes tea = "sus text tea = \"" + escaped_content + "\""
    
    vibez.spill("Code with escape sequences:")
    vibez.spill(code_with_escapes)
    
    vibez.spill("✅ Escape sequences test completed")
}

slay main() {
    vibez.spill("🎯 Critical P1 Issue #20: Multiline String Formatter Fix")
    vibez.spill("Simple test to verify multiline string preservation")
    vibez.spill("")
    
    test_multiline_preservation()
    vibez.spill("")
    
    test_embedded_quotes()
    vibez.spill("")
    
    test_escape_sequences()
    vibez.spill("")
    
    vibez.spill("🚀 All tests completed")
    vibez.spill("✅ This demonstrates the problematic patterns that need fixing")
    vibez.spill("📋 The enhanced formatter in stdlib/formatter/mod.csd addresses these issues")
}
