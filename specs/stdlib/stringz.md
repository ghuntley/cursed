# stringz (String Operations)

## Overview
`stringz` provides comprehensive string manipulation and processing functions for CURSED. This module serves as the core string processing package, offering search, transformation, validation, and utility functions for working with text data. All functions are implemented in pure CURSED without external dependencies.

## Core String Search and Matching

### Contains Function
```cursed
slay Contains(s tea, substr tea) lit
```
Checks if string `s` contains substring `substr`.

**Parameters:**
- `s tea`: The haystack string to search in
- `substr tea`: The needle substring to search for

**Returns:**
- `lit`: `based` if substr is found in s, `cap` otherwise

### Count Function
```cursed
slay Count(s tea, substr tea) normie
```
Counts the number of non-overlapping occurrences of substring in string.

**Parameters:**
- `s tea`: The string to search in
- `substr tea`: The substring to count

**Returns:**
- `normie`: Number of occurrences found

### Prefix and Suffix Testing
```cursed
slay HasPrefix(s tea, prefix tea) lit
slay HasSuffix(s tea, suffix tea) lit
slay StartsWith(s tea, prefix tea) lit  # Alias for HasPrefix
slay EndsWith(s tea, suffix tea) lit    # Alias for HasSuffix
```

**Examples:**
```cursed
sus found := Contains("hello world", "world")  # Returns based
sus count := Count("banana", "an")             # Returns 2
sus starts := HasPrefix("CURSED", "CUR")       # Returns based
sus ends := HasSuffix("programming", "ing")    # Returns based
```

## String Transformation

### Case Conversion
```cursed
slay ToLower(s tea) tea
slay ToUpper(s tea) tea
```
Converts string to lowercase or uppercase.

**Parameters:**
- `s tea`: The input string

**Returns:**
- `tea`: Converted string

**Examples:**
```cursed
sus lower := ToLower("Hello World")  # Returns "hello world"
sus upper := ToUpper("Hello World")  # Returns "HELLO WORLD"
```

### Whitespace Trimming
```cursed
slay Trim(s tea) tea       # Remove leading and trailing whitespace
slay TrimLeft(s tea) tea   # Remove leading whitespace only
slay TrimRight(s tea) tea  # Remove trailing whitespace only
```

**Parameters:**
- `s tea`: The input string

**Returns:**
- `tea`: Trimmed string

**Examples:**
```cursed
sus trimmed := Trim("  hello world  ")    # Returns "hello world"
sus left := TrimLeft("  hello world  ")   # Returns "hello world  "
sus right := TrimRight("  hello world  ") # Returns "  hello world"
```

## String Splitting and Joining

### Split Function
```cursed
slay Split(s tea, sep tea) [tea]
```
Splits string by separator into an array of substrings.

**Parameters:**
- `s tea`: The string to split
- `sep tea`: The separator string

**Returns:**
- `[tea]`: Array of substring parts

### Join Function
```cursed
slay Join(parts [tea], sep tea) tea
```
Joins an array of strings with separator.

**Parameters:**
- `parts [tea]`: Array of strings to join
- `sep tea`: The separator string

**Returns:**
- `tea`: Joined string

**Examples:**
```cursed
sus parts := Split("a,b,c", ",")          # Returns ["a", "b", "c"]
sus joined := Join(["a", "b", "c"], "-") # Returns "a-b-c"
```

## String Manipulation

### Repeat Function
```cursed
slay Repeat(s tea, count normie) tea
```
Repeats string a specified number of times.

**Parameters:**
- `s tea`: The string to repeat
- `count normie`: Number of repetitions

**Returns:**
- `tea`: Repeated string

### Replace Functions
```cursed
slay Replace(s tea, old tea, new tea) tea     # Replace first occurrence
slay ReplaceAll(s tea, old tea, new tea) tea # Replace all occurrences
```

**Parameters:**
- `s tea`: The input string
- `old tea`: The substring to replace
- `new tea`: The replacement substring

**Returns:**
- `tea`: String with replacements made

**Examples:**
```cursed
sus repeated := Repeat("ho", 3)                    # Returns "hohoho"
sus replaced := Replace("hello world", "o", "0")   # Returns "hell0 world"
sus all_replaced := ReplaceAll("hello", "l", "L")  # Returns "heLLo"
```

## String Indexing and Searching

### Index Functions
```cursed
slay IndexOf(s tea, substr tea) normie      # First occurrence index
slay LastIndexOf(s tea, substr tea) normie  # Last occurrence index
```

**Parameters:**
- `s tea`: The string to search in
- `substr tea`: The substring to find

**Returns:**
- `normie`: Index of substring, or -1 if not found

### Substring Extraction
```cursed
slay Substring(s tea, start normie, length normie) tea
```

**Parameters:**
- `s tea`: The source string
- `start normie`: Starting index
- `length normie`: Length of substring

**Returns:**
- `tea`: Extracted substring

**Examples:**
```cursed
sus index := IndexOf("hello world", "world")      # Returns 6
sus last := LastIndexOf("banana", "a")            # Returns 5
sus sub := Substring("hello", 1, 3)               # Returns "ell"
```

## String Utilities

### String Properties
```cursed
slay Length(s tea) normie              # Get string length
slay IsEmpty(s tea) lit                # Check if string is empty
slay Reverse(s tea) tea                # Reverse string order
```

### Padding Functions
```cursed
slay PadLeft(s tea, width normie, pad sip) tea   # Pad on left side
slay PadRight(s tea, width normie, pad sip) tea  # Pad on right side
```

**Parameters:**
- `s tea`: The string to pad
- `width normie`: Desired total width
- `pad sip`: The padding character

