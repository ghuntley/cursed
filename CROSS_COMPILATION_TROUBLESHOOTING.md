# CURSED Cross-Compilation Troubleshooting Guide

## Quick Diagnostics

### Environment Check
```bash
# Verify development environment
devenv shell -- rustc --version
devenv shell -- cargo --version  
devenv shell -- zig version

# Check cross-compilation toolchains
devenv shell -- which x86_64-unknown-linux-gnu-gcc
devenv shell -- which aarch64-unknown-linux-gnu-gcc
devenv shell -- which x86_64-w64-mingw32-gcc
```

### LLVM Configuration Check
```bash
devenv shell -- llvm-config --version
devenv shell -- echo $LLVM_SYS_181_PREFIX
devenv shell -- echo $LLVM_CONFIG_PATH
```

## Common Issues & Solutions

### 1. Development Environment Not Loading

**Symptoms**: 
- `rustc: command not found`
- Missing cross-compilation tools

**Solution**:
```bash
# Reload environment
direnv allow
direnv reload

# Force rebuild if needed  
devenv shell
```

### 2. LLVM Version Mismatch Warnings

**Symptoms**:
- Warning: LLVM version mismatch: system=18.1.8, rust=20.1.5

**Solution**: 
- This is expected and logged for awareness
- LLVM 18 is the closest stable version to Rust's LLVM 20.1.5
- No action required unless specific compatibility issues arise

### 3. Cross-Compilation Toolchain Missing

**Symptoms**:
- `x86_64-unknown-linux-gnu-gcc: command not found`
- Cross-compilation fails with missing linker

**Solution**:
```bash
# Check if cross-compilation tools are in PATH
devenv shell -- echo $PATH | grep -E "(gnu64|mingw|aarch64)"

# Manual verification
devenv shell -- find /nix/store -name "*-gcc" | grep cross
```

### 4. Windows Cross-Compilation pthread Issues

**Symptoms**:
- Windows target fails with pthread linking errors
- `libpthread.a not found`

**Solution**:
- Environment includes comprehensive pthread support
- Multiple pthread implementations available:
  - mingw_w64_pthreads
  - mcfgthreads  
  - fallback pthreads

### 5. Zig Linker Not Available

**Symptoms**:
- `zig: command not found`
- Universal linking fallback unavailable

**Solution**:
```bash
# Verify Zig installation
devenv shell -- which zig
devenv shell -- zig version

# Test Zig compilation
devenv shell -- echo 'int main(){return 0;}' | zig cc -x c -
```

### 6. WASM Compilation Issues

**Symptoms**:
- WASM target compilation fails
- `wasm-pack build` errors

**Solution**:
```bash
# Check WASM tools
devenv shell -- which wasm-pack
devenv shell -- cargo check --target wasm32-unknown-unknown

# Test simple WASM compilation
devenv shell -- rustc --target wasm32-unknown-unknown --crate-type cdylib simple.rs
```

## Advanced Troubleshooting

### Debug Cross-Compilation Environment

```bash
# Full environment diagnostic
devenv shell -- ./test_cross_compilation_enhanced.sh

# Check specific target configuration
devenv shell -- cargo build --target x86_64-unknown-linux-gnu --verbose

# Inspect linker configuration
devenv shell -- cargo rustc --target x86_64-unknown-linux-gnu -- --print native-static-libs
```

### Manual Cross-Compilation Test

```bash
# Create simple test
cat > cross_test.c << 'EOF'
#include <stdio.h>
int main() {
    printf("Cross-compilation test successful!\n");
    return 0;
}
EOF

# Test each cross-compiler
devenv shell -- x86_64-unknown-linux-gnu-gcc cross_test.c -o cross_test_linux
devenv shell -- aarch64-unknown-linux-gnu-gcc cross_test.c -o cross_test_arm64  
devenv shell -- x86_64-w64-mingw32-gcc cross_test.c -o cross_test_windows.exe

# Test Zig universal compilation
devenv shell -- zig cc -target x86_64-linux cross_test.c -o zig_test_linux
devenv shell -- zig cc -target x86_64-windows cross_test.c -o zig_test_windows.exe
```

### Environment Variable Debugging

```bash
devenv shell << 'EOF'
echo "=== Cross-Compilation Environment ==="
echo "CC_x86_64_unknown_linux_gnu: $CC_x86_64_unknown_linux_gnu"
echo "CC_aarch64_unknown_linux_gnu: $CC_aarch64_unknown_linux_gnu"  
echo "CC_x86_64_pc_windows_gnu: $CC_x86_64_pc_windows_gnu"
echo "CC_x86_64_apple_darwin: $CC_x86_64_apple_darwin"
echo ""
echo "=== Cargo Target Configuration ==="
echo "CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER: $CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER"
echo "CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER: $CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER"
echo "CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER: $CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER"
echo ""
echo "=== Universal Linker ==="
echo "ZIG_CC: $ZIG_CC"
echo "ZIG_CXX: $ZIG_CXX"
EOF
```

## Recovery Procedures

### Complete Environment Reset

```bash
# Clean rebuild
rm -rf .devenv/
direnv reload
devenv shell
```

### Force Package Rebuild

```bash
# Nix garbage collection and rebuild
nix store gc
devenv shell
```

### Alternative Environment Activation

```bash
# Direct nix-shell if devenv issues
nix-shell -p rustc cargo llvmPackages_18.llvm zig

# Or use system package manager as fallback
# (not recommended for production)
```

## Performance Optimization

### Parallel Cross-Compilation

```bash
# Set parallel build jobs
export CARGO_BUILD_JOBS=$(nproc)

# Use cargo workspace for parallel target builds
cargo build --workspace --target x86_64-unknown-linux-gnu &
cargo build --workspace --target aarch64-unknown-linux-gnu &
cargo build --workspace --target x86_64-pc-windows-gnu &
wait
```

### Caching Optimization

```bash
# Use sccache for distributed compilation caching
export RUSTC_WRAPPER=sccache

# Or use local caching
export CARGO_TARGET_DIR=target_cache
```

## Contact & Support

### Internal Resources
- Check AGENT.md for build commands
- Review devenv.nix configuration
- Consult CROSS_COMPILATION_SETUP_REPORT.md

### External Resources
- [Rust Cross-Compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)
- [Nix Cross-Compilation](https://nixos.org/manual/nixpkgs/stable/#chap-cross)
- [Zig Cross-Compilation](https://ziglang.org/learn/build-system/)

### Quick Commands Reference

```bash
# Environment status
direnv status

# Quick validation
devenv shell -- cargo check

# Cross-compilation test
devenv shell -- ./test_cross_compilation_enhanced.sh

# Full diagnostic
devenv shell -- cargo check --target x86_64-unknown-linux-gnu --verbose
```

This troubleshooting guide should resolve most cross-compilation environment issues. For complex problems, refer to the comprehensive setup report and ensure source code compilation issues are resolved first.
