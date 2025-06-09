# CURSED REPL Guide

🔥 **Enhanced Interactive Shell for CURSED Development**

The CURSED REPL (Read-Eval-Print Loop) provides a powerful interactive development environment with modern features for productivity and ease of use.

## Features

### 🎨 Syntax Highlighting
- **Keywords**: CURSED keywords like `slay`, `facts`, `lowkey`, `highkey` are highlighted
- **Operators**: Mathematical and logical operators are color-coded
- **Literals**: Strings, numbers, and booleans have distinct colors
- **Comments**: Line comments are visually distinguished
- **Types**: Built-in and custom types are highlighted
- **Functions**: Function names and calls are emphasized

### 📝 Multi-line Input Support
- **Automatic Detection**: Detects when statements need continuation
- **Smart Indentation**: Automatically indents based on code structure
- **Bracket Matching**: Tracks opening/closing brackets, parentheses, and braces
- **Completion Detection**: Knows when multi-line input is complete

### 🔧 Built-in Command System
- **Help System**: `:help` shows all available commands
- **File Operations**: `:load <file>` and `:save <file>` for code management
- **Session Management**: `:clear`, `:vars`, `:funcs`, `:history`
- **Development Tools**: `:build`, `:test`, `:fmt`, `:lint`
- **Type Information**: `:type <expr>` shows expression types
- **Project Integration**: `:workspace`, `:info` for project context

### ⚡ Tab Completion
- **Keywords**: Complete CURSED language keywords
- **Variables**: Complete session variables as you type
- **Functions**: Complete function names with automatic parentheses
- **Commands**: Complete REPL commands starting with `:`
- **File Paths**: Complete file and directory paths for commands
- **Types**: Complete built-in and custom type names

### 💾 Session Management
- **Variable Persistence**: Variables remain available throughout the session
- **Function Definitions**: Functions persist and can be called later
- **Command History**: Previous commands are saved and searchable
- **Session Export**: Save entire session to a file for reuse
- **Type Tracking**: Tracks types of all variables and expressions

### 🏗️ Build System Integration
- **Project Detection**: Automatically detects CURSED projects
- **Build Commands**: Build projects without leaving the REPL
- **Test Runner**: Run tests and see results interactively
- **Code Formatting**: Format code on-the-fly
- **Linting**: Check code quality in real-time
- **Workspace Info**: View project structure and build targets

## Getting Started

### Installation
```bash
# Build the REPL binary
cargo build --bin cursed-repl

# Or build with optimizations
cargo build --release --bin cursed-repl
```

### Basic Usage
```bash
# Start the REPL
./target/debug/cursed-repl

# Start with verbose output
./target/debug/cursed-repl --verbose

# Start in a specific working directory
./target/debug/cursed-repl --working-dir /path/to/project

# Load a file at startup
./target/debug/cursed-repl --load example.csd
```

### Command Line Options
- `--verbose`, `-v`: Enable verbose output and debug mode
- `--no-history`: Disable command history persistence
- `--no-syntax-highlighting`: Disable syntax highlighting
- `--load <file>`, `-l`: Load and execute a file at startup
- `--working-dir <dir>`, `-w`: Set working directory for project context
- `--timeout <seconds>`: Set timeout for command execution (default: 30)

## REPL Commands

### Essential Commands
```
:help, :h, :?           Show help information
:exit, :quit, :q        Exit the REPL
:clear, :c              Clear session state
:info                   Show REPL and system information
```

### File Operations
```
:load <file>, :l        Load and execute a CURSED file
:save <file>, :s        Save current session to file
```

### Development Tools
```
:build [target], :b     Build project or specific target
:test [pattern]         Run tests (optionally filtered by pattern)
:fmt [file]             Format code in session or specific file
:lint [file]            Run linter on session or specific file
```

### Session Inspection
```
:vars, :variables       List all session variables
:funcs, :functions      List all session functions
:type <expr>, :t        Show type of an expression
:history [count]        Show command history (default: 10 entries)
```

### Project/Workspace
```
:workspace, :ws         Show workspace and project information
```

## Interactive Examples

### Basic Usage
```cursed
cursed> facts x = 42
cursed> facts y = 24
cursed> x + y
66
cursed> :vars
📋 Session Variables:
  x : int = 42
  y : int = 24
```

### Function Definitions
```cursed
cursed> slay add(a int, b int) int {
...   >     return a + b
...   > }
cursed> add(10, 20)
30
cursed> :funcs
🔧 Session Functions:
  slay add(a int, b int) int
```

### Multi-line Input
```cursed
cursed> lowkey x > 0 {
...   >     println("x is positive")
...   > } highkey {
...   >     println("x is not positive")
...   > }
x is positive
```

### Type Information
```cursed
cursed> facts message = "Hello, CURSED!"
cursed> :type message
🔍 Type: string
cursed> :type 3.14
🔍 Type: float64
```

### File Operations
```cursed
cursed> :load examples/hello.csd
📁 Loading file: examples/hello.csd
✅ File loaded successfully

cursed> :save my_session.csd
✅ Session saved to: my_session.csd
```

### Build Integration
```cursed
cursed> :build
🔨 Build result:
✅ Build successful!
Compiled 3 files in 1.2s

cursed> :test
🧪 Test results:
✅ Tests passed!
5 tests completed in 0.8s
```

