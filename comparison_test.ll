; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

define i32 @main() {
entry:
  ; Variable: a
  %a = alloca i64, align 8
  store i64 10, ptr %a, align 8
  ; Variable: b
  %b = alloca i64, align 8
  store i64 5, ptr %b, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 1)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [32 x i8] c"Testing comparison operators...\00", align 1
