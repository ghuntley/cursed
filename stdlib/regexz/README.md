# RegexZ - Advanced Regular Expression Engine

RegexZ is a high-performance, feature-complete regular expression engine for the CURSED programming language. It provides comprehensive support for modern regex features including Unicode properties, named capture groups, lookahead/lookbehind assertions, and advanced optimization techniques.

## Features

### ✨ Core Features

- **Full POSIX ERE compliance** with modern extensions
- **Unicode support** with property matching (`\p{Script=Latin}`, `\p{Letter}`, etc.)
- **Named capture groups** (`(?P<name>pattern)`) with easy extraction
- **Lookahead and lookbehind** assertions (positive and negative)
- **Advanced character classes** including Unicode categories
- **Non-greedy quantifiers** (`*?`, `+?`, `??`, `{n,m}?`)
- **Atomic groups** and possessive quantifiers
- **Pattern compilation caching** for improved performance

### 🚀 Performance Optimizations

- **Hybrid NFA/DFA engine** - automatic optimization selection
- **Pattern compilation caching** - reuse compiled patterns
- **Profile-guided optimization** - hot path identification
- **Memory pool allocation** - reduced GC pressure
- **SIMD optimizations** - vectorized character matching
- **Backtracking prevention** - catastrophic backtracking detection

### 🌍 Unicode Support

- **Unicode 15.0 compliance** with full property database
- **Script properties** - Match by writing system (`\p{Script=Han}`)
- **General categories** - Match by character type (`\p{Letter}`, `\p{Number}`)
- **Block properties** - Match by Unicode block (`\p{Block=Basic_Latin}`)
- **Derived properties** - Alphabetic, Lowercase, Uppercase, etc.
- **Case-insensitive matching** with Unicode case folding
- **Normalization support** - NFD, NFC, NFKD, NFKC

## Quick Start

### Basic Usage

```cursed
yeet "regexz"

# Compile a pattern
sus engine RegexEngine = regex_new("\\d{3}-\\d{3}-\\d{4}") shook {
    vibez.spill("Failed to compile regex")
}

# Test if pattern matches
sus result MatchResult = regex_match(&engine, "Call me at 555-123-4567") shook {
    vibez.spill("Match failed")
}

ready (result.matched) {
    vibez.spill("Found phone number:", result.full_match)
    # Output: "Found phone number: 555-123-4567"
}
```

### Named Capture Groups

```cursed
# Extract structured data with named groups
sus date_engine RegexEngine = regex_new("(?P<year>\\d{4})-(?P<month>\\d{2})-(?P<day>\\d{2})")

sus date_result MatchResult = regex_match(&date_engine, "Today is 2023-12-25")

ready (date_result.matched) {
    sus year tea = get_named_group(date_result, "year")
    sus month tea = get_named_group(date_result, "month")  
    sus day tea = get_named_group(date_result, "day")
    
    vibez.spill("Year:", year, "Month:", month, "Day:", day)
}
```

### Unicode Properties

```cursed
# Match Unicode characters by properties
sus unicode_engine RegexEngine = regex_new("\\p{Script=Arabic}+")
sus arabic_result MatchResult = regex_match(&unicode_engine, "مرحبا بك")

ready (arabic_result.matched) {
    vibez.spill("Found Arabic text:", arabic_result.full_match)
}

# Match by general category
sus letter_engine RegexEngine = regex_new("\\p{Letter}+")
sus emoji_engine RegexEngine = regex_new("\\p{Symbol}+")
```

### Lookahead and Lookbehind

```cursed
# Positive lookahead - match digits followed by 'px'
sus pixels RegexEngine = regex_new("\\d+(?=px)")
sus pixel_result MatchResult = regex_match(&pixels, "width: 100px")
# Matches: "100" (not including "px")

# Negative lookbehind - match numbers not preceded by '$'
sus not_money RegexEngine = regex_new("(?<!\\$)\\d+")
sus number_result MatchResult = regex_match(&not_money, "€50 costs $30")
# Matches: "50" (not "30" because it follows '$')
```

### Find and Replace

```cursed
# Find all matches
sus digit_engine RegexEngine = regex_new("\\d+")
sus all_matches []MatchResult = regex_find_all(&digit_engine, "I have 5 apples and 3 oranges")

bestie (match in all_matches) {
    vibez.spill("Found number:", match.full_match)
}

# Replace with static text
sus replaced tea = regex_replace(&digit_engine, "I have 5 apples", "many")
# Result: "I have many apples"

# Replace with captured groups
sus name_engine RegexEngine = regex_new("(\\w+) (\\w+)")
sus swapped tea = regex_replace(&name_engine, "John Doe", "$2, $1")
# Result: "Doe, John"

# Replace with function
sus doubled tea = regex_replace_func(&digit_engine, "Price: $25", slay(match MatchResult) tea {
    sus num drip = match.full_match.to_int()
    damn (num * 2).to_string()
})
# Result: "Price: $50"
```

### String Splitting

