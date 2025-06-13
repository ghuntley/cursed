# CURSED Language Server Protocol - Enhanced Features

This document describes the advanced Language Server Protocol (LSP) features implemented for the CURSED programming language, providing comprehensive IDE integration capabilities.

## Overview

The CURSED LSP server provides cutting-edge IDE features specifically designed for the Gen Z slang keywords and unique language constructs of CURSED. These features enhance developer productivity through intelligent code analysis, contextual information, and modern IDE integration.

## Features

### 1. Semantic Highlighting

Comprehensive semantic highlighting that understands CURSED's Gen Z slang keywords and language constructs.

#### Supported Token Types

**CURSED-Specific Keywords:**
- `slay` - Function declarations (highlighted as function keywords)
- `sus` - Variable declarations (highlighted as variable keywords) 
- `facts` - Constant declarations (highlighted as constant keywords)
- `lowkey` - If statements (highlighted as conditional keywords)
- `highkey` - Else statements (highlighted as conditional keywords)
- `periodt` - Switch statements (highlighted as control flow keywords)
- `bestie` - Case statements (highlighted as case keywords)
- `flex` - Default case (highlighted as default keywords)
- `yolo` - While/for loops (highlighted as loop keywords)
- `stan` - Goroutine spawning (highlighted as concurrency keywords)
- `crush` - Channel operations (highlighted as channel keywords)
- `spill` - Error/panic (highlighted as error keywords)
- `no_cap` - Boolean true (highlighted as boolean keywords)
- `cap` - Boolean false (highlighted as boolean keywords)
- `vibez` - Return statements (highlighted as return keywords)
- `skrr` - Break statements (highlighted as control keywords)
- `yeet` - Continue statements (highlighted as control keywords)
- `gurl` - Finally blocks (highlighted as exception keywords)

**Type System Keywords:**
- `squad` - Struct definitions (highlighted as struct keywords)
- `collab` - Interface definitions (highlighted as interface keywords)
- `map` - Map types (highlighted as type keywords)
- `array` - Array types (highlighted as type keywords)
- `slice` - Slice types (highlighted as type keywords)
- `chan` - Channel types (highlighted as type keywords)

**Special Constructs:**
- `import` - Import statements (highlighted as import keywords)
- `package` - Package declarations (highlighted as package keywords)
- `nil` - Nil literals (highlighted as null keywords)
- `?` - Error propagation operator (highlighted as operator)
- `@` - Annotations (highlighted as decorator)
- `#` - Pragmas (highlighted as preprocessor)

#### Token Modifiers

- **SlangKeyword** - Applied to all Gen Z slang keywords
- **Nullable** - Applied to nullable types
- **Generic** - Applied to generic types and functions
- **Goroutine** - Applied to goroutine-related constructs
- **Channel** - Applied to channel-related constructs
- **ErrorProne** - Applied to functions that can error
- **Concurrent** - Applied to concurrent operations
- **Immutable** - Applied to immutable data
- **Reference** - Applied to reference types

#### Usage

The semantic highlighting provider automatically tokenizes CURSED code and provides rich syntax highlighting that editors can use to display code with appropriate colors and styles.

```rust
// Example usage in LSP server
let provider = SemanticHighlightingProvider::new();
let tokens = provider.get_semantic_tokens(content).await?;
let encoded = provider.encode_semantic_tokens(tokens);
```

### 2. Code Lens

Contextual information overlays that provide actionable insights directly in the code editor.

#### Code Lens Types

**Reference Information:**
- Function reference counts with "Show References" command
- Type usage counts across the codebase
- Variable read/write statistics

**Test Execution:**
- Test function status indicators (✅ passed, ❌ failed, ⚪ not run, 🔄 running)
- "Run Test" commands for individual test functions
- Test suite summaries and results

**Performance Metrics:**
- Average execution time for frequently called functions
- Memory allocation information
- Call frequency statistics
- Performance warnings for hot paths

**Memory Usage:**
- Heap allocation amounts per function
- Memory leak detection warnings
- Garbage collection frequency
- Stack usage information

#### Features

- **Configurable** - Enable/disable different lens types
- **Interactive** - Click lenses to execute commands
- **Real-time** - Updates based on code analysis and profiling data
- **Contextual** - Shows relevant information based on symbol type

#### Example Output

```cursed
slay calculate_fibonacci(n: i32) -> i32 {  // 47 references | ⚡ 2ms avg (1,203x called) | 🧠 64 KB
    lowkey (n <= 1) {
        vibez n;
    }
    vibez calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2);
}

slay test_fibonacci() {  // ✅ test_fibonacci | Run Test
    sus result = calculate_fibonacci(10);
    // Test implementation
}
```

