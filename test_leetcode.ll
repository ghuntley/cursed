; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions
define i64 @solve_partition_array_into_three_parts(i64 %p0, i64 %p1, i64 %p2) {
  %sum = add i64 %p0, %p1
  %sum1 = add i64 %sum, %p2
  ret i64 %sum1
}


define i32 @main() {
entry:
  ; Variable: result
  %result = alloca i64, align 8
  store i64 42, ptr %result, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 42)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  ; Call: solve_partition_array_into_three_parts
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [57 x i8] c"=== LeetCode #1013: Partition Array Into Three Parts ===\00", align 1
@.str.1 = private unnamed_addr constant [13 x i8] c"Demo result:\00", align 1
@.str.2 = private unnamed_addr constant [50 x i8] c"=== Partition Array Into Three Parts Complete ===\00", align 1
