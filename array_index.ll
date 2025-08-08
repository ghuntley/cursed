; Generated LLVM IR for CURSED program
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.str.0 = private unnamed_addr constant [7 x i8] c"Value:\00", align 1
@.int_fmt = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1
@.float_fmt = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.bool_true = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.bool_false = private unnamed_addr constant [7 x i8] c"cringe\00", align 1

define i32 @main() {
entry:
  ; Processing statement: sus arr []drip = [1, 2, 3]
  ; Variable: arr []drip = [1, 2, 3]
  ; Variable: arr []drip = [1, 2, 3]
  ; Array with 3 elements
  %arr = alloca [3 x i32], align 16
  %element_ptr.0.0 = getelementptr [3 x i32], [3 x i32]* %arr, i32 0, i32 0
  store i32 1, i32* %element_ptr.0.0, align 4
  %element_ptr.0.1 = getelementptr [3 x i32], [3 x i32]* %arr, i32 0, i32 1
  store i32 2, i32* %element_ptr.0.1, align 4
  %element_ptr.0.2 = getelementptr [3 x i32], [3 x i32]* %arr, i32 0, i32 2
  store i32 3, i32* %element_ptr.0.2, align 4
  ; Stored array variable 'arr' with type '[3 x i32]'
  ; Processing statement: vibez.spill("Value:", arr[0])
  ; String literal: Value:
  %str_ptr.2 = getelementptr [7 x i8], [7 x i8]* @.str.0, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.2)
  ; Array access: arr[0]
  ; Found array: arr (type: [3 x i32])
  %index.3 = add i32 0, 0
  %element_ptr.4 = getelementptr [3 x i32], [3 x i32]* %arr, i32 0, i32 %index.3
  %element.4 = load i32, i32* %element_ptr.4, align 4
  %extended.4 = sext i32 %element.4 to i64
  %fmt_ptr.4 = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt_ptr.4, i64 %extended.4)
  ret i32 0
}
