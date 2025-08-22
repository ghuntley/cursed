// Test script for CURSED formatter

yeet "testz"

slay test_basic_formatting() {
    sus unformatted tea = "sus x drip=42;ready(x>0){vibez.spill(x);}"
    sus expected tea = "sus x drip = 42;\nready (x > 0) {\n    vibez.spill(x);\n}\n"
    
    // This test will work once we have proper string functions
    vibez.spill("Testing basic formatting...")
    vibez.spill("Input: " + unformatted)
    vibez.spill("Expected well-formatted output")
}

slay test_gen_z_keywords() {
    sus code tea = "sus name tea=\"test\";slay demo(){damn name;}"
    vibez.spill("Testing Gen Z keywords formatting...")
    vibez.spill("Input: " + code)
    vibez.spill("Should format with proper spacing around keywords")
}

slay test_comment_preservation() {
    sus code tea = "fr fr This is a comment\nsus x drip = 42;"
    vibez.spill("Testing comment preservation...")
    vibez.spill("Input: " + code)
    vibez.spill("Comments should be preserved")
}

slay main() {
    vibez.spill("🧪 CURSED Formatter Test Suite")
    vibez.spill("")
    
    test_basic_formatting()
    vibez.spill("")
    
    test_gen_z_keywords()  
    vibez.spill("")
    
    test_comment_preservation()
    vibez.spill("")
    
    vibez.spill("✅ Test suite complete!")
}
