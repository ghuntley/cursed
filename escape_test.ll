; Generated LLVM IR for CURSED program
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.str.0 = private unnamed_addr constant [23 x i8] c"escaped quote: \\\"hello\\\"\00", align 1
@.str.1 = private unnamed_addr constant [22 x i8] c"nested () parentheses\00", align 1
@.int_fmt = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1
@.float_fmt = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.bool_true = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.bool_false = private unnamed_addr constant [7 x i8] c"cringe\00", align 1

define i32 @main() {
entry:
  %str_ptr.0 = getelementptr [23 x i8], [23 x i8]* @.str.0, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.0)
  %str_ptr.1 = getelementptr [22 x i8], [22 x i8]* @.str.1, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.1)
  ret i32 0
}
