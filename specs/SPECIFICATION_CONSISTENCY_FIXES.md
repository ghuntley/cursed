# CURSED Specification Consistency Fixes

## Executive Summary

This document outlines the critical inconsistencies found between the CURSED language specifications and actual compiler implementations (Rust and Zig), along with the fixes applied to achieve specification consistency.

## Critical Issues Identified and Fixed

### 1. **Operator Precedence - CRITICAL PARSING BUG** ✅ FIXED

**Issue**: Rust parser implementation used flat left-to-right parsing without proper operator precedence.
- Expression `2 + 3 * 4` would incorrectly parse as `(2 + 3) * 4` instead of `2 + (3 * 4)`
- This is a fundamental parsing correctness issue

**Fix Applied**:
- Added explicit precedence requirement to `specs/grammar.md`
- Clarified that parsers MUST implement precedence climbing
- Zig parser already implements this correctly

**Impact**: HIGH - Affects mathematical expression correctness

### 2. **Return Statement Canonicalization** ✅ FIXED

**Issue**: Inconsistent treatment of `damn` vs `yolo` keywords across implementations
- Spec stated both were acceptable with unclear preference

**Fix Applied**:
- Clarified `damn` as CANONICAL return keyword in `specs/grammar.md`
- Marked `yolo` as deprecated with removal warning
- Added parser requirement to prefer `damn` and emit warnings for `yolo`

**Impact**: MEDIUM - Affects language consistency and future migration

### 3. **Boolean Literal Standardization** ✅ FIXED

**Issue**: Ambiguous support for deprecated `cap` literal (formerly false)

**Fix Applied**:
- Marked `based`/`cringe`/`nah` as CANONICAL in `specs/lexical.md`
- Added deprecation section for `cap` and `yolo`
- Required parsers to emit deprecation warnings

**Impact**: MEDIUM - Prevents confusion about canonical syntax

### 4. **Channel Operation Syntax** ✅ FIXED

**Issue**: Both Go-style (`<-`) and function-style (`dm_send`) syntaxes documented

**Fix Applied**:
- Marked function-style operations as CANONICAL in `specs/types.md` and `specs/grammar.md`
- Deprecated Go-style operators with removal warning
- Required new parsers to NOT implement legacy syntax

**Impact**: HIGH - Critical for concurrency correctness and consistency

### 5. **While Statement Keywords** ✅ FIXED

**Issue**: Missing `flex` keyword in grammar specification despite implementation support

**Fix Applied**:
- Updated `specs/grammar.md` to accept both `periodt` and `flex` for while statements
- Added compatibility note

**Impact**: LOW - Documentation accuracy improvement

## Specification Files Updated

1. **`specs/grammar.md`**:
   - Added operator precedence requirements
   - Canonicalized return statement syntax
   - Updated while statement grammar
   - Deprecated channel operators

2. **`specs/lexical.md`**:
   - Canonicalized boolean literals
   - Added deprecation section
   - Added parser requirements for warnings

3. **`specs/types.md`**:
   - Canonicalized channel operations
   - Deprecated legacy Go-style syntax
   - Added parser requirements

## Parser Implementation Requirements

### Immediate Actions Required

1. **Rust Parser**:
   - **CRITICAL**: Fix operator precedence parsing to use precedence climbing
   - Remove support for deprecated `<-` channel operators
   - Emit warnings for `yolo` and `cap` usage

2. **Zig Parser**:
   - Already largely specification-compliant
   - Ensure deprecation warnings are implemented
   - Verify channel operation parsing matches specification

### Compliance Testing

Create test cases to verify:
- Operator precedence: `2 + 3 * 4 == 14` (not 20)
- Return statement preferences: `damn` accepted, `yolo` warns
- Channel operations: `dm_send`/`dm_recv` work, `<-` rejected
- Boolean literals: `based`/`cringe` accepted, `cap` warns

## Future Specification Maintenance

1. **Version Compatibility**: Deprecated features will be removed in major version updates
2. **Parser Validation**: New parsers must pass specification compliance tests
3. **Breaking Changes**: All breaking changes must be documented with migration paths
4. **Oracle Consultation**: Use external standards for ambiguous language design decisions

## Implementation Priority

**P0 (Critical)**: Operator precedence fix in Rust parser
**P1 (High)**: Channel operation syntax consistency
**P2 (Medium)**: Return statement and boolean literal canonicalization
**P3 (Low)**: While statement documentation updates

This specification consistency effort ensures both CURSED compiler implementations follow identical language semantics and prevents divergent language evolution.
