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
  ; Variable: int_val
  %int_val = alloca i64, align 8
  store i64 10, ptr %int_val, align 8
  ; Variable: float_val
  %float_val = alloca double, align 8
  store double 3.5, ptr %float_val, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 10)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_float(double 3.5)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 15)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_float(double 6.0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 20)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.6)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_float(double 1.75)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.7)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [35 x i8] c"=== Mixed Type Arithmetic Test ===\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"Integer value:\00", align 1
@.str.2 = private unnamed_addr constant [13 x i8] c"Float value:\00", align 1
@.str.3 = private unnamed_addr constant [19 x i8] c"Integer + Integer:\00", align 1
@.str.4 = private unnamed_addr constant [15 x i8] c"Float + Float:\00", align 1
@.str.5 = private unnamed_addr constant [13 x i8] c"Integer * 2:\00", align 1
@.str.6 = private unnamed_addr constant [11 x i8] c"Float / 2:\00", align 1
@.str.7 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
@newline_str = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@hello_comma_str = private unnamed_addr constant [7 x i8] c"Hello,\00", align 1
