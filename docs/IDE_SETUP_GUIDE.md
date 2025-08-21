# CURSED IDE Setup Guide

**Complete guide for setting up CURSED development environment with professional IDE integration**

## 🚀 VS Code Extension Installation

### Method 1: Install from VSIX Package (Recommended)
```bash
# Download and install the CURSED VS Code extension
code --install-extension cursed-language-support-1.0.0.vsix
```

### Method 2: Manual Installation
1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X)
3. Click "..." menu → "Install from VSIX..."
4. Select `cursed-language-support-1.0.0.vsix`
5. Restart VS Code

### Method 3: Development Installation
```bash
# Clone and build extension from source
git clone https://github.com/ghuntley/cursed.git
cd cursed/vscode-cursed-extension
npm install
npm run compile
code --install-extension .
```

## 🛠️ LSP Server Setup

### Automatic Detection (Default)
The VS Code extension automatically detects the CURSED LSP server in:
1. `./zig-out/bin/cursed-lsp` (workspace-relative)
2. `cursed-lsp` (system PATH)

### Manual Configuration
Add to VS Code `settings.json`:
```json
{
  "cursed.lsp.path": "/path/to/cursed-lsp",
  "cursed.lsp.enabled": true,
  "cursed.lsp.trace": "off"
}
```

### Building LSP Server
```bash
# Build CURSED compiler and LSP server
cd cursed
zig build
./zig-out/bin/cursed-lsp --version
```

## 📁 Project Setup

### Create New CURSED Project
```bash
mkdir my-cursed-project
cd my-cursed-project

# Create main.csd
cat > main.csd << 'EOF'
yeet "vibez"

slay main() {
    vibez.spill("Hello, CURSED! 🔥")
}
EOF

# Open in VS Code
code .
```

### File Association
CURSED files use `.csd` extension and are automatically recognized by VS Code with:
- Syntax highlighting
- Language server integration
- IntelliSense support
- Error detection

## 🎨 IDE Features

### Code Completion
- **Trigger**: Type and press `Ctrl+Space`
- **Keywords**: `sus`, `drip`, `tea`, `lit`, `slay`, `damn`, etc.
- **Modules**: `vibez`, `mathz`, `stringz`, `arrayz`, `concurrenz`
- **Functions**: Context-aware function signatures
- **Variables**: Local and global variable completion

### Navigation
- **Goto Definition**: `F12` or `Ctrl+Click`
- **Find References**: `Shift+F12`
- **Symbol Search**: `Ctrl+T` for workspace symbols
- **Quick Open**: `Ctrl+P` for file search

### Code Actions
- **Format Document**: `Shift+Alt+F`
- **Format Selection**: `Ctrl+K Ctrl+F` 
- **Comment Toggle**: `Ctrl+/`
- **Bracket Matching**: Automatic bracket completion

### Error Detection
- **Real-time Diagnostics**: Errors shown as you type
- **Error Navigation**: `F8` for next error
- **Problem Panel**: View all errors in workspace
- **Hover Information**: Detailed error messages

## ⚙️ Configuration Options

### Extension Settings
```json
{
  // LSP server executable path
  "cursed.lsp.path": "cursed-lsp",
  
  // Enable/disable language server
  "cursed.lsp.enabled": true,
  
  // LSP communication tracing (off, messages, verbose)
  "cursed.lsp.trace": "off"
}
```

### Workspace Settings
Create `.vscode/settings.json` in your project:
```json
{
  "files.associations": {
    "*.csd": "cursed"
  },
  "editor.tabSize": 4,
  "editor.insertSpaces": true,
  "editor.formatOnSave": true,
  "cursed.lsp.trace": "messages"
}
```

### Language Configuration
The extension provides:
- **File Extensions**: `.csd`
- **Comment Style**: `#` for line comments, `/* */` for blocks
- **Bracket Pairs**: `()`, `[]`, `{}`, `<>`
- **Auto Indentation**: Smart indentation for CURSED syntax

## 🚀 Development Workflow

### 1. Create New File
```bash
# Create hello.csd
touch hello.csd
code hello.csd
```

