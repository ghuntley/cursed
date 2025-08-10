# CURSED Self-Hosting Migration Plan

## Overview

This document outlines the migration from Zig built-in functions to pure CURSED implementations for self-hosting capabilities.

## Current Status

### Zig Built-ins to Replace (from `src-zig/built_ins.zig`)

**String Operations:**
- `stringConcat()` → Pure CURSED string concatenation (`+` operator)
- `stringLength()` → Pure CURSED `string_length()` from stringz module
- `stringSubstring()` → Pure CURSED `substring()` from stringz module  
- `stringEquals()` → Pure CURSED `==` operator
- `stringIndexOf()` → Pure CURSED `indexOf()` from stringz module
- `runtimeStringCharAt()` → Pure CURSED `char_at()` from stringz module

**Math Operations:**
- `mathAdd()` → Pure CURSED `+` operator and mathz module functions
- `mathMultiply()` → Pure CURSED `*` operator and mathz module functions
- `mathSubtract()` → Pure CURSED `-` operator and mathz module functions
- `mathDivide()` → Pure CURSED `/` operator and mathz module functions
- `mathAbs()` → Pure CURSED `abs_normie()` from mathz module
- `mathMax()` → Pure CURSED `max_normie()` from mathz module
- `mathMin()` → Pure CURSED `min_normie()` from mathz module

**Channel Operations:**
- `makeFunction()` → Pure CURSED `create_channel()` from concurrenz module
- Channel send/receive → Pure CURSED channel operations from concurrenz module

**I/O Operations:**
- `vibesSpill()` → Pure CURSED `vibez.spill()` function

**Array Operations:**
- Array length (`len()`) → Pure CURSED `array_length()` from arrayz module
- Array access/manipulation → Pure CURSED array functions from arrayz module

## Pure CURSED Implementations Available

### 1. String Operations (stdlib/stringz/mod.csd)
```cursed
slay string_length(s tea) drip { ... }
slay substring(s tea, start drip, length drip) tea { ... }
slay char_at(s tea, index drip) tea { ... }
slay indexOf(s tea, search tea) drip { ... }
slay concat_strings(a tea, b tea) tea { damn a + b }
```

### 2. Math Operations (stdlib/mathz/mod.csd)
```cursed
slay abs_normie(x drip) drip { ... }
slay max_normie(a drip, b drip) drip { ... }
slay min_normie(a drip, b drip) drip { ... }
slay add_two(a drip, b drip) drip { damn a + b }
slay multiply_two(a drip, b drip) drip { damn a * b }
```

### 3. Array Operations (stdlib/arrayz/mod.csd)
```cursed
slay array_size(nums []drip) drip { damn len(nums) }
slay array_get_safe(nums []drip, index drip, default drip) drip { ... }
slay concat_arrays(a []drip, b []drip) []drip { ... }
```

### 4. Channel Operations (stdlib/concurrenz/mod.csd)
```cursed
slay create_channel(capacity normie) *Channel { ... }
slay channel_send(ch *Channel, data normie) lit { ... }
slay channel_receive(ch *Channel) normie { ... }
```

### 5. I/O Operations (stdlib/vibez/mod.csd)
```cursed
slay spill(msg tea) lit { vibez.spill(msg); damn based }
slay spill_multiple(msg1 tea, msg2 tea) lit { ... }
```

## Migration Strategy

### Phase 1: Create Pure CURSED Built-ins Module ✅
- [x] Created `stdlib/self_hosted_builtins/mod.csd` with pure CURSED implementations
- [x] Created `src-zig/built_ins_pure_cursed.zig` as transition layer
- [x] Implemented all core string, math, array, and I/O operations

### Phase 2: Testing and Validation
1. **Test Pure CURSED Implementations**
   ```bash
   ./zig-out/bin/cursed-stable test_self_hosted_builtins.csd
   ./zig-out/bin/cursed-stable simple_self_hosted_test.csd
   ```

2. **Validate Memory Safety**
   ```bash
   valgrind ./zig-out/bin/cursed-stable test_self_hosted_builtins.csd
   ```

3. **Performance Benchmarking**
   - Compare Zig vs Pure CURSED implementation performance
   - Ensure acceptable performance for core operations

### Phase 3: Integration
1. **Replace Zig Built-ins Registry**
   - Update `main_unified.zig` to use `built_ins_pure_cursed.zig`
   - Remove dependencies on original `built_ins.zig`

