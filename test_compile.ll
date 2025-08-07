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
  ; Processing statement: sus x drip = 42
  ; Variable: x drip = 42
  %x = alloca i64, align 8
  store i64 42, i64* %x, align 8
  ; Stored variable 'x' with type 'i64'
  ; Processing statement: vibez.spill("Value:", x)
  ; String literal: Value:
  %str_ptr.1 = getelementptr [7 x i8], [7 x i8]* @.str.0, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.1)
  ; Looking for variable: 'x'
  ; Found variable: x (type: i64)
  %loaded.2 = load i64, i64* %x, align 8
  %fmt_ptr.2 = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt_ptr.2, i64 %loaded.2)
  ret i32 0
}
