; Generated LLVM IR from CURSED with COMPLETE implementation
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; Standard C Library Functions
declare i32 @printf(ptr, ...)
declare i32 @puts(ptr)

; Main function generated from CURSED program
define i32 @main() {
entry:
  ; vibez.spill("Hello from Windows-compatible CURSED! 🔥")
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; vibez.spill(42)
  call void @cursed_runtime_spill_int(i64 42)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [37 x i8] c"Hello from Windows-compatible CURSED!", align 1
