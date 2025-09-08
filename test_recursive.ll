; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @factorial(i64 %p0, i64 %p1) {
  %result = add i64 %p0, %p1
  ret i64 %result
}

define i64 @countdown(i64 %p0, i64 %p1) {
  %result = add i64 %p0, %p1
  ret i64 %result
}


define i32 @main() {
entry:
  ; Variable: fact
  %fact = alloca i64, align 8
  %fact_call = call i64 @factorial(i64 5)
  store i64 %fact_call, ptr %fact, align 8
  ; Call: factorial
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  %temp_call_0 = call i64 @factorial(i64 5)
  call void @cursed_runtime_spill_int(i64 %temp_call_0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: countdown
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [32 x i8] c"=== Recursive Function Test ===\00", align 1
@.str.1 = private unnamed_addr constant [16 x i8] c"Factorial of 5:\00", align 1
@.str.2 = private unnamed_addr constant [18 x i8] c"Countdown from 3:\00", align 1
@.str.3 = private unnamed_addr constant [6 x i8] c"Done!\00", align 1
@.str.4 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
