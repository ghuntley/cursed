; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions

define i32 @main() {
entry:
  ; Variable: large_float
  %large_float = alloca double, align 8
  store double 999999.999999, ptr %large_float, align 8
  ; Variable: mult_result
  %mult_result = alloca double, align 8
  store double 1.0, ptr %mult_result, align 8
  ; Variable: edge_case1
  %edge_case1 = alloca double, align 8
  store double 0.999999999999999, ptr %edge_case1, align 8
  ; Variable: big_sum
  %big_sum = alloca double, align 8
  store double 11111111101.11111, ptr %big_sum, align 8
  ; Variable: edge_case2
  %edge_case2 = alloca double, align 8
  store double 0.0000000000000009992007221626409, ptr %edge_case2, align 8
  ; Variable: big2
  %big2 = alloca double, align 8
  store double 9876543210.987654, ptr %big2, align 8
  ; Variable: big1
  %big1 = alloca double, align 8
  store double 1234567890.1234567, ptr %big1, align 8
  ; Variable: small_float
  %small_float = alloca double, align 8
  store double 0.0000001, ptr %small_float, align 8
  ; Variable: a
  %a = alloca double, align 8
  store double 0.1, ptr %a, align 8
  ; Variable: result
  %result = alloca double, align 8
  store double 0.30000000000000004, ptr %result, align 8
  ; Variable: neg_float
  %neg_float = alloca double, align 8
  store double -123.456789, ptr %neg_float, align 8
  ; Variable: div_result
  %div_result = alloca double, align 8
  store double 0.3333333333333333, ptr %div_result, align 8
  ; Variable: abs_manual
  %abs_manual = alloca double, align 8
  store double 123.456789, ptr %abs_manual, align 8
  ; Variable: b
  %b = alloca double, align 8
  store double 0.2, ptr %b, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_float(double 0.0000001)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_float(double 999999.999999)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_float(double 0.30000000000000004)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_float(double 11111111101.11111)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_float(double 0.3333333333333333)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_float(double 1.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_float(double 0.0000000000000009992007221626409)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_float(double 123.456789)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [35 x i8] c"=== Float Precision Validation ===\00", align 1
@.str.1 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
