; ModuleID = 'test_pure_function'
source_filename = "test_pure_function"

declare void @cursed_runtime_spill_string(ptr %0)

declare void @cursed_runtime_spill_int(i64 %0)

declare i32 @printf(ptr %0, ...)

define i64 @add_test(i64 %0, i64 %1) {
entry:
  %x = alloca i64
  store i64 %0, ptr %x
  %y = alloca i64
  store i64 %1, ptr %y
  %2 = load i64, ptr %x
  %3 = load i64, ptr %y
  %4 = add i64 %2, %3
  ret i64 %4
  ret i64 0
}

define i32 @main() {
entry:
  ret i32 0
  ret i32 0
}
