; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

define i32 @main() {
entry:
  ; Variable: result2
  %result2 = alloca i64, align 8
  store i64 0, ptr %result2, align 8
  ; Variable: result1
  %result1 = alloca i64, align 8
  store i64 0, ptr %result1, align 8
  ; Variable: result3
  %result3 = alloca i64, align 8
  store i64 0, ptr %result3, align 8
  ; Variable: result4
  %result4 = alloca i64, align 8
  store i64 0, ptr %result4, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  %result1_load_0 = load i64, ptr %result1, align 8
  call void @cursed_runtime_spill_int(i64 %result1_load_0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: vibez.spill
  %result2_load_1 = load i64, ptr %result2, align 8
  call void @cursed_runtime_spill_int(i64 %result2_load_1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  %result3_load_2 = load i64, ptr %result3, align 8
  call void @cursed_runtime_spill_int(i64 %result3_load_2)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  ; Call: vibez.spill
  %result4_load_3 = load i64, ptr %result4, align 8
  call void @cursed_runtime_spill_int(i64 %result4_load_3)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [26 x i8] c"=== Mathz Stdlib Test ===\00", align 1
@.str.1 = private unnamed_addr constant [23 x i8] c"Testing mathz.add_two:\00", align 1
@.str.2 = private unnamed_addr constant [19 x i8] c"Testing mathz.abs:\00", align 1
@.str.3 = private unnamed_addr constant [19 x i8] c"Testing mathz.max:\00", align 1
@.str.4 = private unnamed_addr constant [19 x i8] c"Testing mathz.min:\00", align 1
@.str.5 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
