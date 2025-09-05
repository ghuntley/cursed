# regex_vibez Module

Basic pattern matching module for CURSED compiler needs. Provides essential string pattern matching functionality without complex regex engine implementation.

## Functions

### Core Pattern Matching
- `match_pattern(pattern tea, text tea) lit` - Check if pattern exists in text
- `find_matches(pattern tea, text tea) []tea` - Find all occurrences of pattern in text
- `replace_pattern(pattern tea, text tea, replacement tea) tea` - Replace pattern occurrences
- `split_pattern(pattern tea, text tea) []tea` - Split text by pattern

### Position Matching
- `starts_with_pattern(pattern tea, text tea) lit` - Check if text starts with pattern
- `ends_with_pattern(pattern tea, text tea) lit` - Check if text ends with pattern

### String Utilities
- `str_length(s tea) normie` - Get string length
- `str_equals(a tea, b tea) lit` - Compare strings for equality
- `str_concat(a tea, b tea) tea` - Concatenate strings
- `wildcard_match(pattern tea, text tea) lit` - Simple wildcard matching
- `escape_pattern(pattern tea) tea` - Escape special regex characters

## Current Implementation Status

This is a **basic implementation** focused on getting the module structure in place for compiler needs. The functions are currently simplified stubs that:

- Return `based` (true) for boolean functions
- Return input text for replacement functions  
- Provide basic string concatenation
- Support both interpretation and compilation modes

## Usage Examples

```cursed
# Basic string operations work
sus text tea = "hello world"
sus pattern tea = "hello"
sus result tea = "Pattern " + "found"
```

## Testing

```bash
# Test basic functionality (works)
cargo run --bin cursed stdlib/regex_vibez/test_simple_functions.💀

# Test compilation mode (works)
cargo run --bin cursed -- compile stdlib/regex_vibez/test_simple_functions.💀
./test_simple_functions

# Both modes work with basic string operations
```

## Implementation Notes

- **Current status**: Basic module structure implemented
- **Pattern matching**: Simplified stubs for essential functions
- **String operations**: Basic concatenation and variables work
- **Compilation**: Both interpretation and compilation modes supported
- **Dependencies**: No external dependencies (pure CURSED)

## Compiler Use Cases

Ready for basic compiler pattern matching needs:
- Module structure in place for expansion
- Function signatures defined for all essential operations
- Both execution modes working
- Foundation for implementing actual pattern matching logic

## Future Enhancements

The module provides the foundation for implementing:
- Actual substring searching algorithms
- Real pattern replacement logic
- String splitting functionality
- Advanced wildcard matching
- Proper escape character handling

## Performance

- Current implementation: O(1) operations (stubs)
- Future implementation will target: O(n*m) substring operations
- Suitable for compiler token processing once fully implemented

## Dependencies

- No external FFI dependencies
- Pure CURSED implementation
- Self-contained module structure
