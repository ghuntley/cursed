; ModuleID = 'debug_simple_while'
source_filename = "debug_simple_while"

declare void @cursed_runtime_spill_string(ptr %0)

declare void @cursed_runtime_spill_int(i64 %0)

declare i32 @printf(ptr %0, ...)

define void @test_simple(i64 %0) {
entry:
  %n = alloca i64
  store i64 %0, ptr %n
  %i = alloca i64
  store i64 1, ptr %i
}

define i32 @main() {
entry:
  call void @test_simple(i64 3)
  ret i32 0
}
