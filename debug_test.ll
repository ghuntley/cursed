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

define i8* @simple_function(i8* %param) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %1
  ret i32 0
}

define i8* @test_params(i8* %name, i8* %value) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.2, i64 0, i64 0
  %2 = icmp eq i32 %name, %1
  ; Expression result: %2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %3
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %4
  %5 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.3, i64 0, i64 0
  ; Expression result: %5
  ret i32 0
}



; String constants
@.str.7 = private unnamed_addr constant [8 x i8] c"default\00", align 1
@.str.8 = private unnamed_addr constant [27 x i8] c"Testing parameter function\00", align 1
@.str.1 = private unnamed_addr constant [7 x i8] c"result\00", align 1
@.str.3 = private unnamed_addr constant [5 x i8] c"pong\00", align 1
@.str.5 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.6 = private unnamed_addr constant [5 x i8] c"test\00", align 1
@.str.2 = private unnamed_addr constant [5 x i8] c"ping\00", align 1
@.str.0 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.4 = private unnamed_addr constant [24 x i8] c"Testing simple function\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.4, i64 0, i64 0
  ; Converting complex expression to output
  %1 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.5, i64 0, i64 0
  %2 = call i32 (i8*, ...) @printf(i8* %1, i32 %0)
  %3 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.6, i64 0, i64 0
  %4 = call i32 @simple_function(i32 %3)
  %5 = alloca i8*, align 4
  store i8* %4, i8** %5, align 4
  ; Variable result allocated at %5
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %7 = load i32, i32* %5, align 4
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %10 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.7, i64 0, i64 0
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %12 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.8, i64 0, i64 0
  ; Converting complex expression to output
  %13 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.5, i64 0, i64 0
  %14 = call i32 (i8*, ...) @printf(i8* %13, i32 %12)
  %15 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.2, i64 0, i64 0
  %16 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.6, i64 0, i64 0
  %17 = call i32 @test_params(i32 %15, i32 %16)
  %18 = alloca i8*, align 4
  store i8* %17, i8** %18, align 4
  ; Variable test_result allocated at %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %20 = load i32, i32* %18, align 4
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ret i32 0
}
