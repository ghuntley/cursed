; ModuleID = 'cursed_ir_demo'
source_filename = "cursed_ir_demo"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0

declare i32 @puts(ptr)

declare i32 @printf(ptr, ...)

define void @main_character() {
entry:
  ret void
}

define i32 @main() {
entry:
  ret i32 0
}
