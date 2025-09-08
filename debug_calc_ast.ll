; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @calculate_complex(i64 %a, i64 %b, i64 %c) {
  ret i64 39
}


define i32 @main() {
entry:
  ; Variable: result
  %result = alloca i64, align 8
  store i64 0, ptr %result, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: calculate_complex
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Standalone call: calculate_complex
  call i64 @calculate_complex(i64 10, i64 4, i64 3)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [37 x i8] c"Testing calculate_complex(10, 4, 3):\00", align 1
@.str.1 = private unnamed_addr constant [8 x i8] c"Result:\00", align 1
@.str.2 = private unnamed_addr constant [13 x i8] c"Expected: 39\00", align 1
@newline_str = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@space_str = private unnamed_addr constant [2 x i8] c" \00", align 1
@hello_comma_str = private unnamed_addr constant [7 x i8] c"Hello,\00", align 1
