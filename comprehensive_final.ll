; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

define i32 @main() {
entry:
  ; Variable: positive
  %positive = alloca i64, align 8
  store i64 42, ptr %positive, align 8
  ; Variable: negative
  %negative = alloca i64, align 8
  store i64 -17, ptr %negative, align 8
  ; Variable: zero
  %zero = alloca i64, align 8
  store i64 0, ptr %zero, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  %positive_load = load i64, ptr %positive, align 8
  call void @cursed_runtime_spill_int(i64 %positive_load)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: vibez.spill
  %negative_load = load i64, ptr %negative, align 8
  call void @cursed_runtime_spill_int(i64 %negative_load)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  %zero_load = load i64, ptr %zero, align 8
  call void @cursed_runtime_spill_int(i64 %zero_load)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [34 x i8] c"=== CURSED Comprehensive Test ===\00", align 1
@.str.1 = private unnamed_addr constant [17 x i8] c"Positive number:\00", align 1
@.str.2 = private unnamed_addr constant [17 x i8] c"Negative number:\00", align 1
@.str.3 = private unnamed_addr constant [6 x i8] c"Zero:\00", align 1
@.str.4 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
