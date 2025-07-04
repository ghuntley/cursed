; CURSED Language - Advanced LLVM Compilation
target triple = "x86_64-unknown-linux-gnu"


; Runtime function declarations
declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)
declare i8* @malloc(i64)
declare void @free(i8*)
declare i64 @strlen(i8*)
declare i8* @strcpy(i8*, i8*)

; CURSED runtime functions
declare void @cursed_panic(i8*, i64)
declare i8* @cursed_alloc(i64)
declare void @cursed_free(i8*)
declare i32 @cursed_goroutine_spawn(i8*)
declare void @cursed_channel_send(i8*, i8*)
declare i8* @cursed_channel_receive(i8*)

define i32 @add(i32 %x, i32 %y) {
entry:
  %0 = add i32 %x, %y
  ret i32 %0
}

define i32 @main() {
entry:
  %0 = call i32 @add(i32 5, i32 3)
  %1 = alloca i32, align 4
  store i32 %0, i32* %1, align 4
  ; Variable result allocated
  %2 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.0, i64 0, i64 0
  %3 = call i32 @puts(i8* %2)
  %4 = add i32 0, 0
  ; Expression result: %4
  %5 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.fmt.d.1, i64 0, i64 0
  call i32 (i8*, ...) @printf(i8* %5, i32 %1)
  %6 = add i32 0, 0
  ; Expression result: %6
  ret i32 0
}


; String constants
@.str.0 = private unnamed_addr constant [9 x i8] c"Result: \00", align 1
@.str.fmt.d.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
