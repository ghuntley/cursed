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
  ; Variable: name
  %name = alloca ptr, align 8
  store ptr @.str.0, ptr %name, align 8
  ; Variable: y
  %y = alloca i64, align 8
  store i64 50, ptr %y, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 42)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 50)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [33 x i8] c"=== Variable Assignment Test ===\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"x =\00", align 1
@.str.2 = private unnamed_addr constant [12 x i8] c"y = x + 8 =\00", align 1
@.str.3 = private unnamed_addr constant [7 x i8] c"CURSED\00", align 1
@.str.4 = private unnamed_addr constant [7 x i8] c"name =\00", align 1
@.str.5 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