### 3. Inlay Hints

Inline contextual information that appears directly in the code without modifying the source.

#### Hint Types

**Type Information:**
- Inferred types for variables without explicit type annotations
- Return types for functions with inferred returns
- Generic type parameter resolution

**Parameter Names:**
- Parameter names in function calls for better readability
- Especially useful for functions with many parameters

**Conversions:**
- Implicit type conversions and casts
- Helps identify potential type-related issues

**Error Propagation:**
- Error propagation hints for functions that may fail
- Suggestions to use `?` operator where appropriate

**Channel Directions:**
- Channel send/receive operation indicators
- Channel type and direction information

**Goroutine Information:**
- Functions that will run in goroutines
- Concurrency-related hints and warnings

#### Configuration Options

```rust
InlayHintConfig {
    show_type_hints: true,          // Show inferred types
    show_parameter_names: true,     // Show parameter names
    show_return_types: false,       // Show function return types
    show_conversions: true,         // Show implicit conversions
    show_error_propagation: true,   // Show error propagation info
    show_channel_hints: true,       // Show channel directions
    show_goroutine_hints: true,     // Show goroutine information
    max_hint_length: 50,            // Maximum hint text length
    only_complex_types: false,      // Only show hints for complex types
}
```

#### Example Output

```cursed
slay process_data(data: []string) {
    sus result: map<string, i32> = analyze(data);
    //  ^^^^^^^^^^^^^^^^^^^^ inferred type
    
    sus value = calculate(x: 5, y: 10);
    //                   ^^^^    ^^^^^ parameter names
    
    stan process_async(result); // goroutine - will run concurrently
    //   ^^^^^^^^^^^^^^^^^^^^^^ goroutine hint
}
```

### 4. Enhanced Symbol Support

Comprehensive symbol extraction and navigation with CURSED-specific enhancements.

#### Document Symbols

**Symbol Hierarchy:**
- Package declarations
- Import statements  
- Function declarations with parameters as children
- Struct definitions with fields as children
- Interface definitions with methods as children
- Variable and constant declarations
- Local symbols within function scopes

**CURSED-Specific Symbol Kinds:**
- `SlayFunction` - Function declarations using `slay`
- `SusVariable` - Variable declarations using `sus`
- `FactsConstant` - Constant declarations using `facts`
- `SquadStruct` - Struct definitions using `squad`
- `CollabInterface` - Interface definitions using `collab`
- `TestFunction` - Detected test functions
- `GenericFunction` - Functions with generic parameters

#### Workspace Symbol Search

**Features:**
- Fuzzy matching for quick symbol lookup
- Cross-file symbol search
- Symbol filtering by type and scope
- Performance-optimized caching

**Search Capabilities:**
- Name-based search with substring matching
- Fuzzy matching (e.g., "gu" matches "greet_user")
- Type-specific filtering
- Scope-aware results

#### Symbol Information

Each symbol includes:
- **Position Information** - Exact location in source code
- **Type Information** - Inferred or declared types
- **Visibility** - Public, private, package, protected
- **Attributes** - Async, generic, deprecated flags
- **Documentation** - Associated comments and docs
- **References** - All usage locations
- **Implementations** - Implementation locations for interfaces

#### Example Symbol Tree

```
📦 test_package
├── 📁 imports
│   └── "stdlib::io"
├── 🔧 functions
│   ├── slay greet(name: string) -> string
│   │   └── 📊 name: string (parameter)
│   └── slay test_greet() (test function)
├── 🏗️ types
│   ├── squad Person
│   │   ├── 📊 name: string
│   │   └── 📊 age: i32
│   └── collab Greeter
│       └── 🔧 greet(name: string) -> string
└── 📊 variables
    ├── facts PI = 3.14159 (constant)
    └── sus global_var = "test"
```

## IDE Integration

### VS Code Extension

The enhanced LSP features are designed to work seamlessly with VS Code through the CURSED language extension.

**Setup:**
1. Install the CURSED language extension
2. Extension automatically starts the LSP server
3. Features are enabled by default

**Configuration:**
```json
{
  "cursed.lsp.semanticHighlighting": true,
  "cursed.lsp.codeLens": {
    "enabled": true,
    "showReferences": true,
    "showTests": true,
    "showPerformance": false,
    "showMemory": false
  },
  "cursed.lsp.inlayHints": {
    "enabled": true,
    "showTypes": true,
    "showParameters": true,
    "maxLength": 50
  },
  "cursed.lsp.symbols": {
    "fuzzySearch": true,
    "workspaceSymbols": true
  }
}
```

