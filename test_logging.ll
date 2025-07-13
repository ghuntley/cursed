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
declare void @cursed_panic(i8*, i64)
declare i8* @cursed_alloc(i64)
declare void @cursed_free(i8*)
declare i32 @cursed_goroutine_spawn(i8*)
declare void @cursed_channel_send(i8*, i8*)
declare i8* @cursed_channel_receive(i8*)
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
declare i8* @cursed_create_structured_error()
declare i8* @cursed_set_error_message(i8*, i8*)
declare i8* @cursed_set_error_code(i8*, i32)
declare i8* @cursed_set_error_details(i8*, i8*)
declare i8* @cursed_set_error_field(i8*, i8*, i8*)
declare i8* @cursed_get_error_field(i8*, i8*)
declare i32 @cursed_get_error_code(i8*)
declare i8* @cursed_get_error_message(i8*)
declare i8* @cursed_get_error_details(i8*)
declare void @cursed_enhanced_try_begin(i64)
declare void @cursed_enhanced_try_end(i64)
declare i8* @cursed_get_panic_context(i64)
declare i8* @cursed_extract_panic_value(i8*)
declare i8* @cursed_extract_stack_trace(i8*)
declare void @cursed_clear_panic_context(i64)
declare void @cursed_register_panic_handler(i64, i8*)
declare i8* @cursed_handle_panic(i64, i8*)
declare void @cursed_propagate_error_context(i64, i64)
declare i8* @cursed_get_goroutine_error_context(i64)
declare void @cursed_clear_goroutine_error_context(i64)
declare i8* @cursed_create_enhanced_context(i8*, i64)
declare i8* @cursed_link_error_context(i8*, i8*)
declare i8* @cursed_capture_stack_trace()
declare i64 @cursed_get_current_goroutine_id()
declare i64 @time(i64*)
declare i8* @cursed_propagate_with_context(i8*, i8*)
@error_msg_default = private unnamed_addr constant [15 x i8] c"Error occurred\00"

; Module Declarations from Imports
; mod module declarations
declare void @mod_init()
declare void @mod_cleanup()


