" CURSED Advanced Vim/Neovim Plugin
" Provides comprehensive IDE-like features for CURSED development

if exists('g:loaded_cursed') || &compatible
  finish
endif
let g:loaded_cursed = 1

" Plugin Configuration
let g:cursed_compiler_path = get(g:, 'cursed_compiler_path', 'cursed-zig')
let g:cursed_lsp_enabled = get(g:, 'cursed_lsp_enabled', 1)
let g:cursed_auto_format = get(g:, 'cursed_auto_format', 1)
let g:cursed_auto_lint = get(g:, 'cursed_auto_lint', 1)
let g:cursed_show_diagnostics = get(g:, 'cursed_show_diagnostics', 1)
let g:cursed_enable_folding = get(g:, 'cursed_enable_folding', 1)
let g:cursed_enable_snippets = get(g:, 'cursed_enable_snippets', 1)
let g:cursed_debug_enabled = get(g:, 'cursed_debug_enabled', 1)
let g:cursed_performance_hints = get(g:, 'cursed_performance_hints', 1)

" LSP Configuration
let g:cursed_lsp_server_path = get(g:, 'cursed_lsp_server_path', 'cursed-lsp')
let g:cursed_lsp_debug = get(g:, 'cursed_lsp_debug', 0)
let g:cursed_lsp_trace = get(g:, 'cursed_lsp_trace', 'off')

" Build Configuration
let g:cursed_build_optimization = get(g:, 'cursed_build_optimization', 'debug')
let g:cursed_cross_compile_targets = get(g:, 'cursed_cross_compile_targets', 
      \ ['x86_64-linux', 'x86_64-windows', 'aarch64-macos'])

" File type detection
augroup cursed_filetype
  autocmd!
  autocmd BufNewFile,BufRead *.csd,*.cursed setfiletype cursed
  autocmd BufNewFile,BufRead CursedPackage.toml,CursedWorkspace.toml setfiletype toml
augroup END

" Core Commands
command! -nargs=0 CursedRun call cursed#run#current_file()
command! -nargs=0 CursedBuild call cursed#build#current_file()
command! -nargs=0 CursedBuildRelease call cursed#build#release()
command! -nargs=0 CursedTest call cursed#test#run_all()
command! -nargs=0 CursedFormat call cursed#format#current_file()
command! -nargs=0 CursedLint call cursed#lint#current_file()
command! -nargs=? CursedNewProject call cursed#project#new(<q-args>)
command! -nargs=? CursedNewPackage call cursed#project#new_package(<q-args>)

" Debug Commands
command! -nargs=0 CursedShowAST call cursed#debug#show_ast()
command! -nargs=0 CursedShowTokens call cursed#debug#show_tokens()
command! -nargs=0 CursedDebugStart call cursed#debug#start()
command! -nargs=0 CursedDebugStop call cursed#debug#stop()
command! -nargs=0 CursedDebugToggleBreakpoint call cursed#debug#toggle_breakpoint()

" Performance Commands
command! -nargs=0 CursedProfile call cursed#performance#profile()
command! -nargs=0 CursedBenchmark call cursed#performance#benchmark()
command! -nargs=0 CursedMemoryCheck call cursed#performance#memory_check()
command! -nargs=0 CursedAnalyzePerformance call cursed#performance#analyze()

" Package Management Commands
command! -nargs=1 CursedAddDependency call cursed#package#add_dependency(<q-args>)
command! -nargs=0 CursedUpdateDependencies call cursed#package#update_dependencies()
command! -nargs=0 CursedShowDependencies call cursed#package#show_dependencies()

" Cross Compilation Commands
command! -nargs=1 CursedCrossCompile call cursed#build#cross_compile(<q-args>)
command! -nargs=0 CursedListTargets call cursed#build#list_targets()

" Documentation Commands
command! -nargs=? CursedDoc call cursed#doc#show(<q-args>)
command! -nargs=0 CursedDocGenerate call cursed#doc#generate()

