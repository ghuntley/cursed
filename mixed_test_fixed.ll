; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

define i32 @main() {
entry:
  ; Variable: int_val
  %int_val = alloca i64, align 8
  store i64 10, ptr %int_val, align 8
  ; Variable: float_val
  %float_val = alloca i64, align 8
  store i64 3, ptr %float_val, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  %int_val_load = load i64, ptr %int_val, align 8
  call void @cursed_runtime_spill_int(i64 %int_val_load)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: vibez.spill
  %float_val_load = load i64, ptr %float_val, align 8
  call void @cursed_runtime_spill_int(i64 %float_val_load)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 15)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 5)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 20)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.6)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.7)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [35 x i8] c"=== Mixed Type Arithmetic Test ===\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"Integer value:\00", align 1
@.str.2 = private unnamed_addr constant [13 x i8] c"Float value:\00", align 1
@.str.3 = private unnamed_addr constant [19 x i8] c"Integer + Integer:\00", align 1
@.str.4 = private unnamed_addr constant [15 x i8] c"Float + Float:\00", align 1
@.str.5 = private unnamed_addr constant [13 x i8] c"Integer * 2:\00", align 1
@.str.6 = private unnamed_addr constant [11 x i8] c"Float / 2:\00", align 1
@.str.7 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
