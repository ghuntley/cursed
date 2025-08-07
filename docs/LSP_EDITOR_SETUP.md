# CURSED Language Server Setup Guide

This guide helps you set up the CURSED Language Server Protocol (LSP) integration with popular editors.

## Prerequisites

1. Build the CURSED Language Server:
   ```bash
   zig build-exe cursed_lsp_working.zig -lc --name cursed-lsp
   # Or use the build system:
   zig build lsp
   ```

2. Ensure the `cursed-lsp` binary is in your PATH or note its full path.

## Visual Studio Code

### Method 1: Install Pre-built Extension

1. Navigate to the extension directory:
   ```bash
   cd cursed-vscode-extension
   npm install
   npm run compile
   ```

2. Install the extension:
   ```bash
   code --install-extension .
   ```

### Method 2: Manual Configuration

1. Install the "LSP" extension from the marketplace
2. Add to your `settings.json`:
   ```json
   {
     "lsp.serverPath": "/path/to/cursed-lsp",
     "files.associations": {
       "*.csd": "cursed",
       "*.cursed": "cursed"
     }
   }
   ```

## Neovim (with nvim-lspconfig)

Add to your Neovim configuration:

```lua
-- ~/.config/nvim/init.lua or ~/.config/nvim/lua/lsp-setup.lua

require'lspconfig'.cursed_lsp.setup{
  cmd = { '/path/to/cursed-lsp' },
  filetypes = { 'cursed' },
  root_dir = require'lspconfig'.util.root_pattern('.git', 'CursedPackage.toml'),
  settings = {}
}

-- File type detection
vim.cmd([[
  augroup cursed_filetype
    autocmd!
    autocmd BufRead,BufNewFile *.csd set filetype=cursed
    autocmd BufRead,BufNewFile *.cursed set filetype=cursed
  augroup END
]])

-- Syntax highlighting for CURSED
vim.cmd([[
  augroup cursed_syntax
    autocmd!
    autocmd FileType cursed setlocal commentstring=//%s
    autocmd FileType cursed syntax keyword cursedKeyword slay sus damn facts lowkey bestie stan ready later dm select match yeet vibes mood basic ghosted simp squad collab periodt flex spill
    autocmd FileType cursed syntax keyword cursedType normie tea lit drip thicc smol meal vibe
    autocmd FileType cursed syntax keyword cursedBoolean based cringe
    autocmd FileType cursed syntax keyword cursedNull void nil
    autocmd FileType cursed syntax match cursedFunction '\w\+\s*('
    autocmd FileType cursed syntax region cursedString start='"' end='"'
    autocmd FileType cursed syntax match cursedNumber '\d\+\(\.\d\+\)\?'
    autocmd FileType cursed syntax match cursedComment '//.*$'
  augroup END
]])
```

## Vim (with vim-lsp)

Add to your `.vimrc`:

```vim
" Install vim-lsp plugin first
" Plug 'prabirshrestha/vim-lsp'

" Register CURSED language server
if executable('cursed-lsp')
    augroup LspCursed
        autocmd!
        autocmd User lsp_setup call lsp#register_server({
            \ 'name': 'cursed-lsp',
            \ 'cmd': {server_info->['cursed-lsp']},
            \ 'allowlist': ['cursed'],
            \ })
    augroup END
endif

" File type detection
augroup cursed_filetype
    autocmd!
    autocmd BufRead,BufNewFile *.csd set filetype=cursed
    autocmd BufRead,BufNewFile *.cursed set filetype=cursed
augroup END

" Basic syntax highlighting
autocmd FileType cursed syntax keyword cursedKeyword slay sus damn facts lowkey bestie stan ready later dm select match yeet vibes mood basic ghosted simp squad collab periodt flex spill
autocmd FileType cursed syntax keyword cursedType normie tea lit drip thicc smol meal vibe
autocmd FileType cursed syntax keyword cursedBoolean based cringe
autocmd FileType cursed syntax keyword cursedNull void nil
autocmd FileType cursed syntax region cursedString start='"' end='"'
autocmd FileType cursed syntax match cursedNumber '\d\+\(\.\d\+\)\?'
autocmd FileType cursed syntax match cursedComment '//.*$'

" Set comment string
autocmd FileType cursed setlocal commentstring=//%s
```

## Emacs (with lsp-mode)

Add to your Emacs configuration:

