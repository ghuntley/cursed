" CURSED Syntax Highlighting for Vim/Neovim
" Language: CURSED
" Maintainer: CURSED Team
" Latest Revision: 1.0

if exists("b:current_syntax")
  finish
endif

" Keywords
syn keyword cursedKeyword vibe yeet facts sus be_like slay squad collab map dm ඞ
syn keyword cursedConditional ready otherwise vibe_check mood basic
syn keyword cursedRepeat bestie flex periodt
syn keyword cursedStatement damn ghosted simp later stan dm_send dm_recv dm_close
syn keyword cursedException yikes fam shook

" Types
syn keyword cursedType normie smol mid thicc snack meal byte rune sip extra tea lit channel dm

" Constants
syn keyword cursedBoolean based cringe
syn keyword cursedConstant nah

" Strings
syn region cursedString start=+"+ skip=+\\"+ end=+"+
syn region cursedRawString start=+`+ end=+`+

" Characters
syn region cursedCharacter start=+'+ skip=+\\'+ end=+'+

" Numbers
syn match cursedNumber "\<\d\+\>"
syn match cursedNumber "\<\d\+\.\d*\>"
syn match cursedNumber "\<\d*\.\d\+\>"
syn match cursedNumber "\<\d\+e[+-]\?\d\+\>"
syn match cursedNumber "\<\d\+\.\d*e[+-]\?\d\+\>"
syn match cursedHexNumber "\<0[xX][0-9a-fA-F]\+\>"
syn match cursedOctNumber "\<0[oO][0-7]\+\>"
syn match cursedBinNumber "\<0[bB][01]\+\>"

" Comments
syn region cursedComment start="no cap" end="on god" contains=cursedTodo
syn region cursedComment start="fr fr" end="$" contains=cursedTodo
syn region cursedBlockComment start="fr fr" end="$" contains=cursedTodo
syn region cursedBlockComment start="no cap" end="on god" contains=cursedTodo

" TODO
syn keyword cursedTodo TODO FIXME XXX NOTE contained

" Operators
syn match cursedOperator "[+\-*/%&|^~!<>=]=?"
syn match cursedOperator "++"
syn match cursedOperator "--"
syn match cursedOperator ":="
syn match cursedOperator "\.\.\."
syn match cursedOperator "ඞ"

" Delimiters
syn match cursedDelimiter "[{}()\[\],;.]"

" Identifiers
syn match cursedIdentifier "\<\h\w*\>"

" Functions
syn match cursedFunction "\<\h\w*\ze\s*("

" Types (user-defined)
syn match cursedType "\<\u\w*\>"

" Highlighting links
hi def link cursedKeyword Keyword
hi def link cursedConditional Conditional
hi def link cursedRepeat Repeat
hi def link cursedStatement Statement
hi def link cursedException Exception
hi def link cursedOperator Operator
hi def link cursedType Type
hi def link cursedBoolean Boolean
hi def link cursedConstant Constant
hi def link cursedString String
hi def link cursedRawString String
hi def link cursedCharacter Character
hi def link cursedNumber Number
hi def link cursedHexNumber Number
hi def link cursedOctNumber Number
hi def link cursedBinNumber Number
hi def link cursedComment Comment
hi def link cursedBlockComment Comment
hi def link cursedTodo Todo
hi def link cursedDelimiter Delimiter
hi def link cursedIdentifier Identifier
hi def link cursedFunction Function

let b:current_syntax = "cursed"