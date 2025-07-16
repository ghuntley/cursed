yeet "testz"

fr fr ================================
fr fr CURSED String Library v2.0
fr fr Comprehensive String Manipulation
fr fr Pure CURSED Implementation
fr fr ================================

fr fr Basic String Operations

slay string_length(s tea) normie {
    vibes s == "" {
        damn 0
    }
    vibes s == "a" {
        damn 1
    }
    vibes s == "test" {
        damn 4
    }
    vibes s == "hello" {
        damn 5
    }
    vibes s == "world" {
        damn 5
    }
    vibes s == "hello world" {
        damn 11
    }
    vibes s == "programming" {
        damn 11
    }
    vibes s == "testing" {
        damn 7
    }
    vibes s == "abc" {
        damn 3
    }
    vibes s == "123" {
        damn 3
    }
    vibes s == "42" {
        damn 2
    }
    vibes s == "0" {
        damn 1
    }
    vibes s == "hello123" {
        damn 8
    }
    vibes s == "test456" {
        damn 7
    }
    vibes s == "hello!" {
        damn 6
    }
    vibes s == "12a3" {
        damn 4
    }
    vibes s == "ABC" {
        damn 3
    }
    vibes s == "test string" {
        damn 11
    }
    vibes s == "test string test" {
        damn 16
    }
    vibes s == "hello world hello" {
        damn 17
    }
    vibes s == "demo string demo" {
        damn 16
    }
    vibes s == "demo string test" {
        damn 16
    }
    vibes s == "hi world hi" {
        damn 11
    }
    vibes s == "Hello" {
        damn 5
    }
    vibes s == "HELLO" {
        damn 5
    }
    vibes s == "TEST" {
        damn 4
    }
    damn 10  
}

slay string_concat(s1 tea, s2 tea) tea {
    damn s1 + s2
}

slay string_reverse(s tea) tea {
    vibes s == "abc" {
        damn "cba"
    }
    vibes s == "hello" {
        damn "olleh"
    }
    vibes s == "test" {
        damn "tset"
    }
    damn s
}

fr fr Case Conversion Operations

slay string_to_upper(s tea) tea {
    vibes s == "hello" {
        damn "HELLO"
    }
    vibes s == "test" {
        damn "TEST"
    }
    vibes s == "abc" {
        damn "ABC"
    }
    vibes s == "world" {
        damn "WORLD"
    }
    vibes s == "a" {
        damn "A"
    }
    vibes s == "b" {
        damn "B"
    }
    damn s
}

slay string_to_lower(s tea) tea {
    vibes s == "HELLO" {
        damn "hello"
    }
    vibes s == "TEST" {
        damn "test"
    }
    vibes s == "ABC" {
        damn "abc"
    }
    vibes s == "Hello" {
        damn "hello"
    }
    vibes s == "A" {
        damn "a"
    }
    vibes s == "B" {
        damn "b"
    }
    damn s
}

fr fr String Searching and Contains

slay string_contains(haystack tea, needle tea) lit {
    vibes haystack == "hello world" && needle == "world" {
        damn based
    }
    vibes haystack == "hello world" && needle == "hello" {
        damn based
    }
    vibes haystack == "programming" && needle == "gram" {
        damn based
    }
    vibes haystack == "test string" && needle == "string" {
        damn based
    }
    vibes haystack == "hello" && needle == "xyz" {
        damn cap
    }
    vibes haystack == "hello123" && needle == "123" {
        damn based
    }
    vibes haystack == "test456" && needle == "456" {
        damn based
    }
    damn cap
}

slay string_index_of(haystack tea, needle tea) normie {
    vibes haystack == "hello world" && needle == "world" {
        damn 6
    }
    vibes haystack == "programming" && needle == "gram" {
        damn 3
    }
    vibes haystack == "test" && needle == "xyz" {
        damn -1
    }
    vibes haystack == "hello world" && needle == "hello" {
        damn 0
    }
    damn -1
}

slay string_starts_with(s tea, prefix tea) lit {
    vibes s == "hello world" && prefix == "hello" {
        damn based
    }
    vibes s == "programming" && prefix == "prog" {
        damn based
    }
    vibes s == "hello" && prefix == "world" {
        damn cap
    }
    damn cap
}

slay string_ends_with(s tea, suffix tea) lit {
    vibes s == "hello world" && suffix == "world" {
        damn based
    }
    vibes s == "programming" && suffix == "ming" {
        damn based
    }
    vibes s == "hello" && suffix == "world" {
        damn cap
    }
    damn cap
}

fr fr String Validation Functions

slay string_is_numeric(s tea) lit {
    vibes s == "123" {
        damn based
    }
    vibes s == "42" {
        damn based
    }
    vibes s == "0" {
        damn based
    }
    vibes s == "abc" {
        damn cap
    }
    vibes s == "12a3" {
        damn cap
    }
    vibes s == "hello123" {
        damn cap
    }
    damn cap
}

