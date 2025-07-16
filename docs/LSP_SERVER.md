# CURSED Language Server Protocol (LSP) Implementation

The CURSED Language Server provides comprehensive IDE support for the CURSED programming language through the Language Server Protocol. This enables rich editing experiences in any LSP-compatible editor.

## Features

### Core Language Support
- **Syntax Highlighting**: Semantic token-based highlighting for CURSED keywords, types, and constructs
- **Error Diagnostics**: Real-time syntax and semantic error detection with detailed messages
- **Code Completion**: Intelligent completion for keywords, stdlib functions, and workspace symbols
- **Hover Information**: Rich hover tooltips with type information and documentation

### Navigation & Refactoring
- **Go to Definition**: Navigate to symbol definitions across the workspace
- **Find References**: Find all references to symbols throughout the codebase
- **Workspace Symbols**: Quick symbol search across all open files
- **Document Symbols**: Outline view of current document structure

### Code Quality
- **Code Formatting**: Automatic code formatting with consistent indentation
- **Incremental Analysis**: Fast, incremental parsing and analysis on document changes
- **Multi-file Support**: Full workspace analysis with cross-file symbol resolution

## Installation & Setup

### Building the LSP Server

```bash
# Build the CURSED LSP server
cargo build --release --bin cursed-lsp

# The binary will be available at:
# target/release/cursed-lsp
```

### VS Code Setup

1. Install the CURSED language extension (coming soon)
2. Or configure manually in `settings.json`:

```json
{
    "languageServerExample.trace.server": "verbose",
    "cursed.lsp.path": "/path/to/cursed-lsp"
}
```

### Vim/Neovim Setup

Using `nvim-lspconfig`:

```lua
local lspconfig = require('lspconfig')

lspconfig.cursed = {
    cmd = { '/path/to/cursed-lsp' },
    filetypes = { 'cursed' },
    root_dir = lspconfig.util.root_pattern('.git', 'CursedPackage.toml'),
    settings = {}
}

-- Auto-start LSP for .csd files
vim.api.nvim_create_autocmd("FileType", {
    pattern = "cursed",
    callback = function()
        vim.lsp.start({
            name = "cursed-lsp",
            cmd = { "/path/to/cursed-lsp" },
            root_dir = vim.fs.dirname(vim.fs.find({'.git', 'CursedPackage.toml'}, { upward = true })[1]),
        })
    end,
})
```

### Emacs Setup

Using `lsp-mode`:

```elisp
(use-package lsp-mode
  :init
  (setq lsp-keymap-prefix "C-c l")
  :hook ((cursed-mode . lsp))
  :commands lsp)

;; Add CURSED language server
(lsp-register-client
 (make-lsp-client :new-connection (lsp-stdio-connection "/path/to/cursed-lsp")
                  :major-modes '(cursed-mode)
                  :server-id 'cursed-lsp))
```

### Sublime Text Setup

Add to LSP settings:

```json
{
    "clients": {
        "cursed-lsp": {
            "enabled": true,
            "command": ["/path/to/cursed-lsp"],
            "selector": "source.cursed"
        }
    }
}
```

## Usage

### Starting the Server

The LSP server communicates via stdin/stdout according to the LSP specification:

```bash
# Start the server (usually done by your editor)
cursed-lsp

# Check version
cursed-lsp --version

# Show help
cursed-lsp --help
```

### Supported File Types

- **Primary**: `.csd` files (CURSED source code)
- **Configuration**: `CursedPackage.toml`, `CursedWorkspace.toml`

### Code Completion

The LSP provides completion for:

#### CURSED Keywords
- `sus` - Variable declaration
- `slay` - Function declaration  
- `damn` - Return statement
- `vibez` - Module/namespace
- `yeet` - Import statement
- `bestie` - For loop
- `stan` - While loop
- `ready` - Select statement
- `based`/`cap` - Boolean values
- `facts` - Constant declaration

#### Type Keywords
- `lit` - Boolean type
- `tea` - String type
- `drip` - Float type
- `normie` - Integer type
- `smol`/`mid`/`thicc` - Integer sizes
- `snack`/`meal` - Float sizes
- `byte` - Byte type
- `sip` - Character type

#### Standard Library Functions
- `vibez.spill()` - Print output
- `math.*` - Mathematical operations
- `string.*` - String manipulation
- `crypto.*` - Cryptographic functions
- `json.*` - JSON parsing/serialization
- And 200+ more stdlib functions

### Error Diagnostics

The server provides real-time error detection:

- **Syntax Errors**: Invalid CURSED syntax
- **Type Errors**: Type mismatches and violations
- **Semantic Errors**: Undefined symbols, scope violations
- **Import Errors**: Missing or invalid module imports

### Hover Information

Rich hover tooltips show:
- Symbol type information
- Function signatures
- Variable types
- Module documentation
- Error explanations

### Code Formatting

Automatic formatting with:
- Consistent 4-space indentation
- Proper brace alignment
- Statement organization
- Import grouping

## Configuration

### Environment Variables

- `CURSED_LSP_LOG`: Set logging level (`trace`, `debug`, `info`, `warn`, `error`)
- `CURSED_LSP_CACHE_DIR`: Custom cache directory for analysis results
- `CURSED_LSP_STDLIB_PATH`: Custom path to CURSED standard library

### Workspace Configuration

