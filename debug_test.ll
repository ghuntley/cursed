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
  %result = alloca ptr, align 8
  store ptr @.str.0, ptr %result, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  %result_load_0 = load ptr, ptr %result, align 8
  call void @cursed_runtime_spill_string(ptr %result_load_0)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [24 x i8] c"Testing stringz.concat:\00", align 1
@.str.1 = private unnamed_addr constant [11 x i8] c"HelloWorld\00", align 1
