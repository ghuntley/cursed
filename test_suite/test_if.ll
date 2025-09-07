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
  store i64 10, ptr %x, align 8
  ; Variable: score
  %score = alloca i64, align 8
  store i64 85, ptr %score, align 8
  ; Variable: y
  %y = alloca i64, align 8
  store i64 5, ptr %y, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.6)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.7)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.8)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.9)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.10)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [26 x i8] c"=== If Statement Test ===\00", align 1
@.str.1 = private unnamed_addr constant [14 x i8] c"x = 10, y = 5\00", align 1
@.str.2 = private unnamed_addr constant [20 x i8] c"x is greater than y\00", align 1
@.str.3 = private unnamed_addr constant [20 x i8] c"y is greater than x\00", align 1
@.str.4 = private unnamed_addr constant [21 x i8] c"x is not less than y\00", align 1
@.str.5 = private unnamed_addr constant [18 x i8] c"Score evaluation:\00", align 1
@.str.6 = private unnamed_addr constant [8 x i8] c"Grade A\00", align 1
@.str.7 = private unnamed_addr constant [8 x i8] c"Grade B\00", align 1
@.str.8 = private unnamed_addr constant [8 x i8] c"Grade C\00", align 1
@.str.9 = private unnamed_addr constant [8 x i8] c"Grade F\00", align 1
@.str.10 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
