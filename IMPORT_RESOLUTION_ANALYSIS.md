# CURSED Import Resolution Issues Analysis

## Overview
Analysis of import resolution failures in the CURSED codebase that prevent compilation of process management, testing framework, and other modules.

## Root Cause
The primary issue is that test files use `use crate::stdlib::testing::*;` but when running as tests, the crate root path resolution is different, causing multiple import failures.

## Major Import Issues Identified

### 1. Testing Framework - Missing Re-exports

**Problem**: `SequentialExecutor` not re-exported in testing module
**File**: `src/stdlib/testing/mod.rs` 
**Status**: ✅ FIXED - Added `SequentialExecutor` to executor re-exports

**Remaining Issues**:
- Test file uses `super::discovery::TestMetadata` which fails in test context
- All assertion functions are re-exported but path resolution still fails

### 2. Path Resolution Pattern Issues

**Pattern**: Tests use relative paths like:
```rust
super::discovery::TestMetadata
super::attributes::IgnoreContext
crate::stdlib::testing::executor::SequentialExecutor
```

**Problem**: These paths don't resolve correctly in test context where crate root is different.

**Solution Required**: Tests should use fully qualified imports or fix crate path resolution.

### 3. Specific Missing Type Exports

Based on compilation errors, these types need verification in stdlib exports:

**Testing Types** (Most are exported, but path resolution fails):
- ✅ `TestAttribute`, `TestAttributes` - Exported
- ✅ `TestExecutor`, `SequentialExecutor` - Now exported 
- ✅ `TestTimeout`, `TestStatus`, `TestResult` - Exported
- ✅ `TestFramework`, `TestFrameworkConfig` - Exported
- ✅ All assertion functions - Exported

**The issue is not missing exports but path resolution in test context.**

## Critical Files Affected

### `tests/testing_framework_test.rs`
- **140 compilation errors** due to path resolution
- Uses `use crate::stdlib::testing::*;` 
- Imports fail because crate root is different in test context

### Other Test Files Likely Affected
Any test that uses:
- `crate::stdlib::*` imports
- `super::` relative imports from testing modules
- Process management imports
- Database module imports

## Recommended Solutions

### Option 1: Fix Test Import Paths (PREFERRED)
Replace problematic imports in test files:

```rust
// Instead of:
use crate::stdlib::testing::*;
use super::discovery::TestMetadata;

// Use:
use cursed::stdlib::testing::*;
use cursed::stdlib::testing::discovery::TestMetadata;
```

### Option 2: Use Explicit Imports
Replace glob imports with specific function imports:

```rust
// Instead of:
use crate::stdlib::testing::*;

// Use:
use cursed::stdlib::testing::{
    assert_true, assert_false, assert_eq, assert_ne,
    TestInfo, TestMetadata, TestDiscovery, TestFilter,
    // ... other specific imports
};
```

### Option 3: Create Test Helper Module
Create `tests/common/mod.rs` with proper re-exports for test context.

## Implementation Priority

### High Priority (Blocks Compilation)
1. ✅ **COMPLETED**: Fix `SequentialExecutor` re-export
2. **TODO**: Fix test file import paths - Replace `crate::stdlib` with `cursed::stdlib`
3. **TODO**: Fix `super::` relative imports in tests

### Medium Priority  
1. Review all test files for similar import issues
2. Standardize test import patterns across codebase
3. Add import guidelines to development documentation

### Low Priority
1. Consider creating test-specific helper modules
2. Add tooling to detect import resolution issues in CI

## Files Requiring Immediate Fixes

### `tests/testing_framework_test.rs`
**Lines to change**:
- Line 8: `use crate::stdlib::testing::*;` → `use cursed::stdlib::testing::*;`
- Line 235: `super::discovery::TestMetadata` → Direct import or qualified path
- Line 267: `crate::stdlib::testing::executor` → `cursed::stdlib::testing::executor`
- Lines 274, 287, 314, 322, etc.: All `super::discovery::TestMetadata` references

### Pattern for Other Test Files
Search for files containing:
- `use crate::stdlib::`
- `super::`
- `crate::stdlib::testing::`
- `crate::stdlib::process::`

## Verification Steps

1. ✅ **COMPLETED**: Fix `SequentialExecutor` export
2. **TODO**: Apply import path fixes to `tests/testing_framework_test.rs`
3. **TODO**: Test compilation: `cargo check --test testing_framework_test`
4. **TODO**: Search for similar issues in other test files
5. **TODO**: Update import guidelines in development docs

## Expected Outcome

After implementing these fixes:
- `tests/testing_framework_test.rs` should compile successfully
- Other test files with similar patterns should be identified and fixed
- Testing framework should be fully functional
- Process management and other stdlib modules should compile correctly

## Notes

The CURSED testing framework is comprehensive and well-implemented. The issues are primarily path resolution problems in test context, not missing functionality. The stdlib exports are largely correct - the problem is how tests import and reference them.
