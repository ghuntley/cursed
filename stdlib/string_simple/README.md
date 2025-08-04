# Simple String Operations (string_simple)

The `string_simple` module provides basic string manipulation functions for CURSED programs.

## Purpose

This module implements essential string operations including searching, manipulation, formatting, and conversion functions with a focus on simplicity and performance.

## Main Functions

### Basic Operations
- `string_simple.len(str)` - Get string length
- `string_simple.empty(str)` - Check if string is empty
- `string_simple.equals(str1, str2)` - Compare strings for equality
- `string_simple.compare(str1, str2)` - Compare strings lexicographically
- `string_simple.concat(str1, str2)` - Concatenate two strings
- `string_simple.repeat(str, count)` - Repeat string N times

### Case Operations
- `string_simple.to_upper(str)` - Convert to uppercase
- `string_simple.to_lower(str)` - Convert to lowercase
- `string_simple.capitalize(str)` - Capitalize first letter
- `string_simple.title_case(str)` - Convert to title case

### Search and Replace
- `string_simple.contains(str, substr)` - Check if contains substring
- `string_simple.starts_with(str, prefix)` - Check if starts with prefix
- `string_simple.ends_with(str, suffix)` - Check if ends with suffix
- `string_simple.find(str, substr)` - Find first occurrence index
- `string_simple.find_last(str, substr)` - Find last occurrence index
- `string_simple.replace(str, old, new)` - Replace all occurrences
- `string_simple.replace_first(str, old, new)` - Replace first occurrence

### Trimming and Padding
- `string_simple.trim(str)` - Remove leading/trailing whitespace
- `string_simple.trim_left(str)` - Remove leading whitespace
- `string_simple.trim_right(str)` - Remove trailing whitespace
- `string_simple.pad_left(str, width, char)` - Pad on left side
- `string_simple.pad_right(str, width, char)` - Pad on right side
- `string_simple.center(str, width, char)` - Center string with padding

### Splitting and Joining
- `string_simple.split(str, delimiter)` - Split string into array
- `string_simple.split_lines(str)` - Split by newlines
- `string_simple.join(array, separator)` - Join array into string
- `string_simple.slice(str, start, end)` - Extract substring

## Usage Examples

### Basic String Operations

```cursed
yeet "string_simple"

sus name tea = "CURSED"
sus greeting tea = "Hello, "

fr fr Basic operations
vibez.spillf("Length: {}", string_simple.len(name))
vibez.spillf("Empty: {}", string_simple.empty(name))
vibez.spillf("Combined: {}", string_simple.concat(greeting, name))

fr fr Case operations
vibez.spillf("Lowercase: {}", string_simple.to_lower(name))
vibez.spillf("Capitalized: {}", string_simple.capitalize("hello world"))
```

### String Search and Manipulation

```cursed
yeet "string_simple"

sus text tea = "The quick brown fox jumps over the lazy dog"

fr fr Search operations
if string_simple.contains(text, "fox") {
    vibez.spill("Found fox!")
}

sus fox_index = string_simple.find(text, "fox")
vibez.spillf("Fox is at index: {}", fox_index)

fr fr Replace operations
sus new_text = string_simple.replace(text, "fox", "cat")
vibez.spillf("Modified: {}", new_text)

fr fr Extract substring
sus quick = string_simple.slice(text, 4, 9)
vibez.spillf("Extracted: '{}'", quick)
```

### String Formatting and Padding

```cursed
yeet "string_simple"

sus number tea = "42"
sus label tea = "Answer"

fr fr Padding examples
sus padded_number = string_simple.pad_left(number, 5, "0")
vibez.spillf("Padded number: '{}'", padded_number)  # "00042"

sus centered_label = string_simple.center(label, 15, "-")
vibez.spillf("Centered: '{}'", centered_label)  # "-----Answer-----"

fr fr Trimming whitespace
sus messy tea = "  \t  spaced out  \n  "
sus clean = string_simple.trim(messy)
vibez.spillf("Cleaned: '{}'", clean)
```

### Splitting and Joining

```cursed
yeet "string_simple"

sus csv_data tea = "apple,banana,orange,grape"
sus fruits = string_simple.split(csv_data, ",")

vibez.spill("Fruits:")
bestie fruit in fruits {
    vibez.spillf("  - {}", fruit)
}

fr fr Join with different separator
sus pipe_separated = string_simple.join(fruits, " | ")
vibez.spillf("Pipe separated: {}", pipe_separated)

fr fr Split text into lines
sus multi_line tea = "Line 1\nLine 2\nLine 3"
sus lines = string_simple.split_lines(multi_line)
vibez.spillf("Found {} lines", lines.len())
```

### String Comparison and Validation

```cursed
yeet "string_simple"

sus str1 tea = "apple"
sus str2 tea = "banana"

fr fr Comparison operations
sus comparison = string_simple.compare(str1, str2)
if comparison < 0 {
    vibez.spillf("'{}' comes before '{}'", str1, str2)
}

fr fr Prefix and suffix checking
sus filename tea = "document.pdf"
if string_simple.ends_with(filename, ".pdf") {
    vibez.spill("PDF file detected")
}

if string_simple.starts_with(filename, "doc") {
    vibez.spill("Starts with 'doc'")
}
```

## Compilation Examples

### Interpretation Mode
```bash
echo 'yeet "string_simple"
sus result = string_simple.to_upper("hello")
vibez.spillf("Result: {}", result)' > string_test.csd

./cursed-unified string_test.csd
```

### Compilation Mode
```bash
./cursed-unified --compile string_test.csd
./string_test
```

## Performance Optimizations

### Efficient String Building

```cursed
yeet "string_simple"

fr fr For multiple concatenations, collect parts first
sus parts []tea = []
parts.push("Hello")
parts.push(" ")
parts.push("CURSED")
parts.push(" ")
parts.push("World")

sus result = string_simple.join(parts, "")
vibez.spillf("Built string: {}", result)
```

## Implementation Notes

- UTF-8 string handling throughout
- Zero-copy operations where possible
- Memory-efficient for large strings
- Thread-safe for concurrent access
- Pure CURSED implementation (no FFI)

## Dependencies

- `memory` - For string memory management
- Core string type from runtime
- No external dependencies

## Performance Considerations

- Lazy evaluation for chained operations
- Efficient memory allocation for concatenation
- Optimized search algorithms (Boyer-Moore for long patterns)
- Minimal copying for slice operations

## Best Practices

1. **Use string builders** for multiple concatenations
2. **Prefer slicing** over copying for substrings
3. **Cache length calculations** for repeated use
4. **Use appropriate search methods** for different patterns
5. **Consider case-insensitive operations** when needed
6. **Validate input strings** for expected formats
7. **Use trim operations** to clean user input

## Common Patterns

### String Validation
```cursed
slay validate_email(email tea) lit {
    damn string_simple.contains(email, "@") && 
         string_simple.contains(email, ".") &&
         !string_simple.starts_with(email, "@") &&
         !string_simple.ends_with(email, ".")
}
```

### String Cleaning
```cursed
slay clean_input(input tea) tea {
    sus trimmed = string_simple.trim(input)
    sus normalized = string_simple.to_lower(trimmed)
    damn string_simple.replace(normalized, "  ", " ")
}
```
