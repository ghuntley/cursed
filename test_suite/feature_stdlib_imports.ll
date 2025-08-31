; ModuleID = 'feature_stdlib_imports'
source_filename = "feature_stdlib_imports"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0
@.str.0 = private constant [9 x i8] c"\22CURSED\22\00"
@.str.1 = private constant [63 x i8] c"\22Nested module calls: mathz.max(stringz.length('{}'), 3) = {}\22\00"

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

declare i32 @yap(ptr)

define i32 @main() {
entry:
  %processed = alloca i64, align 8
  %text = alloca i64, align 8
  %call_tmp = call i32 @main()
  ret i32 %call_tmp

unreachable_cont:                                 ; No predecessors!
  store ptr @.str.0, ptr %text, align 8
  store i64 0, ptr %processed, align 4
  %puts_call = call i32 @puts(ptr @.str.1)
  ret i32 0
}
