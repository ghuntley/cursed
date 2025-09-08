; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @simple_test(i64 %n) {
  %result = mul i64 %n, 3
  ret i64 %result
}

define i32 @main() {
entry:
  ; Variable: result
  %result = alloca i64, align 8
  %result_call = call i64 @simple_test(i64 7)
  store i64 %result_call, ptr %result, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  %result_call_0 = call i64 @simple_test(i64 7)
  call void @cursed_runtime_spill_int(i64 %result_call_0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [29 x i8] c"=== Simple Function Test ===\00", align 1
@.str.1 = private unnamed_addr constant [17 x i8] c"simple_test(7) =\00", align 1
@.str.2 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
