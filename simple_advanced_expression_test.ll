; Generated LLVM IR for CURSED program
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.str.0 = private unnamed_addr constant [31 x i8] c"Simple advanced test completed\00", align 1
@.int_fmt = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1
@.float_fmt = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.bool_true = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.bool_false = private unnamed_addr constant [7 x i8] c"cringe\00", align 1

define i32 @main() {
entry:
  ; Variable: x normie = 42
  %x = alloca i32, align 4
  store i32 42, i32* %x, align 4
  ; Unknown variable: "Test:", x
  ; Variable: result normie = x + 10
  %result = alloca i32, align 4
  %loaded_op.1 = load i32, i32* %x, align 4
  %add_result.2 = add i32 %loaded_op.1, 10
  store i32 %add_result.2, i32* %result, align 4
  ; Unknown variable: "Result:", result
  ; String literal: Simple advanced test completed
  %str_ptr.4 = getelementptr [31 x i8], [31 x i8]* @.str.0, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.4)
  ret i32 0
}
