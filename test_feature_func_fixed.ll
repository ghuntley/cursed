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
  ; Call: print_message
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.6)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.7)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Standalone call: print_message
  call i64 @print_message(ptr @.str.2)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [27 x i8] c"=== Function Call Test ===\00", align 1
@.str.1 = private unnamed_addr constant [17 x i8] c"Sum of 7 and 13:\00", align 1
@.str.2 = private unnamed_addr constant [25 x i8] c"Function call successful\00", align 1
@.str.3 = private unnamed_addr constant [16 x i8] c"Is 25 positive?\00", align 1
@.str.4 = private unnamed_addr constant [16 x i8] c"Is -5 positive?\00", align 1
@.str.5 = private unnamed_addr constant [31 x i8] c"Function result in expression:\00", align 1
@.str.6 = private unnamed_addr constant [29 x i8] c"Mixed function calls result:\00", align 1
@.str.7 = private unnamed_addr constant [31 x i8] c"=== Function Test Complete ===\00", align 1
@newline_str = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@hello_comma_str = private unnamed_addr constant [7 x i8] c"Hello,\00", align 1
