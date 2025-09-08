; ModuleID = 'test_pure_arithmetic'
source_filename = "test_pure_arithmetic"

declare void @cursed_runtime_spill_string(ptr %0)

declare void @cursed_runtime_spill_int(i64 %0)

declare i32 @printf(ptr %0, ...)

define i64 @add_two(i64 %0, i64 %1) {
entry:
  %a = alloca i64
  store i64 %0, ptr %a
  %b = alloca i64
  store i64 %1, ptr %b
  %2 = load i64, ptr %a
  %3 = load i64, ptr %b
  %4 = add i64 %2, %3
  ret i64 %4
}

define i64 @multiply(i64 %0, i64 %1) {
entry:
  %x = alloca i64
  store i64 %0, ptr %x
  %y = alloca i64
  store i64 %1, ptr %y
  %2 = load i64, ptr %x
  %3 = load i64, ptr %y
  %4 = mul i64 %2, %3
  ret i64 %4
}

define i32 @main() {
entry:
  %a = alloca i64
  store i64 10, ptr %a
  %b = alloca i64
  store i64 5, ptr %b
  %sum = alloca i64
  %0 = load i64, ptr %a
  %1 = load i64, ptr %b
  %2 = call i64 @add_two(i64 %0, i64 %1)
  store i64 %2, ptr %sum
  %product = alloca i64
  %3 = load i64, ptr %a
  %4 = load i64, ptr %b
  %5 = call i64 @multiply(i64 %3, i64 %4)
  store i64 %5, ptr %product
  ret i32 0
}
