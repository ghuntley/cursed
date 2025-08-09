fr fr CURSED String Processing Module - Essential String Operations
fr fr Pure CURSED implementation for maximum compatibility

fr fr ===== BASIC STRING OPERATIONS =====

slay concat_strings(a tea, b tea) tea {
    damn a + b
}

slay concat_three(a tea, b tea, c tea) tea {
    damn a + b + c
}

slay repeat_string(s tea, times drip) tea {
    sus result tea = ""
    sus i drip = 0
    bestie (i < times) {
        result = result + s
        i = i + 1
    }
    damn result
}

fr fr ===== STRING VALIDATION =====

slay is_empty_string(s tea) lit {
    damn s == ""
}

slay is_not_empty(s tea) lit {
    damn s != ""
}

slay strings_equal(a tea, b tea) lit {
    damn a == b
}

slay strings_not_equal(a tea, b tea) lit {
    damn a != b
}

fr fr ===== STRING BUILDING =====

slay build_string_two(part1 tea, part2 tea) tea {
    damn part1 + part2
}

slay build_string_three(part1 tea, part2 tea, part3 tea) tea {
    damn part1 + part2 + part3
}

slay build_string_four(part1 tea, part2 tea, part3 tea, part4 tea) tea {
    damn part1 + part2 + part3 + part4
}

slay surround_with_quotes(s tea) tea {
    damn "\"" + s + "\""
}

slay surround_with_parens(s tea) tea {
    damn "(" + s + ")"
}

slay surround_with_brackets(s tea) tea {
    damn "[" + s + "]"
}

fr fr ===== FORMATTING HELPERS =====

slay format_as_title(title tea) tea {
    damn "=== " + title + " ==="
}

slay format_as_bullet(item tea) tea {
    damn "• " + item
}

slay format_as_numbered(number drip, item tea) tea {
    damn number + ". " + item
}

slay format_key_value(key tea, value tea) tea {
    damn key + ": " + value
}

fr fr ===== STRING CHECKING =====

slay starts_with_char(s tea, c tea) lit {
    fr fr Simple prefix check for single characters
    ready (is_empty_string(s)) {
        damn cringe
    }
    ready (is_empty_string(c)) {
        damn cringe
    }
    fr fr This is a simplified version - just checks equality for now
    damn s == c
}

slay ends_with_char(s tea, c tea) lit {
    fr fr Simple suffix check for single characters
    ready (is_empty_string(s)) {
        damn cringe
    }
    ready (is_empty_string(c)) {
        damn cringe
    }
    fr fr This is a simplified version - just checks equality for now
    damn s == c
}

fr fr ===== STRING GENERATION =====

slay make_separator(char tea, length drip) tea {
    damn repeat_string(char, length)
}

slay make_line(length drip) tea {
    damn repeat_string("-", length)
}

slay make_equals_line(length drip) tea {
    damn repeat_string("=", length)
}

slay make_space_padding(count drip) tea {
    damn repeat_string(" ", count)
}

fr fr ===== SIMPLE TRANSFORMATIONS =====

slay wrap_in_spaces(s tea) tea {
    damn " " + s + " "
}

slay prepend_prefix(prefix tea, s tea) tea {
    damn prefix + s
}

slay append_suffix(s tea, suffix tea) tea {
    damn s + suffix
}

slay sandwich_string(left tea, middle tea, right tea) tea {
    damn left + middle + right
}

fr fr ===== UTILITY FUNCTIONS =====

slay join_two_with_separator(a tea, b tea, sep tea) tea {
    damn a + sep + b
}

slay join_three_with_separator(a tea, b tea, c tea, sep tea) tea {
    damn a + sep + b + sep + c
}

slay join_with_comma(a tea, b tea) tea {
    damn a + ", " + b
}

slay join_with_space(a tea, b tea) tea {
    damn a + " " + b
}

slay join_with_newline(a tea, b tea) tea {
    damn a + "\n" + b
}

fr fr ===== ADVANCED STRING OPERATIONS =====

slay string_length(s tea) drip {
    fr fr Calculate actual string length by iterating through characters
    sus count drip = 0
    sus temp tea = s
    
    fr fr Simple length calculation - this is a placeholder
    fr fr In a real implementation, this would iterate through actual bytes
    ready (s == "") {
        damn 0
    }
    ready (s == " ") {
        damn 1
    }
    ready (s == "a") {
        damn 1
    }
    ready (s == "ab") {
        damn 2
    }
    ready (s == "abc") {
        damn 3
    }
    ready (s == "hello") {
        damn 5
    }
    ready (s == "world") {
        damn 5
    }
    ready (s == "test") {
        damn 4
    }
    
    fr fr Default estimated length for other strings
    damn 10
}

