# CURSED Troubleshooting Guide

Comprehensive troubleshooting guide for common issues with the CURSED compiler and runtime.

## Quick Diagnostics

### System Health Check

```bash
#!/bin/bash
# cursed-doctor.sh - Diagnostic script

echo "=== CURSED System Diagnostics ==="

# Check binary availability
if command -v cursed-zig &> /dev/null; then
    echo "✓ cursed-zig: $(cursed-zig --version)"
else
    echo "✗ cursed-zig not found"
fi

if command -v cursed-stable &> /dev/null; then
    echo "✓ cursed-stable: $(cursed-stable --version)"
else
    echo "✗ cursed-stable not found"
fi

# Check dependencies
echo -n "Zig: "
if command -v zig &> /dev/null; then
    echo "✓ $(zig version)"
else
    echo "✗ Not installed"
fi

echo -n "LLVM: "
if command -v llvm-config &> /dev/null; then
    echo "✓ $(llvm-config --version)"
elif command -v llvm-config-15 &> /dev/null; then
    echo "✓ $(llvm-config-15 --version)"
else
    echo "✗ Not found"
fi

echo -n "Valgrind: "
if command -v valgrind &> /dev/null; then
    echo "✓ Available"
else
    echo "ⓘ Not available (optional)"
fi

# Test basic functionality
echo "=== Basic Functionality Test ==="
echo 'vibez.spill("Health check OK")' > /tmp/cursed_test.csd

if cursed-zig /tmp/cursed_test.csd 2>/dev/null | grep -q "Health check OK"; then
    echo "✓ Basic execution working"
else
    echo "✗ Basic execution failed"
fi

# Test stdlib
echo 'yeet "mathz"; vibez.spill("Math OK:", abs_normie(-42))' > /tmp/cursed_stdlib_test.csd
if cursed-zig /tmp/cursed_stdlib_test.csd 2>/dev/null | grep -q "Math OK: 42"; then
    echo "✓ Standard library working"
else
    echo "✗ Standard library issues"
fi

# Cleanup
rm -f /tmp/cursed_test.csd /tmp/cursed_stdlib_test.csd

echo "=== Diagnostics Complete ==="
```

Run with: `curl -fsSL https://raw.githubusercontent.com/ghuntley/cursed/main/scripts/cursed-doctor.sh | bash`

## Installation Issues

### Issue: "cursed-zig: command not found"

**Symptoms:**
```bash
$ cursed-zig --version
bash: cursed-zig: command not found
```

**Causes:**
- Binary not in PATH
- Incomplete installation
- Permission issues

**Solutions:**

1. **Check installation location:**
   ```bash
   # Find cursed binaries
   find /usr/local/bin /opt -name "cursed-*" 2>/dev/null
   which cursed-zig cursed-stable
   ```

2. **Add to PATH:**
   ```bash
   # For bash
   echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   
   # For zsh
   echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.zshrc
   source ~/.zshrc
   ```

3. **Reinstall with correct permissions:**
   ```bash
   # Remove incomplete installation
   sudo rm -f /usr/local/bin/cursed-*
   
   # Reinstall
   curl -fsSL https://install.cursed.dev | bash
   
   # Verify
   cursed-zig --version
   ```

### Issue: "LLVM not found" during build

**Symptoms:**
```
error: LLVM development libraries not found
ld: cannot find -lLLVM-15
```

**Solutions:**

1. **Ubuntu/Debian:**
   ```bash
   sudo apt update
   sudo apt install llvm-15-dev libllvm15 llvm-15-runtime
   sudo apt install pkg-config
   
   # Set environment variables
   export LLVM_CONFIG="llvm-config-15"
   export LLVM_SYS_150_PREFIX="/usr/lib/llvm-15"
   ```

2. **macOS:**
   ```bash
   brew install llvm@15
   
   # Add to PATH
   echo 'export PATH="/opt/homebrew/opt/llvm@15/bin:$PATH"' >> ~/.zshrc
   source ~/.zshrc
   ```

3. **Alternative LLVM versions:**
   ```bash
   # Check available LLVM versions
   apt list --installed | grep llvm
   
   # Use different version
   export LLVM_CONFIG="llvm-config-14"
   export LLVM_SYS_140_PREFIX="/usr/lib/llvm-14"
   ```

### Issue: Zig version compatibility

**Symptoms:**
```
error: unsupported zig version 0.10.0, required 0.11.0+
```

**Solutions:**

