// Simple test using basic string functions directly
vibez.spill("Testing basic string operations...")

// Test basic string operations
sus text tea = "hello world"
vibez.spill("Original text: " + text)

// Test length
sus len normie = string_len(text)
vibez.spill("Length: " + tea(len))

// Test contains
if string_contains(text, "world") {
    vibez.spill("✓ Contains 'world'")
} else {
    vibez.spill("✗ Does not contain 'world'")
}

// Test case conversion
sus upper tea = string_to_upper(text)
sus lower tea = string_to_lower(text)
vibez.spill("Upper: " + upper)
vibez.spill("Lower: " + lower)

// Test trim
sus padded tea = "  hello  "
sus trimmed tea = string_trim(padded)
vibez.spill("Trimmed: '" + trimmed + "'")

// Test starts with
if string_starts_with(text, "hello") {
    vibez.spill("✓ Starts with 'hello'")
} else {
    vibez.spill("✗ Does not start with 'hello'")
}

// Test ends with
if string_ends_with(text, "world") {
    vibez.spill("✓ Ends with 'world'")
} else {
    vibez.spill("✗ Does not end with 'world'")
}

// Test substring
sus sub tea = string_substring(text, 0, 5)
vibez.spill("Substring (0, 5): " + sub)

// Test character at
sus char tea = string_char_at(text, 0)
vibez.spill("Character at 0: " + char)

// Test reverse
sus reversed tea = string_reverse(text)
vibez.spill("Reversed: " + reversed)

// Test repeat
sus repeated tea = string_repeat("ha", 3)
vibez.spill("Repeated: " + repeated)

// Test split
sus parts [tea] = string_split(text, " ")
vibez.spill("Split parts[0]: " + parts[0])
vibez.spill("Split parts[1]: " + parts[1])

// Test join
sus joined tea = string_join(parts, "-")
vibez.spill("Joined: " + joined)

// Test replace
sus replaced tea = string_replace(text, "world", "CURSED")
vibez.spill("Replaced: " + replaced)

vibez.spill("All basic string tests completed!")
