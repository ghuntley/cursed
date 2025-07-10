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
@.str.0 = private unnamed_addr constant [33 x i8] c"Testing minimal select statement\00", align 1
@.str.4 = private unnamed_addr constant [30 x i8] c"Minimal select test completed\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.3 = private unnamed_addr constant [22 x i8] c"Default case executed\00", align 1
@.str.2 = private unnamed_addr constant [1 x i8] c"\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.0, i64 0, i64 0
  ; Converting complex expression to output
  %1 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %2 = call i32 (i8*, ...) @printf(i8* %1, i32 %0)
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %8 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.3, i64 0, i64 0
  ; Converting complex expression to output
  %9 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %10 = call i32 (i8*, ...) @printf(i8* %9, i32 %8)
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %13 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.4, i64 0, i64 0
  ; Converting complex expression to output
  %14 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %15 = call i32 (i8*, ...) @printf(i8* %14, i32 %13)
  ret i32 0
}
