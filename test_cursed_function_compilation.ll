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

define i32 @add(i32 %a, i32 %b) {
  %0 = add i32 %a, %b
  ret i32 %0
  ret i32 0
}

define i32 @main() {
  %1 = call i32 @add(i32 5, i32 3)
  ; Variable: result = %1
  call i32 @puts(i8* %result)
  %3 = add i32 0, 0 ; stdlib call result
  ret i32 0
  ret i32 0
}