" Key Mappings
if !exists('g:cursed_no_default_mappings') || !g:cursed_no_default_mappings
  " Core mappings
  nmap <leader>cr <Plug>(cursed-run)
  nmap <leader>cb <Plug>(cursed-build)
  nmap <leader>ct <Plug>(cursed-test)
  nmap <leader>cf <Plug>(cursed-format)
  nmap <leader>cl <Plug>(cursed-lint)
  
  " Debug mappings
  nmap <leader>cda <Plug>(cursed-show-ast)
  nmap <leader>cdt <Plug>(cursed-show-tokens)
  nmap <leader>cds <Plug>(cursed-debug-start)
  nmap <leader>cdq <Plug>(cursed-debug-stop)
  nmap <leader>cdb <Plug>(cursed-debug-toggle-breakpoint)
  
  " Performance mappings
  nmap <leader>cpp <Plug>(cursed-profile)
  nmap <leader>cpb <Plug>(cursed-benchmark)
  nmap <leader>cpm <Plug>(cursed-memory-check)
  nmap <leader>cpa <Plug>(cursed-analyze-performance)
  
  " Documentation mappings
  nmap <leader>cd <Plug>(cursed-doc)
  nmap <leader>cg <Plug>(cursed-doc-generate)
  
  " LSP mappings (when available)
  nmap <leader>cgd <Plug>(cursed-goto-definition)
  nmap <leader>cgr <Plug>(cursed-goto-references)
  nmap <leader>cgi <Plug>(cursed-goto-implementation)
  nmap <leader>cgt <Plug>(cursed-goto-type-definition)
  nmap <leader>ch <Plug>(cursed-hover)
  nmap <leader>crn <Plug>(cursed-rename)
  nmap <leader>cca <Plug>(cursed-code-action)
endif

" Plugin mappings
nnoremap <silent> <Plug>(cursed-run) :CursedRun<CR>
nnoremap <silent> <Plug>(cursed-build) :CursedBuild<CR>
nnoremap <silent> <Plug>(cursed-test) :CursedTest<CR>
nnoremap <silent> <Plug>(cursed-format) :CursedFormat<CR>
nnoremap <silent> <Plug>(cursed-lint) :CursedLint<CR>
nnoremap <silent> <Plug>(cursed-show-ast) :CursedShowAST<CR>
nnoremap <silent> <Plug>(cursed-show-tokens) :CursedShowTokens<CR>
nnoremap <silent> <Plug>(cursed-debug-start) :CursedDebugStart<CR>
nnoremap <silent> <Plug>(cursed-debug-stop) :CursedDebugStop<CR>
nnoremap <silent> <Plug>(cursed-debug-toggle-breakpoint) :CursedDebugToggleBreakpoint<CR>
nnoremap <silent> <Plug>(cursed-profile) :CursedProfile<CR>
nnoremap <silent> <Plug>(cursed-benchmark) :CursedBenchmark<CR>
nnoremap <silent> <Plug>(cursed-memory-check) :CursedMemoryCheck<CR>
nnoremap <silent> <Plug>(cursed-analyze-performance) :CursedAnalyzePerformance<CR>
nnoremap <silent> <Plug>(cursed-doc) :CursedDoc<CR>
nnoremap <silent> <Plug>(cursed-doc-generate) :CursedDocGenerate<CR>

" Auto commands
augroup cursed_autocmds
  autocmd!
  
  " Auto format on save
  if g:cursed_auto_format
    autocmd BufWritePre *.csd,*.cursed call cursed#format#current_file()
  endif
  
  " Auto lint on save
  if g:cursed_auto_lint
    autocmd BufWritePost *.csd,*.cursed call cursed#lint#current_file()
  endif
  
  " Set up buffer-local settings
  autocmd FileType cursed call s:setup_cursed_buffer()
  
  " Project detection
  autocmd VimEnter,DirChanged * call cursed#project#detect()
augroup END