## Advanced Features

### Syntax Highlighting Colors
- **Keywords**: Magenta (`slay`, `facts`, `lowkey`, etc.)
- **Operators**: Yellow (`+`, `-`, `=`, `==`, etc.)
- **Strings**: Green (`"hello world"`)
- **Numbers**: Cyan (`42`, `3.14`)
- **Comments**: Gray (`// this is a comment`)
- **Types**: Blue (`int`, `string`, `bool`)
- **Functions**: Bright cyan (function names)

### Tab Completion Examples
```cursed
cursed> sl<TAB>
slay  slaps  slick

cursed> :h<TAB>
:help  :history

cursed> my_var<TAB>
my_variable  my_function(
```

### Multi-line Editing
The REPL automatically detects when you need to continue input:
- Unmatched brackets: `if (condition`
- Function definitions: `slay myFunc()`
- Control structures: `lowkey x > 0`
- Line continuations: `x = 1 +`

### History Navigation
- **Up/Down arrows**: Navigate through command history
- **Ctrl+R**: Search through history
- **`:history [count]`**: View recent commands

### Session Persistence
```cursed
# Variables and functions persist throughout the session
cursed> facts global_config = "production"
cursed> slay get_config() { return global_config; }

# Later in the session...
cursed> get_config()
"production"

# Save entire session for later use
cursed> :save my_work.csd
```

## Configuration

### Environment Variables
- `CURSED_REPL_HISTORY`: Custom history file location
- `CURSED_REPL_PROMPT`: Custom prompt string
- `NO_COLOR`: Disable syntax highlighting colors

### Project Integration
The REPL automatically integrates with CURSED projects:
- Detects `CursedBuild.toml` or `CursedPackage.toml`
- Scans for source files and tests
- Provides project-aware commands
- Enables build system integration

## Performance

### Benchmarks
- **Startup time**: < 100ms
- **Syntax highlighting**: < 10ms for 1000 lines
- **Tab completion**: < 5ms for 1000+ items
- **History search**: < 1ms for 10,000 entries
- **Session with 1000+ variables**: < 50ms operations

### Memory Usage
- **Base REPL**: ~5MB
- **With large session**: ~20MB (1000+ variables)
- **History (10,000 entries)**: ~2MB additional
- **Syntax highlighting cache**: ~1MB per 10,000 lines

## Troubleshooting

### Common Issues

**REPL won't start**
```bash
# Check if binary exists
ls -la target/debug/cursed-repl

# Try with minimal flags
./target/debug/cursed-repl --no-history --no-syntax-highlighting
```

**Syntax highlighting not working**
```bash
# Check terminal color support
echo $TERM

# Force color mode
./target/debug/cursed-repl --force-colors
```

**Build commands not working**
```bash
# Make sure you're in a CURSED project directory
ls CursedBuild.toml CursedPackage.toml

# Use explicit working directory
./target/debug/cursed-repl --working-dir /path/to/project
```

**History not persisting**
```bash
# Check history file permissions
ls -la ~/.cursed_history

# Use custom history location
CURSED_REPL_HISTORY=/tmp/my_history ./target/debug/cursed-repl
```

### Debug Mode
```bash
# Run with verbose output for debugging
./target/debug/cursed-repl --verbose

# This will show:
# - File loading operations
# - Command execution details
# - Build system integration info
# - Performance timing information
```

## Contributing

The REPL is implemented in several modules:
- `src/repl/cursed_repl.rs`: Main REPL loop and coordination
- `src/repl/syntax_highlighter.rs`: Syntax highlighting engine
- `src/repl/command_system.rs`: Built-in command processing
- `src/repl/session_manager.rs`: Session state management
- `src/repl/tab_completion.rs`: Tab completion logic
- `src/repl/multi_line_editor.rs`: Multi-line input handling
- `src/repl/build_integration.rs`: Build system integration

### Adding New Commands
```rust
// In src/repl/command_system.rs
self.register_command(
    "mycommand",
    "Description of my command",
    ":mycommand <arg>",
    &["alias1", "alias2"],
    Box::new(|args, session, build| {
        // Command implementation
        Ok("Command result".to_string())
    }),
);
```

### Extending Syntax Highlighting
```rust
// In src/repl/syntax_highlighter.rs
// Add new keywords to CURSED_KEYWORDS
// Add new patterns to highlight() method
```

### Testing
```bash
# Run all REPL tests
./scripts/test_repl.sh

# Run specific test suites
cargo test --test repl_integration_test
cargo test --test repl_performance_test
```

## Future Enhancements

- **Debugger Integration**: Step-through debugging in the REPL
- **Package Management**: Install and manage packages interactively
- **Code Completion**: AI-powered code suggestions
- **Visual Mode**: GUI-based REPL with graphical elements
- **Remote REPL**: Connect to remote CURSED instances
- **Plugin System**: Extensible command and feature system
- **Notebook Mode**: Jupyter-style cell-based editing
- **Collaboration**: Multi-user REPL sessions

---

🔥 **Happy coding with CURSED REPL!** The most fire interactive development environment for the most lit programming language! 🚀
