; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

define i32 @main() {
entry:
  ; Variable: result
  %result = alloca i64, align 8
  store i64 42, ptr %result, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 42)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [24 x i8] c"=== Arithmetic Test ===\00", align 1
@.str.1 = private unnamed_addr constant [10 x i8] c"15 + 27 =\00", align 1
@.str.2 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
