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
  ; Variable: ptr
  %ptr = alloca i64, align 8
  store i64 0, ptr %ptr, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  %value_load = load i64, ptr %value, align 8
  call void @cursed_runtime_spill_int(i64 %value_load)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: vibez.spill
  %ptr_load = load i64, ptr %ptr, align 8
  call void @cursed_runtime_spill_int(i64 %ptr_load)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [37 x i8] c"Testing proper pointer operations...\00", align 1
@.str.1 = private unnamed_addr constant [16 x i8] c"Original value:\00", align 1
@.str.2 = private unnamed_addr constant [17 x i8] c"Pointer address:\00", align 1
@.str.3 = private unnamed_addr constant [20 x i8] c"Dereferenced value:\00", align 1
