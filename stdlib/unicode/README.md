# Unicode Module

A comprehensive Unicode text processing module for the CURSED programming language. This module provides full Unicode support including normalization, character classification, case conversion, string comparison, encoding/decoding, grapheme cluster handling, and text segmentation.

## Features

### 1. Unicode Normalization
- **NFC (Canonical Decomposition + Canonical Composition)**: Standard normalization form
- **NFD (Canonical Decomposition)**: Decomposed form for analysis
- **NFKC (Compatibility Decomposition + Canonical Composition)**: Compatibility normalization
- **NFKD (Compatibility Decomposition)**: Full compatibility decomposition

### 2. Character Classification
- **General Categories**: Lu, Ll, Nd, Zs, Po, Lo, Cn, etc.
- **Letters**: Support for Latin, Greek, Cyrillic, Hebrew, Arabic, Devanagari, CJK, Hangul
- **Digits**: ASCII, Arabic-Indic, Devanagari, Bengali, and more
- **Whitespace**: Space, Tab, Line separators, Ideographic space
- **Punctuation**: ASCII and Unicode punctuation marks
- **Symbols**: Mathematical, currency, arrows, geometric shapes
- **Marks**: Combining diacritical marks, accents

### 3. Case Conversion
- **Uppercase**: Convert to uppercase with Unicode awareness
- **Lowercase**: Convert to lowercase with Unicode awareness
- **Title Case**: Convert to title case for proper nouns
- **String Conversion**: Full string case conversion functions

### 4. String Comparison
- **Case-insensitive**: Compare strings ignoring case differences
- **Normalized**: Compare with Unicode normalization
- **Collation**: Unicode-aware string collation

### 5. Encoding/Decoding
- **UTF-8**: Validation, encoding, decoding
- **UTF-16**: Encoding/decoding with surrogate pair support
- **UTF-32**: Fixed-width encoding/decoding
- **Validation**: Comprehensive UTF-8 sequence validation

### 6. Grapheme Cluster Handling
- **Boundary Detection**: Identify grapheme cluster boundaries
- **Cluster Counting**: Count user-perceived characters
- **Cluster Extraction**: Extract individual grapheme clusters
- **Script Detection**: Identify character scripts

### 7. Text Segmentation
- **Word Segmentation**: Split text into words
- **Sentence Segmentation**: Split text into sentences
- **Line Breaking**: Break text into lines with width limits

## Usage Examples

### Basic Usage

```cursed
yeet "unicode"

# Normalize text
sus normalized tea = normalize_nfc("café")
vibez.spill(normalized)

# Character classification
sus is_letter lit = is_unicode_letter(0x0041)  # 'A'
sus is_digit lit = is_unicode_digit(0x0030)    # '0'
sus is_space lit = is_unicode_whitespace(0x0020)  # ' '

# Case conversion
sus upper_text tea = string_to_unicode_upper("hello world")
sus lower_text tea = string_to_unicode_lower("HELLO WORLD")
sus title_text tea = string_to_unicode_title("hello world")

# String comparison
sus are_equal lit = unicode_equal_ignore_case("Hello", "HELLO")
sus are_normalized lit = unicode_equal_normalized("café", "café")
```

### Advanced Usage

```cursed
# Grapheme cluster handling
sus cluster_count normie = count_grapheme_clusters("café")
sus first_cluster tea = get_grapheme_cluster_at("café", 0)

# Text segmentation
sus words []tea = segment_words("Hello world, how are you?")
sus sentences []tea = segment_sentences("Hello world. How are you? I am fine!")
sus lines []tea = segment_lines("This is a long line that needs breaking", 20)

# Encoding/decoding
sus utf16_bytes []normie = encode_utf16("Hello")
sus utf16_text tea = decode_utf16(utf16_bytes)
sus utf32_bytes []normie = encode_utf32("Hello")
sus utf32_text tea = decode_utf32(utf32_bytes)

# Character properties
sus script tea = get_script(0x0041)  # "Latin"
sus block tea = get_unicode_block(0x0041)  # "Basic Latin"
sus category tea = get_general_category(0x0041)  # "Lu"
```

### Normalization Examples

```cursed
# Different normalization forms
sus text tea = "café"
sus nfc tea = normalize_nfc(text)    # Composed form
sus nfd tea = normalize_nfd(text)    # Decomposed form
sus nfkc tea = normalize_nfkc(text)  # Compatibility composed
sus nfkd tea = normalize_nfkd(text)  # Compatibility decomposed

# Manual decomposition/composition
sus decomposed tea = canonical_decompose("café")
sus composed tea = canonical_compose(decomposed)
```

