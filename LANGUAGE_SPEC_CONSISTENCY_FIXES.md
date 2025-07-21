# CURSED Language Specification Consistency Fixes

## Summary

Fixed inconsistencies in the CURSED language specifications to ensure consistent syntax and keyword usage across all documentation.

## Changes Made

### 1. Boolean Type Standardization
**File**: `specs/types.md`
- **Fixed**: Changed boolean description from `based` (true) or `cap` (false) to `based` (true) or `cringe` (false)
- **Reason**: Standardized to use `cringe` for false consistently across all specs

### 2. Comment Syntax Standardization  
**File**: `specs/tree-sitter-grammar.md`
- **Fixed**: Changed block comment syntax from `no cap ... on god` to `/* cap ... cap */`
- **Reason**: Unified with the syntax specified in `specs/lexical.md` for consistency

### 3. Goroutine Keyword Cleanup
**File**: `specs/lexical.md`
- **Fixed**: Removed `yolo` from goroutine keyword mapping, kept only `stan`
- **Changed**: `| go | stan / yolo |` → `| go | stan |`
- **Reason**: `stan` is the standard goroutine keyword, `yolo` was causing confusion

### 4. Missing Keywords Addition
**File**: `specs/lexical.md`
- **Added**: `| panic | shook |` - Error/panic handling keyword
- **Added**: `| recover | fam |` - Panic recovery keyword
- **Reason**: These keywords were referenced in other specs but missing from the main keyword table

## Verification

All specifications now consistently use:
- ✅ `based` (true) and `cringe` (false) for boolean values
- ✅ `/* cap ... cap */` for block comments and `fr fr` for line comments
- ✅ `stan` as the sole goroutine keyword (not `yolo`)
- ✅ Complete keyword mapping including `shook` (panic) and `fam` (recover)

## Files Modified

1. `specs/types.md` - Boolean type description fixed
2. `specs/tree-sitter-grammar.md` - Block comment syntax standardized
3. `specs/lexical.md` - Goroutine keywords cleaned up, missing keywords added

## Impact

These changes ensure that:
- Language implementers have consistent specifications to follow
- Documentation is internally consistent across all files
- Boolean, comment, and keyword usage is standardized
- No missing keywords in the lexical specification
