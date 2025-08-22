; ModuleID = 'cursed_program'
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"
@str.7 = private unnamed_addr constant [37 x i8] c"=== Test completed successfully! ===\00", align 1
@str.6 = private unnamed_addr constant [18 x i8] c"Loop iteration:%s\00", align 1
@str.5 = private unnamed_addr constant [30 x i8] c"Result is not greater than 20\00", align 1
@str.4 = private unnamed_addr constant [27 x i8] c"Result is greater than 20!\00", align 1
@str.3 = private unnamed_addr constant [12 x i8] c"10 + 15 =%s\00", align 1
@str.2 = private unnamed_addr constant [23 x i8] c"Name:%sAge:%sActive:%s\00", align 1
@str.1 = private unnamed_addr constant [34 x i8] c"=== CURSED Comprehensive Test ===\00", align 1
@str.0 = private unnamed_addr constant [9 x i8] c"Hello%s!\00", align 1

; String constants

; Main function
define i32 @main() {
entry:
  %name = alloca i64, align 8
  store i64 0, i64* %name, align 8
  %age = alloca i64, align 8
  store i64 25, i64* %age, align 8
  %active = alloca i64, align 8
  store i64 0, i64* %active, align 8
  %result = alloca i64, align 8
  store i64 0, i64* %result, align 8
  %i = alloca i64, align 8
  store i64 0, i64* %i, align 8
  ; Multi-argument printf call
  %call1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([9 x i8], [9 x i8]* @str.0, i32 0, i32 0), i64 %name)
  ; String constant for printf
  %call2 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([34 x i8], [34 x i8]* @str.1, i32 0, i32 0))
  ; Multi-argument printf call
  %call3 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([23 x i8], [23 x i8]* @str.2, i32 0, i32 0), i64 %name, i64 %age, i64 %active)
  ; Multi-argument printf call
  %call4 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @str.3, i32 0, i32 0), i64 %result)
  ; String constant for printf
  %call5 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([27 x i8], [27 x i8]* @str.4, i32 0, i32 0))
  ; String constant for printf
  %call6 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([30 x i8], [30 x i8]* @str.5, i32 0, i32 0))
  ; Multi-argument printf call
  %call7 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([18 x i8], [18 x i8]* @str.6, i32 0, i32 0), i64 %i)
  ; String constant for printf
  %call8 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([37 x i8], [37 x i8]* @str.7, i32 0, i32 0))
  ret i32 0
}

; External function declarations
declare i32 @printf(i8*, ...)
