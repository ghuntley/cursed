; ModuleID = '03_variable_assignment'
source_filename = "03_variable_assignment"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

declare i32 @yap(ptr)

define i32 @main() {
entry:
  ret i32 0
}
