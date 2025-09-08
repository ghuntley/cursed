; ModuleID = 'test_no_stdlib'
source_filename = "test_no_stdlib"

declare void @cursed_runtime_spill_string(ptr %0)

declare void @cursed_runtime_spill_int(i64 %0)

declare i32 @printf(ptr %0, ...)

define i32 @main() {
entry:
  ret i32 0
}
