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


; String constants
@.str.0 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
define i32 @main() {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = alloca i8*, align 4
  store i8* %0, i8** %1, align 4
  ; Variable person allocated
  %2 = load i32, i32* %1, align 4
  ; Member access: %2.name
  %3 = getelementptr inbounds %struct.object, %struct.object* %2, i32 0, i32 0
  %4 = load i32, i32* %3, align 4
  %5 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.fmt.d.0, i64 0, i64 0
  %6 = call i32 (i8*, ...) @printf(i8* %5, i32 %4)
  %7 = add i32 0, 0
  ; Expression result: %7
  ret i32 0
}

