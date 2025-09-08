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
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 5)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 -2)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 -6)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 -6)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 3000)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 999998)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [35 x i8] c"=== Arithmetic Edge Cases Test ===\00", align 1
@.str.1 = private unnamed_addr constant [17 x i8] c"Zero operations:\00", align 1
@.str.2 = private unnamed_addr constant [18 x i8] c"Negative numbers:\00", align 1
@.str.3 = private unnamed_addr constant [15 x i8] c"Large numbers:\00", align 1
@.str.4 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