" Buffer setup function
function! s:setup_cursed_buffer()
  " Enable syntax highlighting
  setlocal syntax=cursed
  
  " Set indentation
  setlocal expandtab
  setlocal tabstop=4
  setlocal shiftwidth=4
  setlocal softtabstop=4
  
  " Enable folding
  if g:cursed_enable_folding
    setlocal foldmethod=syntax
    setlocal foldlevel=1
  endif
  
  " Set comment strings
  setlocal commentstring=//\ %s
  setlocal comments=://,s1:/*,mb:*,ex:*/
  
  " Enable omni completion
  setlocal omnifunc=cursed#completion#omnifunc
  
  " Set up text width and formatting
  setlocal textwidth=120
  setlocal formatoptions=croql
  
  " Enable spell checking for comments and strings
  setlocal spell spelllang=en_us
  syntax match cursedComment "//.*" contains=@Spell
  syntax region cursedString start=+"+ skip=+\\"+ end=+"+ contains=@Spell
endfunction

" Initialize LSP if available and enabled
if g:cursed_lsp_enabled
  if exists('*lsp#register_server')
    " vim-lsp integration
    call s:setup_vim_lsp()
  elseif exists('g:loaded_coc')
    " coc.nvim integration
    call s:setup_coc_nvim()
  elseif has('nvim') && exists('*nvim_lsp')
    " nvim-lspconfig integration
    call s:setup_nvim_lsp()
  endif
endif

" LSP Setup Functions
function! s:setup_vim_lsp()
  if executable(g:cursed_lsp_server_path)
    au User lsp_setup call lsp#register_server({
          \ 'name': 'cursed-lsp',
          \ 'cmd': {server_info->[g:cursed_lsp_server_path]},
          \ 'allowlist': ['cursed'],
          \ 'config': {
          \   'cursed': {
          \     'initialization_options': {
          \       'enableSemanticTokens': v:true,
          \       'enableCodeLens': v:true,
          \       'enableInlayHints': v:true,
          \       'enableDiagnostics': g:cursed_show_diagnostics,
          \       'debug': g:cursed_lsp_debug
          \     }
          \   }
          \ }
          \ })
    
    " LSP mappings for vim-lsp
    function! s:on_lsp_buffer_enabled() abort
      setlocal omnifunc=lsp#complete
      setlocal signcolumn=yes
      
      if exists('+tagfunc') | setlocal tagfunc=lsp#tagfunc | endif
      
      nmap <buffer> <Plug>(cursed-goto-definition) <plug>(lsp-definition)
      nmap <buffer> <Plug>(cursed-goto-references) <plug>(lsp-references)
      nmap <buffer> <Plug>(cursed-goto-implementation) <plug>(lsp-implementation)
      nmap <buffer> <Plug>(cursed-goto-type-definition) <plug>(lsp-type-definition)
      nmap <buffer> <Plug>(cursed-hover) <plug>(lsp-hover)
      nmap <buffer> <Plug>(cursed-rename) <plug>(lsp-rename)
      nmap <buffer> <Plug>(cursed-code-action) <plug>(lsp-code-action)
      
      " Format on save
      if g:cursed_auto_format
        autocmd BufWritePre <buffer> LspDocumentFormatSync
      endif
    endfunction
    
    augroup cursed_lsp
      autocmd!
      autocmd User lsp_buffer_enabled call s:on_lsp_buffer_enabled()
    augroup END
  endif
endfunction

function! s:setup_coc_nvim()
  " Add CURSED language server to coc-settings.json
  let coc_config = {
        \ 'languageserver': {
        \   'cursed': {
        \     'command': g:cursed_lsp_server_path,
        \     'filetypes': ['cursed'],
        \     'initializationOptions': {
        \       'enableSemanticTokens': v:true,
        \       'enableCodeLens': v:true,
        \       'enableInlayHints': v:true,
        \       'enableDiagnostics': g:cursed_show_diagnostics,
        \       'debug': g:cursed_lsp_debug
        \     }
        \   }
        \ }
        \ }
  
  " COC mappings
  nmap <buffer> <Plug>(cursed-goto-definition) <Plug>(coc-definition)
  nmap <buffer> <Plug>(cursed-goto-references) <Plug>(coc-references)
  nmap <buffer> <Plug>(cursed-goto-implementation) <Plug>(coc-implementation)
  nmap <buffer> <Plug>(cursed-goto-type-definition) <Plug>(coc-type-definition)
  nmap <buffer> <Plug>(cursed-hover) :call CocActionAsync('doHover')<CR>
  nmap <buffer> <Plug>(cursed-rename) <Plug>(coc-rename)
  nmap <buffer> <Plug>(cursed-code-action) <Plug>(coc-codeaction)
