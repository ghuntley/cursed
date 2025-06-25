; ModuleID = 'test_simple.csd'
source_filename = "test_simple.csd"

@msg = global [20 x i8] c"Hello, CURSED world!"

define i32 @main() {
entry:
  ret i32 0
}

define void @greet() {
entry:
  ret void
}
