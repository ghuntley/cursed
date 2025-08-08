; Generated LLVM IR for CURSED program
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.str.0 = private unnamed_addr constant [16 x i8] c"Absolute value:\00", align 1
@.int_fmt = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1
@.float_fmt = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.bool_true = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.bool_false = private unnamed_addr constant [7 x i8] c"cringe\00", align 1

define i32 @main() {
entry:
  ; Processing statement: vibez.spill("Absolute value:", abs_normie(-5))
  ; String literal: Absolute value:
  %str_ptr.0 = getelementptr [16 x i8], [16 x i8]* @.str.0, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.0)
  ; Function call: abs_normie(-5)
  %func_result.1 = add i32 0, 5
  %extended.1 = sext i32 %func_result.1 to i64
  %fmt_ptr.1 = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt_ptr.1, i64 %extended.1)
  ret i32 0
}
