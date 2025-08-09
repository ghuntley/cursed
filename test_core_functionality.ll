; Generated LLVM IR for CURSED program
target triple = "x86_64-unknown-linux-gnu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.str.0 = private unnamed_addr constant [8 x i8] c"Result:\00", align 1
@.int_fmt = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1
@.float_fmt = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.bool_true = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.bool_false = private unnamed_addr constant [7 x i8] c"cringe\00", align 1

define i64 @test_function(i64 %a, i64 %b) {
entry:
  %result = mul i64 %a, %b
  ret i64 %result
}

define i32 @main() {
entry:
  %x = alloca i64, align 8
  store i64 42, i64* %x, align 8
  %y = alloca i64, align 8
  %loaded_op.1 = load i64, i64* %x, align 8
  %add_result.2 = add i64 %loaded_op.1, 8
  store i64 %add_result.2, i64* %y, align 8
  %expr4 = alloca i64, align 8
  store i64 0, i64* %expr4, align 8
  %result = alloca i64, align 8
  %call_result.5 = call i64 @test_function(  %arg_load.5 = load i64, i64* %x, align 8
i64 %arg_load.5,   %arg_load.6 = load i64, i64* %y, align 8
i64 %arg_load.6)
  store i64 %call_result.5, i64* %result, align 8
  %str_ptr.8 = getelementptr [8 x i8], [8 x i8]* @.str.0, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.8)
  %loaded.9 = load i64, i64* %result, align 8
  %fmt_ptr.9 = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt_ptr.9, i64 %loaded.9)
  ret i32 0
}