```cursed
# Split on pattern
sus delimiter RegexEngine = regex_new("[,;]\\s*")
sus parts []tea = regex_split(&delimiter, "apple, banana; cherry,date")
# Result: ["apple", "banana", "cherry", "date"]
```

## Advanced Features

### Pattern Compilation Options

```cursed
sus options RegexOptions = {
    optimization_level: 2,        # 0=none, 1=basic, 2=aggressive
    unicode_support: based,       # Enable Unicode properties
    cache_enabled: based,         # Cache compiled patterns
    case_insensitive: nah,        # Case sensitivity
    multiline: nah,              # ^ and $ match line boundaries
    dotall: nah,                 # . matches newlines
    max_backtrack_steps: 100000   # Prevent catastrophic backtracking
}

sus engine RegexEngine = regex_new_with_options("complex.*pattern", options)
```

### Performance Monitoring

```cursed
# Get performance statistics
sus stats RegexStats = get_regex_stats("\\d+")
vibez.spill("Pattern used", stats.match_count, "times")
vibez.spill("Average match time:", stats.total_match_time / stats.match_count, "μs")
vibez.spill("Cache hit rate:", (stats.cache_hits * 100) / (stats.cache_hits + stats.cache_misses), "%")

# Reset statistics
reset_regex_stats()
```

### Pattern Analysis

```cursed
sus analysis PatternAnalysis = analyze_pattern("(?P<year>\\d{4})-(?P<month>\\d{2})")

vibez.spill("Has groups:", analysis.has_groups)
vibez.spill("Has lookaround:", analysis.has_lookaround) 
vibez.spill("Complexity:", analysis.estimated_complexity)  # 1=linear, 2=polynomial, 3=exponential
vibez.spill("Unicode aware:", analysis.unicode_aware)
```

### Error Handling

```cursed
# Pattern validation
ready (regex_is_valid("[invalid")) {
    vibez.spill("Pattern is valid")
} otherwise {
    vibez.spill("Pattern has syntax errors")
}

# Graceful error handling
sus engine RegexEngine = regex_new("(?P<date>\\d{4}-\\d{2}-\\d{2})") fam {
    when ParseError(msg) -> {
        vibez.spill("Parse error:", msg)
        damn create_default_regex()
    }
    when CompileError(msg) -> {
        vibez.spill("Compile error:", msg)
        damn create_default_regex()
    }
}
```

## Unicode Property Reference

### General Categories

| Property | Description | Example |
|----------|-------------|---------|
| `\p{L}` | Any letter | `\p{L}+` matches "Hello世界" |
| `\p{Lu}` | Uppercase letter | `\p{Lu}` matches "A", "Ñ", "Α" |
| `\p{Ll}` | Lowercase letter | `\p{Ll}` matches "a", "ñ", "α" |
| `\p{N}` | Any number | `\p{N}+` matches "123", "٧٨٩" |
| `\p{Nd}` | Decimal number | `\p{Nd}+` matches "123" |
| `\p{P}` | Any punctuation | `\p{P}` matches ".", "!", "?" |
| `\p{S}` | Any symbol | `\p{S}` matches "$", "©", "™" |
| `\p{Z}` | Any separator | `\p{Z}+` matches spaces, tabs |
| `\p{C}` | Any control character | `\p{C}` matches control codes |

### Script Properties

| Script | Usage | Example |
|--------|-------|---------|
| `\p{Script=Latin}` | Latin alphabet | English, Spanish, French |
| `\p{Script=Greek}` | Greek alphabet | Modern and ancient Greek |
| `\p{Script=Cyrillic}` | Cyrillic alphabet | Russian, Bulgarian |
| `\p{Script=Arabic}` | Arabic alphabet | Arabic, Persian, Urdu |
| `\p{Script=Han}` | Chinese characters | Chinese, Japanese kanji |
| `\p{Script=Hiragana}` | Japanese hiragana | あいうえお |
| `\p{Script=Katakana}` | Japanese katakana | アイウエオ |
| `\p{Script=Hangul}` | Korean alphabet | 한글 |

### Block Properties

| Block | Unicode Range | Usage |
|-------|---------------|-------|
| `\p{Block=Basic_Latin}` | U+0000-U+007F | ASCII characters |
| `\p{Block=Latin_1_Supplement}` | U+0080-U+00FF | Extended Latin |
| `\p{Block=Greek_and_Coptic}` | U+0370-U+03FF | Greek letters |
| `\p{Block=Cyrillic}` | U+0400-U+04FF | Cyrillic letters |
| `\p{Block=Arabic}` | U+0600-U+06FF | Arabic letters |
| `\p{Block=CJK_Unified_Ideographs}` | U+4E00-U+9FFF | Chinese characters |

### Derived Properties

| Property | Description |
|----------|-------------|
| `\p{Alphabetic}` | Alphabetic characters (broader than `\p{L}`) |
| `\p{Lowercase}` | Lowercase characters |
| `\p{Uppercase}` | Uppercase characters |
| `\p{White_Space}` | Whitespace characters |
| `\p{Hex_Digit}` | Hexadecimal digits (0-9, A-F, a-f) |
| `\p{ASCII_Hex_Digit}` | ASCII hexadecimal digits only |
| `\p{Ideographic}` | Ideographic characters |
| `\p{Diacritic}` | Diacritical marks |

