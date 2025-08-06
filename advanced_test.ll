; Generated LLVM IR for CURSED program
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.int_fmt = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1
@.float_fmt = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.bool_true = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.bool_false = private unnamed_addr constant [7 x i8] c"cringe\00", align 1

define i32 @main() {
entry:
  ; Variable: x drip = 42
  %x = alloca i64, align 8
  store i64 42, i64* %x, align 8
  ; Variable: y tea = "Hello CURSED!"
  %y = alloca [14 x i8], align 1
  %y.ptr.0 = getelementptr [14 x i8], [14 x i8]* %y, i32 0, i32 0
  store i8 72, i8* %y.ptr.0, align 1
  %y.ptr.1 = getelementptr [14 x i8], [14 x i8]* %y, i32 0, i32 1
  store i8 101, i8* %y.ptr.1, align 1
  %y.ptr.2 = getelementptr [14 x i8], [14 x i8]* %y, i32 0, i32 2
  store i8 108, i8* %y.ptr.2, align 1
  %y.ptr.3 = getelementptr [14 x i8], [14 x i8]* %y, i32 0, i32 3
  store i8 108, i8* %y.ptr.3, align 1
  %y.ptr.4 = getelementptr [14 x i8], [14 x i8]* %y, i32 0, i32 4
  store i8 111, i8* %y.ptr.4, align 1
  %y.ptr.5 = getelementptr [14 x i8], [14 x i8]* %y, i32 0, i32 5
  store i8 32, i8* %y.ptr.5, align 1
  %y.ptr.6 = getelementptr [14 x i8], [14 x i8]* %y, i32 0, i32 6
  store i8 67, i8* %y.ptr.6, align 1
  %y.ptr.7 = getelementptr [14 x i8], [14 x i8]* %y, i32 0, i32 7
  store i8 85, i8* %y.ptr.7, align 1
  %y.ptr.8 = getelementptr [14 x i8], [14 x i8]* %y, i32 0, i32 8
  store i8 82, i8* %y.ptr.8, align 1
  %y.ptr.9 = getelementptr [14 x i8], [14 x i8]* %y, i32 0, i32 9
  store i8 83, i8* %y.ptr.9, align 1
  %y.ptr.10 = getelementptr [14 x i8], [14 x i8]* %y, i32 0, i32 10
  store i8 69, i8* %y.ptr.10, align 1
  %y.ptr.11 = getelementptr [14 x i8], [14 x i8]* %y, i32 0, i32 11
  store i8 68, i8* %y.ptr.11, align 1
  %y.ptr.12 = getelementptr [14 x i8], [14 x i8]* %y, i32 0, i32 12
  store i8 33, i8* %y.ptr.12, align 1
  %y.ptr.13 = getelementptr [14 x i8], [14 x i8]* %y, i32 0, i32 13
  store i8 0, i8* %y.ptr.13, align 1
  ; Variable: y
  %str_ptr.2 = getelementptr [14 x i8], [14 x i8]* %y, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.2)
  ; Unknown variable: "Number: " + tea(x)
  ret i32 0
}
