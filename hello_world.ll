; ModuleID = 'cursed_program'
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"
@str.1 = private unnamed_addr constant [31 x i8] c"CURSED compilation successful!\00", align 1
@str.0 = private unnamed_addr constant [14 x i8] c"Hello, World!\00", align 1

; String constants

; Main function
define i32 @main() {
entry:
  ; String constant for printf
  %call1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([14 x i8], [14 x i8]* @str.0, i32 0, i32 0))
  ; String constant for printf
  %call2 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([31 x i8], [31 x i8]* @str.1, i32 0, i32 0))
  ret i32 0
}

; External function declarations
declare i32 @printf(i8*, ...)
