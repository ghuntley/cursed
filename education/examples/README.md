# CURSED Code Examples

Welcome to the comprehensive CURSED examples repository! This collection contains hundreds of practical examples organized by skill level and topic.

## 📁 Repository Structure

```
examples/
├── beginner/          # Start here if you're new to CURSED
├── intermediate/      # Ready for more complex concepts?
├── advanced/          # Push your CURSED skills to the limit
├── real-world/        # Production-ready applications
├── algorithms/        # Data structures & algorithms
├── stdlib/            # Standard library demonstrations
└── patterns/          # Common programming patterns
```

## 🎯 Quick Navigation

### By Skill Level
- **[Beginner](./beginner/)** - Variables, functions, control flow
- **[Intermediate](./intermediate/)** - Structs, modules, error handling
- **[Advanced](./advanced/)** - Generics, macros, concurrency

### By Topic
- **[Web Development](./real-world/web/)** - Servers, APIs, frameworks
- **[CLI Tools](./real-world/cli/)** - Command-line applications
- **[Games](./real-world/games/)** - Game development examples
- **[Data Processing](./real-world/data/)** - File processing, APIs
- **[Algorithms](./algorithms/)** - Sorting, searching, data structures

### By Standard Library Module
- **[vibez](./stdlib/vibez/)** - I/O operations and printing
- **[networkz](./stdlib/networkz/)** - HTTP, TCP, WebSocket
- **[filez](./stdlib/filez/)** - File system operations
- **[jsonz](./stdlib/jsonz/)** - JSON parsing and generation
- **[concurrenz](./stdlib/concurrenz/)** - Goroutines and channels

## 🚀 Getting Started

### Run Examples Locally

```bash
# Clone examples (if not already done)
git clone https://github.com/ghuntley/cursed.git
cd cursed/education/examples

# Run any example
cursed-zig beginner/hello_world.csd
cursed-zig intermediate/web_server.csd
cursed-zig advanced/concurrent_pipeline.csd
```

### Interactive Playground

Try examples in your browser without installation:
- [Web Playground](../playground/web/)
- [Beginner Examples](../playground/beginner/)
- [Advanced Demos](../playground/advanced/)

## 📚 Learning Paths

### Path 1: Complete Beginner
1. [Hello World](./beginner/01-hello-world.csd)
2. [Variables & Types](./beginner/02-variables.csd)
3. [Functions](./beginner/03-functions.csd)
4. [Control Flow](./beginner/04-control-flow.csd)
5. [Arrays](./beginner/05-arrays.csd)

### Path 2: Coming from Other Languages
- [From Rust](./patterns/migration/from-rust.csd)
- [From Go](./patterns/migration/from-go.csd)
- [From Python](./patterns/migration/from-python.csd)
- [From JavaScript](./patterns/migration/from-javascript.csd)

### Path 3: Real-World Applications
1. [CLI Todo App](./real-world/cli/todo-app/)
2. [REST API Server](./real-world/web/rest-api/)
3. [Web Scraper](./real-world/data/web-scraper/)
4. [Chat Application](./real-world/networking/chat-app/)

## 🏆 Featured Examples

### Beginner Highlights

**Hello World with Style**
```cursed
yeet "vibez"
vibez.spill("🔥 Welcome to CURSED! 🔥")
```
[View full example](./beginner/01-hello-world.csd)

**Interactive Calculator**
```cursed
yeet "vibez"

slay calculate(a drip, op tea, b drip) drip yikes<tea> {
    ready (op == "+") { damn a + b }
    otherwise ready (op == "-") { damn a - b }
    otherwise ready (op == "*") { damn a * b }
    otherwise ready (op == "/") {
        ready (b == 0) { yikes "Division by zero!" }
        damn a / b
    }
    otherwise { yikes "Unknown operator: " + op }
}
```
[View full example](./beginner/calculator.csd)

### Intermediate Highlights

**Task Manager with JSON Storage**
```cursed
yeet "jsonz"
yeet "filez"

squad Task {
    id drip,
    title tea,
    completed lit
}

slay save_tasks(tasks []Task) {
    sus json tea = jsonz.stringify(tasks, indent: 2)
    filez.write_file("tasks.json", json)
}
```
[View full example](./intermediate/task-manager.csd)

