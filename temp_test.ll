; ModuleID = 'feature_arithmetic_operations'
source_filename = "feature_arithmetic_operations"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0
@.str.0 = private constant [37 x i8] c"\22=== Arithmetic Operations Test ===\22\00"
@.str.1 = private constant [21 x i8] c"\22Addition (10 + 3):\22\00"
@.str.2 = private constant [6 x i8] c"%lld\0A\00"
@.str.3 = private constant [24 x i8] c"\22Subtraction (10 - 3):\22\00"
@.str.4 = private constant [27 x i8] c"\22Multiplication (10 * 3):\22\00"
@.str.5 = private constant [21 x i8] c"\22Division (10 / 3):\22\00"
@.str.6 = private constant [6 x i8] c"%.6f\0A\00"
@.str.7 = private constant [31 x i8] c"\22Precedence test (2 + 3 * 4):\22\00"
@.str.8 = private constant [34 x i8] c"\22Parentheses test ((2 + 3) * 4):\22\00"
@.str.9 = private constant [43 x i8] c"\22Complex expression (15 - 2 * 3 + 8 / 4):\22\00"
@.str.10 = private constant [35 x i8] c"\22=== Arithmetic Test Complete ===\22\00"

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

declare i32 @fflush(ptr)

declare i32 @yap(ptr)

define void @main_character() {
entry:
  %complex = alloca double, align 8
  %result2 = alloca i64, align 8
  %result1 = alloca i64, align 8
  %b = alloca i64, align 8
  %a = alloca i64, align 8
  %puts_call = call i32 @puts(ptr @.str.0)
  store i64 10, ptr %a, align 4
  store i64 3, ptr %b, align 4
  %puts_call1 = call i32 @puts(ptr @.str.1)
  %load_var = load i64, ptr %a, align 4
  %load_var2 = load i64, ptr %b, align 4
  %add_tmp = add i64 %load_var, %load_var2
  %printf_call = call i32 (ptr, ...) @printf(ptr @.str.2, i64 %add_tmp)
  %fflush_call = call i32 @fflush(ptr null)
  %puts_call3 = call i32 @puts(ptr @.str.3)
  %load_var4 = load i64, ptr %a, align 4
  %load_var5 = load i64, ptr %b, align 4
  %sub_tmp = sub i64 %load_var4, %load_var5
  %printf_call6 = call i32 (ptr, ...) @printf(ptr @.str.2, i64 %sub_tmp)
  %fflush_call7 = call i32 @fflush(ptr null)
  %puts_call8 = call i32 @puts(ptr @.str.4)
  %load_var9 = load i64, ptr %a, align 4
  %load_var10 = load i64, ptr %b, align 4
  %mul_tmp = mul i64 %load_var9, %load_var10
  %printf_call11 = call i32 (ptr, ...) @printf(ptr @.str.2, i64 %mul_tmp)
  %fflush_call12 = call i32 @fflush(ptr null)
  %puts_call13 = call i32 @puts(ptr @.str.5)
  %load_var14 = load i64, ptr %a, align 4
  %load_var15 = load i64, ptr %b, align 4
  %int_to_double_left = sitofp i64 %load_var14 to double
  %int_to_double_right = sitofp i64 %load_var15 to double
  %fdiv_tmp = fdiv double %int_to_double_left, %int_to_double_right
  %printf_call16 = call i32 (ptr, ...) @printf(ptr @.str.6, double %fdiv_tmp)
  %fflush_call17 = call i32 @fflush(ptr null)
  store i64 14, ptr %result1, align 4
  %puts_call18 = call i32 @puts(ptr @.str.7)
  %load_var19 = load i64, ptr %result1, align 4
  %printf_call20 = call i32 (ptr, ...) @printf(ptr @.str.2, i64 %load_var19)
  %fflush_call21 = call i32 @fflush(ptr null)
  store i64 20, ptr %result2, align 4
  %puts_call22 = call i32 @puts(ptr @.str.8)
  %load_var23 = load i64, ptr %result2, align 4
  %printf_call24 = call i32 (ptr, ...) @printf(ptr @.str.2, i64 %load_var23)
  %fflush_call25 = call i32 @fflush(ptr null)
  store double 1.100000e+01, ptr %complex, align 8
  %puts_call26 = call i32 @puts(ptr @.str.9)
  %load_var27 = load double, ptr %complex, align 8
  %printf_call28 = call i32 (ptr, ...) @printf(ptr @.str.6, double %load_var27)
  %fflush_call29 = call i32 @fflush(ptr null)
  %puts_call30 = call i32 @puts(ptr @.str.10)
  ret void
}

define i32 @main() {
entry:
  call void @main_character()
  ret i32 0
}
