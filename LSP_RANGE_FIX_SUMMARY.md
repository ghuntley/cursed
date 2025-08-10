# Critical P1 Issue #19: LSP Incremental-Diagnostics Range Fix

## Problem Identified
The LSP server was sending negative or invalid ranges to editors, causing crashes or incorrect error highlighting. The issue occurred due to:

1. **Unsafe casting from `usize` to `u32`** without bounds checking in Position structures
2. **Arithmetic underflow** in lexer position tracking (e.g., `self.position -= 1` when position is 0)
3. **Invalid range calculations** where end positions could be before start positions
4. **No validation** of diagnostic ranges before sending to editors

## Root Causes Fixed

### 1. Lexer Underflow Protection (src-zig/lexer.zig)
- **Line 365-378**: Added bounds checking before position/column decrements
- **Line 485**: Protected string literal start calculation from underflow
- **Changes**: Replaced `self.position -= 1` with `if (self.position > 0) self.position -= 1`

### 2. LSP Range Validation (src-zig/lsp_server.zig)
- **Added**: `tokenPositionToLSP()` function with safe `usize` to `u32` conversion
- **Added**: `createSafeRange()` function with validation to ensure valid ranges
- **Lines 309-342**: Updated diagnostic creation to use safe range functions
- **Enhancement**: Parser errors now use actual token positions when available

### 3. Symbol Range Safety (src-zig/tools/lsp_server.zig) 
- **Added**: Same safe conversion helper functions
- **Lines 484-554**: Updated all symbol range creations to use `createSafeRange()`
- **Protection**: Eliminated direct token position casting throughout the file

## Implementation Details

### Safe Position Conversion
```zig
fn tokenPositionToLSP(token_line: usize, token_column: usize) Position {
    return Position{
        .line = @min(@as(u32, @intCast(@min(token_line, std.math.maxInt(u32)))), std.math.maxInt(u32)),
        .character = @min(@as(u32, @intCast(@min(token_column, std.math.maxInt(u32)))), std.math.maxInt(u32)),
    };
}
```

### Safe Range Creation
```zig
fn createSafeRange(start_line: usize, start_char: usize, end_line: usize, end_char: usize) Range {
    const safe_start = tokenPositionToLSP(start_line, start_char);
    const safe_end = tokenPositionToLSP(end_line, end_char);
    
    // Ensure end is not before start
    if (safe_end.line < safe_start.line or 
        (safe_end.line == safe_start.line and safe_end.character < safe_start.character)) {
        return Range{
            .start = safe_start,
            .end = Position{ .line = safe_start.line, .character = safe_start.character + 1 },
        };
    }
    
    return Range{ .start = safe_start, .end = safe_end };
}
```

### Lexer Protection Example
```zig
// Before (vulnerable to underflow)
self.position -= 1;
self.column -= 1;

// After (safe)
if (self.position > 0) self.position -= 1;
if (self.column > 0) self.column -= 1;
```

## Validation Strategy

1. **Bounds Checking**: All token positions are validated before conversion to LSP positions
2. **Underflow Prevention**: Lexer position arithmetic includes bounds checks
3. **Range Validation**: End positions are guaranteed to be at or after start positions
4. **Overflow Protection**: Large `usize` values are clamped to `u32` maximum values

## Impact Assessment

### Before Fix
- ❌ Editors could receive negative line/character positions
- ❌ Invalid ranges caused editor crashes or rendering issues
- ❌ Arithmetic underflow in lexer caused invalid token positions
- ❌ Direct unsafe casting from `usize` to `u32`

### After Fix
- ✅ All diagnostic ranges are guaranteed to be valid and non-negative
- ✅ Lexer position tracking is protected from underflow
- ✅ Safe conversion handles overflow gracefully
- ✅ Invalid ranges are automatically corrected

## Test Cases Addressed
1. **Zero Position Underflow**: `position - 1` when position is 0
2. **Large Position Overflow**: `usize` values larger than `u32` max
3. **Inverted Ranges**: End position before start position  
4. **Token Position Errors**: Invalid positions from lexer/parser

## Editor Compatibility
This fix ensures compatibility with:
- VS Code and other LSP clients expecting valid ranges
- Editors that crash on negative positions
- Language servers requiring LSP specification compliance

## Status: RESOLVED ✅
- **Root causes identified and fixed**
- **Safe conversion functions implemented**
- **Comprehensive protection added**
- **Ready for testing and deployment**
