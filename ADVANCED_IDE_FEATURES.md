# CURSED Advanced IDE Features

The CURSED programming language provides world-class IDE support through an enhanced Language Server Protocol (LSP) implementation. This document outlines the comprehensive IDE features available for CURSED development.

## 🚀 Quick Start

### Installation

1. **Install the Advanced VS Code Extension**:
   ```bash
   cd cursed-vscode-extension-advanced
   npm install
   npm run package
   code --install-extension cursed-language-advanced-2.0.0.vsix
   ```

2. **Build the Advanced LSP Server**:
   ```bash
   zig build
   # This creates ./zig-out/bin/cursed-lsp-advanced
   ```

3. **Configure VS Code**:
   - Set `cursed.lsp.path` to `cursed-lsp-advanced` for enhanced features
   - Enable advanced features in settings

## 🎯 Core IDE Features

### 1. Intelligent Code Completion

**Context-Aware Suggestions**:
- Smart completion based on current scope and imports
- Function signature hints with parameter names
- Module and stdlib completions with documentation
- Snippet expansion for common CURSED patterns

**Example**:
```cursed
sus user = create_|  // Triggers completion for functions starting with 'create_'
vibez.|             // Shows all vibez module functions with documentation
```

### 2. Advanced Semantic Analysis

**Real-Time Type Checking**:
- Comprehensive type inference and validation
- Cross-file dependency analysis
- Unused variable and import detection
- Dead code identification

**Type Information Display**:
- Hover for detailed type information
- Inlay hints for inferred types
- Parameter name hints in function calls

### 3. Code Navigation

**Go to Definition**: Jump to symbol definitions across files
**Find References**: Find all usages of symbols
**Call Hierarchy**: Visualize function call relationships
**Type Hierarchy**: Navigate type inheritance and implementations
**Symbol Search**: Workspace-wide symbol navigation

## 🔧 Advanced Refactoring

### Extract Function
Select code block → Right-click → "Extract Function"
- Automatically detects parameters and return types
- Handles variable scoping correctly
- Updates all call sites

**Before**:
```cursed
slay complex_calculation(a drip, b drip) drip {
    sus temp1 drip = a * 2 + b * 3
    sus temp2 drip = temp1 * temp1 - a
    sus result drip = temp2 / (b + 1)
    damn result
}
```

**After** (extract the calculation logic):
```cursed
slay calculate_formula(a drip, b drip, temp1 drip) drip {
    sus temp2 drip = temp1 * temp1 - a
    damn temp2 / (b + 1)
}

slay complex_calculation(a drip, b drip) drip {
    sus temp1 drip = a * 2 + b * 3
    damn calculate_formula(a, b, temp1)
}
```

### Extract Variable
Select expression → Right-click → "Extract Variable"
- Infers variable type automatically
- Suggests meaningful variable names
- Handles complex expressions

### Inline Variable/Function
- Remove unnecessary intermediate variables
- Inline simple functions for better performance
- Maintains code correctness

### Rename Symbol
- Safe renaming across entire workspace
- Updates imports and references
- Handles name conflicts intelligently

### Convert to Pattern Matching
Automatically converts if-else chains to pattern matching:

**Before**:
```cursed
ready (status == 200) {
    damn "Success"
} otherwise ready (status == 404) {
    damn "Not Found"
} otherwise {
    damn "Error"
}
```

**After**:
```cursed
sick (status) {
    when 200 -> "Success"
    when 404 -> "Not Found"
    when _ -> "Error"
}
```

## 🎨 Code Generation

### Function Generation
`Ctrl+Shift+P` → "CURSED: Generate Function"
- Interactive function signature builder
- Automatic parameter and return type handling
- Optional body generation with common patterns

### Constructor Generation
Inside a struct → Right-click → "Generate Constructor"
- Creates `new_StructName` constructor function
- Handles field initialization
- Optional builder pattern generation

### Interface Implementation
`Ctrl+Shift+P` → "CURSED: Implement Interface"
- Generates method stubs for interface methods
- Maintains correct signatures
- Adds TODO comments for implementation

