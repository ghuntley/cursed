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
@.str.0 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
@.str.6 = private unnamed_addr constant [11 x i8] c"Character:\00", align 1
@.str.3 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.5 = private unnamed_addr constant [9 x i8] c"Boolean:\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"String:\00", align 1
@.str.1 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.2 = private unnamed_addr constant [9 x i8] c"Integer:\00", align 1
define i32 @main() {
  %0 = alloca double, align 4
  store double 42.5, double* %0, align 4
  ; Variable x allocated at %0
  %1 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.0, i64 0, i64 0
  %2 = alloca i8*, align 4
  store i8* %1, i8** %2, align 4
  ; Variable name allocated at %2
  %3 = alloca i1, align 4
  store i1 1, i1* %3, align 4
  ; Variable flag allocated at %3
  %4 = alloca i32, align 4
  store i32 null, i32* %4, align 4
  ; Variable ch allocated at %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %7 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.2, i64 0, i64 0
  ; Converting complex expression to output
  %8 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %9 = call i32 (i8*, ...) @printf(i8* %8, i32 %7)
  %10 = load i32, i32* %0, align 4
  %11 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %12 = call i32 (i8*, ...) @printf(i8* %11, i32 %10)
  %13 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.4, i64 0, i64 0
  ; Converting complex expression to output
  %14 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %15 = call i32 (i8*, ...) @printf(i8* %14, i32 %13)
  %16 = load i32, i32* %2, align 4
  %17 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %18 = call i32 (i8*, ...) @printf(i8* %17, i32 %16)
  %19 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.5, i64 0, i64 0
  ; Converting complex expression to output
  %20 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %21 = call i32 (i8*, ...) @printf(i8* %20, i32 %19)
  %22 = load i1, i1* %3, align 4
  %23 = zext i1 %22 to i32
  %24 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %25 = call i32 (i8*, ...) @printf(i8* %24, i32 %23)
  %26 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.6, i64 0, i64 0
  ; Converting complex expression to output
  %27 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %28 = call i32 (i8*, ...) @printf(i8* %27, i32 %26)
  %29 = load i32, i32* %4, align 4
  %30 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %31 = call i32 (i8*, ...) @printf(i8* %30, i32 %29)
  ret i32 0
}
