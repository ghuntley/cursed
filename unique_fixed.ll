; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

define i32 @main() {
entry:
  ; Variable: my_number
  %my_number = alloca i64, align 8
  store i64 123, ptr %my_number, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  %my_number_load = load i64, ptr %my_number, align 8
  call void @cursed_runtime_spill_int(i64 %my_number_load)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [31 x i8] c"This is a UNIQUE test program!\00", align 1
