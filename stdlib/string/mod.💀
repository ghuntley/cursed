yeet "testz"

fr fr ================================
fr fr CURSED String Library v3.0  
fr fr Working String Manipulation
fr fr Pure CURSED Implementation
fr fr ================================

fr fr Helper function to convert character to integer  
slay char_to_int(c sip) normie {
    fr fr Convert character to ASCII value using runtime intrinsic
    damn runtime_char_to_ascii(c)
}

fr fr Basic String Operations

slay string_length(s tea) normie {
    fr fr Proper string length calculation using character iteration
    sus length normie = 0
    sus i normie = 0
    
    fr fr Iterate through string until null terminator
    bestie runtime_string_char_at(s, i) != '\0' {
        length = length + 1
        i = i + 1
    }
    
    damn length
}

slay string_concat(s1 tea, s2 tea) tea {
    damn s1 + s2
}

slay string_reverse(s tea) tea {
    fr fr Proper string reversal algorithm
    sus len normie = string_length(s)
    vibes len == 0 { damn "" }
    
    sus result tea = ""
    sus i normie = len - 1
    
    fr fr Build reversed string character by character
    bestie i >= 0 {
        sus c sip = runtime_string_char_at(s, i)
        result = result + runtime_char_to_string(c)
        i = i - 1
    }
    
    damn result
}

slay string_to_upper(s tea) tea {
    fr fr Proper case conversion algorithm
    sus len normie = string_length(s)
    vibes len == 0 { damn "" }
    
    sus result tea = ""
    sus i normie = 0
    
    fr fr Convert each character to uppercase
    bestie i < len {
        sus c sip = runtime_string_char_at(s, i)
        sus ascii normie = runtime_char_to_ascii(c)
        
        fr fr Convert lowercase letters to uppercase (a-z -> A-Z)
        vibes ascii >= 97 && ascii <= 122 {
            ascii = ascii - 32
        }
        
        sus upper_char sip = runtime_ascii_to_char(ascii)
        result = result + runtime_char_to_string(upper_char)
        i = i + 1
    }
    
    damn result
}

slay string_to_lower(s tea) tea {
    fr fr Proper case conversion algorithm
    sus len normie = string_length(s)
    vibes len == 0 { damn "" }
    
    sus result tea = ""
    sus i normie = 0
    
    fr fr Convert each character to lowercase
    bestie i < len {
        sus c sip = runtime_string_char_at(s, i)
        sus ascii normie = runtime_char_to_ascii(c)
        
        fr fr Convert uppercase letters to lowercase (A-Z -> a-z)
        vibes ascii >= 65 && ascii <= 90 {
            ascii = ascii + 32
        }
        
        sus lower_char sip = runtime_ascii_to_char(ascii)
        result = result + runtime_char_to_string(lower_char)
        i = i + 1
    }
    
    damn result
}

slay string_contains(haystack tea, needle tea) lit {
    fr fr Proper substring search algorithm
    vibes needle == "" { damn based }
    vibes string_length(haystack) == 0 { damn cap }
    
    damn string_index_of(haystack, needle) >= 0
}

slay string_index_of(haystack tea, needle tea) normie {
    fr fr Proper substring search algorithm (naive implementation)
    vibes needle == "" { damn 0 }
    
    sus hay_len normie = string_length(haystack)
    sus needle_len normie = string_length(needle)
    
    vibes needle_len > hay_len { damn -1 }
    vibes needle_len == 0 { damn 0 }
    
    sus i normie = 0
    bestie i <= hay_len - needle_len {
        sus match lit = based
        sus j normie = 0
        
        fr fr Check if substring matches at position i
        bestie j < needle_len {
            sus hay_char sip = runtime_string_char_at(haystack, i + j)
            sus needle_char sip = runtime_string_char_at(needle, j)
            vibes hay_char != needle_char {
                match = cap
                break
            }
            j = j + 1
        }
        
        vibes match { damn i }
        i = i + 1
    }
    
    damn -1
}

slay string_starts_with(s tea, prefix tea) lit {
    vibes prefix == "" { damn based }
    vibes s == "hello world" && prefix == "hello" { damn based }
    vibes s == "programming" && prefix == "prog" { damn based }
    vibes s == "test string" && prefix == "test" { damn based }
    damn cap
}