## Performance Guide

### Compilation Optimization

RegexZ automatically optimizes patterns during compilation:

1. **NFA to DFA conversion** - For deterministic patterns
2. **State minimization** - Reduces DFA size
3. **Character class optimization** - Efficient range checking
4. **Literal string detection** - Fast Boyer-Moore search
5. **Loop unrolling** - Optimizes common quantifier patterns

### Best Practices

#### ✅ Efficient Patterns

```cursed
# Good: Anchored patterns are faster
sus anchored RegexEngine = regex_new("^https?://")

# Good: Character classes are optimized
sus efficient RegexEngine = regex_new("[a-zA-Z0-9]+")

# Good: Non-capturing groups when you don't need the capture
sus non_capturing RegexEngine = regex_new("(?:foo|bar)+")

# Good: Specific quantifiers over general ones
sus specific RegexEngine = regex_new("\\d{3,5}")  # Better than \\d{3,}
```

#### ❌ Patterns to Avoid

```cursed
# Bad: Catastrophic backtracking
sus bad RegexEngine = regex_new("(a+)+b")  # Avoid nested quantifiers

# Bad: Excessive alternation
sus bad2 RegexEngine = regex_new("word1|word2|word3|...")  # Use character classes instead

# Bad: Unbounded quantifiers at start
sus bad3 RegexEngine = regex_new(".*important")  # Anchor when possible: "^.*important"
```

### Memory Management

RegexZ uses arena allocators and object pooling for optimal memory usage:

```cursed
# Pattern caching reduces compilation overhead
sus cached_engine RegexEngine = get_cached_engine("\\d+", default_options)

# Large match operations use memory pools
sus large_text tea = load_large_file()
sus matches []MatchResult = regex_find_all(&engine, large_text)
```

## Testing

RegexZ includes a comprehensive test suite covering all features:

```bash
# Run the test suite
./zig-out/bin/cursed-zig stdlib/regexz/regex_tests.💀

# Memory leak testing
valgrind --leak-check=full ./zig-out/bin/cursed-zig stdlib/regexz/regex_tests.💀

# Performance benchmarking
./zig-out/bin/cursed-zig --benchmark stdlib/regexz/regex_performance_test.💀
```

Test coverage includes:

- ✅ Basic pattern matching
- ✅ All quantifier types (greedy, lazy, possessive)
- ✅ Capture groups (numbered and named)
- ✅ Character classes and ranges
- ✅ Unicode properties and scripts
- ✅ Lookahead and lookbehind assertions
- ✅ String replacement and splitting
- ✅ Performance optimization validation
- ✅ Edge cases and error handling
- ✅ Memory safety verification

## Architecture

### Engine Components

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Pattern       │───▶│    Parser        │───▶│      AST        │
│   String        │    │   (Syntax Tree)  │    │  (Abstract)     │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                                                          │
                                                          ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Optimized     │◀───│   NFA Compiler   │◀───│   AST Walker    │
│     DFA         │    │   (Thompson)     │    │  (Recursive)    │
└─────────────────┘    └──────────────────┘    └─────────────────┘
          │                       │
          ▼                       ▼
┌─────────────────┐    ┌──────────────────┐
│  DFA Matcher    │    │   NFA Matcher    │
│  (Linear Time)  │    │  (Backtracking)  │
└─────────────────┘    └──────────────────┘
          │                       │
          └───────────┬───────────┘
                      ▼
              ┌──────────────────┐
              │  Match Results   │
              │   (Groups &      │
              │   Positions)     │
              └──────────────────┘
```

### Performance Characteristics

| Operation | Time Complexity | Notes |
|-----------|----------------|-------|
| Pattern compilation | O(m) | m = pattern length, cached |
| DFA matching | O(n) | n = text length, guaranteed linear |
| NFA matching | O(n×m) | Worst case, optimizations reduce average |
| Unicode property lookup | O(1) | Cached range tables |
| Capture group extraction | O(g) | g = number of groups |

## License

RegexZ is part of the CURSED standard library and is licensed under the same terms as the CURSED programming language.

## Contributing

Contributions are welcome! Please see the main CURSED contribution guidelines. Areas for improvement:

- Additional Unicode property support
- More optimization strategies  
- Extended POSIX compliance
- Performance improvements
- Bug fixes and edge cases

## Changelog

### v1.0.0 (Current)
- ✅ Full Unicode 15.0 support
- ✅ Named capture groups
- ✅ Lookahead/lookbehind assertions
- ✅ Performance optimizations
- ✅ Comprehensive test suite
- ✅ Memory safety validation
- ✅ Production-ready stability

---

For more examples and advanced usage, see the [examples directory](../examples/regexz/) and the [test suite](regex_tests.💀).
