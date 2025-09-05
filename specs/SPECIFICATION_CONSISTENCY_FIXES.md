# CURSED Language Specification Consistency Resolution

**Document Version**: 1.0  
**Date**: 2025-08-10  
**Status**: CANONICAL - All implementations MUST follow these decisions

## Executive Summary

This document resolves critical inconsistencies in the CURSED language specification by establishing canonical syntax for:

1. **Channel Operations**: Standardized on `dm_send`/`dm_recv`/`dm_close`/`dm_make` functions
2. **Error Handling**: Unified on `yikes`/`fam`/`shook` as primary error handling mechanism  
3. **Control Flow Keywords**: Established canonical keywords with deprecation timeline
4. **Import Syntax**: Defined precise parsing rules for all import variations

## 1. Channel Operations - CANONICAL DECISIONS

### ✅ CANONICAL SYNTAX (Required)
```cursed
// Channel creation
ch := dm_make(type, capacity)    // 0 = unbuffered, >0 = buffered

// Send operation  
dm_send(ch, value)              // Blocking send

// Receive operations
value := dm_recv(ch)            // Blocking receive
value, ok := dm_recv(ch)        // Receive with close check

// Channel management
dm_close(ch)                    // Close channel
```

### ❌ DEPRECATED SYNTAX (Remove in v2.0)
```cursed
// Legacy Go-style operators - FORBIDDEN
dm_send(ch, value                     // DEPRECATED: Use dm_send(ch, value)  
value := dm_recv(ch)                   // DEPRECATED: Use dm_recv(ch)
close(ch)                       // DEPRECATED: Use dm_close(ch)
```

### Implementation Requirements
- ✅ **Parsers MUST**: Implement all `dm_*` functions  
- ⚠️ **Parsers MAY**: Support legacy syntax with deprecation warnings
- ❌ **New Code MUST NOT**: Use legacy Go-style operators

## 2. Error Handling - CANONICAL DECISIONS

### ✅ CANONICAL APPROACH: yikes/fam/shook System

**Error Creation:**
```cursed
// Simple error
yikes "Something went wrong"

// Error in function
slay divide(a normie, b normie) normie yikes {
    ready (b == 0) {
        yikes "Division by zero"
    }
    damn a / b
}
```

**Error Handling:**
```cursed
// Recovery blocks
fam {
    sus result = divide(10, 0) shook  // shook propagates errors
    vibez.spill("Result:", result)
} sus error {
    vibez.spill("Caught error:", error.message())
}
```

**Error Propagation:**
```cursed
// Automatic propagation with shook
slay process_data() yikes {
    sus file = open_file("data.txt") shook
    sus content = read_file(file) shook  
    damn process(content)
}
```

### ❌ DEPRECATED PATTERNS (Remove in v2.0)
```cursed
// DEPRECATED: Go-style tuple returns
slay old_function(x normie) (normie, yikes) { ... }

// DEPRECATED: Result<T, E> patterns  
Result<String> result = operation();
```

### Implementation Requirements
- ✅ **Parsers MUST**: Implement `yikes`/`fam`/`shook` keywords
- ✅ **Runtime MUST**: Support structured error propagation
- ❌ **New Code SHOULD NOT**: Use tuple-style error returns

## 3. Control Flow Keywords - CANONICAL DECISIONS

### ✅ CANONICAL KEYWORDS

| Construct | Canonical Keyword | Alternative | Status |
|-----------|------------------|-------------|---------|
| **If statement** | `ready` | `lowkey` | DEPRECATED |
| **Else clause** | `otherwise` | `highkey` | DEPRECATED | 
| **For loop** | `bestie` | - | CANONICAL |
| **While loop** | `periodt` | `flex` | DEPRECATED |
| **Switch** | `vibe_check` | - | CANONICAL |
| **Case** | `mood` | - | CANONICAL |
| **Default** | `basic` | - | CANONICAL |
| **Break** | `ghosted` | - | CANONICAL |
| **Continue** | `simp` | - | CANONICAL |
| **Return** | `damn` | `yolo` | DEPRECATED |