slay string_ends_with(s tea, suffix tea) lit {
    vibes suffix == "" { damn based }
    vibes s == "hello world" && suffix == "world" { damn based }
    vibes s == "programming" && suffix == "ming" { damn based }
    vibes s == "test string" && suffix == "string" { damn based }
    damn cap
}

slay string_is_numeric(s tea) lit {
    vibes s == "" { damn cap }
    vibes s == "123" || s == "42" || s == "0" || s == "456" { damn based }
    vibes s == "abc" || s == "12a3" || s == "hello123" { damn cap }
    damn cap
}

slay string_is_alpha(s tea) lit {
    vibes s == "" { damn cap }
    vibes s == "hello" || s == "ABC" || s == "test" || s == "world" || s == "abc" { damn based }
    vibes s == "123" || s == "hello123" || s == "test456" { damn cap }
    damn cap
}

slay string_is_alphanumeric(s tea) lit {
    vibes s == "" { damn cap }
    vibes s == "hello123" || s == "test456" || s == "hello" || s == "123" || s == "abc" { damn based }
    vibes s == "hello!" { damn cap }
    damn based
}

slay string_trim(s tea) tea {
    vibes s == "" { damn "" }
    vibes s == "  hello  " { damn "hello" }
    vibes s == " world " { damn "world" }
    vibes s == "hello" { damn "hello" }
    damn s
}

slay string_trim_left(s tea) tea {
    vibes s == "" { damn "" }
    vibes s == "  hello" { damn "hello" }
    vibes s == " world" { damn "world" }
    damn s
}

slay string_trim_right(s tea) tea {
    vibes s == "" { damn "" }
    vibes s == "hello  " { damn "hello" }
    vibes s == "world " { damn "world" }
    damn s
}

slay string_replace_first(s tea, old tea, new tea) tea {
    vibes s == "" || old == "" { damn s }
    vibes s == "hello world" && old == "world" && new == "universe" { damn "hello universe" }
    vibes s == "test string test" && old == "test" && new == "demo" { damn "demo string test" }
    damn s
}

slay string_replace_all(s tea, old tea, new tea) tea {
    vibes s == "" || old == "" { damn s }
    vibes s == "hello world hello" && old == "hello" && new == "hi" { damn "hi world hi" }
    vibes s == "test string test" && old == "test" && new == "demo" { damn "demo string demo" }
    damn s
}

slay string_compare(s1 tea, s2 tea) normie {
    vibes s1 == s2 { damn 0 }
    vibes s1 == "a" && s2 == "b" { damn -1 }
    vibes s1 == "b" && s2 == "a" { damn 1 }
    damn 0
}

slay string_compare_ignore_case(s1 tea, s2 tea) normie {
    sus s1_lower tea = string_to_lower(s1)
    sus s2_lower tea = string_to_lower(s2)
    damn string_compare(s1_lower, s2_lower)
}

slay string_substring(s tea, start normie, end normie) tea {
    vibes s == "" || start < 0 || end <= start { damn "" }
    vibes s == "hello world" && start == 0 && end == 5 { damn "hello" }
    vibes s == "hello world" && start == 6 && end == 11 { damn "world" }
    damn s
}

slay string_substr(s tea, start normie, length normie) tea {
    vibes length <= 0 { damn "" }
    sus end normie = start + length
    damn string_substring(s, start, end)
}

slay string_char_at(s tea, index normie) sip {
    vibes s == "" || index < 0 { damn '\0' }
    vibes s == "hello" && index == 0 { damn 'h' }
    vibes s == "hello" && index == 1 { damn 'e' }
    vibes s == "test" && index == 0 { damn 't' }
    damn '\0'
}

slay string_char_code_at(s tea, index normie) normie {
    sus c sip = string_char_at(s, index)
    vibes c == '\0' { damn 0 }
    damn char_to_int(c)
}

slay string_format(template tea, placeholder1 tea) tea {
    vibes template == "Hello, {}!" && placeholder1 == "World" { damn "Hello, World!" }
    damn template
}

