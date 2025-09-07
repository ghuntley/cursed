; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

define i32 @main() {
entry:
  ; Variable: test_var
  %test_var = alloca ptr, align 8
  store ptr @.str.0, ptr %test_var, align 8
  ; Call: vibez.spill
  %test_var_load_0 = load ptr, ptr %test_var, align 8
  call void @cursed_runtime_spill_string(ptr %test_var_load_0)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [11 x i8] c"HelloWorld\00", align 1
