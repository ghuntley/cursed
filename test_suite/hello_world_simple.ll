; ModuleID = 'hello_world_simple'
source_filename = "hello_world_simple"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

declare i32 @yap(ptr)

define void @main_character() {
entry:
  %y = alloca i64, align 8
  %x = alloca i64, align 8
  store i64 42, ptr %x, align 4
  %load_var = load i64, ptr %x, align 4
  %add_tmp = add i64 %load_var, 5
  store i64 %add_tmp, ptr %y, align 4
  ret void
}

define i32 @main() {
entry:
  call void @main_character()
  ret i32 0
}
