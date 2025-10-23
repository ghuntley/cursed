" Vim syntax file for CURSED programming language
" Language: CURSED
" Maintainer: CURSED Language Team
" Latest Revision: 2025

if exists("b:current_syntax")
   finish
endif

" Comments
syn region cursedComment start="no cap" end="on god" contains=cursedTodo
syn region cursedComment start="fr fr" end="$" contains=cursedTodo

" TODO
syn keyword cursedTodo TODO FIXME XXX NOTE contained

" Keywords
syn keyword cursedConditional ready otherwise vibe_check mood basic
syn keyword cursedRepeat bestie
syn keyword cursedKeyword vibe yeet facts sus be_like slay squad collab dm_send dm_recv dm_close yikes fam shook
syn keyword cursedStatement damn ghosted simp later stan
syn keyword cursedType normie smol mid thicc snack meal byte rune extra tea lit sip map dm ඞ
syn keyword cursedBoolean based cringe
syn keyword cursedConstant nah
syn keyword cursedStatement damn yikes shook ghosted simp later
syn keyword cursedOperator flex
syn keyword cursedStorageClass sus facts be_like
syn keyword cursedStructure squad collab
syn keyword cursedFunction slay
syn keyword cursedErrorHandling yikes fam shook
syn keyword cursedConcurrency stan dm_send dm_recv dm_close ready

 " Additional language keywords from specification

" Standard library modules
syn keyword cursedStdlib mathz stringz arrayz testz cryptz filez httpz timez jsonz vibez concurrenz
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
 syn match cursedOperator "\v\<\<\=
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
syn match cursedFloat "\v<\.\d+e[+-]?\\d+>"
   
" Strings
syn region cursedString start='"' end='"' contains=cursedEscape
syn region cursedString start="'" end="'" contains=cursedEscape
syn region cursedRawString start='`' end='`'

" Character escapes
syn match cursedEscape contained "\\[nrt\\\"']"
syn match cursedEscape contained \\x\x\{2}
syn match cursedEscape contained \\u\x\{4}
syn match cursedEscape contained \\U\x\{8}

" Comments
syn match cursedComment "//.*$"
syn region cursedComment start="/\*" end="\*/"

" Function definitions
syn match cursedFunction "\v<slay\s+\zs\w+\ze\s*\("
syn match cursedFunction "\v<slay\s+\zs\w+\ze\s*\("
syn match cursedFunction "\v<slay\s+\zs\w+\ze\s*\("
 
" Variable declarations
syn match cursedVariable "\v<sus\s+\zs\w+\ze\s+"
syn match cursedVariable "\v<sus\s+\zs\w+\ze\s+"
 
" Type annotations
syn match cursedTypeAnnotation "\v\w+\s+\zs(normie|smol|mid|thicc|drip|snack|meal|byte|rune|extra|tea|lit|sip|channel)\ze"
 
" Import statement
 syn match cursedImport "\v<yeet\s+"
syn match cursedImport "\v<yeet\s+"
 
" Module qualifiers
syn match cursedModuleQualifier "\v\w+\ze\."
syn match cursedModuleQualifier "\v\w+\ze\."
 
" Brackets and delimiters
syn match cursedDelimiter "[\[\]{}();,.]"
syn match cursedDelimiter "[\[\]{}();,.]"
   
" Pattern matching
syn match cursedPatternArrow "->"
syn match cursedPatternArrow "->"
 
" Error handling
syn keyword cursedErrorHandling yikes fam shook
syn keyword cursedErrorHandling yikes fam shook
 
" Concurrency
syn keyword cursedConcurrency stan dm chan ready
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

" Additional keywords from specification
syn keyword cursedKeyword ghosted simp defer
syn keyword cursedKeyword channel dm
syn keyword cursedKeyword channel dm
syn keyword cursedType channel dm

let b:current_syntax = "cursed"