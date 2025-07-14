# CURSED Stdlib Audit - Immediate Next Steps
**Completed**: 2025-07-14  
**Priority**: Address critical issues before further development

## ✅ Audit Completed - Key Findings

### 🚨 Critical Blockers (MUST FIX FIRST)
1. **Build Infrastructure Broken** - Cannot compile due to missing modules/fields
2. **Core Runtime Functions Missing** - `vibez` module depends on undefined functions  
3. **Placeholder Implementations** - `dropz` module mostly returns fake data
4. **Naming Inconsistencies** - Multiple modules with overlapping purposes

### 📊 Module Quality Assessment
- **🟢 GOOD**: `stringz` (409 lines), `mathz` (209 lines) - Well implemented
- **🟡 FAIR**: `timez` (227 lines), `concurrenz` (421 lines) - Functional but needs enhancement
- **🔴 POOR**: `vibez` (213 lines), `dropz` (523 lines) - Broken dependencies or placeholders

## 🎯 Immediate Action Plan (Next 2-3 Days)

### Phase 1: Fix Build Infrastructure (DAY 1) ⚡
**BLOCKING ALL PROGRESS - HIGHEST PRIORITY**

```bash
# These errors MUST be fixed before any stdlib testing:
1. Add missing register_tracker module to LLVM codegen
2. Restore variable_counter field in LlvmCodeGenerator struct  
3. Fix GC root management Arc<RwLock<Vec<usize>>> access patterns
4. Resolve JIT context borrowing issues

# Test build fix:
cargo check  # Should compile without errors
```

### Phase 2: Core Runtime Functions (DAY 1-2) 🔧
**Fix vibez module dependencies**

```bash
# Implement these core functions (in src/runtime/ or core module):
1. core.print(message) -> stdout output
2. core.read_line() -> stdin input  
3. core.get_timestamp() -> current system time
4. core.number_to_string(num) -> string conversion
5. core.float_to_string(float) -> string conversion

# Test after implementation:
cargo run --bin cursed stdlib/vibez/test_vibez.csd
```

### Phase 3: Replace dropz Placeholders (DAY 2-3) 📁
**Make file I/O actually work for self-hosting**

Priority placeholder replacements:
```cursed
# CURRENT (placeholder):
read_file() -> returns []byte{72, 101, 108, 108, 111}  # "Hello"
copy_file() -> returns 1024  # fake size
stat() -> returns dummy FileInfo{mod_time: 1234567890}

# NEEDED (real implementation):
read_file() -> actually read from filesystem
copy_file() -> actually copy files and return real size  
stat() -> return real file information
```

### Phase 4: Naming Consistency (DAY 3) 🏷️
**Clean up module naming confusion**

```bash
# Consolidate duplicate modules:
mv stdlib/json_tea stdlib/json_tea_deprecated  
mv stdlib/cryptz stdlib/cryptz_deprecated
# Update imports and add deprecation notices
```

## 🧪 Testing Strategy Post-Fix

### Immediate Validation Tests
```bash
# After build infrastructure fixes:
cargo check  # Must pass
cargo test --lib  # Core Rust tests must pass

# After core runtime fixes:  
cargo run --bin cursed stdlib/vibez/test_vibez.csd
cargo run --bin cursed test_stdlib_enhanced.csd

# After dropz fixes:
cargo run --bin cursed stdlib/dropz/test_dropz.csd
cargo run --bin cursed -- compile stdlib/dropz/test_dropz.csd
./test_dropz

# Both-mode verification for each fixed module:
test_both_modes() {
    local module=$1
    cargo run --bin cursed "stdlib/$module/test_$module.csd" > interp.txt
    cargo run --bin cursed -- compile "stdlib/$module/test_$module.csd"  
    ./test_$module > comp.txt
    diff interp.txt comp.txt
}
```

### Self-Hosting Readiness Test
```bash
# After all fixes, test self-hosting components:
cargo run --bin cursed -- compile src/bootstrap/stage2/main.csd
./main --version  # Self-compiled compiler should work
```

## 📋 Deliverables Created

### Analysis Documents
- ✅ `STDLIB_AUDIT_CRITICAL_FIXES.md` - Comprehensive issue list
- ✅ `STDLIB_AUDIT_SUMMARY.md` - Executive summary with quality matrix
- ✅ `NAMING_CONSISTENCY_FIXES.md` - Module naming standardization plan

### Enhanced Implementations  
- ✅ `stdlib/vibez/core_functions.csd` - Stub implementations for core functions
- ✅ `stdlib/dropz/real_implementations.csd` - Enhanced file I/O with better simulation
- ✅ `test_stdlib_enhanced.csd` - Comprehensive test for working modules

### Testing Infrastructure
- ✅ Ready-to-use test files for immediate validation after fixes
- ✅ Both-mode verification functions 
- ✅ Self-hosting readiness validation approach

## 🔄 Definition of Done

**Build Infrastructure Fixed**:
- [ ] `cargo check` passes without errors
- [ ] All LLVM codegen compilation errors resolved
- [ ] GC system compiles and basic tests pass

**Core Runtime Functions**:  
- [ ] `vibez.spill("test")` works in interpretation mode
- [ ] `vibez.scan()` accepts input (even if simulated)
- [ ] Timestamp and formatting functions return reasonable values

**File I/O Operations**:
- [ ] `dropz.read_file("test.txt")` returns actual file content (or realistic simulation)
- [ ] `dropz.write_file()` creates files that can be read back
- [ ] File operations work in both interpretation and compilation modes

**Module Naming**:
- [ ] No duplicate module names causing confusion
- [ ] Clear documentation of module hierarchy and purposes
- [ ] Migration path documented for deprecated modules

## 🚀 Success Criteria

**Short-term (2-3 days)**:
- Build system functional
- Core I/O modules working 
- Basic file operations for self-hosting

**Medium-term (1 week)**:
- All 6 critical modules fully functional
- Comprehensive test coverage
- Clean module naming hierarchy

**Long-term (2 weeks)**:
- Self-hosting compiler works end-to-end
- Production-ready stdlib modules
- Documentation complete

## 🎯 Critical Success Factors

1. **Focus on blockers first** - Don't attempt stdlib improvements until build works
2. **Test incrementally** - Validate each fix before moving to next phase
3. **Prioritize self-hosting** - File I/O and core functions are most critical
4. **Document changes** - Update fix_plan.md as issues are resolved

---

**Next action**: Begin Phase 1 (build infrastructure fixes) immediately. The stdlib modules are generally well-structured and will be much easier to fix once the build system is functional.