1. **Download correct Zig version:**
   ```bash
   # Remove old version
   sudo rm -rf /opt/zig /usr/local/bin/zig
   
   # Download Zig 0.11.0
   wget https://ziglang.org/download/0.11.0/zig-linux-x86_64-0.11.0.tar.xz
   tar xf zig-linux-x86_64-0.11.0.tar.xz
   sudo mv zig-linux-x86_64-0.11.0 /opt/zig
   sudo ln -s /opt/zig/zig /usr/local/bin/zig
   
   # Verify
   zig version
   ```

2. **macOS with Homebrew:**
   ```bash
   brew uninstall zig
   brew install zig@0.11
   ```

## Build Issues

### Issue: Build hangs during cross-compilation

**Symptoms:**
```bash
$ zig build -Dtarget=aarch64-linux
# Hangs indefinitely
```

**Solutions:**

1. **Use native builds only:**
   ```bash
   # Force native compilation
   zig build -Dtarget=native
   ```

2. **Clean rebuild:**
   ```bash
   rm -rf zig-cache/ zig-out/
   zig build
   ```

3. **Use alternative binary:**
   ```bash
   # If main binary has wrong architecture
   ./zig-out/bin/cursed-syscall program.csd
   ./zig-out/bin/cursed-stable program.csd
   ```

### Issue: Out of memory during build

**Symptoms:**
```
error: OutOfMemory
The system cannot allocate sufficient memory
```

**Solutions:**

1. **Reduce build parallelism:**
   ```bash
   # Build with single thread
   zig build -j1
   
   # Or specify max jobs
   zig build -j2
   ```

2. **Increase swap space:**
   ```bash
   # Add 4GB swap
   sudo fallocate -l 4G /swapfile
   sudo chmod 600 /swapfile
   sudo mkswap /swapfile
   sudo swapon /swapfile
   
   # Make permanent
   echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab
   ```

3. **Use smaller build:**
   ```bash
   # Build minimal compiler only
   zig build cursed-stable
   ```

### Issue: Permission denied errors

**Symptoms:**
```
error: Permission denied (os error 13)
unable to write to /usr/local/bin/cursed-zig
```

**Solutions:**

1. **Fix binary permissions:**
   ```bash
   sudo chmod +x /usr/local/bin/cursed-*
   sudo chown root:root /usr/local/bin/cursed-*
   ```

2. **Fix stdlib permissions:**
   ```bash
   sudo chown -R root:root /usr/local/lib/cursed
   sudo chmod -R 755 /usr/local/lib/cursed
   ```

