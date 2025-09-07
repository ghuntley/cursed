; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

define i32 @main() {
entry:
  ; Variable: x
  %x = alloca i64, align 8
  store i64 42, ptr %x, align 8
  ; Variable: name
  %name = alloca ptr, align 8
  store ptr @.str.1, ptr %name, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  %x_load = load i64, ptr %x, align 8
  call void @cursed_runtime_spill_int(i64 %x_load)
  ; Call: vibez.spill
  %name_load = load i64, ptr %name, align 8
  call void @cursed_runtime_spill_int(i64 %name_load)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [7 x i8] c"CURSED\00", align 1
@.str.1 = private unnamed_addr constant [32 x i8] c"Testing extended LLVM pipeline!\00", align 1
