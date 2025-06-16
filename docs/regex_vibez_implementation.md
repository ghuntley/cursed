# RegexVibez Implementation Documentation

## Overview

RegexVibez is a comprehensive regular expression module for the CURSED programming language that provides powerful pattern matching capabilities with a vibez-focused interface. Built on top of Rust's `regex` crate for performance and reliability, it offers both familiar regex functionality and CURSED-specific enhancements.

## Architecture

### Module Structure

```
src/stdlib/regex_vibez/
├── mod.rs              # Main module exports and core functions
├── error.rs            # Error handling and types
├── pattern.rs          # Core VibePattern implementation
├── groups.rs           # Named capture group functionality
├── builder.rs          # Fluent pattern builder interface
├── common.rs           # Pre-compiled common patterns
└── utils.rs            # Utility functions and helpers
```

### Core Components

#### 1. **VibePattern** (`pattern.rs`)
The main regex pattern type that wraps Rust's `Regex` with CURSED-friendly methods:

```rust
pub struct VibePattern {
    regex: Regex,
    pattern: String,
}
```

**Key Features:**
- Compiles regex patterns with error handling
- Supports POSIX regex compilation
- All matching operations (match, find, replace, split)
- Named group extraction and manipulation
- Template-based replacements

**Methods:**
- `match_string(s: &str) -> bool` - Test if string matches
- `find_string(s: &str) -> String` - Find first match
- `find_all_string(s: &str, n: i32) -> Vec<String>` - Find all matches
- `replace_all_string(src: &str, repl: &str) -> String` - Replace matches
- `split(s: &str, n: i32) -> Vec<String>` - Split by pattern

#### 2. **VibeGroups** (`groups.rs`)
Enhanced named capture group functionality:

```rust
pub struct VibeGroups {
    pattern: VibePattern,
}
```

**Key Features:**
- Named group validation and statistics
- Group value extraction and matching
- Multi-match group processing
- Group existence checking

#### 3. **PatternBuilder** (`builder.rs`)
Fluent interface for building complex regex patterns:

```rust
pub struct PatternBuilder {
    pattern: String,
    case_insensitive: bool,
    multiline: bool,
    dot_matches_newline: bool,
    unicode: bool,
}
```

**Key Features:**
- Fluent method chaining
- Built-in pattern templates (email, URL, phone, etc.)
- Quantifier support
- Character class builders
- Configuration options

#### 4. **Common Patterns** (`common.rs`)
Pre-compiled patterns for common use cases:

```rust
lazy_static! {
    pub static ref EMAIL_PATTERN: VibePattern = VibePattern::compile(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).expect("Email pattern should compile");
    // ... more patterns
}
```

**Available Patterns:**
- Email addresses
- URLs (HTTP/HTTPS)
- Phone numbers (US format)
- Dates (YYYY-MM-DD)
- Times (HH:MM:SS)
- IP addresses (IPv4/IPv6)
- Credit cards
- UUIDs
- And many more...

#### 5. **Utilities** (`utils.rs`)
Helper functions for advanced regex operations:

- Pattern validation and analysis
- Performance benchmarking
- String optimization
- Glob pattern conversion
- Text processing utilities

### Error Handling

Comprehensive error handling with `RegexVibesError` enum:

```rust
pub enum RegexVibesError {
    CompilationError(String),    // Invalid regex patterns
    InvalidInput(String),        // Bad input data
    TemplateError(String),       // Template replacement errors
    IndexError(String),          // Index out of bounds
    IoError(String),            // IO operations
    EncodingError(String),      // UTF-8 encoding issues
    GeneralError(String),       // General errors
}
```

Integration with CURSED's error system through `From` trait implementations.

## Usage Examples

### Basic Pattern Matching

```cursed
import "stdlib::regex_vibez";

// Compile a pattern
sus pattern = regex_vibez.compile(r"\d+").unwrap();

// Test matches
lowkey (pattern.match_string("123")) {
    println("Found numbers!");
}

// Find all matches
facts numbers = pattern.find_all_string("I have 123 and 456 items", -1);
// Result: ["123", "456"]
```

### Named Capture Groups

