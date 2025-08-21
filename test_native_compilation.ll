; ModuleID = 'cursed_program'
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"
@str.2 = private unnamed_addr constant [43 x i8] c"=== Native compilation test completed! ===\00", align 1
@str.1 = private unnamed_addr constant [26 x i8] c"File written successfully\00", align 1
@str.0 = private unnamed_addr constant [39 x i8] c"=== CURSED Native Compilation Test ===\00", align 1

; String constants

; Main function
define i32 @main() {
entry:
  ; String constant for printf
  %call1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([39 x i8], [39 x i8]* @str.0, i32 0, i32 0))
  %result = alloca i64, align 8
  store i64 0, i64* %result, align 8
  %file_content = alloca i64, align 8
  store i64 0, i64* %file_content, align 8
  ; String constant for printf
  %call2 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([26 x i8], [26 x i8]* @str.1, i32 0, i32 0))
  %url = alloca i64, align 8
  store i64 0, i64* %url, align 8
  ; String constant for printf
  %call3 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([43 x i8], [43 x i8]* @str.2, i32 0, i32 0))
  ret i32 0
}

; External function declarations
declare i32 @printf(i8*, ...)
