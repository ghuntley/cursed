" Vim syntax file for CURSED programming language
" Language: CURSED
" Maintainer: CURSED Language Team
" Latest Revision: 2025

if exists("b:current_syntax")
  finish
endif

" Keywords
syn keyword cursedConditional ready otherwise bestie periodt aight
syn keyword cursedRepeat bestie periodt flex
syn keyword cursedKeyword sus slay damn vibez yeet stan dm chan ready vibe yikes shook fam based cap cringe facts lit tea drip normie smol mid thicc squad collab sick when spill basic mood defer ghosted simp
syn keyword cursedType normie smol mid thicc drip snack meal byte rune extra tea lit sip map chan dm
syn keyword cursedBoolean based cringe cap nah
syn keyword cursedConstant based cringe cap nah
syn keyword cursedStatement damn yikes shook
syn keyword cursedOperator flex dm chan
syn keyword cursedStorageClass sus facts be_like
syn keyword cursedStructure squad collab vibe
syn keyword cursedFunction slay

" Standard library modules
syn keyword cursedStdlib mathz stringz arrayz testz cryptz filez httpz timez jsonz vibez concurrenz

" Standard library functions
syn keyword cursedStdlibFunc abs_normie slice_tea len sha256_hash read_file write_file sleep timestamp make_channel test_start assert_eq_int assert_eq_string assert_true assert_false print_test_summary

" Operators
syn match cursedOperator "\v\+"
syn match cursedOperator "\v-"
syn match cursedOperator "\v\*"
syn match cursedOperator "\v/"
syn match cursedOperator "\v\%"
syn match cursedOperator "\v\="
syn match cursedOperator "\v\!"
syn match cursedOperator "\v\<"
syn match cursedOperator "\v\>"
syn match cursedOperator "\v\&"
syn match cursedOperator "\v\|"
syn match cursedOperator "\v\^"
syn match cursedOperator "\v\~"
syn match cursedOperator "\v\<\="
syn match cursedOperator "\v\>\="
syn match cursedOperator "\v\=\="
syn match cursedOperator "\v\!\="
syn match cursedOperator "\v\&\&"
syn match cursedOperator "\v\|\|"
syn match cursedOperator "\v\<\<"
syn match cursedOperator "\v\>\>"
syn match cursedOperator "\v\:\="
syn match cursedOperator "\v\+\="
syn match cursedOperator "\v-\="
syn match cursedOperator "\v\*\="
syn match cursedOperator "\v/\="
syn match cursedOperator "\v\%\="
syn match cursedOperator "\v\&\="
syn match cursedOperator "\v\|\="
syn match cursedOperator "\v\^\="
syn match cursedOperator "\v\<\<\="
syn match cursedOperator "\v\>\>\="
syn match cursedOperator "\v\+\+"
syn match cursedOperator "\v--"
syn match cursedOperator "\v\<-"

" Numbers
syn match cursedNumber "\v<\d+>"
syn match cursedNumber "\v<0x\x+>"
syn match cursedNumber "\v<0o\o+>"
syn match cursedNumber "\v<0b[01]+>"
syn match cursedFloat "\v<\d+\.\d*>"
syn match cursedFloat "\v<\d+\.\d*e[+-]?\d+>"
syn match cursedFloat "\v<\d+e[+-]?\d+>"
syn match cursedFloat "\v<\.\d+>"
syn match cursedFloat "\v<\.\d+e[+-]?\d+>"

" Strings
syn region cursedString start='"' end='"' contains=cursedEscape
syn region cursedString start="'" end="'" contains=cursedEscape
syn region cursedRawString start='`' end='`'

" Character escapes
syn match cursedEscape contained "\\[nrt\\\"']"
syn match cursedEscape contained "\\x\x\{2}"
syn match cursedEscape contained "\\u\x\{4}"
syn match cursedEscape contained "\\U\x\{8}"

" Comments
syn match cursedComment "//.*$"
syn region cursedComment start="/\*" end="\*/"

" Function definitions
syn match cursedFunction "\v<slay\s+\zs\w+\ze\s*\("

" Variable declarations
syn match cursedVariable "\v<sus\s+\zs\w+\ze\s+"

" Type annotations
syn match cursedTypeAnnotation "\v\w+\s+\zs(normie|smol|mid|thicc|drip|snack|meal|byte|rune|extra|tea|lit|sip)\ze"

" Import statements
syn match cursedImport "\v<yeet\s+"

" Module qualifiers
syn match cursedModuleQualifier "\v\w+\ze\."

" Brackets and delimiters
syn match cursedDelimiter "[\[\]{}();,.]"

" Pattern matching
syn keyword cursedPatternMatch sick when
syn match cursedPatternArrow "->"

" Error handling
syn keyword cursedErrorHandling yikes fam shook

" Concurrency
syn keyword cursedConcurrency stan dm chan ready

" Define highlights
hi def link cursedKeyword Keyword
hi def link cursedConditional Conditional
hi def link cursedRepeat Repeat
hi def link cursedType Type
hi def link cursedBoolean Boolean
hi def link cursedConstant Constant
hi def link cursedStatement Statement
hi def link cursedOperator Operator
hi def link cursedStorageClass StorageClass
hi def link cursedStructure Structure
hi def link cursedFunction Function
hi def link cursedStdlib Include
hi def link cursedStdlibFunc Function
hi def link cursedNumber Number
hi def link cursedFloat Float
hi def link cursedString String
hi def link cursedRawString String
hi def link cursedEscape SpecialChar
hi def link cursedComment Comment
hi def link cursedVariable Identifier
hi def link cursedTypeAnnotation Type
hi def link cursedImport Include
hi def link cursedModuleQualifier Include
hi def link cursedDelimiter Delimiter
hi def link cursedPatternMatch Conditional
hi def link cursedPatternArrow Operator
hi def link cursedErrorHandling Exception
hi def link cursedConcurrency Keyword

let b:current_syntax = "cursed"