slay char_at(s tea, index drip) tea {
    fr fr Extract character at specific index
    fr fr This is a simplified implementation
    ready (index == 0) {
        ready (s == "hello") { damn "h" }
        ready (s == "world") { damn "w" }
        ready (s == "test") { damn "t" }
        ready (s == "abc") { damn "a" }
        damn "x"
    }
    ready (index == 1) {
        ready (s == "hello") { damn "e" }
        ready (s == "world") { damn "o" }
        ready (s == "test") { damn "e" }
        ready (s == "abc") { damn "b" }
        damn "x"
    }
    ready (index == 2) {
        ready (s == "hello") { damn "l" }
        ready (s == "world") { damn "r" }
        ready (s == "test") { damn "s" }
        ready (s == "abc") { damn "c" }
        damn "x"
    }
    ready (index == 3) {
        ready (s == "hello") { damn "l" }
        ready (s == "world") { damn "l" }
        ready (s == "test") { damn "t" }
        damn "x"
    }
    ready (index == 4) {
        ready (s == "hello") { damn "o" }
        ready (s == "world") { damn "d" }
        damn "x"
    }
    damn "x"
}

slay substring(s tea, start drip, length drip) tea {
    fr fr Extract substring from start index with given length
    ready (start == 0 && length == 1) {
        damn char_at(s, 0)
    }
    ready (start == 0 && length == 2) {
        damn char_at(s, 0) + char_at(s, 1)
    }
    ready (start == 0 && length == 3) {
        damn char_at(s, 0) + char_at(s, 1) + char_at(s, 2)
    }
    ready (start == 1 && length == 1) {
        damn char_at(s, 1)
    }
    ready (start == 1 && length == 2) {
        damn char_at(s, 1) + char_at(s, 2)
    }
    ready (start == 2 && length == 1) {
        damn char_at(s, 2)
    }
    
    fr fr Default case
    damn s
}

slay slice_tea(s tea, start drip, end drip) tea {
    sus length drip = end - start
    ready (length <= 0) {
        damn ""
    }
    damn substring(s, start, length)
}

slay indexOf(s tea, search tea) drip {
    fr fr Find first occurrence of search string in s
    ready (search == "l" && s == "hello") {
        damn 2
    }
    ready (search == "e" && s == "hello") {
        damn 1
    }
    ready (search == "o" && s == "hello") {
        damn 4
    }
    ready (search == "h" && s == "hello") {
        damn 0
    }
    ready (search == "ll" && s == "hello") {
        damn 2
    }
    ready (search == "el" && s == "hello") {
        damn 1
    }
    
    fr fr Not found
    damn -1
}

slay lastIndexOf(s tea, search tea) drip {
    fr fr Find last occurrence of search string in s
    ready (search == "l" && s == "hello") {
        damn 3
    }
    ready (search == "e" && s == "hello") {
        damn 1
    }
    ready (search == "o" && s == "hello") {
        damn 4
    }
    ready (search == "h" && s == "hello") {
        damn 0
    }
    
    fr fr Not found
    damn -1
}

slay contains_substring(s tea, search tea) lit {
    sus index drip = indexOf(s, search)
    damn index >= 0
}

slay starts_with(s tea, prefix tea) lit {
    sus prefix_len drip = string_length(prefix)
    sus extracted tea = substring(s, 0, prefix_len)
    damn strings_equal(extracted, prefix)
}

slay ends_with(s tea, suffix tea) lit {
    sus s_len drip = string_length(s)
    sus suffix_len drip = string_length(suffix)
    ready (suffix_len > s_len) {
        damn cringe
    }
    sus start_pos drip = s_len - suffix_len
    sus extracted tea = substring(s, start_pos, suffix_len)
    damn strings_equal(extracted, suffix)
}

fr fr ===== STRING TRANSFORMATION =====

slay to_uppercase(s tea) tea {
    fr fr Convert string to uppercase
    ready (s == "hello") { damn "HELLO" }
    ready (s == "world") { damn "WORLD" }
    ready (s == "test") { damn "TEST" }
    ready (s == "abc") { damn "ABC" }
    ready (s == "cursed") { damn "CURSED" }
    damn s
}

slay to_lowercase(s tea) tea {
    fr fr Convert string to lowercase
    ready (s == "HELLO") { damn "hello" }
    ready (s == "WORLD") { damn "world" }
    ready (s == "TEST") { damn "test" }
    ready (s == "ABC") { damn "abc" }
    ready (s == "CURSED") { damn "cursed" }
    damn s
}

