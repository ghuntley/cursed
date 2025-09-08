; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @greet(i64 %p0) {
  ret i64 %p0
}


define i32 @main() {
entry:
  ; Call: greet
  ; Standalone call: greet
  call i64 @greet(ptr @.str.0)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [6 x i8] c"Hello\00", align 1
@newline_str = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@hello_comma_str = private unnamed_addr constant [7 x i8] c"Hello,\00", align 1
