# CURSED Language Specification Decisions

## Breaking Changes Resolved

### 1. Map Type Syntax: **`map[K]V` CANONICAL**

**Decision**: Use `map[K]V` syntax for all map type declarations.

**Rationale**: 
- Analysis shows **557 usages** of `map[K]V` vs only **6 usages** of `vibes[K]V` 
- The vast majority of existing code uses `map[K]V`
- `vibes[K]V` was only used in runtime_core module documentation
- Consistency with existing codebase is paramount

**Examples**:
```cursed
sus user_data map[tea]normie = {}
sus config map[tea]tea = {"debug": "true"}
be_like UserCache squad {
    users map[normie]*User
    sessions map[tea]*Session
}
```

### 2. Return Keywords: **`damn` CANONICAL**

**Decision**: `damn` is the canonical return keyword. `yolo` is deprecated.

**Rationale**:
- Analysis shows **5089 usages** of `damn` vs **113 usages** of `yolo`
- `damn` usage is 45x more common in the codebase
- `yolo` appears mostly in specific contexts (goroutine spawning, error handling)
- Better aligns with CURSED's expressive nature

**Examples**:
```cursed
slay add(x, y normie) normie {
    damn x + y
}

slay get_user(id normie) User {
    lowkey id < 0 {
        damn User{}  // empty user
    }
    damn users[id]
}
```

### 3. Nil Representation: **`nah` CANONICAL**

**Decision**: `nah` is the canonical nil representation.

**Rationale**:
- Analysis shows **271 consistent usages** of `nah` for nil checks and assignments
- Used correctly throughout conditional expressions: `nah condition {}`
- No evidence of `based` being used for nil (it's boolean true)
- Fits CURSED's slang aesthetic perfectly

**Examples**:
```cursed
sus ptr @User = nah
lowkey ptr == nah {
    vibez.spill("Pointer is nil")
}

// Nil checks in conditionals
nah file_exists(path) {
    damn cap  // file doesn't exist
}
```

## Updated Language Elements

### Map Types
- **Syntax**: `map[KeyType]ValueType`
- **Zero value**: `nah`
- **Declaration**: `sus cache map[tea]normie = {}`
- **Type alias**: `be_like UserMap map[normie]*User`

### Return Statements  
- **Primary keyword**: `damn`
- **Usage**: `damn expression`
- **Empty return**: `damn` (function with no return value)

### Nil Values
- **Literal**: `nah`
- **Pointer nil**: `@T = nah`
- **Slice nil**: `[]T = nah`
- **Map nil**: `map[K]V = nah`
- **Channel nil**: `dm<T> = nah`

## Files Updated

This decision affects the following specification files:
- `specs/types.md` - Map syntax, nil values
- `specs/lexical.md` - Keywords and literals
- `specs/grammar.md` - Return statements, nil usage

The canonical forms are now consistently defined across all specifications.
