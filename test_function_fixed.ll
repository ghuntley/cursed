; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @add_numbers(i64 %a, i64 %b) {
  %result = add i64 %a, %b
  ret i64 %result
}
define void @greet(ptr %name) {
  call void @cursed_runtime_spill_string(ptr @hello_comma_str)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  call void @cursed_runtime_spill_string(ptr %name)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ret void
}

define i32 @main() {
entry:
  ; Variable: result
  %result = alloca i64, align 8
  %result_call = call i64 @add_numbers(i64 7, i64 3)
  store i64 %result_call, ptr %result, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  ; Variable not available (unsupported type)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  %temp_call_0 = call i64 @add_numbers(i64 7, i64 3)
  call void @cursed_runtime_spill_int(i64 %temp_call_0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: greet
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Standalone call: greet
  call i64 @greet(ptr @.str.0)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [7 x i8] c"Hello,\00", align 1
@.str.1 = private unnamed_addr constant [29 x i8] c"=== Simple Function Test ===\00", align 1
@.str.2 = private unnamed_addr constant [21 x i8] c"Testing add_numbers:\00", align 1
@.str.3 = private unnamed_addr constant [24 x i8] c"Testing greet function:\00", align 1
@.str.4 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
@newline_str = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@hello_comma_str = private unnamed_addr constant [7 x i8] c"Hello,\00", align 1
