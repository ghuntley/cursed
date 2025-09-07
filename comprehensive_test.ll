; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

define i32 @main() {
entry:
  ; Variable: x
  %x = alloca i64, align 8
  store i64 42, ptr %x, align 8
  ; Variable: y
  %y = alloca i64, align 8
  store i64 24, ptr %y, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  ; Variable not available (unsupported type)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: vibez.spill
  ; Variable not available (unsupported type)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  %x_load = load i64, ptr %x, align 8
  call void @cursed_runtime_spill_int(i64 %x_load)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  ; Call: vibez.spill
  %y_load = load i64, ptr %y, align 8
  call void @cursed_runtime_spill_int(i64 %y_load)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [37 x i8] c"🔥 Testing Extended LLVM Pipeline!\00", align 1
@.str.1 = private unnamed_addr constant [17 x i8] c"Addition result:\00", align 1
@.str.2 = private unnamed_addr constant [20 x i8] c"Subtraction result:\00", align 1
@.str.3 = private unnamed_addr constant [9 x i8] c"x value:\00", align 1
@.str.4 = private unnamed_addr constant [9 x i8] c"y value:\00", align 1
@.str.5 = private unnamed_addr constant [42 x i8] c"✅ Extended LLVM pipeline test complete!\00", align 1
