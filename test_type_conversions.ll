; Generated LLVM IR for CURSED program
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.str.0 = private unnamed_addr constant [30 x i8] c"Enhanced type system working!\00", align 1
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
  ; Variable: flag lit = based
  %flag = alloca i1, align 1
  store i1 true, i1* %flag, align 1
  ; Variable: float_val meal = 42.0
  %float_val = alloca double, align 8
  store double 4.2e1, double* %float_val, align 8
  ; Variable: int_val drip = 100
  %int_val = alloca i64, align 8
  store i64 100, i64* %int_val, align 8
  ; Variable: sum drip = 10 + 5
  %sum = alloca i64, align 8
  store i64 0, i64* %sum, align 8
  ; Variable: product drip = 6 * 7
  %product = alloca i64, align 8
  store i64 0, i64* %product, align 8
  ; Variable: division meal = 15.0 / 3.0
  %division = alloca double, align 8
  store double 0.0, double* %division, align 8
  ; Variable: comparison lit = x > 30
  %comparison = alloca i1, align 1
  store i1 false, i1* %comparison, align 1
  ; String literal: Enhanced type system working!
  %str_ptr.9 = getelementptr [30 x i8], [30 x i8]* @.str.0, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.9)
  ; Unknown variable: "Integer sum: %d", sum
  ; Unknown variable: "Product: %d", product
  ; Unknown variable: "Division: %f", division
  ret i32 0
}
