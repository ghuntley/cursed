# CURSED IDE Integration

Professional-grade IDE support for the CURSED programming language across VS Code, IntelliJ IDEA, and Vim/Neovim.

## 🚀 Features

### VS Code Extension (Advanced)
- **Advanced Syntax Highlighting** with semantic tokens
- **IntelliSense** with type inference and auto-completion
- **Real-time Error Checking** with quick fixes
- **Integrated Debugging** with breakpoints and variable inspection
- **Goroutine Debugging** support
- **Code Formatting** and linting
- **Performance Analysis** tools
- **Cross-compilation** support
- **Project Management** with dependency handling
- **Custom Themes** optimized for CURSED

### IntelliJ IDEA Plugin
- **Full Language Support** with parser integration
- **Intelligent Code Completion** with context awareness
- **Advanced Refactoring** capabilities
- **Built-in Build System** integration
- **Debugging Support** with advanced features
- **Project Templates** and scaffolding
- **Code Analysis** and inspections
- **Documentation Integration**

### Vim/Neovim Plugin
- **Comprehensive Syntax Highlighting**
- **LSP Integration** for modern IDE features
- **Code Completion** with omni-completion
- **Debugging Support** via DAP
- **File Type Detection** and configuration
- **Project Management** commands
- **Performance Tools** integration
- **Customizable Key Bindings**

## 📦 Quick Installation

```bash
# Install all IDE integrations
./install-ide-support.sh

# Install specific components
./install-ide-support.sh --vscode     # VS Code only
./install-ide-support.sh --vim        # Vim/Neovim only
./install-ide-support.sh --intellij   # IntelliJ only
```

## 🔧 Manual Installation

### VS Code

1. **From Marketplace** (Recommended):
   ```bash
   code --install-extension cursed-lang.cursed-language-advanced
   ```

2. **From Source**:
   ```bash
   cd cursed-vscode-extension-advanced
   npm install
   npm run compile
   vsce package
   code --install-extension cursed-language-advanced-2.0.0.vsix
   ```

### IntelliJ IDEA

1. **Build and Install**:
   ```bash
   cd cursed-intellij-plugin
   gradle clean buildPlugin
   ```

2. **Install in IntelliJ**:
   - File → Settings → Plugins
   - Install Plugin from Disk
   - Select the generated `.zip` file

### Vim/Neovim

#### Using Package Managers

**vim-plug**:
```vim
Plug 'ghuntley/cursed', {'rtp': 'cursed-vim-advanced'}
```

**Vundle**:
```vim
Plugin 'ghuntley/cursed', {'rtp': 'cursed-vim-advanced'}
```

**Manual Installation**:
```bash
# Vim
mkdir -p ~/.vim/pack/cursed/start
cp -r cursed-vim-advanced ~/.vim/pack/cursed/start/cursed

# Neovim
mkdir -p ~/.config/nvim/pack/cursed/start
cp -r cursed-vim-advanced ~/.config/nvim/pack/cursed/start/cursed
```

## ⚙️ Configuration

### VS Code Settings

```json
{
  "cursed.compiler.path": "cursed-zig",
  "cursed.lsp.enabled": true,
  "cursed.format.onSave": true,
  "cursed.lint.onSave": true,
  "cursed.intellisense.enabled": true,
  "cursed.debug.enableBreakpoints": true,
  "cursed.performance.enableAnalysis": true
}
```

### Vim Configuration

```vim
" Enable CURSED plugin features
let g:cursed_compiler_path = 'cursed-zig'
let g:cursed_lsp_enabled = 1
let g:cursed_auto_format = 1
let g:cursed_auto_lint = 1
let g:cursed_debug_enabled = 1

" Custom key mappings
nmap <leader>cr <Plug>(cursed-run)
nmap <leader>cb <Plug>(cursed-build)
nmap <leader>ct <Plug>(cursed-test)
```

### LSP Server Configuration

The Language Server Protocol (LSP) server provides advanced IDE features across all editors.

**Server Capabilities**:
- Syntax highlighting with semantic tokens
- Code completion with type information
- Real-time diagnostics and error checking
- Go-to-definition and find references
- Hover information and documentation
- Code formatting and document symbols
- Rename refactoring
- Code actions and quick fixes

**Configuration**:
```json
{
  "enableSemanticTokens": true,
  "enableCodeLens": true,
  "enableInlayHints": true,
  "enableDiagnostics": true,
  "experimentalFeatures": false
}
```

## 🔍 Debugging

### VS Code Debugging

1. **Set Breakpoints**: Click in the gutter or press F9
2. **Start Debugging**: Press F5 or use the Debug panel
3. **Debug Configuration**:
   ```json
   {
     "type": "cursed",
     "request": "launch",
     "name": "Debug CURSED Program",
     "program": "${file}",
     "args": [],
     "stopOnEntry": false,
     "enableGoroutineDebugging": true
   }
   ```

### Vim Debugging (with nvim-dap)

```lua
-- Neovim DAP configuration
local dap = require('dap')

dap.adapters.cursed = {
  type = 'executable',
  command = 'cursed-debug-adapter',
}

dap.configurations.cursed = {
  {
    type = 'cursed',
    request = 'launch',
    name = 'Launch CURSED Program',
    program = '${file}',
  }
}
```

### Debugging Features

- **Breakpoints**: Line breakpoints with conditions
- **Variable Inspection**: View local and global variables
- **Goroutine Debugging**: Inspect concurrent execution
- **Call Stack**: Navigate execution frames
- **Expression Evaluation**: Evaluate expressions in context
- **Memory Inspection**: View memory allocation
- **Exception Handling**: Break on exceptions

## 🏗️ Project Templates

