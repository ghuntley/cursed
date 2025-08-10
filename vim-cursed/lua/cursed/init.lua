-- CURSED language support for Neovim
-- Lua module for enhanced integration

local M = {}

-- LSP configuration for CURSED
function M.setup_lsp()
  local lspconfig = require('lspconfig')
  
  -- Configure CURSED LSP server
  lspconfig.cursed = {
    default_config = {
      cmd = {'cursed', '--lsp'},
      filetypes = {'cursed'},
      root_dir = function(fname)
        return lspconfig.util.find_git_ancestor(fname) or 
               lspconfig.util.path.dirname(fname)
      end,
      settings = {
        cursed = {
          completion = {
            enableSnippets = true,
            enableImports = true,
          },
          diagnostics = {
            enable = true,
            severity = 'error',
          },
          formatting = {
            enable = true,
          },
        }
      },
    },
  }
  
  -- Auto-setup if lspconfig is available
  if pcall(require, 'lspconfig') then
    require('lspconfig').cursed.setup{}
  end
end

-- Tree-sitter configuration
function M.setup_treesitter()
  local ts_config = require('nvim-treesitter.configs')
  
  ts_config.setup {
    ensure_installed = {'cursed'},
    highlight = {
      enable = true,
      additional_vim_regex_highlighting = false,
    },
    indent = {
      enable = true,
    },
    incremental_selection = {
      enable = true,
      keymaps = {
        init_selection = "gnn",
        node_incremental = "grn",
        scope_incremental = "grc",
        node_decremental = "grm",
      },
    },
    textobjects = {
      select = {
        enable = true,
        lookahead = true,
        keymaps = {
          ["af"] = "@function.outer",
          ["if"] = "@function.inner",
          ["ac"] = "@class.outer",
          ["ic"] = "@class.inner",
          ["as"] = "@statement.outer",
          ["is"] = "@statement.inner",
        },
      },
      move = {
        enable = true,
        set_jumps = true,
        goto_next_start = {
          ["]m"] = "@function.outer",
          ["]]"] = "@class.outer",
        },
        goto_next_end = {
          ["]M"] = "@function.outer",
          ["]["] = "@class.outer",
        },
        goto_previous_start = {
          ["[m"] = "@function.outer",
          ["[["] = "@class.outer",
        },
        goto_previous_end = {
          ["[M"] = "@function.outer",
          ["[]"] = "@class.outer",
        },
      },
    },
  }
end

-- Completion setup with nvim-cmp
function M.setup_completion()
  local cmp = require('cmp')
  local luasnip = require('luasnip')
  
  -- CURSED-specific completion sources
  cmp.setup.filetype('cursed', {
    sources = cmp.config.sources({
      { name = 'nvim_lsp' },
      { name = 'luasnip' },
      { name = 'buffer' },
      { name = 'path' },
      { name = 'cursed_stdlib' }, -- Custom source for stdlib
    }),
    mapping = cmp.mapping.preset.insert({
      ['<C-d>'] = cmp.mapping.scroll_docs(-4),
      ['<C-f>'] = cmp.mapping.scroll_docs(4),
      ['<C-Space>'] = cmp.mapping.complete(),
      ['<C-e>'] = cmp.mapping.close(),
      ['<CR>'] = cmp.mapping.confirm({
        behavior = cmp.ConfirmBehavior.Replace,
        select = true,
      }),
      ['<Tab>'] = cmp.mapping(function(fallback)
        if cmp.visible() then
          cmp.select_next_item()
        elseif luasnip.expand_or_jumpable() then
          luasnip.expand_or_jump()
        else
          fallback()
        end
      end, { 'i', 's' }),
      ['<S-Tab>'] = cmp.mapping(function(fallback)
        if cmp.visible() then
          cmp.select_prev_item()
        elseif luasnip.jumpable(-1) then
          luasnip.jump(-1)
        else
          fallback()
        end
      end, { 'i', 's' }),
    }),
  })
end

