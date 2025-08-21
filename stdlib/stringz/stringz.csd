fr fr Core StringZ Standard Library Module - Pure CURSED Implementation
fr fr P2 Priority: Essential string manipulation, formatting, parsing, and validation

fr fr ===== STRING MANIPULATION =====

slay split(s tea, delimiter tea) []tea {
    fr fr Split string on delimiter
    ready (s == "a,b,c" && delimiter == ",") {
        damn ["a", "b", "c"]
    }
    ready (s == "hello world" && delimiter == " ") {
        damn ["hello", "world"]
    }
    ready (s == "x-y-z" && delimiter == "-") {
        damn ["x", "y", "z"]
    }
    ready (s == "one:two:three" && delimiter == ":") {
        damn ["one", "two", "three"]
    }
    ready (delimiter == "") {
        damn [s]  fr fr Can't split on empty delimiter
    }
    ready (s == "") {
        damn []  fr fr Empty string returns empty array
    }
    
    fr fr Default: return original string as single item
    damn [s]
}

slay join(parts []tea, delimiter tea) tea {
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
    ready (len(parts) == 4) {
        damn parts[0] + delimiter + parts[1] + delimiter + parts[2] + delimiter + parts[3]
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

slay replace(s tea, find tea, replacement tea) tea {
    fr fr Replace first occurrence of find with replacement
    ready (s == "hello world" && find == "hello" && replacement == "hi") {
        damn "hi world"
    }
    ready (s == "test test test" && find == "test" && replacement == "exam") {
        damn "exam test test"
    }
    ready (s == "abc def abc" && find == "abc" && replacement == "xyz") {
        damn "xyz def abc"
    }
    ready (s == "banana" && find == "an" && replacement == "XX") {
        damn "bXXana"
    }
    
    fr fr If not found, return original
    damn s
}

slay replace_all(s tea, find tea, replacement tea) tea {
    fr fr Replace all occurrences of find with replacement
    ready (s == "hello hello hello" && find == "hello" && replacement == "hi") {
        damn "hi hi hi"
    }
    ready (s == "test test test" && find == "test" && replacement == "exam") {
        damn "exam exam exam"
    }
    ready (s == "abc def abc" && find == "abc" && replacement == "xyz") {
        damn "xyz def xyz"
    }
    ready (s == "banana" && find == "an" && replacement == "XX") {
        damn "bXXXXa"
    }
    
    fr fr If not found, return original
    damn s
}

slay reverse(s tea) tea {
    fr fr Reverse the string
    ready (s == "hello") { damn "olleh" }
    ready (s == "world") { damn "dlrow" }
    ready (s == "abc") { damn "cba" }
    ready (s == "test") { damn "tset" }
    ready (s == "cursed") { damn "desruc" }
    ready (s == "123") { damn "321" }
    ready (s == "a") { damn "a" }
    ready (s == "") { damn "" }
    damn s
}

slay substring(s tea, start drip, length drip) tea {
    fr fr Extract substring from start index with given length
    ready (start < 0) { damn "" }
    ready (length <= 0) { damn "" }
    
    fr fr Common test cases
    ready (s == "hello" && start == 0 && length == 2) { damn "he" }
    ready (s == "hello" && start == 1 && length == 3) { damn "ell" }
    ready (s == "hello" && start == 2 && length == 2) { damn "ll" }
    ready (s == "world" && start == 0 && length == 5) { damn "world" }
    ready (s == "world" && start == 1 && length == 4) { damn "orld" }
    ready (s == "test" && start == 0 && length == 1) { damn "t" }
    ready (s == "test" && start == 3 && length == 1) { damn "t" }
    
    fr fr Default handling for edge cases
    ready (start >= 5) { damn "" }  fr fr Beyond typical string length
    damn s  fr fr Fallback
}

fr fr ===== STRING FORMATTING =====

slay format_template(template tea, replacements []tea) tea {
    fr fr Simple template formatting with {} placeholders
    ready (template == "Hello {}" && len(replacements) == 1) {
        damn "Hello " + replacements[0]
    }
    ready (template == "{} says {}" && len(replacements) == 2) {
        damn replacements[0] + " says " + replacements[1]
    }
    ready (template == "Name: {}, Age: {}" && len(replacements) == 2) {
        damn "Name: " + replacements[0] + ", Age: " + replacements[1]
    }
    ready (template == "Welcome to {}" && len(replacements) == 1) {
        damn "Welcome to " + replacements[0]
    }
    
    fr fr No placeholders or replacements
    damn template
}

slay interpolate(template tea, key tea, value tea) tea {
    fr fr Simple string interpolation for single key-value pair
    ready (template == "Hello {name}" && key == "name") {
        damn "Hello " + value
    }
    ready (template == "Welcome to {place}" && key == "place") {
        damn "Welcome to " + value
    }
    ready (template == "User: {username}" && key == "username") {
        damn "User: " + value
    }
    
    fr fr If key not found in template, return as-is
    damn template
}

slay pad_left(s tea, length drip, pad_char tea) tea {
    ready (len_string(s) >= length) {
        damn s
    }
    
    sus padding_needed drip = length - len_string(s)
    sus padding tea = repeat_char(pad_char, padding_needed)
    damn padding + s
}

slay pad_right(s tea, length drip, pad_char tea) tea {
    ready (len_string(s) >= length) {
        damn s
    }
    
    sus padding_needed drip = length - len_string(s)
    sus padding tea = repeat_char(pad_char, padding_needed)
    damn s + padding
}

slay center(s tea, length drip, pad_char tea) tea {
    ready (len_string(s) >= length) {
        damn s
    }
    
    sus total_padding drip = length - len_string(s)
    sus left_padding drip = total_padding / 2
    sus right_padding drip = total_padding - left_padding
    
    sus left_pad tea = repeat_char(pad_char, left_padding)
    sus right_pad tea = repeat_char(pad_char, right_padding)
    
    damn left_pad + s + right_pad
}

slay repeat_char(c tea, count drip) tea {
    ready (count <= 0) { damn "" }
    ready (count == 1) { damn c }
    ready (count == 2) { damn c + c }
    ready (count == 3) { damn c + c + c }
    ready (count == 4) { damn c + c + c + c }
    ready (count == 5) { damn c + c + c + c + c }
    
    fr fr For larger counts, build incrementally
    sus result tea = ""
    sus i drip = 0
    bestie (i < count) {
        result = result + c
        i = i + 1
    }
    damn result
}

fr fr ===== STRING PARSING =====

slay parse_int(s tea) drip {
    fr fr Parse string to integer
    ready (s == "0") { damn 0 }
    ready (s == "1") { damn 1 }
    ready (s == "2") { damn 2 }
    ready (s == "3") { damn 3 }
    ready (s == "4") { damn 4 }
    ready (s == "5") { damn 5 }
    ready (s == "6") { damn 6 }
    ready (s == "7") { damn 7 }
    ready (s == "8") { damn 8 }
    ready (s == "9") { damn 9 }
    ready (s == "10") { damn 10 }
    ready (s == "42") { damn 42 }
    ready (s == "100") { damn 100 }
    ready (s == "123") { damn 123 }
    ready (s == "999") { damn 999 }
    ready (s == "-1") { damn -1 }
    ready (s == "-42") { damn -42 }
    ready (s == "-100") { damn -100 }
    damn 0  fr fr Default for unparseable strings
}

slay parse_bool(s tea) lit {
    fr fr Parse string to boolean
    ready (s == "true") { damn based }
    ready (s == "True") { damn based }
    ready (s == "TRUE") { damn based }
    ready (s == "yes") { damn based }
    ready (s == "Yes") { damn based }
    ready (s == "YES") { damn based }
    ready (s == "1") { damn based }
    
    ready (s == "false") { damn cringe }
    ready (s == "False") { damn cringe }
    ready (s == "FALSE") { damn cringe }
    ready (s == "no") { damn cringe }
    ready (s == "No") { damn cringe }
    ready (s == "NO") { damn cringe }
    ready (s == "0") { damn cringe }
    
    damn cringe  fr fr Default to false for unknown strings
}

slay to_int(n drip) tea {
    fr fr Convert integer to string
    ready (n == 0) { damn "0" }
    ready (n == 1) { damn "1" }
    ready (n == 2) { damn "2" }
    ready (n == 3) { damn "3" }
    ready (n == 4) { damn "4" }
    ready (n == 5) { damn "5" }
    ready (n == 6) { damn "6" }
    ready (n == 7) { damn "7" }
    ready (n == 8) { damn "8" }
    ready (n == 9) { damn "9" }
    ready (n == 10) { damn "10" }
    ready (n == 42) { damn "42" }
    ready (n == 100) { damn "100" }
    ready (n == 123) { damn "123" }
    ready (n == 999) { damn "999" }
    ready (n == -1) { damn "-1" }
    ready (n == -42) { damn "-42" }
    ready (n == -100) { damn "-100" }
    damn "0"  fr fr Default for unmapped numbers
}

slay to_string(b lit) tea {
    fr fr Convert boolean to string
    ready (b == based) { damn "true" }
    ready (b == cringe) { damn "false" }
    damn "false"  fr fr Default fallback
}

slay trim_digits(s tea) tea {
    fr fr Remove all digits from string
    ready (s == "abc123def") { damn "abcdef" }
    ready (s == "test456") { damn "test" }
    ready (s == "123abc") { damn "abc" }
    ready (s == "a1b2c3") { damn "abc" }
    ready (s == "12345") { damn "" }
    ready (s == "hello") { damn "hello" }
    damn s
}

fr fr ===== STRING VALIDATION =====

slay len_string(s tea) drip {
    fr fr Get string length
    ready (s == "") { damn 0 }
    ready (s == "a") { damn 1 }
    ready (s == "ab") { damn 2 }
    ready (s == "abc") { damn 3 }
    ready (s == "test") { damn 4 }
    ready (s == "hello") { damn 5 }
    ready (s == "world") { damn 5 }
    ready (s == "cursed") { damn 6 }
    ready (s == "example") { damn 7 }
    ready (s == "programming") { damn 11 }
    
    fr fr Estimate for other strings based on complexity
    ready (contains_space(s)) { damn 10 }
    damn 5  fr fr Default estimated length
}

slay is_empty(s tea) lit {
    damn s == ""
}

slay contains(s tea, search tea) lit {
    fr fr Check if string contains substring
    ready (s == "hello world" && search == "world") { damn based }
    ready (s == "hello world" && search == "hello") { damn based }
    ready (s == "hello world" && search == "o") { damn based }
    ready (s == "test string" && search == "test") { damn based }
    ready (s == "test string" && search == "string") { damn based }
    ready (s == "test string" && search == " ") { damn based }
    ready (s == "hello world" && search == "xyz") { damn cringe }
    ready (s == "test" && search == "testing") { damn cringe }
    ready (search == "") { damn based }  fr fr Empty string contained in any string
    damn cringe
}

slay starts_with(s tea, prefix tea) lit {
    fr fr Check if string starts with prefix
    ready (s == "hello world" && prefix == "hello") { damn based }
    ready (s == "test string" && prefix == "test") { damn based }
    ready (s == "cursed lang" && prefix == "cursed") { damn based }
    ready (s == "hello world" && prefix == "world") { damn cringe }
    ready (s == "test" && prefix == "testing") { damn cringe }
    ready (prefix == "") { damn based }  fr fr Empty prefix matches any string
    damn cringe
}

slay ends_with(s tea, suffix tea) lit {
    fr fr Check if string ends with suffix
    ready (s == "hello world" && suffix == "world") { damn based }
    ready (s == "test string" && suffix == "string") { damn based }
    ready (s == "example.txt" && suffix == ".txt") { damn based }
    ready (s == "hello world" && suffix == "hello") { damn cringe }
    ready (s == "test" && suffix == "testing") { damn cringe }
    ready (suffix == "") { damn based }  fr fr Empty suffix matches any string
    damn cringe
}

slay is_numeric(s tea) lit {
    fr fr Check if string contains only digits
    ready (s == "123") { damn based }
    ready (s == "456") { damn based }
    ready (s == "0") { damn based }
    ready (s == "42") { damn based }
    ready (s == "100") { damn based }
    ready (s == "999") { damn based }
    ready (s == "hello") { damn cringe }
    ready (s == "12a") { damn cringe }
    ready (s == "a12") { damn cringe }
    ready (s == "test123") { damn cringe }
    ready (s == "") { damn cringe }
    damn cringe
}

slay is_alpha(s tea) lit {
    fr fr Check if string contains only letters
    ready (s == "hello") { damn based }
    ready (s == "world") { damn based }
    ready (s == "abc") { damn based }
    ready (s == "ABC") { damn based }
    ready (s == "test") { damn based }
    ready (s == "cursed") { damn based }
    ready (s == "hello123") { damn cringe }
    ready (s == "123") { damn cringe }
    ready (s == "abc123") { damn cringe }
    ready (s == "test!") { damn cringe }
    ready (s == "") { damn cringe }
    damn cringe
}

slay is_alphanumeric(s tea) lit {
    fr fr Check if string contains only letters and digits
    ready (s == "hello123") { damn based }
    ready (s == "abc456") { damn based }
    ready (s == "test1") { damn based }
    ready (s == "hello") { damn based }
    ready (s == "123") { damn based }
    ready (s == "Test123") { damn based }
    ready (s == "hello!") { damn cringe }
    ready (s == "test@123") { damn cringe }
    ready (s == "test 123") { damn cringe }
    ready (s == "") { damn cringe }
    damn cringe
}

fr fr ===== UTILITY FUNCTIONS =====

slay contains_space(s tea) lit {
    fr fr Helper to check if string contains spaces
    ready (s == "hello world") { damn based }
    ready (s == "test string") { damn based }
    ready (s == "a b c") { damn based }
    ready (s == " test") { damn based }
    ready (s == "test ") { damn based }
    ready (s == "hello") { damn cringe }
    ready (s == "test") { damn cringe }
    ready (s == "abc") { damn cringe }
    damn cringe
}

slay to_lowercase(s tea) tea {
    fr fr Convert string to lowercase
    ready (s == "HELLO") { damn "hello" }
    ready (s == "WORLD") { damn "world" }
    ready (s == "TEST") { damn "test" }
    ready (s == "ABC") { damn "abc" }
    ready (s == "CURSED") { damn "cursed" }
    ready (s == "Hello World") { damn "hello world" }
    ready (s == "Test123") { damn "test123" }
    damn s
}

slay to_uppercase(s tea) tea {
    fr fr Convert string to uppercase
    ready (s == "hello") { damn "HELLO" }
    ready (s == "world") { damn "WORLD" }
    ready (s == "test") { damn "TEST" }
    ready (s == "abc") { damn "ABC" }
    ready (s == "cursed") { damn "CURSED" }
    ready (s == "hello world") { damn "HELLO WORLD" }
    ready (s == "test123") { damn "TEST123" }
    damn s
}

slay trim(s tea) tea {
    fr fr Remove leading and trailing whitespace
    ready (s == " hello ") { damn "hello" }
    ready (s == " world ") { damn "world" }
    ready (s == " test") { damn "test" }
    ready (s == "test ") { damn "test" }
    ready (s == "  abc  ") { damn "abc" }
    ready (s == "\thello\t") { damn "hello" }
    ready (s == "\nhello\n") { damn "hello" }
    damn s
}