3. **Install to user directory:**
   ```bash
   # Install to home directory
   mkdir -p ~/.local/bin
   cp zig-out/bin/cursed-* ~/.local/bin/
   echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

## Runtime Issues

### Issue: Segmentation fault

**Symptoms:**
```bash
$ cursed-zig program.csd
Segmentation fault (core dumped)
```

**Debugging Steps:**

1. **Use stable compiler:**
   ```bash
   # Try minimal compiler first
   cursed-stable program.csd
   ```

2. **Enable debug mode:**
   ```bash
   # Compile with debug info
   cursed-zig --debug program.csd
   
   # Run with debugger
   gdb ./program
   (gdb) run
   (gdb) bt
   ```

3. **Check with valgrind:**
   ```bash
   valgrind --tool=memcheck cursed-zig program.csd
   ```

4. **Simplify program:**
   ```cursed
   # Minimal test case
   vibez.spill("Hello")
   ```

### Issue: Memory leaks

**Symptoms:**
```
valgrind reports memory leaks
definitely lost: 1,024 bytes in 1 blocks
```

**Solutions:**

1. **Use arena allocators:**
   ```cursed
   # Ensure proper cleanup in functions
   slay process_data() {
       sus data []drip = [1, 2, 3, 4, 5]
       # Memory automatically freed when function exits
   }
   ```

2. **Use defer statements:**
   ```cursed
   slay file_operation() yikes<tea> {
       sus file = open_file("data.txt")?
       defer file.close()  # Always called
       
       # Process file...
       damn "Success"
   }
   ```

3. **Report persistent leaks:**
   ```bash
   # Create minimal reproduction case
   echo 'sus x drip = 42; vibez.spill(x)' > leak_test.csd
   valgrind --leak-check=full cursed-zig leak_test.csd
   
   # File issue with valgrind output
   ```

### Issue: "Module not found" errors

**Symptoms:**
```
error: module 'mathz' not found
cannot resolve import: mathz
```

**Solutions:**

1. **Check stdlib installation:**
   ```bash
   ls -la /usr/local/lib/cursed/stdlib/
   find /usr/local/lib/cursed -name "mathz" -type d
   ```

2. **Set environment variables:**
   ```bash
   export CURSED_STDLIB_PATH="/usr/local/lib/cursed/stdlib"
   export CURSED_HOME="/opt/cursed"
   ```

3. **Verify module syntax:**
   ```cursed
   # Correct syntax
   yeet "mathz"
   sus result = abs_normie(-42)
   
   # Not this
   # import mathz
   # use mathz::*
   ```

4. **Check available modules:**
   ```bash
   ls /usr/local/lib/cursed/stdlib/
   # Should show: vibez/ mathz/ stringz/ arrayz/ testz/ etc.
   ```

## Language Issues

### Issue: Syntax errors with correct CURSED code

**Symptoms:**
```
error: unexpected token 'sus'
expected expression, found 'drip'
```

**Solutions:**

1. **Verify file extension:**
   ```bash
   # Must use .csd extension
   mv program.txt program.csd
   cursed-zig program.csd
   ```

2. **Check language version:**
   ```bash
   cursed-zig --version
   # Should be v1.0+ for full syntax support
   ```

3. **Common syntax issues:**
   ```cursed
   # Correct
   sus count drip = 42
   sus name tea = "Alice"
   
   # Wrong
   let count int = 42     # Old syntax
   var name string = "Alice"  # Wrong keywords
   ```

### Issue: Pattern matching not working

**Symptoms:**
```
error: exhaustive pattern checking failed
pattern matching syntax error
```

**Solutions:**

1. **Ensure exhaustive patterns:**
   ```cursed
   enum Color { Red, Green, Blue }
   
   sick (color) {
       when Red -> vibez.spill("red")
       when Green -> vibez.spill("green")
       when Blue -> vibez.spill("blue")
       # Must cover all cases or use default
       # when _ -> vibez.spill("other")
   }
   ```

2. **Check enum syntax:**
   ```cursed
   # Correct enum definition
   enum Status {
       Pending,
       Processing,
       Complete,
       Failed(tea)  # With associated data
   }
   ```

### Issue: Goroutines not working

**Symptoms:**
```
error: 'vibe' not recognized
goroutine spawn failed
```

**Solutions:**

1. **Import concurrency module:**
   ```cursed
   yeet "concurrenz"
   
   vibe {
       vibez.spill("In goroutine")
   }
   ```

2. **Check channel syntax:**
   ```cursed
   yeet "concurrenz"
   
   sus messages chan<tea> = make_channel()
   
   vibe {
       messages <- "hello"
   }
   
   sus msg tea = <-messages
   ```

## Performance Issues

### Issue: Slow compilation

**Symptoms:**
```bash
$ time cursed-zig large_program.csd
real    0m30.123s  # Should be under 1s
```

**Solutions:**

1. **Use optimized build:**
   ```bash
   zig build -Doptimize=ReleaseFast
   ```

2. **Enable compilation cache:**
   ```bash
   export CURSED_ENABLE_CACHE=true
   export CURSED_CACHE_DIR=~/.cursed/cache
   mkdir -p ~/.cursed/cache
   ```

3. **Parallel compilation:**
   ```bash
   export CURSED_PARALLEL_JOBS=$(nproc)
   ```

4. **Clean build cache:**
   ```bash
   rm -rf ~/.cursed/cache/*
   rm -rf zig-cache/ zig-out/
   zig build
   ```

### Issue: Poor runtime performance

**Symptoms:**
```bash
# Program runs slower than expected
time ./compiled_program
real    0m5.123s  # Expected under 1s
```

**Solutions:**

1. **Use LLVM compilation:**
   ```bash
   cursed-zig --compile --lto program.csd
   ./program
   ```

2. **Enable optimizations:**
   ```bash
   cursed-zig --compile --optimize=speed program.csd
   ```

3. **Profile-guided optimization:**
   ```bash
   cursed-zig --compile --profile-generate program.csd
   ./program  # Run with typical workload
   cursed-zig --compile --profile-use program.csd
   ```

## Environment Issues

### Issue: Environment variables not working

**Symptoms:**
```bash
$ echo $CURSED_HOME

# Empty or incorrect path
```

**Solutions:**

1. **Set required variables:**
   ```bash
   # Add to ~/.bashrc or ~/.zshrc
   export CURSED_HOME="/opt/cursed"
   export CURSED_STDLIB_PATH="/usr/local/lib/cursed/stdlib"
   export CURSED_CACHE_DIR="$HOME/.cursed/cache"
   export CURSED_LOG_LEVEL="info"
   
   # Reload shell
   source ~/.bashrc
   ```

2. **System-wide configuration:**
   ```bash
   # Create system-wide config
   sudo tee /etc/environment <<EOF
   CURSED_HOME="/opt/cursed"
   CURSED_STDLIB_PATH="/usr/local/lib/cursed/stdlib"
   EOF
   ```

### Issue: WSL2 specific problems

**Symptoms:**
```
Windows Subsystem for Linux has issues with CURSED
```

**Solutions:**

1. **Update WSL2:**
   ```powershell
   wsl --update
   wsl --shutdown
   wsl
   ```

2. **Install in WSL2, not Windows:**
   ```bash
   # Inside WSL2 terminal
   curl -fsSL https://install.cursed.dev | bash
   ```

3. **Fix file permissions:**
   ```bash
   # In WSL2
   sudo chmod +x /usr/local/bin/cursed-*
   ```

### Issue: Docker/Container issues

**Symptoms:**
```
CURSED not working properly in containers
```

**Solutions:**

1. **Use correct base image:**
   ```dockerfile
   FROM ubuntu:22.04
   RUN apt-get update && apt-get install -y \
       llvm-15-dev libllvm15
   ```

2. **Set container environment:**
   ```dockerfile
   ENV CURSED_STDLIB_PATH="/usr/local/lib/cursed/stdlib"
   ENV CURSED_LOG_LEVEL="info"
   ```

3. **Copy stdlib correctly:**
   ```dockerfile
   COPY --from=builder /src/stdlib /usr/local/lib/cursed/stdlib
   ```

## Debugging Tools

### Memory Debugging

```bash
# Valgrind full check
valgrind --tool=memcheck --leak-check=full --show-leak-kinds=all cursed-zig program.csd

# Address sanitizer (if available)
CURSED_ENABLE_ASAN=1 cursed-zig program.csd

# Memory profiling
valgrind --tool=massif cursed-zig program.csd
ms_print massif.out.*
```

### Performance Profiling

```bash
# CPU profiling with perf
perf record cursed-zig program.csd
perf report

# Strace system calls
strace -c cursed-zig program.csd

# Time breakdown
time cursed-zig program.csd
```

### Compilation Debugging

```bash
# Verbose compilation
cursed-zig --verbose program.csd

# Show AST
cursed-zig --dump-ast program.csd

# Show generated LLVM IR
cursed-zig --emit-llvm program.csd
cat program.ll
```

## Getting Help

### Before Filing Issues

1. **Run diagnostics:**
   ```bash
   curl -fsSL https://raw.githubusercontent.com/ghuntley/cursed/main/scripts/cursed-doctor.sh | bash
   ```

2. **Create minimal reproduction:**
   ```cursed
   # Simplest possible program that shows the issue
   vibez.spill("Issue reproduction")
   ```

3. **Gather system info:**
   ```bash
   uname -a
   cursed-zig --version
   zig version
   llvm-config --version
   ```

### Issue Template

```markdown
## Issue Description
Brief description of the problem

## Steps to Reproduce
1. Create file: `echo 'code here' > test.csd`
2. Run command: `cursed-zig test.csd`
3. Observe error: [paste error output]

## Environment
- OS: [e.g., Ubuntu 22.04]
- CURSED version: [cursed-zig --version]
- Zig version: [zig version]
- LLVM version: [llvm-config --version]

## Expected Behavior
What should happen

## Actual Behavior
What actually happens

## Additional Context
Any other relevant information
```

### Support Channels

- **GitHub Issues**: [Bug reports and feature requests](https://github.com/ghuntley/cursed/issues)
- **Discussions**: [General questions](https://github.com/ghuntley/cursed/discussions)
- **Discord**: [Real-time community support](https://discord.gg/cursed-lang)
- **Stack Overflow**: Tag questions with `cursed-lang`

### Emergency Workarounds

If CURSED is completely broken:

1. **Use stable compiler:**
   ```bash
   cursed-stable program.csd
   ```

2. **Manual compilation:**
   ```bash
   # Compile to LLVM IR manually
   cursed-zig --emit-llvm program.csd
   llc program.ll -o program.o
   gcc program.o -o program
   ```

3. **Rollback to previous version:**
   ```bash
   git checkout previous-working-commit
   zig build
   ```

Most issues can be resolved with a clean rebuild and proper environment setup. When in doubt, start with the diagnostic script and work through the solutions systematically.
