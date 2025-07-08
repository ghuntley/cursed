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
@.str.4 = private unnamed_addr constant [14 x i8] c"Boolean flag:\00", align 1
@.str.3 = private unnamed_addr constant [12 x i8] c"Variable x:\00", align 1
@.str.0 = private unnamed_addr constant [36 x i8] c"Baseline test - interpretation mode\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.2 = private unnamed_addr constant [1 x i8] c"\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.0, i64 0, i64 0
  ; Converting complex expression to output
  %1 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %2 = call i32 (i8*, ...) @printf(i8* %1, i32 %0)
  %3 = alloca i32, align 4
  store i32 null, i32* %3, align 4
  ; Variable x allocated at %3
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %5 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.3, i64 0, i64 0
  ; Converting complex expression to output
  %6 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %7 = call i32 (i8*, ...) @printf(i8* %6, i32 %5)
  %8 = load i32, i32* %3, align 4
  %9 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %10 = call i32 (i8*, ...) @printf(i8* %9, i32 %8)
  %11 = alloca i1, align 4
  store i1 1, i1* %11, align 4
  ; Variable flag allocated at %11
  %12 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.4, i64 0, i64 0
  ; Converting complex expression to output
  %13 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %14 = call i32 (i8*, ...) @printf(i8* %13, i32 %12)
  %15 = load i1, i1* %11, align 4
  %16 = zext i1 %15 to i32
  %17 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %18 = call i32 (i8*, ...) @printf(i8* %17, i32 %16)
  ret i32 0
}
