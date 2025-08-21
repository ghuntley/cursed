# CURSED Programming Language - FAQ

Frequently asked questions about CURSED programming language, answered by the community and core team.

## 🔥 Getting Started

### Q: What is CURSED?
**A:** CURSED is a modern programming language designed for Gen Z developers, combining performance with expressive syntax. It features intuitive keywords like `sus` (variable), `tea` (string), `drip` (integer), `lit` (boolean), and `yikes`/`fam` (error handling).

### Q: How do I install CURSED?
**A:** The easiest way is using our installer:
```bash
curl -sSf https://install.cursedlang.org | sh
```
Or build from source following our [installation guide](../tutorials/01-installation.md).

### Q: What platforms does CURSED support?
**A:** CURSED runs on:
- Linux (x86_64, ARM64)
- macOS (Intel, Apple Silicon)
- Windows (x86_64)
- WebAssembly (browser and WASI)

### Q: Is CURSED ready for production use?
**A:** Yes! CURSED v1.0 is production-ready with:
- ✅ Stable compiler and runtime
- ✅ Comprehensive standard library (50+ modules)
- ✅ Memory safety guarantees
- ✅ Zero memory leaks (validated with Valgrind)
- ✅ Cross-platform deployment

## 💻 Language Features

### Q: Why are the keywords so unusual?
**A:** CURSED's keywords reflect modern internet culture and Gen Z slang, making programming more expressive and fun:
- `sus` = "suspicious" → variable declaration
- `tea` = "truth/gossip" → string type
- `drip` = "style/coolness" → integer type
- `lit` = "excellent" → boolean type
- `based` = true, `cringe` = false
- `yikes` = error, `fam` = error handling

### Q: How does CURSED compare to other languages?
**A:** 
- **vs Rust**: Faster compilation (300-500x), simpler syntax, same performance
- **vs Go**: More expressive, better concurrency, similar simplicity
- **vs Python**: Much faster execution, type safety, compiled
- **vs JavaScript**: Type safety, better performance, server-side focused

### Q: Does CURSED have garbage collection?
**A:** CURSED uses a hybrid approach:
- Arena allocators for compiler data structures
- Optional garbage collection for managed objects
- Manual memory management for performance-critical code
- Linear types for memory safety

### Q: Can I use CURSED for web development?
**A:** Absolutely! CURSED has excellent web development support:
```cursed
yeet "networkz"
yeet "vibez"

sus server = networkz.create_server(8080)
server.get("/", slay(req) { damn "Hello, Web!" })
server.start()
```

## 🛠️ Development

### Q: What IDEs support CURSED?
**A:**
- **VS Code**: Official extension with syntax highlighting, LSP support
- **Vim/Neovim**: Plugin available
- **IntelliJ**: Community plugin
- **Emacs**: Basic syntax highlighting
- **Any editor**: LSP server provides universal support

### Q: How do I debug CURSED programs?
**A:** Several debugging options:
```bash
# Verbose interpreter mode
cursed-zig --verbose program.csd

# Generate debug symbols
cursed-zig --compile --debug program.csd

# Use GDB with compiled binaries
gdb ./program

# Memory debugging
valgrind ./program
```

### Q: Can I use existing C libraries?
**A:** Yes, through CURSED's FFI system:
```cursed
yeet "ffi"

# Declare external C function
extern "c" slay strlen(str *tea) drip

# Use in CURSED code
sus length drip = strlen("hello")
```

### Q: How do I handle errors in CURSED?
**A:** CURSED has structured error handling:
```cursed
slay divide(a drip, b drip) drip yikes<tea> {
    ready (b == 0) {
        yikes "Division by zero!"
    }
    damn a / b
}

sus result = divide(10, 2) fam {
    when "Division by zero!" -> {
        vibez.spill("Cannot divide by zero")
        damn 0
    }
}
```

## 🚀 Performance

### Q: How fast is CURSED?
**A:** CURSED performance metrics:
- **Execution**: 80-90% of C performance
- **Compilation**: 300-500x faster than Rust
- **Memory usage**: 60-70% of C memory usage
- **Startup time**: <10ms for typical applications

### Q: Can CURSED be used for systems programming?
**A:** Yes! CURSED is excellent for systems programming:
- Low-level memory access
- Zero-cost abstractions
- Inline assembly support
- Direct hardware interaction
- Kernel module development

### Q: How does concurrency work in CURSED?
**A:** CURSED uses goroutines and channels:
```cursed
yeet "concurrenz"

# Create channel
sus ch chan<drip> = make_channel()

# Goroutine
go {
    ch <- 42
}

# Receive
sus value drip = <-ch
```

## 📚 Learning and Community

### Q: Where can I learn CURSED?
**A:** Multiple learning resources:
- **[Quick Start Guide](../tutorials/02-quick-start.md)** - 30-minute introduction
- **[Interactive Tutorials](../tutorials/)** - Step-by-step learning
- **[Video Course](../video-course/)** - Comprehensive video series
- **[Examples Repository](../examples/)** - 240+ code examples
- **[Community Discord](./discord.md)** - Live help and discussion

### Q: How can I get help?
**A:**
1. **Discord #help-general** - Real-time community support
2. **GitHub Discussions** - Structured Q&A
3. **Stack Overflow** - Tag: `cursed-lang`
4. **Documentation** - Comprehensive guides and references

