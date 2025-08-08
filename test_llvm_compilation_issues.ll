; Generated LLVM IR for CURSED program
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.str.0 = private unnamed_addr constant [14 x i8] c"Factorial(5):\00", align 1
@.int_fmt = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1
@.float_fmt = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.bool_true = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.bool_false = private unnamed_addr constant [7 x i8] c"cringe\00", align 1

define i64 @factorial(i64 %n) {
entry:
  %cond = icmp sle i64 %n, 1
  br i1 %cond, label %base_case, label %recursive_case

base_case:
  ret i64 1

recursive_case:
  %n_minus_1 = sub i64 %n, 1
  %recursive_result = call i64 @factorial(i64 %n_minus_1)
  %result = mul i64 %n, %recursive_result
  ret i64 %result
}

define i64 @complex_expr_test() {
entry:
  ret i64 0
}

define i64 @array_length_test() {
entry:
  ret i64 0
}

define i64 @struct_field_test() {
entry:
  ret i64 0
}

define i64 @fib(i64 %n) {
entry:
  %cond0 = icmp sle i64 %n, 0
  br i1 %cond0, label %case_zero, label %check_one

case_zero:
  ret i64 0

check_one:
  %cond1 = icmp eq i64 %n, 1
  br i1 %cond1, label %case_one, label %recursive_case

case_one:
  ret i64 1

recursive_case:
  %n_minus_1 = sub i64 %n, 1
  %n_minus_2 = sub i64 %n, 2
  %fib_n_1 = call i64 @fib(i64 %n_minus_1)
  %fib_n_2 = call i64 @fib(i64 %n_minus_2)
  %result = add i64 %fib_n_1, %fib_n_2
  ret i64 %result
}

define i64 @array_operations_test() {
entry:
  ret i64 0
}

define i32 @main() {
entry:
  ret i32 0
}

define i32 @main() {
entry:
  %expr0 = alloca i64, align 8
  store i64 0, i64* %expr0, align 8
  %a = alloca i64, align 8
  store i64 5, i64* %a, align 8
  %b = alloca i64, align 8
  store i64 3, i64* %b, align 8
  %result = alloca i64, align 8
  %add_result.3 = add i64 0, 0
  store i64 %add_result.3, i64* %result, align 8
  %count = alloca i64, align 8
  store i64 0, i64* %count, align 8
  %sum = alloca i64, align 8
  %add_result.8 = add i64 0, 0
  store i64 %add_result.8, i64* %sum, align 8
  %expr10 = alloca i64, align 8
  store i64 0, i64* %expr10, align 8
  %total = alloca i64, align 8
  store i64 0, i64* %total, align 8
  %i = alloca i64, align 8
  store i64 0, i64* %i, align 8
  %count = alloca i64, align 8
  store i64 5, i64* %count, align 8  ; placeholder array length
  %temp15 = alloca i64, align 8
  %add_result = add i64 5, 3  ; simplified calculation
  %mul_result = mul i64 %add_result, 2
  store i64 %mul_result, i64* %temp15, align 8
  %expr16 = alloca i64, align 8
  store i64 0, i64* %expr16, align 8
  %fact_result = alloca i64, align 8
  %call_result.17 = call i64 @factorial(i64 5)
  store i64 %call_result.17, i64* %fact_result, align 8
  %str_ptr.18 = getelementptr [14 x i8], [14 x i8]* @.str.0, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.18)
  %loaded.19 = load i64, i64* %fact_result, align 8
  %fmt_ptr.19 = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt_ptr.19, i64 %loaded.19)
  %expr_result = alloca i64, align 8
  %call_result.20 = call i64 @complex_expr_test()
  store i64 %call_result.20, i64* %expr_result, align 8
  %loaded.21 = load i64, i64* %expr_result, align 8
  %fmt_ptr.21 = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt_ptr.21, i64 %loaded.21)
  %length_result = alloca i64, align 8
  %call_result.22 = call i64 @array_length_test()
  store i64 %call_result.22, i64* %length_result, align 8
  %loaded.23 = load i64, i64* %length_result, align 8
  %fmt_ptr.23 = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt_ptr.23, i64 %loaded.23)
  %struct_result = alloca i64, align 8
  %call_result.24 = call i64 @struct_field_test()
  store i64 %call_result.24, i64* %struct_result, align 8
  %loaded.25 = load i64, i64* %struct_result, align 8
  %fmt_ptr.25 = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt_ptr.25, i64 %loaded.25)
  %fib_result = alloca i64, align 8
  %call_result.26 = call i64 @fib(i64 6)
  store i64 %call_result.26, i64* %fib_result, align 8
  %loaded.27 = load i64, i64* %fib_result, align 8
  %fmt_ptr.27 = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt_ptr.27, i64 %loaded.27)
  %array_result = alloca i64, align 8
  %call_result.28 = call i64 @array_operations_test()
  store i64 %call_result.28, i64* %array_result, align 8
  %loaded.29 = load i64, i64* %array_result, align 8
  %fmt_ptr.29 = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt_ptr.29, i64 %loaded.29)
  ret i32 0
}
