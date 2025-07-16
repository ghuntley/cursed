; Check what's being generated
declare i32 @printf(i8*, ...)

define i32 @main() {
entry:
  %my_tuple = alloca { i32, i8* }, align 8
  %1 = getelementptr inbounds { i32, i8* }, { i32, i8* }* %my_tuple, i32 0, i32 0
  store i32 42, i32* %1, align 4
  %2 = getelementptr inbounds { i32, i8* }, { i32, i8* }* %my_tuple, i32 0, i32 1
  store i8* @str0, i8** %2, align 4
  
  ; Load first element
  %3 = getelementptr inbounds { i32, i8* }, { i32, i8* }* %my_tuple, i32 0, i32 0
  %4 = load i32, i32* %3, align 4
  
  ; Print result
  %5 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_int, i32 0, i32 0), i32 %4)
  
  ret i32 0
}

@str0 = constant [6 x i8] c"hello\00"
@fmt_int = constant [4 x i8] c"%d\0A\00"
