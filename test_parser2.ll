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
  ; Variable: x
  %x = alloca i64, align 8
  store i64 15, ptr %x, align 8
  ; Variable: result
  %result = alloca i64, align 8
  store i64 22, ptr %result, align 8
  ; Variable: y
  %y = alloca i64, align 8
  store i64 7, ptr %y, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [25 x i8] c"Arithmetic test complete\00", align 1