; String constants
@.str.12 = private unnamed_addr constant [28 x i8] c"Message with default prefix\00", align 1
@.str.6 = private unnamed_addr constant [6 x i8] c"%d\\0A\00", align 1
@.str.18 = private unnamed_addr constant [48 x i8] c"🎯 CURSED logging module is fully functional!\00", align 1
@.str.13 = private unnamed_addr constant [22 x i8] c"Testing named logger:\00", align 1
@.str.1 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.14 = private unnamed_addr constant [9 x i8] c"Database\00", align 1
@.str.15 = private unnamed_addr constant [4 x i8] c"API\00", align 1
@.str.11 = private unnamed_addr constant [9 x i8] c"[CURSED]\00", align 1
@.str.2 = private unnamed_addr constant [33 x i8] c"Testing basic logging functions:\00", align 1
@.str.4 = private unnamed_addr constant [26 x i8] c"This is a warning message\00", align 1
@.str.3 = private unnamed_addr constant [24 x i8] c"This is an info message\00", align 1
@.str.8 = private unnamed_addr constant [30 x i8] c"Testing prefix configuration:\00", align 1
@.str.10 = private unnamed_addr constant [27 x i8] c"Message with custom prefix\00", align 1
@.str.16 = private unnamed_addr constant [6 x i8] c" and \00", align 1
@.str.0 = private unnamed_addr constant [42 x i8] c"🚀 Starting CURSED Logging Module Tests\00", align 1
@.str.5 = private unnamed_addr constant [25 x i8] c"This is an error message\00", align 1
@.str.9 = private unnamed_addr constant [9 x i8] c"[CUSTOM]\00", align 1
@.str.17 = private unnamed_addr constant [46 x i8] c"✅ All logging tests completed successfully!\00", align 1
@.str.7 = private unnamed_addr constant [29 x i8] c"Testing log level constants:\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [42 x i8], [42 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.2, i64 0, i64 0
  %7 = call i32 @puts(i8* %6)
  %8 = add i32 0, 0
  ; Expression result: %8
  %9 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.3, i64 0, i64 0
  %10 = call i32 @logging_log_info(i32 %9)
  %11 = alloca i32, align 4
  store i32 %10, i32* %11, align 4
  ; Variable info_msg allocated
  %12 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.4, i64 0, i64 0
  %13 = call i32 @logging_log_warn(i32 %12)
  %14 = alloca i32, align 4
  store i32 %13, i32* %14, align 4
  ; Variable warn_msg allocated
  %15 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.5, i64 0, i64 0
  %16 = call i32 @logging_log_error(i32 %15)
  %17 = alloca i32, align 4
  store i32 %16, i32* %17, align 4
  ; Variable error_msg allocated
  %18 = load i32, i32* %11, align 4
  %19 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.6, i64 0, i64 0
  %20 = call i32 (i8*, ...) @printf(i8* %19, i32 %18)
  %21 = add i32 0, 0
  ; Expression result: %21
  %22 = load i32, i32* %14, align 4
  %23 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.6, i64 0, i64 0
  %24 = call i32 (i8*, ...) @printf(i8* %23, i32 %22)
  %25 = add i32 0, 0
  ; Expression result: %25
  %26 = load i32, i32* %17, align 4
  %27 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.6, i64 0, i64 0
  %28 = call i32 (i8*, ...) @printf(i8* %27, i32 %26)
  %29 = add i32 0, 0
  ; Expression result: %29
  %30 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %31 = call i32 @puts(i8* %30)
  %32 = add i32 0, 0
  ; Expression result: %32
  %33 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.7, i64 0, i64 0
  %34 = call i32 @puts(i8* %33)
  %35 = add i32 0, 0
  ; Expression result: %35
  %36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %36
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %37
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %38
  ; Expression result: %LOG_INFO
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %39
  %40 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %40
  %41 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %41
  %42 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %42
  %43 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %43
  ; Expression result: %LOG_WARN
  %44 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %44
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %45
  %46 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %46
  %47 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %47
  %48 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %48
  ; Expression result: %LOG_ERROR
  %49 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %49
  %50 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %50
  %51 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %52 = call i32 @puts(i8* %51)
  %53 = add i32 0, 0
  ; Expression result: %53
  %54 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.8, i64 0, i64 0
  %55 = call i32 @puts(i8* %54)
  %56 = add i32 0, 0
  ; Expression result: %56
  %57 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.9, i64 0, i64 0
  %58 = call i32 @logging_set_log_prefix(i32 %57)
  ; Expression result: %58
  %59 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.10, i64 0, i64 0
  %60 = call i32 @logging_log_info(i32 %59)
  %61 = alloca i32, align 4
  store i32 %60, i32* %61, align 4
  ; Variable custom_msg allocated
  %62 = load i32, i32* %61, align 4
  %63 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.6, i64 0, i64 0
  %64 = call i32 (i8*, ...) @printf(i8* %63, i32 %62)
  %65 = add i32 0, 0
  ; Expression result: %65
  %66 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.11, i64 0, i64 0
  %67 = call i32 @logging_set_log_prefix(i32 %66)
  ; Expression result: %67
  %68 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.12, i64 0, i64 0
  %69 = call i32 @logging_log_info(i32 %68)
  %70 = alloca i32, align 4
  store i32 %69, i32* %70, align 4
  ; Variable default_msg allocated
  %71 = load i32, i32* %70, align 4
  %72 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.6, i64 0, i64 0
  %73 = call i32 (i8*, ...) @printf(i8* %72, i32 %71)
  %74 = add i32 0, 0
  ; Expression result: %74
  %75 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %76 = call i32 @puts(i8* %75)
  %77 = add i32 0, 0
  ; Expression result: %77
  %78 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.13, i64 0, i64 0
  %79 = call i32 @puts(i8* %78)
  %80 = add i32 0, 0
  ; Expression result: %80
  %81 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.14, i64 0, i64 0
  %82 = call i32 @logging_create_logger(i32 %81)
  %83 = alloca i32, align 4
  store i32 %82, i32* %83, align 4
  ; Variable db_logger allocated
  %84 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.15, i64 0, i64 0
  %85 = call i32 @logging_create_logger(i32 %84)
  %86 = alloca i32, align 4
  store i32 %85, i32* %86, align 4
  ; Variable api_logger allocated
  %87 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %87
  %88 = load i32, i32* %83, align 4
  %89 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.16, i64 0, i64 0
  %90 = add i32 %88, %89
  %91 = load i32, i32* %86, align 4
  %92 = add i32 %90, %91
  ; Expression result: %92
  %93 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %93
  %94 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %95 = call i32 @puts(i8* %94)
  %96 = add i32 0, 0
  ; Expression result: %96
  %97 = getelementptr inbounds [46 x i8], [46 x i8]* @.str.17, i64 0, i64 0
  %98 = call i32 @puts(i8* %97)
  %99 = add i32 0, 0
  ; Expression result: %99
  %100 = getelementptr inbounds [48 x i8], [48 x i8]* @.str.18, i64 0, i64 0
  %101 = call i32 @puts(i8* %100)
  %102 = add i32 0, 0
  ; Expression result: %102
  ret i32 0
}

