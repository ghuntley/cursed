; ModuleID = '01_hello_world'
source_filename = "01_hello_world"

declare void @cursed_runtime_spill_string(ptr %0)

declare void @cursed_runtime_spill_int(i64 %0)

declare i32 @printf(ptr %0, ...)

define i32 @main() {
entry:
  ret i32 0
}
