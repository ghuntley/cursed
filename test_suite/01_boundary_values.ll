; ModuleID = '01_boundary_values'
source_filename = "01_boundary_values"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0
@.str.0 = private constant [31 x i8] c"\22=== Boundary Values Test ===\22\00"
@.str.1 = private constant [17 x i8] c"\22Small numbers:\22\00"
@.str.2 = private constant [17 x i8] c"\22Large numbers:\22\00"
@.str.3 = private constant [22 x i8] c"\22Decimal boundaries:\22\00"
@.str.4 = private constant [24 x i8] c"\22=== Test Complete ===\22\00"

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

declare i32 @yap(ptr)

define void @main_character() {
entry:
  %puts_call = call i32 @puts(ptr @.str.0)
  %puts_call1 = call i32 @puts(ptr @.str.1)
  %spill_i64_result = call i32 @cursed_dbg_spill_i64(i64 1)
  %spill_i64_result2 = call i32 @cursed_dbg_spill_i64(i64 0)
  %spill_i64_result3 = call i32 @cursed_dbg_spill_i64(i64 -1)
  %puts_call4 = call i32 @puts(ptr @.str.2)
  %spill_i64_result5 = call i32 @cursed_dbg_spill_i64(i64 999999)
  %spill_i64_result6 = call i32 @cursed_dbg_spill_i64(i64 1000000)
  %spill_i64_result7 = call i32 @cursed_dbg_spill_i64(i64 -999999)
  %puts_call8 = call i32 @puts(ptr @.str.3)
  %spill_f64_result = call i32 @cursed_dbg_spill_f64(double 1.000000e-01)
  %spill_f64_result9 = call i32 @cursed_dbg_spill_f64(double 0.000000e+00)
  %spill_f64_result10 = call i32 @cursed_dbg_spill_f64(double -1.000000e-01)
  %puts_call11 = call i32 @puts(ptr @.str.4)
  ret void
}

declare i32 @cursed_dbg_spill_i64(i64)

declare i32 @cursed_dbg_spill_f64(double)

define i32 @main() {
entry:
  call void @main_character()
  ret i32 0
}
