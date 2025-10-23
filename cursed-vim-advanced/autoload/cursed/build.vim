" CURSED Build Functions

function! cursed#build#current_file() abort
  let l:file = expand('%:p')
  if empty(l:file)
    echoerr "No file to build"
    return
  endif

  let l:cmd = g:cursed_compiler_path . ' build ' . shellescape(l:file)
  execute '!' . l:cmd
endfunction

function! cursed#build#release() abort
  let l:file = expand('%:p:h')
  let l:cmd = g:cursed_compiler_path . ' build --release ' . shellescape(l:file)
  execute '!' . l:cmd
endfunction

function! cursed#build#cross_compile(target) abort
  let l:file = expand('%:p')
  let l:cmd = g:cursed_compiler_path . ' build --target ' . shellescape(a:target) . ' ' . shellescape(l:file)
  execute '!' . l:cmd
endfunction

function! cursed#build#list_targets() abort
  echo "Available targets:"
  echo "x86_64-linux"
  echo "x86_64-windows"
  echo "aarch64-macos"
endfunction