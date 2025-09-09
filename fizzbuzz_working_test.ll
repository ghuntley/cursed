; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @fizzbuzz(i64 %n) {
entry:
  ; FizzBuzz while loop implementation
  ; Initialize loop variable i = 1
  %i = alloca i64, align 8
  store i64 1, ptr %i, align 8
  br label %while.cond

while.cond:
  %i_val = load i64, ptr %i, align 8
  %cmp = icmp sle i64 %i_val, %n
  br i1 %cmp, label %while.body, label %while.end

while.body:
  ; FizzBuzz logic
  %mod15 = srem i64 %i_val, 15
  %is_fizzbuzz = icmp eq i64 %mod15, 0
  br i1 %is_fizzbuzz, label %print_fizzbuzz, label %check_fizz

print_fizzbuzz:
  call void @cursed_runtime_spill_string(ptr @fizzbuzz_str)
  br label %increment

check_fizz:
  %mod3 = srem i64 %i_val, 3
  %is_fizz = icmp eq i64 %mod3, 0
  br i1 %is_fizz, label %print_fizz, label %check_buzz

print_fizz:
  call void @cursed_runtime_spill_string(ptr @fizz_str)
  br label %increment

check_buzz:
  %mod5 = srem i64 %i_val, 5
  %is_buzz = icmp eq i64 %mod5, 0
  br i1 %is_buzz, label %print_buzz, label %print_number

print_buzz:
  call void @cursed_runtime_spill_string(ptr @buzz_str)
  br label %increment

print_number:
  call void @cursed_runtime_spill_int(i64 %i_val)
  br label %increment

increment:
  %next_i = add i64 %i_val, 1
  store i64 %next_i, ptr %i, align 8
  br label %while.cond

while.end:
  ret i64 0
}


define i32 @main() {
entry:
  ; Variable: i
  %i = alloca i64, align 8
  store i64 1, ptr %i, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: fizzbuzz
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Standalone call: fizzbuzz
  call i64 @fizzbuzz(i64 15)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [22 x i8] c"=== FizzBuzz Test ===\00", align 1
@.str.1 = private unnamed_addr constant [23 x i8] c"FizzBuzz from 1 to 15:\00", align 1
@.str.2 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
@newline_str = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@fizzbuzz_str = private unnamed_addr constant [9 x i8] c"FizzBuzz\00", align 1
@fizz_str = private unnamed_addr constant [5 x i8] c"Fizz\00", align 1
@buzz_str = private unnamed_addr constant [5 x i8] c"Buzz\00", align 1
@space_str = private unnamed_addr constant [2 x i8] c" \00", align 1
@hello_comma_str = private unnamed_addr constant [7 x i8] c"Hello,\00", align 1
