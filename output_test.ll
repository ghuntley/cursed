; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

define i32 @main() {
entry:
  ; Variable: value
  %value = alloca i64, align 8
  store i64 42, ptr %value, align 8
  ; Variable: flag
  %flag = alloca i1, align 1
  store i1 true, ptr %flag, align 1
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  %value_load_0 = load i64, ptr %value, align 8
  call void @cursed_runtime_spill_int(i64 %value_load_0)
  ; Call: vibez.spill
  %flag_load_1 = load i1, ptr %flag, align 1
  call void @cursed_runtime_spill_bool(i1 %flag_load_1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [26 x i8] c"Testing output comparison\00", align 1
@.str.1 = private unnamed_addr constant [14 x i8] c"Test complete\00", align 1