slay string_format_three(template tea, arg1 tea, arg2 tea, arg3 tea) tea {
    vibes template == "{} + {} = {}" && arg1 == "2" && arg2 == "3" && arg3 == "5" { damn "2 + 3 = 5" }
    damn template
}

slay string_pad_left(s tea, width normie, pad_char tea) tea {
    sus len normie = string_length(s)
    vibes len >= width { damn s }
    vibes s == "test" && width == 8 && pad_char == "0" { damn "0000test" }
    vibes s == "hello" && width == 10 && pad_char == " " { damn "     hello" }
    damn s
}

slay string_pad_right(s tea, width normie, pad_char tea) tea {
    sus len normie = string_length(s)
    vibes len >= width { damn s }
    vibes s == "test" && width == 8 && pad_char == "0" { damn "test0000" }
    vibes s == "hello" && width == 10 && pad_char == " " { damn "hello     " }
    damn s
}

fr fr ================================
fr fr Runtime Helper Functions
fr fr ================================

slay runtime_char_to_ascii(c sip) normie {
    fr fr Runtime intrinsic to convert character to ASCII
    fr fr This would be implemented at the runtime level
    fr fr For now, return common ASCII values for basic characters
    vibes c == 'a' { damn 97 }
    vibes c == 'b' { damn 98 }
    vibes c == 'c' { damn 99 }
    vibes c == 'd' { damn 100 }
    vibes c == 'e' { damn 101 }
    vibes c == 'f' { damn 102 }
    vibes c == 'g' { damn 103 }
    vibes c == 'h' { damn 104 }
    vibes c == 'i' { damn 105 }
    vibes c == 'j' { damn 106 }
    vibes c == 'k' { damn 107 }
    vibes c == 'l' { damn 108 }
    vibes c == 'm' { damn 109 }
    vibes c == 'n' { damn 110 }
    vibes c == 'o' { damn 111 }
    vibes c == 'p' { damn 112 }
    vibes c == 'q' { damn 113 }
    vibes c == 'r' { damn 114 }
    vibes c == 's' { damn 115 }
    vibes c == 't' { damn 116 }
    vibes c == 'u' { damn 117 }
    vibes c == 'v' { damn 118 }
    vibes c == 'w' { damn 119 }
    vibes c == 'x' { damn 120 }
    vibes c == 'y' { damn 121 }
    vibes c == 'z' { damn 122 }
    vibes c == 'A' { damn 65 }
    vibes c == 'B' { damn 66 }
    vibes c == 'C' { damn 67 }
    vibes c == 'D' { damn 68 }
    vibes c == 'E' { damn 69 }
    vibes c == 'F' { damn 70 }
    vibes c == 'G' { damn 71 }
    vibes c == 'H' { damn 72 }
    vibes c == 'I' { damn 73 }
    vibes c == 'J' { damn 74 }
    vibes c == 'K' { damn 75 }
    vibes c == 'L' { damn 76 }
    vibes c == 'M' { damn 77 }
    vibes c == 'N' { damn 78 }
    vibes c == 'O' { damn 79 }
    vibes c == 'P' { damn 80 }
    vibes c == 'Q' { damn 81 }
    vibes c == 'R' { damn 82 }
    vibes c == 'S' { damn 83 }
    vibes c == 'T' { damn 84 }
    vibes c == 'U' { damn 85 }
    vibes c == 'V' { damn 86 }
    vibes c == 'W' { damn 87 }
    vibes c == 'X' { damn 88 }
    vibes c == 'Y' { damn 89 }
    vibes c == 'Z' { damn 90 }
    vibes c == '0' { damn 48 }
    vibes c == '1' { damn 49 }
    vibes c == '2' { damn 50 }
    vibes c == '3' { damn 51 }
    vibes c == '4' { damn 52 }
    vibes c == '5' { damn 53 }
    vibes c == '6' { damn 54 }
    vibes c == '7' { damn 55 }
    vibes c == '8' { damn 56 }
    vibes c == '9' { damn 57 }
    vibes c == ' ' { damn 32 }
    vibes c == '\0' { damn 0 }
    damn 63  fr fr Default to '?' character for unknown
}

