; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @test_function() {
  ret i64 0
}


define i32 @main() {
entry:
  ; Variable: x
  %x = alloca i64, align 8
  store i64 1, ptr %x, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: test_function
  ; Standalone call: test_function
  call i64 @test_function()
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [4 x i8] c"One\00", align 1
@.str.1 = private unnamed_addr constant [9 x i8] c"After if\00", align 1
@newline_str = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@space_str = private unnamed_addr constant [2 x i8] c" \00", align 1
@hello_comma_str = private unnamed_addr constant [7 x i8] c"Hello,\00", align 1
