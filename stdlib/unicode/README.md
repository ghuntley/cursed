# Unicode Module

Pure CURSED implementation of Unicode and UTF-8 validation, encoding, and character classification.

## Features

- **UTF-8 Validation**: Complete UTF-8 byte sequence validation
- **Unicode Character Classification**: Comprehensive character type detection
- **Case Conversion**: Unicode-aware case conversion
- **Encoding/Decoding**: UTF-8 to Unicode codepoint conversion
- **Character Counting**: String length in characters vs bytes
- **Block Detection**: Unicode block identification

## Core Functions

### UTF-8 Validation

```cursed
# Check if byte is valid UTF-8 start byte
is_utf8_start_byte(byte normie) lit

# Check if byte is UTF-8 continuation byte
is_utf8_continuation_byte(byte normie) lit

# Get expected byte count for UTF-8 sequence
utf8_sequence_length(first_byte normie) normie

# Validate UTF-8 byte sequence
validate_utf8_sequence(bytes []normie, start_pos normie) lit

# Validate entire UTF-8 string
validate_utf8_string(text tea) lit
```

### Unicode Conversion

```cursed
# Convert UTF-8 bytes to Unicode code point
utf8_to_codepoint(bytes []normie, start_pos normie) normie

# Convert Unicode code point to UTF-8 bytes
codepoint_to_utf8(codepoint normie) []normie
```

### Character Counting

```cursed
# Count UTF-8 characters in string (not bytes)
utf8_char_count(text tea) normie

# Get byte length of UTF-8 string
utf8_byte_count(text tea) normie
```

### Character Classification

```cursed
# Basic Unicode ranges
is_ascii(codepoint normie) lit
is_latin1(codepoint normie) lit
is_bmp(codepoint normie) lit
is_valid_unicode(codepoint normie) lit

# Character types
is_unicode_digit(codepoint normie) lit
is_unicode_letter(codepoint normie) lit
is_unicode_whitespace(codepoint normie) lit
```

### Case Conversion

```cursed
# Convert individual codepoints
to_unicode_upper(codepoint normie) normie
to_unicode_lower(codepoint normie) normie

# Convert entire strings
string_to_unicode_upper(text tea) tea
string_to_unicode_lower(text tea) tea
```

### Unicode Information

```cursed
# Get Unicode codepoint at character position
get_codepoint_at(text tea, char_pos normie) normie

# Get Unicode block name for codepoint
get_unicode_block(codepoint normie) tea

# Check if string contains only ASCII
is_ascii_string(text tea) lit
```

## Usage Examples

### Basic UTF-8 Validation

```cursed
yeet "unicode"

# Validate UTF-8 string
bestie validate_utf8_string("Hello, 世界! 🌍") {
    vibez.spill("Valid UTF-8 string")
} nah {
    vibez.spill("Invalid UTF-8 string")
}

# Count characters vs bytes
sus text tea = "Hello, 世界!"
sus char_count normie = utf8_char_count(text)
sus byte_count normie = utf8_byte_count(text)
vibez.spill("Characters: ", char_count, ", Bytes: ", byte_count)
```

### Character Classification

```cursed
yeet "unicode"

# Classify characters
sus codepoint normie = 0x4E16  # 世
vibez.spill("Is letter: ", is_unicode_letter(codepoint))
vibez.spill("Unicode block: ", get_unicode_block(codepoint))

# Check character types
sus digit normie = 0x0966  # Devanagari ०
vibez.spill("Is digit: ", is_unicode_digit(digit))

sus space normie = 0x3000  # Ideographic space
vibez.spill("Is whitespace: ", is_unicode_whitespace(space))
```

### Case Conversion

```cursed
yeet "unicode"

# Convert case
sus text tea = "Hello, Wörld!"
sus upper tea = string_to_unicode_upper(text)
sus lower tea = string_to_unicode_lower(text)
vibez.spill("Original: ", text)
vibez.spill("Upper: ", upper)
vibez.spill("Lower: ", lower)
```

### UTF-8 Encoding/Decoding

```cursed
yeet "unicode"

# Convert codepoint to UTF-8
sus codepoint normie = 0x4E16  # 世
sus utf8_bytes []normie = codepoint_to_utf8(codepoint)
vibez.spill("UTF-8 bytes: ", utf8_bytes)

# Convert UTF-8 to codepoint
sus decoded normie = utf8_to_codepoint(utf8_bytes, 0)
vibez.spill("Decoded codepoint: ", decoded)
```

## Unicode Standards Compliance

- **UTF-8**: RFC 3629 compliant encoding/decoding
- **Unicode**: Unicode Standard 15.0 character classification
- **Character Ranges**: Comprehensive Unicode block detection
- **Case Conversion**: Unicode case mapping (basic implementation)
- **Validation**: Strict UTF-8 validation with error detection

## Supported Unicode Blocks

The module supports detection of 100+ Unicode blocks including:

- Basic Latin and Latin Extensions
- Greek, Cyrillic, Hebrew, Arabic
- Devanagari, Bengali, and other Indic scripts
- CJK (Chinese, Japanese, Korean) characters
- Mathematical symbols and operators
- Emoji and pictographic symbols
- Private Use Areas
- And many more...

## Performance Characteristics

- **Pure CURSED**: No external dependencies or unsafe operations
- **Memory Safe**: All operations use safe array bounds checking
- **Efficient**: Optimized for common ASCII and Latin-1 cases
- **Comprehensive**: Full Unicode range support up to U+10FFFF

## Integration

This module is designed to integrate with:

- **String Module**: Enhanced string processing with Unicode awareness
- **Regex Module**: Unicode-aware pattern matching
- **JSON Module**: Proper Unicode handling in JSON parsing
- **Text Processing**: Any module requiring Unicode support

## Testing

Run comprehensive tests:

```bash
cargo run --bin cursed stdlib/unicode/test_unicode.csd
```

The test suite includes:

- UTF-8 validation with various byte sequences
- Character classification across multiple languages
- Case conversion with edge cases
- Unicode block detection
- Boundary condition testing
- Multi-language character support

## Future Enhancements

- Full Unicode normalization (NFD, NFKC, NFKD)
- Extended case conversion with special mappings
- Grapheme cluster boundary detection
- Bidirectional text support
- Unicode collation support