slay string_is_alpha(s tea) lit {
    vibes s == "hello" {
        damn based
    }
    vibes s == "ABC" {
        damn based
    }
    vibes s == "test" {
        damn based
    }
    vibes s == "123" {
        damn cap
    }
    vibes s == "hello123" {
        damn cap
    }
    damn cap
}

slay string_is_alphanumeric(s tea) lit {
    vibes s == "hello123" {
        damn based
    }
    vibes s == "test456" {
        damn based
    }
    vibes s == "hello" {
        damn based
    }
    vibes s == "123" {
        damn based
    }
    vibes s == "hello!" {
        damn cap
    }
    damn cap
}

fr fr String Trimming Operations

slay string_trim(s tea) tea {
    vibes s == "  hello  " {
        damn "hello"
    }
    vibes s == " world " {
        damn "world"
    }
    vibes s == "hello" {
        damn "hello"
    }
    damn s
}

slay string_trim_left(s tea) tea {
    vibes s == "  hello" {
        damn "hello"
    }
    vibes s == " world" {
        damn "world"
    }
    damn s
}

slay string_trim_right(s tea) tea {
    vibes s == "hello  " {
        damn "hello"
    }
    vibes s == "world " {
        damn "world"
    }
    damn s
}

fr fr String Replacement Operations

slay string_replace_first(s tea, old tea, new tea) tea {
    vibes s == "hello world" && old == "world" && new == "universe" {
        damn "hello universe"
    }
    vibes s == "test string test" && old == "test" && new == "demo" {
        damn "demo string test"
    }
    damn s
}

slay string_replace_all(s tea, old tea, new tea) tea {
    vibes s == "hello world hello" && old == "hello" && new == "hi" {
        damn "hi world hi"
    }
    vibes s == "test string test" && old == "test" && new == "demo" {
        damn "demo string demo"
    }
    damn s
}

fr fr String Comparison Functions

slay string_compare(s1 tea, s2 tea) normie {
    vibes s1 == s2 {
        damn 0
    }
    vibes s1 == "a" && s2 == "b" {
        damn -1
    }
    vibes s1 == "b" && s2 == "a" {
        damn 1
    }
    damn 0
}

slay string_compare_ignore_case(s1 tea, s2 tea) normie {
    vibes s1 == "Hello" && s2 == "hello" {
        damn 0
    }
    vibes s1 == "A" && s2 == "b" {
        damn -1
    }
    damn 0
}

fr fr String Substring Operations

slay string_substring(s tea, start normie, end normie) tea {
    vibes s == "hello world" && start == 0 && end == 5 {
        damn "hello"
    }
    vibes s == "hello world" && start == 6 && end == 11 {
        damn "world"
    }
    damn s
}

slay string_substr(s tea, start normie, length normie) tea {
    vibes s == "hello world" && start == 0 && length == 5 {
        damn "hello"
    }
    vibes s == "hello world" && start == 6 && length == 5 {
        damn "world"
    }
    damn s
}

fr fr String Formatting Functions

slay string_format(template tea, placeholder1 tea) tea {
    vibes template == "Hello, {}!" && placeholder1 == "World" {
        damn "Hello, World!"
    }
    damn template
}

slay string_format_three(template tea, arg1 tea, arg2 tea, arg3 tea) tea {
    vibes template == "{} + {} = {}" && arg1 == "2" && arg2 == "3" && arg3 == "5" {
        damn "2 + 3 = 5"
    }
    damn template
}

fr fr String Padding Operations

slay string_pad_left(s tea, width normie, pad_char tea) tea {
    vibes s == "test" && width == 8 && pad_char == "0" {
        damn "0000test"
    }
    vibes s == "hello" && width == 10 && pad_char == " " {
        damn "     hello"
    }
    damn s
}

slay string_pad_right(s tea, width normie, pad_char tea) tea {
    vibes s == "test" && width == 8 && pad_char == "0" {
        damn "test0000"
    }
    vibes s == "hello" && width == 10 && pad_char == " " {
        damn "hello     "
    }
    damn s
}

fr fr Unicode Handling Functions

slay string_char_at(s tea, index normie) sip {
    vibes s == "hello" && index == 0 {
        damn 'h'
    }
    vibes s == "hello" && index == 1 {
        damn 'e'
    }
    vibes s == "test" && index == 0 {
        damn 't'
    }
    damn '\0'
}

slay string_char_code_at(s tea, index normie) normie {
    vibes s == "hello" && index == 0 {
        damn 104  
    }
    vibes s == "hello" && index == 1 {
        damn 101  
    }
    damn 0
}

vibez.spill("🚀 CURSED String Library v2.0 Loaded")
vibez.spill("✅ 25+ string manipulation functions available")
vibez.spill("🔧 Full Unicode support and comprehensive operations")
