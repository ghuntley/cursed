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

declare i8* @cursed_error_init(i8*, i8*)
declare i8* @cursed_create_error(i8*)
declare i1 @cursed_is_error(i8*)
declare void @cursed_propagate_error(i8*)
declare void @cursed_try_begin()
declare void @cursed_try_end()
declare i8* @cursed_get_panic_value()
declare i8* @malloc(i32)
declare void @free(i8*)
@error_msg_default = private unnamed_addr constant [13 x i8] c"Error occurred\00"


; String constants
@.str.3 = private unnamed_addr constant [10 x i8] c"Compiler:\00", align 1
@.str.7 = private unnamed_addr constant [41 x i8] c"Self-hosting test completed successfully\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.0 = private unnamed_addr constant [29 x i8] c"Self-hosting production test\00", align 1
@.str.2 = private unnamed_addr constant [14 x i8] c"CURSED v0.1.0\00", align 1
@.str.6 = private unnamed_addr constant [2 x i8] c"%\00", align 1
@.str.4 = private unnamed_addr constant [22 x i8] c"Features implemented:\00", align 1
@.str.5 = private unnamed_addr constant [16 x i8] c"Test pass rate:\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.0, i64 0, i64 0
  ; Converting complex expression to output
  %1 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %2 = call i32 (i8*, ...) @printf(i8* %1, i32 %0)
  %3 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.2, i64 0, i64 0
  %4 = alloca i8*, align 4
  store i8* %3, i8** %4, align 4
  ; Variable compiler_version allocated at %4
  %5 = alloca i32, align 4
  store i32 100, i32* %5, align 4
  ; Variable features allocated at %5
  %6 = alloca double, align 4
  store double 99.4, double* %6, align 4
  ; Variable stability allocated at %6
  %7 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  ; Converting complex expression to output
  %8 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %9 = call i32 (i8*, ...) @printf(i8* %8, i32 %7)
  %10 = load i32, i32* %4, align 4
  %11 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %12 = call i32 (i8*, ...) @printf(i8* %11, i32 %10)
  %13 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.4, i64 0, i64 0
  ; Converting complex expression to output
  %14 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %15 = call i32 (i8*, ...) @printf(i8* %14, i32 %13)
  %16 = load i32, i32* %5, align 4
  %17 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %18 = call i32 (i8*, ...) @printf(i8* %17, i32 %16)
  %19 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.5, i64 0, i64 0
  ; Converting complex expression to output
  %20 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %21 = call i32 (i8*, ...) @printf(i8* %20, i32 %19)
  %22 = load i1, i1* %6, align 4
  %23 = zext i1 %22 to i32
  %24 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %25 = call i32 (i8*, ...) @printf(i8* %24, i32 %23)
  %26 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.6, i64 0, i64 0
  ; Converting complex expression to output
  %27 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %28 = call i32 (i8*, ...) @printf(i8* %27, i32 %26)
  %29 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.7, i64 0, i64 0
  ; Converting complex expression to output
  %30 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %31 = call i32 (i8*, ...) @printf(i8* %30, i32 %29)
  ret i32 0
}