```cursed
// Email pattern with named groups
sus email_pattern = regex_vibez.compile(r"(?P<user>\w+)@(?P<domain>\w+\.\w+)").unwrap();
facts groups = email_pattern.vibe_groups();

facts matches = groups.find_groups_string("admin@example.com");
// matches["user"] = "admin"
// matches["domain"] = "example.com"
```

### Pattern Builder

```cursed
// Build complex patterns fluently
facts url_pattern = regex_vibez.new_pattern_builder()
    .starts_with("")
    .named_group("protocol", "https?")
    .literal("://")
    .named_group("domain", r"[a-zA-Z0-9.-]+")
    .build()
    .unwrap();
```

### Common Patterns

```cursed
// Use pre-compiled patterns
lowkey (regex_vibez.EMAIL_PATTERN.match_string("user@example.com")) {
    println("Valid email!");
}

lowkey (regex_vibez.PHONE_PATTERN.match_string("(555) 123-4567")) {
    println("Valid phone number!");
}
```

## Performance Characteristics

### Compilation Performance
- Pattern compilation is cached when using `VibePattern`
- Common patterns are pre-compiled for immediate use
- Builder patterns compile only once when `build()` is called

### Runtime Performance
- Built on Rust's highly optimized `regex` crate
- O(n) time complexity for most operations
- Efficient memory usage with minimal allocations

### Benchmarking
Built-in benchmarking utilities:

```cursed
facts result = regex_vibez.benchmark_pattern(
    r"\d+", 
    &["test123", "hello456", "world789"], 
    1000
).unwrap();

println(format!("Operations per second: {}", result.operations_per_second));
```

## Integration with CURSED

### Type Conventions
- `tea` (String) for text input/output
- `normie` (i32) for numeric limits and counts
- `lit` (bool) for boolean results
- `Vec<String>` for multiple results
- `HashMap<String, String>` for named groups

### Error Integration
- All errors convert to `CursedError` automatically
- Optional error propagation with `?` operator support
- Panic-safe alternatives (`must_compile` for known-good patterns)

### Memory Safety
- All operations are memory-safe through Rust's ownership system
- No buffer overflows or memory leaks
- Thread-safe operations where applicable

## Testing

### Unit Tests
- Individual component testing in `tests/regex_vibez_unit_test.rs`
- All public APIs covered
- Error condition testing
- Edge case validation

### Integration Tests
- Real-world scenario testing in `tests/regex_vibez_integration_test.rs`
- Email validation workflows
- Log parsing examples
- Data extraction patterns
- Performance validation

### Examples
- Comprehensive demo in `examples/regex_vibez_demo.csd`
- Practical applications in `examples/regex_vibez_practical.csd`
- Real-world usage patterns

## Future Enhancements

### Planned Features
1. **Advanced Pattern Analysis**
   - Pattern complexity analysis
   - Optimization suggestions
   - Performance profiling

2. **Enhanced Template System**
   - Complex template replacements
   - Conditional replacements
   - Template validation

3. **Unicode Improvements**
   - Better Unicode category support
   - International text processing
   - Locale-aware patterns

4. **Streaming Support**
   - Large file processing
   - Streaming pattern matching
   - Memory-efficient operations

### Performance Optimizations
1. **Compilation Caching**
   - Global pattern cache
   - LRU cache for frequently used patterns
   - Persistent cache across runs

2. **Parallel Processing**
   - Multi-threaded pattern matching
   - Parallel text processing
   - Work-stealing for large inputs

## Best Practices

### Pattern Design
1. Use pre-compiled common patterns when possible
2. Validate patterns before use in production
3. Consider pattern complexity for performance-critical code
4. Use named groups for better maintainability

### Error Handling
1. Always handle compilation errors gracefully
2. Use `validate_pattern()` for user input validation
3. Prefer `compile()` over `must_compile()` for untrusted patterns
4. Log regex errors with context for debugging

### Performance
1. Compile patterns once and reuse
2. Use appropriate quantifiers to avoid backtracking
3. Consider pattern alternatives for better performance
4. Benchmark critical regex operations

### Testing
1. Test with both valid and invalid inputs
2. Include edge cases in pattern testing
3. Validate captured groups thoroughly
4. Test with various input sizes

This implementation provides a production-ready regex module that combines the power of Rust's regex engine with CURSED's distinctive syntax and error handling patterns, making regular expressions both powerful and accessible to CURSED developers.