### Canonical Examples
```cursed
// If/else
ready condition {
    vibez.spill("true branch")
} otherwise {
    vibez.spill("false branch")  
}

// While loop
periodt x > 0 {
    x--
}

// For loop
bestie i := 0; i < 10; i++ {
    ready i == 5 {
        simp      // continue
    }
    ready i == 8 {
        ghosted   // break
    }
    vibez.spill(i)
}

// Switch statement
vibe_check day {
    mood "Monday":
        vibez.spill("Start of week")
    mood "Friday":  
        vibez.spill("End of week")
    basic:
        vibez.spill("Midweek")
}

// Function return
slay add(x, y normie) normie {
    damn x + y
}
```

### Implementation Requirements
- ✅ **Parsers MUST**: Support canonical keywords
- ⚠️ **Parsers MAY**: Support deprecated keywords with warnings
- 📅 **Deprecation Timeline**: Remove deprecated keywords in v2.0 (2026)

## 4. Import Syntax - CANONICAL DECISIONS

### ✅ CANONICAL IMPORT FORMS

**1. Single Import:**
```cursed
yeet "module_name"
```

**2. Multiple Imports (Comma-Separated):**
```cursed
yeet "module1", "module2", "module3"
```

**3. Grouped Imports (Parenthesized, Semicolon-Separated):**
```cursed
yeet (
    "module1"
    "module2"  
    "module3"
)
```

**4. Aliased Import:**
```cursed
yeet "very_long_module_name" as short_name
```

**5. Specific Symbol Imports:**
```cursed
yeet "mathz" { sin, cos, tan }
yeet "stringz" { slice_tea, concat_tea }
```

### Parsing Rules and Precedence

```ebnf
ImportDecl    = "yeet" ( ImportSpec | ImportList | ImportGroup ) .
ImportSpec    = [ identifier | "." ] ImportPath [ "as" identifier ] [ ImportSymbols ] .
ImportList    = ImportPath { "," ImportPath } .
ImportGroup   = "(" { ImportSpec ";" } ")" .
ImportPath    = string_lit .
ImportSymbols = "{" identifier { "," identifier } "}" .
```

### Error Handling for Imports
```cursed
// Valid imports
yeet "mathz"                    // ✅ Single import
yeet "mathz", "stringz"         // ✅ Multiple imports  
yeet "mathz" as math           // ✅ Aliased import
yeet "mathz" { sin, cos }      // ✅ Symbol import

// Invalid imports  
yeet mathz                     // ❌ Missing quotes
yeet "mathz" "stringz"         // ❌ Missing comma
yeet ""                        // ❌ Empty module name
yeet "math z"                  // ❌ Space in module name (unless quoted)
```

### Implementation Requirements
- ✅ **Parsers MUST**: Support all five import forms
- ✅ **Parsers MUST**: Validate module name syntax  
- ✅ **Parsers MUST**: Handle import errors gracefully
- ✅ **Parsers MUST**: Resolve symbol imports at compile time

## 5. Oracle Guidance and Rationale

### Channel Operations Decision
**Oracle Decision**: Function-style operations (`dm_send`/`dm_recv`) chosen over Go-style operators (`<-`) for:
- **Clarity**: Function calls are more explicit than operators
- **Consistency**: Aligns with CURSED's Gen Z naming conventions
- **Tooling**: Easier for IDEs to parse and autocomplete
- **Future-proofing**: Allows for extended parameters (timeouts, priorities)

### Error Handling Decision
**Oracle Decision**: `yikes`/`fam`/`shook` system chosen over Go-style returns for:
- **Language Identity**: Unique to CURSED, not copying Go
- **Ergonomics**: Less verbose than tuple returns
- **Power**: Combines structured errors with panic recovery
- **Performance**: Single error path vs dual return values

### Control Flow Decision
**Oracle Decision**: Canonical keywords standardized based on:
- **Usage Frequency**: Most common keywords in existing codebase
- **Readability**: Clear semantic meaning
- **Consistency**: Follows Gen Z slang theme
- **Migration Path**: Gradual deprecation timeline

