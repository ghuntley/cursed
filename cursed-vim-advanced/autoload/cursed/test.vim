" CURSED Test Functions

function! cursed#test#run_all() abort
  let l:cmd = g:cursed_compiler_path . ' test'
  execute '!' . l:cmd
endfunction