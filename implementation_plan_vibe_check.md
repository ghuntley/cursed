# `vibe_check` (switch/case) Statement Implementation Plan

This document outlines the plan for implementing the `vibe_check` (switch/case) statement generation in LLVM codegen for the CURSED language.

## Current Progress

1. We have identified the AST representation for switch/case statements:
   - `SwitchStatement` in `src/ast/control_flow/conditionals.rs` for the overall switch statement (called "vibe_check" in CURSED)
   - `CaseStatement` for individual case branches (called "mood" in CURSED)

2. We have created test cases in `tests/vibe_check_test.csd` to test:
   - Basic switch/case functionality
   - Multiple values in a single case
   - Fallthrough behavior

3. We have created test file `tests/llvm_vibe_check_test.rs` to test the LLVM IR generation

## Issues Encountered

1. The LlvmCodeGenerator API appears to be different than expected. The implementation we started doesn't match the actual API, particularly around:
   - Access to the builder object
   - Compile methods for expressions and statements

2. The CaseStatement doesn't implement Clone and has fields with types that don't implement Clone (dyn Expression and BlockStatement).

## Next Steps

### Approach 1: Fix the existing implementation in switch.rs

1. Examine the actual LlvmCodeGenerator API more carefully
2. Adapt our switch.rs implementation to match the existing API
3. Update statement.rs to call our new compile_switch_statement method
4. Implement tests and verify functionality

### Approach 2: Directly implement in statement.rs

1. Instead of creating a separate switch.rs file, directly implement the switch statement handling within the existing statement.rs compile_statement method
2. This approach avoids API mismatch issues but makes the code less modular

### Approach 3: Extend the control_flow.rs file

1. Add the switch statement handling to control_flow.rs alongside the existing if, while, and for implementations
2. This maintains the current code organization pattern and leverages existing patterns

## Selected Approach

Approach 3 is recommended as it aligns with the current code organization and will be easier to implement given the existing patterns in control_flow.rs.

## Implementation Tasks

1. Add compile_switch_statement implementation to control_flow.rs
2. Update statement.rs to call this new method when a SwitchStatement is encountered
3. Run tests to verify implementation
4. Fix any issues identified during testing

## Success Criteria

All tests for switch/case statements pass, and they can be used in real code examples with proper behavior for:
1. Basic switch/case with single values
2. Multiple values per case
3. Default case handling
4. Fallthrough behavior (without explicit breaks)