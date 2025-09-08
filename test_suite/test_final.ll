; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @multiply_numbers(i64 %x, i64 %y) {
  %result = mul i64 %x, %y
  ret i64 %result
}
define i64 @subtract_and_double(i64 %a, i64 %b) {
  %diff = sub i64 %a, %b
  %result = mul i64 %diff, 2
  ret i64 %result
}

define i32 @main() {
entry:
  ; Variable: result2
  %result2 = alloca i64, align 8
  %result2_call = call i64 @subtract_and_double(i64 20, i64 8)
  store i64 %result2_call, ptr %result2, align 8
  ; Variable: result1
  %result1 = alloca i64, align 8
  %result1_call = call i64 @multiply_numbers(i64 6, i64 7)
  store i64 %result1_call, ptr %result1, align 8
  ; Variable: diff
  %diff = alloca i64, align 8
  store i64 0, ptr %diff, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  %result1_call_0 = call i64 @multiply_numbers(i64 6, i64 7)
  call void @cursed_runtime_spill_int(i64 %result1_call_0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: vibez.spill
  %result2_call_1 = call i64 @subtract_and_double(i64 20, i64 8)
  call void @cursed_runtime_spill_int(i64 %result2_call_1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [31 x i8] c"=== Multiple Function Test ===\00", align 1
@.str.1 = private unnamed_addr constant [25 x i8] c"multiply_numbers(6, 7) =\00", align 1
@.str.2 = private unnamed_addr constant [29 x i8] c"subtract_and_double(20, 8) =\00", align 1
@.str.3 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
