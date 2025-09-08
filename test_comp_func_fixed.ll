; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @print_message(i64 %p0) {
  ret i64 %p0
}


define i32 @main() {
entry:
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: print_message
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.6)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Standalone call: print_message
  call i64 @print_message(ptr @.str.4)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [34 x i8] c"=== Function Definitions Test ===\00", align 1
@.str.1 = private unnamed_addr constant [9 x i8] c"10 + 5 =\00", align 1
@.str.2 = private unnamed_addr constant [12 x i8] c"2 * 3 * 4 =\00", align 1
@.str.3 = private unnamed_addr constant [23 x i8] c"Message from function:\00", align 1
@.str.4 = private unnamed_addr constant [21 x i8] c"Hello from function!\00", align 1
@.str.5 = private unnamed_addr constant [23 x i8] c"Average of 10, 20, 30:\00", align 1
@.str.6 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
@newline_str = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@hello_comma_str = private unnamed_addr constant [7 x i8] c"Hello,\00", align 1