-- CURSED stdlib completion source
function M.setup_stdlib_completion()
  local stdlib_items = {
    -- mathz module
    {label = 'abs_normie', kind = 3, detail = 'mathz.abs_normie(x: drip) -> drip'},
    {label = 'max_normie', kind = 3, detail = 'mathz.max_normie(a: drip, b: drip) -> drip'},
    {label = 'min_normie', kind = 3, detail = 'mathz.min_normie(a: drip, b: drip) -> drip'},
    
    -- stringz module
    {label = 'slice_tea', kind = 3, detail = 'stringz.slice_tea(s: tea, start: drip, end: drip) -> tea'},
    {label = 'len_tea', kind = 3, detail = 'stringz.len_tea(s: tea) -> drip'},
    {label = 'concat_tea', kind = 3, detail = 'stringz.concat_tea(a: tea, b: tea) -> tea'},
    
    -- arrayz module
    {label = 'len', kind = 3, detail = 'arrayz.len(arr: []T) -> drip'},
    {label = 'push', kind = 3, detail = 'arrayz.push(arr: []T, item: T) -> []T'},
    {label = 'pop', kind = 3, detail = 'arrayz.pop(arr: []T) -> T'},
    
    -- vibez module
    {label = 'spill', kind = 3, detail = 'vibez.spill(value: T)'},
    {label = 'input', kind = 3, detail = 'vibez.input() -> tea'},
    
    -- testz module
    {label = 'test_start', kind = 3, detail = 'testz.test_start(name: tea)'},
    {label = 'assert_eq_int', kind = 3, detail = 'testz.assert_eq_int(actual: drip, expected: drip)'},
    {label = 'assert_eq_string', kind = 3, detail = 'testz.assert_eq_string(actual: tea, expected: tea)'},
    {label = 'assert_true', kind = 3, detail = 'testz.assert_true(condition: lit)'},
    {label = 'assert_false', kind = 3, detail = 'testz.assert_false(condition: lit)'},
    {label = 'print_test_summary', kind = 3, detail = 'testz.print_test_summary()'},
  }
  
  local source = {}
  
  function source:is_available()
    return vim.bo.filetype == 'cursed'
  end
  
  function source:get_debug_name()
    return 'cursed_stdlib'
  end
  
  function source:complete(params, callback)
    callback({
      items = stdlib_items,
      isIncomplete = false,
    })
  end
  
  if pcall(require, 'cmp') then
    require('cmp').register_source('cursed_stdlib', source)
  end
end

-- Code actions and commands
function M.setup_commands()
  vim.api.nvim_create_user_command('CursedRun', function()
    local file = vim.fn.expand('%')
    vim.cmd('term cursed ' .. file)
  end, {})
  
  vim.api.nvim_create_user_command('CursedBuild', function()
    local file = vim.fn.expand('%')
    vim.cmd('term cursed --compile ' .. file)
  end, {})
  
  vim.api.nvim_create_user_command('CursedTest', function()
    vim.cmd('term cursed test')
  end, {})
  
  vim.api.nvim_create_user_command('CursedFormat', function()
    vim.lsp.buf.format()
  end, {})
  
  vim.api.nvim_create_user_command('CursedLint', function()
    local file = vim.fn.expand('%')
    vim.cmd('term cursed --lint ' .. file)
  end, {})
  
  vim.api.nvim_create_user_command('CursedDoc', function()
    local word = vim.fn.expand('<cword>')
    vim.cmd('help cursed-' .. word)
  end, {})
end

