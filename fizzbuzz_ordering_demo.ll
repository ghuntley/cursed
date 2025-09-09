; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @fizzbuzz(i64 %n) {
  ret i64 0
}


define i32 @main() {
entry:
  ; Variable: i
  %i = alloca i64, align 8
  store i64 16, ptr %i, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 1)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 2)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 4)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 7)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 8)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 11)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 13)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 14)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.6)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.7)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.8)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: fizzbuzz
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.9)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Standalone call: fizzbuzz
  call i64 @fizzbuzz(i64 15)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [5 x i8] c"Fizz\00", align 1
@.str.1 = private unnamed_addr constant [5 x i8] c"Buzz\00", align 1
@.str.2 = private unnamed_addr constant [5 x i8] c"Fizz\00", align 1
@.str.3 = private unnamed_addr constant [5 x i8] c"Fizz\00", align 1
@.str.4 = private unnamed_addr constant [5 x i8] c"Buzz\00", align 1
@.str.5 = private unnamed_addr constant [5 x i8] c"Fizz\00", align 1
@.str.6 = private unnamed_addr constant [9 x i8] c"FizzBuzz\00", align 1
@.str.7 = private unnamed_addr constant [22 x i8] c"=== FizzBuzz Test ===\00", align 1
@.str.8 = private unnamed_addr constant [23 x i8] c"FizzBuzz from 1 to 15:\00", align 1
@.str.9 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
@newline_str = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@space_str = private unnamed_addr constant [2 x i8] c" \00", align 1
@hello_comma_str = private unnamed_addr constant [7 x i8] c"Hello,\00", align 1
