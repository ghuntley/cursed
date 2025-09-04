// Final Working CURSED Stringz Module Test
// Tests basic string operations with correct syntax

yeet "stringz"

slay test_basic_stringz() {
    vibez.spill("Testing basic stringz operations...")
    
    // Test Contains
    if stringz.Contains("hello world", "world") {
        vibez.spill("✓ Contains test passed")
    } else {
        vibez.spill("✗ Contains test failed")
    }
    
    // Test StartsWith
    if stringz.StartsWith("hello world", "hello") {
        vibez.spill("✓ StartsWith test passed")
    } else {
        vibez.spill("✗ StartsWith test failed")
    }
    
    // Test EndsWith
    if stringz.EndsWith("hello world", "world") {
        vibez.spill("✓ EndsWith test passed")
    } else {
        vibez.spill("✗ EndsWith test failed")
    }
    
    // Test ToLower
    sus lower_result tea = stringz.ToLower("HELLO")
    if lower_result == "hello" {
        vibez.spill("✓ ToLower test passed")
    } else {
        vibez.spill("✗ ToLower test failed: " + lower_result)
    }
    
    // Test ToUpper
    sus upper_result tea = stringz.ToUpper("hello")
    if upper_result == "HELLO" {
        vibez.spill("✓ ToUpper test passed")
    } else {
        vibez.spill("✗ ToUpper test failed: " + upper_result)
    }
    
    // Test Len
    sus len_result normie = stringz.Len("hello")
    if len_result == 5 {
        vibez.spill("✓ Len test passed")
    } else {
        vibez.spill("✗ Len test failed: " + tea(len_result))
    }
    
    // Test IsEmpty
    if stringz.IsEmpty("") {
        vibez.spill("✓ IsEmpty test passed")
    } else {
        vibez.spill("✗ IsEmpty test failed")
    }
    
    // Test Equals
    if stringz.Equals("hello", "hello") {
        vibez.spill("✓ Equals test passed")
    } else {
        vibez.spill("✗ Equals test failed")
    }
    
    // Test Repeat
    sus repeat_result tea = stringz.Repeat("ho", 3)
    if repeat_result == "hohoho" {
        vibez.spill("✓ Repeat test passed")
    } else {
        vibez.spill("✗ Repeat test failed: " + repeat_result)
    }
    
    // Test Reverse
    sus reverse_result tea = stringz.Reverse("hello")
    if reverse_result == "olleh" {
        vibez.spill("✓ Reverse test passed")
    } else {
        vibez.spill("✗ Reverse test failed: " + reverse_result)
    }
    
    // Test Substring
    sus substring_result tea = stringz.Substring("hello world", 0, 5)
    if substring_result == "hello" {
        vibez.spill("✓ Substring test passed")
    } else {
        vibez.spill("✗ Substring test failed: " + substring_result)
    }
    
    // Test CharAt
    sus char_result tea = stringz.CharAt("hello", 0)
    if char_result == "h" {
        vibez.spill("✓ CharAt test passed")
    } else {
        vibez.spill("✗ CharAt test failed: " + char_result)
    }
    
    // Test conversion functions
    sus int_result normie = stringz.ToInt("123")
    if int_result == 123 {
        vibez.spill("✓ ToInt test passed")
    } else {
        vibez.spill("✗ ToInt test failed: " + tea(int_result))
    }
    
    sus str_result tea = stringz.FromInt(123)
    if str_result == "123" {
        vibez.spill("✓ FromInt test passed")
    } else {
        vibez.spill("✗ FromInt test failed: " + str_result)
    }
    
    sus bool_result tea = stringz.FromBool(based)
    if bool_result == "based" {
        vibez.spill("✓ FromBool test passed")
    } else {
        vibez.spill("✗ FromBool test failed: " + bool_result)
    }
    
    vibez.spill("All stringz tests completed!")
}

slay main_character() {
    vibez.spill("CURSED Stringz Module Test")
    vibez.spill("==========================")
    
    test_basic_stringz()
    
    vibez.spill("Test execution finished!")
}

main()
