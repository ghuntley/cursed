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
  store i64 6, ptr %result2, align 8
  ; Variable: result1
  %result1 = alloca ptr, align 8
  store ptr @.str.0, ptr %result1, align 8
  ; Variable: result3
  %result3 = alloca ptr, align 8
  store ptr @.str.1, ptr %result3, align 8
  ; Variable: result4
  %result4 = alloca ptr, align 8
  store ptr @.str.2, ptr %result4, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  %result1_load_0 = load ptr, ptr %result1, align 8
  call void @cursed_runtime_spill_string(ptr %result1_load_0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  %result2_load_1 = load i64, ptr %result2, align 8
  call void @cursed_runtime_spill_int(i64 %result2_load_1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  ; Call: vibez.spill
  %result3_load_2 = load ptr, ptr %result3, align 8
  call void @cursed_runtime_spill_string(ptr %result3_load_2)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.6)
  ; Call: vibez.spill
  %result4_load_3 = load ptr, ptr %result4, align 8
  call void @cursed_runtime_spill_string(ptr %result4_load_3)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.8)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [28 x i8] c"=== Stringz Stdlib Test ===\00", align 1
@.str.1 = private unnamed_addr constant [24 x i8] c"Testing stringz.concat:\00", align 1
@.str.2 = private unnamed_addr constant [12 x i8] c"Hello World\00", align 1
@.str.3 = private unnamed_addr constant [24 x i8] c"Testing stringz.length:\00", align 1
@.str.4 = private unnamed_addr constant [23 x i8] c"Testing stringz.upper:\00", align 1
@.str.5 = private unnamed_addr constant [7 x i8] c"CURSED\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"Testing stringz.lower:\00", align 1
@.str.7 = private unnamed_addr constant [12 x i8] c"programming\00", align 1
@.str.8 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
