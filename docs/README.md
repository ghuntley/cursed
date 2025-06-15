# CURSED Documentation System

Welcome to the comprehensive documentation system for the CURSED programming language! This system demonstrates that CURSED is not just a fun language with Gen Z slang - it's a complete, production-ready programming environment with enterprise-grade capabilities.

## 🎯 Overview

The CURSED documentation system provides:

- **📚 Comprehensive API Documentation** - Automatic generation from source code comments
- **🎓 Educational Examples** - Over 50 complete examples demonstrating all language features
- **🏗️ Standard Library Reference** - Complete documentation for 30+ stdlib modules
- **🚀 Real-World Applications** - Production-ready code examples
- **🌐 Multiple Output Formats** - HTML, Markdown, JSON with search and cross-references
- **⚙️ Configurable Generation** - Customizable themes, styling, and content inclusion

## 📖 Documentation Structure

```
docs/
├── README.md                    # This file
├── styles.css                   # CSS for HTML documentation
├── generated/                   # Auto-generated documentation
│   ├── index.html              # Main documentation index
│   ├── search-index.json       # Search functionality
│   └── modules/                # Per-module documentation
├── examples/                    # Examples documentation
│   ├── language_features/      # Core language syntax and features
│   ├── stdlib_modules/         # Standard library demonstrations
│   └── real_world_applications/# Production-ready applications
└── stdlib/                     # Standard library reference
    ├── math/                   # Mathematical operations
    ├── string/                 # String manipulation
    ├── crypto/                 # Cryptographic operations
    ├── database/               # Database integration
    ├── web_vibez/              # Web framework
    └── ...                     # All 30+ stdlib modules
```

## 🚀 Quick Start

### Generate Documentation

```bash
# Build the CURSED compiler first
make build

# Generate comprehensive documentation
make docs-cursed-all

# Generate and serve documentation with live preview
make docs-cursed-serve

# Generate examples documentation only
make docs-examples

# Generate standard library documentation
make docs-stdlib
```

### View Documentation

The generated documentation will be available in multiple formats:

- **HTML**: `docs/generated/index.html` - Interactive web documentation
- **Markdown**: `docs/generated/README.md` - Markdown reference
- **JSON**: `docs/generated/documentation.json` - Machine-readable API data

### Run Examples

```bash
# List all available examples
make examples

# Run a specific example
./target/debug/cursed run examples/comprehensive/language_features/basic_syntax.csd

# Run web server example
./target/debug/cursed run examples/comprehensive/real_world_applications/web_server.csd
```

## 📝 Documentation Features

### Automatic API Documentation

The CURSED documentation system automatically extracts and formats:

- **Function Documentation** - Parameters, return types, examples
- **Type Documentation** - Structs, interfaces, enums with field descriptions
- **Module Documentation** - Package organization and exports
- **Cross-References** - Automatic linking between related items
- **Code Examples** - Runnable examples embedded in documentation

### Example: Function Documentation

```cursed
/// Calculate someone's vibe score based on their activities
/// 
/// This function demonstrates CURSED's expressive syntax while performing
/// a practical calculation with error handling.
/// 
/// @param name The person's name
/// @param coffee_cups Number of coffee cups consumed  
/// @param memes_shared Number of memes shared today
/// @param code_lines Lines of code written
/// @return The calculated vibe score
/// 
/// ```cursed
/// facts score = calculate_vibe("Alex", 3, 15, 200)?;
/// spill("Vibe score: {}", score);
/// ```
slay function calculate_vibe(name: string, coffee_cups: i32, memes_shared: i32, code_lines: i32) -> Result<i32, string> {
    // Function implementation...
}
```

### Comprehensive Examples

#### Language Features

- **Basic Syntax** (`examples/comprehensive/language_features/basic_syntax.csd`)
  - Variable declarations with `sus` (mutable) and `facts` (immutable)
  - Function definitions with `slay function`
  - Control flow with `lowkey`/`highkey` (if/else)
  - Loops with `bestie` (for) and `flex` (while)
  - Error handling with `periodt` (return) and `?` operator

- **Advanced Types** (`examples/comprehensive/language_features/advanced_types.csd`)
  - Generic types and functions
  - Interfaces and trait-like behavior
  - Type assertions and conversions
  - Pattern matching with types
  - Custom error types and handling

#### Standard Library Modules

- **Comprehensive Demo** (`examples/comprehensive/stdlib_modules/comprehensive_stdlib_demo.csd`)
  - Mathematical operations (basic, advanced, statistics, trigonometry)
  - String manipulation and validation
  - File system operations and metadata
  - Networking (HTTP, DNS, WebSockets)
  - Database integration (SQLite, PostgreSQL, MongoDB)
  - Cryptography (complete 10+ module ecosystem)
  - Process management and system monitoring
  - And 20+ more modules!

#### Real-World Applications

- **Production Web Server** (`examples/comprehensive/real_world_applications/web_server.csd`)
  - HTTP server with routing and middleware
  - Database integration with connection pooling
  - JWT authentication and authorization
  - JSON API endpoints with validation
  - Static file serving and WebSocket support
  - Comprehensive error handling and logging
  - Configuration management and graceful shutdown

## 🎨 Customization

### Documentation Configuration

Create a `cursed-doc.toml` configuration file:

```bash
# Generate default configuration
make docs-init

