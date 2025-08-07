; Generated LLVM IR for CURSED program
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.str.0 = private unnamed_addr constant [30 x i8] c"CURSED Debug Information Test\00", align 1
@.str.1 = private unnamed_addr constant [46 x i8] c"This program demonstrates DWARF debug support\00", align 1
@.str.2 = private unnamed_addr constant [33 x i8] c"Variables created for debugging:\00", align 1
@.int_fmt = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1
@.float_fmt = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.bool_true = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.bool_false = private unnamed_addr constant [7 x i8] c"cringe\00", align 1

define i32 @main() {
entry:
  ; String literal: CURSED Debug Information Test
  %str_ptr.0 = getelementptr [30 x i8], [30 x i8]* @.str.0, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.0)
  ; String literal: This program demonstrates DWARF debug support
  %str_ptr.1 = getelementptr [46 x i8], [46 x i8]* @.str.1, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.1)
  ; Variable: drip_value drip = 42
  %drip_value = alloca i64, align 8
  store i64 42, i64* %drip_value, align 8
  ; Variable: tea_value tea = "Hello Debug World!"
  %tea_value = alloca [19 x i8], align 1
  %tea_value.ptr.0 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 0
  store i8 72, i8* %tea_value.ptr.0, align 1
  %tea_value.ptr.1 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 1
  store i8 101, i8* %tea_value.ptr.1, align 1
  %tea_value.ptr.2 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 2
  store i8 108, i8* %tea_value.ptr.2, align 1
  %tea_value.ptr.3 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 3
  store i8 108, i8* %tea_value.ptr.3, align 1
  %tea_value.ptr.4 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 4
  store i8 111, i8* %tea_value.ptr.4, align 1
  %tea_value.ptr.5 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 5
  store i8 32, i8* %tea_value.ptr.5, align 1
  %tea_value.ptr.6 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 6
  store i8 68, i8* %tea_value.ptr.6, align 1
  %tea_value.ptr.7 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 7
  store i8 101, i8* %tea_value.ptr.7, align 1
  %tea_value.ptr.8 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 8
  store i8 98, i8* %tea_value.ptr.8, align 1
  %tea_value.ptr.9 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 9
  store i8 117, i8* %tea_value.ptr.9, align 1
  %tea_value.ptr.10 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 10
  store i8 103, i8* %tea_value.ptr.10, align 1
  %tea_value.ptr.11 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 11
  store i8 32, i8* %tea_value.ptr.11, align 1
  %tea_value.ptr.12 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 12
  store i8 87, i8* %tea_value.ptr.12, align 1
  %tea_value.ptr.13 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 13
  store i8 111, i8* %tea_value.ptr.13, align 1
  %tea_value.ptr.14 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 14
  store i8 114, i8* %tea_value.ptr.14, align 1
  %tea_value.ptr.15 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 15
  store i8 108, i8* %tea_value.ptr.15, align 1
  %tea_value.ptr.16 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 16
  store i8 100, i8* %tea_value.ptr.16, align 1
  %tea_value.ptr.17 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 17
  store i8 33, i8* %tea_value.ptr.17, align 1
  %tea_value.ptr.18 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 18
  store i8 0, i8* %tea_value.ptr.18, align 1
  ; Variable: lit_value lit = based
  %lit_value = alloca i1, align 1
  store i1 true, i1* %lit_value, align 1
  ; Variable: meal_value meal = 3.14159
  %meal_value = alloca double, align 8
  store double 3.14159e0, double* %meal_value, align 8
  ; String literal: Variables created for debugging:
  %str_ptr.6 = getelementptr [33 x i8], [33 x i8]* @.str.2, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.6)
  ; Variable: drip_value
  %loaded.7 = load i64, i64* %drip_value, align 8
  %fmt_ptr.7 = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt_ptr.7, i64 %loaded.7)
  ; Variable: tea_value
  %str_ptr.8 = getelementptr [19 x i8], [19 x i8]* %tea_value, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.8)
  ; Variable: lit_value
  %loaded.9 = load i1, i1* %lit_value, align 1
  %select.9 = select i1 %loaded.9, i8* getelementptr ([6 x i8], [6 x i8]* @.bool_true, i32 0, i32 0), i8* getelementptr ([7 x i8], [7 x i8]* @.bool_false, i32 0, i32 0)
  call i32 @puts(i8* %select.9)
  ; Variable: meal_value
  %loaded.10 = load double, double* %meal_value, align 8
  %fmt_ptr.10 = getelementptr [4 x i8], [4 x i8]* @.float_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt_ptr.10, double %loaded.10)
  ret i32 0
}
