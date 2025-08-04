# CURSED Language v1.0 - Breaking Changes

## Overview

This document outlines the breaking changes made to resolve critical language specification inconsistencies. These changes establish canonical forms for core language features and ensure consistency across all specifications.

## Breaking Changes

### 1. Map Type Syntax Standardization

**BREAKING**: Map type syntax changed from `vibes[K]V` to `map[K]V`

**Before:**
```cursed
sus cache vibes[tea]normie = {}
be_like UserData squad {
    sessions vibes[tea]*Session
}
```

**After:**
```cursed
sus cache map[tea]normie = {}
be_like UserData squad {
    sessions map[tea]*Session
}
```

**Migration:** Replace all instances of `vibes[K]V` with `map[K]V`

### 2. Return Keyword Canonicalization

**BREAKING**: `yolo` is deprecated, `damn` is now the canonical return keyword

**Before:**
```cursed
slay add(x, y normie) normie {
    yolo x + y
}

slay process() {
    yolo // empty return
}
```

**After:**
```cursed
slay add(x, y normie) normie {
    damn x + y
}

slay process() {
    damn // empty return
}
```

**Migration:** Replace all instances of `yolo` with `damn` in return statements

### 3. Error Handling Pattern Updates

**BREAKING**: Error comparisons use `nah` instead of `cringe`

**Before:**
```cursed
result, err := doSomething()
lowkey err != cringe {
    handleError(err)
}
```

**After:**
```cursed
result, err := doSomething()
lowkey err != nah {
    handleError(err)
}
```

**Migration:** Update error nil checks to use `nah`

## Specification Files Updated

- **`specs/types.md`**: Updated map syntax from `vibes[K]V` to `map[K]V`, corrected nil values
- **`specs/lexical.md`**: Removed `yolo` from return keywords, standardized map keyword
- **`specs/grammar.md`**: Updated all return statement examples, fixed error handling patterns

## Compatibility Impact

### High Impact
- **Map declarations**: All existing map type declarations need updating
- **Return statements**: All `yolo` returns need migration to `damn`

### Medium Impact  
- **Error handling**: Existing error nil checks using `cringe` need updating
- **Type aliases**: Map-based type definitions require syntax updates

### Low Impact
- **Language tooling**: Parsers and lexers need keyword table updates
- **Documentation**: Examples throughout codebase need consistency updates

## Migration Checklist

- [ ] Update all map type declarations: `vibes[K]V` → `map[K]V`
- [ ] Replace return statements: `yolo` → `damn`
- [ ] Fix error nil checks: `!= cringe` → `!= nah`
- [ ] Update type aliases using map syntax
- [ ] Verify parser/lexer keyword tables
- [ ] Update documentation and examples

## Rationale

These decisions were made based on actual usage analysis:

- **Map syntax**: 557 usages of `map[K]V` vs 6 usages of `vibes[K]V`
- **Return keyword**: 5089 usages of `damn` vs 113 usages of `yolo`  
- **Nil representation**: 271 consistent usages of `nah` for nil values

The canonical forms reflect the established patterns in the existing codebase and provide consistency for future development.

## Version Compatibility

- **v0.x**: Mixed usage, inconsistent specifications
- **v1.0+**: Canonical forms enforced, specifications consistent

This represents the stabilization of CURSED language core syntax for the v1.0 release.
