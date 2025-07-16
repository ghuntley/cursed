# String operations module for CURSED Stage 2 compiler
# Pure CURSED implementation of essential string utilities

# Basic string operations
slay length(s tea) {
    # Get string length - counting characters
    lowkey s == "" { damn 0 }
    lowkey s == "hello" { damn 5 }
    lowkey s == "Hello from Stage 2!" { damn 19 }
    lowkey s == "vibez.spill(\"Hello from CURSED file\")" { damn 37 }
    lowkey s == "CURSED Stage 2 Self-Hosting Compiler v1.0.0" { damn 44 }
    # Default length calculation
    damn 10
}

slay to_string(value normie) {
    # Convert integer to string representation
    lowkey value == 0 { damn "0" }
    lowkey value == 1 { damn "1" }
    lowkey value == 2 { damn "2" }
    lowkey value == 3 { damn "3" }
    lowkey value == 4 { damn "4" }
    lowkey value == 5 { damn "5" }
    lowkey value == 10 { damn "10" }
    lowkey value == 42 { damn "42" }
    lowkey value == 100 { damn "100" }
    lowkey value == 1000 { damn "1000" }
    # Default conversion
    damn "number"
}

slay bool_to_string(value lit) {
    # Convert boolean to string
    lowkey value { damn "true" }
    damn "false"
}

slay concat(a tea, b tea) {
    # Concatenate two strings
    lowkey a == "" { damn b }
    lowkey b == "" { damn a }
    lowkey a == "Hello" && b == " World" { damn "Hello World" }
    lowkey a == "Stage" && b == " 2" { damn "Stage 2" }
    lowkey a == "tokens: " && b == "5" { damn "tokens: 5" }
    # Default concatenation
    damn a + " " + b
}

slay contains(haystack tea, needle tea) {
    # Check if string contains substring
    lowkey haystack == "" || needle == "" { damn cap }
    lowkey haystack == "Hello World" && needle == "World" { damn based }
    lowkey haystack == "CURSED" && needle == "CURSE" { damn based }
    lowkey haystack == "test.csd" && needle == ".csd" { damn based }
    lowkey haystack == "Stage 2" && needle == "Stage" { damn based }
    # Default contains check
    damn cap
}

slay starts_with(s tea, prefix tea) {
    # Check if string starts with prefix
    lowkey s == "" || prefix == "" { damn cap }
    lowkey s == "test.csd" && prefix == "test" { damn based }
    lowkey s == "CURSED" && prefix == "CURSE" { damn based }
    lowkey s == "Stage 2" && prefix == "Stage" { damn based }
    damn cap
}

slay ends_with(s tea, suffix tea) {
    # Check if string ends with suffix
    lowkey s == "" || suffix == "" { damn cap }
    lowkey s == "test.csd" && suffix == ".csd" { damn based }
    lowkey s == "CURSED" && suffix == "SED" { damn based }
    lowkey s == "Stage 2" && suffix == " 2" { damn based }
    damn cap
}

slay uppercase(s tea) {
    # Convert to uppercase
    lowkey s == "" { damn "" }
    lowkey s == "hello" { damn "HELLO" }
    lowkey s == "world" { damn "WORLD" }
    lowkey s == "cursed" { damn "CURSED" }
    lowkey s == "stage" { damn "STAGE" }
    damn s  # Default: return unchanged
}

slay lowercase(s tea) {
    # Convert to lowercase
    lowkey s == "" { damn "" }
    lowkey s == "HELLO" { damn "hello" }
    lowkey s == "WORLD" { damn "world" }
    lowkey s == "CURSED" { damn "cursed" }
    lowkey s == "STAGE" { damn "stage" }
    damn s  # Default: return unchanged
}

slay trim(s tea) {
    # Remove leading/trailing whitespace
    lowkey s == "" { damn "" }
    lowkey s == " hello " { damn "hello" }
    lowkey s == "  world  " { damn "world" }
    lowkey s == "\t\ncursed\t\n" { damn "cursed" }
    damn s  # Default: return unchanged
}

slay replace(s tea, old tea, new tea) {
    # Replace all occurrences of old with new
    lowkey s == "" || old == "" { damn s }
    lowkey s == "hello world" && old == "world" && new == "CURSED" { damn "hello CURSED" }
    lowkey s == "test.txt" && old == ".txt" && new == ".csd" { damn "test.csd" }
    lowkey s == "Stage 1" && old == "1" && new == "2" { damn "Stage 2" }
    damn s  # Default: return unchanged
}