```elisp
;; ~/.emacs.d/init.el

(require 'lsp-mode)

;; Define CURSED major mode
(define-derived-mode cursed-mode prog-mode "CURSED"
  "Major mode for CURSED programming language."
  (setq-local comment-start "// ")
  (setq-local comment-end "")
  
  ;; Font lock (syntax highlighting)
  (font-lock-add-keywords nil
    '(("\\<\\(slay\\|sus\\|damn\\|facts\\|lowkey\\|bestie\\|stan\\|ready\\|later\\|dm\\|select\\|match\\|yeet\\|vibes\\|mood\\|basic\\|ghosted\\|simp\\|squad\\|collab\\|periodt\\|flex\\|spill\\)\\>" . 'font-lock-keyword-face)
      ("\\<\\(normie\\|tea\\|lit\\|drip\\|thicc\\|smol\\|meal\\|vibe\\)\\>" . 'font-lock-type-face)
      ("\\<\\(based\\|cringe\\)\\>" . 'font-lock-constant-face)
      ("\\<\\(void\\|nil\\)\\>" . 'font-lock-constant-face)
      ("\"[^\"]*\"" . 'font-lock-string-face)
      ("\\<[0-9]+\\(\\.[0-9]+\\)?\\>" . 'font-lock-constant-face)
      ("//.*$" . 'font-lock-comment-face))))

;; Auto-mode for CURSED files
(add-to-list 'auto-mode-alist '("\\.csd\\'" . cursed-mode))
(add-to-list 'auto-mode-alist '("\\.cursed\\'" . cursed-mode))

;; LSP configuration for CURSED
(add-to-list 'lsp-language-id-configuration '(cursed-mode . "cursed"))

(lsp-register-client
 (make-lsp-client :new-connection (lsp-stdio-connection "/path/to/cursed-lsp")
                  :major-modes '(cursed-mode)
                  :server-id 'cursed-lsp))

;; Enable LSP for CURSED files
(add-hook 'cursed-mode-hook #'lsp)
```

## Sublime Text

Create a package for CURSED:

1. Create `~/.config/sublime-text-3/Packages/CURSED/` directory

2. Create `CURSED.sublime-syntax`:
```yaml
%YAML 1.2
---
name: CURSED
file_extensions: [csd, cursed]
scope: source.cursed

contexts:
  main:
    - match: '//'
      scope: punctuation.definition.comment.cursed
      push: line_comment
    
    - match: '"'
      scope: punctuation.definition.string.begin.cursed
      push: string
    
    - match: '\b(slay|sus|damn|facts|lowkey|bestie|stan|ready|later|dm|select|match|yeet|vibes|mood|basic|ghosted|simp|squad|collab|periodt|flex|spill)\b'
      scope: keyword.control.cursed
    
    - match: '\b(normie|tea|lit|drip|thicc|smol|meal|vibe)\b'
      scope: storage.type.cursed
    
    - match: '\b(based|cringe|void|nil)\b'
      scope: constant.language.cursed
    
    - match: '\b\d+(\.\d+)?\b'
      scope: constant.numeric.cursed

  line_comment:
    - meta_scope: comment.line.cursed
    - match: $
      pop: true

  string:
    - meta_scope: string.quoted.double.cursed
    - match: '\\.'
      scope: constant.character.escape.cursed
    - match: '"'
      scope: punctuation.definition.string.end.cursed
      pop: true
```

3. Create `LSP-cursed.sublime-settings`:
```json
{
  "command": ["/path/to/cursed-lsp"],
  "selector": "source.cursed"
}
```

## Testing Your Setup

1. Create a test file `test.csd`:
   ```cursed
   slay main() {
       sus greeting tea = "Hello, CURSED!"
       vibez.spill(greeting)
   }
   ```

2. Open the file in your editor
3. Check for:
   - Syntax highlighting
   - Code completion (try typing `sl` and see if `slay` appears)
   - Hover information (hover over keywords)
   - Error diagnostics (try syntax errors)

## Troubleshooting

### LSP Server Not Starting
- Check if `cursed-lsp` is in your PATH
- Verify the binary has execute permissions
- Check editor LSP logs for error messages

### No Syntax Highlighting
- Ensure file extensions `.csd` and `.cursed` are properly associated
- Check that syntax definitions are loaded
- Restart your editor after configuration changes

### Code Completion Not Working
- Verify LSP server is running (check editor status or logs)
- Ensure the LSP client is properly configured
- Try manual completion triggers (usually Ctrl+Space)

### Performance Issues
- The LSP server is lightweight and should be fast
- Check for file permission issues
- Ensure you're using the optimized build: `zig build -Doptimize=ReleaseFast`

## Features Supported

✅ **Syntax Highlighting** - Full CURSED keyword and syntax support
✅ **Code Completion** - Context-aware completions for keywords and functions  
✅ **Hover Information** - Documentation on hover for CURSED symbols
✅ **Error Diagnostics** - Real-time syntax and semantic error checking
✅ **Document Formatting** - Automatic code formatting
✅ **Go to Definition** - Navigate to symbol definitions (basic support)
✅ **Find References** - Find all references to symbols (basic support)

## Advanced Configuration

### Custom LSP Settings

You can customize the LSP server behavior by passing settings during initialization. Currently supported:

- Text document synchronization
- Completion trigger characters: `.` and `:`
- Hover information format: Markdown
- Diagnostic severity levels

### Performance Tuning

For large projects:
- Use the optimized build: `zig build -Doptimize=ReleaseFast`
- Configure your editor's LSP client to limit concurrent requests
- Consider file watching patterns for better performance
