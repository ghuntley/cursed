# Test Analysis: 02_extreme_inputs.csd

## Current Status
The test compiles successfully but shows different outputs between interpreter and compiled modes.

## Issues Identified

### 1. String Functions (stringz module)
**Problem**: `stringz.concat()` and `stringz.length()` behave differently in each mode

**Expected**: After 3 concatenations of "CURSED", should get "CURSEDCURSEDCURSEDCURSED" (24 chars)

**Current Results**:
- Interpreter: Shows length 32 (incorrect - possible overflow/wrapping)
- Compiled: Shows length 8 (incorrect - stdlib functions not loading properly)

**Root Cause**: 
- Compiled mode: stringz module functions not being loaded/compiled during LLVM pipeline
- Interpreter mode: Some calculation error or different function being called

### 2. Array Support
**Problem**: Array expressions cause interpreter to crash

**Status**: ✅ FIXED - Added `evaluateArray` function to interpreter, arrays now parse correctly

**Current Results**:
- Interpreter: Still crashes due to collections.length() function not found
- Compiled: Shows array length 0 (collections.length returns hardcoded 0)

### 3. Collections Module 
**Problem**: Collections stdlib module has parsing errors

**Error Log**:
```
Error at unknown:4:10 - Error parsing function statement
Error at unknown:7:1 - Failed to parse statement  
```

**Status**: Collections module fails to parse, causing collections.length() to be unhandled

## Test Behavior Analysis

### Working correctly:
- ✅ Large number arithmetic (both modes consistent)  
- ✅ Mathematical operations (mathz.abs_normie works in both modes)
- ✅ Basic variable assignments and calculations

### Inconsistent between modes:
- ❌ String concatenation and length calculations
- ❌ Array/collection length operations (crash vs 0)

## Recommendations

### Priority 1 - Critical Issues:
1. **Fix stringz module loading** in compiled mode
2. **Fix collections module parsing** errors  
3. **Fix interpreter string length calculation** to match expected behavior

### Priority 2 - Consistency Issues:  
1. Ensure both modes use identical stdlib implementations
2. Add proper error handling for missing module functions
3. Implement proper module loading verification

## Current Test Classification
- **Compiles**: ✅ Yes
- **Interpreter runs to completion**: ❌ No (crashes on collections.length)  
- **Compiled runs to completion**: ✅ Yes
- **Results match**: ❌ No (string lengths differ, array behavior differs)

## Next Steps
1. Fix collections module syntax errors
2. Investigate why stringz module functions aren't being loaded in compiled mode
3. Debug interpreter string length calculation
4. Re-run test to verify consistent behavior
