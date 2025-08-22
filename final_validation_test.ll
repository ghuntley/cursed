; ModuleID = 'cursed_program'
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"
@str.9 = private unnamed_addr constant [56 x i8] c"✅ All tests passed - Rust to Zig conversion COMPLETE!\00", align 1
@str.8 = private unnamed_addr constant [26 x i8] c"- Ready for production:%s\00", align 1
@str.7 = private unnamed_addr constant [13 x i8] c"- Version:%s\00", align 1
@str.6 = private unnamed_addr constant [14 x i8] c"- Language:%s\00", align 1
@str.5 = private unnamed_addr constant [29 x i8] c"🎉 CURSED Compiler Status:\00", align 1
@str.4 = private unnamed_addr constant [27 x i8] c"Sum 1-10 =%s(should be 55)\00", align 1
@str.3 = private unnamed_addr constant [36 x i8] c"❌ Factorial calculation incorrect\00", align 1
@str.2 = private unnamed_addr constant [35 x i8] c"✅ Factorial calculation correct!\00", align 1
@str.1 = private unnamed_addr constant [17 x i8] c"factorial(5) =%s\00", align 1
@str.0 = private unnamed_addr constant [37 x i8] c"=== CURSED Final Validation Test ===\00", align 1

; String constants

; Main function
define i32 @main() {
entry:
  %name = alloca i64, align 8
  store i64 0, i64* %name, align 8
  %version = alloca i64, align 8
  store i64 1, i64* %version, align 8
  %ready = alloca i64, align 8
  store i64 0, i64* %ready, align 8
  %fact_5 = alloca i64, align 8
  store i64 0, i64* %fact_5, align 8
  %sum = alloca i64, align 8
  store i64 0, i64* %sum, align 8
  %i = alloca i64, align 8
  store i64 1, i64* %i, align 8
  ; String constant for printf
  %call1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([37 x i8], [37 x i8]* @str.0, i32 0, i32 0))
  ; Multi-argument printf call
  %call2 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([17 x i8], [17 x i8]* @str.1, i32 0, i32 0), i64 %fact_5)
  ; String constant for printf
  %call3 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([35 x i8], [35 x i8]* @str.2, i32 0, i32 0))
  ; String constant for printf
  %call4 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([36 x i8], [36 x i8]* @str.3, i32 0, i32 0))
  ; Multi-argument printf call
  %call5 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([27 x i8], [27 x i8]* @str.4, i32 0, i32 0), i64 %sum)
  ; String constant for printf
  %call6 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([29 x i8], [29 x i8]* @str.5, i32 0, i32 0))
  ; Multi-argument printf call
  %call7 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([14 x i8], [14 x i8]* @str.6, i32 0, i32 0), i64 %name)
  ; Multi-argument printf call
  %call8 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([13 x i8], [13 x i8]* @str.7, i32 0, i32 0), i64 %version)
  ; Multi-argument printf call
  %call9 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([26 x i8], [26 x i8]* @str.8, i32 0, i32 0), i64 %ready)
  ; String constant for printf
  %call10 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([56 x i8], [56 x i8]* @str.9, i32 0, i32 0))
  ret i32 0
}

; External function declarations
declare i32 @printf(i8*, ...)
