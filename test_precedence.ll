; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

; User-defined CURSED Functions

define i32 @main() {
entry:
  ; Variable: result2
  %result2 = alloca i64, align 8
  store i64 6, ptr %result2, align 8
  ; Variable: comp1
  %comp1 = alloca i64, align 8
  store i64 0, ptr %comp1, align 8
  ; Variable: result5
  %result5 = alloca i64, align 8
  store i64 14, ptr %result5, align 8
  ; Variable: bool1
  %bool1 = alloca i64, align 8
  store i64 1, ptr %bool1, align 8
  ; Variable: bool3
  %bool3 = alloca i64, align 8
  store i64 0, ptr %bool3, align 8
  ; Variable: complex2
  %complex2 = alloca i64, align 8
  store i64 1, ptr %complex2, align 8
  ; Variable: result1
  %result1 = alloca i64, align 8
  store i64 14, ptr %result1, align 8
  ; Variable: bool2
  %bool2 = alloca i64, align 8
  store i64 1, ptr %bool2, align 8
  ; Variable: result4
  %result4 = alloca i64, align 8
  store i64 20, ptr %result4, align 8
  ; Variable: result6
  %result6 = alloca i64, align 8
  store i64 2, ptr %result6, align 8
  ; Variable: result3
  %result3 = alloca i64, align 8
  store i64 22, ptr %result3, align 8
  ; Variable: comp2
  %comp2 = alloca i64, align 8
  store i64 1, ptr %comp2, align 8
  ; Variable: comp3
  %comp3 = alloca i64, align 8
  store i64 1, ptr %comp3, align 8
  ; Variable: complex1
  %complex1 = alloca i64, align 8
  store i64 13, ptr %complex1, align 8
  ret i32 0
}

; String Constants
