# STDLIB Error Handling Standardization Plan

## Current State Analysis

After auditing all stdlib modules, I've identified the following inconsistent error patterns:

### ✅ **Already Standardized (Using yikes/fam/shook)**
- `error_management` - Uses proper yikes<T> return types
- `networkz` - Uses yikes<T> for all error-returning functions  
- `enterprise_security` - Proper yikes/fam error handling
- `tlsz` - Consistent yikes<T> return types
- `cloudz` - Uses yikes<T> with proper fam blocks
- `deploymentz` - Consistent error handling pattern
- `kubernetesz` - Uses yikes<T> return pattern
- `chadlogging` - Proper yikes<T> implementation

### ❌ **Need Standardization (Using inconsistent patterns)**

#### **Tuple Returns Pattern**
- `filez` - Returns bare types, prints errors instead of propagating
- `mathz` - Returns 0 or sentinel values on error instead of yikes
- `stringz` - Mixed patterns, some functions return empty strings on error
- `arrayz` - Array bounds errors handled inconsistently
- `vibez` - Basic printing module, minimal error handling needed

#### **Result<T,E> Style References**
- Several modules reference Result<T,E> in documentation but implement differently
- Mixed between traditional error handling and CURSED patterns

#### **String/Print Error Patterns**
- Many modules use `vibez.spill()` to print errors instead of returning them
- `zip_zilla` - Returns error strings instead of proper error objects
- File I/O modules often print instead of propagating errors

#### **Legacy Error Handling**
- Some modules return sentinel values (0, -1, empty strings) instead of proper errors
- Inconsistent null/cringe handling

## Standardization Strategy

### **Phase 1: Define Standard Error Pattern**
1. All error-returning functions MUST use `yikes<T>` return type
2. All error handling MUST use `fam { when condition -> action }` blocks  
3. Error propagation MUST use `shook` operator for chaining
4. No sentinel values, tuple returns, or print-only error handling

### **Phase 2: Module-by-Module Standardization**
Priority order based on criticality and usage:

**High Priority (Core Infrastructure):**
1. `filez` - File operations
2. `mathz` - Mathematical operations  
3. `stringz` - String manipulation
4. `arrayz` - Array operations
5. `vibez` - I/O operations

**Medium Priority (Common Modules):**
6. `jsonz` - JSON handling
7. `timez` - Time operations
8. `ioz` - Advanced I/O
9. `cryptz` - Cryptography
10. `procesz` - Process management

**Lower Priority (Specialized):**
11. All other modules in alphabetical order

### **Phase 3: Update Error Types**
- Standardize on `error_management.managed_error` type
- Create module-specific error categories where needed
- Ensure consistent error codes and messages

### **Phase 4: Documentation and Testing**
- Update all documentation to show standard patterns
- Create comprehensive error handling test suite
- Add error scenario tests for all modules

## Standard Error Pattern Template

```cursed
// ❌ OLD (Multiple inconsistent patterns)
slay old_function(input tea) tea {
    ready (input == "") {
        vibez.spill("Error: empty input")  // Print instead of return
        damn ""                            // Sentinel value
    }
    damn process_input(input)
}

slay old_function_tuple(input tea) (tea, tea) {
    ready (input == "") {
        damn ("", "empty input error")     // Tuple return
    }
    damn (process_input(input), "")
}

// ✅ NEW (Standard CURSED pattern)
slay new_function(input tea) yikes<tea> {
    ready (input == "") {
        yikes "empty input provided"      // Proper error return
    }
    damn process_input(input)
}

// Usage with proper error handling
sus result tea = new_function(user_input) fam {
    when "empty input provided" -> {
        vibez.spill("Please provide valid input")
        damn
    }
    when _ -> {
        vibez.spill("Unexpected error occurred")
        damn
    }
}
```

## Implementation Plan

### **Automated Refactoring Scripts**
1. `standardize_returns.sh` - Convert return patterns to yikes<T>
2. `standardize_error_handling.sh` - Convert error handling to fam blocks
3. `remove_sentinel_values.sh` - Replace sentinel returns with proper errors
4. `validate_error_patterns.sh` - Verify all modules follow standard

### **Quality Assurance**
1. Add linting rules to enforce error handling patterns
2. Create comprehensive error handling test suite
3. Update build system to validate error handling compliance
4. Documentation review for all standardized modules

## Timeline

- **Week 1**: Complete high-priority modules (filez, mathz, stringz, arrayz, vibez)
- **Week 2**: Medium-priority modules (jsonz, timez, ioz, cryptz, procesz) 
- **Week 3**: Remaining modules + documentation updates
- **Week 4**: Testing, validation, and quality assurance

## Benefits

1. **Consistent Developer Experience**: All stdlib modules follow same error pattern
2. **Better Error Propagation**: Proper error chaining with yikes/fam/shook
3. **Type Safety**: yikes<T> provides compile-time error handling verification
4. **Idiomatic CURSED**: Fully embraces the language's Gen Z error handling style
5. **Maintainability**: Standard patterns make code easier to understand and maintain
