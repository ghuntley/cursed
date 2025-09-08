; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @simple_add(i64 %p0, i64 %p1) {
  %result = add i64 %p0, %p1
  ret i64 %result
}


define i32 @main() {
entry:
  ; Variable: result
  %result = alloca i64, align 8
  %result_call = call i64 @simple_add(i64 5, i64 3)
  store i64 %result_call, ptr %result, align 8
  ; Call: vibez.spill
  %temp_call_0 = call i64 @simple_add(i64 5, i64 3)
  call void @cursed_runtime_spill_int(i64 %temp_call_0)
  ret i32 0
}

; String Constants
