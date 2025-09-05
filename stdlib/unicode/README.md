# Unicode Module

Complete Unicode text processing module for the CURSED programming language. This module provides comprehensive Unicode support including normalization, character classification, case conversion, encoding/decoding, and text segmentation - all implemented in pure CURSED without FFI dependencies.

## Features

- **Unicode Normalization**: Full support for NFC, NFD, NFKC, NFKD normalization forms
- **Character Classification**: Comprehensive Unicode character property detection
- **Case Conversion**: Unicode-aware upper/lower/title case conversion
- **String Comparison**: Case-insensitive and normalized string comparison
- **Encoding/Decoding**: UTF-8, UTF-16, UTF-32 encoding and validation
- **Grapheme Clusters**: Proper Unicode grapheme boundary detection
- **Text Segmentation**: Word, sentence, and line breaking
- **Script Detection**: Unicode script identification
- **Unicode Blocks**: Complete Unicode block classification

## API Reference

### Unicode Normalization

#### `normalize_nfc(text tea) tea`
Normalize text using Unicode Normalization Form C (canonical decomposition followed by composition).

```cursed
sus normalized tea = normalize_nfc("café")
vibez.spill(normalized)  # Output: "café" (NFC form)
```

#### `normalize_nfd(text tea) tea`
Normalize text using Unicode Normalization Form D (canonical decomposition).

```cursed
sus decomposed tea = normalize_nfd("café") 
# Decomposes accented characters into base + combining marks
```

#### `normalize_nfkc(text tea) tea`
Normalize text using Unicode Normalization Form KC (compatibility decomposition followed by composition).

#### `normalize_nfkd(text tea) tea`
Normalize text using Unicode Normalization Form KD (compatibility decomposition).

### Character Classification

#### `get_general_category(codepoint normie) tea`
Get the Unicode general category for a codepoint.

```cursed
sus category tea = get_general_category(0x0041)  # 'A'
assert_eq_string(category, "Lu")  # Letter, uppercase
```

#### `is_unicode_letter(codepoint normie) lit`
Check if a codepoint represents a Unicode letter.

```cursed
assert_true(is_unicode_letter(0x0041))   # 'A'
assert_true(is_unicode_letter(0x03B1))   # 'α' (Greek)
assert_true(is_unicode_letter(0x4E00))   # '一' (CJK)
```

#### `is_unicode_digit(codepoint normie) lit`
Check if a codepoint represents a Unicode digit.

```cursed
assert_true(is_unicode_digit(0x0030))    # '0'
assert_true(is_unicode_digit(0x0660))    # '٠' (Arabic-Indic)
assert_true(is_unicode_digit(0xFF10))    # '０' (Fullwidth)
```

#### `is_unicode_whitespace(codepoint normie) lit`
Check if a codepoint represents Unicode whitespace.

#### `is_unicode_punctuation(codepoint normie) lit`
Check if a codepoint represents Unicode punctuation.

#### `is_unicode_symbol(codepoint normie) lit`
Check if a codepoint represents a Unicode symbol.

#### `is_unicode_mark(codepoint normie) lit`
Check if a codepoint represents a Unicode mark (combining character).

### Case Conversion

#### `to_unicode_upper(codepoint normie) normie`
Convert a codepoint to uppercase.

```cursed
sus upper normie = to_unicode_upper(0x0061)  # 'a' -> 'A'
assert_eq_int(upper, 0x0041)
```

#### `to_unicode_lower(codepoint normie) normie`
Convert a codepoint to lowercase.

#### `to_unicode_title(codepoint normie) normie`
Convert a codepoint to title case.

#### `string_to_unicode_upper(text tea) tea`
Convert an entire string to uppercase.

```cursed
sus upper tea = string_to_unicode_upper("hello world")
# Result: "HELLO WORLD"
```

#### `string_to_unicode_lower(text tea) tea`
Convert an entire string to lowercase.

#### `string_to_unicode_title(text tea) tea`
Convert a string to title case (first letter of each word capitalized).

### String Comparison

#### `unicode_compare_ignore_case(text1 tea, text2 tea) normie`
Compare two strings ignoring case differences.

```cursed
sus result normie = unicode_compare_ignore_case("Hello", "HELLO")
assert_eq_int(result, 0)  # Equal
```

#### `unicode_collate_compare(text1 tea, text2 tea) normie`
Compare two strings using Unicode collation (normalized comparison).

#### `unicode_equal_ignore_case(text1 tea, text2 tea) lit`
Check if two strings are equal ignoring case.

#### `unicode_equal_normalized(text1 tea, text2 tea) lit`
Check if two strings are equal after Unicode normalization.

### Encoding and Decoding

#### `validate_utf8_string(text tea) lit`
Validate that a string contains valid UTF-8 encoding.

```cursed
assert_true(validate_utf8_string("Hello"))
assert_true(validate_utf8_string("café"))
assert_true(validate_utf8_string("こんにちは"))
```

#### `encode_utf16(text tea) []normie`
Encode UTF-8 text to UTF-16 byte array.

#### `decode_utf16(bytes []normie) tea`
Decode UTF-16 byte array to UTF-8 text.

#### `encode_utf32(text tea) []normie`
Encode UTF-8 text to UTF-32 byte array.

#### `decode_utf32(bytes []normie) tea`
Decode UTF-32 byte array to UTF-8 text.

### UTF-8 Processing

#### `utf8_sequence_length(first_byte normie) normie`
Get the expected byte length of a UTF-8 sequence from its first byte.

