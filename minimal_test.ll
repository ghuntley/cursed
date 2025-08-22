; ModuleID = 'cursed_program'
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"
@str.1 = private unnamed_addr constant [16 x i8] c"Write result:%s\00", align 1
@str.0 = private unnamed_addr constant [13 x i8] c"Hello World!\00", align 1

; String constants

; Main function
define i32 @main() {
entry:
  %result = alloca i64, align 8
  store i64 0, i64* %result, align 8
  ; String constant for printf
  %call1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([13 x i8], [13 x i8]* @str.0, i32 0, i32 0))
  ; Multi-argument printf call
  %call2 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([16 x i8], [16 x i8]* @str.1, i32 0, i32 0), i64 %result)
  ret i32 0
}

; External function declarations
declare i32 @printf(i8*, ...)
