; ModuleID = 'cursed_program'
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"
@str.3 = private unnamed_addr constant [41 x i8] c"Full parser test completed successfully!\00", align 1
@str.2 = private unnamed_addr constant [27 x i8] c"Sum is not greater than 25\00", align 1
@str.1 = private unnamed_addr constant [24 x i8] c"Sum is greater than 25!\00", align 1
@str.0 = private unnamed_addr constant [35 x i8] c"Testing full parser integration...\00", align 1

; String constants

; Main function
define i32 @main() {
entry:
  %result = alloca i64, align 8
  store i64 0, i64* %result, align 8
  ; String constant for printf
  %call1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([35 x i8], [35 x i8]* @str.0, i32 0, i32 0))
  %x = alloca i64, align 8
  store i64 10, i64* %x, align 8
  %y = alloca i64, align 8
  store i64 20, i64* %y, align 8
  %sum = alloca i64, align 8
  store i64 0, i64* %sum, align 8
  ; String constant for printf
  %call2 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([24 x i8], [24 x i8]* @str.1, i32 0, i32 0))
  ; String constant for printf
  %call3 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([27 x i8], [27 x i8]* @str.2, i32 0, i32 0))
  %i = alloca i64, align 8
  store i64 0, i64* %i, align 8
  ; String constant for printf
  %call4 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([41 x i8], [41 x i8]* @str.3, i32 0, i32 0))
  ret i32 0
}

; External function declarations
declare i32 @printf(i8*, ...)
