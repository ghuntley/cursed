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
  ; Variable: ptr
  %ptr = alloca i64, align 8
  store i64 0, ptr %ptr, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  %x_load = load i64, ptr %x, align 8
  call void @cursed_runtime_spill_int(i64 %x_load)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: vibez.spill
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  %x_load = load i64, ptr %x, align 8
  call void @cursed_runtime_spill_int(i64 %x_load)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  ; Call: vibez.spill
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [33 x i8] c"=== Basic Pointer Operations ===\00", align 1
@.str.1 = private unnamed_addr constant [16 x i8] c"Original value:\00", align 1
@.str.2 = private unnamed_addr constant [23 x i8] c"Value through pointer:\00", align 1
@.str.3 = private unnamed_addr constant [25 x i8] c"Modified original value:\00", align 1
@.str.4 = private unnamed_addr constant [42 x i8] c"Value through pointer after modification:\00", align 1
@.str.5 = private unnamed_addr constant [22 x i8] c"=== Test Complete ===\00", align 1
