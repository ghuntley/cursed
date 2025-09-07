; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

define i32 @main() {
entry:
  ; Variable: message
  %message = alloca ptr, align 8
  store ptr @.str.0, ptr %message, align 8
  ; Variable: number
  %number = alloca i64, align 8
  store i64 42, ptr %number, align 8
  ; Variable: flag
  %flag = alloca i64, align 8
  store i64 1, ptr %flag, align 8
  ; Call: vibez.spill
  %message_load_0 = load ptr, ptr %message, align 8
  call void @cursed_runtime_spill_string(ptr %message_load_0)
  ; Call: vibez.spill
  %number_load_1 = load i64, ptr %number, align 8
  call void @cursed_runtime_spill_int(i64 %number_load_1)
  ; Call: vibez.spill
  %flag_load_2 = load i64, ptr %flag, align 8
  call void @cursed_runtime_spill_int(i64 %flag_load_2)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [21 x i8] c"Hello, CURSED World!\00", align 1
