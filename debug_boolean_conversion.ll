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
define i8* @test_boolean_simple() {
entry:
  %0 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = alloca i1, align 4
  store i1 1, i1* %3, align 4
  ; Variable true_val allocated
  %4 = alloca i1, align 4
  store i1 0, i1* %4, align 4
  ; Variable false_val allocated
  %5 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.1, i64 0, i64 0
  %6 = call i32 @puts(i8* %5)
  %7 = add i32 0, 0
  ; Expression result: %7
  %8 = load i1, i1* %3, align 4
  %9 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %10 = zext i1 %8 to i32
  %11 = call i32 (i8*, ...) @printf(i8* %9, i32 %10)
  %12 = add i32 0, 0
  ; Expression result: %12
  %13 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.3, i64 0, i64 0
  %14 = call i32 @puts(i8* %13)
  %15 = add i32 0, 0
  ; Expression result: %15
  %16 = load i1, i1* %4, align 4
  %17 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %18 = zext i1 %16 to i32
  %19 = call i32 (i8*, ...) @printf(i8* %17, i32 %18)
  %20 = add i32 0, 0
  ; Expression result: %20
  %21 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.4, i64 0, i64 0
  %22 = call i32 @puts(i8* %21)
  %23 = add i32 0, 0
  ; Expression result: %23
  %24 = load i1, i1* %4, align 4
  ; Expression result: %24
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  ; Expression result: %25
  ret i32 0
}



; String constants
@.str.3 = private unnamed_addr constant [12 x i8] c"false_val: \00", align 1
@.str.5 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.1 = private unnamed_addr constant [11 x i8] c"true_val: \00", align 1
@.str.4 = private unnamed_addr constant [28 x i8] c"Testing logic operations...\00", align 1
@.str.2 = private unnamed_addr constant [6 x i8] c"%d\\0A\00", align 1
@.str.0 = private unnamed_addr constant [26 x i8] c"Testing boolean values...\00", align 1
define i32 @main() {
  %0 = call i32 @test_boolean_simple()
  ret i32 0
}