### Character Classification Examples

```cursed
# Test various character types
sus codepoint normie = 0x0041  # 'A'

bestie is_unicode_letter(codepoint) {
    vibez.spill("Character is a letter")
}

bestie is_unicode_digit(codepoint) {
    vibez.spill("Character is a digit")
}

bestie is_unicode_whitespace(codepoint) {
    vibez.spill("Character is whitespace")
}

bestie is_unicode_punctuation(codepoint) {
    vibez.spill("Character is punctuation")
}

bestie is_unicode_symbol(codepoint) {
    vibez.spill("Character is a symbol")
}

bestie is_unicode_mark(codepoint) {
    vibez.spill("Character is a combining mark")
}
```

### Case Conversion Examples

```cursed
# Codepoint case conversion
sus upper_a normie = to_unicode_upper(0x0061)  # 'a' -> 'A'
sus lower_a normie = to_unicode_lower(0x0041)  # 'A' -> 'a'
sus title_a normie = to_unicode_title(0x0061)  # 'a' -> 'A'

# String case conversion
sus text tea = "Hello World"
sus upper tea = string_to_unicode_upper(text)   # "HELLO WORLD"
sus lower tea = string_to_unicode_lower(text)   # "hello world"
sus title tea = string_to_unicode_title(text)   # "Hello World"

# Unicode case conversion
sus unicode_text tea = "café résumé"
sus upper_unicode tea = string_to_unicode_upper(unicode_text)  # "CAFÉ RÉSUMÉ"
sus lower_unicode tea = string_to_unicode_lower(unicode_text)  # "café résumé"
```

### Encoding/Decoding Examples

```cursed
# UTF-8 validation
sus valid_utf8 lit = validate_utf8_string("Hello, 世界!")
bestie valid_utf8 {
    vibez.spill("Valid UTF-8 string")
}

# UTF-16 encoding/decoding
sus text tea = "Hello, 世界!"
sus utf16_encoded []normie = encode_utf16(text)
sus utf16_decoded tea = decode_utf16(utf16_encoded)

# UTF-32 encoding/decoding
sus utf32_encoded []normie = encode_utf32(text)
sus utf32_decoded tea = decode_utf32(utf32_encoded)

# Character counting
sus byte_count normie = utf8_byte_count(text)
sus char_count normie = utf8_char_count(text)
sus cluster_count normie = count_grapheme_clusters(text)
```

### Text Segmentation Examples

```cursed
# Word segmentation
sus text tea = "Hello, world! How are you today?"
sus words []tea = segment_words(text)
sus i normie = 0
bestie i < len(words) {
    vibez.spill("Word: " + words[i])
    i++
}

# Sentence segmentation
sus paragraph tea = "Hello world. How are you? I am fine! Thanks for asking."
sus sentences []tea = segment_sentences(paragraph)
sus j normie = 0
bestie j < len(sentences) {
    vibez.spill("Sentence: " + sentences[j])
    j++
}

# Line breaking
sus long_text tea = "This is a very long line that needs to be broken into multiple lines for better readability"
sus lines []tea = segment_lines(long_text, 30)
sus k normie = 0
bestie k < len(lines) {
    vibez.spill("Line: " + lines[k])
    k++
}
```

## Function Reference

### Normalization Functions

- `normalize_nfc(text tea) tea` - NFC normalization
- `normalize_nfd(text tea) tea` - NFD normalization
- `normalize_nfkc(text tea) tea` - NFKC normalization
- `normalize_nfkd(text tea) tea` - NFKD normalization
- `canonical_decompose(text tea) tea` - Canonical decomposition
- `compatibility_decompose(text tea) tea` - Compatibility decomposition
- `canonical_compose(text tea) tea` - Canonical composition

### Character Classification Functions

- `get_general_category(codepoint normie) tea` - Get Unicode general category
- `is_unicode_letter(codepoint normie) lit` - Check if character is a letter
- `is_unicode_digit(codepoint normie) lit` - Check if character is a digit
- `is_unicode_whitespace(codepoint normie) lit` - Check if character is whitespace
- `is_unicode_punctuation(codepoint normie) lit` - Check if character is punctuation
- `is_unicode_symbol(codepoint normie) lit` - Check if character is a symbol
- `is_unicode_mark(codepoint normie) lit` - Check if character is a combining mark

### Case Conversion Functions

