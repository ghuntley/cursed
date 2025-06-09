# CURSED Language Support for Vim/Neovim

This directory contains Vim/Neovim configuration files for CURSED language support.

## Installation

### For Vim

1. Copy syntax file:
   ```bash
   cp cursed.vim ~/.vim/syntax/
   ```

2. Copy filetype detection:
   ```bash
   mkdir -p ~/.vim/ftdetect
   cp ftdetect/cursed.vim ~/.vim/ftdetect/
   ```

### For Neovim

1. Copy syntax file:
   ```bash
   cp cursed.vim ~/.config/nvim/syntax/
   ```

2. Copy filetype detection:
   ```bash
   mkdir -p ~/.config/nvim/ftdetect
   cp ftdetect/cursed.vim ~/.config/nvim/ftdetect/
   ```

## LSP Integration

### Using coc.nvim

Add to your `coc-settings.json`:

```json
{
  "languageserver": {
    "cursed": {
      "command": "cursed-lsp",
      "args": ["--debug"],
      "filetypes": ["cursed"],
      "rootPatterns": ["CursedPackage.toml", "CursedBuild.toml", ".git"],
      "settings": {
        "cursed": {
          "format": {
            "enable": true,
            "indentSize": 4,
            "lineWidth": 120
          },
          "lint": {
            "enable": true,
            "checkStyle": true,
            "checkPerformance": true
          }
        }
      }
    }
  }
}
```

### Using nvim-lspconfig (Neovim)

Add to your `init.lua`:

```lua
local lspconfig = require('lspconfig')

-- Configure CURSED language server
local configs = require('lspconfig.configs')

if not configs.cursed then
  configs.cursed = {
    default_config = {
      cmd = {'cursed-lsp', '--debug'},
      filetypes = {'cursed'},
      root_dir = function(fname)
        return lspconfig.util.find_git_ancestor(fname) or
               lspconfig.util.path.dirname(fname)
      end,
      settings = {
        cursed = {
          format = {
            enable = true,
            indentSize = 4,
            lineWidth = 120
          },
          lint = {
            enable = true,
            checkStyle = true,
            checkPerformance = true
          }
        }
      }
    }
  }
end

lspconfig.cursed.setup{}
```

### Using vim-lsp (Vim)

Add to your `.vimrc`:

```vim
if executable('cursed-lsp')
  augroup LspCursed
    autocmd!
    autocmd User lsp_setup call lsp#register_server({
      \ 'name': 'cursed-lsp',
      \ 'cmd': {server_info->['cursed-lsp', '--debug']},
      \ 'allowlist': ['cursed'],
      \ 'root_uri': {server_info->lsp#utils#path_to_uri(
      \   lsp#utils#find_nearest_parent_file_directory(
      \     lsp#utils#get_buffer_path(),
      \     ['CursedPackage.toml', 'CursedBuild.toml', '.git']
      \   )
      \ )},
      \ })
  augroup END
endif
```

## Manual Configuration

If you prefer manual setup, add to your `.vimrc` or `init.vim`:

```vim
" CURSED language configuration
autocmd BufRead,BufNewFile *.csd setfiletype cursed

" Optional: CURSED-specific settings
autocmd FileType cursed setlocal tabstop=4 shiftwidth=4 expandtab
autocmd FileType cursed setlocal commentstring=//\ %s

" Optional: Key mappings for CURSED
autocmd FileType cursed nnoremap <buffer> <leader>cf :!cursed-lsp format %<CR>
autocmd FileType cursed nnoremap <buffer> <leader>cl :!cursed-lsp lint %<CR>
```

## Features

- Syntax highlighting for all CURSED keywords and constructs
- Automatic filetype detection for `.csd` files
- LSP integration with:
  - Auto-completion
  - Go to definition
  - Find references
  - Hover information
  - Diagnostics (errors, warnings, hints)
  - Document formatting
  - Real-time linting

## Troubleshooting

1. **LSP not starting**: Make sure `cursed-lsp` is installed and in your PATH
2. **No syntax highlighting**: Ensure the syntax file is in the correct location
3. **Filetype not detected**: Check that the ftdetect file is properly installed

## Additional Plugins

For enhanced CURSED development experience, consider these Vim plugins:

- **coc.nvim** or **vim-lsp**: LSP client
- **fzf.vim**: Fuzzy file finding
- **nerdtree**: File explorer
- **vim-commentary**: Easy commenting/uncommenting
- **vim-surround**: Surround text with brackets, quotes, etc.
- **ale**: Asynchronous linting (alternative to LSP diagnostics)
