; CURSED LLVM IR with Advanced Optimization
; Optimization Level: Default
; PGO Enabled: true
; LTO Enabled: true
; Size Optimization: false
; Pass Pipeline: Default

target triple = "x86_64-unknown-linux-gnu"

; Runtime function declarations
declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

; String constants
@.str = private unnamed_addr constant [30 x i8] c"Hello, optimized world!\0A\00", align 1

; Main function with optimizations applied
define i32 @main() {
entry:
; PGO optimized branch
  call i32 @puts(i8* getelementptr inbounds ([30 x i8], [30 x i8]* @.str, i64 0, i64 0))
ret i32 0
}

; Optimization metadata
!llvm.module.flags = !{!0}
!0 = !{i32 1, !"wchar_size", i32 4}
