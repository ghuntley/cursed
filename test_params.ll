; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @no_params(i64 %p0, i64 %p1) {
  %result = add i64 %p0, %p1
  ret i64 %result
}

define i64 @multiple_params(i64 %p0, i64 %p1) {
  %result = mul i64 %p0, %p1
  ret i64 %result
}

define i64 @one_param(i64 %p0, i64 %p1) {
  %result = add i64 %p0, %p1
  ret i64 %result
}


define i32 @main() {
entry:
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: vibez.spill
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  ; Call: vibez.spill
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [33 x i8] c"=== Function Parameters Test ===\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"No parameters:\00", align 1
@.str.2 = private unnamed_addr constant [15 x i8] c"One parameter:\00", align 1
@.str.3 = private unnamed_addr constant [21 x i8] c"Multiple parameters:\00", align 1
@.str.4 = private unnamed_addr constant [29 x i8] c"Parameters with expressions:\00", align 1
@.str.5 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