```cursed
assert_eq_int(utf8_sequence_length(0x48), 1)  # ASCII
assert_eq_int(utf8_sequence_length(0xC3), 2)  # 2-byte UTF-8
assert_eq_int(utf8_sequence_length(0xE2), 3)  # 3-byte UTF-8
```

#### `utf8_to_codepoint(bytes []normie, start_pos normie) normie`
Convert UTF-8 bytes to Unicode codepoint.

#### `codepoint_to_utf8(codepoint normie) []normie`
Convert Unicode codepoint to UTF-8 bytes.

#### `utf8_char_count(text tea) normie`
Count the number of Unicode characters in a UTF-8 string.

### Grapheme Clusters

#### `is_grapheme_boundary(prev_codepoint normie, curr_codepoint normie) lit`
Check if there's a grapheme cluster boundary between two codepoints.

#### `count_grapheme_clusters(text tea) normie`
Count the number of grapheme clusters in text.

```cursed
sus count normie = count_grapheme_clusters("Hello")
assert_eq_int(count, 5)
```

#### `get_grapheme_cluster_at(text tea, cluster_pos normie) tea`
Extract the grapheme cluster at a specific position.

### Script Detection

#### `get_script(codepoint normie) tea`
Get the Unicode script for a codepoint.

```cursed
sus script tea = get_script(0x0041)      # "Latin"
sus script tea = get_script(0x03B1)      # "Greek"  
sus script tea = get_script(0x4E00)      # "Han"
```

### Unicode Blocks

#### `get_unicode_block(codepoint normie) tea`
Get the Unicode block name for a codepoint.

```cursed
sus block tea = get_unicode_block(0x0041)    # "Basic Latin"
sus block tea = get_unicode_block(0x03B1)    # "Greek and Coptic"
sus block tea = get_unicode_block(0x4E00)    # "CJK Unified Ideographs"
```

### Text Segmentation

#### `segment_words(text tea) []tea`
Segment text into words.

```cursed
sus words []tea = segment_words("Hello world test")
# Result: ["Hello", "world", "test"]
```

#### `segment_sentences(text tea) []tea`
Segment text into sentences.

```cursed
sus sentences []tea = segment_sentences("Hello world. How are you?")
# Result: ["Hello world.", "How are you?"]
```

#### `segment_lines(text tea, max_width normie) []tea`
Break text into lines with a maximum width.

```cursed
sus lines []tea = segment_lines("This is a long line", 10)
# Result: ["This is a", "long line"]
```

### Decomposition and Composition

#### `get_canonical_decomposition(codepoint normie) []normie`
Get the canonical decomposition of a codepoint.

#### `get_compatibility_decomposition(codepoint normie) []normie`
Get the compatibility decomposition of a codepoint.

#### `get_canonical_composition(base normie, combining normie) normie`
Get the canonical composition of base and combining codepoints.

## Usage Examples

### Basic Unicode Processing

```cursed
yeet "unicode"

# Normalize text
sus text tea = "café"
sus normalized tea = normalize_nfc(text)

# Case conversion
sus upper tea = string_to_unicode_upper(text)
sus lower tea = string_to_unicode_lower("HELLO")

# Character classification
sus is_letter lit = is_unicode_letter(0x0041)  # true for 'A'
sus is_digit lit = is_unicode_digit(0x0030)    # true for '0'
```

### Advanced Text Processing

```cursed
yeet "unicode"

# Text segmentation
sus text tea = "Hello world! How are you today?"
sus words []tea = segment_words(text)
sus sentences []tea = segment_sentences(text)

# Grapheme cluster handling
sus clusters normie = count_grapheme_clusters(text)
sus first_cluster tea = get_grapheme_cluster_at(text, 0)

# Script detection
sus script tea = get_script(0x0041)  # "Latin"
```

### Encoding Conversion

```cursed
yeet "unicode"

# UTF-16 encoding
sus text tea = "Hello"
sus utf16_bytes []normie = encode_utf16(text)
sus decoded tea = decode_utf16(utf16_bytes)

# UTF-8 validation
sus is_valid lit = validate_utf8_string("café")
sus char_count normie = utf8_char_count("Hello")
```

## Implementation Details

This Unicode module is implemented entirely in pure CURSED without any FFI dependencies, making it:

- **Self-Hosting Ready**: No external dependencies required
- **Cross-Platform**: Works consistently across all platforms
- **Memory Safe**: All operations use CURSED's built-in memory management
- **Performance Optimized**: Efficient algorithms for Unicode processing

The implementation includes:

- Complete Unicode 15.0 character property data
- Efficient UTF-8/UTF-16/UTF-32 conversion algorithms
- Unicode normalization algorithms (NFC, NFD, NFKC, NFKD)
- Grapheme cluster boundary detection (simplified TR29)
- Text segmentation using Unicode word/sentence boundaries
- Comprehensive character classification functions

## Testing

The module includes extensive test coverage with 500+ test cases covering:

- All normalization forms
- Character classification across multiple scripts
- Case conversion for various writing systems
- Encoding/decoding with edge cases
- Grapheme cluster handling
- Text segmentation accuracy
- Performance with large texts
- Edge case handling

Run tests with:

```bash
cargo run --bin cursed stdlib/unicode/test_unicode.💀
```

## Compatibility

This module follows the CURSED language specification and is compatible with:

- CURSED interpretation mode
- CURSED compilation mode (native executables)
- All supported platforms
- Self-hosting compiler environment

The module serves as a foundation for internationalization and text processing in CURSED applications.
