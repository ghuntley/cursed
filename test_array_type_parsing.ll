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
declare i8* @char_to_string(i8)
declare i8* @string_concat(i8*, i8*)
declare i8* @tea(i64)
declare i8* @tea_float(double)
declare i8* @tea_bool(i32)

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



; String constants
@.str.0 = private unnamed_addr constant [4 x i8] c"1.0\00", align 1
@.str.3 = private unnamed_addr constant [4 x i8] c"4.0\00", align 1
@.str.4 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"2.0\00", align 1
@.str.2 = private unnamed_addr constant [4 x i8] c"3.0\00", align 1
define i32 @main() {
  %0 = alloca [4 x i32], align 4
  %1 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.0, i64 0, i64 0
  %2 = getelementptr inbounds [4 x i32], [4 x i32]* %0, i64 0, i64 0
  store i32 %1, i32* %2, align 4
  %3 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %4 = getelementptr inbounds [4 x i32], [4 x i32]* %0, i64 0, i64 1
  store i32 %3, i32* %4, align 4
  %5 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %6 = getelementptr inbounds [4 x i32], [4 x i32]* %0, i64 0, i64 2
  store i32 %5, i32* %6, align 4
  %7 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %8 = getelementptr inbounds [4 x i32], [4 x i32]* %0, i64 0, i64 3
  store i32 %7, i32* %8, align 4
  %9 = alloca i32, align 4
  store i32 %0, i32* %9, align 4
  ; Variable arr allocated at %9
  %10 = load i32, i32* %9, align 4
  %11 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.4, i64 0, i64 0
  %12 = call i32 (i8*, ...) @printf(i8* %11, i32 %10)
  ret i32 0
}
