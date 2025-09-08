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
  ; Variable: pi
  %pi = alloca double, align 8
  store double 3.14159, ptr %pi, align 8
  ; Variable: a
  %a = alloca i64, align 8
  store i64 10, ptr %a, align 8
  ; Variable: is_valid
  %is_valid = alloca i1, align 1
  store i1 true, ptr %is_valid, align 1
  ; Variable: b
  %b = alloca i64, align 8
  store i64 20, ptr %b, align 8
  ; Variable: score
  %score = alloca i64, align 8
  store i64 95, ptr %score, align 8
  ; Variable: age
  %age = alloca i64, align 8
  store i64 25, ptr %age, align 8
  ; Variable: name
  %name = alloca ptr, align 8
  store ptr @.str.2, ptr %name, align 8
  ; Variable: is_complete
  %is_complete = alloca i1, align 1
  store i1 false, ptr %is_complete, align 1
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  call void @cursed_runtime_spill_int(i64 25)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  call void @cursed_runtime_spill_string(ptr @.str.2)
  call void @cursed_runtime_spill_int(i64 95)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  call void @cursed_runtime_spill_bool(i1 true)
  call void @cursed_runtime_spill_bool(i1 false)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  call void @cursed_runtime_spill_float(double 3.14159)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.6)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.7)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.8)
  call void @cursed_runtime_spill_int(i64 10)
  call void @cursed_runtime_spill_int(i64 20)
  call void @cursed_runtime_spill_int(i64 30)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [21 x i8] c"Hello, CURSED world!\00", align 1
@.str.1 = private unnamed_addr constant [18 x i8] c"I am {} years old\00", align 1
@.str.2 = private unnamed_addr constant [6 x i8] c"Alice\00", align 1
@.str.3 = private unnamed_addr constant [20 x i8] c"{} scored {} points\00", align 1
@.str.4 = private unnamed_addr constant [24 x i8] c"Valid: {}, Complete: {}\00", align 1
@.str.5 = private unnamed_addr constant [13 x i8] c"Pi value: {}\00", align 1
@.str.6 = private unnamed_addr constant [21 x i8] c"No formatting needed\00", align 1
@.str.7 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.8 = private unnamed_addr constant [21 x i8] c"Result: {} + {} = {}\00", align 1
