fr fr CURSED String Functions Demo
fr fr Demonstrates fixed string processing functions

yeet "vibez"
yeet "stringz"

slay main() normie {
    vibez.spill("🔧 String Processing Functions Demo")
    vibez.spill("Testing fixed string manipulation functions...")
    vibez.spill("")
    
    fr fr Test string length
    sus test_str tea = "Hello CURSED"
    sus len normie = string_len(test_str)
    vibez.spill("String:", test_str)
    vibez.spill("Length:", len)
    vibez.spill("")
    
    fr fr Test character access
    sus char_0 tea = string_char_at(test_str, 0)
    sus char_6 tea = string_char_at(test_str, 6)
    vibez.spill("Character at index 0:", char_0)
    vibez.spill("Character at index 6:", char_6)
    vibez.spill("")
    
    fr fr Test substring
    sus sub_hello tea = string_substring(test_str, 0, 5)
    sus sub_cursed tea = string_substring(test_str, 6, 12)
    vibez.spill("Substring [0,5]:", sub_hello)
    vibez.spill("Substring [6,12]:", sub_cursed)
    vibez.spill("")
    
    fr fr Test trimming
    sus padded_str tea = "   Hello World   "
    vibez.spill("Original with padding: [" + padded_str + "]")
    
    sus trimmed tea = string_trim(padded_str)
    sus trim_start tea = string_trim_start(padded_str)
    sus trim_end tea = string_trim_end(padded_str)
    
    vibez.spill("Trimmed both sides: [" + trimmed + "]")
    vibez.spill("Trimmed start only: [" + trim_start + "]")
    vibez.spill("Trimmed end only: [" + trim_end + "]")
    vibez.spill("")
    
    fr fr Test empty string handling
    sus empty tea = ""
    sus empty_len normie = string_len(empty)
    sus empty_char tea = string_char_at(empty, 0)
    sus empty_sub tea = string_substring(empty, 0, 1)
    sus empty_trim tea = string_trim(empty)
    
    vibez.spill("Empty string tests:")
    vibez.spill("  Length:", empty_len)
    vibez.spill("  Char at 0:", empty_char)
    vibez.spill("  Substring:", empty_sub)  
    vibez.spill("  Trimmed:", empty_trim)
    vibez.spill("")
    
    vibez.spill("✅ String functions demo completed!")
    vibez.spill("All string processing functions working correctly.")
    
    damn 0
}