endfunction

function! s:setup_nvim_lsp()
  if has('nvim')
    lua << EOF
if pcall(require, 'lspconfig') then
  local lspconfig = require('lspconfig')
  local configs = require('lspconfig.configs')
  
  if not configs.cursed_lsp then
    configs.cursed_lsp = {
      default_config = {
        cmd = {vim.g.cursed_lsp_server_path},
        filetypes = {'cursed'},
        root_dir = function(fname)
          return lspconfig.util.find_git_ancestor(fname) or 
                 lspconfig.util.path.dirname(fname)
        end,
        settings = {},
        init_options = {
          enableSemanticTokens = true,
          enableCodeLens = true,
          enableInlayHints = true,
          enableDiagnostics = vim.g.cursed_show_diagnostics == 1,
          debug = vim.g.cursed_lsp_debug == 1
        }
      }
    }
  end
  
  lspconfig.cursed_lsp.setup({
    on_attach = function(client, bufnr)
      local opts = { noremap=true, silent=true, buffer=bufnr }
      
      vim.keymap.set('n', '<Plug>(cursed-goto-definition)', vim.lsp.buf.definition, opts)
      vim.keymap.set('n', '<Plug>(cursed-goto-references)', vim.lsp.buf.references, opts)
      vim.keymap.set('n', '<Plug>(cursed-goto-implementation)', vim.lsp.buf.implementation, opts)
      vim.keymap.set('n', '<Plug>(cursed-goto-type-definition)', vim.lsp.buf.type_definition, opts)
      vim.keymap.set('n', '<Plug>(cursed-hover)', vim.lsp.buf.hover, opts)
      vim.keymap.set('n', '<Plug>(cursed-rename)', vim.lsp.buf.rename, opts)
      vim.keymap.set('n', '<Plug>(cursed-code-action)', vim.lsp.buf.code_action, opts)
      
      if vim.g.cursed_auto_format == 1 then
        vim.api.nvim_create_autocmd('BufWritePre', {
          buffer = bufnr,
          callback = function()
            vim.lsp.buf.format({ async = false })
          end,
        })
      end
    end,
    capabilities = require('cmp_nvim_lsp').default_capabilities()
  })
end
EOF
  endif
endfunction

" Status line integration
function! cursed#statusline#lsp_status()
  if !g:cursed_lsp_enabled
    return ''
  endif
  
  let l:status = ''
  
  if exists('*lsp#get_server_status')
    " vim-lsp
    let l:servers = lsp#get_server_status()
    if has_key(l:servers, 'cursed-lsp') && l:servers['cursed-lsp'] ==# 'running'
      let l:status = '⚡'
    endif
  elseif exists('g:loaded_coc')
    " coc.nvim
    let l:info = get(b:, 'coc_diagnostic_info', {})
    if empty(l:info) | return '' | endif
    let l:msgs = []
    if get(l:info, 'error', 0)
      call add(l:msgs, '✗' . l:info['error'])
    endif
    if get(l:info, 'warning', 0)
      call add(l:msgs, '⚠' . l:info['warning'])
    endif
    let l:status = join(l:msgs, ' ')
  elseif has('nvim') && exists('*luaeval')
    " nvim-lsp
    let l:status = luaeval('vim.lsp.status()')
  endif
  
  return l:status
endfunction