2. **Update Compiler Integration**
   - Modify function call resolution to use pure CURSED stdlib modules
   - Update import system to automatically load required modules

### Phase 4: Complete Migration
1. **Remove Zig Dependencies**
   - Delete `src-zig/built_ins.zig` 
   - Update build system to remove Zig function dependencies
   - Ensure all operations work with pure CURSED code

2. **Self-Hosting Validation**
   - Compile CURSED programs using only pure CURSED built-ins
   - Verify complete independence from Zig runtime functions

## Implementation Details

### String Operations Migration
```cursed
// Before (Zig FFI)
sus result tea = string.concat("hello", " world")

// After (Pure CURSED)
sus result tea = "hello" + " world"  // Direct operator
// OR
yeet "stringz"
sus result tea = stringz.concat_strings("hello", " world")
```

### Math Operations Migration
```cursed
// Before (Zig FFI)
sus result drip = math.add(5, 3)

// After (Pure CURSED)
sus result drip = 5 + 3  // Direct operator
// OR
yeet "mathz"
sus result drip = mathz.add_two(5, 3)
```

### Array Operations Migration
```cursed
// Before (Zig FFI)
sus length drip = array.length(arr)

// After (Pure CURSED)
sus length drip = len(arr)  // Built-in function
// OR
yeet "arrayz"
sus length drip = arrayz.array_size(arr)
```

### Channel Operations Migration
```cursed
// Before (Zig FFI)
sus ch thicc = make_channel(10)

// After (Pure CURSED)
yeet "concurrenz"
sus ch *Channel = concurrenz.create_channel(10)
```

## Benefits of Self-Hosting

1. **Complete Independence**: No dependencies on Zig runtime
2. **Self-Contained**: All functionality implemented in CURSED
3. **Transparency**: All operations visible and modifiable in CURSED source
4. **Performance**: Direct CURSED execution without FFI overhead
5. **Debugging**: Full stack trace in CURSED code
6. **Portability**: Pure CURSED code runs on any target platform

## Compatibility Layer

During migration, maintain compatibility functions:
```cursed
// Legacy compatibility in self_hosted_builtins/mod.csd
slay legacy_string_concat(a tea, b tea) tea {
    damn string_concat_pure(a, b)
}

slay legacy_math_add(a drip, b drip) drip {
    damn math_add_pure(a, b)
}
```

## Testing Strategy

### Core Function Tests
- String operations: concatenation, length, substring, char access
- Math operations: arithmetic, abs, min/max
- Array operations: length, access, manipulation
- I/O operations: print, format
- Channel operations: create, send, receive

### Integration Tests
- Complex programs using multiple operation types
- Memory leak detection with valgrind
- Performance comparison benchmarks
- Cross-platform compatibility

### Regression Tests
- Ensure existing CURSED programs continue working
- Validate all stdlib modules still function correctly
- Check LLVM compilation pipeline compatibility

## Migration Checklist

### Phase 1: Foundation ✅
- [x] Create pure CURSED implementations for all built-in functions
- [x] Create transition layer in Zig that calls pure CURSED functions
- [x] Implement comprehensive test suite

### Phase 2: Integration
- [ ] Update compiler to use pure CURSED built-ins
- [ ] Remove dependencies on original Zig built_ins.zig
- [ ] Validate all existing functionality

### Phase 3: Testing
- [ ] Run full test suite with pure CURSED implementations
- [ ] Performance benchmark comparison
- [ ] Memory safety validation with valgrind

### Phase 4: Production
- [ ] Deploy self-hosted version as default
- [ ] Remove all Zig function dependencies
- [ ] Document self-hosting achievement

## Expected Outcomes

1. **100% Self-Hosted**: CURSED compiler using only CURSED code
2. **Performance**: Comparable or better performance than Zig FFI
3. **Memory Safety**: Zero memory leaks in pure CURSED implementations
4. **Maintainability**: All functionality visible and editable in CURSED
5. **Portability**: Complete independence from Zig runtime dependencies

## Success Metrics

- All existing CURSED programs compile and run correctly
- Performance within 10% of current Zig-based implementations
- Zero memory leaks detected by valgrind
- 100% of built-in functions implemented in pure CURSED
- Complete removal of Zig FFI dependencies for core operations

## Timeline

- **Week 1**: Complete pure CURSED implementations and testing
- **Week 2**: Integration with compiler and build system
- **Week 3**: Comprehensive testing and performance validation
- **Week 4**: Production deployment and documentation

This migration represents a major milestone toward complete CURSED self-hosting capabilities.
