; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @test_control_flow(i64 %p0, i64 %p1) {
  %result = add i64 %p0, %p1
  ret i64 %result
}

define i64 @test_arithmetic_operations(i64 %p0, i64 %p1) {
  %result = add i64 %p0, %p1
  ret i64 %result
}

define i64 @test_loops(i64 %p0, i64 %p1) {
  %result = add i64 %p0, %p1
  ret i64 %result
}

define i64 @simple_multiply(i64 %p0, i64 %p1) {
  %result = mul i64 %p0, %p1
  ret i64 %result
}


define i32 @main() {
entry:
  ; Variable: b
  %b = alloca i64, align 8
  store i64 7, ptr %b, align 8
  ; Variable: a
  %a = alloca i64, align 8
  store i64 15, ptr %a, align 8
  ; Variable: i
  %i = alloca i64, align 8
  store i64 4, ptr %i, align 8
  ; Variable: multiply_result
  %multiply_result = alloca i64, align 8
  %multiply_result_call = call i64 @simple_multiply(i64 5, i64 6)
  store i64 %multiply_result_call, ptr %multiply_result, align 8
  ; Variable: integer_var
  %integer_var = alloca i64, align 8
  store i64 42, ptr %integer_var, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 22)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 8)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 105)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 2)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.6)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.7)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.8)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.8)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 2)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.8)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 3)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.11)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.12)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.13)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 42)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.14)
  ; Call: vibez.spill
  %temp_call_0 = call i64 @simple_multiply(i64 5, i64 6)
  call void @cursed_runtime_spill_int(i64 %temp_call_0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.15)
  ; Call: test_control_flow
  ; Call: test_control_flow
  ; Call: test_control_flow
  ; Call: test_loops
  ; Call: test_arithmetic_operations
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.16)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [31 x i8] c"Testing arithmetic operations:\00", align 1
@.str.1 = private unnamed_addr constant [10 x i8] c"Addition:\00", align 1
@.str.2 = private unnamed_addr constant [13 x i8] c"Subtraction:\00", align 1
@.str.3 = private unnamed_addr constant [16 x i8] c"Multiplication:\00", align 1
@.str.4 = private unnamed_addr constant [10 x i8] c"Division:\00", align 1
@.str.5 = private unnamed_addr constant [26 x i8] c"Math operations completed\00", align 1
@.str.6 = private unnamed_addr constant [15 x i8] c"Value is small\00", align 1
@.str.7 = private unnamed_addr constant [22 x i8] c"Testing periodt loop:\00", align 1
@.str.8 = private unnamed_addr constant [16 x i8] c"Loop iteration:\00", align 1
@.str.9 = private unnamed_addr constant [16 x i8] c"Loop iteration:\00", align 1
@.str.10 = private unnamed_addr constant [16 x i8] c"Loop iteration:\00", align 1
@.str.11 = private unnamed_addr constant [44 x i8] c"CURSED Comprehensive Language Features Test\00", align 1
@.str.12 = private unnamed_addr constant [42 x i8] c"=========================================\00", align 1
@.str.13 = private unnamed_addr constant [22 x i8] c"Variable declaration:\00", align 1
@.str.14 = private unnamed_addr constant [37 x i8] c"Function call test - multiply 5 * 6:\00", align 1
@.str.15 = private unnamed_addr constant [19 x i8] c"Control flow test:\00", align 1
@.str.16 = private unnamed_addr constant [34 x i8] c"All tests completed successfully!\00", align 1
