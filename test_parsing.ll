; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @helper_function(i64 %p0, i64 %p1) {
  %result = add i64 %p0, %p1
  ret i64 %result
}


define i32 @main() {
entry:
  ; Variable: x
  %x = alloca i64, align 8
  store i64 42, ptr %x, align 8
  ; Variable: z
  %z = alloca i64, align 8
  store i64 142, ptr %z, align 8
  ; Variable: value
  %value = alloca i64, align 8
  store i64 999, ptr %value, align 8
  ; Variable: y
  %y = alloca i64, align 8
  store i64 100, ptr %y, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 142)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [33 x i8] c"Package and import parsing works\00", align 1
@.str.1 = private unnamed_addr constant [27 x i8] c"Variable declarations work\00", align 1
@.str.2 = private unnamed_addr constant [17 x i8] c"Arithmetic works\00", align 1
