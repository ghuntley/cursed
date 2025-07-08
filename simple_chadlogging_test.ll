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
@.str.3 = private unnamed_addr constant [26 x i8] c"This is a warning message\00", align 1
@.str.0 = private unnamed_addr constant [22 x i8] c"Testing log levels...\00", align 1
@.str.9 = private unnamed_addr constant [11 x i8] c"db_service\00", align 1
@.str.10 = private unnamed_addr constant [27 x i8] c"Testing logger creation...\00", align 1
@.str.16 = private unnamed_addr constant [9 x i8] c"test.log\00", align 1
@.str.15 = private unnamed_addr constant [25 x i8] c"Testing configuration...\00", align 1
@.str.2 = private unnamed_addr constant [24 x i8] c"This is an info message\00", align 1
@.str.17 = private unnamed_addr constant [20 x i8] c"[%level%] %message%\00", align 1
@.str.7 = private unnamed_addr constant [12 x i8] c"user_id=123\00", align 1
@.str.12 = private unnamed_addr constant [23 x i8] c"Authentication warning\00", align 1
@.str.11 = private unnamed_addr constant [5 x i8] c"auth\00", align 1
@.str.14 = private unnamed_addr constant [15 x i8] c"test_operation\00", align 1
@.str.8 = private unnamed_addr constant [15 x i8] c"Database error\00", align 1
@.str.4 = private unnamed_addr constant [25 x i8] c"This is an error message\00", align 1
@.str.18 = private unnamed_addr constant [47 x i8] c"chadlogging basic test completed successfully!\00", align 1
@.str.13 = private unnamed_addr constant [31 x i8] c"Testing performance logging...\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.6 = private unnamed_addr constant [12 x i8] c"User action\00", align 1
@.str.5 = private unnamed_addr constant [30 x i8] c"Testing structured logging...\00", align 1
define i32 @main() {
  %0 = call i32 @init_logging()
  %1 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.0, i64 0, i64 0
  ; Converting complex expression to output
  %2 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %3 = call i32 (i8*, ...) @printf(i8* %2, i32 %1)
  %4 = call i32 @set_log_level(i32 %LOG_INFO)
  %5 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.2, i64 0, i64 0
  %6 = call i32 @info(i32 %5)
  %7 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.3, i64 0, i64 0
  %8 = call i32 @warn(i32 %7)
  %9 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.4, i64 0, i64 0
  %10 = call i32 @error(i32 %9)
  %11 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.5, i64 0, i64 0
  ; Converting complex expression to output
  %12 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %13 = call i32 (i8*, ...) @printf(i8* %12, i32 %11)
  %14 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.6, i64 0, i64 0
  %15 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.7, i64 0, i64 0
  %16 = call i32 @log_with_fields(i32 %LOG_INFO, i32 %14, i32 %15)
  %17 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.8, i64 0, i64 0
  %18 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.9, i64 0, i64 0
  %19 = call i32 @log_with_context(i32 %LOG_ERROR, i32 %17, i32 %18)
  %20 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.10, i64 0, i64 0
  ; Converting complex expression to output
  %21 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %22 = call i32 (i8*, ...) @printf(i8* %21, i32 %20)
  %23 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.11, i64 0, i64 0
  %24 = call i32 @create_logger(i32 %23, i32 %LOG_WARN)
  %25 = alloca i8*, align 4
  store i8* %24, i8** %25, align 4
  ; Variable auth_logger allocated at %25
  %26 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.11, i64 0, i64 0
  %27 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.12, i64 0, i64 0
  %28 = call i32 @log_with_logger(i32 %26, i32 %LOG_WARN, i32 %27)
  %29 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.13, i64 0, i64 0
  ; Converting complex expression to output
  %30 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %31 = call i32 (i8*, ...) @printf(i8* %30, i32 %29)
  %32 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.14, i64 0, i64 0
  %33 = call i32 @perf_start(i32 %32)
  %34 = alloca i32, align 4
  store i32 %33, i32* %34, align 4
  ; Variable start_time allocated at %34
  %35 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.14, i64 0, i64 0
  %36 = load i32, i32* %34, align 4
  %37 = call i32 @perf_end(i32 %35, i32 %36)
  %38 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.15, i64 0, i64 0
  ; Converting complex expression to output
  %39 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %40 = call i32 (i8*, ...) @printf(i8* %39, i32 %38)
  %41 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.16, i64 0, i64 0
  %42 = call i32 @set_log_file(i32 %41)
  %43 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.17, i64 0, i64 0
  %44 = call i32 @set_log_format(i32 %43)
  %45 = call i32 @set_max_log_size(i32 1048576)
  %46 = call i32 @cleanup_logging()
  %47 = getelementptr inbounds [47 x i8], [47 x i8]* @.str.18, i64 0, i64 0
  ; Converting complex expression to output
  %48 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %49 = call i32 (i8*, ...) @printf(i8* %48, i32 %47)
  ret i32 0
}
