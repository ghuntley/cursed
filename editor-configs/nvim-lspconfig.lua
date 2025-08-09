-- CURSED Language Server Configuration for Neovim
-- Place this in your ~/.config/nvim/lua/cursed-lsp.lua
-- Then require it in your init.lua with: require('cursed-lsp')

local lspconfig = require('lspconfig')
local util = require('lspconfig.util')

-- Define CURSED language server configuration
local cursed_lsp = {
  default_config = {
    cmd = { 'cursed-lsp' },
    filetypes = { 'cursed' },
    root_dir = util.root_pattern('CursedProject.toml', 'CursedWorkspace.toml', '.git'),
    single_file_support = true,
    settings = {},
  },
  docs = {
    description = [[
https://github.com/ghuntley/cursed

Language server for the CURSED programming language.

Requires the cursed-lsp binary to be available in your PATH or specify the full path:

```lua
require('lspconfig').cursed.setup{
  cmd = { '/path/to/cursed-lsp' }
}
```
    ]],
    default_config = {
      root_dir = [[root_pattern('CursedProject.toml', 'CursedWorkspace.toml', '.git')]],
    },
  },
}

-- Register the CURSED language server
lspconfig.configs.cursed = cursed_lsp

-- Setup CURSED LSP with default configuration
lspconfig.cursed.setup({
  -- Optional: Custom settings
  settings = {
    cursed = {
      diagnostics = {
        enable = true,
      },
      completion = {
        enable = true,
      },
      hover = {
        enable = true,
      },
    },
  },
  
  -- Optional: Custom capabilities
  capabilities = require('cmp_nvim_lsp').default_capabilities(),
  
  -- Optional: Custom on_attach function
  on_attach = function(client, bufnr)
    -- Enable completion triggered by <c-x><c-o>
    vim.bo[bufnr].omnifunc = 'v:lua.vim.lsp.omnifunc'

    -- Buffer local mappings
    local bufopts = { noremap=true, silent=true, buffer=bufnr }
    vim.keymap.set('n', 'gD', vim.lsp.buf.declaration, bufopts)
    vim.keymap.set('n', 'gd', vim.lsp.buf.definition, bufopts)
    vim.keymap.set('n', 'K', vim.lsp.buf.hover, bufopts)
    vim.keymap.set('n', 'gi', vim.lsp.buf.implementation, bufopts)
    vim.keymap.set('n', '<C-k>', vim.lsp.buf.signature_help, bufopts)
    vim.keymap.set('n', '<space>wa', vim.lsp.buf.add_workspace_folder, bufopts)
    vim.keymap.set('n', '<space>wr', vim.lsp.buf.remove_workspace_folder, bufopts)
    vim.keymap.set('n', '<space>wl', function()
      print(vim.inspect(vim.lsp.buf.list_workspace_folders()))
    end, bufopts)
    vim.keymap.set('n', '<space>D', vim.lsp.buf.type_definition, bufopts)
    vim.keymap.set('n', '<space>rn', vim.lsp.buf.rename, bufopts)
    vim.keymap.set('n', '<space>ca', vim.lsp.buf.code_action, bufopts)
    vim.keymap.set('n', 'gr', vim.lsp.buf.references, bufopts)
    vim.keymap.set('n', '<space>f', function() vim.lsp.buf.format { async = true } end, bufopts)
  end,
})

-- Set up file type detection for .csd files
vim.cmd([[
  augroup CursedFiletype
    autocmd!
    autocmd BufRead,BufNewFile *.csd set filetype=cursed
  augroup END
]])

-- Optional: Set up syntax highlighting
vim.cmd([[
  augroup CursedSyntax
    autocmd!
    autocmd FileType cursed setlocal commentstring=#%s
    autocmd FileType cursed setlocal shiftwidth=4 tabstop=4 expandtab
  augroup END
]])

print("CURSED LSP configuration loaded")
