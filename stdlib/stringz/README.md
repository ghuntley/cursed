# Stringz Module (strings)

The `stringz` module provides string manipulation functions for the CURSED programming language, equivalent to Go's `strings` package.

## Core Specification Functions

### String Searching and Testing

- `Contains(s tea, substr tea) lit` - Check if string contains substring
- `HasPrefix(s tea, prefix tea) lit` - Check if string starts with prefix
- `HasSuffix(s tea, suffix tea) lit` - Check if string ends with suffix
- `Count(s tea, substr tea) normie` - Count occurrences of substring in string

### String Splitting and Joining

- `Split(s tea, sep tea) []tea` - Split string by separator
- `Join(elems []tea, sep tea) tea` - Join array of strings with separator
- `SplitLines(s tea) []tea` - Split string by newlines
- `SplitWhitespace(s tea) []tea` - Split string by whitespace

### String Case Conversion

- `ToLower(s tea) tea` - Convert string to lowercase
- `ToUpper(s tea) tea` - Convert string to uppercase
- `Capitalize(s tea) tea` - Capitalize first letter

### String Trimming

- `Trim(s tea, cutset tea) tea` - Trim characters from both ends
- `TrimLeft(s tea, cutset tea) tea` - Trim characters from left end
- `TrimRight(s tea, cutset tea) tea` - Trim characters from right end

## Advanced String Functions

### String Searching

- `IndexOf(s tea, substr tea) normie` - Find first occurrence of substring
- `LastIndexOf(s tea, substr tea) normie` - Find last occurrence of substring

### String Replacement

- `Replace(s tea, old tea, new tea, n normie) tea` - Replace first n occurrences
- `ReplaceAll(s tea, old tea, new tea) tea` - Replace all occurrences

### String Utilities

- `Len(s tea) normie` - Get string length
- `Repeat(s tea, count normie) tea` - Repeat string count times
- `Reverse(s tea) tea` - Reverse string
- `Substring(s tea, start normie, length normie) tea` - Extract substring
- `Slice(s tea, start normie, end normie) tea` - Extract slice
- `CharAt(s tea, index normie) tea` - Get character at index

## String Validation

### Character Type Checking

- `IsEmpty(s tea) lit` - Check if string is empty
- `IsNumeric(s tea) lit` - Check if string contains only numeric characters
- `IsAlpha(s tea) lit` - Check if string contains only alphabetic characters
- `IsAlphanumeric(s tea) lit` - Check if string contains only alphanumeric characters
- `IsWhitespace(s tea) lit` - Check if string contains only whitespace
- `IsAscii(s tea) lit` - Check if string contains only ASCII characters

## String Comparison

- `Compare(s1 tea, s2 tea) normie` - Compare strings lexicographically
- `Equals(s1 tea, s2 tea) lit` - Check if strings are equal
- `EqualsIgnoreCase(s1 tea, s2 tea) lit` - Check if strings are equal ignoring case

## String Conversion

### To/From Other Types

- `ToInt(s tea) normie` - Convert string to integer
- `ToFloat(s tea) meal` - Convert string to float
- `ToBool(s tea) lit` - Convert string to boolean
- `FromInt(i normie) tea` - Convert integer to string
- `FromFloat(f meal) tea` - Convert float to string
- `FromBool(b lit) tea` - Convert boolean to string

### Encoding/Decoding

- `ToBytes(s tea) []byte` - Convert string to byte array
- `FromBytes(bytes []byte) tea` - Convert byte array to string
- `Escape(s tea) tea` - Escape special characters
- `Unescape(s tea) tea` - Unescape special characters

## String Padding

- `PadLeft(s tea, length normie, pad_char tea) tea` - Pad string on left
- `PadRight(s tea, length normie, pad_char tea) tea` - Pad string on right
- `PadCenter(s tea, length normie, pad_char tea) tea` - Pad string in center

## Advanced Operations

### String Analysis

- `Hash(s tea) normie` - Calculate hash of string
- `LevenshteinDistance(s1 tea, s2 tea) normie` - Calculate edit distance
- `Similarity(s1 tea, s2 tea) meal` - Calculate similarity (0.0 to 1.0)

### String Formatting

- `Format(template tea, args []tea) tea` - Format string with arguments

## Usage Examples

```cursed
yeet "stringz"

slay main() {
    // Core specification functions
    sus text tea = "Hello, World!"
    
    // String searching
    sus contains_world lit = stringz.Contains(text, "World")
    sus starts_with_hello lit = stringz.HasPrefix(text, "Hello")
    sus ends_with_exclamation lit = stringz.HasSuffix(text, "!")
    
    // String splitting and joining
    sus words []tea = stringz.Split("apple,banana,cherry", ",")
    sus joined tea = stringz.Join(words, " | ")
    
    // Case conversion
    sus upper tea = stringz.ToUpper(text)
    sus lower tea = stringz.ToLower(text)
    
    // String trimming
    sus trimmed tea = stringz.Trim("  hello world  ", " ")
    
    // String validation
    sus is_numeric lit = stringz.IsNumeric("12345")
    sus is_alpha lit = stringz.IsAlpha("hello")
    sus is_empty lit = stringz.IsEmpty("")
    
    // String replacement
    sus replaced tea = stringz.ReplaceAll("hello hello", "hello", "hi")
    
    // String utilities
    sus length normie = stringz.Len(text)
    sus repeated tea = stringz.Repeat("abc", 3)
    sus reversed tea = stringz.Reverse("hello")
    
    // String conversion
    sus number normie = stringz.ToInt("123")
    sus float_val meal = stringz.ToFloat("3.14")
    sus bool_val lit = stringz.ToBool("true")
    
    // String padding
    sus padded tea = stringz.PadLeft("test", 10, "0")
    
    // String comparison
    sus comparison normie = stringz.Compare("apple", "banana")
    sus equals lit = stringz.Equals("hello", "hello")
}
```

## Testing

Run the test suite with:

```bash
cargo run --bin cursed stdlib/stringz/test_stringz_complete.csd
```

## Implementation Details

- Pure CURSED implementation without FFI dependencies
- Supports both interpretation and compilation modes
- Includes comprehensive string manipulation functions
- Provides efficient string searching and replacement
- Includes validation and type checking functions
- Supports Unicode and multi-byte character operations

## Self-Hosting Support

The stringz module is critical for self-hosting and includes all string operations needed for:

- Source code processing
- Token parsing and manipulation
- String formatting and output
- File path manipulation
- Configuration parsing
- Error message formatting

All functions are implemented in pure CURSED without external dependencies, making them suitable for bootstrap compilation and self-hosting scenarios.

## Performance Considerations

- String operations are optimized for common use cases
- Large string operations are handled efficiently
- Memory usage is minimized through intelligent string handling
- Functions are designed to work well with the CURSED runtime system
