; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @handle_empty(i64 %p0, i64 %p1) {
  %result = add i64 %p0, %p1
  ret i64 %result
}


define i32 @main() {
entry:
  ; Variable: empty_str
  %empty_str = alloca ptr, align 8
  store ptr @.str.2, ptr %empty_str, align 8
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
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  ; Call: vibez.spill
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.6)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [26 x i8] c"=== Empty Inputs Test ===\00", align 1
@.str.1 = private unnamed_addr constant [11 x i8] c"Empty tea:\00", align 1
@.str.2 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.3 = private unnamed_addr constant [16 x i8] c"After empty tea\00", align 1
@.str.4 = private unnamed_addr constant [12 x i8] c"Zero value:\00", align 1
@.str.5 = private unnamed_addr constant [16 x i8] c"Non-zero value:\00", align 1
@.str.6 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
