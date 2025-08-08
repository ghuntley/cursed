; Generated LLVM IR for CURSED program
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.str.0 = private unnamed_addr constant [55 x i8] c"Created array") sus index drip = 0 vibez.spill("Index:\00", align 1
@.int_fmt = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1
@.float_fmt = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.bool_true = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.bool_false = private unnamed_addr constant [7 x i8] c"cringe\00", align 1

define i32 @main() {
entry:
  %index = alloca i64, align 8
  store i64 0, i64* %index, align 8
  %loaded.2 = load i64, i64* %index, align 8
  %fmt_ptr.2 = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt_ptr.2, i64 %loaded.2)
  %val = alloca i64, align 8
  store i64 0, i64* %val, align 8
  %loaded.4 = load i64, i64* %val, align 8
  %fmt_ptr.4 = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt_ptr.4, i64 %loaded.4)
  ret i32 0
}
