# CURSED LSP Server Usage Guide

## Overview

The CURSED Language Server Protocol (LSP) implementation provides IDE integration for the CURSED programming language. This enables features like syntax highlighting, code completion, hover information, and diagnostics in your favorite editor.

## Quick Start

### 1. Build the LSP Server

```bash
zig build
```

This creates the `cursed-lsp` executable in `zig-out/bin/`.

### 2. Test the LSP Server

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | ./zig-out/bin/cursed-lsp
```

Expected output:
```
🚀 CURSED LSP Server v1.0.0 starting...
Content-Length: 200

{"jsonrpc":"2.0","id":1,"result":{"capabilities":{"textDocumentSync":1,"completionProvider":{"resolveProvider":false},"hoverProvider":true,"definitionProvider":true}}}
✅ Sent initialize response
👋 CURSED LSP Server stopped
```

## IDE Integration

### VS Code Setup

1. **Create LSP Configuration**

Create `.vscode/settings.json` in your project:

```json
{
  "cursed-lsp": {
    "command": "/path/to/cursed/zig-out/bin/cursed-lsp",
    "args": [],
    "filetypes": ["cursed"],
    "rootPatterns": ["*.csd", ".cursed-project"]
  }
}
```

2. **File Association**

Add to `.vscode/settings.json`:

```json
{
  "files.associations": {
    "*.csd": "cursed"
  }
}
```

### Vim/Neovim Setup

Using vim-lsp plugin:

```vim
" Add to your .vimrc or init.vim
if executable('cursed-lsp')
    au User lsp_setup call lsp#register_server({
        \ 'name': 'cursed-lsp',
        \ 'cmd': {server_info->['cursed-lsp']},
        \ 'allowlist': ['cursed'],
        \ })
endif

" File type detection
au BufRead,BufNewFile *.csd setfiletype cursed
```

### Emacs Setup

Using lsp-mode:

```elisp
;; Add to your .emacs or init.el
(with-eval-after-load 'lsp-mode
  (add-to-list 'lsp-language-id-configuration '(cursed-mode . "cursed"))
  (lsp-register-client
   (make-lsp-client :new-connection (lsp-stdio-connection "cursed-lsp")
                    :major-modes '(cursed-mode)
                    :server-id 'cursed-lsp)))

;; Basic syntax highlighting
(define-derived-mode cursed-mode prog-mode "CURSED"
  "Major mode for CURSED programming language."
  (setq-local comment-start "//")
  (setq-local comment-end ""))

(add-to-list 'auto-mode-alist '("\\.csd\\'" . cursed-mode))
```

## LSP Features

### Currently Supported

- ✅ **Initialization** - Server capabilities negotiation
- ✅ **Text Document Sync** - File change monitoring  
- ✅ **Code Completion** - Basic CURSED keywords (`sus`, `slay`, `damn`)
- ✅ **Hover Information** - Documentation for language constructs
- ✅ **Definition Lookup** - Go-to-definition support
- ✅ **Diagnostics** - Basic error reporting

### Feature Examples

#### Code Completion
When you type in a CURSED file, the LSP server provides completions for:
- `sus` - Variable declaration
- `slay` - Function declaration  
- `damn` - Return statement
- `bestie` - Loop construct
- `ready` - Conditional statement

#### Hover Information
Hovering over CURSED keywords shows documentation:
- Explains the purpose of each keyword
- Shows syntax examples
- Provides usage guidelines

## Development Tools Ecosystem

### Main Interpreter
```bash
./zig-out/bin/cursed-zig program.csd
```

### Package Manager
```bash
./zig-out/bin/cursed-pkg init my-project
./zig-out/bin/cursed-pkg build
./zig-out/bin/cursed-pkg test
```

### Development Tools (In Progress)
- **cursed-fmt** - Code formatter (source available)
- **cursed-lint** - Code linter (source available)  
- **cursed-debug** - Interactive debugger (source available)

## Troubleshooting

### Common Issues

1. **LSP Server Not Starting**
   - Verify the executable exists: `ls zig-out/bin/cursed-lsp`
   - Check permissions: `chmod +x zig-out/bin/cursed-lsp`
   - Test manually: `echo "test" | ./zig-out/bin/cursed-lsp`

2. **IDE Not Recognizing LSP**
   - Ensure file association is correct (`.csd` files)
   - Check LSP configuration paths are absolute
   - Restart IDE after configuration changes

3. **No Completions**
   - Verify file is saved with `.csd` extension
   - Check that LSP server is running in IDE logs
   - Try manual completion trigger (usually Ctrl+Space)

### Debug Mode

Enable LSP server logging:

```bash
# Set log level for debugging
CURSED_LSP_LOG=debug ./zig-out/bin/cursed-lsp
```

### Testing LSP Requests

Manual testing with JSON-RPC requests:

```bash
# Initialize
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"capabilities":{}}}' | ./zig-out/bin/cursed-lsp

# Completion
echo '{"jsonrpc":"2.0","id":2,"method":"textDocument/completion","params":{"textDocument":{"uri":"file://test.csd"},"position":{"line":0,"character":0}}}' | ./zig-out/bin/cursed-lsp

# Hover  
echo '{"jsonrpc":"2.0","id":3,"method":"textDocument/hover","params":{"textDocument":{"uri":"file://test.csd"},"position":{"line":0,"character":0}}}' | ./zig-out/bin/cursed-lsp
```

## Next Steps

1. **Enhanced Features**
   - Semantic highlighting
   - Symbol navigation
   - Code actions (refactoring)
   - Workspace symbols

2. **Editor Plugins**
   - Official VS Code extension
   - Vim plugin package
   - Emacs package

3. **Advanced Diagnostics**
   - Type checking integration
   - Real-time error detection
   - Warning suggestions

## Contributing

To improve the LSP server:

1. **Core LSP Implementation**: `src-zig/lsp_simple_working.zig`
2. **Build Configuration**: `build.zig` 
3. **Test Files**: Create `.csd` examples for testing

For questions or issues, please check the project documentation or create an issue in the repository.
