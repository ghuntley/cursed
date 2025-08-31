; ModuleID = '03_operator_precedence'
source_filename = "03_operator_precedence"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0
@.str.0 = private constant [35 x i8] c"\22=== Operator Precedence Test ===\22\00"
@.str.1 = private constant [26 x i8] c"\222 + 3 * 4 should be 14:\22\00"
@.str.2 = private constant [28 x i8] c"\22(2 + 3) * 4 should be 20:\22\00"
@.str.3 = private constant [26 x i8] c"\2210 - 3 + 2 should be 9:\22\00"
@.str.4 = private constant [26 x i8] c"\228 / 2 * 3 should be 12:\22\00"
@.str.5 = private constant [24 x i8] c"\22=== Test Complete ===\22\00"

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

declare i32 @yap(ptr)

define void @main_character() {
entry:
  %puts_call = call i32 @puts(ptr @.str.0)
  %puts_call1 = call i32 @puts(ptr @.str.1)
  %spill_i64_result = call i32 @cursed_dbg_spill_i64(i64 14)
  %puts_call2 = call i32 @puts(ptr @.str.2)
  %spill_i64_result3 = call i32 @cursed_dbg_spill_i64(i64 20)
  %puts_call4 = call i32 @puts(ptr @.str.3)
  %spill_i64_result5 = call i32 @cursed_dbg_spill_i64(i64 9)
  %puts_call6 = call i32 @puts(ptr @.str.4)
  %spill_i64_result7 = call i32 @cursed_dbg_spill_i64(i64 12)
  %puts_call8 = call i32 @puts(ptr @.str.5)
  ret void
}

declare i32 @cursed_dbg_spill_i64(i64)

define i32 @main() {
entry:
  call void @main_character()
  ret i32 0
}
