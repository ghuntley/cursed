; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @pure_computation() {
  ret i64 0
}


define i32 @main() {
entry:
  ; Variable: sum
  %sum = alloca i64, align 8
  store i64 15, ptr %sum, align 8
  ; Variable: i
  %i = alloca i64, align 8
  store i64 6, ptr %i, align 8
  ; Variable: result
  %result = alloca i64, align 8
  %result_call = call i64 @pure_computation()
  store i64 %result_call, ptr %result, align 8
  ; Call: pure_computation
  ; Standalone call: pure_computation
  call i64 @pure_computation()
  ret i32 0
}

; String Constants
@newline_str = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@space_str = private unnamed_addr constant [2 x i8] c" \00", align 1
@hello_comma_str = private unnamed_addr constant [7 x i8] c"Hello,\00", align 1
