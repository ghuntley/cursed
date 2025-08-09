; Generated LLVM IR for CURSED program
target triple = "wasm32-unknown-unknown"
target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.str.0 = private unnamed_addr constant [24 x i8] c"Cross-compilation test:\00", align 1
@.int_fmt = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1
@.float_fmt = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.bool_true = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.bool_false = private unnamed_addr constant [7 x i8] c"cringe\00", align 1

define i32 @main() {
entry:
  %x = alloca i64, align 8
  store i64 42, i64* %x, align 8
  %str_ptr.1 = getelementptr [24 x i8], [24 x i8]* @.str.0, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.1)
  %loaded.2 = load i64, i64* %x, align 8
  %fmt_ptr.2 = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt_ptr.2, i64 %loaded.2)
  ret i32 0
}
