# CURSED Parser Array Syntax Update Plan

## Current State Analysis

The parser currently supports these array syntaxes:
1. `[]normie` - Slice types (prefix)
2. `[5]normie` - Sized arrays (prefix) 
3. `[][]normie` - Nested slices (prefix)

## Target State: NEW SYNTAX

The new syntax will be:
1. `normie[value]` - Slice types (postfix)
2. `normie[5]` - Sized arrays (postfix)
3. `normie[value][value]` - Nested slices (postfix)

## Key Changes Required

### 1. parseType() Function
- Remove prefix `[` parsing logic from start of function
- Add postfix `[` parsing logic after basic type parsing
- Handle multiple consecutive `[expr]` patterns

### 2. parseCompositeLiteral() Function  
- Change from `[]Type{...}` to `Type[value]{...}` parsing

### 3. Expression vs Type Disambiguation
- `arr[i]` = array access (expression context)
- `normie[5]` = type declaration (type context)

### 4. Error Messages
- Update error messages to reflect new syntax
- Add helpful error for old syntax usage

## Implementation Strategy

1. Update `parseType()` with postfix array parsing
2. Update composite literal parsing
3. Test basic functionality
4. Handle edge cases and error recovery
5. Update tests and documentation
