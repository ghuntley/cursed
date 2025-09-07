; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

define i32 @main() {
entry:
  ; Variable: result_sub
  %result_sub = alloca i64, align 8
  store i64 27, ptr %result_sub, align 8
  ; Variable: result_abs_neg
  %result_abs_neg = alloca i64, align 8
  store i64 42, ptr %result_abs_neg, align 8
  ; Variable: result_pow
  %result_pow = alloca i64, align 8
  store i64 0, ptr %result_pow, align 8
  ; Variable: result_mod
  %result_mod = alloca i64, align 8
  store i64 0, ptr %result_mod, align 8
  ; Variable: result_mul
  %result_mul = alloca i64, align 8
  store i64 56, ptr %result_mul, align 8
  ; Variable: result_add
  %result_add = alloca i64, align 8
  store i64 42, ptr %result_add, align 8
  ; Variable: result_div
  %result_div = alloca i64, align 8
  store i64 21, ptr %result_div, align 8
  ; Variable: result_abs_pos
  %result_abs_pos = alloca i64, align 8
  store i64 42, ptr %result_abs_pos, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 42)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 27)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 56)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 21)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 42)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.6)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 42)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.7)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.8)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.9)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [32 x i8] c"=== MATHZ MODULE VALIDATION ===\00", align 1
@.str.1 = private unnamed_addr constant [10 x i8] c"15 + 27 =\00", align 1
@.str.2 = private unnamed_addr constant [10 x i8] c"50 - 23 =\00", align 1
@.str.3 = private unnamed_addr constant [8 x i8] c"7 * 8 =\00", align 1
@.str.4 = private unnamed_addr constant [9 x i8] c"84 / 4 =\00", align 1
@.str.5 = private unnamed_addr constant [10 x i8] c"abs(42) =\00", align 1
@.str.6 = private unnamed_addr constant [11 x i8] c"abs(-42) =\00", align 1
@.str.7 = private unnamed_addr constant [6 x i8] c"2^8 =\00", align 1
@.str.8 = private unnamed_addr constant [9 x i8] c"17 % 5 =\00", align 1
@.str.9 = private unnamed_addr constant [34 x i8] c"=== MATHZ VALIDATION COMPLETE ===\00", align 1