### Neovim Integration

Integration with Neovim through the built-in LSP client.

**Setup:**
```lua
local lspconfig = require('lspconfig')

lspconfig.cursed_lsp.setup({
  cmd = {"cursed", "lsp", "--stdio"},
  filetypes = {"cursed"},
  root_dir = lspconfig.util.root_pattern("CursedPackage.toml", ".git"),
  settings = {
    cursed = {
      semanticHighlighting = true,
      codeLens = true,
      inlayHints = true,
    }
  }
})
```

### Other Editors

The LSP server follows the Language Server Protocol 3.17 specification and works with any editor that supports LSP:

- **Emacs** - Through lsp-mode or eglot
- **Vim** - Through vim-lsp or coc.nvim
- **Sublime Text** - Through LSP package
- **Kate** - Built-in LSP support
- **Eclipse** - Through LSP4E

## Performance Considerations

### Caching Strategy

- **Symbol Cache** - Symbols are cached per document for fast access
- **Type Cache** - Type information is cached to avoid repeated analysis
- **Incremental Updates** - Only re-analyze changed portions of code
- **Background Processing** - Heavy computations run in background threads

### Memory Management

- **Lazy Loading** - Features are loaded on demand
- **Cache Limits** - Configurable cache size limits
- **Garbage Collection** - Automatic cleanup of unused cached data
- **Resource Monitoring** - Monitor memory usage and cleanup as needed

### Scalability

- **Large Files** - Efficient handling of files with thousands of lines
- **Large Projects** - Workspace-wide symbol search with performance optimization
- **Concurrent Operations** - Thread-safe operations for concurrent requests
- **Batch Processing** - Batch multiple requests for better throughput

## Error Handling

### Graceful Degradation

- **Parser Errors** - Fall back to lexical analysis when parsing fails
- **Type Errors** - Provide basic features even with type checking failures
- **Performance Issues** - Disable expensive features automatically if needed
- **Resource Limits** - Graceful handling of memory and time limits

### Error Recovery

- **Partial Results** - Return partial results when complete analysis fails
- **Fallback Modes** - Switch to simpler analysis modes when needed
- **Error Reporting** - Detailed error logs for debugging
- **Automatic Retry** - Retry failed operations with exponential backoff

## Troubleshooting

### Common Issues

**Semantic Highlighting Not Working:**
- Verify the client supports semantic tokens
- Check that the token legend is properly configured
- Ensure content is valid CURSED syntax

**Code Lenses Not Appearing:**
- Verify code lens capability is enabled
- Check that the document contains analyzable symbols
- Ensure the content can be parsed successfully

**Inlay Hints Not Showing:**
- Verify client supports inlay hints (LSP 3.17+)
- Check inlay hint configuration settings
- Ensure hints are enabled for the current range

**Symbol Search Not Working:**
- Verify workspace folders are properly configured
- Check that files are discoverable and readable
- Ensure symbol cache is populated

### Debug Mode

Enable debug logging to troubleshoot issues:

```bash
RUST_LOG=debug cursed lsp --stdio
```

This will provide detailed logging of all LSP operations, including feature-specific debug information.

## Future Enhancements

### Planned Features

- **Call Hierarchy** - Complete call graph visualization
- **Type Hierarchy** - Type inheritance and implementation trees
- **Folding Ranges** - Intelligent code folding based on CURSED syntax
- **Selection Ranges** - Smart selection expansion
- **Document Links** - Clickable links to imported modules
- **Code Actions** - Quick fixes and refactoring suggestions

### Performance Improvements

- **Incremental Parsing** - Update only changed parts of the AST
- **Parallel Analysis** - Parallel symbol extraction and type checking
- **Streaming Results** - Stream large result sets for better responsiveness
- **Predictive Caching** - Pre-cache likely-to-be-requested information

### Advanced Features

- **Machine Learning** - AI-powered code completion and suggestions
- **Real-time Collaboration** - Multi-user editing support
- **Advanced Diagnostics** - Deeper static analysis and lint rules
- **Performance Profiling** - Integrated performance profiling and optimization suggestions

The CURSED LSP server represents a modern, feature-rich language server implementation that brings cutting-edge IDE capabilities to the CURSED programming language, making development more productive and enjoyable for users familiar with Gen Z slang and modern programming paradigms.
