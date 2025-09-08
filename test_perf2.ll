; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @sum_range(i64 %p0, i64 %p1) {
  %result = add i64 %p0, %p1
  ret i64 %result
}

define i64 @fibonacci(i64 %p0, i64 %p1) {
  %result = add i64 %p0, %p1
  ret i64 %result
}


define i32 @main() {
entry:
  ; Variable: sum
  %sum = alloca i64, align 8
  store i64 0, ptr %sum, align 8
  ; Variable: i
  %i = alloca i64, align 8
  store i64 1, ptr %i, align 8
  ; Call: fibonacci
  ; Call: fibonacci
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  ; Call: vibez.spill
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: vibez.spill
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [35 x i8] c"=== Computation Intensive Test ===\00", align 1
@.str.1 = private unnamed_addr constant [20 x i8] c"Fibonacci sequence:\00", align 1
@.str.2 = private unnamed_addr constant [14 x i8] c"Sum of range:\00", align 1
@.str.3 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
