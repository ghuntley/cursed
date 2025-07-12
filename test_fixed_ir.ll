target triple = "x86_64-unknown-linux-gnu"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.str = private unnamed_addr constant [12 x i8] c"Hello World\00", align 1

define i32 @main() {
entry:
  call i32 @puts(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @.str, i64 0, i64 0))
  ret i32 0
}
