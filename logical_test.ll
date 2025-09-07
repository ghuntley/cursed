; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

define i32 @main() {
entry:
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [29 x i8] c"Testing logical operators...\00", align 1
