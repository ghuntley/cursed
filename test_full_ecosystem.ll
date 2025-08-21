; ModuleID = 'cursed_program'
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"
@str.1 = private unnamed_addr constant [52 x i8] c"=== Full ecosystem test completed successfully! ===\00", align 1
@str.0 = private unnamed_addr constant [35 x i8] c"=== CURSED Full Ecosystem Test ===\00", align 1

; String constants

; Main function
define i32 @main() {
entry:
  ; String constant for printf
  %call1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([35 x i8], [35 x i8]* @str.0, i32 0, i32 0))
  %pi_val = alloca i64, align 8
  store i64 0, i64* %pi_val, align 8
  %sqrt_result = alloca i64, align 8
  store i64 0, i64* %sqrt_result, align 8
  %message = alloca i64, align 8
  store i64 0, i64* %message, align 8
  %msg_length = alloca i64, align 8
  store i64 0, i64* %msg_length, align 8
  %test_content = alloca i64, align 8
  store i64 0, i64* %test_content, align 8
  %read_content = alloca i64, align 8
  store i64 0, i64* %read_content, align 8
  %hash_input = alloca i64, align 8
  store i64 0, i64* %hash_input, align 8
  %hash_result = alloca i64, align 8
  store i64 0, i64* %hash_result, align 8
  %api_url = alloca i64, align 8
  store i64 0, i64* %api_url, align 8
  ; String constant for printf
  %call2 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([52 x i8], [52 x i8]* @str.1, i32 0, i32 0))
  ret i32 0
}

; External function declarations
declare i32 @printf(i8*, ...)