### Advanced Highlights

**Concurrent Web Crawler**
```cursed
yeet "concurrenz"
yeet "networkz"

sus workers drip = 10
sus urls chan<tea> = make_channel(100)
sus results chan<CrawlResult> = make_channel(100)

# Start worker goroutines
bestie (i in 0..workers) {
    go {
        bestie (url := <-urls) {
            sus result = crawl_url(url)
            results <- result
        }
    }
}
```
[View full example](./advanced/web-crawler.csd)

## 📖 Example Categories

### Beginner Examples (25 examples)
Perfect for your first week with CURSED:
- Variables and basic types
- Functions and return values
- Control flow (if/else, loops)
- Arrays and basic collections
- Simple file operations
- Basic error handling

### Intermediate Examples (40 examples)
Ready to level up your skills:
- Structs and methods
- Module system and imports
- Advanced error handling
- JSON and data processing
- Basic concurrency
- Testing and debugging

### Advanced Examples (50 examples)
Master CURSED's powerful features:
- Generics and type system
- Macro programming
- Advanced concurrency patterns
- Memory management
- Performance optimization
- FFI and system integration

### Real-World Applications (30 examples)
Production-ready code you can learn from:
- Web servers and APIs
- CLI tools and utilities
- Database applications
- Network services
- Data processing pipelines
- Games and multimedia

### Algorithm Implementations (35 examples)
Computer science fundamentals in CURSED:
- Sorting algorithms (quick, merge, heap)
- Search algorithms (binary, graph)
- Data structures (trees, graphs, heaps)
- Dynamic programming
- Graph algorithms
- String algorithms

### Standard Library Showcase (60 examples)
Explore CURSED's 50+ built-in modules:
- File system operations
- Network programming
- JSON/XML processing
- Cryptography and security
- Date/time handling
- Mathematical computations

## 🎨 Code Style Guide

All examples follow CURSED best practices:

```cursed
# Use descriptive variable names
sus user_count drip = 0
sus active_connections []Connection = []

# Clear function signatures
slay process_user_data(user User, options ProcessOptions) ProcessResult yikes<tea> {
    # Implementation here
}

# Proper error handling
sus result = risky_operation() fam {
    when NetworkError -> {
        vibez.spill("Network issue, retrying...")
        damn retry_operation()
    }
    when ParseError -> yikes "Invalid data format"
}

# Clean imports
yeet "vibez"    # I/O operations
yeet "networkz" # Network operations
yeet "jsonz"    # JSON handling
```

## 🔧 Building Examples

### Individual Examples
```bash
# Run in interpreter mode (fast for development)
cursed-zig example.csd

# Compile to binary (optimized for production)
cursed-zig --compile example.csd
./example
```

### Batch Testing
```bash
# Test all beginner examples
./scripts/test-examples.sh beginner/

# Test all examples
./scripts/test-all-examples.sh

# Performance benchmark
./scripts/benchmark-examples.sh advanced/
```

### Cross-Platform
```bash
# Build for different targets
cursed-zig --compile --target=x86_64-linux example.csd
cursed-zig --compile --target=aarch64-macos example.csd
cursed-zig --compile --target=wasm32-wasi example.csd
```

## 🤝 Contributing Examples

We welcome new examples! See our [contribution guide](../community/contributing.md).

**Example contribution checklist:**
- [ ] Includes comprehensive comments
- [ ] Demonstrates one clear concept
- [ ] Includes expected output
- [ ] Works on all platforms
- [ ] Follows style guidelines
- [ ] Includes performance notes (if applicable)

## 📊 Example Statistics

- **Total Examples**: 240
- **Lines of Code**: 15,000+
- **Topics Covered**: 50+
- **Difficulty Levels**: 4
- **Languages Compared**: 8
- **Standard Library Coverage**: 100%

## 🎓 Learning Resources

**Complement your example study with:**
- [Interactive Tutorials](../tutorials/)
- [Video Course](../video-course/)
- [Language Reference](../reference/)
- [Community Discord](../community/discord.md)

---

**Ready to start coding?** 🚀  
Begin with [Hello World](./beginner/01-hello-world.csd) or jump to your skill level!
