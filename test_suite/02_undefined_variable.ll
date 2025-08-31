; ModuleID = '02_undefined_variable'
source_filename = "02_undefined_variable"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

declare i32 @yap(ptr)

define void @main_character() {
entry:
  ret void
}

define i32 @main() {
entry:
  call void @main_character()
  ret i32 0
}
