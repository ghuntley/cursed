# CURSED Unicode Enhanced Features

## Overview

The CURSED StringZ Unicode Enhanced Module provides comprehensive Unicode support including normalization, categorization, grapheme cluster processing, and advanced text collation. This addresses P2 Priority Unicode support requirements.

## Features

### 1. Unicode Normalization Forms

#### Supported Forms
- **NFC**: Canonical Decomposition followed by Canonical Composition
- **NFD**: Canonical Decomposition only  
- **NFKC**: Compatibility Decomposition followed by Canonical Composition
- **NFKD**: Compatibility Decomposition only

#### Usage Examples

```cursed
yeet "stringz/unicode_enhanced"

// Normalize text to NFC form
sus normalized tea = normalize_unicode("café", "NFC")

// Normalize to NFD (decomposed) form
sus decomposed tea = normalize_unicode("café", "NFD")

// Compatibility normalization
sus compat tea = normalize_unicode("²", "NFKC")  // Superscript 2 → 2
```

#### Character Decomposition

```cursed
// Canonical decomposition
sus decomposed tea = canonical_decompose_char(0x00C0)  // À → A + combining grave

// Compatibility decomposition  
sus compat tea = compatibility_decompose_char(0x2126)  // Ω (Ohm) → Ω (Omega)
```

### 2. Unicode Category Detection

#### General Categories
- **Letter**: Lu (uppercase), Ll (lowercase), Lt (titlecase), Lm (modifier), Lo (other)
- **Mark**: Mn (nonspacing), Mc (spacing combining), Me (enclosing)
- **Number**: Nd (decimal digit), Nl (letter), No (other)
- **Punctuation**: Pc (connector), Pd (dash), Ps (open), Pe (close), Pi (initial quote), Pf (final quote), Po (other)
- **Symbol**: Sm (math), Sc (currency), Sk (modifier), So (other)
- **Separator**: Zs (space), Zl (line), Zp (paragraph)
- **Other**: Cc (control), Cf (format), Cs (surrogate), Co (private use), Cn (not assigned)

#### Usage Examples

```cursed
// Get Unicode category
sus category tea = get_unicode_category(0x0041)  // "Lu" (Letter, uppercase)
sus digit_cat tea = get_unicode_category(0x0035)  // "Nd" (Number, decimal digit)

// Helper functions
sus is_letter_char lit = is_letter(0x0041)         // true
sus is_digit_char lit = is_digit(0x0035)           // true
sus is_combining lit = is_combining_mark(0x0300)   // true (combining grave)
sus is_space lit = is_whitespace_category(0x0020)  // true
```

### 3. Grapheme Cluster Support

Grapheme clusters represent user-perceived characters, which may consist of multiple Unicode codepoints.

#### Features
- Cluster boundary detection
- Proper length calculation
- Substring operations by cluster

#### Usage Examples

```cursed
// Break text into grapheme clusters
sus clusters []tea = get_grapheme_clusters("héllo")

// Get grapheme length (user-perceived character count)
sus length drip = grapheme_length("héllo")  // 5, not byte length

// Extract substring by grapheme position
sus substring tea = grapheme_substring("héllo", 1, 3)  // "éll"

// Check if character extends grapheme cluster
sus extends lit = is_grapheme_extend(0x0300)  // true (combining grave)
```

### 4. Unicode Collation Algorithms

Implements Unicode Collation Algorithm (UCA) with multiple strength levels.

#### Collation Strengths
- **Primary**: Ignore case and accents (base character comparison)
- **Secondary**: Consider accents but ignore case  
- **Tertiary**: Consider case and accents
- **Identical**: Bitwise comparison

#### Usage Examples

```cursed
// Primary collation (ignore case and accents)
sus result drip = unicode_collate("café", "CAFE", "primary")  // 0 (equal)

// Secondary collation (consider accents)
sus result2 drip = unicode_collate("cafe", "café", "secondary")  // ≠ 0

// Tertiary collation (consider case)
sus result3 drip = unicode_collate("Cafe", "cafe", "tertiary")  // ≠ 0

// Identical collation
sus result4 drip = unicode_collate("café", "café", "identical")  // 0
```

#### Normalization for Collation

```cursed
// Normalize text for specific collation strength
sus primary_norm tea = normalize_for_collation("CAFÉ", "primary")
sus secondary_norm tea = normalize_for_collation("CAFÉ", "secondary")
```

### 5. Script Detection

Identifies writing systems used in text.

#### Supported Scripts
- Latin, Greek, Cyrillic, Arabic, Hebrew
- CJK (Han ideographs), Hangul, Hiragana, Katakana
- Thai, Devanagari, and more

#### Usage Examples

```cursed
// Detect script of individual character
sus script tea = get_script(0x0041)      // "Latin"
sus greek tea = get_script(0x03B1)       // "Greek" (α)
sus arabic tea = get_script(0x0627)      // "Arabic" (ا)

// Find dominant script in text
sus dominant tea = get_dominant_script("Hello world")  // "Latin"
sus mixed tea = get_dominant_script("Hello 世界")      // depends on character count
```

### 6. Advanced Text Processing

#### Word Breaking
Breaks text into words according to Unicode Word Break algorithm.

