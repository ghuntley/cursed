; CURSED Language - Advanced LLVM Compilation
target triple = "x86_64-unknown-linux-gnu"


; Runtime function declarations
declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)
declare i8* @malloc(i64)
declare void @free(i8*)
declare i64 @strlen(i8*)
declare i8* @strcpy(i8*, i8*)
declare i8* @i32_to_string(i32)
declare i8* @string_concat(i8*, i8*)

; CURSED runtime functions
declare void @cursed_panic(i8*, i64)
declare i8* @cursed_alloc(i64)
declare void @cursed_free(i8*)
declare i32 @cursed_goroutine_spawn(i8*)
declare void @cursed_channel_send(i8*, i8*)
declare i8* @cursed_channel_receive(i8*)

; Exception handling declarations
declare i32 @__gxx_personality_v0(...)
declare i8* @__cxa_begin_catch(i8*)
declare void @__cxa_end_catch()
declare void @__cxa_rethrow()
declare i8* @__cxa_allocate_exception(i64)
declare void @__cxa_throw(i8*, i8*, i8*)
declare i8* @_Unwind_GetLanguageSpecificData(i8*)
declare i32 @_Unwind_GetRegionStart(i8*)
declare i32 @_Unwind_GetDataRelBase(i8*)
declare i32 @_Unwind_GetTextRelBase(i8*)

; CURSED exception type info
@_ZTI11CursedError = constant { i8*, i8* } { i8* null, i8* bitcast ([14 x i8]* @_ZTS11CursedError to i8*) }
@_ZTS11CursedError = constant [14 x i8] c"11CursedError\00"

define i32 @test() {
entry:
  %0 = alloca [3x i32], align 4
  %1 = getelementptr inbounds [3x i32], [3x i32]* %0, i64 0, i64 0
  store i32 1, i32* %1, align 4
  %2 = getelementptr inbounds [3x i32], [3x i32]* %0, i64 0, i64 1
  store i32 2, i32* %2, align 4
  %3 = getelementptr inbounds [3x i32], [3x i32]* %0, i64 0, i64 2
  store i32 3, i32* %3, align 4
  %4 = alloca i8*, align 4
  store i8* %0, i8** %4, align 4
  ; Variable numbers allocated
  %6 = load i8*, i8** %4, align 4
  %7 = getelementptr inbounds [5 x i32], [5 x i32]* %6, i64 0, i64 0
  %8 = load i32, i32* %7, align 4
  %9 = alloca i32, align 4
  store i32 %8, i32* %9, align 4
  ; Variable first allocated
  %10 = load i32, i32* %9, align 4
  %11 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.0, i64 0, i64 0
  %12 = call i32 (i8*, ...) @printf(i8* %11, i32 %10)
  %13 = add i32 0, 0
  ; Expression result: %13
  ret i32 0
}



; String constants
@.str.0 = private unnamed_addr constant [6 x i8] c"%d\0A\00", align 1
define i32 @main() {
  ret i32 0
}
