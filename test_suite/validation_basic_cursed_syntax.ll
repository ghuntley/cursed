; ModuleID = 'validation_basic_cursed_syntax'
source_filename = "validation_basic_cursed_syntax"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

declare i32 @yap(ptr)

define i32 @main() {
entry:
  %call_tmp = call i32 @main()
  ret i32 %call_tmp
}
