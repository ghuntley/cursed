; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @double_value(i64 %p0) {
  %result = mul i64 %p0, 2
  ret i64 %result
}

define i64 @add_three(i64 %p0, i64 %p1, i64 %p2) {
  %sum = add i64 %p0, %p1
  %sum1 = add i64 %sum, %p2
  ret i64 %sum1
}

define i64 @simple_function(i64 %p0, i64 %p1) {
  %result = add i64 %p0, %p1
  ret i64 %result
}

define i64 @compare_values(i64 %p0, i64 %p1) {
  %result = add i64 %p0, %p1
  ret i64 %result
}


define i32 @main() {
entry:
  ; Variable: result2
  %result2 = alloca i64, align 8
  %result2_call = call i64 @add_three(i64 5, i64 10, i64 15)
  store i64 %result2_call, ptr %result2, align 8
  ; Variable: result1
  %result1 = alloca i64, align 8
  %result1_call = call i64 @double_value(i64 21)
  store i64 %result1_call, ptr %result1, align 8
  ; Variable: is_greater
  %is_greater = alloca i64, align 8
  %is_greater_call = call i64 @compare_values(i64 10, i64 5)
  store i64 %is_greater_call, ptr %is_greater, align 8
  ; Variable: x
  %x = alloca i64, align 8
  store i64 42, ptr %x, align 8
  ; Variable: nested_result
  %nested_result = alloca i64, align 8
  %nested_result_call = call i64 @double_value(i64 0)
  store i64 %nested_result_call, ptr %nested_result, align 8
  ; Call: simple_function
  ; Call: vibez.spill
  %temp_call_0 = call i64 @double_value(i64 21)
  call void @cursed_runtime_spill_int(i64 %temp_call_0)
  ; Call: vibez.spill
  %temp_call_1 = call i64 @add_three(i64 5, i64 10, i64 15)
  call void @cursed_runtime_spill_int(i64 %temp_call_1)
  ; Call: vibez.spill
  %temp_call_2 = call i64 @compare_values(i64 10, i64 5)
  call void @cursed_runtime_spill_int(i64 %temp_call_2)
  ; Call: vibez.spill
  %temp_call_3 = call i64 @double_value(i64 0)
  call void @cursed_runtime_spill_int(i64 %temp_call_3)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 42)
  ret i32 0
}

; String Constants
