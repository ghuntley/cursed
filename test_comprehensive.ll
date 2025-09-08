; ModuleID = 'comprehensive_core_test'
source_filename = "comprehensive_core_test"

declare void @cursed_runtime_spill_string(ptr %0)

declare void @cursed_runtime_spill_int(i64 %0)

declare i32 @printf(ptr %0, ...)

define i64 @add(i64 %0, i64 %1) {
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

define i64 @subtract(i64 %0, i64 %1) {
entry:
  %x = alloca i64
  store i64 %0, ptr %x
  %y = alloca i64
  store i64 %1, ptr %y
  %2 = load i64, ptr %x
  %3 = load i64, ptr %y
  %4 = sub i64 %2, %3
  ret i64 %4
}

define i64 @multiply(i64 %0, i64 %1) {
entry:
  %m = alloca i64
  store i64 %0, ptr %m
  %n = alloca i64
  store i64 %1, ptr %n
  %2 = load i64, ptr %m
  %3 = load i64, ptr %n
  %4 = mul i64 %2, %3
  ret i64 %4
}

define i32 @main() {
entry:
  %a = alloca i64
  store i64 10, ptr %a
  %b = alloca i64
  store i64 5, ptr %b
  %c = alloca i64
  store i64 2, ptr %c
  %sum = alloca i64
  %0 = load i64, ptr %a
  %1 = load i64, ptr %b
  %2 = call i64 @add(i64 %0, i64 %1)
  store i64 %2, ptr %sum
  %diff = alloca i64
  %3 = load i64, ptr %a
  %4 = load i64, ptr %b
  %5 = call i64 @subtract(i64 %3, i64 %4)
  store i64 %5, ptr %diff
  %product = alloca i64
  %6 = load i64, ptr %sum
  %7 = load i64, ptr %c
  %8 = call i64 @multiply(i64 %6, i64 %7)
  store i64 %8, ptr %product
  store i64 20, ptr %a
  store i64 30, ptr %b
  %9 = load i64, ptr %a
  %10 = load i64, ptr %b
  %11 = call i64 @add(i64 %9, i64 %10)
  store i64 %11, ptr %c
  %final = alloca i64
  %12 = load i64, ptr %a
  %13 = load i64, ptr %b
  %14 = call i64 @add(i64 %12, i64 %13)
  %15 = load i64, ptr %c
  %16 = load i64, ptr %diff
  %17 = call i64 @subtract(i64 %15, i64 %16)
  %18 = call i64 @multiply(i64 %14, i64 %17)
  store i64 %18, ptr %final
  ret i32 0
}