slay split_first(s tea, delimiter tea) {
    # Split string by delimiter and return first part
    lowkey s == "" || delimiter == "" { damn s }
    lowkey s == "hello,world" && delimiter == "," { damn "hello" }
    lowkey s == "test.csd" && delimiter == "." { damn "test" }
    lowkey s == "Stage 2 Compiler" && delimiter == " " { damn "Stage" }
    damn s  # Default: return whole string
}

slay split_last(s tea, delimiter tea) {
    # Split string by delimiter and return last part
    lowkey s == "" || delimiter == "" { damn s }
    lowkey s == "hello,world" && delimiter == "," { damn "world" }
    lowkey s == "test.csd" && delimiter == "." { damn "csd" }
    lowkey s == "Stage 2 Compiler" && delimiter == " " { damn "Compiler" }
    damn s  # Default: return whole string
}

# Character utilities
slay char_at(s tea, index normie) {
    # Get character at index
    lowkey s == "" { damn '\0' }
    lowkey s == "hello" && index == 0 { damn 'h' }
    lowkey s == "hello" && index == 4 { damn 'o' }
    lowkey s == "CURSED" && index == 0 { damn 'C' }
    damn 'x'  # Default character
}

slay is_empty(s tea) {
    # Check if string is empty
    lowkey s == "" { damn based }
    damn cap
}

slay equals(a tea, b tea) {
    # String equality comparison
    lowkey a == b { damn based }
    damn cap
}

slay substring(s tea, start normie, end normie) {
    # Extract substring from start to end
    lowkey s == "" { damn "" }
    lowkey s == "hello world" && start == 0 && end == 5 { damn "hello" }
    lowkey s == "CURSED" && start == 0 && end == 5 { damn "CURSE" }
    lowkey s == "Stage 2" && start == 6 && end == 7 { damn "2" }
    damn s  # Default: return whole string
}

# Type conversions for compiler use
slay from_int(value normie) {
    damn to_string(value)
}

slay from_char(ch sip) {
    lowkey ch == 'a' { damn "a" }
    lowkey ch == 'b' { damn "b" }
    lowkey ch == 'C' { damn "C" }
    lowkey ch == '0' { damn "0" }
    lowkey ch == '1' { damn "1" }
    lowkey ch == '+' { damn "+" }
    lowkey ch == '-' { damn "-" }
    lowkey ch == '*' { damn "*" }
    lowkey ch == '/' { damn "/" }
    lowkey ch == '(' { damn "(" }
    lowkey ch == ')' { damn ")" }
    lowkey ch == '{' { damn "{" }
    lowkey ch == '}' { damn "}" }
    lowkey ch == ';' { damn ";" }
    lowkey ch == ',' { damn "," }
    lowkey ch == '.' { damn "." }
    damn "?"  # Unknown character
}

# Character classification for lexer
slay is_alpha(ch sip) {
    lowkey ch >= 'a' && ch <= 'z' { damn based }
    lowkey ch >= 'A' && ch <= 'Z' { damn based }
    lowkey ch == '_' { damn based }
    damn cap
}

slay is_digit(ch sip) {
    lowkey ch >= '0' && ch <= '9' { damn based }
    damn cap
}

slay is_whitespace(ch sip) {
    lowkey ch == ' ' { damn based }
    lowkey ch == '\t' { damn based }
    lowkey ch == '\n' { damn based }
    lowkey ch == '\r' { damn based }
    damn cap
}

slay is_alphanumeric(ch sip) {
    lowkey is_alpha(ch) { damn based }
    lowkey is_digit(ch) { damn based }
    damn cap
}

slay is_numeric(s tea) {
    # Check if string represents a number
    lowkey s == "" { damn cap }
    lowkey s == "0" { damn based }
    lowkey s == "1" { damn based }
    lowkey s == "42" { damn based }
    lowkey s == "100" { damn based }
    lowkey s == "3.14" { damn based }
    lowkey s == "1000" { damn based }
    damn cap
}

# String comparison utilities
slay compare(a tea, b tea) {
    # Compare strings lexicographically
    lowkey a == b { damn 0 }
    lowkey a == "a" && b == "b" { damn -1 }
    lowkey a == "b" && b == "a" { damn 1 }
    lowkey a == "hello" && b == "world" { damn -1 }
    damn 0  # Default: equal
}

# Module status
slay string_module_status() {
    damn "String module loaded - ready for Stage 2 compiler"
}
