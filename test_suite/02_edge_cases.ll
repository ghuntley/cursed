; ModuleID = '02_edge_cases'
source_filename = "02_edge_cases"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0
@.str.0 = private constant [37 x i8] c"\22=== Arithmetic Edge Cases Test ===\22\00"
@.str.1 = private constant [19 x i8] c"\22Zero operations:\22\00"
@.str.2 = private constant [20 x i8] c"\22Negative numbers:\22\00"
@.str.3 = private constant [17 x i8] c"\22Large numbers:\22\00"
@.str.4 = private constant [24 x i8] c"\22=== Test Complete ===\22\00"

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

declare i32 @yap(ptr)

define void @main_character() {
entry:
  %puts_call = call i32 @puts(ptr @.str.0)
  %puts_call1 = call i32 @puts(ptr @.str.1)
  %spill_i64_result = call i32 @cursed_dbg_spill_i64(i64 5)
  %spill_i64_result2 = call i32 @cursed_dbg_spill_i64(i64 0)
  %spill_i64_result3 = call i32 @cursed_dbg_spill_i64(i64 0)
  %puts_call4 = call i32 @puts(ptr @.str.2)
  %spill_i64_result5 = call i32 @cursed_dbg_spill_i64(i64 -2)
  %spill_i64_result6 = call i32 @cursed_dbg_spill_i64(i64 -6)
  %spill_i64_result7 = call i32 @cursed_dbg_spill_i64(i64 -6)
  %puts_call8 = call i32 @puts(ptr @.str.3)
  %spill_i64_result9 = call i32 @cursed_dbg_spill_i64(i64 3000)
  %spill_i64_result10 = call i32 @cursed_dbg_spill_i64(i64 999998)
  %puts_call11 = call i32 @puts(ptr @.str.4)
  ret void
}

declare i32 @cursed_dbg_spill_i64(i64)

define i32 @main() {
entry:
  call void @main_character()
  ret i32 0
}
