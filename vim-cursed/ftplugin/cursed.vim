" CURSED filetype plugin
" Language: CURSED
" Maintainer: CURSED Language Team

if exists("b:did_ftplugin")
  finish
endif
let b:did_ftplugin = 1

" Set local options
setlocal commentstring=fr\ fr\ %s
setlocal comments=fr\ fr,no\ cap:on\ god
setlocal formatoptions-=t formatoptions+=croql
setlocal suffixesadd=.💀
setlocal tabstop=4
setlocal shiftwidth=4
setlocal expandtab
setlocal autoindent
setlocal smartindent

" Set up folding
setlocal foldmethod=syntax
setlocal foldlevel=99

" Indentation
setlocal indentexpr=CursedIndent()
setlocal indentkeys=0{,0},0),0],!^F,o,O,e

" Matchit support
if exists("loaded_matchit")
  let b:match_words = '\<ready\>:\<otherwise\>,\<bestie\>:\<aight\>,\<sick\>:\<when\>,\<slay\>:\<damn\>'
endif

" Compilation settings
compiler cursed

" Key mappings
nnoremap <buffer> <LocalLeader>r :CursedRun<CR>
nnoremap <buffer> <LocalLeader>b :CursedBuild<CR>
nnoremap <buffer> <LocalLeader>t :CursedTest<CR>
nnoremap <buffer> <LocalLeader>f :CursedFormat<CR>
nnoremap <buffer> <LocalLeader>l :CursedLint<CR>

" Commands
command! -buffer CursedRun call cursed#run()
command! -buffer CursedBuild call cursed#build()
command! -buffer CursedTest call cursed#test()
command! -buffer CursedFormat call cursed#format()
command! -buffer CursedLint call cursed#lint()
command! -buffer CursedDoc call cursed#doc()

" Abbreviations for common patterns
iabbrev <buffer> slay slay<Space>
iabbrev <buffer> sus sus<Space>
iabbrev <buffer> damn damn<Space>
iabbrev <buffer> ready ready<Space>
iabbrev <buffer> otherwise otherwise<Space>
iabbrev <buffer> bestie bestie<Space>
iabbrev <buffer> squad squad<Space>
iabbrev <buffer> collab collab<Space>
iabbrev <buffer> yeet yeet<Space>

" Syntax-aware text objects
call textobj#user#plugin('cursed', {
\   'function': {
\     'pattern': '\<slay\>\s\+\w\+\s*(\_.\{-})\s*{\_.\{-}}',
\     'select-a': 'af',
\     'select-i': 'if',
\   },
\   'struct': {
\     'pattern': '\<squad\>\s\+\w\+\s*{\_.\{-}}',
\     'select-a': 'as',
\     'select-i': 'is',
\   },
\   'interface': {
\     'pattern': '\<collab\>\s\+\w\+\s*{\_.\{-}}',
\     'select-a': 'ac',
\     'select-i': 'ic',
\   },
\ })

" Undo changes
let b:undo_ftplugin = "setl cms< com< fo< sua< ts< sw< et< ai< si< fdm< fdl< inde< indk<"
      \ . "| unlet! b:match_words"

function! CursedIndent()
  let line = getline(v:lnum)
  let prevline = getline(v:lnum - 1)
  
  " Don't change indent for comments
  if line =~ '^\s*//'
    return -1
  endif
  
  let ind = indent(v:lnum - 1)
  
  " Increase indent after opening braces, control structures
  if prevline =~ '\v(\{|ready|otherwise|bestie|slay|squad|collab|vibe_check).*$'
    let ind += &shiftwidth
  endif
  
  " Decrease indent for closing braces
  if line =~ '^\s*}'
    let ind -= &shiftwidth
  endif
  
  " Decrease indent for 'otherwise'
  if line =~ '^\s*otherwise'
    let ind -= &shiftwidth
  endif
  
  return ind
endfunction