### Test Generation
`Ctrl+Shift+P` → "CURSED: Generate Tests"
- Creates comprehensive test suites
- Generates test data automatically
- Follows CURSED testing conventions
- Includes arrange-act-assert patterns

**Generated Test Example**:
```cursed
slay test_calculate_area() lit {
    // Arrange
    sus width drip = 5
    sus height drip = 10
    sus expected drip = 50
    
    // Act
    sus result drip = calculate_area(width, height)
    
    // Assert
    testz.assert_eq(result, expected)
    
    damn based
}
```

### Error Handling Generation
Select code → Right-click → "Add Error Handling"
- Wraps code with appropriate CURSED error handling
- Supports multiple error handling patterns
- Generates meaningful error messages

## 🔍 Advanced Analysis

### Security Analysis
`Ctrl+Shift+P` → "CURSED: Run Security Analysis"

**Detects**:
- SQL injection vulnerabilities
- Weak cryptographic algorithms
- Path traversal risks
- Buffer overflow potential
- Insecure random number generation

**Example Warning**:
```cursed
// Security: Weak cryptography detected
sus hash tea = cryptz.md5(password)  // ⚠️ Use SHA-256 or stronger
```

### Performance Analysis
`Ctrl+Shift+P` → "CURSED: Run Performance Analysis"

**Identifies**:
- Algorithmic complexity issues
- Inefficient string operations
- Memory allocation patterns
- Nested loop optimizations
- Cache-unfriendly access patterns

**Example Suggestion**:
```cursed
// Performance: O(n) search could be optimized
bestie (i := 0; i < users.len(); i++) {  // 💡 Consider using HashMap for O(1) lookups
    ready (users[i].name == target) {
        damn users[i]
    }
}
```

### Memory Analysis
`Ctrl+Shift+P` → "CURSED: Run Memory Analysis"

**Checks for**:
- Memory leaks
- Large allocations
- Buffer overruns
- Unfreed resources
- Stack overflow risks

### Concurrency Analysis
`Ctrl+Shift+P` → "CURSED: Run Concurrency Analysis"

**Detects**:
- Race conditions
- Deadlock potential
- Channel leaks
- Goroutine leaks
- Shared state issues

**Example Warning**:
```cursed
go {
    shared_variable = 42  // ⚠️ Race condition: shared state without synchronization
}
```

## 📊 Code Lens Features

**Function Reference Counts**:
```cursed
slay calculate_area(width drip, height drip) drip {  // 3 references
    damn width * height
}
```

**Implementation Counts**:
```cursed
collab Processor {  // 2 implementations
    slay process(data tea) tea
}
```

**Test Coverage**:
```cursed
slay user_validation(user User) lit {  // ✅ Tested | 🏃‍♂️ Run Tests
    // function body
}
```

## 💡 Inlay Hints

**Type Annotations**:
```cursed
sus user = create_user("Alice", 30)  // : User
sus result = calculate(10, 20)       // : drip
```

**Parameter Names**:
```cursed
create_user(name: "Alice", age: 30)
```

**Generic Type Parameters**:
```cursed
sus list = []           // : []T
sus map = make_map()    // : map<K, V>
```

## 🛠 Build and Debug Integration

### Task Integration
Pre-configured tasks for:
- `zig build` - Build project
- `zig test` - Run tests
- `cursed-fmt` - Format code
- `cursed-lint` - Lint code

### Debug Support
- Breakpoint support
- Variable inspection
- Call stack navigation
- Watch expressions
- Conditional breakpoints

### Test Runner
- Integrated test discovery
- One-click test execution
- Test result visualization
- Coverage reporting

## ⚙️ Configuration

### VS Code Settings

```json
{
    "cursed.lsp.path": "cursed-lsp-advanced",
    "cursed.lsp.enabled": true,
    "cursed.inlayHints.enabled": true,
    "cursed.inlayHints.typeAnnotations": true,
    "cursed.inlayHints.parameterNames": true,
    "cursed.codeLens.enabled": true,
    "cursed.codeLens.references": true,
    "cursed.analysis.security.enabled": true,
    "cursed.analysis.performance.enabled": true,
    "cursed.analysis.memory.enabled": true,
    "cursed.analysis.concurrency.enabled": true,
    "cursed.format.onSave": true,
    "cursed.codeGeneration.autoImport.enabled": true,
    "cursed.codeGeneration.generateTests.enabled": true
}
```