### 2. Write CURSED Code
```cursed
yeet "vibez"
yeet "mathz"

slay factorial(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial(n - 1)
}

slay main() {
    sus result drip = factorial(5)
    vibez.spill("Factorial of 5:", result)
}
```

### 3. Use IDE Features
- Type `slay` → get function completion
- Hover over `factorial` → see function signature
- Press `F12` on function call → goto definition
- `Shift+Alt+F` → format code

### 4. Run and Debug
```bash
# Interpret CURSED code
./zig-out/bin/cursed-zig hello.csd

# Compile to native binary
./zig-out/bin/cursed-zig --compile hello.csd
./hello
```

## 🎯 Advanced Features

### Multi-file Projects
```
project/
├── main.csd           # Main entry point
├── utils.csd          # Utility functions
├── types.csd          # Type definitions  
└── .vscode/
    └── settings.json  # Project settings
```

Cross-file navigation works automatically:
- Goto definition across files
- Find references in entire workspace
- Symbol search includes all files

### Custom Modules
```cursed
# In utils.csd
slay helper_function(input tea) tea {
    damn stringz.upper(input)
}

# In main.csd  
yeet "utils"  # Import local module

slay main() {
    sus result tea = helper_function("hello")
    vibez.spill(result)
}
```

### Error Handling Integration
```cursed
slay risky_operation() yikes<tea> {
    ready (random_condition()) {
        yikes "something went wrong"
    }
    damn "success"
}

slay main() {
    sus result tea = risky_operation() fam {
        when "something went wrong" -> {
            vibez.spill("Handled error!")
            damn "default"
        }
    }
}
```

## 🔧 Troubleshooting

### LSP Server Not Starting
```bash
# Check if LSP server exists
ls -la ./zig-out/bin/cursed-lsp

# Test LSP server manually
./zig-out/bin/cursed-lsp --version

# Check VS Code output
# View → Output → CURSED Language Server
```

### Performance Issues
```json
{
  // Reduce trace level
  "cursed.lsp.trace": "off",
  
  // Exclude large directories
  "files.watcherExclude": {
    "**/node_modules/**": true,
    "**/zig-cache/**": true,
    "**/target/**": true
  }
}
```

### Syntax Highlighting Not Working
1. Check file extension is `.csd`
2. Reload VS Code window (`Ctrl+Shift+P` → "Reload Window")
3. Verify extension is enabled in Extensions panel
4. Check language mode in bottom-right corner of VS Code

### Extension Conflicts
```bash
# Disable conflicting extensions
code --disable-extension ms-vscode.vscode-json
code --enable-extension cursed-lang.cursed-language-support
```

## 🏗️ Building from Source

### Prerequisites
- Node.js 16+
- TypeScript 4.9+
- VS Code 1.74+

### Build Steps
```bash
# Clone repository
git clone https://github.com/ghuntley/cursed.git
cd cursed/vscode-cursed-extension

# Install dependencies
npm install

# Compile TypeScript
npm run compile

# Package extension
npx vsce package

# Install locally
code --install-extension cursed-language-support-1.0.0.vsix
```

### Development Mode
```bash
# Watch for changes
npm run watch

# Debug in Extension Development Host
# Press F5 in VS Code with extension folder open
```

## 📚 Additional Resources

### Documentation
- [CURSED Language Reference](../README.md)
- [Standard Library Documentation](../stdlib/)
- [Example Projects](../examples/)
- [Contributing Guide](../CONTRIBUTING.md)

### Community
- [GitHub Issues](https://github.com/ghuntley/cursed/issues)
- [Discord Server](https://discord.gg/cursed-lang)
- [Stack Overflow](https://stackoverflow.com/questions/tagged/cursed-lang)

### Learning Resources
- [CURSED Tutorial](tutorial.md)
- [Best Practices](best-practices.md)
- [Migration Guide](migration-guide.md)
- [Performance Tips](performance.md)

---

**Ready to start developing with CURSED!** 🚀

The VS Code extension provides a professional development experience with:
- Real-time error detection
- Intelligent code completion
- Fast navigation and search
- Automatic formatting
- Rich hover information

Happy coding! 💻✨
