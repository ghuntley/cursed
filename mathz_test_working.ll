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
  call void @cursed_runtime_spill_int(i64 42)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 10)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 5)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 7)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [27 x i8] c"Testing mathz functions...\00", align 1
