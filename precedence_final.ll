; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

define i32 @main() {
entry:
  ; Variable: expr4
  %expr4 = alloca i64, align 8
  store i64 12, ptr %expr4, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 14)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 20)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 9)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  ; Call: vibez.spill
  %expr4_load = load i64, ptr %expr4, align 8
  call void @cursed_runtime_spill_int(i64 %expr4_load)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [33 x i8] c"=== Operator Precedence Test ===\00", align 1
@.str.1 = private unnamed_addr constant [24 x i8] c"2 + 3 * 4 should be 14:\00", align 1
@.str.2 = private unnamed_addr constant [26 x i8] c"(2 + 3) * 4 should be 20:\00", align 1
@.str.3 = private unnamed_addr constant [24 x i8] c"10 - 3 + 2 should be 9:\00", align 1
@.str.4 = private unnamed_addr constant [24 x i8] c"8 / 2 * 3 should be 12:\00", align 1
@.str.5 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
