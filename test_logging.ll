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
define i32 @test_basic_logging() {
entry:
  %0 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.1, i64 0, i64 0
  %3 = call i32 @logging_log_trace(i32 %2)
  ; Expression result: %3
  %4 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.2, i64 0, i64 0
  %5 = call i32 @logging_log_debug(i32 %4)
  ; Expression result: %5
  %6 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.3, i64 0, i64 0
  %7 = call i32 @logging_log_info(i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.4, i64 0, i64 0
  %9 = call i32 @logging_log_warn(i32 %8)
  ; Expression result: %9
  %10 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.5, i64 0, i64 0
  %11 = call i32 @logging_log_error(i32 %10)
  ; Expression result: %11
  %12 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.6, i64 0, i64 0
  %13 = call i32 @logging_log_fatal(i32 %12)
  ; Expression result: %13
  %14 = call i32 @assert_true(i32 1)
  ; Expression result: %14
  ret i32 0
}

define i32 @test_log_level_filtering() {
entry:
  %0 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.7, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  ; Member access: %logging.LOG_WARN
  %2 = getelementptr inbounds %struct.object, %struct.object* %logging, i32 0, i32 0
  %3 = load i32, i32* %2, align 4
  %4 = call i32 @logging_set_log_level(i32 %3)
  ; Expression result: %4
  %5 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.8, i64 0, i64 0
  %6 = call i32 @logging_log_debug(i32 %5)
  ; Expression result: %6
  %7 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.8, i64 0, i64 0
  %8 = call i32 @logging_log_info(i32 %7)
  ; Expression result: %8
  %9 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.9, i64 0, i64 0
  %10 = call i32 @logging_log_warn(i32 %9)
  ; Expression result: %10
  %11 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.9, i64 0, i64 0
  %12 = call i32 @logging_log_error(i32 %11)
  ; Expression result: %12
  ; Member access: %logging.LOG_INFO
  %13 = getelementptr inbounds %struct.object, %struct.object* %logging, i32 0, i32 0
  %14 = load i32, i32* %13, align 4
  %15 = call i32 @logging_set_log_level(i32 %14)
  ; Expression result: %15
  %16 = call i32 @assert_true(i32 1)
  ; Expression result: %16
  ret i32 0
}

define i32 @test_log_formatting() {
entry:
  %0 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.10, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.11, i64 0, i64 0
  %3 = call i32 @logging_set_log_format(i32 %2)
  ; Expression result: %3
  %4 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.12, i64 0, i64 0
  %5 = call i32 @logging_log_info(i32 %4)
  ; Expression result: %5
  %6 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.13, i64 0, i64 0
  %7 = call i32 @logging_set_log_format(i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.14, i64 0, i64 0
  %9 = call i32 @logging_log_info(i32 %8)
  ; Expression result: %9
  %10 = call i32 @assert_true(i32 1)
  ; Expression result: %10
  ret i32 0
}

define i8* @test_structured_logging() {
entry:
  %0 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.15, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = add i32 0, 0 ; literal placeholder
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable fields allocated
  %4 = alloca [0x i32], align 4
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %5
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %6
  ; Expression result: %map
  %7 = alloca [0x i32], align 4
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %9
  %10 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.17, i64 0, i64 0
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %11
  %12 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.18, i64 0, i64 0
  ; Expression result: %12
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %13
  %14 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.19, i64 0, i64 0
  ; Expression result: %14
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %15
  %16 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.20, i64 0, i64 0
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %17
  %18 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.21, i64 0, i64 0
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %19
  %20 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.22, i64 0, i64 0
  ; Expression result: %20
  ret i32 0
}

define i32 @test_named_logger() {
entry:
  %0 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.23, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = add i32 0, 0 ; literal placeholder
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable logger allocated
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %4
  ; Expression result: %Logger
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %5
  %6 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.24, i64 0, i64 0
  %7 = call i32 @logging_create_logger(i32 %6)
  ; Expression result: %7
  %8 = load i8*, i8** %3, align 4
  %9 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.25, i64 0, i64 0
  %10 = call i32 @logging_logger_info(i32 %8, i32 %9)
  ; Expression result: %10
  %11 = load i8*, i8** %3, align 4
  %12 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.26, i64 0, i64 0
  %13 = call i32 @logging_logger_warn(i32 %11, i32 %12)
  ; Expression result: %13
  %14 = load i8*, i8** %3, align 4
  %15 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.27, i64 0, i64 0
  %16 = call i32 @logging_logger_error(i32 %14, i32 %15)
  ; Expression result: %16
  %17 = call i32 @assert_true(i32 1)
  ; Expression result: %17
  ret i32 0
}

define i32 @test_performance_logging() {
entry:
  %0 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.28, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.29, i64 0, i64 0
  %3 = call i32 @logging_log_performance(i32 %2, i32 150)
  ; Expression result: %3
  %4 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.30, i64 0, i64 0
  %5 = call i32 @logging_log_performance(i32 %4, i32 2500)
  ; Expression result: %5
  %6 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.31, i64 0, i64 0
  %7 = call i32 @logging_log_memory_usage(i32 %6, i32 1024000)
  ; Expression result: %7
  %8 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.32, i64 0, i64 0
  %9 = call i32 @logging_log_memory_usage(i32 %8, i32 512000)
  ; Expression result: %9
  %10 = call i32 @assert_true(i32 1)
  ; Expression result: %10
  ret i32 0
}

define i32 @test_error_logging() {
entry:
  %0 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.33, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [44 x i8], [44 x i8]* @.str.34, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable stack_trace allocated
  %4 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.35, i64 0, i64 0
  %5 = load i8*, i8** %3, align 4
  %6 = call i32 @logging_log_error_with_stack(i32 %4, i32 %5)
  ; Expression result: %6
  %7 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.36, i64 0, i64 0
  %8 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.37, i64 0, i64 0
  %9 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.38, i64 0, i64 0
  %10 = call i32 @logging_log_exception(i32 %7, i32 %8, i32 %9)
  ; Expression result: %10
  %11 = call i32 @assert_true(i32 1)
  ; Expression result: %11
  ret i32 0
}

define i32 @test_conditional_logging() {
entry:
  %0 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.39, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = alloca i1, align 4
  store i1 1, i1* %2, align 4
  ; Variable debug_enabled allocated
  %3 = load i1, i1* %2, align 4
  ; Member access: %logging.LOG_DEBUG
  %4 = getelementptr inbounds %struct.object, %struct.object* %logging, i32 0, i32 0
  %5 = load i32, i32* %4, align 4
  %6 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.40, i64 0, i64 0
  %7 = call i32 @logging_log_if(i32 %3, i32 %5, i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  %9 = alloca i8*, align 4
  store i8* %8, i8** %9, align 4
  ; Variable error_condition allocated
  %10 = load i8*, i8** %9, align 4
  ; Member access: %logging.LOG_ERROR
  %11 = getelementptr inbounds %struct.object, %struct.object* %logging, i32 0, i32 0
  %12 = load i32, i32* %11, align 4
  %13 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.8, i64 0, i64 0
  %14 = call i32 @logging_log_if(i32 %10, i32 %12, i32 %13)
  ; Expression result: %14
  %15 = call i32 @assert_true(i32 1)
  ; Expression result: %15
  ret i32 0
}

define i32 @test_debug_utilities() {
entry:
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.41, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.42, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable test_variable allocated
  %4 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.43, i64 0, i64 0
  %5 = load i8*, i8** %3, align 4
  %6 = call i32 @logging_log_variable(i32 %4, i32 %5)
  ; Expression result: %6
  %7 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.44, i64 0, i64 0
  %8 = call i32 @logging_log_function_entry(i32 %7)
  ; Expression result: %8
  %9 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.45, i64 0, i64 0
  %10 = call i32 @logging_log_checkpoint(i32 %9)
  ; Expression result: %10
  %11 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.44, i64 0, i64 0
  %12 = call i32 @logging_log_function_exit(i32 %11)
  ; Expression result: %12
  %13 = call i32 @assert_true(i32 1)
  ; Expression result: %13
  ret i32 0
}

define i32 @test_log_constants() {
entry:
  %0 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.46, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  ; Member access: %logging.LOG_TRACE
  %2 = getelementptr inbounds %struct.object, %struct.object* %logging, i32 0, i32 0
  %3 = load i32, i32* %2, align 4
  %4 = call i32 @assert_eq_int(i32 %3, i32 0)
  ; Expression result: %4
  ; Member access: %logging.LOG_DEBUG
  %5 = getelementptr inbounds %struct.object, %struct.object* %logging, i32 0, i32 0
  %6 = load i32, i32* %5, align 4
  %7 = call i32 @assert_eq_int(i32 %6, i32 1)
  ; Expression result: %7
  ; Member access: %logging.LOG_INFO
  %8 = getelementptr inbounds %struct.object, %struct.object* %logging, i32 0, i32 0
  %9 = load i32, i32* %8, align 4
  %10 = call i32 @assert_eq_int(i32 %9, i32 2)
  ; Expression result: %10
  ; Member access: %logging.LOG_WARN
  %11 = getelementptr inbounds %struct.object, %struct.object* %logging, i32 0, i32 0
  %12 = load i32, i32* %11, align 4
  %13 = call i32 @assert_eq_int(i32 %12, i32 3)
  ; Expression result: %13
  ; Member access: %logging.LOG_ERROR
  %14 = getelementptr inbounds %struct.object, %struct.object* %logging, i32 0, i32 0
  %15 = load i32, i32* %14, align 4
  %16 = call i32 @assert_eq_int(i32 %15, i32 4)
  ; Expression result: %16
  ; Member access: %logging.LOG_FATAL
  %17 = getelementptr inbounds %struct.object, %struct.object* %logging, i32 0, i32 0
  %18 = load i32, i32* %17, align 4
  %19 = call i32 @assert_eq_int(i32 %18, i32 5)
  ; Expression result: %19
  ret i32 0
}

define i32 @test_log_configuration() {
entry:
  %0 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.47, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.48, i64 0, i64 0
  %3 = call i32 @logging_set_log_file(i32 %2)
  ; Expression result: %3
  %4 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.49, i64 0, i64 0
  %5 = call i32 @logging_log_info(i32 %4)
  ; Expression result: %5
  %6 = add i32 0, 0 ; literal placeholder
  %7 = alloca i8*, align 4
  store i8* %6, i8** %7, align 4
  ; Variable file_size allocated
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %9
  %10 = call i32 @logging_get_log_file_size()
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %11
  ; Expression result: 0
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %12
  %13 = call i32 @logging_rotate_log_file()
  ; Expression result: %13
  %14 = call i32 @logging_clear_log_file()
  ; Expression result: %14
  %15 = call i32 @assert_true(i32 1)
  ; Expression result: %15
  ret i32 0
}

define i32 @test_logger_methods() {
entry:
  %0 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.50, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = add i32 0, 0 ; literal placeholder
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable logger allocated
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %4
  ; Expression result: %Logger
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %5
  %6 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.51, i64 0, i64 0
  %7 = call i32 @logging_create_logger(i32 %6)
  ; Expression result: %7
  %8 = load i8*, i8** %3, align 4
  %9 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.52, i64 0, i64 0
  %10 = call i32 @logging_logger_trace(i32 %8, i32 %9)
  ; Expression result: %10
  %11 = load i8*, i8** %3, align 4
  %12 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.53, i64 0, i64 0
  %13 = call i32 @logging_logger_debug(i32 %11, i32 %12)
  ; Expression result: %13
  %14 = load i8*, i8** %3, align 4
  %15 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.54, i64 0, i64 0
  %16 = call i32 @logging_logger_info(i32 %14, i32 %15)
  ; Expression result: %16
  %17 = load i8*, i8** %3, align 4
  %18 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.55, i64 0, i64 0
  %19 = call i32 @logging_logger_warn(i32 %17, i32 %18)
  ; Expression result: %19
  %20 = load i8*, i8** %3, align 4
  %21 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.56, i64 0, i64 0
  %22 = call i32 @logging_logger_error(i32 %20, i32 %21)
  ; Expression result: %22
  %23 = load i8*, i8** %3, align 4
  %24 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.57, i64 0, i64 0
  %25 = call i32 @logging_logger_fatal(i32 %23, i32 %24)
  ; Expression result: %25
  %26 = call i32 @assert_true(i32 1)
  ; Expression result: %26
  ret i32 0
}

define i32 @test_log_message_formatting() {
entry:
  %0 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.10, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @logging_get_timestamp()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable timestamp allocated
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %4
  ; Expression result: 0
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %5
  %6 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.58, i64 0, i64 0
  %7 = alloca i8*, align 4
  store i8* %6, i8** %7, align 4
  ; Variable original allocated
  %8 = load i8*, i8** %7, align 4
  %9 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.59, i64 0, i64 0
  %10 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.60, i64 0, i64 0
  %11 = call i32 @logging_replace_placeholder(i32 %8, i32 %9, i32 %10)
  %12 = alloca i32, align 4
  store i32 %11, i32* %12, align 4
  ; Variable replaced allocated
  %13 = call i32 @assert_true(i32 1)
  ; Expression result: %13
  ret i32 0
}


; String constants
@.str.7 = private unnamed_addr constant [20 x i8] c"Log Level Filtering\00", align 1
@.str.2 = private unnamed_addr constant [24 x i8] c"This is a debug message\00", align 1
@.str.0 = private unnamed_addr constant [24 x i8] c"Basic Logging Functions\00", align 1
@.str.55 = private unnamed_addr constant [16 x i8] c"Warning message\00", align 1
@.str.10 = private unnamed_addr constant [23 x i8] c"Log Message Formatting\00", align 1
@.str.27 = private unnamed_addr constant [24 x i8] c"Error from named logger\00", align 1
@.str.29 = private unnamed_addr constant [15 x i8] c"database_query\00", align 1
@.str.44 = private unnamed_addr constant [14 x i8] c"test_function\00", align 1
@.str.59 = private unnamed_addr constant [7 x i8] c"{name}\00", align 1
@.str.19 = private unnamed_addr constant [7 x i8] c"action\00", align 1
@.str.54 = private unnamed_addr constant [13 x i8] c"Info message\00", align 1
@.str.21 = private unnamed_addr constant [3 x i8] c"ip\00", align 1
@.str.1 = private unnamed_addr constant [24 x i8] c"This is a trace message\00", align 1
@.str.38 = private unnamed_addr constant [22 x i8] c"data_processor.csd:25\00", align 1
@.str.9 = private unnamed_addr constant [19 x i8] c"This should appear\00", align 1
@.str.14 = private unnamed_addr constant [20 x i8] c"Default format test\00", align 1
@.str.53 = private unnamed_addr constant [14 x i8] c"Debug message\00", align 1
@.str.60 = private unnamed_addr constant [6 x i8] c"World\00", align 1
@.str.30 = private unnamed_addr constant [16 x i8] c"file_processing\00", align 1
@.str.61 = private unnamed_addr constant [37 x i8] c"Starting CURSED Logging Module Tests\00", align 1
@.str.52 = private unnamed_addr constant [14 x i8] c"Trace message\00", align 1
@.str.45 = private unnamed_addr constant [19 x i8] c"middle_of_function\00", align 1
@.str.6 = private unnamed_addr constant [24 x i8] c"This is a fatal message\00", align 1
@.str.31 = private unnamed_addr constant [13 x i8] c"data_loading\00", align 1
@.str.40 = private unnamed_addr constant [22 x i8] c"Debug mode is enabled\00", align 1
@.str.20 = private unnamed_addr constant [6 x i8] c"login\00", align 1
@.str.49 = private unnamed_addr constant [16 x i8] c"Message to file\00", align 1
@.str.37 = private unnamed_addr constant [18 x i8] c"Variable was null\00", align 1
@.str.15 = private unnamed_addr constant [19 x i8] c"Structured Logging\00", align 1
@.str.36 = private unnamed_addr constant [21 x i8] c"NullPointerException\00", align 1
@.str.32 = private unnamed_addr constant [17 x i8] c"cache_allocation\00", align 1
@.str.39 = private unnamed_addr constant [20 x i8] c"Conditional Logging\00", align 1
@.str.56 = private unnamed_addr constant [14 x i8] c"Error message\00", align 1
@.str.18 = private unnamed_addr constant [6 x i8] c"12345\00", align 1
@.str.5 = private unnamed_addr constant [25 x i8] c"This is an error message\00", align 1
@.str.33 = private unnamed_addr constant [24 x i8] c"Error Logging Utilities\00", align 1
@.str.23 = private unnamed_addr constant [22 x i8] c"Named Logger Creation\00", align 1
@.str.24 = private unnamed_addr constant [11 x i8] c"TestLogger\00", align 1
@.str.42 = private unnamed_addr constant [11 x i8] c"test_value\00", align 1
@.str.4 = private unnamed_addr constant [26 x i8] c"This is a warning message\00", align 1
@.str.17 = private unnamed_addr constant [8 x i8] c"user_id\00", align 1
@.str.26 = private unnamed_addr constant [26 x i8] c"Warning from named logger\00", align 1
@.str.43 = private unnamed_addr constant [14 x i8] c"test_variable\00", align 1
@.str.51 = private unnamed_addr constant [11 x i8] c"MethodTest\00", align 1
@.str.47 = private unnamed_addr constant [18 x i8] c"Log Configuration\00", align 1
@.str.57 = private unnamed_addr constant [14 x i8] c"Fatal message\00", align 1
@.str.16 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.34 = private unnamed_addr constant [44 x i8] c"at main() line 42\0Aat process_data() line 15\00", align 1
@.str.12 = private unnamed_addr constant [19 x i8] c"Custom format test\00", align 1
@.str.35 = private unnamed_addr constant [27 x i8] c"Database connection failed\00", align 1
@.str.48 = private unnamed_addr constant [9 x i8] c"test.log\00", align 1
@.str.58 = private unnamed_addr constant [33 x i8] c"Hello {name}, welcome to {place}\00", align 1
@.str.8 = private unnamed_addr constant [23 x i8] c"This should not appear\00", align 1
@.str.25 = private unnamed_addr constant [23 x i8] c"Info from named logger\00", align 1
@.str.41 = private unnamed_addr constant [16 x i8] c"Debug Utilities\00", align 1
@.str.3 = private unnamed_addr constant [24 x i8] c"This is an info message\00", align 1
@.str.50 = private unnamed_addr constant [24 x i8] c"Logger Instance Methods\00", align 1
@.str.11 = private unnamed_addr constant [19 x i8] c"{level}: {message}\00", align 1
@.str.46 = private unnamed_addr constant [20 x i8] c"Log Level Constants\00", align 1
@.str.13 = private unnamed_addr constant [33 x i8] c"[{timestamp}] {level}: {message}\00", align 1
@.str.28 = private unnamed_addr constant [20 x i8] c"Performance Logging\00", align 1
@.str.22 = private unnamed_addr constant [12 x i8] c"192.168.1.1\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.61, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = call i32 @test_basic_logging()
  ; Expression result: %3
  %4 = call i32 @test_log_level_filtering()
  ; Expression result: %4
  %5 = call i32 @test_log_formatting()
  ; Expression result: %5
  %6 = call i32 @test_structured_logging()
  ; Expression result: %6
  %7 = call i32 @test_named_logger()
  ; Expression result: %7
  %8 = call i32 @test_performance_logging()
  ; Expression result: %8
  %9 = call i32 @test_error_logging()
  ; Expression result: %9
  %10 = call i32 @test_conditional_logging()
  ; Expression result: %10
  %11 = call i32 @test_debug_utilities()
  ; Expression result: %11
  %12 = call i32 @test_log_constants()
  ; Expression result: %12
  %13 = call i32 @test_log_configuration()
  ; Expression result: %13
  %14 = call i32 @test_logger_methods()
  ; Expression result: %14
  %15 = call i32 @test_log_message_formatting()
  ; Expression result: %15
  %16 = call i32 @print_test_summary()
  ; Expression result: %16
  ret i32 0
}

