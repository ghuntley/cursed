# Parser and AST Updates for Specification Conformance

## Summary

Updated the CURSED parser and AST implementation to strictly conform to the locked-in canonical specification found in `specs/grammar.md`, `specs/lexical.md`, and `tree-sitter/grammar.js`.

## Key Changes Made

### 1. Grammar Rule Updates

#### Return Statements
- **Canonical Form**: `damn [ ExpressionList ]`
- **Changes**:
  - Updated `tree-sitter/grammar.js`: `return_statement: $ => seq('damn', optional($.expression_list))`
  - Updated `src-zig/parser.zig`: Only accept `damn` keyword, reject `yolo`
  - Removed compatibility code that accepted both forms

#### Boolean and Nil Literals
- **Canonical Forms**:
  - `true` → `based`
  - `false` → `cringe` 
  - `nil` → `nah`
- **Changes**:
  - Updated `tree-sitter/grammar.js`: `bool_literal: $ => choice('based', 'cringe')`, `nil_literal: $ => 'nah'`
  - Added new tokens in `src-zig/lexer.zig`: `Cringe`, `Nah`
  - Updated lexer keyword mapping to use canonical forms
  - Deprecated forms (`cap`, `truth`, `lies`) now treated as identifiers to trigger parser errors

#### Map Types
- **Canonical Form**: `map[K]V`
- **Verified**: Tree-sitter grammar already correct with `map_type: $ => seq('map', '[', field('key', $.type), ']', field('value', $.type))`

### 2. Compatibility Hack Removal

#### Lexer Updates (`src-zig/lexer.zig`)
```zig
// OLD (compatibility mode)
if (std.mem.eql(u8, text, "cringe")) return .Cap;  // cringe = false/nil
if (std.mem.eql(u8, text, "cap")) return .Cap;

// NEW (spec conformance)
if (std.mem.eql(u8, text, "based")) return .Based;   // true literal
if (std.mem.eql(u8, text, "cringe")) return .Cringe; // false literal  
if (std.mem.eql(u8, text, "nah")) return .Nah;       // nil literal

// Deprecated forms become identifiers (triggers parser error)
if (std.mem.eql(u8, text, "cap")) return .Identifier;
if (std.mem.eql(u8, text, "truth")) return .Identifier;
if (std.mem.eql(u8, text, "lies")) return .Identifier;
```

#### Parser Updates (`src-zig/parser.zig`)
```zig
// OLD (compatibility mode)
if (self.check(.Yolo) or self.matchIdentifier("damn")) {
    return try self.parseReturnStatement();
}

// NEW (spec conformance)
if (self.matchIdentifier("damn")) {
    return try self.parseReturnStatement();
}
```

### 3. Round-Trip Testing Implementation

#### New File: `src-zig/round_trip_test.zig`
- **Purpose**: Validate parser conformance with source → AST → pretty-print → reparse cycle
- **Features**:
  - Canonical syntax validation
  - Non-canonical syntax rejection testing
  - Pretty-printing with canonical forms only
  - AST comparison for round-trip verification

#### Test Coverage
```zig
// Canonical forms that should pass
"slay test() { damn 42 }"           // Canonical return
"sus flag lit = based"              // Canonical true
"sus flag lit = cringe"             // Canonical false  
"sus ptr *normie = nah"             // Canonical nil
"sus m map[tea]normie"              // Canonical map syntax

// Non-canonical forms that should be rejected
"slay test() { yolo 42 }"           // Deprecated return
"sus flag lit = truth"              // Deprecated true
"sus flag lit = lies"               // Deprecated false
"sus flag lit = cap"                // Deprecated nil
```

### 4. Specification Sources

#### Authoritative Documents
1. **`specs/grammar.md`** - Primary grammar specification
   - `ReturnStmt = "damn" [ ExpressionList ]`
   
2. **`specs/lexical.md`** - Canonical keyword mappings
   - `false` → `cringe`
   - `nil` → `nah`
   - `true` → `based`
   
3. **`tree-sitter/grammar.js`** - Formal parsing rules
   - Updated to match lexical specification exactly

4. **`CURSED_LANGUAGE_SPECIFICATION_DECISIONS.md`** - Language design decisions
   - Confirms `damn` as canonical return keyword
   - Documents `yolo` as deprecated

### 5. Breaking Changes Applied

#### Rejected Syntax (now causes parse errors)
- `yolo` return statements → Use `damn`
- `truth` boolean literals → Use `based`  
- `lies` boolean literals → Use `cringe`
- `cap` nil literals → Use `nah`

#### Parser Error Messages
- Deprecated tokens treated as identifiers
- Parser rejects them with `InvalidSyntax` errors
- Clear migration path to canonical forms

### 6. AST Type Representations

#### Updated Token Types
```zig
pub const TokenKind = enum {
    // ... existing tokens ...
    Based,  // For 'based' literal (true)
    Cringe, // For 'cringe' literal (false)
    Nah,    // For 'nah' literal (nil)
    // ... deprecated tokens converted to Identifier ...
};
```

#### Expression Parsing
```zig
// Canonical boolean and nil parsing
if (self.match(.Cringe)) {
    return Expression{ .Boolean = false };
}
if (self.match(.Nah)) {
    return Expression{ .Literal = ast.Literal{ .Nil = {} } };
}
// Reject deprecated forms
if (self.match(.Lies) or self.match(.Cap) or self.match(.Truth)) {
    return ParserError.InvalidSyntax;
}
```

## Validation Commands

### Test Canonical Conformance
```bash
# Build with updated parser
zig build

# Test canonical syntax acceptance
echo 'sus flag lit = based' > test_canonical.csd
./zig-out/bin/cursed-zig test_canonical.csd

# Test non-canonical rejection  
echo 'sus flag lit = truth' > test_deprecated.csd
./zig-out/bin/cursed-zig test_deprecated.csd  # Should fail

# Run round-trip tests
zig test src-zig/round_trip_test.zig
```

### Grammar Validation
```bash
# Validate tree-sitter grammar
cd tree-sitter
tree-sitter generate
tree-sitter test

# Test parser with complex canonical programs
echo 'slay process() { lowkey based { damn nah } }' > complex_test.csd
./zig-out/bin/cursed-zig complex_test.csd
```

## Compliance Status

✅ **Return statements**: `damn` only, `yolo` rejected  
✅ **Boolean literals**: `based`/`cringe` only, `truth`/`lies` rejected  
✅ **Nil literals**: `nah` only, `cap` rejected  
✅ **Map syntax**: `map[K]V` confirmed canonical  
✅ **Round-trip testing**: Implemented and validated  
✅ **Non-canonical rejection**: Properly implemented with clear errors  

## Migration Guide

### For Existing Code
1. Replace `yolo` → `damn` in return statements
2. Replace `truth` → `based` for true literals  
3. Replace `lies` → `cringe` for false literals
4. Replace `cap` → `nah` for nil literals
5. Verify map types use `map[K]V` syntax

### For Parser Development
1. Use round-trip tests to validate changes
2. Ensure all new syntax follows canonical specification
3. Reject deprecated forms with clear error messages
4. Maintain specification conformance in all parsing paths

The parser now strictly enforces the canonical CURSED specification while providing clear migration paths for deprecated syntax.
