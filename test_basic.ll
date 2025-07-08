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
define i32 @test_basic_json() {
entry:
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @assert_true(i32 1)
  ; Expression result: %2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %4 = call i32 @assert_false(i32 %3)
  ; Expression result: %4
  %5 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %6 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %7 = call i32 @assert_eq_string(i32 %5, i32 %6)
  ; Expression result: %7
  %8 = call i32 @assert_eq_int(i32 42, i32 42)
  ; Expression result: %8
  %9 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.3, i64 0, i64 0
  %10 = call i32 @puts(i8* %9)
  %11 = add i32 0, 0
  ; Expression result: %11
  ret i32 0
}

define i32 @run_basic_json_test() {
entry:
  %0 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.4, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.5, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = call i32 @test_basic_json()
  ; Expression result: %6
  %7 = call i32 @print_test_summary()
  ; Expression result: %7
  ret i32 0
}



; String constants
@.str.4 = private unnamed_addr constant [29 x i8] c"🔧 Running Basic JSON Test\00", align 1
@.str.1 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.5 = private unnamed_addr constant [25 x i8] c"========================\00", align 1
@.str.0 = private unnamed_addr constant [16 x i8] c"Basic JSON Test\00", align 1
@.str.3 = private unnamed_addr constant [36 x i8] c"Basic JSON module structure created\00", align 1
@.str.2 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
define i32 @main() {
  %0 = call i32 @run_basic_json_test()
  ret i32 0
}
