; ModuleID = 'cursed_program'
source_filename = "cursed_program"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; External function declarations
declare i32 @printf(i8*, ...)

@.str = private unnamed_addr constant [12 x i8] c"Value: %ld\0A\00", align 1

define i32 @main() {
  %1 = alloca i64
  store i64 42, i64* %1
  %2 = load i64, i64* %1
  %3 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @.str, i32 0, i32 0), i64 %2)
  ret i32 0
}
