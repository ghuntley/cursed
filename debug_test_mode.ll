; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @simple_loop() {
  ret i64 0
}


define i32 @main() {
entry:
  ; Variable: i
  %i = alloca i64, align 8
  store i64 4, ptr %i, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  call void @cursed_runtime_spill_string(ptr @newline_str)
  ; Call: simple_loop
  ; Standalone call: simple_loop
  call i64 @simple_loop()
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [6 x i8] c"Hello\00", align 1
@.str.1 = private unnamed_addr constant [6 x i8] c"Hello\00", align 1
@.str.2 = private unnamed_addr constant [6 x i8] c"Hello\00", align 1
@newline_str = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@space_str = private unnamed_addr constant [2 x i8] c" \00", align 1
@hello_comma_str = private unnamed_addr constant [7 x i8] c"Hello,\00", align 1
