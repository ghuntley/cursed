; ModuleID = 'debug_function_call'
source_filename = "debug_function_call"

declare void @cursed_runtime_spill_string(ptr %0)

declare void @cursed_runtime_spill_int(i64 %0)

declare i32 @printf(ptr %0, ...)

define i64 @simple_add(i64 %0, i64 %1) {
entry:
  %a = alloca i64
  store i64 %0, ptr %a
  %b = alloca i64
  store i64 %1, ptr %b
  %2 = load i64, ptr %a
  %3 = load i64, ptr %b
  %4 = add i64 %2, %3
  ret i64 %4
  ret i64 0
}

define i32 @main() {
entry:
  %0 = alloca i64
  ret i32 0
}
