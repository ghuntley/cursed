; ModuleID = 'enhanced_demo.csd'
source_filename = "enhanced_demo.csd"

@message = global [23 x i8] c"Enhanced CURSED Program"
@count = global i64 100

define i32 @main() {
entry:
  ret i32 0
}

define i32 @main.1() {
entry:
  %local_var = alloca i64, align 8
  store i64 42, ptr %local_var, align 4
  ret i32 0
}

define void @helper() {
entry:
  ret void
}
