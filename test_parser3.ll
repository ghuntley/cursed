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
  ; Variable: result2
  %result2 = alloca i64, align 8
  store i64 7, ptr %result2, align 8
  ; Variable: bool_result2
  %bool_result2 = alloca i64, align 8
  store i64 0, ptr %bool_result2, align 8
  ; Variable: result1
  %result1 = alloca i64, align 8
  store i64 14, ptr %result1, align 8
  ; Variable: y
  %y = alloca i64, align 8
  store i64 10, ptr %y, align 8
  ; Variable: comp_result
  %comp_result = alloca i64, align 8
  store i64 0, ptr %comp_result, align 8
  ; Variable: x
  %x = alloca i64, align 8
  store i64 5, ptr %x, align 8
  ; Variable: a
  %a = alloca i1, align 1
  store i1 true, ptr %a, align 1
  ; Variable: result4
  %result4 = alloca i64, align 8
  store i64 26, ptr %result4, align 8
  ; Variable: z
  %z = alloca i64, align 8
  store i64 3, ptr %z, align 8
  ; Variable: b
  %b = alloca i1, align 1
  store i1 false, ptr %b, align 1
  ; Variable: result3
  %result3 = alloca i64, align 8
  store i64 20, ptr %result3, align 8
  ; Variable: c
  %c = alloca i1, align 1
  store i1 true, ptr %c, align 1
  ; Variable: bool_result1
  %bool_result1 = alloca i64, align 8
  store i64 0, ptr %bool_result1, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  call void @cursed_runtime_spill_int(i64 14)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  call void @cursed_runtime_spill_int(i64 7)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  call void @cursed_runtime_spill_int(i64 20)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  call void @cursed_runtime_spill_int(i64 26)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  call void @cursed_runtime_spill_int(i64 0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  call void @cursed_runtime_spill_int(i64 0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.6)
  call void @cursed_runtime_spill_int(i64 0)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [15 x i8] c"2 + 3 * 4 = {}\00", align 1
@.str.1 = private unnamed_addr constant [16 x i8] c"20 / 4 + 2 = {}\00", align 1
@.str.2 = private unnamed_addr constant [17 x i8] c"(2 + 3) * 4 = {}\00", align 1
@.str.3 = private unnamed_addr constant [19 x i8] c"2 * 3 + 4 * 5 = {}\00", align 1
@.str.4 = private unnamed_addr constant [27 x i8] c"true || false && true = {}\00", align 1
@.str.5 = private unnamed_addr constant [29 x i8] c"(true || false) && true = {}\00", align 1
@.str.6 = private unnamed_addr constant [20 x i8] c"5 + 3 > 10 - 2 = {}\00", align 1
