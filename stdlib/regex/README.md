# CURSED Regex Module - Pure CURSED Implementation

A comprehensive regular expression library with pattern matching, compilation, and PCRE-style features. This module provides regex functionality implemented entirely in pure CURSED without FFI dependencies.

## Features

### Core PCRE Support
- **Full PCRE Syntax**: Complete Perl-compatible regular expression syntax
- **Compilation Flags**: Support for ignore case, multiline, dotall, extended, anchored, unicode, and ungreedy modes
- **Pattern Optimization**: Automatic optimization of regex patterns for better performance
- **NFA/DFA Engines**: Dual execution engines optimized for different pattern types

### Unicode Support
- **Unicode Character Classes**: Full support for Unicode categories (\p{L}, \p{N}, \p{P}, \p{S}, \p{Z}, \p{M}, \p{C})
- **Multilingual Text**: Support for text in any language and script
- **Emoji Support**: Proper handling of emoji and other Unicode symbols
- **UTF-8 Aware**: Native UTF-8 string processing

### Advanced Pattern Features
- **Named Capture Groups**: `(?<name>pattern)` syntax for named groups
- **Backreferences**: Support for numbered (`\1`) and named (`\k<name>`) backreferences
- **Lookahead Assertions**: Positive (`(?=...)`) and negative (`(?!...)`) lookahead
- **Lookbehind Assertions**: Positive (`(?<=...)`) and negative (`(?<!...)`) lookbehind
- **Quantifiers**: Lazy (`*?`), possessive (`*+`), and exact (`{n,m}`) quantifiers

### Performance & Optimization
- **Catastrophic Backtracking Detection**: Automatic detection of problematic patterns
- **Pattern Complexity Analysis**: Detailed analysis of regex performance characteristics
- **Benchmarking Tools**: Built-in performance measurement and profiling
- **Compiler Optimizations**: Multiple optimization passes for better execution speed

### Debugging & Development
- **Pattern Explanation**: Human-readable explanations of regex patterns
- **Syntax Validation**: Comprehensive pattern validation with detailed error reporting
- **Fuzzing Support**: Input validation and security testing capabilities
- **Debug Information**: Detailed execution traces and match information

## Quick Start

```cursed
yeet "regex"

// Basic pattern matching
sus engine regex.RegexEngine = regex.regex_compile_pcre("hello", 0)
sus result regex.AdvancedMatchResult = regex.regex_match_full(engine, "hello world", 0)

// Unicode-aware matching
sus unicode_engine regex.RegexEngine = regex.regex_compile_pcre("\\p{L}+", regex.PCRE_UNICODE)
sus unicode_result regex.AdvancedMatchResult = regex.regex_match_unicode(unicode_engine, "Héllo 世界")

// Named capture groups
sus named_engine regex.RegexEngine = regex.regex_compile_pcre("(?<word>\\w+)", regex.PCRE_UNICODE)
sus named_groups [regex.NamedGroup] = regex.regex_extract_named_groups(named_engine, "hello")
```

## API Reference

### Core Functions

#### `regex_compile_pcre(pattern tea, flags normie) RegexEngine`
Compiles a PCRE pattern with specified flags.

**Parameters:**
- `pattern`: The regular expression pattern
- `flags`: Compilation flags (see PCRE flags below)

**Returns:** Compiled regex engine

**Example:**
```cursed
sus engine regex.RegexEngine = regex.regex_compile_pcre("test.*", regex.PCRE_IGNORECASE)
```

#### `regex_match_unicode(regex RegexEngine, text tea) AdvancedMatchResult`
Performs Unicode-aware pattern matching.

**Parameters:**
- `regex`: Compiled regex engine
- `text`: Text to search in

**Returns:** Advanced match result with capture groups and positions

#### `regex_find_all_advanced(regex RegexEngine, text tea) [AdvancedMatchResult]`
Finds all matches in the text.

**Parameters:**
- `regex`: Compiled regex engine
- `text`: Text to search in

**Returns:** Array of all matches found

### PCRE Compilation Flags

```cursed
sus PCRE_IGNORECASE normie = 1     // Case-insensitive matching
sus PCRE_MULTILINE normie = 2      // ^ and $ match line boundaries
sus PCRE_DOTALL normie = 4         // . matches newlines
sus PCRE_EXTENDED normie = 8       // Ignore whitespace and comments
sus PCRE_ANCHORED normie = 16      // Pattern is anchored at start
sus PCRE_UNICODE normie = 32       // Enable Unicode support
sus PCRE_UNGREEDY normie = 64      // Make quantifiers ungreedy by default
```

### Unicode Character Classes

```cursed
sus UNICODE_LETTER tea = "\\p{L}"      // All letters
sus UNICODE_DIGIT tea = "\\p{N}"       // All numbers  
sus UNICODE_PUNCTUATION tea = "\\p{P}" // All punctuation
sus UNICODE_SYMBOL tea = "\\p{S}"      // All symbols
sus UNICODE_SEPARATOR tea = "\\p{Z}"   // All separators
sus UNICODE_MARK tea = "\\p{M}"        // All marks
sus UNICODE_OTHER tea = "\\p{C}"       // Other/control characters
```

### Advanced Features

#### Named Capture Groups
```cursed
// Define named groups
sus pattern tea = "(?<year>\\d{4})-(?<month>\\d{2})-(?<day>\\d{2})"
sus engine regex.RegexEngine = regex.regex_compile_pcre(pattern, regex.PCRE_UNICODE)

// Extract named groups
sus groups [regex.NamedGroup] = regex.regex_extract_named_groups(engine, "2024-01-15")
// groups[0].name == "year", groups[0].text == "2024"
```