### Available Templates

```bash
# List available templates
cursed-scaffold list

# Create new projects
cursed-scaffold new web-app my-api
cursed-scaffold new cli-tool my-cli
cursed-scaffold new library math-utils
cursed-scaffold new game-engine my-game
```

### Template Features

- **Web Application**: HTTP server, database, authentication
- **CLI Tool**: Argument parsing, configuration, utilities
- **Library**: Reusable modules with documentation
- **Game Engine**: Graphics, input, physics integration
- **Desktop App**: GUI framework integration
- **Microservice**: Docker, monitoring, health checks

## 🎨 Themes and Appearance

### VS Code Themes

- **CURSED Dark Pro**: Optimized dark theme
- **CURSED Light Pro**: Professional light theme
- **CURSED High Contrast**: Accessibility-focused

### Syntax Highlighting

Advanced syntax highlighting with:
- **Semantic Tokens**: Context-aware coloring
- **Error Highlighting**: Real-time error indication
- **Type Information**: Color-coded type hints
- **Performance Hints**: Visual performance indicators

## 🚀 Performance Features

### Performance Analysis

- **Code Profiling**: Identify performance bottlenecks
- **Memory Analysis**: Track memory usage and leaks
- **Benchmarking**: Built-in benchmark runner
- **Optimization Hints**: Suggestions for performance improvements

### Commands

```bash
# VS Code
Ctrl+Shift+P → "CURSED: Analyze Performance"
Ctrl+Shift+P → "CURSED: Benchmark Code"
Ctrl+Shift+P → "CURSED: Memory Check"

# Vim
:CursedAnalyzePerformance
:CursedBenchmark
:CursedMemoryCheck
```

## 🔧 Build Integration

### Build System Support

- **Zig Build**: Native build system integration
- **Cross Compilation**: Multiple target support
- **Incremental Builds**: Fast development cycles
- **Optimization Levels**: Debug, release, release-fast

### Build Commands

```bash
# VS Code
F7 - Build current file
Ctrl+Shift+B - Build project
Ctrl+F5 - Run without debugging

# Vim
:CursedBuild - Build current file
:CursedBuildRelease - Release build
:CursedRun - Run current file
```

## 📚 Documentation Integration

### In-Editor Documentation

- **Hover Information**: Function signatures and descriptions
- **Go-to-Documentation**: Jump to detailed docs
- **Auto-generated Docs**: From code comments
- **API Reference**: Built-in standard library docs

### Documentation Commands

```bash
# VS Code
F1 → "CURSED: Show Documentation"

# Vim
:CursedDoc - Show documentation
K - Hover documentation (LSP)
```

## 🧪 Testing Integration

### Test Runner

- **Unit Tests**: Built-in test framework
- **Integration Tests**: End-to-end testing
- **Test Coverage**: Coverage analysis
- **Test Debugging**: Debug failing tests

### Test Commands

```bash
# VS Code
Ctrl+Shift+T - Run tests
Ctrl+Shift+P → "CURSED: Run Tests"

# Vim
:CursedTest - Run all tests
:CursedTestFile - Run current file tests
```

## 🔄 Version Control Integration

### Git Integration

- **Syntax Highlighting**: Git-aware highlighting
- **Change Indicators**: Modified line indicators
- **Blame Information**: Author and commit info
- **Merge Conflict Resolution**: Visual merge tools

## 🛠️ Troubleshooting

### Common Issues

1. **LSP Server Not Starting**:
   ```bash
   # Check if LSP server is available
   which cursed-lsp
   
   # Build from source if needed
   cd cursed && zig build cursed-lsp
   ```

2. **Syntax Highlighting Issues**:
   - Ensure file extension is `.💀` (skull emoji is the proper CURSED file extension)
   - Restart editor after installation
   - Check language association in editor settings

3. **Debugging Not Working**:
   - Verify debug adapter is installed: `cursed-debug-adapter`
   - Check debug configuration in editor
   - Ensure CURSED compiler supports debug mode

4. **Performance Issues**:
   - Disable real-time linting if needed
   - Reduce LSP verbosity in settings
   - Use incremental compilation

### Getting Help

- **GitHub Issues**: [Report bugs and request features](https://github.com/ghuntley/cursed/issues)
- **Documentation**: [Complete documentation](https://docs.cursed-lang.org)
- **Discord**: [Community support](https://discord.gg/cursed-lang)
- **Stack Overflow**: Tag questions with `cursed-lang`

## 🤝 Contributing

### Development Setup

```bash
# Clone repository
git clone https://github.com/ghuntley/cursed.git
cd cursed/ide-integration

# VS Code extension development
cd cursed-vscode-extension-advanced
npm install
npm run watch

# Vim plugin development
cd cursed-vim-advanced
# Edit plugin files directly
```

### Contributing Guidelines

1. **Follow Code Style**: Use existing formatting conventions
2. **Add Tests**: Include tests for new features
3. **Update Documentation**: Keep README and docs current
4. **Test Across Editors**: Verify changes work in all supported editors

## 📄 License

This IDE integration package is licensed under the MIT License. See LICENSE file for details.

## 🙏 Acknowledgments

- **VS Code Extension API**: Microsoft Visual Studio Code team
- **IntelliJ Platform**: JetBrains for the IntelliJ Platform
- **Language Server Protocol**: Microsoft and contributors
- **Tree-sitter**: GitHub and the Tree-sitter community
- **Vim Community**: Plugin architecture and LSP integration

---

**Ready to build amazing CURSED applications with professional IDE support!** 🚀

For more information, visit [cursed-lang.org](https://cursed-lang.org) or check out the [documentation](https://docs.cursed-lang.org).
