; ModuleID = 'test_comprehensive_language_features'
source_filename = "test_comprehensive_language_features"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0
@.str.0 = private constant [31 x i8] c"Testing arithmetic operations:\00"
@.str.1 = private constant [4 x i8] c"%s\0A\00"
@.str.2 = private constant [10 x i8] c"Addition:\00"
@.str.3 = private constant [6 x i8] c"%lld\0A\00"
@.str.4 = private constant [13 x i8] c"Subtraction:\00"
@.str.5 = private constant [16 x i8] c"Multiplication:\00"
@.str.6 = private constant [10 x i8] c"Division:\00"
@.str.7 = private constant [17 x i8] c"Division by zero\00"
@.str.8 = private constant [26 x i8] c"Math operations completed\00"
@.str.9 = private constant [15 x i8] c"Value is large\00"
@.str.10 = private constant [16 x i8] c"Value is medium\00"
@.str.11 = private constant [15 x i8] c"Value is small\00"
@.str.12 = private constant [22 x i8] c"Testing periodt loop:\00"
@.str.13 = private constant [16 x i8] c"Loop iteration:\00"
@.str.14 = private constant [44 x i8] c"CURSED Comprehensive Language Features Test\00"
@.str.15 = private constant [42 x i8] c"=========================================\00"
@.str.16 = private constant [22 x i8] c"Variable declaration:\00"
@.str.17 = private constant [37 x i8] c"Function call test - multiply 5 * 6:\00"
@.str.18 = private constant [4 x i8] c"%d\0A\00"
@.str.19 = private constant [19 x i8] c"Control flow test:\00"
@.str.20 = private constant [34 x i8] c"All tests completed successfully!\00"

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

declare i32 @fflush(ptr)

declare i32 @yap(ptr)

define i32 @simple_multiply(i32 %x, i32 %y) {
entry:
  %y2 = alloca i32, align 4
  %x1 = alloca i32, align 4
  store i32 %x, ptr %x1, align 4
  store i32 %y, ptr %y2, align 4
  %load_var = load i32, ptr %x1, align 4
  %load_var3 = load i32, ptr %y2, align 4
  %mul_tmp = mul i32 %load_var, %load_var3
  ret i32 %mul_tmp
}

define void @test_arithmetic_operations() {
entry:
  %b = alloca i64, align 8
  %a = alloca i64, align 8
  store i64 15, ptr %a, align 4
  store i64 7, ptr %b, align 4
  %printf_string_call = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.0)
  %printf_string_call1 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.2)
  %load_var = load i64, ptr %a, align 4
  %load_var2 = load i64, ptr %b, align 4
  %add_tmp = add i64 %load_var, %load_var2
  %printf_call = call i32 (ptr, ...) @printf(ptr @.str.3, i64 %add_tmp)
  %fflush_call = call i32 @fflush(ptr null)
  %printf_string_call3 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.4)
  %load_var4 = load i64, ptr %a, align 4
  %load_var5 = load i64, ptr %b, align 4
  %sub_tmp = sub i64 %load_var4, %load_var5
  %printf_call6 = call i32 (ptr, ...) @printf(ptr @.str.3, i64 %sub_tmp)
  %fflush_call7 = call i32 @fflush(ptr null)
  %printf_string_call8 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.5)
  %load_var9 = load i64, ptr %a, align 4
  %load_var10 = load i64, ptr %b, align 4
  %mul_tmp = mul i64 %load_var9, %load_var10
  %printf_call11 = call i32 (ptr, ...) @printf(ptr @.str.3, i64 %mul_tmp)
  %fflush_call12 = call i32 @fflush(ptr null)
  %printf_string_call13 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.6)
  %load_var14 = load i64, ptr %a, align 4
  %load_var15 = load i64, ptr %b, align 4
  %div_by_zero_check = icmp eq i64 %load_var15, 0
  br i1 %div_by_zero_check, label %div_by_zero, label %div_ok