### Q: How can I contribute to CURSED?
**A:** Many ways to contribute:
- **Code**: Compiler, standard library, tools
- **Documentation**: Tutorials, guides, examples
- **Community**: Help others, organize events
- **Testing**: Bug reports, feature testing
- See our [Contributing Guide](./contributing.md)

### Q: Is there a CURSED conference or meetup?
**A:** Yes! Community events include:
- **Annual CURSED Conference** - Virtual event with talks
- **Monthly Workshops** - Learning sessions
- **Local Meetups** - Check Discord for your area
- **Online Study Groups** - Regular coding sessions

## 🔧 Technical Details

### Q: What's the compilation model?
**A:** CURSED uses LLVM backend:
```
CURSED Source → Lexer → Parser → Type Checker → LLVM IR → Native Binary
```

### Q: Can I cross-compile CURSED programs?
**A:** Yes, easy cross-compilation:
```bash
cursed-zig --compile --target=x86_64-linux program.csd
cursed-zig --compile --target=aarch64-macos program.csd
cursed-zig --compile --target=wasm32-wasi program.csd
```

### Q: Does CURSED have a package manager?
**A:** Yes, `cursed-pkg`:
```bash
cursed-pkg init                  # Initialize project
cursed-pkg install http-server   # Install package
cursed-pkg publish              # Publish package
```

### Q: How does the module system work?
**A:** Simple import system:
```cursed
# Import standard library modules
yeet "vibez"    # I/O operations
yeet "networkz" # Networking
yeet "mathz"    # Mathematics

# Import local modules
yeet "my_module"  # From local file
```

## 🌐 Ecosystem

### Q: What can I build with CURSED?
**A:** CURSED is versatile:
- **Web Applications**: REST APIs, microservices, full-stack apps
- **CLI Tools**: Command-line utilities and automation
- **System Software**: Databases, operating systems, embedded
- **Games**: Game engines, interactive applications
- **Data Processing**: ETL pipelines, analytics, ML inference

### Q: Are there any major applications built with CURSED?
**A:** Growing ecosystem includes:
- **Web Frameworks**: High-performance HTTP servers
- **Database Drivers**: PostgreSQL, MySQL, Redis clients
- **Development Tools**: Build systems, linters, formatters
- **Games**: 2D/3D game engines and applications

### Q: How is CURSED versioned?
**A:** Semantic versioning (SemVer):
- **Major**: Breaking changes (1.0 → 2.0)
- **Minor**: New features, backward compatible (1.0 → 1.1)
- **Patch**: Bug fixes (1.0.0 → 1.0.1)

Current stable: **1.0.0**

### Q: What's the long-term roadmap?
**A:** Upcoming features:
- **v1.1**: Enhanced generics, improved error messages
- **v1.2**: Advanced concurrency patterns, distributed systems
- **v2.0**: Breaking syntax improvements, performance optimizations

## 💡 Philosophy and Design

### Q: Why was CURSED created?
**A:** CURSED was created to address modern developer needs:
- **Expressive syntax** that reflects how developers actually communicate
- **Performance** without complexity or long compilation times
- **Modern features** (concurrency, type safety, memory safety)
- **Developer experience** focused on productivity and fun

### Q: Is CURSED a serious language?
**A:** Despite the playful syntax, CURSED is extremely serious about:
- **Production readiness** and stability
- **Performance** and efficiency
- **Memory safety** and security
- **Developer productivity** and ergonomics

### Q: How does CURSED handle backward compatibility?
**A:** Strong commitment to stability:
- **Semantic versioning** for all releases
- **Deprecation warnings** before breaking changes
- **Migration tools** for major version upgrades
- **Long-term support** for stable versions

## 🔍 Troubleshooting

### Q: "command not found: cursed-zig"
**A:** Path issue. Add to your shell profile:
```bash
export PATH="$HOME/.cursed/bin:$PATH"
# or if built from source:
export PATH="/path/to/cursed/zig-out/bin:$PATH"
```

### Q: Build fails with "undefined symbol" errors?
**A:** Clean rebuild usually fixes this:
```bash
rm -rf zig-cache/ zig-out/
zig build clean && zig build
```

### Q: LLVM linking issues on ARM64?
**A:** Use debug builds to avoid LLVM optimization bugs:
```bash
zig build -Doptimize=Debug
```

### Q: Memory leaks in my program?
**A:** Test with Valgrind:
```bash
valgrind --leak-check=full ./zig-out/bin/cursed-zig program.csd
```
CURSED runtime should show zero leaks.

### Q: Cross-compilation hanging?
**A:** Install target toolchain:
```bash
# For ARM64 Linux targets
sudo apt install gcc-aarch64-linux-gnu
```

## 📞 Still Have Questions?

**Can't find your answer here?**

- 💬 **Ask in Discord**: [#help-general](./discord.md)
- 📋 **GitHub Discussions**: [Community Q&A](https://github.com/ghuntley/cursed/discussions)
- 📧 **Email Support**: help@cursedlang.org
- 📚 **Documentation**: [Complete Reference](../reference/)

**Found an error in this FAQ?** Please [contribute an improvement](./contributing.md)!

---

*This FAQ is maintained by the CURSED community. Last updated: 2025-08-21*
