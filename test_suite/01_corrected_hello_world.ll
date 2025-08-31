; ModuleID = '01_corrected_hello_world'
source_filename = "01_corrected_hello_world"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

declare i32 @yap(ptr)

define i32 @main() {
entry:
  %main = alloca i64, align 8
  ret i32 0
}
