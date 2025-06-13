# CURSED VS Code Extension - Installation Guide

## Quick Installation

### Option 1: From VS Code Marketplace (Future)
1. Open VS Code
2. Press `Ctrl+Shift+X` to open Extensions
3. Search for "CURSED Language Support"
4. Click "Install"

### Option 2: Install from VSIX Package
1. Download the latest `.vsix` file from the releases
2. Open VS Code
3. Press `Ctrl+Shift+P` to open Command Palette
4. Type "Extensions: Install from VSIX..."
5. Select the downloaded `.vsix` file

### Option 3: Development Installation
```bash
# Clone the repository
git clone https://github.com/ghuntley/cursed.git
cd cursed/editors/vscode

# Install dependencies
npm install

# Compile the extension
npm run compile

# Package the extension (optional)
npm run package
```

## Prerequisites

### Required Software
1. **VS Code 1.74.0 or later**
2. **CURSED Compiler** - Install from the main CURSED repository
3. **CURSED LSP Server** - Should be available as `cursed-lsp` in your PATH

### Installing CURSED Language Tools

```bash
# Install CURSED (example - adjust based on actual installation method)
cargo install cursed

# Verify installation
cursed --version
cursed-lsp --version
```

## Configuration

### Automatic Configuration
The extension works out of the box with default settings. The language server will automatically start when you open a `.csd` file or a folder containing CURSED files.

### Manual Configuration
Add these settings to your VS Code settings.json:

```json
{
    "cursed.languageServer.enabled": true,
    "cursed.languageServer.command": "cursed-lsp",
    "cursed.languageServer.args": [],
    "cursed.languageServer.debug": false,
    
    "cursed.format.enable": true,
    "cursed.format.indentSize": 4,
    "cursed.format.useTabs": false,
    "cursed.format.lineWidth": 120,
    "cursed.format.braceStyle": "same-line",
    
    "cursed.lint.enable": true,
    "cursed.lint.checkStyle": true,
    "cursed.lint.checkPerformance": true,
    "cursed.lint.checkSecurity": true
}
```

## Verification

### Test the Installation
1. Create a new file with `.csd` extension
2. Write some CURSED code:
   ```cursed
   slay main() {
       println("Hello, CURSED! 🔥")?
   }
   ```
3. Verify you see:
   - Syntax highlighting
   - Language server status in status bar
   - No errors in the Problems panel

### Check Extension Status
1. Press `Ctrl+Shift+P`
2. Type "CURSED: Show Project Structure"
3. If the command appears, the extension is properly installed

## Troubleshooting

### Language Server Not Starting
**Problem**: LSP errors in output panel
**Solutions**:
1. Verify `cursed-lsp` is in your PATH:
   ```bash
   which cursed-lsp  # On Unix/macOS
   where cursed-lsp  # On Windows
   ```
2. Check LSP output channel: View → Output → "CURSED Language Server"
3. Restart the language server: `Ctrl+Shift+P` → "CURSED: Restart Language Server"

### Syntax Highlighting Missing
**Problem**: `.csd` files show as plain text
**Solutions**:
1. Check file association in bottom-right corner of VS Code
2. Manually set language: `Ctrl+Shift+P` → "Change Language Mode" → "CURSED"
3. Reload window: `Ctrl+Shift+P` → "Developer: Reload Window"

### Commands Not Available
**Problem**: CURSED commands missing from Command Palette
**Solutions**:
1. Check if extension is enabled: Extensions panel → Search "CURSED"
2. Reload window: `Ctrl+Shift+P` → "Developer: Reload Window"
3. Check extension host logs: Help → Toggle Developer Tools → Console

### Build Commands Failing
**Problem**: Build/run commands not working
**Solutions**:
1. Ensure you're in a CURSED project directory
2. Check if `cursed` command is available:
   ```bash
   cursed --version
   ```
3. Verify `CursedPackage.toml` exists in your project
4. Check build output: View → Output → "CURSED Build"

## Development Setup

### For Extension Development
```bash
# Clone repository
git clone https://github.com/ghuntley/cursed.git
cd cursed/editors/vscode

# Install dependencies
npm install

# Open in VS Code
code .

# Press F5 to launch Extension Development Host
```

### Building from Source
```bash
# Compile TypeScript
npm run compile

# Watch for changes (development)
npm run watch

# Run tests
npm test

# Package extension
npm run package

# Lint code
npm run lint
```

## Uninstallation

### Remove Extension
1. Open VS Code
2. Go to Extensions (`Ctrl+Shift+X`)
3. Find "CURSED Language Support"
4. Click the gear icon → "Uninstall"

### Clean Configuration (Optional)
Remove CURSED-related settings from your VS Code settings.json file.

## Support

If you encounter issues:
1. Check the [GitHub Issues](https://github.com/ghuntley/cursed/issues)
2. Review the [README](./README.md) for detailed features
3. Check the [CHANGELOG](./CHANGELOG.md) for recent changes
4. Join the CURSED community discussions

---

**Ready to start coding in CURSED?** 🔥 Your VS Code setup is now complete!
