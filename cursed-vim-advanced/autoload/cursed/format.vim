" CURSED Format Functions

function! cursed#format#current_file() abort
  let l:file = expand('%:p')
  if empty(l:file)
    return
  endif

  let l:cmd = g:cursed_compiler_path . ' fmt ' . shellescape(l:file)
  let l:output = system(l:cmd)
  if v:shell_error == 0
    " Replace buffer content with formatted output
    let l:lines = split(l:output, '\n')
    call setline(1, l:lines)
    execute 'delete ' . len(l:lines) . ',$'
  else
    echoerr "Format failed: " . l:output
  endif
endfunction