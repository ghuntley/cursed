" CURSED Lint Functions

function! cursed#lint#current_file() abort
  let l:file = expand('%:p')
  if empty(l:file)
    return
  endif

  let l:cmd = g:cursed_compiler_path . ' lint ' . shellescape(l:file)
  let l:output = system(l:cmd)
  if !empty(l:output)
    echo l:output
  endif
endfunction