# Oracle's Final v1.0 Burn-Down Plan

## 🎯 Strategic Focus: Last 15% to Production v1.0

**Current Status**: 85% completion with P0 blockers resolved  
**Oracle Guidance**: 4-week focused burn-down to complete v1.0  
**Success Criteria**: Zero correctness gaps + professional developer experience  

## 🚨 v1.0 SURFACE FREEZE - EFFECTIVE IMMEDIATELY

### **Ship List** (Final 15% - Must Complete)
- [x] ✅ Interactive REPL in CURSED  
- [x] ✅ LSP Server in CURSED with JSON-RPC 2.0
- [x] ✅ Standard Library placeholder elimination (44% → <5%)
- [x] ✅ Complex expression parsing fixes
- [x] ✅ Package Manager in CURSED
- [ ] Core correctness gaps (type system, code generation)
- [ ] Memory management integration (GC stack maps, bounds checking)
- [ ] Performance optimization (PGO system completion)
- [ ] Final tooling polish (LSP semantic tokens, debugger MVP)

### **Slip List** (Post-v1.0)
- ❌ Macro hygiene system
- ❌ Async/await transformation  
- ❌ Advanced FFI features
- ❌ Loop optimizer
- ❌ Security hardening beyond basics
- ❌ Full self-hosting compiler

## 📅 ORACLE'S 4-WEEK BURN-DOWN

### **Week 1: Core Correctness** ⏰ Current
**Objective**: Stop-the-world blockers resolved

#### **Priority Actions**:
- [ ] **Type Inference Edge Cases**: `src-zig/enhanced_type_inference.zig:640`
- [ ] **Generic Function Declarations**: `src-zig/type_inference.zig:531`
- [ ] **Struct Field Validation**: `src-zig/type_system.zig:689`
- [ ] **Interface Dispatch**: `src-zig/codegen_clean.zig:1447-1663`
- [ ] **VTable Lookups**: `src-zig/advanced_codegen.zig:4655-4670`

#### **Acceptance Criteria**:
- Fuzz tests with random generic signatures pass
- Struct field type mismatches rejected with clear errors
- Interface dispatch emits guaranteed vtable lookup paths
- All changes merge with green CI

### **Week 2: Memory & Performance**
**Objective**: Production-grade runtime performance

#### **Priority Actions**:
- [ ] **GC Stack Maps**: `src-zig/gc_integration.zig:363-398` 
- [ ] **Array Bounds IR**: `src-zig/array_runtime.zig:176`
- [ ] **PGO System**: Profile blob persistence and auto-rebuild
- [ ] **Memory Optimizer**: Heap overhead ≤10% vs jemalloc
- [ ] **Performance Regression Gate**: CI fails if >5% slowdown

#### **Acceptance Criteria**:
- Zero false positives in Valgrind + MSAN nightly runs
- Memory fuzzer green ≥48h continuous testing
- PGO end-to-end stable (build → run → rebuild)
- Performance baselines updated and enforced

### **Week 3: Tooling & Documentation**  
**Objective**: Professional developer experience

#### **Priority Actions**:
- [ ] **LSP API Freeze**: Semantic tokens, goto-definition, find-references
- [ ] **VS Code Extension**: Ship vscode-cursed 1.0 extension
- [ ] **Debugger CLI Beta**: Step/run/continue, variable inspect, expression eval
- [ ] **Documentation Complete**: Getting Started, Interop, Concurrency, Error Handling
- [ ] **Reference Applications**: 5 working examples (CLI, web server, DB, crypto, concurrent)

#### **Acceptance Criteria**:
- LSP provides completion <50ms, diagnostics <200ms on 100k-LOC project
- VS Code extension published and functional
- Documentation complete with runnable examples
- Reference apps compile on all targets

### **Week 4: Release Candidate**
**Objective**: Final validation and v1.0 release

#### **Priority Actions**:
- [ ] **RC1 Tagged**: Full regression + fuzz testing pass
- [ ] **External Beta**: Community testing with "release-blocker" label triage
- [ ] **Quality Gates**: All CI green, zero critical security findings
- [ ] **Cross-Platform Matrix**: Linux x86-64/ARM64, macOS, Windows, WASI
- [ ] **Performance Validation**: ≥3x compile-time advantage vs Rust 1.80

#### **Acceptance Criteria**:
- "hello.cursed" compiles & runs in <200ms on mid-tier laptop
- 100% stdlib functions have non-stub docs & examples  
- All tier-1 platforms pass release testing with zero failures
- RC1 survives 7 days of external beta testing

## 🎯 SUCCESS METRICS FOR v1.0

### **Technical Excellence**
- **Build Performance**: <200ms for hello world program
- **Memory Safety**: Zero leaks confirmed across all test scenarios
- **Cross-Platform**: 100% success rate on all tier-1 platforms
- **Performance**: ≥3x compile-time advantage vs Rust
- **Quality**: 95% compiler coverage, 90% stdlib coverage

### **Developer Experience**  
- **IDE Integration**: VS Code extension with completion, diagnostics, formatting
- **Interactive Development**: REPL with history, tab completion, error recovery
- **Package Management**: Install, update, publish workflow functional
- **Documentation**: Complete guides with runnable examples
- **Debugging**: CLI debugger with breakpoints and variable inspection

### **Production Readiness**
- **Standard Library**: 90%+ real implementations (zero critical placeholders)
- **Error Handling**: Structured error system with recovery
- **Concurrency**: Goroutines and channels with race-condition protection
- **Type System**: Advanced generics with constraint validation
- **Code Generation**: Complete LLVM backend with optimization

## 🚀 IMMEDIATE NEXT STEPS

### **This Week (Week 1 Focus)**
1. **Fix Type System Edge Cases**: Deploy focused subagents on inference and validation
2. **Complete Interface Dispatch**: Finish vtable generation and method resolution
3. **Validate Memory Integration**: Ensure GC and bounds checking work correctly
4. **Run Comprehensive Testing**: Validate all fixes with complex CURSED programs

### **Quality Assurance**
- **Continuous Testing**: Run complex CURSED programs to validate fixes
- **Memory Validation**: Maintain zero-leak guarantee across all changes
- **Performance Monitoring**: Ensure changes don't regress build performance
- **Cross-Platform Testing**: Validate working binaries across all platforms

## 🏁 FINAL ASSESSMENT

**Oracle's 4-week plan provides the optimal path** to complete CURSED v1.0 with:
- **Technical Correctness**: All edge cases and integration issues resolved
- **Professional Tooling**: IDE integration and interactive development
- **Production Quality**: Memory safety, performance, and reliability
- **Developer Adoption**: Complete ecosystem for real-world programming

Following this focused approach ensures **CURSED v1.0 launches with excellence** rather than rushing to market with remaining gaps.

**Target**: True production-ready v1.0 within 4 weeks of focused execution.
