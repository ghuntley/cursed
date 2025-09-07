; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

define i32 @main() {
entry:
  ; Variable: num2
  %num2 = alloca i64, align 8
  store i64 -17, ptr %num2, align 8
  ; Variable: abs_num
  %abs_num = alloca i64, align 8
  store i64 17, ptr %abs_num, align 8
  ; Variable: final_text
  %final_text = alloca ptr, align 8
  store ptr @.str.0, ptr %final_text, align 8
  ; Variable: text1
  %text1 = alloca ptr, align 8
  store ptr @.str.1, ptr %text1, align 8
  ; Variable: text2
  %text2 = alloca ptr, align 8
  store ptr @.str.2, ptr %text2, align 8
  ; Variable: num_str
  %num_str = alloca ptr, align 8
  store ptr @.str.3, ptr %num_str, align 8
  ; Variable: num1
  %num1 = alloca i64, align 8
  store i64 42, ptr %num1, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  call void @cursed_runtime_spill_int(i64 17)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  call void @cursed_runtime_spill_int(i64 42)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  call void @cursed_runtime_spill_int(i64 17)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.8)
  call void @cursed_runtime_spill_int(i64 5)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.9)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.10)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.11)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.14)
  call void @cursed_runtime_spill_string(ptr @.str.13)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.15)
  call void @cursed_runtime_spill_int(i64 7)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.16)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [32 x i8] c"=== Stdlib Integration Test ===\00", align 1
@.str.1 = private unnamed_addr constant [28 x i8] c"Testing mathz operations...\00", align 1
@.str.2 = private unnamed_addr constant [11 x i8] c"abs(-17) =\00", align 1
@.str.3 = private unnamed_addr constant [14 x i8] c"max(42, 17) =\00", align 1
@.str.4 = private unnamed_addr constant [14 x i8] c"min(42, 17) =\00", align 1
@.str.5 = private unnamed_addr constant [30 x i8] c"Testing stringz operations...\00", align 1
@.str.6 = private unnamed_addr constant [6 x i8] c"Hello\00", align 1
@.str.7 = private unnamed_addr constant [6 x i8] c"World\00", align 1
@.str.8 = private unnamed_addr constant [19 x i8] c"Length of 'Hello':\00", align 1
@.str.9 = private unnamed_addr constant [11 x i8] c"Uppercase:\00", align 1
@.str.10 = private unnamed_addr constant [11 x i8] c"Lowercase:\00", align 1
@.str.11 = private unnamed_addr constant [23 x i8] c"Combined operations...\00", align 1
@.str.12 = private unnamed_addr constant [3 x i8] c"17\00", align 1
@.str.13 = private unnamed_addr constant [8 x i8] c"Hello17\00", align 1
@.str.14 = private unnamed_addr constant [14 x i8] c"Final result:\00", align 1
@.str.15 = private unnamed_addr constant [14 x i8] c"Final length:\00", align 1
@.str.16 = private unnamed_addr constant [34 x i8] c"Stdlib integration test completed\00", align 1