#### Backreferences
```cursed
// Use numbered backreferences
sus match regex.AdvancedMatchResult = // ... get match result
sus expanded tea = regex.regex_expand_backreferences("Found: \\1", match)

// Use named backreferences  
sus named_expanded tea = regex.regex_expand_backreferences("Date: \\k<year>", match)
```

#### Lookahead/Lookbehind
```cursed
// Positive lookahead: match "test" only if followed by "ing"
sus pattern tea = "test(?=ing)"

// Negative lookbehind: match "test" only if not preceded by "pre"
sus pattern tea = "(?<!pre)test"
```

### Debugging & Analysis

#### Pattern Explanation
```cursed
sus explanation tea = regex.regex_explain("^(?<word>\\w+)\\s+(?=\\d)")
// Returns detailed explanation of pattern components
```

#### Complexity Analysis
```cursed
sus analysis tea = regex.analyze_pattern_complexity("(a+)+b")
// Returns complexity score and performance warnings
```

#### Performance Benchmarking
```cursed
sus report tea = regex.regex_benchmark("\\w+", "hello world test", 1000)
// Returns detailed performance metrics
```

### Validation & Security

#### Input Validation
```cursed
sus is_valid lit = regex.regex_validate_input(pattern, 1000)
// Validates pattern safety and syntax
```

#### Catastrophic Backtracking Detection
```cursed
sus is_dangerous lit = regex.has_catastrophic_backtracking("(.*)*")
// Returns based for potentially dangerous patterns
```

## Pattern Examples

### Basic Patterns
```cursed
// Email validation
sus email_pattern tea = "^[\\w._%+-]+@[\\w.-]+\\.[A-Za-z]{2,}$"

// Phone number
sus phone_pattern tea = "^\\+?[1-9]\\d{1,14}$"

// URL validation
sus url_pattern tea = "^https?://[\\w.-]+(?:\\.[\\w\\.-]+)+[\\w\\-\\._~:/?#[\\]@!\\$&'\\(\\)\\*\\+,;=.]+$"
```

### Unicode Patterns
```cursed
// Match any letter in any language
sus letters_pattern tea = "\\p{L}+"

// Match numbers in any script
sus numbers_pattern tea = "\\p{N}+"

// Match emoji
sus emoji_pattern tea = "\\p{So}"
```

### Advanced Patterns
```cursed
// Named capture groups for date parsing
sus date_pattern tea = "(?<year>\\d{4})-(?<month>\\d{2})-(?<day>\\d{2})"

// Lookahead for password validation
sus password_pattern tea = "^(?=.*[a-z])(?=.*[A-Z])(?=.*\\d)(?=.*[\\W]).{8,}$"

// Balanced parentheses (simplified)
sus balanced_pattern tea = "\\([^()]*\\)"
```

## Performance Guidelines

### Pattern Optimization
1. **Avoid Catastrophic Backtracking**: Patterns like `(.*)*` can cause exponential time complexity
2. **Use Atomic Groups**: `(?>...)` prevents backtracking within the group
3. **Optimize Character Classes**: `[a-zA-Z0-9]` is faster than multiple alternations
4. **Anchor Patterns**: Use `^` and `$` when appropriate to limit search scope

### Best Practices
1. **Compile Once, Use Many**: Store compiled regex engines for repeated use
2. **Use Unicode Mode**: Enable `PCRE_UNICODE` for international text
3. **Validate Input**: Always validate patterns before compilation
4. **Monitor Complexity**: Use complexity analysis for performance-critical code

### Performance Monitoring
```cursed
// Benchmark critical patterns
sus report tea = regex.regex_benchmark(pattern, sample_text, 1000)

// Analyze pattern complexity
sus complexity tea = regex.analyze_pattern_complexity(pattern)

// Validate pattern safety
sus is_safe lit = regex.regex_validate_input(pattern, 10000)
```

## Testing

Run the comprehensive test suite:

```bash
# Test basic functionality
cargo run --bin cursed stdlib/regex/test_regex.💀

# Test in compilation mode
cargo run --bin cursed -- compile stdlib/regex/test_regex.💀
./test_regex

# Both-mode verification
test_both_modes stdlib/regex/test_regex.💀
```

The test suite includes:
- 200+ test cases covering all features
- Unicode text testing with multilingual content
- Performance benchmarks with large datasets
- Edge case testing and error handling
- Fuzzing and security validation

## Implementation Details

### Architecture
- **Pure CURSED Implementation**: No external dependencies
- **NFA/DFA Dual Engines**: Automatic selection based on pattern complexity
- **Unicode-First Design**: Native Unicode support throughout
- **Optimization Pipeline**: Multiple passes for performance improvement

### Algorithms
- **Thompson's Construction**: NFA generation from regex patterns
- **Subset Construction**: DFA generation from NFA for simple patterns
- **Backtracking Engine**: Full PCRE feature support for complex patterns
- **Unicode Normalization**: Proper handling of Unicode equivalence

### Security Features
- **Input Sanitization**: Pattern validation and safety checks
- **ReDoS Protection**: Catastrophic backtracking detection
- **Memory Safety**: Bounds checking and resource limits
- **Fuzzing Support**: Built-in input validation for security testing

## Contributing

The regex module follows CURSED stdlib conventions:

1. **Pure CURSED**: No FFI dependencies
2. **Test Coverage**: Comprehensive test suite with 100% feature coverage
3. **Documentation**: Detailed API documentation and examples
4. **Performance**: Optimized for both correctness and speed
5. **Unicode Support**: Full internationalization support

## License

Part of the CURSED programming language standard library.
