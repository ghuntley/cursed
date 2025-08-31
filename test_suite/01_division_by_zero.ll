; ModuleID = '01_division_by_zero'
source_filename = "01_division_by_zero"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0
@.str.0 = private constant [32 x i8] c"\22=== Division by Zero Test ===\22\00"
@.str.1 = private constant [19 x i8] c"\22Normal division:\22\00"
@.str.2 = private constant [31 x i8] c"\22Attempting division by zero:\22\00"
@.str.3 = private constant [55 x i8] c"\22This line should not execute if error handling works\22\00"

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

declare i32 @yap(ptr)

define void @main_character() {
entry:
  %puts_call = call i32 @puts(ptr @.str.0)
  %puts_call1 = call i32 @puts(ptr @.str.1)
  %spill_i64_result = call i32 @cursed_dbg_spill_i64(i64 5)
  %puts_call2 = call i32 @puts(ptr @.str.2)
  %spill_i64_result3 = call i32 @cursed_dbg_spill_i64(i64 poison)
  %puts_call4 = call i32 @puts(ptr @.str.3)
  ret void
}

declare i32 @cursed_dbg_spill_i64(i64)

define i32 @main() {
entry:
  call void @main_character()
  ret i32 0
}