" Completion function
function! cursed#completion#omnifunc(findstart, base)
  if a:findstart
    " Find the start of the current word
    let line = getline('.')
    let start = col('.') - 1
    while start > 0 && line[start - 1] =~ '\a'
      let start -= 1
    endwhile
    return start
  else
    " Find completions
    let completions = []
    
    " Keywords
    let keywords = [
          \ 'sus', 'drip', 'tea', 'lit', 'cap', 'vibe', 'flex', 'no_cap', 'periodt',
          \ 'slay', 'squad', 'collab', 'yeet', 'nocap', 'vibes',
          \ 'ready', 'otherwise', 'bestie', 'break', 'continue', 'damn',
          \ 'yikes', 'fam', 'shook', 'sick', 'go',
          \ 'and', 'or', 'not', 'in', 'is', 'as'
          \ ]
    
    for keyword in keywords
      if keyword =~ '^' . a:base
        call add(completions, {
              \ 'word': keyword,
              \ 'kind': 'k',
              \ 'menu': '[keyword]'
              \ })
      endif
    endfor
    
    " Built-in functions and modules
    let builtins = [
          \ 'print', 'println', 'spill', 'format', 'parse',
          \ 'len', 'push', 'pop', 'slice', 'map', 'filter', 'reduce',
          \ 'spawn', 'channel', 'select', 'timeout'
          \ ]
    
    for builtin in builtins
      if builtin =~ '^' . a:base
        call add(completions, {
              \ 'word': builtin,
              \ 'kind': 'f',
              \ 'menu': '[builtin]'
              \ })
      endif
    endfor
    
    return completions
  endif
endfunction

" Debugging integration
function! cursed#debug#setup_dap()
  if has('nvim') && exists('*luaeval')
    lua << EOF
if pcall(require, 'dap') then
  local dap = require('dap')
  
  dap.adapters.cursed = {
    type = 'executable',
    command = vim.g.cursed_compiler_path,
    args = {'--debug-adapter'}
  }
  
  dap.configurations.cursed = {
    {
      type = 'cursed',
      request = 'launch',
      name = 'Launch CURSED Program',
      program = '${file}',
      cwd = '${workspaceFolder}',
      args = {},
      stopOnEntry = false,
      runInTerminal = false,
    },
    {
      type = 'cursed',
      request = 'launch',
      name = 'Launch with Arguments',
      program = '${file}',
      cwd = '${workspaceFolder}',
      args = function()
        return vim.split(vim.fn.input('Arguments: '), ' ')
      end,
      stopOnEntry = false,
      runInTerminal = false,
    }
  }
end
EOF
  endif
endfunction

" Initialize debugging if nvim-dap is available
if has('nvim') && g:cursed_debug_enabled
  autocmd VimEnter * call cursed#debug#setup_dap()
endif

" Help tags
if exists('g:loaded_cursed_help')
  finish
endif
let g:loaded_cursed_help = 1

" Generate help tags
silent! execute 'helptags ' . fnamemodify(resolve(expand('<sfile>:p')), ':h:h') . '/doc'

" Health check for Neovim
if has('nvim')
  function! health#cursed#check() abort
    call health#report_start('CURSED Plugin Health Check')
    
    " Check compiler
    if executable(g:cursed_compiler_path)
      call health#report_ok('CURSED compiler found: ' . g:cursed_compiler_path)
    else
      call health#report_error('CURSED compiler not found: ' . g:cursed_compiler_path)
    endif
    
    " Check LSP server
    if g:cursed_lsp_enabled
      if executable(g:cursed_lsp_server_path)
        call health#report_ok('CURSED LSP server found: ' . g:cursed_lsp_server_path)
      else
        call health#report_warn('CURSED LSP server not found: ' . g:cursed_lsp_server_path)
      endif
    endif
    
    " Check dependencies
    if exists('*lsp#register_server')
      call health#report_ok('vim-lsp detected')
    elseif exists('g:loaded_coc')
      call health#report_ok('coc.nvim detected')
    elseif exists('*nvim_lsp')
      call health#report_ok('nvim-lspconfig detected')
    else
      call health#report_info('No LSP client detected - LSP features will be limited')
    endif
  endfunction
endif

echo "CURSED Advanced Vim Plugin loaded successfully!"
