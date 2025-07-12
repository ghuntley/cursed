# FFI Elimination Comprehensive Plan

## Overview
This document outlines the systematic elimination of Foreign Function Interface (FFI) dependencies across the CURSED codebase to create a purely native implementation.

## Analysis Summary

### FFI Dependencies Found:
1. **src/stdlib/net/mod.rs** - 2 `extern "system"` declarations (WSAStartup, WSACleanup)
2. **Runtime Functions** - Multiple `extern "C"` declarations for LLVM integration
3. **JIT Engine** - LLVM-specific FFI bridges
4. **Execution Functions** - Network runtime functions

### Critical Dependencies:
- **LLVM Integration**: Required for compilation, but can be isolated
- **Windows Sockets**: Only needed for Windows networking 
- **Runtime Bridges**: Can be replaced with pure CURSED implementations

## Implementation Strategy

### Phase 1: Remove extern "system" declarations
- [x] Identify 2 extern "system" declarations in src/stdlib/net/mod.rs
- [ ] Replace with pure CURSED Windows socket alternatives
- [ ] Create cross-platform networking abstraction

### Phase 2: Create Pure CURSED Networking
- [ ] Implement pure CURSED TCP/UDP sockets
- [ ] Create platform-agnostic networking layer
- [ ] Replace extern "C" network functions with CURSED equivalents

### Phase 3: Isolate LLVM FFI
- [ ] Move LLVM FFI to dedicated compilation module
- [ ] Create runtime/compilation separation
- [ ] Ensure stdlib has zero FFI dependencies

### Phase 4: Verification
- [ ] Verify no stdlib module depends on FFI
- [ ] Test pure CURSED networking implementation
- [ ] Ensure both interpretation and compilation modes work

## Priority Order:
1. **HIGH**: Remove extern "system" declarations (immediate)
2. **MEDIUM**: Create pure CURSED networking alternatives
3. **LOW**: Isolate LLVM FFI to compilation-only modules

## Success Criteria:
- Zero FFI dependencies in stdlib modules
- Pure CURSED networking implementation
- Both interpretation and compilation modes functional
- All tests pass after migration
