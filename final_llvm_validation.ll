; ModuleID = 'cursed_program'
source_filename = "cursed_program"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

declare i32 @printf(i8*, ...)

@.str = private unnamed_addr constant [12 x i8] c"Value: %ld\0A\00", align 1
define i64 @add(i64 %x, i64 %y) {
  %result = add i64 %x, %y
  ret i64 %result
}
define i32 @main() {  %1 = call i64 @add(i64 2, i64 3)
  %2 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @.str, i32 0, i32 0), i64 %1)  ret i32 0
}
