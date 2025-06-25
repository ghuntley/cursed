; ModuleID = 'test_simple.csd'
source_filename = "test_simple.csd"

@msg = global [5 x i8] c"Hello"

define i32 @main() {
entry:
  ret i32 0
}

define i32 @main.1() {
entry:
  ret i32 0
}
