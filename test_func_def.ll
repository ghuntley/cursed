; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @add_numbers(i64 %a, i64 %b) {
  %result = add i64 %a, %b
  ret i64 %result
}
define i64 @calculate_average(i64 %p0, i64 %p1) {
  %result = add i64 %p0, %p1
  ret i64 %result
}

define i64 @print_message(i64 %p0, i64 %p1) {
  %result = add i64 %p0, %p1
  ret i64 %result
}

define i64 @multiply_three(i64 %p0, i64 %p1, i64 %p2) {
  %sum = add i64 %p0, %p1
  %sum1 = add i64 %sum, %p2
  ret i64 %sum1
}


define i32 @main() {
entry:
  ; Variable: sum
  %sum = alloca i64, align 8
  store i64 0, ptr %sum, align 8
  ; Variable: result1
  %result1 = alloca i64, align 8
  %result1_call = call i64 @add_numbers(i64 10, i64 5)
  store i64 %result1_call, ptr %result1, align 8
  ; Variable: result2
  %result2 = alloca i64, align 8
  %result2_call = call i64 @multiply_three(i64 2, i64 3, i64 4)
  store i64 %result2_call, ptr %result2, align 8
  ; Variable: avg
  %avg = alloca i64, align 8
  %avg_call = call i64 @calculate_average(i64 10, i64 20, i64 30)
  store i64 %avg_call, ptr %avg, align 8
  ; Call: vibez.spill
  ; Variable not available (unsupported type)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  %temp_call_0 = call i64 @add_numbers(i64 10, i64 5)
  call void @cursed_runtime_spill_int(i64 %temp_call_0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: vibez.spill
  %temp_call_1 = call i64 @multiply_three(i64 2, i64 3, i64 4)
  call void @cursed_runtime_spill_int(i64 %temp_call_1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: print_message
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  ; Call: vibez.spill
  %temp_call_2 = call i64 @calculate_average(i64 10, i64 20, i64 30)
  call void @cursed_runtime_spill_int(i64 %temp_call_2)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [34 x i8] c"=== Function Definitions Test ===\00", align 1
@.str.1 = private unnamed_addr constant [9 x i8] c"10 + 5 =\00", align 1
@.str.2 = private unnamed_addr constant [12 x i8] c"2 * 3 * 4 =\00", align 1
@.str.3 = private unnamed_addr constant [23 x i8] c"Message from function:\00", align 1
@.str.4 = private unnamed_addr constant [23 x i8] c"Average of 10, 20, 30:\00", align 1
@.str.5 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
