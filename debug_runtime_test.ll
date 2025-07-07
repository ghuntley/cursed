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
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.0 = private unnamed_addr constant [11 x i8] c"Basic test\00", align 1
define i32 @main() {
  %0 = alloca i32, align 4
  store i32 42, i32* %0, align 4
  ; Variable x allocated at %0
  %1 = alloca i32, align 4
  store i32 10, i32* %1, align 4
  ; Variable y allocated at %1
  %2 = load i32, i32* %0, align 4
  %3 = load i32, i32* %1, align 4
  %4 = add i32 %2, %3
  %5 = alloca i32, align 4
  store i32 %4, i32* %5, align 4
  ; Variable result allocated at %5
  %6 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.0, i64 0, i64 0
  ; Converting complex expression to output
  %7 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %8 = call i32 (i8*, ...) @printf(i8* %7, i32 %6)
  %9 = load i32, i32* %5, align 4
  %10 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %11 = call i32 (i8*, ...) @printf(i8* %10, i32 %9)
  ret i32 0
}
