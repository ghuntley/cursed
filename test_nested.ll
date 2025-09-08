; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @calculate_complex(i64 %p0, i64 %p1) {
  %result = add i64 %p0, %p1
  ret i64 %result
}


define i32 @main() {
entry:
  ; Variable: result2
  %result2 = alloca i64, align 8
  %result2_call = call i64 @calculate_complex(i64 10, i64 4, i64 3)
  store i64 %result2_call, ptr %result2, align 8
  ; Variable: x
  %x = alloca i64, align 8
  store i64 15, ptr %x, align 8
  ; Variable: a
  %a = alloca i64, align 8
  store i64 2, ptr %a, align 8
  ; Variable: b
  %b = alloca i64, align 8
  store i64 3, ptr %b, align 8
  ; Variable: result1
  %result1 = alloca i64, align 8
  store i64 15, ptr %result1, align 8
  ; Variable: c
  %c = alloca i64, align 8
  store i64 4, ptr %c, align 8
  ; Variable: final
  %final = alloca i64, align 8
  store i64 12, ptr %final, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 15)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  %temp_call_0 = call i64 @calculate_complex(i64 10, i64 4, i64 3)
  call void @cursed_runtime_spill_int(i64 %temp_call_0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 12)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.6)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [31 x i8] c"=== Nested Operations Test ===\00", align 1
@.str.1 = private unnamed_addr constant [20 x i8] c"Complex arithmetic:\00", align 1
@.str.2 = private unnamed_addr constant [23 x i8] c"Nested function calls:\00", align 1
@.str.3 = private unnamed_addr constant [21 x i8] c"Nested conditionals:\00", align 1
@.str.4 = private unnamed_addr constant [23 x i8] c"x is between 10 and 20\00", align 1
@.str.5 = private unnamed_addr constant [21 x i8] c"Multiple operations:\00", align 1
@.str.6 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
@newline_str = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@hello_comma_str = private unnamed_addr constant [7 x i8] c"Hello,\00", align 1
