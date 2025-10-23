" CURSED Run Functions
" Provides functions for running CURSED programs

function! cursed#run#current_file() abort
  let l:file = expand('%:p')
  if empty(l:file)
    echoerr "No file to run"
    return
  endif

  " Check if file has .💀 or .cursed extension
  if l:file !~ '\.\(💀\|cursed\)$'
    echoerr "Not a CURSED file"
    return
  endif

  " Run the file using the CURSED compiler
  let l:cmd = g:cursed_compiler_path . ' run ' . shellescape(l:file)
  execute '!' . l:cmd
endfunction