Create `.cursed-lsp.toml` in your workspace root:

```toml
[lsp]
# Enable/disable features
completion = true
diagnostics = true
hover = true
formatting = true

# Analysis settings
max_file_size = "1MB"
analysis_timeout = "5s"
cache_enabled = true

# Formatting options
indent_size = 4
max_line_length = 100
format_on_save = true

[stdlib]
# Standard library configuration
path = "stdlib/"
auto_import = true
completion_priority = "high"

[diagnostics]
# Error reporting settings
show_warnings = true
show_hints = true
max_diagnostics_per_file = 100
```

## Architecture

### Components

```
cursed-lsp
├── server.rs          # Main LSP server implementation
├── completion.rs      # Code completion provider
├── diagnostics.rs     # Error diagnostics engine
├── formatting.rs      # Code formatting
├── navigation.rs      # Go-to-definition, references
├── document.rs        # Document synchronization
├── workspace.rs       # Workspace management
└── semantic_highlighting.rs  # Syntax highlighting
```

### Analysis Pipeline

1. **Lexical Analysis**: Tokenization using CURSED lexer
2. **Syntax Analysis**: AST generation with CURSED parser
3. **Semantic Analysis**: Type checking and symbol resolution
4. **Symbol Extraction**: Build symbol table for navigation
5. **Diagnostic Generation**: Error/warning reporting
6. **Completion Generation**: Context-aware completion items

### Performance

- **Incremental Parsing**: Only re-analyze changed parts
- **Async Processing**: Non-blocking analysis and responses
- **Caching**: Intelligent caching of analysis results
- **Memory Efficient**: Minimal memory footprint for large codebases

## Troubleshooting

### Common Issues

#### Server Not Starting
```bash
# Check if binary exists and is executable
ls -la target/release/cursed-lsp
chmod +x target/release/cursed-lsp

# Test manual startup
target/release/cursed-lsp --version
```

#### No Completions
- Verify file is saved as `.csd` extension
- Check LSP client configuration
- Enable debug logging to see LSP communication

#### Slow Performance
- Reduce `max_file_size` in configuration
- Enable caching in `.cursed-lsp.toml`
- Check for large files in workspace

#### Parse Errors
- Verify CURSED syntax is correct
- Check for missing dependencies
- Update to latest CURSED version

### Debug Logging

Enable verbose logging:

```bash
# Set environment variable
export CURSED_LSP_LOG=debug

# Or start with logging
CURSED_LSP_LOG=trace cursed-lsp
```

### LSP Communication Debug

Most editors support LSP communication logging:

**VS Code**: Set `"cursed.trace.server": "verbose"` in settings
**Vim/Neovim**: `:lua vim.lsp.set_log_level("debug")`
**Emacs**: `(setq lsp-log-io t)`

## Contributing

### Development Setup

```bash
# Clone and build
git clone https://github.com/ghuntley/cursed
cd cursed
cargo build --bin cursed-lsp

# Run tests
cargo test lsp_integration_tests
cargo test --lib lsp

# Format code
cargo fmt

# Check lints
cargo clippy --bin cursed-lsp
```

### Adding New Features

1. **Extend ServerCapabilities** in `initialize()` method
2. **Implement LSP Methods** in `LanguageServer` trait
3. **Add Tests** in `tests/lsp_integration_tests.rs`
4. **Update Documentation** in this file

### Testing LSP Features

```bash
# Test specific LSP functionality
cargo test test_completion_keywords
cargo test test_hover_information  
cargo test test_formatting
cargo test test_go_to_definition

# Integration tests
cargo test lsp_integration_tests

# Manual testing with LSP client
./scripts/test_lsp_manually.sh
```

## Specifications

### LSP Version
- **Protocol Version**: 3.17
- **Transport**: JSON-RPC over stdin/stdout
- **Encoding**: UTF-8

### Supported LSP Methods

#### Lifecycle
- `initialize`
- `initialized`
- `shutdown`
- `exit`

#### Document Synchronization
- `textDocument/didOpen`
- `textDocument/didChange`
- `textDocument/didSave`
- `textDocument/didClose`

#### Language Features
- `textDocument/completion`
- `textDocument/hover`
- `textDocument/definition`
- `textDocument/references`
- `textDocument/formatting`
- `textDocument/semanticTokens`
- `workspace/symbol`

#### Diagnostics
- `textDocument/publishDiagnostics`

## Roadmap

### Planned Features
- **Incremental Compilation**: Fast feedback for large projects
- **Code Actions**: Quick fixes and refactoring suggestions
- **Inlay Hints**: Type annotations and parameter hints
- **Call Hierarchy**: Function call trees
- **Folding Ranges**: Code folding support
- **Signature Help**: Parameter hints during function calls
- **Rename Symbol**: Workspace-wide symbol renaming
- **Document Highlights**: Symbol highlighting in current file

### Future Enhancements
- **Debugger Support**: DAP (Debug Adapter Protocol) integration
- **Testing Integration**: Test discovery and execution
- **Performance Profiling**: Built-in performance analysis
- **Package Management**: Integration with CURSED package manager
- **Documentation Generation**: Auto-generate docs from code
- **Code Metrics**: Complexity analysis and code quality metrics

## License

The CURSED Language Server is licensed under the MIT License. See [LICENSE](../LICENSE) for details.
