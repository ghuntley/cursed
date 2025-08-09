# CURSED Language Server Protocol (LSP) Setup Guide

The CURSED programming language includes a full-featured Language Server Protocol (LSP) implementation that provides rich IDE support across multiple editors.

## Features

- **Syntax Highlighting**: Full semantic syntax highlighting for CURSED code
- **Error Diagnostics**: Real-time error detection and reporting
- **Code Completion**: Intelligent autocompletion for keywords, functions, and variables
- **Go to Definition**: Navigate to symbol definitions
- **Hover Information**: Get detailed information about symbols on hover
- **Find References**: Find all references to a symbol
- **Document Formatting**: Automatic code formatting
- **Workspace Symbols**: Search symbols across the entire workspace
- **Rename Refactoring**: Intelligent symbol renaming

## Building the LSP Server

The LSP server is built automatically when you build the CURSED compiler:

```bash
# Build all CURSED tools including LSP server
zig build

# The LSP server will be available at:
./zig-out/bin/cursed-lsp
```

## Editor Setup

### VS Code

1. **Install Extension**: Install the CURSED language extension (if published) or load the development extension:

```bash
# From the cursed repository root
cd vscode-cursed-extension
npm install
npm run compile
code --install-extension .
```

2. **Configuration**: Add to your VS Code settings:

```json
{
    "cursed.lsp.enabled": true,
    "cursed.lsp.path": "cursed-lsp",
    "cursed.lsp.trace": "off"
}
```

3. **Workspace Configuration**: For project-specific LSP server path:

```json
{
    "cursed.lsp.path": "./zig-out/bin/cursed-lsp"
}
```

### Neovim with nvim-lspconfig

1. **Install Dependencies**:
   - Install `nvim-lspconfig`
   - Install `nvim-cmp` and `cmp-nvim-lsp` for completion

2. **Configuration**: Add to your Neovim configuration:

```lua
-- Load the CURSED LSP configuration
require('lspconfig').cursed.setup({
  cmd = { 'cursed-lsp' }, -- or full path: '/path/to/cursed-lsp'
  settings = {
    cursed = {
      diagnostics = { enable = true },
      completion = { enable = true },
      hover = { enable = true },
    },
  },
})

-- Set file type for .csd files
vim.cmd([[
  augroup CursedFiletype
    autocmd!
    autocmd BufRead,BufNewFile *.csd set filetype=cursed
  augroup END
]])
```

3. **Key Bindings**: The LSP will automatically set up standard LSP key bindings.

### Vim with vim-lsp

1. **Install vim-lsp plugin**

2. **Configuration**:

```vim
" Register CURSED language server
if executable('cursed-lsp')
    augroup lsp_cursed
        autocmd!
        autocmd User lsp_setup call lsp#register_server({
            \ 'name': 'cursed-lsp',
            \ 'cmd': {server_info->['cursed-lsp']},
            \ 'allowlist': ['cursed'],
            \ })
    augroup END
endif

" Set filetype for .csd files
autocmd BufRead,BufNewFile *.csd set filetype=cursed
```

### Emacs with lsp-mode

1. **Install lsp-mode**

2. **Configuration**:

```elisp
;; Add CURSED language support
(add-to-list 'auto-mode-alist '("\\.csd\\'" . cursed-mode))

;; Define cursed-mode (basic version)
(define-derived-mode cursed-mode prog-mode "CURSED"
  "Major mode for CURSED programming language."
  (setq-local comment-start "#")
  (setq-local comment-end ""))

;; Register CURSED LSP server
(with-eval-after-load 'lsp-mode
  (add-to-list 'lsp-language-id-configuration '(cursed-mode . "cursed"))
  (lsp-register-client
   (make-lsp-client :new-connection (lsp-stdio-connection "cursed-lsp")
                    :major-modes '(cursed-mode)
                    :server-id 'cursed-lsp)))
```

### Sublime Text with LSP Package

1. **Install LSP package from Package Control**

2. **Configuration**: Add to LSP settings:

```json
{
  "clients": {
    "cursed-lsp": {
      "enabled": true,
      "command": ["cursed-lsp"],
      "selector": "source.cursed",
      "settings": {}
    }
  }
}
```

3. **Syntax Definition**: Create a CURSED syntax definition file.

## Usage

### Command Line Options

```bash
# Show version information
cursed-lsp --version

# Show help and features
cursed-lsp --help

# Run LSP server (usually called by editor)
cursed-lsp
```

### Features in Action

1. **Completion**: Type partial keywords or symbols and get intelligent suggestions
2. **Diagnostics**: Syntax and semantic errors are highlighted in real-time
3. **Hover**: Hover over symbols to see type information and documentation
4. **Navigation**: Use "Go to Definition" to jump to symbol definitions
5. **References**: Find all uses of a symbol across your codebase
6. **Formatting**: Use the format document command to auto-format code

### Example CURSED Code with LSP Features

```cursed
# This comment explains the function
slay calculate_fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    } otherwise {
        damn calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2)
    }
}

# Variables with type inference
sus result drip = calculate_fibonacci(10)
vibez.spill("Fibonacci(10) =", result)
```

## Troubleshooting

### LSP Server Not Starting

1. **Check Path**: Ensure `cursed-lsp` is in your PATH or specify the full path
2. **Build Issues**: Rebuild the LSP server with `zig build`
3. **Permissions**: Ensure the executable has proper permissions

### No Completion/Features

1. **File Type**: Ensure `.csd` files are recognized as CURSED files
2. **Server Status**: Check your editor's LSP status/logs
3. **Workspace**: Ensure you're in a CURSED project directory

### Performance Issues

1. **Large Files**: The LSP server handles large files efficiently
2. **Memory Usage**: Monitor memory usage for very large workspaces
3. **Concurrent Projects**: Multiple CURSED projects may each spawn an LSP server

### Debug Information

Enable verbose logging in your editor to see LSP communication:

- **VS Code**: Set `"cursed.lsp.trace": "verbose"`
- **Neovim**: Use `:LspLog` to view logs
- **Vim**: Check vim-lsp debug output

## Advanced Configuration

### Custom Settings

The LSP server supports various configuration options:

```json
{
  "cursed": {
    "diagnostics": {
      "enable": true,
      "level": "error" // "error", "warning", "info", "hint"
    },
    "completion": {
      "enable": true,
      "snippets": true,
      "keywords": true
    },
    "hover": {
      "enable": true,
      "documentation": true
    },
    "formatting": {
      "enable": true,
      "indentSize": 4,
      "insertFinalNewline": true
    }
  }
}
```

### Workspace Configuration

For CURSED projects, create a `.cursed-lsp.json` file in your project root:

```json
{
  "diagnostics": {
    "enable": true
  },
  "completion": {
    "enable": true
  },
  "includePaths": [
    "./stdlib",
    "./src"
  ]
}
```

## Contributing

The CURSED LSP server is implemented in Zig and located in:
- `src-zig/lsp_server.zig` - Core LSP implementation
- `src-zig/lsp_main.zig` - Main entry point

To contribute:
1. Fork the repository
2. Make changes to the LSP implementation
3. Test with multiple editors
4. Submit a pull request

## Version Compatibility

- **CURSED Compiler**: v1.0.0+
- **LSP Protocol**: v3.17.0
- **VS Code**: v1.74.0+
- **Neovim**: v0.8.0+
- **Vim**: v8.2+ (with vim-lsp)
- **Emacs**: v27.1+ (with lsp-mode)
