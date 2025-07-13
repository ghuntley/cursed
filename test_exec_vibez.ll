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
; mod module declarations



; String constants
@.str.2 = private unnamed_addr constant [25 x i8] c"exec_vibez add arguments\00", align 1
@.str.3 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
@.str.43 = private unnamed_addr constant [9 x i8] c"test.txt\00", align 1
@.str.18 = private unnamed_addr constant [33 x i8] c"exec_vibez environment variables\00", align 1
@.str.22 = private unnamed_addr constant [12 x i8] c"NONEXISTENT\00", align 1
@.str.7 = private unnamed_addr constant [23 x i8] c"exec_vibez set timeout\00", align 1
@.str.39 = private unnamed_addr constant [24 x i8] c"exec_vibez process info\00", align 1
@.str.41 = private unnamed_addr constant [10 x i8] c"not_found\00", align 1
@.str.52 = private unnamed_addr constant [26 x i8] c"exec_vibez error handling\00", align 1
@.str.8 = private unnamed_addr constant [24 x i8] c"exec_vibez command info\00", align 1
@.str.24 = private unnamed_addr constant [17 x i8] c"echo hello world\00", align 1
@.str.14 = private unnamed_addr constant [12 x i8] c"hello world\00", align 1
@.str.51 = private unnamed_addr constant [9 x i8] c"file.txt\00", align 1
@.str.53 = private unnamed_addr constant [23 x i8] c"exec_vibez state reset\00", align 1
@.str.1 = private unnamed_addr constant [5 x i8] c"echo\00", align 1
@.str.28 = private unnamed_addr constant [7 x i8] c"whoami\00", align 1
@.str.30 = private unnamed_addr constant [32 x i8] c"exec_vibez directory operations\00", align 1
@.str.21 = private unnamed_addr constant [5 x i8] c"PATH\00", align 1
@.str.35 = private unnamed_addr constant [6 x i8] c"sleep\00", align 1
@.str.37 = private unnamed_addr constant [23 x i8] c"exec_vibez system info\00", align 1
@.str.19 = private unnamed_addr constant [5 x i8] c"HOME\00", align 1
@.str.40 = private unnamed_addr constant [8 x i8] c"running\00", align 1
@.str.12 = private unnamed_addr constant [3 x i8] c"ls\00", align 1
@.str.20 = private unnamed_addr constant [18 x i8] c"/home/cursed_user\00", align 1
@.str.15 = private unnamed_addr constant [26 x i8] c"exec_vibez command exists\00", align 1
@.str.27 = private unnamed_addr constant [5 x i8] c"date\00", align 1
@.str.16 = private unnamed_addr constant [4 x i8] c"pwd\00", align 1
@.str.13 = private unnamed_addr constant [29 x i8] c"exec_vibez command with args\00", align 1
@.str.4 = private unnamed_addr constant [6 x i8] c"world\00", align 1
@.str.5 = private unnamed_addr constant [33 x i8] c"exec_vibez set working directory\00", align 1
@.str.17 = private unnamed_addr constant [20 x i8] c"nonexistent_command\00", align 1
@.str.10 = private unnamed_addr constant [23 x i8] c"exec_vibez run command\00", align 1
@.str.42 = private unnamed_addr constant [27 x i8] c"exec_vibez file operations\00", align 1
@.str.44 = private unnamed_addr constant [11 x i8] c"output.txt\00", align 1
@.str.46 = private unnamed_addr constant [30 x i8] c"exec_vibez network operations\00", align 1
@.str.31 = private unnamed_addr constant [23 x i8] c"exec_vibez path exists\00", align 1
@.str.47 = private unnamed_addr constant [10 x i8] c"localhost\00", align 1
@.str.48 = private unnamed_addr constant [10 x i8] c"127.0.0.1\00", align 1
@.str.32 = private unnamed_addr constant [9 x i8] c"/usr/bin\00", align 1
@.str.33 = private unnamed_addr constant [14 x i8] c"some_file.txt\00", align 1
@.str.9 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.34 = private unnamed_addr constant [32 x i8] c"exec_vibez background execution\00", align 1
@.str.36 = private unnamed_addr constant [3 x i8] c"10\00", align 1
@.str.26 = private unnamed_addr constant [30 x i8] c"exec_vibez different commands\00", align 1
@.str.49 = private unnamed_addr constant [13 x i8] c"invalid_host\00", align 1
@.str.45 = private unnamed_addr constant [13 x i8] c"test content\00", align 1
@.str.29 = private unnamed_addr constant [30 x i8] c"exec_vibez process operations\00", align 1
@.str.6 = private unnamed_addr constant [5 x i8] c"/tmp\00", align 1
@.str.25 = private unnamed_addr constant [30 x i8] c"exec_vibez empty command line\00", align 1
@.str.0 = private unnamed_addr constant [28 x i8] c"exec_vibez command creation\00", align 1
@.str.23 = private unnamed_addr constant [34 x i8] c"exec_vibez command line execution\00", align 1
@.str.11 = private unnamed_addr constant [28 x i8] c"exec_vibez simple execution\00", align 1
@.str.38 = private unnamed_addr constant [26 x i8] c"exec_vibez system metrics\00", align 1
@.str.50 = private unnamed_addr constant [19 x i8] c"http://example.com\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  %2 = call i32 @exec_reset_state()
  %3 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.1, i64 0, i64 0
  %4 = call i32 @exec_new_command(i32 %3)
  %5 = alloca i1, align 4
  store i1 %4, i1* %5, align 4
  ; Variable success allocated at %5
  %6 = load i32, i32* %5, align 4
  %7 = call i32 @assert_true(i32 %6)
  %8 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.2, i64 0, i64 0
  %9 = call i32 @test_start(i32 %8)
  %10 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %11 = call i32 @exec_add_arg(i32 %10)
  %12 = alloca i1, align 4
  store i1 %11, i1* %12, align 4
  ; Variable arg_success allocated at %12
  %13 = load i32, i32* %12, align 4
  %14 = call i32 @assert_true(i32 %13)
  %15 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.4, i64 0, i64 0
  %16 = call i32 @exec_add_arg(i32 %15)
  %17 = alloca i1, align 4
  store i1 %16, i1* %17, align 4
  ; Variable arg_success2 allocated at %17
  %18 = load i32, i32* %17, align 4
  %19 = call i32 @assert_true(i32 %18)
  %20 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.5, i64 0, i64 0
  %21 = call i32 @test_start(i32 %20)
  %22 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.6, i64 0, i64 0
  %23 = call i32 @exec_set_dir(i32 %22)
  %24 = alloca i1, align 4
  store i1 %23, i1* %24, align 4
  ; Variable dir_success allocated at %24
  %25 = load i32, i32* %24, align 4
  %26 = call i32 @assert_true(i32 %25)
  %27 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.7, i64 0, i64 0
  %28 = call i32 @test_start(i32 %27)
  %29 = call i32 @exec_set_timeout(i32 60)
  %30 = alloca i1, align 4
  store i1 %29, i1* %30, align 4
  ; Variable timeout_success allocated at %30
  %31 = load i32, i32* %30, align 4
  %32 = call i32 @assert_true(i32 %31)
  %33 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.8, i64 0, i64 0
  %34 = call i32 @test_start(i32 %33)
  %35 = call i32 @exec_get_command_info()
  %36 = alloca i8*, align 4
  store i8* %35, i8** %36, align 4
  ; Variable info allocated at %36
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %39 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.10, i64 0, i64 0
  %40 = call i32 @test_start(i32 %39)
  %41 = call i32 @exec_run_command()
  %42 = alloca i1, align 4
  store i1 %41, i1* %42, align 4
  ; Variable run_success allocated at %42
  %43 = load i32, i32* %42, align 4
  %44 = call i32 @assert_true(i32 %43)
  %45 = call i32 @exec_get_exit_code()
  %46 = call i32 @assert_eq_int(i32 %45, i32 0)
  %47 = call i32 @exec_get_success()
  %48 = call i32 @assert_true(i32 %47)
  %49 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %50 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %51 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.11, i64 0, i64 0
  %52 = call i32 @test_start(i32 %51)
  %53 = call i32 @exec_reset_state()
  %54 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.12, i64 0, i64 0
  %55 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %56 = call i32 @exec_simple(i32 %54, i32 %55)
  %57 = alloca i1, align 4
  store i1 %56, i1* %57, align 4
  ; Variable simple_success allocated at %57
  %58 = load i32, i32* %57, align 4
  %59 = call i32 @assert_true(i32 %58)
  %60 = call i32 @exec_get_exit_code()
  %61 = call i32 @assert_eq_int(i32 %60, i32 0)
  %62 = call i32 @exec_get_success()
  %63 = call i32 @assert_true(i32 %62)
  %64 = call i32 @exec_get_stdout()
  %65 = alloca i8*, align 4
  store i8* %64, i8** %65, align 4
  ; Variable ls_output allocated at %65
  %66 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %67 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %68 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.13, i64 0, i64 0
  %69 = call i32 @test_start(i32 %68)
  %70 = call i32 @exec_reset_state()
  %71 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.1, i64 0, i64 0
  %72 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.14, i64 0, i64 0
  %73 = call i32 @exec_simple(i32 %71, i32 %72)
  %74 = alloca i1, align 4
  store i1 %73, i1* %74, align 4
  ; Variable args_success allocated at %74
  %75 = load i32, i32* %74, align 4
  %76 = call i32 @assert_true(i32 %75)
  %77 = call i32 @exec_get_exit_code()
  %78 = call i32 @assert_eq_int(i32 %77, i32 0)
  %79 = call i32 @exec_get_stdout()
  %80 = alloca i8*, align 4
  store i8* %79, i8** %80, align 4
  ; Variable echo_output allocated at %80
  %81 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %82 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %83 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.15, i64 0, i64 0
  %84 = call i32 @test_start(i32 %83)
  %85 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.1, i64 0, i64 0
  %86 = call i32 @exec_command_exists(i32 %85)
  %87 = call i32 @assert_true(i32 %86)
  %88 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.12, i64 0, i64 0
  %89 = call i32 @exec_command_exists(i32 %88)
  %90 = call i32 @assert_true(i32 %89)
  %91 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.16, i64 0, i64 0
  %92 = call i32 @exec_command_exists(i32 %91)
  %93 = call i32 @assert_true(i32 %92)
  %94 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.17, i64 0, i64 0
  %95 = call i32 @exec_command_exists(i32 %94)
  %96 = call i32 @assert_false(i32 %95)
  %97 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.18, i64 0, i64 0
  %98 = call i32 @test_start(i32 %97)
  %99 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.19, i64 0, i64 0
  %100 = call i32 @exec_get_env(i32 %99)
  %101 = alloca i8*, align 4
  store i8* %100, i8** %101, align 4
  ; Variable home allocated at %101
  %102 = load i32, i32* %101, align 4
  %103 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.20, i64 0, i64 0
  %104 = call i32 @assert_eq_string(i32 %102, i32 %103)
  %105 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.21, i64 0, i64 0
  %106 = call i32 @exec_get_env(i32 %105)
  %107 = alloca i8*, align 4
  store i8* %106, i8** %107, align 4
  ; Variable path allocated at %107
  %108 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %109 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %110 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.22, i64 0, i64 0
  %111 = call i32 @exec_get_env(i32 %110)
  %112 = alloca i8*, align 4
  store i8* %111, i8** %112, align 4
  ; Variable empty allocated at %112
  %113 = load i32, i32* %112, align 4
  %114 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %115 = call i32 @assert_eq_string(i32 %113, i32 %114)
  %116 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.23, i64 0, i64 0
  %117 = call i32 @test_start(i32 %116)
  %118 = call i32 @exec_reset_state()
  %119 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.24, i64 0, i64 0
  %120 = call i32 @exec_command_line(i32 %119)
  %121 = alloca i1, align 4
  store i1 %120, i1* %121, align 4
  ; Variable cmdline_success allocated at %121
  %122 = load i32, i32* %121, align 4
  %123 = call i32 @assert_true(i32 %122)
  %124 = call i32 @exec_get_exit_code()
  %125 = call i32 @assert_eq_int(i32 %124, i32 0)
  %126 = call i32 @exec_get_success()
  %127 = call i32 @assert_true(i32 %126)
  %128 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.25, i64 0, i64 0
  %129 = call i32 @test_start(i32 %128)
  %130 = call i32 @exec_reset_state()
  %131 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %132 = call i32 @exec_command_line(i32 %131)
  %133 = alloca i1, align 4
  store i1 %132, i1* %133, align 4
  ; Variable empty_success allocated at %133
  %134 = load i32, i32* %133, align 4
  %135 = call i32 @assert_false(i32 %134)
  %136 = call i32 @exec_get_exit_code()
  %137 = call i32 @assert_eq_int(i32 %136, i32 1)
  %138 = call i32 @exec_get_success()
  %139 = call i32 @assert_false(i32 %138)
  %140 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.26, i64 0, i64 0
  %141 = call i32 @test_start(i32 %140)
  %142 = call i32 @exec_reset_state()
  %143 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.16, i64 0, i64 0
  %144 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %145 = call i32 @exec_simple(i32 %143, i32 %144)
  %146 = call i32 @exec_get_stdout()
  %147 = alloca i8*, align 4
  store i8* %146, i8** %147, align 4
  ; Variable pwd_output allocated at %147
  %148 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %149 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %150 = call i32 @exec_reset_state()
  %151 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.27, i64 0, i64 0
  %152 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %153 = call i32 @exec_simple(i32 %151, i32 %152)
  %154 = call i32 @exec_get_stdout()
  %155 = alloca i8*, align 4
  store i8* %154, i8** %155, align 4
  ; Variable date_output allocated at %155
  %156 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %157 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %158 = call i32 @exec_reset_state()
  %159 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.28, i64 0, i64 0
  %160 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %161 = call i32 @exec_simple(i32 %159, i32 %160)
  %162 = call i32 @exec_get_stdout()
  %163 = alloca i8*, align 4
  store i8* %162, i8** %163, align 4
  ; Variable whoami_output allocated at %163
  %164 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %165 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %166 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.29, i64 0, i64 0
  %167 = call i32 @test_start(i32 %166)
  %168 = call i32 @exec_kill_process(i32 1234)
  %169 = alloca i1, align 4
  store i1 %168, i1* %169, align 4
  ; Variable kill_success allocated at %169
  %170 = load i32, i32* %169, align 4
  %171 = call i32 @assert_true(i32 %170)
  %172 = call i32 @exec_kill_process(i32 0)
  %173 = alloca i1, align 4
  store i1 %172, i1* %173, align 4
  ; Variable invalid_kill allocated at %173
  %174 = load i32, i32* %173, align 4
  %175 = call i32 @assert_false(i32 %174)
  %176 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.30, i64 0, i64 0
  %177 = call i32 @test_start(i32 %176)
  %178 = call i32 @exec_getcwd()
  %179 = alloca i8*, align 4
  store i8* %178, i8** %179, align 4
  ; Variable cwd allocated at %179
  %180 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %181 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %182 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.6, i64 0, i64 0
  %183 = call i32 @exec_chdir(i32 %182)
  %184 = alloca i1, align 4
  store i1 %183, i1* %184, align 4
  ; Variable chdir_success allocated at %184
  %185 = load i32, i32* %184, align 4
  %186 = call i32 @assert_true(i32 %185)
  %187 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %188 = call i32 @exec_chdir(i32 %187)
  %189 = alloca i1, align 4
  store i1 %188, i1* %189, align 4
  ; Variable chdir_fail allocated at %189
  %190 = load i32, i32* %189, align 4
  %191 = call i32 @assert_false(i32 %190)
  %192 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.31, i64 0, i64 0
  %193 = call i32 @test_start(i32 %192)
  %194 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.32, i64 0, i64 0
  %195 = call i32 @exec_path_exists(i32 %194)
  %196 = call i32 @assert_true(i32 %195)
  %197 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.33, i64 0, i64 0
  %198 = call i32 @exec_path_exists(i32 %197)
  %199 = call i32 @assert_true(i32 %198)
  %200 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %201 = call i32 @exec_path_exists(i32 %200)
  %202 = call i32 @assert_false(i32 %201)
  %203 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.34, i64 0, i64 0
  %204 = call i32 @test_start(i32 %203)
  %205 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.35, i64 0, i64 0
  %206 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.36, i64 0, i64 0
  %207 = call i32 @exec_background(i32 %205, i32 %206)
  %208 = alloca i32, align 4
  store i32 %207, i32* %208, align 4
  ; Variable bg_pid allocated at %208
  %209 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %210 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %211 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.37, i64 0, i64 0
  %212 = call i32 @test_start(i32 %211)
  %213 = call i32 @exec_get_system_info()
  %214 = alloca i8*, align 4
  store i8* %213, i8** %214, align 4
  ; Variable system_info allocated at %214
  %215 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %216 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %217 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.38, i64 0, i64 0
  %218 = call i32 @test_start(i32 %217)
  %219 = call i32 @exec_get_uptime()
  %220 = alloca i32, align 4
  store i32 %219, i32* %220, align 4
  ; Variable uptime allocated at %220
  %221 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %222 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %223 = call i32 @exec_get_load_average()
  %224 = alloca double, align 4
  store double %223, double* %224, align 4
  ; Variable load allocated at %224
  %225 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %226 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %227 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.39, i64 0, i64 0
  %228 = call i32 @test_start(i32 %227)
  %229 = call i32 @exec_get_process_pid()
  %230 = alloca i32, align 4
  store i32 %229, i32* %230, align 4
  ; Variable proc_pid allocated at %230
  %231 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %232 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %233 = call i32 @exec_get_process_status(i32 1234)
  %234 = alloca i8*, align 4
  store i8* %233, i8** %234, align 4
  ; Variable proc_status allocated at %234
  %235 = load i32, i32* %234, align 4
  %236 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.40, i64 0, i64 0
  %237 = call i32 @assert_eq_string(i32 %235, i32 %236)
  %238 = call i32 @exec_get_process_status(i32 0)
  %239 = alloca i8*, align 4
  store i8* %238, i8** %239, align 4
  ; Variable invalid_status allocated at %239
  %240 = load i32, i32* %239, align 4
  %241 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.41, i64 0, i64 0
  %242 = call i32 @assert_eq_string(i32 %240, i32 %241)
  %243 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.42, i64 0, i64 0
  %244 = call i32 @test_start(i32 %243)
  %245 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.43, i64 0, i64 0
  %246 = call i32 @exec_file_exists(i32 %245)
  %247 = call i32 @assert_true(i32 %246)
  %248 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %249 = call i32 @exec_file_exists(i32 %248)
  %250 = call i32 @assert_false(i32 %249)
  %251 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.43, i64 0, i64 0
  %252 = call i32 @exec_read_file(i32 %251)
  %253 = alloca i8*, align 4
  store i8* %252, i8** %253, align 4
  ; Variable file_content allocated at %253
  %254 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %255 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %256 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.44, i64 0, i64 0
  %257 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.45, i64 0, i64 0
  %258 = call i32 @exec_write_file(i32 %256, i32 %257)
  %259 = alloca i1, align 4
  store i1 %258, i1* %259, align 4
  ; Variable write_success allocated at %259
  %260 = load i32, i32* %259, align 4
  %261 = call i32 @assert_true(i32 %260)
  %262 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.46, i64 0, i64 0
  %263 = call i32 @test_start(i32 %262)
  %264 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.47, i64 0, i64 0
  %265 = call i32 @exec_ping(i32 %264)
  %266 = call i32 @assert_true(i32 %265)
  %267 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.48, i64 0, i64 0
  %268 = call i32 @exec_ping(i32 %267)
  %269 = call i32 @assert_true(i32 %268)
  %270 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.49, i64 0, i64 0
  %271 = call i32 @exec_ping(i32 %270)
  %272 = call i32 @assert_false(i32 %271)
  %273 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.50, i64 0, i64 0
  %274 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.51, i64 0, i64 0
  %275 = call i32 @exec_download(i32 %273, i32 %274)
  %276 = alloca i1, align 4
  store i1 %275, i1* %276, align 4
  ; Variable download_success allocated at %276
  %277 = load i32, i32* %276, align 4
  %278 = call i32 @assert_true(i32 %277)
  %279 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.52, i64 0, i64 0
  %280 = call i32 @test_start(i32 %279)
  %281 = call i32 @exec_reset_state()
  %282 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %283 = call i32 @exec_new_command(i32 %282)
  %284 = call i32 @exec_run_command()
  %285 = alloca i1, align 4
  store i1 %284, i1* %285, align 4
  ; Variable error_run allocated at %285
  %286 = load i32, i32* %285, align 4
  %287 = call i32 @assert_false(i32 %286)
  %288 = call i32 @exec_get_exit_code()
  %289 = call i32 @assert_eq_int(i32 %288, i32 1)
  %290 = call i32 @exec_get_success()
  %291 = call i32 @assert_false(i32 %290)
  %292 = call i32 @exec_get_stderr()
  %293 = alloca i8*, align 4
  store i8* %292, i8** %293, align 4
  ; Variable error_msg allocated at %293
  %294 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %295 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.9, i64 0, i64 0
  %296 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.53, i64 0, i64 0
  %297 = call i32 @test_start(i32 %296)
  %298 = call i32 @exec_reset_state()
  %299 = call i32 @exec_reset_state()
  %300 = alloca i1, align 4
  store i1 %299, i1* %300, align 4
  ; Variable reset_success allocated at %300
  %301 = load i32, i32* %300, align 4
  %302 = call i32 @assert_true(i32 %301)
  %303 = call i32 @print_test_summary()
  ret i32 0
}
