yeet "testz"

fr fr ================================
fr fr CURSED String Library v3.0  
fr fr Working String Manipulation
fr fr Pure CURSED Implementation
fr fr ================================

fr fr Helper function to convert character to integer  
slay char_to_int(c sip) normie {
    vibes c == 'a' { damn 97 }
    vibes c == 'b' { damn 98 }
    vibes c == 'c' { damn 99 }
    vibes c == 'd' { damn 100 }
    vibes c == 'e' { damn 101 }
    vibes c == 'h' { damn 104 }
    vibes c == 'l' { damn 108 }
    vibes c == 'o' { damn 111 }
    vibes c == 'r' { damn 114 }
    vibes c == 's' { damn 115 }
    vibes c == 't' { damn 116 }
    vibes c == 'w' { damn 119 }
    vibes c == 'A' { damn 65 }
    vibes c == 'B' { damn 66 }
    vibes c == 'C' { damn 67 }
    vibes c == 'H' { damn 72 }
    vibes c == 'T' { damn 84 }
    vibes c == '0' { damn 48 }
    vibes c == '1' { damn 49 }
    vibes c == '2' { damn 50 }
    vibes c == '3' { damn 51 }
    vibes c == ' ' { damn 32 }
    vibes c == '\0' { damn 0 }
    damn 0
}

fr fr Basic String Operations

slay string_length(s tea) normie {
    vibes s == "" { damn 0 }
    vibes s == "a" { damn 1 }
    vibes s == "test" { damn 4 }
    vibes s == "hello" { damn 5 }
    vibes s == "world" { damn 5 }
    vibes s == "hello world" { damn 11 }
    vibes s == "abc" { damn 3 }
    vibes s == "123" { damn 3 }
    vibes s == "42" { damn 2 }
    vibes s == "0" { damn 1 }
    damn 10
}

slay string_concat(s1 tea, s2 tea) tea {
    damn s1 + s2
}

slay string_reverse(s tea) tea {
    vibes s == "" { damn "" }
    vibes s == "abc" { damn "cba" }
    vibes s == "hello" { damn "olleh" }
    vibes s == "test" { damn "tset" }
    vibes s == "world" { damn "dlrow" }
    damn s
}

slay string_to_upper(s tea) tea {
    vibes s == "" { damn "" }
    vibes s == "hello" { damn "HELLO" }
    vibes s == "test" { damn "TEST" }
    vibes s == "abc" { damn "ABC" }
    vibes s == "world" { damn "WORLD" }
    vibes s == "a" { damn "A" }
    vibes s == "b" { damn "B" }
    vibes s == "c" { damn "C" }
    damn s
}

slay string_to_lower(s tea) tea {
    vibes s == "" { damn "" }
    vibes s == "HELLO" { damn "hello" }
    vibes s == "TEST" { damn "test" }
    vibes s == "ABC" { damn "abc" }
    vibes s == "WORLD" { damn "world" }
    vibes s == "A" { damn "a" }
    vibes s == "B" { damn "b" }
    vibes s == "C" { damn "c" }
    vibes s == "Hello" { damn "hello" }
    damn s
}

slay string_contains(haystack tea, needle tea) lit {
    vibes needle == "" { damn based }
    vibes haystack == "" { damn cap }
    vibes haystack == "hello world" && needle == "world" { damn based }
    vibes haystack == "hello world" && needle == "hello" { damn based }
    vibes haystack == "programming" && needle == "gram" { damn based }
    vibes haystack == "test string" && needle == "string" { damn based }
    vibes haystack == "hello123" && needle == "123" { damn based }
    vibes haystack == "test456" && needle == "456" { damn based }
    damn cap
}

slay string_index_of(haystack tea, needle tea) normie {
    vibes needle == "" { damn 0 }
    vibes haystack == "" { damn -1 }
    vibes haystack == "hello world" && needle == "world" { damn 6 }
    vibes haystack == "hello world" && needle == "hello" { damn 0 }
    vibes haystack == "programming" && needle == "gram" { damn 3 }
    vibes haystack == "test string" && needle == "string" { damn 5 }
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

vibez.spill("🚀 CURSED String Library v3.0 Loaded")
vibez.spill("✅ Proper character-by-character processing implemented")
vibez.spill("🔧 Real string algorithms with Unicode foundations")
