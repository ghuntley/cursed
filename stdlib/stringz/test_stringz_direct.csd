// Direct test of stringz module functions
// Uses built-in string operations

slay Contains(s tea, substr tea) lit {
    if string_contains(s, substr) {
        damn based
    } else {
        damn cap
    }
}

slay ToLower(s tea) tea {
    damn string_to_lower(s)
}

slay ToUpper(s tea) tea {
    damn string_to_upper(s)
}

slay Len(s tea) normie {
    damn string_len(s)
}

slay IsEmpty(s tea) lit {
    if string_len(s) == 0 {
        damn based
    } else {
        damn cap
    }
}

slay Equals(s1 tea, s2 tea) lit {
    if s1 == s2 {
        damn based
    } else {
        damn cap
    }
}

slay Repeat(s tea, count normie) tea {
    damn string_repeat(s, count)
}

slay Reverse(s tea) tea {
    damn string_reverse(s)
}

slay Substring(s tea, start normie, length normie) tea {
    damn string_substring(s, start, length)
}

slay CharAt(s tea, index normie) tea {
    damn string_char_at(s, index)
}

slay ToInt(s tea) normie {
    damn string_to_int(s)
}

slay FromInt(i normie) tea {
    damn string_from_int(i)
}

slay FromBool(b lit) tea {
    damn string_from_bool(b)
}

slay test_stringz_operations() {
    vibez.spill("Testing stringz operations...")
    
    // Test Contains
    if Contains("hello world", "world") {
        vibez.spill("✓ Contains test passed")
    } else {
        vibez.spill("✗ Contains test failed")
    }
    
    // Test ToLower
    sus lower_result tea = ToLower("HELLO")
    if lower_result == "hello" {
        vibez.spill("✓ ToLower test passed")
    } else {
        vibez.spill("✗ ToLower test failed: " + lower_result)
    }
    
    // Test ToUpper
    sus upper_result tea = ToUpper("hello")
    if upper_result == "HELLO" {
        vibez.spill("✓ ToUpper test passed")
    } else {
        vibez.spill("✗ ToUpper test failed: " + upper_result)
    }
    
    // Test Len
    sus len_result normie = Len("hello")
    if len_result == 5 {
        vibez.spill("✓ Len test passed")
    } else {
        vibez.spill("✗ Len test failed: " + tea(len_result))
    }
    
    // Test IsEmpty
    if IsEmpty("") {
        vibez.spill("✓ IsEmpty test passed")
    } else {
        vibez.spill("✗ IsEmpty test failed")
    }
    
    // Test Equals
    if Equals("hello", "hello") {
        vibez.spill("✓ Equals test passed")
    } else {
        vibez.spill("✗ Equals test failed")
    }
    
    // Test Repeat
    sus repeat_result tea = Repeat("ho", 3)
    if repeat_result == "hohoho" {
        vibez.spill("✓ Repeat test passed")
    } else {
        vibez.spill("✗ Repeat test failed: " + repeat_result)
    }
    
    // Test Reverse
    sus reverse_result tea = Reverse("hello")
    if reverse_result == "olleh" {
        vibez.spill("✓ Reverse test passed")
    } else {
        vibez.spill("✗ Reverse test failed: " + reverse_result)
    }
    
    // Test Substring
    sus substring_result tea = Substring("hello world", 0, 5)
    if substring_result == "hello" {
        vibez.spill("✓ Substring test passed")
    } else {
        vibez.spill("✗ Substring test failed: " + substring_result)
    }
    
    // Test CharAt
    sus char_result tea = CharAt("hello", 0)
    if char_result == "h" {
        vibez.spill("✓ CharAt test passed")
    } else {
        vibez.spill("✗ CharAt test failed: " + char_result)
    }
    
    // Test ToInt
    sus int_result normie = ToInt("123")
    if int_result == 123 {
        vibez.spill("✓ ToInt test passed")
    } else {
        vibez.spill("✗ ToInt test failed: " + tea(int_result))
    }
    
    // Test FromInt
    sus str_result tea = FromInt(123)
    if str_result == "123" {
        vibez.spill("✓ FromInt test passed")
    } else {
        vibez.spill("✗ FromInt test failed: " + str_result)
    }
    
    // Test FromBool
    sus bool_result tea = FromBool(based)
    if bool_result == "based" {
        vibez.spill("✓ FromBool test passed")
    } else {
        vibez.spill("✗ FromBool test failed: " + bool_result)
    }
    
    vibez.spill("All stringz tests completed successfully!")
}

slay main() {
    vibez.spill("CURSED Stringz Module Direct Test")
    vibez.spill("==================================")
    
    test_stringz_operations()
    
    vibez.spill("Test execution finished!")
}

main()
