; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions

define i32 @main() {
entry:
  ; Variable: value
  %value = alloca i64, align 8
  store i64 0, ptr %value, align 8
  ; Variable: counter
  %counter = alloca i64, align 8
  store i64 6, ptr %counter, align 8
  ; Variable: inner
  %inner = alloca i64, align 8
  store i64 3, ptr %inner, align 8
  ; Variable: countdown
  %countdown = alloca i64, align 8
  store i64 0, ptr %countdown, align 8
  ; Variable: outer
  %outer = alloca i64, align 8
  store i64 3, ptr %outer, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 1)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 2)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 3)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 4)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 5)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.7)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.8)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 3)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.8)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 2)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.8)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 1)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.11)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.12)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.13)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 1)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.14)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 1)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.14)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 2)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.13)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 2)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.17)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.18)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 10)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.19)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 9)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.18)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 8)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.19)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 7)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.18)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 6)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.23)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.19)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 5)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.18)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 4)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.19)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 3)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.18)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 2)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.19)
  call void @cursed_runtime_spill_string(ptr @space_str)
  call void @cursed_runtime_spill_int(i64 1)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.29)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [36 x i8] c"=== Loop Syntax Regression Test ===\00", align 1
@.str.1 = private unnamed_addr constant [30 x i8] c"Testing basic periodt loop...\00", align 1
@.str.2 = private unnamed_addr constant [7 x i8] c"Count:\00", align 1
@.str.3 = private unnamed_addr constant [7 x i8] c"Count:\00", align 1
@.str.4 = private unnamed_addr constant [7 x i8] c"Count:\00", align 1
@.str.5 = private unnamed_addr constant [7 x i8] c"Count:\00", align 1
@.str.6 = private unnamed_addr constant [7 x i8] c"Count:\00", align 1
@.str.7 = private unnamed_addr constant [26 x i8] c"Testing countdown loop...\00", align 1
@.str.8 = private unnamed_addr constant [11 x i8] c"Countdown:\00", align 1
@.str.9 = private unnamed_addr constant [11 x i8] c"Countdown:\00", align 1
@.str.10 = private unnamed_addr constant [11 x i8] c"Countdown:\00", align 1
@.str.11 = private unnamed_addr constant [16 x i8] c"Loop completed!\00", align 1
@.str.12 = private unnamed_addr constant [24 x i8] c"Testing nested loops...\00", align 1
@.str.13 = private unnamed_addr constant [12 x i8] c"Outer loop:\00", align 1
@.str.14 = private unnamed_addr constant [14 x i8] c"  Inner loop:\00", align 1
@.str.15 = private unnamed_addr constant [14 x i8] c"  Inner loop:\00", align 1
@.str.16 = private unnamed_addr constant [12 x i8] c"Outer loop:\00", align 1
@.str.17 = private unnamed_addr constant [32 x i8] c"Testing loop with conditions...\00", align 1
@.str.18 = private unnamed_addr constant [13 x i8] c"Even number:\00", align 1
@.str.19 = private unnamed_addr constant [12 x i8] c"Odd number:\00", align 1
@.str.20 = private unnamed_addr constant [13 x i8] c"Even number:\00", align 1
@.str.21 = private unnamed_addr constant [12 x i8] c"Odd number:\00", align 1
@.str.22 = private unnamed_addr constant [13 x i8] c"Even number:\00", align 1
@.str.23 = private unnamed_addr constant [22 x i8] c"Halfway point reached\00", align 1
@.str.24 = private unnamed_addr constant [12 x i8] c"Odd number:\00", align 1
@.str.25 = private unnamed_addr constant [13 x i8] c"Even number:\00", align 1
@.str.26 = private unnamed_addr constant [12 x i8] c"Odd number:\00", align 1
@.str.27 = private unnamed_addr constant [13 x i8] c"Even number:\00", align 1
@.str.28 = private unnamed_addr constant [12 x i8] c"Odd number:\00", align 1
@.str.29 = private unnamed_addr constant [38 x i8] c"Loop syntax regression test completed\00", align 1
@newline_str = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@space_str = private unnamed_addr constant [2 x i8] c" \00", align 1
@hello_comma_str = private unnamed_addr constant [7 x i8] c"Hello,\00", align 1
