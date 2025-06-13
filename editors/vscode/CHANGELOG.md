# Changelog

All notable changes to the CURSED Language Support extension will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2024-01-15

### Added
- 🔥 **Initial Release** - CURSED Language Support for VS Code
- **Comprehensive Language Server Integration**
  - Real-time diagnostics and error checking
  - Auto-completion with context awareness
  - Hover information for types and documentation
  - Go to definition and find references
  - Document and workspace symbol search
  - Rename symbol with automatic reference updates
- **Advanced Syntax Highlighting**
  - Full support for CURSED's Gen Z slang syntax
  - Special highlighting for `slay`, `sus`, `facts`, `stan` keywords
  - Goroutine and channel operation highlighting
  - Control flow constructs (`vibe_check`, `lowkey`, `highkey`)
  - String interpolation and template support
- **Project Management Features**
  - Custom project explorer with CURSED file organization
  - Dependency view with package visualization
  - Configuration file recognition (`CursedPackage.toml`, `CursedBuild.toml`)
  - Project structure analysis and display
- **Build System Integration**
  - One-click build, run, test, and clean operations
  - Integrated task provider for CURSED build system
  - Real-time status updates in status bar
  - Comprehensive output channels for different operations
- **Developer Tools**
  - Interactive REPL integration
  - Code formatting with CURSED style preferences
  - Linting with configurable quality checks
  - Package manager integration for dependencies
  - Documentation generation support
  - Benchmark execution capabilities
- **Advanced Code Analysis**
  - AST (Abstract Syntax Tree) viewer for debugging
  - Type information display with hierarchy
  - Goroutine analysis and deadlock detection
  - Channel operation visualization
  - Performance analysis and metrics
- **Terminal Integration**
  - Smart terminal management for different tasks
  - Background command execution
  - Terminal-specific welcome messages
  - Automatic cleanup and lifecycle management
- **Debug Support Foundation**
  - Debug configuration templates
  - Launch configuration snippets
  - Preparation for future DAP (Debug Adapter Protocol) support
- **User Experience Enhancements**
  - Welcome screen for new users
  - Getting started guidance
  - New project creation wizard
  - Context-sensitive command availability
  - Keyboard shortcuts for common operations
- **Configuration Options**
  - Language server settings (command, args, debug mode)
  - Formatting preferences (indent, tabs, line width, brace style)
  - Linting options (style, performance, security checks)
  - Completion settings (snippets, slang suggestions)
  - UI preferences (inlay hints, semantic highlighting)

### Technical Features
- **TypeScript Implementation**
  - Fully typed extension with comprehensive error handling
  - Modular architecture with separate managers for different concerns
  - Async/await pattern for non-blocking operations
  - Proper resource cleanup and disposal
- **VS Code Integration**
  - Custom tree data providers for project and dependency views
  - Task definition and problem matcher integration
  - Output channel management with categorization
  - Status bar integration with progress indication
  - File system watcher for real-time updates
- **Language Server Protocol**
  - Full LSP client implementation
  - Custom CURSED-specific LSP extensions
  - Automatic server restart and error recovery
  - Configuration synchronization
- **Extension Architecture**
  - Clean separation of concerns with dedicated modules
  - Event-driven architecture for responsive UI
  - Configurable and extensible design
  - Production-ready error handling and logging

### Command Palette
- `CURSED: New CURSED Project` - Create a new CURSED project
- `CURSED: Build Project` - Build the current project
- `CURSED: Run Project` - Execute the current project
- `CURSED: Run Tests` - Run project test suite
- `CURSED: Clean Build` - Clean build artifacts
- `CURSED: Open REPL` - Start interactive REPL session
- `CURSED: Install Dependencies` - Install project dependencies
- `CURSED: Update Dependencies` - Update project dependencies
- `CURSED: Format Document` - Format current document
- `CURSED: Run Linter` - Execute code linter
- `CURSED: Generate Documentation` - Generate project documentation
- `CURSED: Run Benchmarks` - Execute performance benchmarks
- `CURSED: Show AST Node` - Display AST for current selection
- `CURSED: Show Type Information` - Show detailed type information
- `CURSED: Show Goroutine Information` - Analyze goroutine usage
- `CURSED: Show Channel Information` - Analyze channel operations
- `CURSED: Show Project Structure` - Display project structure
- `CURSED: Show Diagnostics` - Focus on problems panel
- `CURSED: Restart Language Server` - Restart LSP server

### Keyboard Shortcuts
- `Ctrl+Shift+B` - Build Project
- `Ctrl+F5` - Run Project
- `Ctrl+Shift+T` - Run Tests
- `Ctrl+Shift+F` - Format Document
- `Ctrl+Shift+R` - Open REPL
- `Ctrl+K Ctrl+I` - Show Type Information

### File Support
- `.csd` files - CURSED source files
- `CursedPackage.toml` - Package configuration
- `CursedBuild.toml` - Build configuration
- `.cursed-doc.toml` - Documentation configuration
- `.cursed-lint.toml` - Linting configuration

### Marketplace Information
- **Categories**: Programming Languages, Linters, Formatters, Snippets, Debuggers, Testing
- **Keywords**: cursed, programming language, gen z, slang, go-like, systems programming
- **License**: MIT
- **Repository**: https://github.com/ghuntley/cursed

### System Requirements
- VS Code 1.74.0 or later
- CURSED compiler and language server (`cursed-lsp`)
- Node.js (for extension development)

## [Unreleased]

### Planned Features
- **Debug Adapter Protocol (DAP) Integration**
  - Full debugging support with breakpoints
  - Variable inspection and watch expressions
  - Step-through debugging for CURSED programs
  - Goroutine debugging with concurrent execution visualization

- **Enhanced Language Features**
  - Code actions and quick fixes
  - Extract method/variable refactoring
  - Import organization and optimization
  - More comprehensive code snippets

- **Advanced Project Features**
  - Multi-workspace support
  - Project templates for different types of applications
  - Integrated package publishing
  - Remote development support

- **Performance Improvements**
  - Incremental compilation support
  - Faster LSP communication
  - Optimized syntax highlighting
  - Reduced memory footprint

- **Community Features**
  - Extension marketplace for CURSED packages
  - Code sharing and collaboration tools
  - Community-driven snippet library
  - Integration with popular development tools

### Known Issues
- Language server startup may take a few seconds on first run
- Large files (>10MB) may have slower syntax highlighting
- Some advanced LSP features require latest CURSED compiler version

### Feedback
We welcome feedback and contributions! Please report issues or suggest features on our GitHub repository.

---

**Note**: This extension is in active development. New features and improvements are added regularly based on community feedback and the evolution of the CURSED language.
