; Generated LLVM IR for CURSED program
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.str.0 = private unnamed_addr constant [11 x i8] c"Element 1:\00", align 1
@.int_fmt = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1
@.float_fmt = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.bool_true = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.bool_false = private unnamed_addr constant [7 x i8] c"cringe\00", align 1

define i32 @main() {
entry:
  ; Processing statement: sus numbers []drip = [5, -3, 8]
  ; Variable: numbers []drip = [5, -3, 8]
  ; Variable: numbers []drip = [5, -3, 8]
  ; Array with 3 elements
  %numbers = alloca [3 x i32], align 16
  %element_ptr.0.0 = getelementptr [3 x i32], [3 x i32]* %numbers, i32 0, i32 0
  store i32 5, i32* %element_ptr.0.0, align 4
  %element_ptr.0.1 = getelementptr [3 x i32], [3 x i32]* %numbers, i32 0, i32 1
  store i32 -3, i32* %element_ptr.0.1, align 4
  %element_ptr.0.2 = getelementptr [3 x i32], [3 x i32]* %numbers, i32 0, i32 2
  store i32 8, i32* %element_ptr.0.2, align 4
  ; Stored array variable 'numbers' with type '[3 x i32]'
  ; Processing statement: vibez.spill("Element 1:", numbers[1])
  ; String literal: Element 1:
  %str_ptr.2 = getelementptr [11 x i8], [11 x i8]* @.str.0, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.2)
  ; Array access: numbers[1]
  ; Found array: numbers (type: [3 x i32])
  %index.3 = add i32 0, 1
  %element_ptr.4 = getelementptr [3 x i32], [3 x i32]* %numbers, i32 0, i32 %index.3
  %element.4 = load i32, i32* %element_ptr.4, align 4
  %extended.4 = sext i32 %element.4 to i64
  %fmt_ptr.4 = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt_ptr.4, i64 %extended.4)
  ret i32 0
}
