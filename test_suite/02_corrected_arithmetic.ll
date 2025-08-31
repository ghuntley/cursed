; ModuleID = '02_corrected_arithmetic'
source_filename = "02_corrected_arithmetic"
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