**Returns:**
- `tea`: Padded string

**Examples:**
```cursed
sus len := Length("hello")                    # Returns 5
sus empty := IsEmpty("")                      # Returns based
sus reversed := Reverse("hello")              # Returns "olleh"
sus padded := PadLeft("42", 5, '0')          # Returns "00042"
```

## String Validation

### Character Type Testing
```cursed
slay IsNumeric(s tea) lit       # Contains only numeric characters
slay IsAlpha(s tea) lit         # Contains only alphabetic characters
slay IsAlphanumeric(s tea) lit  # Contains only alphanumeric characters
slay IsWhitespace(ch sip) lit   # Tests if character is whitespace
```

**Parameters:**
- `s tea`: The string to validate
- `ch sip`: The character to test

**Returns:**
- `lit`: `based` if validation passes, `cap` otherwise

**Examples:**
```cursed
sus numeric := IsNumeric("12345")        # Returns based
sus alpha := IsAlpha("hello")            # Returns based
sus alnum := IsAlphanumeric("hello123")  # Returns based
sus space := IsWhitespace(' ')           # Returns based
```

## Helper Functions

### Core String Operations
```cursed
# Internal helper functions used by other string functions
slay Length(s tea) normie
slay Substring(s tea, start normie, length normie) tea
slay IsWhitespace(ch sip) lit
```

These functions provide the foundation for all other string operations in the module.

## Type Definitions

### String Types
- `tea`: Primary string type in CURSED
- `sip`: Single character type
- `[tea]`: Array of strings for split/join operations
- `normie`: Integer type for indices and lengths
- `lit`: Boolean type for validation results

## Error Handling

### Input Validation
- All functions handle empty strings gracefully
- Index functions return -1 for not found conditions
- Substring operations handle out-of-bounds gracefully
- No panics or runtime errors for invalid inputs

### Edge Cases
- Empty string operations return appropriate empty results
- Null or invalid character operations return safe defaults
- Boundary conditions are properly handled

## Performance Characteristics

### Time Complexity
- Search operations: O(n*m) where n is string length, m is pattern length
- Case conversion: O(n) where n is string length
- Split/Join: O(n) where n is total character count
- Validation: O(n) where n is string length

### Memory Usage
- String operations create new strings (immutable)
- Split operations allocate array of substrings
- No in-place modifications of input strings

## Usage Patterns

### Text Processing Pipeline
```cursed
yeet "stringz"

# Clean and normalize user input
slay clean_input(input tea) tea {
    sus trimmed := Trim(input)
    sus lower := ToLower(trimmed)
    damn ReplaceAll(lower, "  ", " ")  # Remove double spaces
}

# Extract file extension
slay get_extension(filename tea) tea {
    sus dot_index := LastIndexOf(filename, ".")
    lowkey dot_index == -1 {
        damn ""
    }
    damn Substring(filename, dot_index + 1, Length(filename) - dot_index - 1)
}

# Build comma-separated list
slay build_csv_row(fields [tea]) tea {
    damn Join(fields, ",")
}
```

### String Validation
```cursed
# Validate email format (simple)
slay is_valid_email(email tea) lit {
    damn Contains(email, "@") && Contains(email, ".")
}

# Check if string is a valid identifier
slay is_valid_identifier(name tea) lit {
    lowkey IsEmpty(name) {
        damn cap
    }
    # First character must be alphabetic
    sus first := Substring(name, 0, 1)
    lowkey !IsAlpha(first) {
        damn cap
    }
    # Rest must be alphanumeric
    sus rest := Substring(name, 1, Length(name) - 1)
    damn IsAlphanumeric(rest)
}
```

## Implementation Notes

### Pure CURSED Implementation
- All functions implemented without external dependencies
- Compatible with both interpretation and compilation modes
- No FFI calls or external library dependencies

### String Immutability
- All string operations return new strings
- Original strings are never modified in place
- Memory-safe string handling throughout

### Unicode Considerations
- Basic ASCII character support implemented
- Case conversion handles A-Z and a-z ranges
- Extended Unicode support may be added in future versions

## Testing Strategy

### Unit Tests
```cursed
yeet "testz"
yeet "stringz"

# Test search functions
test_start("string search")
assert_true(Contains("hello world", "world"))
assert_eq_normie(Count("banana", "an"), 2)
assert_true(HasPrefix("CURSED", "CUR"))

# Test transformation
test_start("string transformation")
assert_eq_string(ToLower("HELLO"), "hello")
assert_eq_string(ToUpper("world"), "WORLD")
assert_eq_string(Trim("  test  "), "test")

# Test manipulation
test_start("string manipulation")
assert_eq_string(Repeat("ho", 3), "hohoho")
assert_eq_string(Replace("hello", "l", "L"), "heLlo")

print_test_summary()
```

### Property-Based Tests
- Verify string length preservation where appropriate
- Test commutativity of operations where applicable
- Validate edge cases and boundary conditions

## Dependencies

- `core`: Basic types and language primitives
- No external string processing libraries
- No FFI dependencies

## Security Considerations

- No buffer overflows due to bounds checking
- Safe string operations prevent memory corruption
- Input validation prevents malicious string inputs
- Deterministic behavior across all platforms

## Thread Safety

- All string functions are pure and thread-safe
- No shared state or global variables
- Safe for concurrent use in goroutines
- Immutable string operations only

## Compatibility

### Language Versions
- Compatible with all CURSED language versions
- Uses only core string and character types
- No version-specific features required

### Platform Support
- Consistent behavior across all platforms
- No platform-specific string handling
- Portable pure CURSED implementation