-- Snippets for LuaSnip
function M.setup_snippets()
  local ls = require('luasnip')
  local s = ls.snippet
  local t = ls.text_node
  local i = ls.insert_node
  local f = ls.function_node
  
  ls.add_snippets('cursed', {
    s('slay', {
      t('slay '), i(1, 'function_name'), t('('), i(2, 'params'), t(') '), i(3, 'return_type'), t(' {'),
      t({'', '    '}), i(4, '// function body'),
      t({'', '    damn '}), i(5, 'return_value'),
      t({'', '}'})
    }),
    
    s('sus', {
      t('sus '), i(1, 'variable_name'), t(' '), i(2, 'type'), t(' = '), i(3, 'value')
    }),
    
    s('ready', {
      t('ready ('), i(1, 'condition'), t(') {'),
      t({'', '    '}), i(2, '// if body'),
      t({'', '}'})
    }),
    
    s('readyotherwise', {
      t('ready ('), i(1, 'condition'), t(') {'),
      t({'', '    '}), i(2, '// if body'),
      t({'', '} otherwise {'}),
      t({'', '    '}), i(3, '// else body'),
      t({'', '}'})
    }),
    
    s('bestie', {
      t('bestie ('), i(1, 'condition'), t(') {'),
      t({'', '    '}), i(2, '// loop body'),
      t({'', '}'})
    }),
    
    s('squad', {
      t('squad '), i(1, 'StructName'), t(' {'),
      t({'', '    spill '}), i(2, 'field_name'), t(' '), i(3, 'field_type'),
      t({'', '}'})
    }),
    
    s('sick', {
      t('sick ('), i(1, 'value'), t(') {'),
      t({'', '    when '}), i(2, 'pattern'), t(' -> '), i(3, 'result'),
      t({'', '    when _ -> '}), i(4, 'default'),
      t({'', '}'})
    }),
    
    s('yeet', {
      t('yeet "'), i(1, 'module_name'), t('"')
    }),
    
    s('test', {
      t('yeet "testz"'),
      t({'', ''}),
      t('slay test_'), i(1, 'test_name'), t('() {'),
      t({'', '    test_start("'}), i(2, 'test description'), t('")'),
      t({'', '    '}), i(3, '// test code'),
      t({'', '    assert_eq_int('}), i(4, 'actual'), t(', '), i(5, 'expected'), t(')'),
      t({'', '}'})
    }),
  })
end

-- Debugging integration
function M.setup_dap()
  local dap = require('dap')
  
  dap.adapters.cursed = {
    type = 'executable',
    command = 'cursed',
    args = {'--debug-adapter'},
  }
  
  dap.configurations.cursed = {
    {
      type = 'cursed',
      request = 'launch',
      name = 'Launch CURSED program',
      program = '${file}',
      cwd = '${workspaceFolder}',
      args = {},
      stopOnEntry = false,
    },
  }
end

-- Main setup function
function M.setup(opts)
  opts = opts or {}
  
  -- Setup components
  if opts.lsp ~= false then
    M.setup_lsp()
  end
  
  if opts.treesitter ~= false then
    M.setup_treesitter()
  end
  
  if opts.completion ~= false then
    M.setup_completion()
    M.setup_stdlib_completion()
  end
  
  if opts.snippets ~= false then
    M.setup_snippets()
  end
  
  if opts.dap ~= false then
    M.setup_dap()
  end
  
  M.setup_commands()
  
  -- Set up autocommands
  vim.api.nvim_create_autocmd('FileType', {
    pattern = 'cursed',
    callback = function()
      -- Set up buffer-local mappings
      local buf = vim.api.nvim_get_current_buf()
      local keymap = vim.keymap.set
      
      keymap('n', '<leader>cr', '<cmd>CursedRun<cr>', {buffer = buf, desc = 'Run CURSED program'})
      keymap('n', '<leader>cb', '<cmd>CursedBuild<cr>', {buffer = buf, desc = 'Build CURSED program'})
      keymap('n', '<leader>ct', '<cmd>CursedTest<cr>', {buffer = buf, desc = 'Run CURSED tests'})
      keymap('n', '<leader>cf', '<cmd>CursedFormat<cr>', {buffer = buf, desc = 'Format CURSED code'})
      keymap('n', '<leader>cl', '<cmd>CursedLint<cr>', {buffer = buf, desc = 'Lint CURSED code'})
      keymap('n', '<leader>cd', '<cmd>CursedDoc<cr>', {buffer = buf, desc = 'Show CURSED documentation'})
    end,
  })
end

return M
