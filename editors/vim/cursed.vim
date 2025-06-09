" CURSED language support for Vim/Neovim
" File: ~/.vim/syntax/cursed.vim or ~/.config/nvim/syntax/cursed.vim

if exists("b:current_syntax")
  finish
endif

" Keywords
syn keyword cursedKeyword slay yolo facts sus squad collab vibes use from
syn keyword cursedKeyword lowkey highkey periodt bestie flex vibe_check mood basic
syn keyword cursedKeyword bounce yeet catch finally spawn await async
syn keyword cursedKeyword public private chan send recv select defer go
syn keyword cursedConstant true false nil

" Comments
syn match cursedComment "//.*$"
syn region cursedComment start="/\*" end="\*/"

" Strings
syn region cursedString start='"' end='"' contains=cursedEscape
syn region cursedString start="'" end="'" contains=cursedEscape
syn region cursedString start='`' end='`'
syn match cursedEscape /\\./

" Numbers
syn match cursedNumber /\<\d\+\>/
syn match cursedFloat /\<\d\+\.\d\+\([eE][+-]\?\d\+\)\?\>/
syn match cursedHex /\<0[xX][0-9a-fA-F]\+\>/
syn match cursedBinary /\<0[bB][01]\+\>/
syn match cursedOctal /\<0[oO][0-7]\+\>/

" Types
syn keyword cursedType string int float bool char byte array slice map set chan
syn keyword cursedType Vec HashMap Option Result Any interface

" Built-in functions
syn keyword cursedBuiltin print println len str int float type panic spawn make
syn keyword cursedBuiltin append copy delete close

" Operators
syn match cursedOperator /[+\-*/%=<>!&|^]/
syn match cursedOperator /<-/
syn match cursedOperator /&&\|||\|==\|!=\|<=\|>=/

" Function definitions
syn match cursedFunction /\<slay\s\+\w\+\s*(/me=e-1
syn match cursedFunction /\<yolo\s\+\w\+\s*(/me=e-1

" Highlight groups
hi def link cursedKeyword Keyword
hi def link cursedConstant Constant
hi def link cursedComment Comment
hi def link cursedString String
hi def link cursedEscape SpecialChar
hi def link cursedNumber Number
hi def link cursedFloat Float
hi def link cursedHex Number
hi def link cursedBinary Number
hi def link cursedOctal Number
hi def link cursedType Type
hi def link cursedBuiltin Function
hi def link cursedOperator Operator
hi def link cursedFunction Function

let b:current_syntax = "cursed"
