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
@.str.36 = private unnamed_addr constant [107 x i8] c"Very long message that might test the limits of the logging system and see how it handles extended content\00", align 1
@.str.27 = private unnamed_addr constant [18 x i8] c"Connection failed\00", align 1
@.str.59 = private unnamed_addr constant [19 x i8] c"System initialized\00", align 1
@.str.42 = private unnamed_addr constant [38 x i8] c"user_id=456 amount=99.99 currency=USD\00", align 1
@.str.35 = private unnamed_addr constant [11 x i8] c"Edge cases\00", align 1
@.str.40 = private unnamed_addr constant [27 x i8] c"Complex structured logging\00", align 1
@.str.33 = private unnamed_addr constant [15 x i8] c"Log statistics\00", align 1
@.str.0 = private unnamed_addr constant [16 x i8] c"Log level names\00", align 1
@.str.44 = private unnamed_addr constant [17 x i8] c"database_monitor\00", align 1
@.str.17 = private unnamed_addr constant [12 x i8] c"user_id=123\00", align 1
@.str.1 = private unnamed_addr constant [6 x i8] c"DEBUG\00", align 1
@.str.15 = private unnamed_addr constant [19 x i8] c"Structured logging\00", align 1
@.str.24 = private unnamed_addr constant [12 x i8] c"test_logger\00", align 1
@.str.9 = private unnamed_addr constant [14 x i8] c"Debug message\00", align 1
@.str.11 = private unnamed_addr constant [16 x i8] c"Warning message\00", align 1
@.str.52 = private unnamed_addr constant [15 x i8] c"Database error\00", align 1
@.str.3 = private unnamed_addr constant [5 x i8] c"WARN\00", align 1
@.str.50 = private unnamed_addr constant [3 x i8] c"db\00", align 1
@.str.46 = private unnamed_addr constant [12 x i8] c"api_request\00", align 1
@.str.34 = private unnamed_addr constant [22 x i8] c"System initialization\00", align 1
@.str.5 = private unnamed_addr constant [14 x i8] c"Set log level\00", align 1
@.str.49 = private unnamed_addr constant [5 x i8] c"auth\00", align 1
@.str.56 = private unnamed_addr constant [10 x i8] c"Warn test\00", align 1
@.str.51 = private unnamed_addr constant [23 x i8] c"Authentication warning\00", align 1
@.str.25 = private unnamed_addr constant [24 x i8] c"Logger-specific logging\00", align 1
@.str.55 = private unnamed_addr constant [10 x i8] c"Info test\00", align 1
@.str.22 = private unnamed_addr constant [20 x i8] c"[%level%] %message%\00", align 1
@.str.28 = private unnamed_addr constant [12 x i8] c"auth_logger\00", align 1
@.str.29 = private unnamed_addr constant [14 x i8] c"Invalid token\00", align 1
@.str.47 = private unnamed_addr constant [25 x i8] c"Configuration edge cases\00", align 1
@.str.45 = private unnamed_addr constant [24 x i8] c"Performance measurement\00", align 1
@.str.43 = private unnamed_addr constant [20 x i8] c"Slow query detected\00", align 1
@.str.60 = private unnamed_addr constant [16 x i8] c"Buffer flushing\00", align 1
@.str.2 = private unnamed_addr constant [5 x i8] c"INFO\00", align 1
@.str.26 = private unnamed_addr constant [10 x i8] c"db_logger\00", align 1
@.str.38 = private unnamed_addr constant [19 x i8] c"Timestamp function\00", align 1
@.str.39 = private unnamed_addr constant [28 x i8] c"Log filtering comprehensive\00", align 1
@.str.19 = private unnamed_addr constant [11 x i8] c"db_service\00", align 1
@.str.12 = private unnamed_addr constant [14 x i8] c"Error message\00", align 1
@.str.23 = private unnamed_addr constant [16 x i8] c"Logger creation\00", align 1
@.str.41 = private unnamed_addr constant [15 x i8] c"Payment failed\00", align 1
@.str.21 = private unnamed_addr constant [9 x i8] c"test.log\00", align 1
@.str.10 = private unnamed_addr constant [13 x i8] c"Info message\00", align 1
@.str.18 = private unnamed_addr constant [15 x i8] c"Database query\00", align 1
@.str.30 = private unnamed_addr constant [20 x i8] c"Performance logging\00", align 1
@.str.58 = private unnamed_addr constant [27 x i8] c"System state after cleanup\00", align 1
@.str.13 = private unnamed_addr constant [15 x i8] c"Log formatting\00", align 1
@.str.53 = private unnamed_addr constant [23 x i8] c"Log message formatting\00", align 1
@.str.8 = private unnamed_addr constant [24 x i8] c"Basic logging functions\00", align 1
@.str.57 = private unnamed_addr constant [11 x i8] c"Error test\00", align 1
@.str.7 = private unnamed_addr constant [20 x i8] c"Log level filtering\00", align 1
@.str.16 = private unnamed_addr constant [12 x i8] c"User action\00", align 1
@.str.32 = private unnamed_addr constant [13 x i8] c"Log rotation\00", align 1
@.str.20 = private unnamed_addr constant [24 x i8] c"Configuration functions\00", align 1
@.str.48 = private unnamed_addr constant [26 x i8] c"Multiple logger instances\00", align 1
@.str.4 = private unnamed_addr constant [6 x i8] c"ERROR\00", align 1
@.str.37 = private unnamed_addr constant [20 x i8] c"Log level constants\00", align 1
@.str.31 = private unnamed_addr constant [15 x i8] c"database_query\00", align 1
@.str.14 = private unnamed_addr constant [13 x i8] c"Test message\00", align 1
@.str.54 = private unnamed_addr constant [11 x i8] c"Debug test\00", align 1
@.str.6 = private unnamed_addr constant [1 x i8] c"\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  %2 = call i32 @get_log_level_name(i32 %LOG_DEBUG)
  %3 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %4 = call i32 @assert_eq_string(i32 %2, i32 %3)
  %5 = call i32 @get_log_level_name(i32 %LOG_INFO)
  %6 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.2, i64 0, i64 0
  %7 = call i32 @assert_eq_string(i32 %5, i32 %6)
  %8 = call i32 @get_log_level_name(i32 %LOG_WARN)
  %9 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.3, i64 0, i64 0
  %10 = call i32 @assert_eq_string(i32 %8, i32 %9)
  %11 = call i32 @get_log_level_name(i32 %LOG_ERROR)
  %12 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.4, i64 0, i64 0
  %13 = call i32 @assert_eq_string(i32 %11, i32 %12)
  %14 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.5, i64 0, i64 0
  %15 = call i32 @test_start(i32 %14)
  %16 = call i32 @set_log_level(i32 %LOG_DEBUG)
  %17 = call i32 @assert_true(i32 %16)
  %18 = call i32 @set_log_level(i32 %LOG_ERROR)
  %19 = call i32 @assert_true(i32 %18)
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %22 = call i32 @set_log_level(i32 10)
  %23 = call i32 @assert_false(i32 %22)
  %24 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.7, i64 0, i64 0
  %25 = call i32 @test_start(i32 %24)
  %26 = call i32 @set_log_level(i32 %LOG_WARN)
  %27 = call i32 @should_log(i32 %LOG_DEBUG)
  %28 = call i32 @assert_false(i32 %27)
  %29 = call i32 @should_log(i32 %LOG_INFO)
  %30 = call i32 @assert_false(i32 %29)
  %31 = call i32 @should_log(i32 %LOG_WARN)
  %32 = call i32 @assert_true(i32 %31)
  %33 = call i32 @should_log(i32 %LOG_ERROR)
  %34 = call i32 @assert_true(i32 %33)
  %35 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.8, i64 0, i64 0
  %36 = call i32 @test_start(i32 %35)
  %37 = call i32 @set_log_level(i32 %LOG_DEBUG)
  %38 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.9, i64 0, i64 0
  %39 = call i32 @debug(i32 %38)
  %40 = call i32 @assert_true(i32 %39)
  %41 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.10, i64 0, i64 0
  %42 = call i32 @info(i32 %41)
  %43 = call i32 @assert_true(i32 %42)
  %44 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.11, i64 0, i64 0
  %45 = call i32 @warn(i32 %44)
  %46 = call i32 @assert_true(i32 %45)
  %47 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.12, i64 0, i64 0
  %48 = call i32 @error(i32 %47)
  %49 = call i32 @assert_true(i32 %48)
  %50 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.13, i64 0, i64 0
  %51 = call i32 @test_start(i32 %50)
  %52 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.14, i64 0, i64 0
  %53 = call i32 @format_log_message(i32 %LOG_INFO, i32 %52)
  %54 = alloca i8*, align 4
  store i8* %53, i8** %54, align 4
  ; Variable formatted allocated at %54
  %55 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %56 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %57 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %58 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.15, i64 0, i64 0
  %59 = call i32 @test_start(i32 %58)
  %60 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.16, i64 0, i64 0
  %61 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.17, i64 0, i64 0
  %62 = call i32 @log_with_fields(i32 %LOG_INFO, i32 %60, i32 %61)
  %63 = call i32 @assert_true(i32 %62)
  %64 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.18, i64 0, i64 0
  %65 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.19, i64 0, i64 0
  %66 = call i32 @log_with_context(i32 %LOG_INFO, i32 %64, i32 %65)
  %67 = call i32 @assert_true(i32 %66)
  %68 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.20, i64 0, i64 0
  %69 = call i32 @test_start(i32 %68)
  %70 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.21, i64 0, i64 0
  %71 = call i32 @set_log_file(i32 %70)
  %72 = call i32 @assert_true(i32 %71)
  %73 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.22, i64 0, i64 0
  %74 = call i32 @set_log_format(i32 %73)
  %75 = call i32 @assert_true(i32 %74)
  %76 = call i32 @set_max_log_size(i32 2097152)
  %77 = call i32 @assert_true(i32 %76)
  %78 = call i32 @set_max_log_files(i32 10)
  %79 = call i32 @assert_true(i32 %78)
  %80 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.23, i64 0, i64 0
  %81 = call i32 @test_start(i32 %80)
  %82 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.24, i64 0, i64 0
  %83 = call i32 @create_logger(i32 %82, i32 %LOG_INFO)
  %84 = alloca i8*, align 4
  store i8* %83, i8** %84, align 4
  ; Variable logger_name allocated at %84
  %85 = load i32, i32* %84, align 4
  %86 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.24, i64 0, i64 0
  %87 = call i32 @assert_eq_string(i32 %85, i32 %86)
  %88 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.25, i64 0, i64 0
  %89 = call i32 @test_start(i32 %88)
  %90 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.26, i64 0, i64 0
  %91 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.27, i64 0, i64 0
  %92 = call i32 @log_with_logger(i32 %90, i32 %LOG_ERROR, i32 %91)
  %93 = call i32 @assert_true(i32 %92)
  %94 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.28, i64 0, i64 0
  %95 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.29, i64 0, i64 0
  %96 = call i32 @log_with_logger(i32 %94, i32 %LOG_WARN, i32 %95)
  %97 = call i32 @assert_true(i32 %96)
  %98 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.30, i64 0, i64 0
  %99 = call i32 @test_start(i32 %98)
  %100 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.31, i64 0, i64 0
  %101 = call i32 @perf_start(i32 %100)
  %102 = alloca i32, align 4
  store i32 %101, i32* %102, align 4
  ; Variable start_time allocated at %102
  %103 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.31, i64 0, i64 0
  %104 = load i32, i32* %102, align 4
  %105 = call i32 @perf_end(i32 %103, i32 %104)
  %106 = call i32 @assert_true(i32 %105)
  %107 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.32, i64 0, i64 0
  %108 = call i32 @test_start(i32 %107)
  %109 = call i32 @should_rotate_log()
  %110 = call i32 @assert_false(i32 %109)
  %111 = call i32 @rotate_logs()
  %112 = call i32 @assert_true(i32 %111)
  %113 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.33, i64 0, i64 0
  %114 = call i32 @test_start(i32 %113)
  %115 = call i32 @get_log_stats()
  %116 = alloca i8*, align 4
  store i8* %115, i8** %116, align 4
  ; Variable stats allocated at %116
  %117 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %118 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %119 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %120 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.34, i64 0, i64 0
  %121 = call i32 @test_start(i32 %120)
  %122 = call i32 @init_logging()
  %123 = call i32 @assert_true(i32 %122)
  %124 = call i32 @cleanup_logging()
  %125 = call i32 @assert_true(i32 %124)
  %126 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.35, i64 0, i64 0
  %127 = call i32 @test_start(i32 %126)
  %128 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %129 = call i32 @log_message(i32 %LOG_INFO, i32 %128)
  %130 = call i32 @assert_true(i32 %129)
  %131 = getelementptr inbounds [107 x i8], [107 x i8]* @.str.36, i64 0, i64 0
  %132 = call i32 @log_message(i32 %LOG_DEBUG, i32 %131)
  %133 = call i32 @assert_true(i32 %132)
  %134 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.37, i64 0, i64 0
  %135 = call i32 @test_start(i32 %134)
  %136 = call i32 @assert_eq_int(i32 %LOG_DEBUG, i32 0)
  %137 = call i32 @assert_eq_int(i32 %LOG_INFO, i32 1)
  %138 = call i32 @assert_eq_int(i32 %LOG_WARN, i32 2)
  %139 = call i32 @assert_eq_int(i32 %LOG_ERROR, i32 3)
  %140 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.38, i64 0, i64 0
  %141 = call i32 @test_start(i32 %140)
  %142 = call i32 @get_timestamp()
  %143 = alloca i8*, align 4
  store i8* %142, i8** %143, align 4
  ; Variable timestamp allocated at %143
  %144 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %145 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %146 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %147 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.39, i64 0, i64 0
  %148 = call i32 @test_start(i32 %147)
  %149 = call i32 @set_log_level(i32 %LOG_ERROR)
  %150 = call i32 @should_log(i32 %LOG_DEBUG)
  %151 = call i32 @assert_false(i32 %150)
  %152 = call i32 @should_log(i32 %LOG_INFO)
  %153 = call i32 @assert_false(i32 %152)
  %154 = call i32 @should_log(i32 %LOG_WARN)
  %155 = call i32 @assert_false(i32 %154)
  %156 = call i32 @should_log(i32 %LOG_ERROR)
  %157 = call i32 @assert_true(i32 %156)
  %158 = call i32 @set_log_level(i32 %LOG_DEBUG)
  %159 = call i32 @should_log(i32 %LOG_DEBUG)
  %160 = call i32 @assert_true(i32 %159)
  %161 = call i32 @should_log(i32 %LOG_INFO)
  %162 = call i32 @assert_true(i32 %161)
  %163 = call i32 @should_log(i32 %LOG_WARN)
  %164 = call i32 @assert_true(i32 %163)
  %165 = call i32 @should_log(i32 %LOG_ERROR)
  %166 = call i32 @assert_true(i32 %165)
  %167 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.40, i64 0, i64 0
  %168 = call i32 @test_start(i32 %167)
  %169 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.41, i64 0, i64 0
  %170 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.42, i64 0, i64 0
  %171 = call i32 @log_with_fields(i32 %LOG_ERROR, i32 %169, i32 %170)
  %172 = call i32 @assert_true(i32 %171)
  %173 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.43, i64 0, i64 0
  %174 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.44, i64 0, i64 0
  %175 = call i32 @log_with_context(i32 %LOG_WARN, i32 %173, i32 %174)
  %176 = call i32 @assert_true(i32 %175)
  %177 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.45, i64 0, i64 0
  %178 = call i32 @test_start(i32 %177)
  %179 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.46, i64 0, i64 0
  %180 = call i32 @perf_start(i32 %179)
  %181 = alloca i32, align 4
  store i32 %180, i32* %181, align 4
  ; Variable perf_start_time allocated at %181
  %182 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %183 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %184 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.46, i64 0, i64 0
  %185 = load i32, i32* %181, align 4
  %186 = call i32 @perf_end(i32 %184, i32 %185)
  %187 = call i32 @assert_true(i32 %186)
  %188 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.47, i64 0, i64 0
  %189 = call i32 @test_start(i32 %188)
  %190 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %191 = call i32 @set_log_file(i32 %190)
  %192 = call i32 @assert_true(i32 %191)
  %193 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %194 = call i32 @set_log_format(i32 %193)
  %195 = call i32 @assert_true(i32 %194)
  %196 = call i32 @set_max_log_size(i32 0)
  %197 = call i32 @assert_true(i32 %196)
  %198 = call i32 @set_max_log_files(i32 0)
  %199 = call i32 @assert_true(i32 %198)
  %200 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.48, i64 0, i64 0
  %201 = call i32 @test_start(i32 %200)
  %202 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.49, i64 0, i64 0
  %203 = call i32 @create_logger(i32 %202, i32 %LOG_WARN)
  %204 = alloca i8*, align 4
  store i8* %203, i8** %204, align 4
  ; Variable logger1 allocated at %204
  %205 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.50, i64 0, i64 0
  %206 = call i32 @create_logger(i32 %205, i32 %LOG_ERROR)
  %207 = alloca i8*, align 4
  store i8* %206, i8** %207, align 4
  ; Variable logger2 allocated at %207
  %208 = load i32, i32* %204, align 4
  %209 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.49, i64 0, i64 0
  %210 = call i32 @assert_eq_string(i32 %208, i32 %209)
  %211 = load i32, i32* %207, align 4
  %212 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.50, i64 0, i64 0
  %213 = call i32 @assert_eq_string(i32 %211, i32 %212)
  %214 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.49, i64 0, i64 0
  %215 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.51, i64 0, i64 0
  %216 = call i32 @log_with_logger(i32 %214, i32 %LOG_WARN, i32 %215)
  %217 = call i32 @assert_true(i32 %216)
  %218 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.50, i64 0, i64 0
  %219 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.52, i64 0, i64 0
  %220 = call i32 @log_with_logger(i32 %218, i32 %LOG_ERROR, i32 %219)
  %221 = call i32 @assert_true(i32 %220)
  %222 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.53, i64 0, i64 0
  %223 = call i32 @test_start(i32 %222)
  %224 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.54, i64 0, i64 0
  %225 = call i32 @format_log_message(i32 %LOG_DEBUG, i32 %224)
  %226 = alloca i8*, align 4
  store i8* %225, i8** %226, align 4
  ; Variable debug_msg allocated at %226
  %227 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.55, i64 0, i64 0
  %228 = call i32 @format_log_message(i32 %LOG_INFO, i32 %227)
  %229 = alloca i8*, align 4
  store i8* %228, i8** %229, align 4
  ; Variable info_msg allocated at %229
  %230 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.56, i64 0, i64 0
  %231 = call i32 @format_log_message(i32 %LOG_WARN, i32 %230)
  %232 = alloca i8*, align 4
  store i8* %231, i8** %232, align 4
  ; Variable warn_msg allocated at %232
  %233 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.57, i64 0, i64 0
  %234 = call i32 @format_log_message(i32 %LOG_ERROR, i32 %233)
  %235 = alloca i8*, align 4
  store i8* %234, i8** %235, align 4
  ; Variable error_msg allocated at %235
  %236 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %237 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %238 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %239 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %240 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %241 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %242 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %243 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %244 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %245 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %246 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %247 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %248 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.58, i64 0, i64 0
  %249 = call i32 @test_start(i32 %248)
  %250 = call i32 @init_logging()
  %251 = call i32 @assert_true(i32 %250)
  %252 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.59, i64 0, i64 0
  %253 = call i32 @info(i32 %252)
  %254 = call i32 @assert_true(i32 %253)
  %255 = call i32 @cleanup_logging()
  %256 = call i32 @assert_true(i32 %255)
  %257 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.60, i64 0, i64 0
  %258 = call i32 @test_start(i32 %257)
  %259 = call i32 @flush_logs()
  %260 = call i32 @assert_true(i32 %259)
  %261 = call i32 @print_test_summary()
  ret i32 0
}
