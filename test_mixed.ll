; ModuleID = '01_mixed_types'
source_filename = "01_mixed_types"

declare void @cursed_runtime_spill_string(ptr %0)

declare void @cursed_runtime_spill_int(i64 %0)

declare i32 @printf(ptr %0, ...)

define i32 @main() {
entry:
  %int_val = alloca i64
  store i64 10, ptr %int_val
  %float_val = alloca i64
  store double 0x400C000000000000, ptr %float_val
  ret i32 0
}
