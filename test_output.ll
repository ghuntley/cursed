; ModuleID = 'test_ir_module'
source_filename = "test_ir_module"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

define void @main_character() {
entry:
  ret void
}

define i32 @main() {
entry:
  call void @main_character()
  ret i32 0
}
