target triple = "x86_64-unknown-linux-gnu"

@.str = global [5 x i8] c"%ld\0A\00"
declare i32 @printf(i8*, ...)

define i32 @main() {
entry:
  %1 = alloca i64
  store i64 10, i64* %1
  %2 = load i64, i64* %1
  %3 = add i64 %2, 5
  %4 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.str, i32 0, i32 0), i64 %3)
  ret i32 0
}