```cursed
// Break text into words
sus words []tea = unicode_word_break("Hello, world! How are you?")
// Result: ["Hello", "world", "How", "are", "you"]

// Check if character causes word break
sus is_break lit = is_word_break_char(0x0020)  // true (space)
```

#### Line Breaking
Breaks text into lines with specified maximum width.

```cursed
// Break text into lines
sus lines []tea = unicode_line_break("This is a long sentence", 10)
// Result: multiple lines with max 10 grapheme clusters each
```

### 7. Hangul Support

Special support for Korean Hangul syllable decomposition.

#### Usage Examples

```cursed
// Decompose Hangul syllable into Jamo
sus decomposed tea = decompose_hangul(0xAC00)  // 가 → ㄱ + ㅏ
```

### 8. Integration with Existing StringZ

The enhanced Unicode module integrates seamlessly with existing StringZ functions:

```cursed
yeet "stringz"              // Basic string operations
yeet "stringz/unicode_stringz"    // Enhanced Unicode support
yeet "stringz/unicode_enhanced"   // Advanced Unicode features

// All modules work together
sus text tea = "Hello 世界!"
sus length drip = unicode_length(text)
sus normalized tea = normalize_unicode(text, "NFC")
sus script tea = get_dominant_script(text)
```

## Performance Considerations

### Optimizations
- Efficient UTF-8 decoding/encoding
- Lazy evaluation of normalization
- Caching of category lookups
- Streamlined grapheme cluster detection

### Memory Usage
- Arena allocators for temporary data
- Zero-copy operations where possible
- Minimal memory allocation for common cases

## Testing

Comprehensive test suite available in `unicode_enhanced_test.csd`:

```bash
./zig-out/bin/cursed-zig unicode_enhanced_test.csd
```

### Test Coverage
- ✅ Unicode normalization (all forms)
- ✅ Category detection (all major categories)
- ✅ Grapheme cluster processing
- ✅ Collation algorithms (all strengths)
- ✅ Script detection (major scripts)
- ✅ Text processing (word/line breaking)
- ✅ Hangul decomposition
- ✅ Integration with existing StringZ

## Implementation Status

| Feature | Status | Coverage |
|---------|--------|----------|
| Unicode Normalization | ✅ Complete | NFC, NFD, NFKC, NFKD |
| Category Detection | ✅ Complete | All major categories |
| Grapheme Clusters | ✅ Complete | Boundary detection, operations |
| Collation | ✅ Complete | All UCA strength levels |
| Script Detection | ✅ Complete | 15+ major scripts |
| Text Processing | ✅ Complete | Word/line breaking |
| Hangul Support | ✅ Complete | Syllable decomposition |

## Unicode Standard Compliance

This implementation follows Unicode 15.0 standards:
- Unicode Normalization (UAX #15)
- Unicode Collation Algorithm (UTS #10)  
- Unicode Text Segmentation (UAX #29)
- Unicode Script Property (UAX #24)

## Examples

### Complete Unicode Processing Pipeline

```cursed
yeet "stringz/unicode_enhanced"

slay process_multilingual_text(input tea) tea {
    // 1. Normalize to NFC
    sus normalized tea = normalize_unicode(input, "NFC")
    
    // 2. Detect dominant script
    sus script tea = get_dominant_script(normalized)
    
    // 3. Break into grapheme clusters for processing
    sus clusters []tea = get_grapheme_clusters(normalized)
    
    // 4. Process based on script
    ready (script == "Arabic" || script == "Hebrew") {
        // Right-to-left processing
        damn process_rtl_text(normalized)
    } otherwise ready (script == "Han" || script == "Hangul") {
        // CJK processing
        damn process_cjk_text(normalized)
    }
    
    // 5. Default processing
    damn process_default_text(normalized)
}
```

### Advanced Search with Unicode Collation

```cursed
slay unicode_search(haystack tea, needle tea, case_sensitive lit) lit {
    sus strength tea = case_sensitive ? "tertiary" : "primary"
    
    sus haystack_len drip = grapheme_length(haystack)
    sus needle_len drip = grapheme_length(needle)
    
    sus i drip = 0
    bestie (i <= haystack_len - needle_len) {
        sus candidate tea = grapheme_substring(haystack, i, needle_len)
        sus cmp drip = unicode_collate(candidate, needle, strength)
        ready (cmp == 0) {
            damn based
        }
        i = i + 1
    }
    
    damn cap
}
```

## Future Enhancements

- **Locale-specific collation**: Support for language-specific sorting
- **Advanced normalization**: Custom normalization forms
- **Regex support**: Unicode-aware regular expressions
- **Bidirectional text**: Full BiDi algorithm implementation
- **Additional scripts**: Support for more writing systems

## Contributing

To contribute to Unicode support:

1. Understand Unicode standards (see specifications above)
2. Add test cases for new features
3. Ensure memory safety with Valgrind
4. Follow existing code patterns
5. Document new features

## References

- [Unicode Standard 15.0](https://www.unicode.org/versions/Unicode15.0.0/)
- [UAX #15: Unicode Normalization Forms](https://www.unicode.org/reports/tr15/)
- [UTS #10: Unicode Collation Algorithm](https://www.unicode.org/reports/tr10/)
- [UAX #29: Unicode Text Segmentation](https://www.unicode.org/reports/tr29/)
- [UAX #24: Unicode Script Property](https://www.unicode.org/reports/tr24/)
