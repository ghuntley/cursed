; ModuleID = 'final_test.csd'
source_filename = "final_test.csd"

@welcome = global [22 x i8] c"CURSED Compiler Works!"
@version = global i64 1

define i32 @main() {
entry:
  ret i32 0
}

define i32 @main.1() {
entry:
  %result = alloca i64, align 8
  store i64 42, ptr %result, align 4
  ret i32 0
}
