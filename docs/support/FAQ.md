# CURSED v1.0.x Frequently Asked Questions

## Installation & Setup

### Q: How do I install CURSED v1.0.0-stable?
```bash
curl -sSf https://install.cursedlang.org | sh
```
Or download pre-built binaries from [GitHub Releases](https://github.com/ghuntley/cursed/releases/tag/v1.0.0).

### Q: How do I verify my installation?
```bash
cursed-zig --version
# Should output: CURSED v1.0.0-stable
```

### Q: What are the system requirements?
- **Linux**: glibc 2.17+ (Ubuntu 16.04+, CentOS 7+)
- **macOS**: 10.15+ (Catalina or newer)
- **Windows**: Windows 10+ (64-bit)
- **Memory**: 512MB RAM minimum, 2GB recommended
- **Storage**: 100MB for compiler, additional for projects

## Language Features

### Q: What's the difference between `sus` and `drip` variables?
- `sus` - Immutable variable (like `const` in other languages)
- `drip` - Mutable variable (like `let mut` in Rust)

```cursed
sus name tea = "CURSED"        # Immutable string
drip counter drip = 0          # Mutable number
```

### Q: How do I handle errors in CURSED?
Use the `yikes`/`fam` error handling system:

```cursed
slay divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "Division by zero"
    }
    damn a / b
}

sus result drip = divide(10, 2) fam {
    when "Division by zero" -> damn 0
    when _ -> damn -1
}
```

### Q: How do I work with arrays?
```cursed
yeet "arrayz"

sus numbers []drip = [1, 2, 3, 4, 5]
vibez.spill("Length:", len(numbers))
vibez.spill("First:", numbers[0])

# Iteration
bestie (i drip = 0; i < len(numbers); i += 1) {
    vibez.spill("Item:", numbers[i])
}
```

## Standard Library

### Q: What standard library modules are available?
Core modules include:
- `vibez` - I/O operations and printing
- `mathz` - Mathematical functions
- `stringz` - String manipulation
- `arrayz` - Array operations
- `filez` - File system operations
- `networkz` - Networking and HTTP
- `testz` - Testing framework
- `concurrenz` - Concurrency and goroutines

### Q: How do I import modules?
```cursed
yeet "mathz"                    # Import math module
yeet "networkz" as net         # Import with alias

sus result drip = mathz.sqrt(16.0)
sus response tea = net.get("https://api.example.com")
```

### Q: How do I write tests?
```cursed
yeet "testz"

test_start("Math Operations")
assert_eq_int(2 + 2, 4)
assert_eq_string("hello", "hello")
print_test_summary()
```

## Concurrency

### Q: How do I use goroutines?
```cursed
yeet "concurrenz"

go {
    vibez.spill("Running in goroutine")
    sleep(1000)  # Sleep for 1 second
}

vibez.spill("Main thread continues")
```

### Q: How do channels work?
```cursed
yeet "concurrenz"

sus ch chan<drip> = make_channel()

go {
    ch <- 42                   # Send to channel
}

sus value drip = <-ch         # Receive from channel
vibez.spill("Received:", value)
```

## Compilation

### Q: How do I compile CURSED programs?
```bash
# Interpret and run
cursed-zig program.csd

# Compile to native binary
cursed-zig --compile program.csd
./program

# Compile for different targets
cursed-zig --compile --target x86_64-linux program.csd
cursed-zig --compile --target aarch64-macos program.csd
```

### Q: How do I optimize my programs?
```bash
# Release build with optimizations
cursed-zig --compile --release program.csd

# Debug build with symbols
cursed-zig --compile --debug program.csd
```

## Troubleshooting

### Q: My program compiles but crashes at runtime. What should I check?
1. **Array bounds**: CURSED has runtime bounds checking
2. **Null dereferencing**: Check for uninitialized variables
3. **Memory issues**: Use `valgrind` for debugging
4. **Concurrency**: Check for race conditions in goroutines

```bash
# Debug with valgrind
valgrind cursed-zig program.csd

# Enable debug output
cursed-zig --verbose program.csd
```

### Q: Compilation is slow. How can I speed it up?
1. Use incremental compilation: `cursed-zig --incremental`
2. Disable optimizations during development: `--debug`
3. Use parallel compilation: `--jobs 4`
4. Clean build cache: `cursed-zig --clean`

### Q: I get "undefined symbol" errors when compiling. What's wrong?
This usually indicates:
1. Missing standard library module import
2. Typo in function or variable name
3. LLVM linking issues (try `--debug` build)
4. Cross-compilation toolchain not installed

```bash
# Check what symbols are missing
cursed-zig --compile --verbose program.csd 2>&1 | grep undefined
```

## Performance

### Q: How does CURSED performance compare to other languages?
- **Runtime Speed**: 80-90% of C performance
- **Memory Usage**: 60-70% of C memory usage  
- **Compile Speed**: 50-300x faster than Rust
- **Startup Time**: <10ms for typical applications

### Q: How can I profile my CURSED programs?
```bash
# Built-in profiler
cursed-zig --profile program.csd

# Memory profiling
cursed-zig --memory-profile program.csd

# Performance benchmarks
cursed-zig --benchmark program.csd
```

## IDE Support

### Q: What editors support CURSED?
- **VS Code**: Official extension with syntax highlighting, LSP
- **Vim/Neovim**: Syntax highlighting and LSP support
- **IntelliJ**: Community plugin available
- **Emacs**: Major mode with basic features

### Q: How do I set up LSP support?
```bash
# LSP server is included in installation
cursed-lsp --version

# For VS Code, install the official extension
code --install-extension cursed-lang.cursed-vscode

# For other editors, point LSP client to:
cursed-lsp --stdio
```

## Community & Support

### Q: Where can I get help?
- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and help
- **Discord**: Real-time community chat
- **Stack Overflow**: Tag your questions with `cursed-lang`

### Q: How can I contribute to CURSED?
1. **Code**: Submit pull requests for bug fixes or features
2. **Documentation**: Improve guides and examples
3. **Testing**: Report bugs and edge cases
4. **Community**: Help answer questions and support users

### Q: What's the release schedule for CURSED?
- **Patch releases (v1.0.x)**: As needed for bugs and security
- **Minor releases (v1.1.x)**: Every 3-6 months with new features
- **Major releases (v2.x)**: Yearly with potential breaking changes

### Q: How long is v1.0.x supported?
- **Active Support**: 18 months (regular updates and features)
- **Extended Support**: Additional 12 months (security and critical fixes)
- **Migration Support**: Clear upgrade path to newer versions

For more help, see the [Troubleshooting Guide](TROUBLESHOOTING.md) or ask in our [community channels](../SUPPORT.md).
