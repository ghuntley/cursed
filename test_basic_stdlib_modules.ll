; CURSED Language - Advanced LLVM Compilation
target triple = "x86_64-unknown-linux-gnu"


; Runtime function declarations
declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)
declare i32 @print(i8*)
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
declare i32 @cursed_channel_send(i8*, i64)
declare i32 @cursed_channel_receive(i8*, i64*)
declare void @cursed_channel_error(i32)
declare void @panic_non_exhaustive_match()
declare i1 @cursed_check_type_compatibility(i8*, i32, i32)
declare i1 @cursed_check_interface_type(i8*)
declare i1 @cursed_check_generic_type(i8*)
declare i1 @cursed_check_array_type(i8*)
declare i1 @cursed_check_function_type(i8*)
declare i8* @cursed_cast_type(i8*, i32, i32)
declare i8* @cursed_empty_string()
declare i8* @cursed_null_value()
declare void @cursed_panic_type_assertion(i32, i32)
declare i1 @cursed_type_switch_check_type(i8*, i32)
declare i1 @cursed_implements_interface(i8*, i8*)
declare i1 @cursed_test_method_impl(i8*)
declare i8* @cursed_dispatch_simple_method(i8*, i8*, i32)
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
declare void @cursed_defer_cleanup()
declare void @defer_generic_cleanup()
declare void @defer_function()
declare void @cursed_enhanced_try_begin(i64)
declare void @cursed_enhanced_try_end(i64)
declare i8* @cursed_get_panic_context(i64)
declare i8* @cursed_extract_panic_value(i8*)
declare i8* @cursed_extract_stack_trace(i8*)
declare void @cursed_clear_panic_context(i64)

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


; Main function entry point

; String constants
@.str.0 = private unnamed_addr constant [28 x i8] c"Basic stdlib module testing\00", align 1
@.str.2 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
@.str.1 = private unnamed_addr constant [1 x i8] c"\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  %2 = call i32 @assert_true(i32 1)
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %4 = call i32 @assert_false(i32 %3)
  %5 = call i32 @assert_eq_int(i32 42, i32 42)
  %6 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %7 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %8 = call i32 @assert_eq_string(i32 %6, i32 %7)
  %9 = alloca i32, align 4
  store i32 10, i32* %9, align 4
  ; Variable x allocated at %9
  %10 = alloca i32, align 4
  store i32 20, i32* %10, align 4
  ; Variable y allocated at %10
  %11 = load i32, i32* %9, align 4
  %12 = load i32, i32* %10, align 4
  %13 = add i32 %11, %12
  %14 = call i32 @assert_eq_int(i32 %13, i32 30)
  %15 = load i32, i32* %9, align 4
  %16 = mul i32 %15, 2
  %17 = call i32 @assert_eq_int(i32 %16, i32 20)
  %18 = call i32 @print_test_summary()
  ret i32 0
}
