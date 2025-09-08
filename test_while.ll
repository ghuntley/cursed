; ModuleID = 'test_while_oracle'
source_filename = "test_while_oracle"

declare void @cursed_runtime_spill_string(ptr %0)

declare void @cursed_runtime_spill_int(i64 %0)

declare i32 @printf(ptr %0, ...)

define i32 @main() {
entry:
  %i = alloca i64
  store i64 1, ptr %i
  %result = alloca i64
  %0 = load i64, ptr %i
  %1 = mul i64 %0, 2
  store i64 %1, ptr %result
  %2 = load i64, ptr %i
  %3 = add i64 %2, 1
  store i64 %3, ptr %i
  %4 = load i64, ptr %i
  %5 = mul i64 %4, 2
  store i64 %5, ptr %result
  %6 = load i64, ptr %i
  %7 = add i64 %6, 1
  store i64 %7, ptr %i
  %8 = load i64, ptr %i
  %9 = mul i64 %8, 2
  store i64 %9, ptr %result
  %10 = load i64, ptr %i
  %11 = add i64 %10, 1
  store i64 %11, ptr %i
  %12 = load i64, ptr %i
  %13 = mul i64 %12, 2
  store i64 %13, ptr %result
  %14 = load i64, ptr %i
  %15 = add i64 %14, 1
  store i64 %15, ptr %i
  %16 = load i64, ptr %i
  %17 = mul i64 %16, 2
  store i64 %17, ptr %result
  %18 = load i64, ptr %i
  %19 = add i64 %18, 1
  store i64 %19, ptr %i
  %20 = load i64, ptr %i
  %21 = mul i64 %20, 2
  store i64 %21, ptr %result
  %22 = load i64, ptr %i
  %23 = add i64 %22, 1
  store i64 %23, ptr %i
  %24 = load i64, ptr %i
  %25 = mul i64 %24, 2
  store i64 %25, ptr %result
  %26 = load i64, ptr %i
  %27 = add i64 %26, 1
  store i64 %27, ptr %i
  %28 = load i64, ptr %i
  %29 = mul i64 %28, 2
  store i64 %29, ptr %result
  %30 = load i64, ptr %i
  %31 = add i64 %30, 1
  store i64 %31, ptr %i
  %32 = load i64, ptr %i
  %33 = mul i64 %32, 2
  store i64 %33, ptr %result
  %34 = load i64, ptr %i
  %35 = add i64 %34, 1
  store i64 %35, ptr %i
  %36 = load i64, ptr %i
  %37 = mul i64 %36, 2
  store i64 %37, ptr %result
  %38 = load i64, ptr %i
  %39 = add i64 %38, 1
  store i64 %39, ptr %i
  %40 = load i64, ptr %i
  %41 = mul i64 %40, 2
  store i64 %41, ptr %result
  %42 = load i64, ptr %i
  %43 = add i64 %42, 1
  store i64 %43, ptr %i
  %44 = load i64, ptr %i
  %45 = mul i64 %44, 2
  store i64 %45, ptr %result
  %46 = load i64, ptr %i
  %47 = add i64 %46, 1
  store i64 %47, ptr %i
  %48 = load i64, ptr %i
  %49 = mul i64 %48, 2
  store i64 %49, ptr %result
  %50 = load i64, ptr %i
  %51 = add i64 %50, 1
  store i64 %51, ptr %i
  %52 = load i64, ptr %i
  %53 = mul i64 %52, 2
  store i64 %53, ptr %result
  %54 = load i64, ptr %i
  %55 = add i64 %54, 1
  store i64 %55, ptr %i
  %56 = load i64, ptr %i
  %57 = mul i64 %56, 2
  store i64 %57, ptr %result
  %58 = load i64, ptr %i
  %59 = add i64 %58, 1
  store i64 %59, ptr %i
  %60 = load i64, ptr %i
  %61 = mul i64 %60, 2
  store i64 %61, ptr %result
  %62 = load i64, ptr %i
  %63 = add i64 %62, 1
  store i64 %63, ptr %i
  %64 = load i64, ptr %i
  %65 = mul i64 %64, 2
  store i64 %65, ptr %result
  %66 = load i64, ptr %i
  %67 = add i64 %66, 1
  store i64 %67, ptr %i
  %68 = load i64, ptr %i
  %69 = mul i64 %68, 2
  store i64 %69, ptr %result
  %70 = load i64, ptr %i
  %71 = add i64 %70, 1
  store i64 %71, ptr %i
  %72 = load i64, ptr %i
  %73 = mul i64 %72, 2
  store i64 %73, ptr %result
  %74 = load i64, ptr %i
  %75 = add i64 %74, 1
  store i64 %75, ptr %i
  %76 = load i64, ptr %i
  %77 = mul i64 %76, 2
  store i64 %77, ptr %result
  %78 = load i64, ptr %i
  %79 = add i64 %78, 1
  store i64 %79, ptr %i
  ret i32 0
}
