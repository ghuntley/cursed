; Generated LLVM IR for CURSED program
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.str.0 = private unnamed_addr constant [25 x i8] c"Testing enhanced codegen\00", align 1
@.int_fmt = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1
@.float_fmt = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.bool_true = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.bool_false = private unnamed_addr constant [7 x i8] c"cringe\00", align 1

define i32 @main() {
entry:
  ; Variable: x drip = 42
  %x = alloca i64, align 8
  store i64 42, i64* %x, align 8
  ; Variable: y meal = 3.14
  %y = alloca double, align 8
  store double 3.14e0, double* %y, align 8
  ; Variable: name tea = "Hello"
  %name = alloca [6 x i8], align 1
  %name.ptr.0 = getelementptr [6 x i8], [6 x i8]* %name, i32 0, i32 0
  store i8 72, i8* %name.ptr.0, align 1
  %name.ptr.1 = getelementptr [6 x i8], [6 x i8]* %name, i32 0, i32 1
  store i8 101, i8* %name.ptr.1, align 1
  %name.ptr.2 = getelementptr [6 x i8], [6 x i8]* %name, i32 0, i32 2
  store i8 108, i8* %name.ptr.2, align 1
  %name.ptr.3 = getelementptr [6 x i8], [6 x i8]* %name, i32 0, i32 3
  store i8 108, i8* %name.ptr.3, align 1
  %name.ptr.4 = getelementptr [6 x i8], [6 x i8]* %name, i32 0, i32 4
  store i8 111, i8* %name.ptr.4, align 1
  %name.ptr.5 = getelementptr [6 x i8], [6 x i8]* %name, i32 0, i32 5
  store i8 0, i8* %name.ptr.5, align 1
  ; Variable: result drip = 50
  %result = alloca i64, align 8
  store i64 50, i64* %result, align 8
  ; String literal: Testing enhanced codegen
  %str_ptr.4 = getelementptr [25 x i8], [25 x i8]* @.str.0, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.4)
  ret i32 0
}
