; ModuleID = '01_mixed_types'
source_filename = "01_mixed_types"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0
@.str.0 = private constant [37 x i8] c"\22=== Mixed Type Arithmetic Test ===\22\00"
@.str.1 = private constant [17 x i8] c"\22Integer value:\22\00"
@.str.2 = private constant [6 x i8] c"%lld\0A\00"
@.str.3 = private constant [15 x i8] c"\22Float value:\22\00"
@.str.4 = private constant [4 x i8] c"%g\0A\00"
@.str.5 = private constant [21 x i8] c"\22Integer + Integer:\22\00"
@.str.6 = private constant [17 x i8] c"\22Float + Float:\22\00"
@.str.7 = private constant [15 x i8] c"\22Integer * 2:\22\00"
@.str.8 = private constant [13 x i8] c"\22Float / 2:\22\00"
@.str.9 = private constant [24 x i8] c"\22=== Test Complete ===\22\00"

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

declare i32 @fflush(ptr)

declare i32 @yap(ptr)

define void @main_character() {
entry:
  %float_val = alloca double, align 8
  %int_val = alloca i64, align 8
  %puts_call = call i32 @puts(ptr @.str.0)
  store i64 10, ptr %int_val, align 4
  store double 3.500000e+00, ptr %float_val, align 8
  %puts_call1 = call i32 @puts(ptr @.str.1)
  %load_var = load i64, ptr %int_val, align 4
  %printf_call = call i32 (ptr, ...) @printf(ptr @.str.2, i64 %load_var)
  %fflush_call = call i32 @fflush(ptr null)
  %puts_call2 = call i32 @puts(ptr @.str.3)
  %load_var3 = load double, ptr %float_val, align 8
  %printf_call4 = call i32 (ptr, ...) @printf(ptr @.str.4, double %load_var3)
  %fflush_call5 = call i32 @fflush(ptr null)
  %puts_call6 = call i32 @puts(ptr @.str.5)
  %load_var7 = load i64, ptr %int_val, align 4
  %add_tmp = add i64 %load_var7, 5
  %printf_call8 = call i32 (ptr, ...) @printf(ptr @.str.2, i64 %add_tmp)
  %fflush_call9 = call i32 @fflush(ptr null)
  %puts_call10 = call i32 @puts(ptr @.str.6)
  %load_var11 = load double, ptr %float_val, align 8
  %fadd_tmp = fadd double %load_var11, 2.500000e+00
  %printf_call12 = call i32 (ptr, ...) @printf(ptr @.str.4, double %fadd_tmp)
  %fflush_call13 = call i32 @fflush(ptr null)
  %puts_call14 = call i32 @puts(ptr @.str.7)
  %load_var15 = load i64, ptr %int_val, align 4
  %mul_tmp = mul i64 %load_var15, 2
  %printf_call16 = call i32 (ptr, ...) @printf(ptr @.str.2, i64 %mul_tmp)
  %fflush_call17 = call i32 @fflush(ptr null)
  %puts_call18 = call i32 @puts(ptr @.str.8)
  %load_var19 = load double, ptr %float_val, align 8
  %fdiv_tmp = fdiv double %load_var19, 2.000000e+00
  %printf_call20 = call i32 (ptr, ...) @printf(ptr @.str.4, double %fdiv_tmp)
  %fflush_call21 = call i32 @fflush(ptr null)
  %puts_call22 = call i32 @puts(ptr @.str.9)
  ret void
}

define i32 @main() {
entry:
  call void @main_character()
  ret i32 0
}
