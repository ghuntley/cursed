; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @check_condition(i64 %p0, i64 %p1) {
  %result = add i64 %p0, %p1
  ret i64 %result
}


define i32 @main() {
entry:
  ; Variable: x
  %x = alloca i64, align 8
  store i64 10, ptr %x, align 8
  ; Variable: a
  %a = alloca i1, align 1
  store i1 true, ptr %a, align 1
  ; Variable: b
  %b = alloca i1, align 1
  store i1 false, ptr %b, align 1
  ; Variable: result
  %result = alloca i64, align 8
  %result_call = call i64 @check_condition(i64 10, i64 15)
  store i64 %result_call, ptr %result, align 8
  ; Variable: y
  %y = alloca i64, align 8
  store i64 15, ptr %y, align 8
  ; Variable: grade
  %grade = alloca i64, align 8
  store i64 85, ptr %grade, align 8
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
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [29 x i8] c"=== Testing Control Flow ===\00", align 1
@.str.1 = private unnamed_addr constant [20 x i8] c"x is greater than 5\00", align 1
@.str.2 = private unnamed_addr constant [33 x i8] c"Both x and y are positive, y > x\00", align 1
@.str.3 = private unnamed_addr constant [8 x i8] c"Grade B\00", align 1
@.str.4 = private unnamed_addr constant [34 x i8] c"Expression-based condition passed\00", align 1
