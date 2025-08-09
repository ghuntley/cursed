target triple = "x86_64-unknown-linux-gnu"

@.str = global [12 x i8] c"Value: %ld\0A\00"
declare i32 @printf(i8*, ...)

define i64 @add(i64 %x, i64 %y) {
entry:
  %result = add i64 %x, %y
  ret i64 %result
}
define i32 @main() {
entry:
  %1 = call i64 @add(i64 2, i64 3)
  %2 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @.str, i32 0, i32 0), i64 %1)
  ret i32 0

}
