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


define i32 @main() {
entry:
  ; Variable: result
  %result = alloca i64, align 8
  %result_call = call i64 @add_numbers(i64 10, i64 5)
  store i64 %result_call, ptr %result, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  %temp_call_0 = call i64 @add_numbers(i64 10, i64 5)
  call void @cursed_runtime_spill_int(i64 %temp_call_0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [22 x i8] c"=== Function Test ===\00", align 1
@.str.1 = private unnamed_addr constant [21 x i8] c"add_numbers(10, 5) =\00", align 1
@.str.2 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
