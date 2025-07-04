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

  %2 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.0, i64 0, i64 0
  call i32 @puts(i8* %2)
  %1 = add i32 0, 0

; String constants
@.str.0 = private unnamed_addr constant [24 x i8] c"x is not greater than 5\00", align 1

define i32 @main() {
  ret i32 0
}
