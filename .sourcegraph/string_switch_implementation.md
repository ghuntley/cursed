# String Switch Implementation

## Overview

The string switch implementation allows CURSED to support switch statements (`vibe_check`) that use string values in their case expressions. This is a powerful feature that allows for more idiomatic pattern matching in string-based code.

## Implementation Details

The implementation follows a multiphase approach:

### Phase 1: String Comparison Foundation
- Added support for string comparison using C's `strcmp` function
- Implemented `generate_string_comparison` that correctly handles null-terminated strings
- Added string constant creation and management in LLVM IR

### Phase 2: Basic String Switch Structure
- Created the framework for handling string-based switch statements
- Added basic block generation for cases, default, and end blocks
- Implemented the chain of string comparisons that branch to the appropriate case blocks

### Phase 3: Complete Implementation
- Implemented full support for switch statement semantics:
  - Multiple string values per case
  - Proper branch and control flow management
  - Default case handling
  - Break/ghosted statement support for exiting the switch
  - Integration with the existing statement compilation system

## Code Structure

The implementation is primarily located in `src/codegen/llvm/string_switch.rs` and consists of several key components:

1. `generate_string_comparison`: Creates code to compare two strings
2. `evaluate_string_expr`: Evaluates string expressions to get their constant values
3. `compile_string_switch_statement`: Generates the full LLVM IR for the switch structure

## Usage Example

A typical CURSED string switch looks like this:

```
vibe_check day {
    mood "Monday": 
        // Monday specific code
    mood "Tuesday", "Wednesday", "Thursday": 
        // Mid-week code
    mood "Friday": 
        // Friday code
    mood "Saturday", "Sunday": 
        // Weekend code
    basic: 
        // Default case
}
```

## Performance Considerations

String comparison is more expensive than integer comparison. Some potential future optimizations include:

1. String interning (storing strings in a global table) to enable pointer comparison instead of content comparison
2. Hashing strings for faster comparison
3. Jump table optimization for common patterns

## Limitations

Current limitations include:
- No pattern matching or regex support in case values
- Each string comparison requires a separate branch, potentially less efficient than jump tables
- All string literals must be evaluated at compile time

## Testing

The implementation has been tested with various test cases, including:
- Basic string comparisons
- Multiple case values
- Nested switch statements
- Break statements within cases
- Default case handling

All tests pass, demonstrating that the string switch implementation is working correctly.