; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

define i32 @main() {
entry:
  ; Variable: result1
  %result1 = alloca i64, align 8
  store i64 15, ptr %result1, align 8
  ; Variable: result2
  %result2 = alloca i64, align 8
  store i64 0, ptr %result2, align 8
  ; Variable: x
  %x = alloca i64, align 8
  store i64 15, ptr %x, align 8
  ; Variable: a
  %a = alloca i64, align 8
  store i64 2, ptr %a, align 8
  ; Variable: b
  %b = alloca i64, align 8
  store i64 3, ptr %b, align 8
  ; Variable: c
  %c = alloca i64, align 8
  store i64 4, ptr %c, align 8
  ; Variable: final
  %final = alloca i64, align 8
  store i64 12, ptr %final, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  %result1_load_0 = load i64, ptr %result1, align 8
  call void @cursed_runtime_spill_int(i64 %result1_load_0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: vibez.spill
  %result2_load_1 = load i64, ptr %result2, align 8
  call void @cursed_runtime_spill_int(i64 %result2_load_1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  ; Call: vibez.spill
  %final_load_2 = load i64, ptr %final, align 8
  call void @cursed_runtime_spill_int(i64 %final_load_2)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [31 x i8] c"=== Nested Operations Test ===\00", align 1
@.str.1 = private unnamed_addr constant [20 x i8] c"Complex arithmetic:\00", align 1
@.str.2 = private unnamed_addr constant [23 x i8] c"Nested function calls:\00", align 1
@.str.3 = private unnamed_addr constant [21 x i8] c"Nested conditionals:\00", align 1
@.str.4 = private unnamed_addr constant [21 x i8] c"Multiple operations:\00", align 1
@.str.5 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