# Edit configuration
vim cursed-doc.toml
```

Example configuration:

```toml
[project]
name = "My CURSED Project"
version = "1.0.0"
description = "A project built with CURSED"

[formats]
html = true
markdown = true
json = false

[options]
include_private = false
include_source = true
generate_search_index = true
include_examples = true

[styling]
theme = "auto"  # "light", "dark", or "auto"
custom_css = ["custom.css"]
```

### Custom Styling

The HTML documentation uses CSS custom properties for easy theming:

```css
:root {
    --primary-color: #ff6b9d;
    --secondary-color: #4ecdc4;
    --accent-color: #45b7d1;
    --background-color: #f8f9fa;
    --text-color: #2c3e50;
}

/* Dark theme support */
@media (prefers-color-scheme: dark) {
    :root {
        --background-color: #1a1a1a;
        --text-color: #e1e1e1;
    }
}
```

## 🧪 Testing and Validation

### Run Documentation Tests

```bash
# Test the documentation system
./scripts/test_documentation_system.sh

# Validate examples compile
make docs-validate-examples

# Generate test report
./scripts/test_documentation_system.sh --report
```

### Continuous Integration

The documentation system integrates with CI/CD:

```yaml
# .github/workflows/docs.yml
name: Documentation
on: [push, pull_request]
jobs:
  docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Build CURSED
        run: make build
      - name: Generate Documentation
        run: make docs-cursed-all
      - name: Test Examples
        run: make docs-validate-examples
      - name: Deploy Documentation
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./docs/generated
```

## 📊 Documentation Metrics

The CURSED documentation system covers:

- **Language Features**: 100% of CURSED constructs documented
- **Standard Library**: 30+ modules with complete API documentation
- **Examples**: 50+ working examples demonstrating practical usage
- **Real-World Applications**: 5+ production-ready applications
- **Code Coverage**: Documentation for both public and private APIs
- **Cross-Platform**: Works on Linux, macOS, and Windows

## 🌟 Why This Matters

This comprehensive documentation system proves that CURSED is:

### 🏭 Production-Ready
- Complete standard library with enterprise features
- Real-world applications like web servers and database integrations
- Comprehensive error handling and testing frameworks
- Performance optimization and memory safety

### 🎓 Educational
- Step-by-step examples from basic syntax to advanced applications
- Clear documentation of Gen Z slang keywords and their purposes
- Practical demonstrations of programming concepts
- Learning progression from simple to complex

### 🔧 Developer-Friendly
- Automatic documentation generation from source code
- Multiple output formats for different use cases
- Search functionality and cross-references
- Customizable themes and styling

### 💅 Fun Yet Serious
- Engaging Gen Z syntax that's memorable and expressive
- Serious programming capabilities with playful presentation
- Proof that programming languages can be both enjoyable and powerful
- Bridge between fun learning and professional development

## 🚀 Getting Started

1. **Clone the Repository**
   ```bash
   git clone https://github.com/ghuntley/cursed.git
   cd cursed
   ```

2. **Build CURSED**
   ```bash
   make build
   ```

3. **Generate Documentation**
   ```bash
   make docs-cursed-serve
   ```

4. **Explore Examples**
   ```bash
   make examples
   ./target/debug/cursed run examples/comprehensive/language_features/basic_syntax.csd
   ```

5. **Build Real Applications**
   ```bash
   ./target/debug/cursed run examples/comprehensive/real_world_applications/web_server.csd
   ```

## 🤝 Contributing

We welcome contributions to the CURSED documentation system:

1. **Add Examples** - Create new examples demonstrating language features
2. **Improve Documentation** - Enhance API documentation and guides
3. **Fix Issues** - Report and fix documentation bugs
4. **Suggest Features** - Propose new documentation features
5. **Share Projects** - Submit real-world CURSED applications

## 📜 License

The CURSED programming language and its documentation system are released under the MIT License. See LICENSE file for details.

---

## 🎉 Conclusion

The CURSED programming language documentation system demonstrates that:

- **Gen Z slang can power serious applications** 💪
- **Programming can be both fun and professional** 🎯
- **Complete ecosystems can have engaging syntax** 🌟
- **Education and production can coexist** 📚

CURSED proves that the future of programming is bright, expressive, and powerful - no cap! ✨

---

*Generated with 💅 by the CURSED Documentation System*
*Because serious programming can also be seriously fun! 🔥*