- `to_unicode_upper(codepoint normie) normie` - Convert codepoint to uppercase
- `to_unicode_lower(codepoint normie) normie` - Convert codepoint to lowercase
- `to_unicode_title(codepoint normie) normie` - Convert codepoint to title case
- `string_to_unicode_upper(text tea) tea` - Convert string to uppercase
- `string_to_unicode_lower(text tea) tea` - Convert string to lowercase
- `string_to_unicode_title(text tea) tea` - Convert string to title case

### String Comparison Functions

- `unicode_compare_ignore_case(text1 tea, text2 tea) normie` - Case-insensitive comparison
- `unicode_collate_compare(text1 tea, text2 tea) normie` - Collation comparison
- `unicode_equal_ignore_case(text1 tea, text2 tea) lit` - Case-insensitive equality
- `unicode_equal_normalized(text1 tea, text2 tea) lit` - Normalized equality

### Encoding/Decoding Functions

- `validate_utf8_string(text tea) lit` - Validate UTF-8 string
- `encode_utf16(text tea) []normie` - Encode to UTF-16
- `decode_utf16(bytes []normie) tea` - Decode from UTF-16
- `encode_utf32(text tea) []normie` - Encode to UTF-32
- `decode_utf32(bytes []normie) tea` - Decode from UTF-32
- `utf8_char_count(text tea) normie` - Count UTF-8 characters
- `utf8_sequence_length(first_byte normie) normie` - Get UTF-8 sequence length
- `utf8_to_codepoint(bytes []normie, start_pos normie) normie` - Convert UTF-8 to codepoint
- `codepoint_to_utf8(codepoint normie) []normie` - Convert codepoint to UTF-8

### Grapheme Cluster Functions

- `is_grapheme_boundary(prev_codepoint normie, curr_codepoint normie) lit` - Check boundary
- `get_script(codepoint normie) tea` - Get character script
- `count_grapheme_clusters(text tea) normie` - Count grapheme clusters
- `get_grapheme_cluster_at(text tea, cluster_pos normie) tea` - Get cluster at position

### Text Segmentation Functions

- `segment_words(text tea) []tea` - Segment text into words
- `segment_sentences(text tea) []tea` - Segment text into sentences
- `segment_lines(text tea, max_width normie) []tea` - Break text into lines

### Advanced Functions

- `get_unicode_block(codepoint normie) tea` - Get Unicode block name
- `get_canonical_decomposition(codepoint normie) []normie` - Get canonical decomposition
- `get_compatibility_decomposition(codepoint normie) []normie` - Get compatibility decomposition
- `get_canonical_composition(base normie, combining normie) normie` - Get canonical composition

## Testing

The module includes comprehensive tests covering all functionality:

```bash
# Run Unicode module tests
cargo run --bin cursed stdlib/unicode/test_unicode.csd

# Test both interpretation and compilation modes
cargo run --bin cursed stdlib/unicode/test_unicode.csd
cargo run --bin cursed -- compile stdlib/unicode/test_unicode.csd
./test_unicode
```

## Performance Considerations

- **Normalization**: NFC is recommended for most text processing
- **Character Classification**: Functions are optimized for common Unicode ranges
- **Case Conversion**: Supports ASCII, Latin-1, and major Unicode scripts
- **Encoding**: UTF-8 is the primary encoding with UTF-16/32 support
- **Grapheme Clusters**: Simplified implementation for common use cases
- **Text Segmentation**: Basic algorithms suitable for most languages

## Unicode Standards Compliance

This implementation follows these Unicode standards:
- **Unicode 15.0**: Character properties and classification
- **UAX #15**: Unicode Normalization Forms
- **UAX #29**: Unicode Text Segmentation
- **RFC 3629**: UTF-8 encoding
- **RFC 2781**: UTF-16 encoding

## Limitations

- **Normalization**: Basic implementation of canonical and compatibility forms
- **Character Classification**: Limited to common Unicode ranges
- **Case Conversion**: ASCII, Latin-1, and major scripts only
- **Grapheme Clusters**: Simplified boundary detection
- **Text Segmentation**: Basic algorithms without locale-specific rules

## Contributing

To extend the Unicode module:

1. Add new functions to `stdlib/unicode/mod.csd`
2. Add comprehensive tests to `stdlib/unicode/test_unicode.csd`
3. Update documentation in `stdlib/unicode/README.md`
4. Ensure all tests pass in both interpretation and compilation modes

## License

This module is part of the CURSED programming language and follows the same license terms.
