# CURSED Language Support for VS Code

A comprehensive VS Code extension for the **CURSED** programming language - where Gen Z meets systems programming! 🔥

## Features

### 🔥 Core Language Support
- **Syntax Highlighting**: Full syntax highlighting for CURSED (.csd) files with Gen Z slang keywords
- **Language Configuration**: Proper bracket matching, comment handling, and indentation rules
- **Code Snippets**: Pre-built snippets for common CURSED constructs and patterns
- **File Icons**: Custom CURSED file icons in the explorer

### 🧠 Intelligent Code Assistance
- **Language Server Protocol (LSP)**: Full integration with CURSED LSP server
- **Diagnostics**: Real-time error detection and warnings
- **Auto-completion**: Intelligent code completion with context awareness
- **Hover Information**: Detailed type and documentation information on hover
- **Go to Definition**: Navigate to symbol definitions across your codebase
- **Find References**: Find all references to symbols
- **Document Symbols**: Outline view of document structure
- **Workspace Symbols**: Search symbols across the entire workspace
- **Rename Symbol**: Safe renaming with automatic reference updates

### 🛠️ Development Tools
- **Build Integration**: One-click build, run, test, and clean operations
- **Task Runner**: Integrated task provider for CURSED build system
- **Terminal Integration**: Smart terminal management for different operations
- **REPL Support**: Interactive CURSED REPL sessions
- **Package Manager**: Dependency management with visual package view
- **Linting**: Code quality checks with configurable rules
- **Formatting**: Automatic code formatting with CURSED style preferences

### 📊 Project Management
- **Project Explorer**: Custom project view showing CURSED files, tests, and configuration
- **Dependency View**: Visual representation of project dependencies
- **Status Bar**: Real-time status updates for builds, tests, and LSP
- **Output Channels**: Organized output for different operations (build, test, LSP, etc.)
- **Problem Matchers**: Automatic error parsing and problem panel integration

### 🐛 Debugging Support
- **Debug Configuration**: Pre-configured debug settings for CURSED programs
- **Launch Configurations**: Ready-to-use launch configurations
- **Breakpoint Support**: Full debugging capabilities (when DAP is implemented)

### ⚡ Advanced Features
- **Goroutine Analysis**: Special support for CURSED's goroutine system
- **Channel Visualization**: Channel operation analysis and deadlock detection
- **AST Viewer**: Visualize Abstract Syntax Tree for debugging
- **Type Information**: Detailed type hierarchy and interface information
- **Performance Analysis**: Basic performance metrics and benchmarking

## Installation

### From VS Code Marketplace
1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X)
3. Search for "CURSED Language Support"
4. Click Install

### From VSIX Package
1. Download the `.vsix` file from releases
2. In VS Code: `Ctrl+Shift+P` → "Extensions: Install from VSIX"
3. Select the downloaded file

## Getting Started

### Prerequisites
- **CURSED Compiler**: Install the CURSED language compiler
- **CURSED LSP Server**: The language server should be available as `cursed-lsp`

### Quick Start
1. **Create a new CURSED project**:
   - `Ctrl+Shift+P` → "CURSED: New CURSED Project"
   - Or use the welcome view in the CURSED Project panel

2. **Open an existing project**:
   - Open any folder containing `.csd` files or `CursedPackage.toml`

3. **Start coding**:
   - Create a `.csd` file and start writing CURSED code
   - Enjoy syntax highlighting, completion, and error checking!

## Configuration

### Language Server Settings
```json
{
    "cursed.languageServer.enabled": true,
    "cursed.languageServer.command": "cursed-lsp",
    "cursed.languageServer.args": [],
    "cursed.languageServer.debug": false
}
```

### Formatting Settings
```json
{
    "cursed.format.enable": true,
    "cursed.format.indentSize": 4,
    "cursed.format.useTabs": false,
    "cursed.format.lineWidth": 120,
    "cursed.format.braceStyle": "same-line"
}
```

### Linting Settings
```json
{
    "cursed.lint.enable": true,
    "cursed.lint.checkStyle": true,
    "cursed.lint.checkPerformance": true,
    "cursed.lint.checkSecurity": true
}
```

## Commands

| Command | Description | Keybinding |
|---------|-------------|------------|
| `CURSED: Build Project` | Build the current project | `Ctrl+Shift+B` |
| `CURSED: Run Project` | Run the current project | `Ctrl+F5` |
| `CURSED: Run Tests` | Execute project tests | `Ctrl+Shift+T` |
| `CURSED: Format Document` | Format current document | `Ctrl+Shift+F` |
| `CURSED: Open REPL` | Start interactive REPL | `Ctrl+Shift+R` |
| `CURSED: Show Type Information` | Show type info at cursor | `Ctrl+K Ctrl+I` |
| `CURSED: New CURSED Project` | Create new project | - |
| `CURSED: Install Dependencies` | Install project dependencies | - |
| `CURSED: Run Linter` | Run code linter | - |

## Language Features

### Gen Z Syntax Highlighting
The extension provides full syntax highlighting for CURSED's unique Gen Z-inspired syntax:

```cursed
// Function definitions with "slay"
slay main() {
    // Variables with "sus" (mutable) and "facts" (immutable)
    sus name = "CURSED"
    facts version = "1.0.0"
    
    // Control flow with style
    vibe_check name {
        name "CURSED" => {
            println("This language is absolutely fire! 🔥")?
        }
        basic => {
            println("Still vibes though")?
        }
    }
    
    // Loops with "lowkey" and "highkey"
    lowkey (sus i = 0; i < 5; i++) {
        println("Loop " + string(i) + " is sending me!")?
    }
}
```

### Goroutine Support
Special highlighting and analysis for CURSED's goroutine features:

```cursed
// Spawn goroutines with "stan"
stan calculate_fibonacci(10)

// Channel operations
sus ch = make(chan<int>, 5)
stan sender(ch)
stan receiver(ch)
```

## Troubleshooting

### Language Server Not Starting
1. Ensure `cursed-lsp` is installed and in your PATH
2. Check the CURSED Language Server output channel for errors
3. Try restarting the language server: `Ctrl+Shift+P` → "CURSED: Restart Language Server"

### Syntax Highlighting Issues
1. Ensure the file has a `.csd` extension
2. Check if the language is correctly detected in the bottom-right corner of VS Code
3. Try reloading the window: `Ctrl+Shift+P` → "Developer: Reload Window"

### Build/Run Commands Not Working
1. Ensure the CURSED compiler is installed
2. Check that you're in a valid CURSED project directory
3. Review the build output in the CURSED Build output channel

## Contributing

We welcome contributions! The extension is open source and available on GitHub.

### Development Setup
1. Clone the repository
2. `cd editors/vscode`
3. `npm install`
4. `npm run compile`
5. Press F5 to launch Extension Development Host

### Building
- `npm run compile` - Compile TypeScript
- `npm run watch` - Watch for changes
- `npm run package` - Package as VSIX
- `npm run lint` - Run ESLint

## Changelog

### 1.0.0
- Initial release
- Full LSP integration
- Project management
- Build and test integration
- Comprehensive syntax highlighting
- Gen Z slang keyword support
- Goroutine and channel analysis

## License

MIT License - see LICENSE file for details.

## Support

- **Issues**: [GitHub Issues](https://github.com/ghuntley/cursed/issues)
- **Documentation**: [CURSED Language Docs](https://github.com/ghuntley/cursed)
- **Community**: Join the CURSED community for support and discussions

---

**Ready to slay some code?** 🔥 Happy coding with CURSED!
