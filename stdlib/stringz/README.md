# stringz Module - CURSED String Operations

The `stringz` module provides comprehensive string manipulation functions for the CURSED programming language, equivalent to Go's `strings` package.

## Core Functions

### String Properties

- `length(s tea)` - Calculate string length
- `is_empty(s tea)` - Check if string is empty
- `equals(a tea, b tea)` - Compare string equality

### String Access

- `char_at(s tea, index normie)` - Get character at position
- `substring(s tea, start normie, length normie)` - Extract substring

### String Manipulation

- `concat(a tea, b tea)` - Concatenate two strings
- `trim(s tea)` - Remove leading/trailing whitespace
- `replace(s tea, old tea, new tea)` - Replace substring

### String Search

- `contains(s tea, substr tea)` - Check if string contains substring
- `split(s tea, delimiter tea)` - Split string by delimiter
- `join(parts [tea], separator tea)` - Join string array

### Case Conversion

- `to_lower(s tea)` - Convert to lowercase
- `to_upper(s tea)` - Convert to uppercase

### Legacy Aliases

- `string_length(s tea)` - Alias for length()
- `string_concat(a tea, b tea)` - Alias for concat()

## Example Usage

```cursed
yeet "stringz"

slay main() {
    sus text tea = "Hello, World!"
    
    fr fr Get string length
    sus len normie = stringz.length(text)
    vibez.spillf("Length: %d", len)
    
    fr fr Check if contains substring
    lowkey stringz.contains(text, "World") {
        vibez.spill("Found 'World'!")
    }
    
    fr fr Extract substring
    sus greeting tea = stringz.substring(text, 0, 5)
    vibez.spillf("Greeting: %s", greeting)
    
    fr fr Concatenate strings
    sus result tea = stringz.concat("Hello", " CURSED")
    vibez.spill(result)
    
    fr fr Split string
    sus parts [tea] = stringz.split("a,b,c", ",")
    vibez.spillf("Parts: %d", len(parts))
}
```

## Implementation Details

- Pure CURSED implementation using runtime helpers
- Efficient character-by-character string processing
- Bounds checking for all string operations
- Memory-safe substring extraction
- Unicode-aware string handling (planned)

## Current Limitations

This is a basic implementation with simplified functionality:

- Case conversion returns original string (ASCII conversion planned)
- Split returns single-element array (full implementation planned)
- Join returns first element only (full implementation planned)
- Trim returns original string (whitespace trimming planned)

## Runtime Dependencies

The module uses these runtime helper functions:

- `runtime_string_char_at(s tea, index normie)` - Low-level character access
- `runtime_char_to_string(c sip)` - Character to string conversion

## Testing

Run tests with:
```bash
cargo run --bin cursed stdlib/stringz/test_stringz.csd
```

The test suite covers string length, concatenation, character access, substring extraction, equality, and contains operations.

## Future Enhancements

- Full Unicode support
- Advanced pattern matching
- Regular expression integration
- Locale-aware case conversion
- Performance optimizations for large strings
