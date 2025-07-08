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
@.str.0 = private unnamed_addr constant [25 x i8] c"Testing debug_tea module\00", align 1
@.str.6 = private unnamed_addr constant [25 x i8] c"Variable should equal 42\00", align 1
@.str.7 = private unnamed_addr constant [32 x i8] c"Debug tea module test completed\00", align 1
@.str.2 = private unnamed_addr constant [31 x i8] c"Debug tea module test starting\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.5 = private unnamed_addr constant [9 x i8] c"test_var\00", align 1
@.str.4 = private unnamed_addr constant [21 x i8] c"Test warning message\00", align 1
@.str.3 = private unnamed_addr constant [19 x i8] c"Test error message\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.0, i64 0, i64 0
  ; Converting complex expression to output
  %1 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %2 = call i32 (i8*, ...) @printf(i8* %1, i32 %0)
  %3 = call i32 @enable_debug()
  %4 = call i32 @set_debug_level(i32 %DEBUG_LEVEL_INFO)
  %5 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.2, i64 0, i64 0
  %6 = call i32 @debug_info(i32 %5)
  %7 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.3, i64 0, i64 0
  %8 = call i32 @debug_error(i32 %7)
  %9 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.4, i64 0, i64 0
  %10 = call i32 @debug_warn(i32 %9)
  %11 = alloca i32, align 4
  store i32 42, i32* %11, align 4
  ; Variable test_var allocated at %11
  %12 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.5, i64 0, i64 0
  %13 = load i32, i32* %11, align 4
  %14 = call i32 @inspect_int(i32 %12, i32 %13)
  %15 = load i32, i32* %11, align 4
  %16 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.6, i64 0, i64 0
  %17 = call i32 @debug_assert_eq_int(i32 %15, i32 42, i32 %16)
  %18 = call i32 @debug_print_config()
  %19 = call i32 @debug_print_summary()
  %20 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.7, i64 0, i64 0
  ; Converting complex expression to output
  %21 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %22 = call i32 (i8*, ...) @printf(i8* %21, i32 %20)
  ret i32 0
}