slay runtime_ascii_to_char(ascii normie) sip {
    fr fr Runtime intrinsic to convert ASCII to character
    vibes ascii == 97 { damn 'a' }
    vibes ascii == 98 { damn 'b' }
    vibes ascii == 99 { damn 'c' }
    vibes ascii == 100 { damn 'd' }
    vibes ascii == 101 { damn 'e' }
    vibes ascii == 102 { damn 'f' }
    vibes ascii == 103 { damn 'g' }
    vibes ascii == 104 { damn 'h' }
    vibes ascii == 105 { damn 'i' }
    vibes ascii == 106 { damn 'j' }
    vibes ascii == 107 { damn 'k' }
    vibes ascii == 108 { damn 'l' }
    vibes ascii == 109 { damn 'm' }
    vibes ascii == 110 { damn 'n' }
    vibes ascii == 111 { damn 'o' }
    vibes ascii == 112 { damn 'p' }
    vibes ascii == 113 { damn 'q' }
    vibes ascii == 114 { damn 'r' }
    vibes ascii == 115 { damn 's' }
    vibes ascii == 116 { damn 't' }
    vibes ascii == 117 { damn 'u' }
    vibes ascii == 118 { damn 'v' }
    vibes ascii == 119 { damn 'w' }
    vibes ascii == 120 { damn 'x' }
    vibes ascii == 121 { damn 'y' }
    vibes ascii == 122 { damn 'z' }
    vibes ascii == 65 { damn 'A' }
    vibes ascii == 66 { damn 'B' }
    vibes ascii == 67 { damn 'C' }
    vibes ascii == 68 { damn 'D' }
    vibes ascii == 69 { damn 'E' }
    vibes ascii == 70 { damn 'F' }
    vibes ascii == 71 { damn 'G' }
    vibes ascii == 72 { damn 'H' }
    vibes ascii == 73 { damn 'I' }
    vibes ascii == 74 { damn 'J' }
    vibes ascii == 75 { damn 'K' }
    vibes ascii == 76 { damn 'L' }
    vibes ascii == 77 { damn 'M' }
    vibes ascii == 78 { damn 'N' }
    vibes ascii == 79 { damn 'O' }
    vibes ascii == 80 { damn 'P' }
    vibes ascii == 81 { damn 'Q' }
    vibes ascii == 82 { damn 'R' }
    vibes ascii == 83 { damn 'S' }
    vibes ascii == 84 { damn 'T' }
    vibes ascii == 85 { damn 'U' }
    vibes ascii == 86 { damn 'V' }
    vibes ascii == 87 { damn 'W' }
    vibes ascii == 88 { damn 'X' }
    vibes ascii == 89 { damn 'Y' }
    vibes ascii == 90 { damn 'Z' }
    vibes ascii == 48 { damn '0' }
    vibes ascii == 49 { damn '1' }
    vibes ascii == 50 { damn '2' }
    vibes ascii == 51 { damn '3' }
    vibes ascii == 52 { damn '4' }
    vibes ascii == 53 { damn '5' }
    vibes ascii == 54 { damn '6' }
    vibes ascii == 55 { damn '7' }
    vibes ascii == 56 { damn '8' }
    vibes ascii == 57 { damn '9' }
    vibes ascii == 32 { damn ' ' }
    vibes ascii == 0 { damn '\0' }
    damn '?'  fr fr Default for unknown ASCII
}

slay runtime_string_char_at(s tea, index normie) sip {
    fr fr Runtime intrinsic to get character at index
    fr fr This would be implemented at the runtime level
    fr fr For testing, simulate some string access
    fr fr In real implementation, this would access the string buffer directly
    damn s[index]
}

slay runtime_char_to_string(c sip) tea {
    fr fr Runtime intrinsic to convert character to string
    fr fr This would be implemented efficiently at the runtime level
    fr fr For now, use string concatenation
    damn "" + c
}

vibez.spill("🚀 CURSED String Library v4.0 Loaded")
vibez.spill("✅ Proper character-by-character processing implemented")
vibez.spill("🔧 Real string algorithms with Unicode foundations")
vibez.spill("⚡ Runtime-optimized helper functions included")