div_by_zero:                                      ; preds = %entry
  %0 = call i32 (ptr, ...) @printf(ptr @.str.7)
  call void @exit(i32 1)
  unreachable

div_ok:                                           ; preds = %entry
  %sdiv_tmp = sdiv i64 %load_var14, %load_var15
  %printf_call16 = call i32 (ptr, ...) @printf(ptr @.str.3, i64 %sdiv_tmp)
  %fflush_call17 = call i32 @fflush(ptr null)
  %printf_string_call18 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.8)
  ret void
}

declare void @exit(i32)

define void @test_control_flow(i32 %value) {
entry:
  %value1 = alloca i32, align 4
  store i32 %value, ptr %value1, align 4
  %load_var = load i32, ptr %value1, align 4
  %extend_left = sext i32 %load_var to i64
  %gt_tmp = icmp sgt i64 %extend_left, 10
  br i1 %gt_tmp, label %if_then, label %if_else

if_then:                                          ; preds = %entry
  %printf_string_call = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.9)
  br label %if_merge

if_else:                                          ; preds = %entry
  %load_var5 = load i32, ptr %value1, align 4
  %extend_left6 = sext i32 %load_var5 to i64
  %gt_tmp7 = icmp sgt i64 %extend_left6, 5
  br i1 %gt_tmp7, label %if_then2, label %if_else3

if_merge:                                         ; preds = %if_merge4, %if_then
  ret void

if_then2:                                         ; preds = %if_else
  %printf_string_call8 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.10)
  br label %if_merge4

if_else3:                                         ; preds = %if_else
  %printf_string_call9 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.11)
  br label %if_merge4

if_merge4:                                        ; preds = %if_else3, %if_then2
  br label %if_merge
}

define void @test_loops() {
entry:
  %i = alloca i64, align 8
  %printf_string_call = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.12)
  store i64 1, ptr %i, align 4
  br label %while_cond

while_cond:                                       ; preds = %while_body, %entry
  %load_var = load i64, ptr %i, align 4
  %le_tmp = icmp sle i64 %load_var, 3
  %while_bool = icmp ne i1 %le_tmp, false
  br i1 %while_bool, label %while_body, label %while_exit

while_body:                                       ; preds = %while_cond
  %printf_string_call1 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.13)
  %load_var2 = load i64, ptr %i, align 4
  %printf_call = call i32 (ptr, ...) @printf(ptr @.str.3, i64 %load_var2)
  %fflush_call = call i32 @fflush(ptr null)
  %load_var3 = load i64, ptr %i, align 4
  %add_tmp = add i64 %load_var3, 1
  store i64 %add_tmp, ptr %i, align 4
  br label %while_cond

while_exit:                                       ; preds = %while_cond
  ret void
}

define i32 @main() {
entry:
  %multiply_result = alloca i32, align 4
  %integer_var = alloca i64, align 8
  %printf_string_call = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.14)
  %printf_string_call1 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.15)
  store i64 42, ptr %integer_var, align 4
  %printf_string_call2 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.16)
  %load_var = load i64, ptr %integer_var, align 4
  %printf_call = call i32 (ptr, ...) @printf(ptr @.str.3, i64 %load_var)
  %fflush_call = call i32 @fflush(ptr null)
  %printf_string_call3 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.17)
  %call_tmp = call i32 @simple_multiply(i32 5, i32 6)
  store i32 %call_tmp, ptr %multiply_result, align 4
  %load_var4 = load i32, ptr %multiply_result, align 4
  %printf_call5 = call i32 (ptr, ...) @printf(ptr @.str.18, i32 %load_var4)
  %fflush_call6 = call i32 @fflush(ptr null)
  %printf_string_call7 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.19)
  call void @test_control_flow(i32 15)
  call void @test_control_flow(i32 7)
  call void @test_control_flow(i32 3)
  call void @test_loops()
  call void @test_arithmetic_operations()
  %printf_string_call8 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.20)
  ret i32 0
}
