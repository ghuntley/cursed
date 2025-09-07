; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

define i32 @main() {
entry:
  ; Variable: number
  %number = alloca i64, align 8
  store i64 42, ptr %number, align 8
  ; Variable: pi
  %pi = alloca double, align 8
  store double 3.14159, ptr %pi, align 8
  ; Variable: name
  %name = alloca ptr, align 8
  store ptr @.str.2, ptr %name, align 8
  ; Variable: flag
  %flag = alloca i64, align 8
  store i64 1, ptr %flag, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: vibez.spill
  %number_load = load i64, ptr %number, align 8
  call void @cursed_runtime_spill_int(i64 %number_load)
  ; Call: vibez.spill
  %pi_load = load double, ptr %pi, align 8
  call void @cursed_runtime_spill_float(double %pi_load)
  ; Call: vibez.spill
  %name_load = load ptr, ptr %name, align 8
  call void @cursed_runtime_spill_string(ptr %name_load)
  ; Call: vibez.spill
  %flag_load = load i64, ptr %flag, align 8
  call void @cursed_runtime_spill_int(i64 %flag_load)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 50)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_float(double 6.28318)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 14)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 5)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_float(double 1.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.6)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 25)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 50)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 5)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 42)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.7)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  ; Call: vibez.spill
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.8)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [42 x i8] c"=== CURSED Complete Language Showcase ===\00", align 1
@.str.1 = private unnamed_addr constant [16 x i8] c"CURSED Language\00", align 1
@.str.2 = private unnamed_addr constant [13 x i8] c"Basic Types:\00", align 1
@.str.3 = private unnamed_addr constant [12 x i8] c"Arithmetic:\00", align 1
@.str.4 = private unnamed_addr constant [13 x i8] c"Comparisons:\00", align 1
@.str.5 = private unnamed_addr constant [9 x i8] c"Logical:\00", align 1
@.str.6 = private unnamed_addr constant [17 x i8] c"Mathz Functions:\00", align 1
@.str.7 = private unnamed_addr constant [21 x i8] c"Complex Expressions:\00", align 1
@.str.8 = private unnamed_addr constant [50 x i8] c"=== Showcase Complete - All Features Working! ===\00", align 1
