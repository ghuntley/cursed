; ModuleID = '01_hello_world'
source_filename = "01_hello_world"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0
@.str.0 = private constant [23 x i8] c"\22Hello, CURSED World!\22\00"

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

declare i32 @yap(ptr)

define void @main_character() {
entry:
  %puts_call = call i32 @puts(ptr @.str.0)
  ret void
}

define i32 @main() {
entry:
  call void @main_character()
  ret i32 0
}