### LSP Server Features

The advanced LSP server supports:
- Full LSP 3.17 protocol compliance
- Custom CURSED-specific features
- Real-time semantic analysis
- Cross-file dependency tracking
- Incremental compilation hints

## 🎨 Themes and Syntax Highlighting

### Custom CURSED Themes
- **CURSED Dark**: Optimized dark theme
- **CURSED Light**: Clean light theme
- **Gen-Z Vibes**: Colorful theme matching CURSED's personality

### Enhanced Syntax Highlighting
- Semantic token support
- Context-aware highlighting
- Error highlighting
- Unused code dimming

## 📝 Code Snippets

### Built-in Snippets

| Trigger | Description | Expansion |
|---------|-------------|-----------|
| `slay` | Function definition | Complete function template |
| `sus` | Variable declaration | Variable with type inference |
| `ready` | If statement | If-else template |
| `bestie` | While loop | Loop template |
| `squad` | Struct definition | Struct template |
| `collab` | Interface definition | Interface template |
| `sick` | Pattern matching | Match template |
| `test` | Test function | Test template with AAA pattern |
| `main` | Main function | Entry point template |

### Custom Snippets
Users can define custom snippets for common patterns in their codebase.

## 🚀 Performance Features

### Fast Startup
- Sub-second LSP server startup
- Incremental analysis
- Smart caching

### Memory Efficiency
- Minimal memory footprint
- Efficient AST representation
- Smart garbage collection

### Scalability
- Handles large codebases
- Multi-threaded analysis
- Efficient incremental updates

## 🔧 Troubleshooting

### Common Issues

**LSP Server Not Starting**:
```bash
# Check if cursed-lsp-advanced is in PATH
which cursed-lsp-advanced

# Manual start for debugging
./zig-out/bin/cursed-lsp-advanced
```

**Performance Issues**:
- Disable analysis features temporarily
- Check system resources
- Clear LSP cache

**Analysis Not Working**:
- Verify configuration settings
- Check output channel for errors
- Restart language server

### Debug Mode
Enable debug tracing:
```json
{
    "cursed.lsp.trace": "verbose"
}
```

## 🌟 Advanced Tips

### Productivity Shortcuts

| Shortcut | Action |
|----------|--------|
| `F2` | Rename symbol |
| `Shift+F12` | Find references |
| `Ctrl+Shift+R` | Extract function |
| `Shift+Alt+F` | Format document |
| `Ctrl+;` | Run tests |
| `F5` | Debug |

### Workflow Integration
- Git integration for blame and history
- Project templates for common patterns
- Live share support for collaborative development
- Integrated terminal with CURSED REPL

### Extension Ecosystem
The CURSED extension works with:
- GitLens for enhanced Git integration
- Better Comments for improved comment highlighting
- Bracket Pair Colorizer for better code structure
- Error Lens for inline error display

## 📈 Roadmap

### Upcoming Features
- AI-powered code completion
- Automated refactoring suggestions
- Advanced profiling integration
- Real-time collaboration features
- Plugin system for custom analyzers

## 🤝 Contributing

### Extension Development
```bash
git clone https://github.com/ghuntley/cursed.git
cd cursed/cursed-vscode-extension-advanced
npm install
npm run watch
```

### LSP Server Development
```bash
cd cursed
zig build
# Edit src-zig/advanced_lsp_server.zig
```

### Adding New Features
1. Define LSP protocol extensions
2. Implement server-side logic
3. Add client-side integration
4. Update documentation
5. Add tests

## 📚 Resources

- [CURSED Language Documentation](./docs/)
- [LSP Protocol Specification](https://microsoft.github.io/language-server-protocol/)
- [VS Code Extension API](https://code.visualstudio.com/api)
- [CURSED Examples](./examples/)

---

The CURSED Advanced IDE features provide a comprehensive development experience that rivals any modern programming language. From intelligent code completion to advanced security analysis, developers have all the tools needed for productive CURSED development.

**Experience the future of programming with CURSED! 🔥**
