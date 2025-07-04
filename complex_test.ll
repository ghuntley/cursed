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
  %0 = load i32, i32* %x, align 4
  %1 = load i32, i32* %y, align 4
  %2 = add i32 %0, %1
  ret i32 %2
}

define i32 @main() {
entry:
  %0 = call i32 @add(i32 5, i32 10)
  %1 = alloca i32, align 4
  store i32 %0, i32* %1, align 4
  ; Variable result allocated
  %2 = load i32, i32* %1, align 4
  ret i32 %2
}

