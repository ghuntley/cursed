# CURSED Language Server Protocol (LSP) Setup Guide

This guide explains how to set up IDE support for the CURSED programming language using the built-in Language Server Protocol (LSP) implementation.

## Table of Contents
- [VS Code Setup](#vs-code-setup)
- [Neovim Setup](#neovim-setup)
- [Emacs Setup](#emacs-setup)
- [Vim Setup](#vim-setup)
- [Sublime Text Setup](#sublime-text-setup)
- [Generic LSP Client Setup](#generic-lsp-client-setup)
- [Features](#features)
- [Troubleshooting](#troubleshooting)

## Prerequisites

1. **Install CURSED Compiler**: Make sure the `cursed` command is available in your PATH
2. **Verify Installation**: Run `cursed --version` to confirm installation
3. **Test LSP Server**: Run `cursed lsp --help` to verify LSP support

## VS Code Setup

### Option 1: Install Extension (Recommended)

1. Open VS Code
2. Press `Ctrl+Shift+X` (or `Cmd+Shift+X` on macOS) to open Extensions
3. Search for "CURSED Language Support"
4. Click "Install"

### Option 2: Manual Setup

1. Copy the extension from `.vscode/extensions/cursed-language-support/` to your VS Code extensions directory:
   - **Windows**: `%USERPROFILE%\.vscode\extensions\`
   - **macOS**: `~/.vscode/extensions/`
   - **Linux**: `~/.vscode/extensions/`

2. Reload VS Code

3. Configure the extension in VS Code settings (`Ctrl+,`):
   ```json
   {
     "cursed.languageServer.enabled": true,
     "cursed.languageServer.path": "cursed",
     "cursed.languageServer.arguments": ["lsp"],
     "cursed.diagnostics.enabled": true,
     "cursed.completion.enabled": true,
     "cursed.highlighting.semantic": true,
     "cursed.formatting.enabled": true,
     "cursed.vibe.mode": "standard"
   }
   ```

### VS Code Features
- **Syntax Highlighting**: CURSED-specific keyword highlighting
- **Code Completion**: Intelligent completion for keywords, functions, and variables
- **Hover Information**: Type and documentation on hover
- **Go to Definition**: Navigate to symbol definitions
- **Error Reporting**: Real-time syntax and semantic error detection
- **Code Formatting**: Automatic code formatting with `Shift+Alt+F`
- **Commands**:
  - `Ctrl+Shift+C`: Compile CURSED file
  - `Ctrl+Shift+R`: Run CURSED file
  - `Shift+Alt+F`: Format CURSED file

## Neovim Setup

### Using nvim-lspconfig

1. Install `nvim-lspconfig` using your package manager (packer, lazy.nvim, etc.)

2. Add to your Neovim configuration:
   ```lua
   require('lspconfig.configs').cursed = {
     default_config = {
       cmd = { 'cursed', 'lsp' },
       filetypes = { 'cursed' },
       root_dir = require('lspconfig.util').root_pattern('.git', '.cursed-project'),
       settings = {},
     },
   }
   
   require('lspconfig').cursed.setup{
     on_attach = function(client, bufnr)
       -- Enable completion triggered by <c-x><c-o>
       vim.api.nvim_buf_set_option(bufnr, 'omnifunc', 'v:lua.vim.lsp.omnifunc')
       
       -- Mappings
       local bufopts = { noremap=true, silent=true, buffer=bufnr }
       vim.keymap.set('n', 'gD', vim.lsp.buf.declaration, bufopts)
       vim.keymap.set('n', 'gd', vim.lsp.buf.definition, bufopts)
       vim.keymap.set('n', 'K', vim.lsp.buf.hover, bufopts)
       vim.keymap.set('n', 'gi', vim.lsp.buf.implementation, bufopts)
       vim.keymap.set('n', '<C-k>', vim.lsp.buf.signature_help, bufopts)
       vim.keymap.set('n', '<space>rn', vim.lsp.buf.rename, bufopts)
       vim.keymap.set('n', '<space>ca', vim.lsp.buf.code_action, bufopts)
       vim.keymap.set('n', 'gr', vim.lsp.buf.references, bufopts)
       vim.keymap.set('n', '<space>f', function() vim.lsp.buf.format { async = true } end, bufopts)
     end,
   }
   ```

3. Add CURSED filetype detection to your configuration:
   ```lua
   vim.filetype.add({
     extension = {
       csd = 'cursed',
     },
   })
   ```

### Neovim Features
- **Syntax Highlighting**: Use a tree-sitter parser or vim syntax file
- **LSP Integration**: Full LSP features through nvim-lspconfig
- **Code Completion**: nvim-cmp integration for autocompletion
- **Diagnostics**: Built-in diagnostic display

## Emacs Setup

### Using lsp-mode

1. Install `lsp-mode` using your package manager

2. Add to your Emacs configuration:
   ```elisp
   (use-package lsp-mode
     :hook (cursed-mode . lsp)
     :commands lsp
     :config
     (lsp-register-client
      (make-lsp-client
       :new-connection (lsp-stdio-connection '("cursed" "lsp"))
       :major-modes '(cursed-mode)
       :server-id 'cursed-lsp)))
   
   ;; Define cursed-mode
   (define-derived-mode cursed-mode prog-mode "CURSED"
     "Major mode for CURSED programming language."
     (setq-local comment-start "// ")
     (setq-local comment-end ""))
   
   ;; Associate .csd files with cursed-mode
   (add-to-list 'auto-mode-alist '("\\.csd\\'" . cursed-mode))
   ```

3. Optional: Install additional packages for enhanced experience:
   ```elisp
   (use-package company-lsp)  ; For completion
   (use-package flycheck)     ; For error checking
   (use-package lsp-ui)       ; For enhanced UI
   ```

## Vim Setup

### Using vim-lsp

1. Install `vim-lsp` using your plugin manager

2. Add to your `.vimrc`:
   ```vim
   if executable('cursed')
     au User lsp_setup call lsp#register_server({
       \ 'name': 'cursed-lsp',
       \ 'cmd': {server_info->['cursed', 'lsp']},
       \ 'allowlist': ['cursed'],
       \ })
   endif
   
   " CURSED filetype detection
   au BufNewFile,BufRead *.csd set filetype=cursed
   
   " LSP mappings
   function! s:on_lsp_buffer_enabled() abort
     setlocal omnifunc=lsp#complete
     setlocal signcolumn=yes
     if exists('+tagfunc') | setlocal tagfunc=lsp#tagfunc | endif
     nmap <buffer> gd <plug>(lsp-definition)
     nmap <buffer> gs <plug>(lsp-document-symbol-search)
     nmap <buffer> gS <plug>(lsp-workspace-symbol-search)
     nmap <buffer> gr <plug>(lsp-references)
     nmap <buffer> gi <plug>(lsp-implementation)
     nmap <buffer> gt <plug>(lsp-type-definition)
     nmap <buffer> <leader>rn <plug>(lsp-rename)
     nmap <buffer> [g <plug>(lsp-previous-diagnostic)
     nmap <buffer> ]g <plug>(lsp-next-diagnostic)
     nmap <buffer> K <plug>(lsp-hover)
   endfunction
   
   augroup lsp_install
     au!
     autocmd User lsp_buffer_enabled call s:on_lsp_buffer_enabled()
   augroup END
   ```

## Sublime Text Setup

### Using LSP Package

1. Install Package Control if not already installed
2. Install the "LSP" package via Package Control
3. Create or edit `Packages/User/LSP.sublime-settings`:
   ```json
   {
     "clients": {
       "cursed": {
         "enabled": true,
         "command": ["cursed", "lsp"],
         "selector": "source.cursed"
       }
     }
   }
   ```

4. Create syntax highlighting by adding `Packages/User/CURSED.sublime-syntax`:
   ```yaml
   %YAML 1.2
   ---
   name: CURSED
   file_extensions:
     - csd
   scope: source.cursed
   
   contexts:
     main:
       - match: '\b(sus|slay|damn|lowkey|otherwise|bestie|ghosted|simp|yikes|shook|fam|yolo|ready|defer|yeet|vibes|vibe)\b'
         scope: keyword.control.cursed
       
       - match: '\b(lit|tea|normie|drip|thicc|smol|mid|meal|byte|rune|sip|extra)\b'
         scope: storage.type.cursed
       
       - match: '\b(based|cap|cringe)\b'
         scope: constant.language.cursed
       
       - match: '"'
         push: string
       
       - match: '//'
         push: comment
   
     string:
       - meta_scope: string.quoted.double.cursed
       - match: '\\.'
         scope: constant.character.escape.cursed
       - match: '"'
         pop: true
   
     comment:
       - meta_scope: comment.line.cursed
       - match: '$'
         pop: true
   ```

## Generic LSP Client Setup

For any LSP-compatible editor, use these settings:

- **Server Command**: `cursed lsp`
- **File Extensions**: `.csd`
- **Language ID**: `cursed`
- **Root Directory Patterns**: `.git`, `.cursed-project`, `Cargo.toml`

### Example Configuration:
```json
{
  "languageServer": {
    "cursed": {
      "command": "cursed",
      "args": ["lsp"],
      "filetypes": ["cursed"],
      "rootPatterns": [".git", ".cursed-project"]
    }
  }
}
```

## Features

The CURSED LSP server provides the following features:

### Code Completion
- **Keywords**: All CURSED keywords (`sus`, `slay`, `damn`, etc.)
- **Types**: All CURSED types (`lit`, `tea`, `normie`, etc.)
- **Literals**: Boolean and nil literals (`based`, `cap`, `cringe`)
- **Builtins**: Standard library functions (`vibez.spill`, `math.add`, etc.)
- **Snippets**: Common code patterns (function declaration, if statement, etc.)
- **Context-aware**: Member access completions (e.g., `vibez.` shows available methods)

### Hover Information
- **Symbol Information**: Type and documentation for variables, functions, and types
- **Error Context**: Detailed information about compiler errors
- **CURSED-specific**: Special handling for CURSED syntax and semantics

### Go to Definition
- **Function Definitions**: Navigate to function declarations
- **Variable Definitions**: Jump to variable declarations
- **Type Definitions**: Navigate to type definitions
- **Cross-file Navigation**: Works across multiple files in a project

### Diagnostics (Error Reporting)
- **Syntax Errors**: Real-time syntax error detection
- **Type Errors**: Semantic analysis and type checking
- **CURSED Linting**: CURSED-specific style and best practice warnings
- **Performance**: Incremental analysis for fast feedback

### Code Formatting
- **Auto-formatting**: Consistent code style formatting
- **CURSED Conventions**: Follows CURSED language style guidelines
- **Configurable**: Customizable formatting options

### Semantic Highlighting
- **Context-aware Highlighting**: Different colors for keywords, types, functions, etc.
- **CURSED-specific Tokens**: Special highlighting for CURSED slang and vibes
- **Scope-based Coloring**: Variables, functions, and types highlighted by scope

## Configuration Options

### Server Configuration
```bash
# Start LSP server with custom options
cursed lsp --log-level debug --log-file cursed-lsp.log

# Available options:
# --stdio              Use stdio communication (default)
# --tcp                Use TCP communication  
# --port PORT          TCP port (default: 9257)
# --log-level LEVEL    Logging level: error, warn, info, debug, trace
# --log-file FILE      Log to file instead of stderr
```

### Client Configuration

Most LSP clients allow configuration of:
- **Completion triggers**: Characters that trigger completion
- **Diagnostics**: Enable/disable error reporting
- **Formatting**: Auto-format on save
- **Hover**: Enable/disable hover information

## Troubleshooting

### Common Issues

1. **"cursed command not found"**
   - Ensure CURSED is installed and in your PATH
   - Try full path: `/path/to/cursed lsp`

2. **LSP server not starting**
   - Check server logs: `cursed lsp --log-level debug --log-file lsp.log`
   - Verify client configuration
   - Test server manually: `echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | cursed lsp`

3. **No completions or diagnostics**
   - Verify file is recognized as CURSED (check language mode)
   - Check LSP client is connected to server
   - Try restarting the language server

4. **Slow performance**
   - Enable incremental synchronization in client
   - Reduce diagnostic frequency
   - Check for large files causing analysis issues

### Debug Information

Enable debug logging:
```bash
# Server-side logging
cursed lsp --log-level debug --log-file cursed-lsp.log

# Check server status
ps aux | grep "cursed lsp"
```

### Getting Help

- **GitHub Issues**: Report bugs at https://github.com/cursed-lang/cursed/issues
- **Documentation**: See docs/ directory for more information
- **Community**: Join the CURSED community discussions

### Performance Tips

1. **Use incremental sync**: Configure your client for incremental document synchronization
2. **Limit workspace scope**: Open only necessary files/directories
3. **Configure diagnostics**: Adjust diagnostic update frequency
4. **File watching**: Ensure proper file watching for external changes

## Advanced Setup

### Custom Commands

Add custom editor commands for CURSED-specific operations:

```json
{
  "commands": [
    {
      "command": "cursed.compile",
      "title": "Compile CURSED File"
    },
    {
      "command": "cursed.run", 
      "title": "Run CURSED File"
    },
    {
      "command": "cursed.test",
      "title": "Run CURSED Tests"
    }
  ]
}
```

### Workspace Configuration

Create `.cursed-project` in your project root:
```json
{
  "name": "my-cursed-project",
  "version": "1.0.0",
  "dependencies": {},
  "lsp": {
    "diagnostics": {
      "enabled": true,
      "level": "info"
    },
    "completion": {
      "snippets": true,
      "keywords": true
    }
  }
}
```

This completes the comprehensive LSP setup guide for CURSED language support across multiple editors and IDEs.
