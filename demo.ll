; ModuleID = 'demo_program.csd'
source_filename = "demo_program.csd"

@greeting = global [18 x i8] c"Hello from CURSED!"
@number = global i64 42

define i32 @main() {
entry:
  ret i32 0
}

define void @main.1() {
entry:
  ret void
}

define void @calculate() {
entry:
  ret void
}