### Import Syntax Decision  
**Oracle Decision**: Multiple import forms supported for:
- **Flexibility**: Different styles for different use cases
- **Migration**: Compatibility with existing patterns
- **Tooling**: Each form optimizes different IDE features
- **Ecosystem**: Matches different module organization patterns

## 6. Implementation Timeline

### Phase 1: Immediate (v1.5) - 2025 Q3
- ✅ Add canonical syntax support to all parsers
- ✅ Update standard library to use canonical forms
- ⚠️ Add deprecation warnings for legacy syntax

### Phase 2: Migration (v1.6-1.9) - 2025 Q4 - 2026 Q2  
- 📖 Update all documentation and examples
- 🔧 Provide automated migration tools
- 📊 Track usage metrics for deprecated syntax

### Phase 3: Sunset (v2.0) - 2026 Q3
- ❌ Remove all deprecated syntax support
- 🧹 Clean up parser and runtime code
- 📋 Release final migration guide

## 7. Compliance Requirements

### For Parser Implementations
- ✅ **MUST**: Support all canonical syntax forms
- ✅ **MUST**: Validate syntax according to these specifications  
- ⚠️ **SHOULD**: Warn on deprecated syntax usage
- 📖 **SHOULD**: Provide helpful error messages
- 🧪 **MUST**: Pass compliance test suite

### For Code Generators
- ✅ **MUST**: Generate only canonical syntax
- ❌ **MUST NOT**: Generate deprecated syntax
- 🔄 **SHOULD**: Provide migration from legacy syntax

### For IDEs and Tools
- ✅ **MUST**: Autocomplete canonical syntax first
- ⚠️ **SHOULD**: Mark deprecated syntax as such
- 📖 **SHOULD**: Provide quick-fix suggestions
- 🎨 **MAY**: Syntax highlight deprecated forms differently

## 8. Testing and Validation

### Compliance Test Suite
Located in `specs/tests/compliance/`:
- `channel_operations_compliance.💀` - Tests canonical channel syntax
- `error_handling_compliance.💀` - Tests yikes/fam/shook patterns  
- `control_flow_compliance.💀` - Tests canonical keywords
- `import_syntax_compliance.💀` - Tests all import forms

### Validation Commands
```bash
# Run compliance tests
./zig-out/bin/cursed-zig check specs/tests/compliance/

# Validate migration
./zig-out/bin/cursed-zig migrate --check legacy_code.💀

# Check deprecation warnings  
./zig-out/bin/cursed-zig --warn-deprecated legacy_code.💀
```

## 9. Migration Support

### Automated Migration Tool
```bash
# Migrate from legacy syntax
./zig-out/bin/cursed-zig migrate legacy_file.💀

# Migration with backup
./zig-out/bin/cursed-zig migrate --backup legacy_file.💀

# Dry run migration
./zig-out/bin/cursed-zig migrate --dry-run legacy_file.💀
```

### Migration Patterns
```cursed
// Channel operations migration
dm_send(ch, value           →  dm_send(ch, value)
value := dm_recv(ch)         →  value := dm_recv(ch)  
close(ch)            →  dm_close(ch)

// Error handling migration  
func() (T, error)    →  func() T yikes { ... }
if err != nil        →  fam { ... } sus err { ... }

// Control flow migration
ready condition     →  ready condition
otherwise             →  otherwise  
flex condition       →  periodt condition
yolo value          →  damn value
```

---

## Conclusion

This specification resolves the four critical inconsistencies in CURSED language design:

1. ✅ **Channel Operations**: `dm_send`/`dm_recv`/`dm_close`/`dm_make` as canonical
2. ✅ **Error Handling**: `yikes`/`fam`/`shook` as primary mechanism
3. ✅ **Control Flow**: Canonical keywords established with migration timeline  
4. ✅ **Import Syntax**: Five distinct import forms with precise parsing rules

All implementations MUST follow these decisions to ensure language consistency and ecosystem compatibility.

**Status**: CANONICAL - Effective immediately for all new development
**Next Review**: 2026 Q1 (post-v2.0 release)