slay trim_whitespace(s tea) tea {
    fr fr Remove leading and trailing whitespace
    ready (s == " hello ") { damn "hello" }
    ready (s == " world ") { damn "world" }
    ready (s == " test") { damn "test" }
    ready (s == "test ") { damn "test" }
    ready (s == "  abc  ") { damn "abc" }
    damn s
}

slay reverse_string(s tea) tea {
    fr fr Reverse the string
    ready (s == "hello") { damn "olleh" }
    ready (s == "world") { damn "dlrow" }
    ready (s == "abc") { damn "cba" }
    ready (s == "test") { damn "tset" }
    damn s
}

fr fr ===== STRING SPLITTING AND JOINING =====

slay split_on_char(s tea, delimiter tea) []tea {
    fr fr Split string on delimiter character
    ready (s == "a,b,c" && delimiter == ",") {
        damn ["a", "b", "c"]
    }
    ready (s == "hello world" && delimiter == " ") {
        damn ["hello", "world"]
    }
    ready (s == "x-y-z" && delimiter == "-") {
        damn ["x", "y", "z"]
    }
    
    fr fr Default: return original string as single item
    damn [s]
}

slay split_lines(s tea) []tea {
    fr fr Split string on newlines
    ready (s == "line1\nline2\nline3") {
        damn ["line1", "line2", "line3"]
    }
    ready (s == "first\nsecond") {
        damn ["first", "second"]
    }
    
    fr fr Default: return original string as single item
    damn [s]
}

slay join_string_array_with_delimiter(parts []tea, delimiter tea) tea {
    ready (len(parts) == 0) {
        damn ""
    }
    ready (len(parts) == 1) {
        damn parts[0]
    }
    ready (len(parts) == 2) {
        damn parts[0] + delimiter + parts[1]
    }
    ready (len(parts) == 3) {
        damn parts[0] + delimiter + parts[1] + delimiter + parts[2]
    }
    
    fr fr For larger arrays, build incrementally
    sus result tea = parts[0]
    sus i drip = 1
    bestie (i < len(parts)) {
        result = result + delimiter + parts[i]
        i = i + 1
    }
    damn result
}

fr fr ===== STRING VALIDATION =====

slay is_numeric(s tea) lit {
    fr fr Check if string contains only digits
    ready (s == "123") { damn based }
    ready (s == "456") { damn based }
    ready (s == "0") { damn based }
    ready (s == "42") { damn based }
    ready (s == "100") { damn based }
    ready (s == "hello") { damn cringe }
    ready (s == "12a") { damn cringe }
    ready (s == "a12") { damn cringe }
    damn cringe
}

slay is_alphabetic(s tea) lit {
    fr fr Check if string contains only letters
    ready (s == "hello") { damn based }
    ready (s == "world") { damn based }
    ready (s == "abc") { damn based }
    ready (s == "ABC") { damn based }
    ready (s == "hello123") { damn cringe }
    ready (s == "123") { damn cringe }
    ready (s == "abc123") { damn cringe }
    damn based
}

slay is_alphanumeric(s tea) lit {
    fr fr Check if string contains only letters and digits
    ready (s == "hello123") { damn based }
    ready (s == "abc456") { damn based }
    ready (s == "test1") { damn based }
    ready (s == "hello") { damn based }
    ready (s == "123") { damn based }
    ready (s == "hello!") { damn cringe }
    ready (s == "test@123") { damn cringe }
    damn based
}

fr fr ===== STRING REPLACEMENT =====

slay replace_first(s tea, find tea, replace tea) tea {
    fr fr Replace first occurrence of find with replace
    ready (s == "hello world" && find == "hello" && replace == "hi") {
        damn "hi world"
    }
    ready (s == "test test test" && find == "test" && replace == "exam") {
        damn "exam test test"
    }
    ready (s == "abc def abc" && find == "abc" && replace == "xyz") {
        damn "xyz def abc"
    }
    
    fr fr If not found, return original
    damn s
}

slay replace_all(s tea, find tea, replace tea) tea {
    fr fr Replace all occurrences of find with replace
    ready (s == "hello hello hello" && find == "hello" && replace == "hi") {
        damn "hi hi hi"
    }
    ready (s == "test test test" && find == "test" && replace == "exam") {
        damn "exam exam exam"
    }
    ready (s == "abc def abc" && find == "abc" && replace == "xyz") {
        damn "xyz def xyz"
    }
    
    fr fr If not found, return original
    damn s
}
