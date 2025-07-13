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
@.str.3 = private unnamed_addr constant [16 x i8] c"Success value: \00", align 1
@.str.5 = private unnamed_addr constant [5 x i8] c"echo\00", align 1
@.str.8 = private unnamed_addr constant [23 x i8] c"Testing command exists\00", align 1
@.str.10 = private unnamed_addr constant [36 x i8] c"Echo exists result (should be true)\00", align 1
@.str.6 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
@.str.9 = private unnamed_addr constant [12 x i8] c"invalid_cmd\00", align 1
@.str.7 = private unnamed_addr constant [12 x i8] c"Exit code: \00", align 1
@.str.12 = private unnamed_addr constant [31 x i8] c"exec_vibez basic test complete\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.4 = private unnamed_addr constant [29 x i8] c"Testing exec_simple function\00", align 1
@.str.0 = private unnamed_addr constant [25 x i8] c"Starting exec_vibez test\00", align 1
@.str.11 = private unnamed_addr constant [40 x i8] c"Invalid exists result (should be false)\00", align 1
@.str.2 = private unnamed_addr constant [27 x i8] c"EXEC_SUCCESS constant test\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.0, i64 0, i64 0
  ; Converting complex expression to output
  %1 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %2 = call i32 (i8*, ...) @printf(i8* %1, i32 %0)
  %3 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.2, i64 0, i64 0
  ; Converting complex expression to output
  %4 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %5 = call i32 (i8*, ...) @printf(i8* %4, i32 %3)
  %6 = alloca i32, align 4
  store i32 %EXEC_SUCCESS, i32* %6, align 4
  ; Variable success_val allocated at %6
  %7 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.3, i64 0, i64 0
  ; Converting complex expression to output
  %8 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %9 = call i32 (i8*, ...) @printf(i8* %8, i32 %7)
  %10 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.4, i64 0, i64 0
  ; Converting complex expression to output
  %11 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %12 = call i32 (i8*, ...) @printf(i8* %11, i32 %10)
  %13 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.5, i64 0, i64 0
  %14 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.6, i64 0, i64 0
  %15 = call i32 @exec_simple(i32 %13, i32 %14)
  %16 = call i32 @exec_get_exit_code()
  %17 = alloca i32, align 4
  store i32 %16, i32* %17, align 4
  ; Variable exit_code allocated at %17
  %18 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.7, i64 0, i64 0
  ; Converting complex expression to output
  %19 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %20 = call i32 (i8*, ...) @printf(i8* %19, i32 %18)
  %21 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.8, i64 0, i64 0
  ; Converting complex expression to output
  %22 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %23 = call i32 (i8*, ...) @printf(i8* %22, i32 %21)
  %24 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.5, i64 0, i64 0
  %25 = call i32 @exec_command_exists(i32 %24)
  %26 = alloca i1, align 4
  store i1 %25, i1* %26, align 4
  ; Variable echo_exists allocated at %26
  %27 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.9, i64 0, i64 0
  %28 = call i32 @exec_command_exists(i32 %27)
  %29 = alloca i1, align 4
  store i1 %28, i1* %29, align 4
  ; Variable invalid_exists allocated at %29
  %30 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.10, i64 0, i64 0
  ; Converting complex expression to output
  %31 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %32 = call i32 (i8*, ...) @printf(i8* %31, i32 %30)
  %33 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.11, i64 0, i64 0
  ; Converting complex expression to output
  %34 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %35 = call i32 (i8*, ...) @printf(i8* %34, i32 %33)
  %36 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.12, i64 0, i64 0
  ; Converting complex expression to output
  %37 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %38 = call i32 (i8*, ...) @printf(i8* %37, i32 %36)
  ret i32 0
}
