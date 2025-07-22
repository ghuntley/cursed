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

; Function: assert_eq_int
define i1 @assert_eq_int(i32 %arg_0, i32 %arg_1) personality i32 (...)* @__gxx_personality_v0 {
; Function Attrs: uwtable noinline optnone
entry:
  %0 = alloca i32, align 4
  store i32 %arg_0, i32* %0, align 4
  %1 = alloca i32, align 4
  store i32 %arg_1, i32* %1, align 4
  ; DEBUG: generate_if_statement_with_init called
  ; DEBUG: about to process condition
  %2 = load i32, i32* %0, align 4
  %3 = load i32, i32* %1, align 4
  %4 = icmp eq i32 %2, %3
  br i1 %4, label %label0, label %label1
label0:
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %6 = add i32 %pass_count, 1
  %7 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.1, i64 0, i64 0
  %8 = call i32 @puts(i8* %7)
  %9 = load i32, i32* %0, align 4
  %10 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %11 = call i32 (i8*, ...) @printf(i8* %10, i32 %9)
  %12 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.3, i64 0, i64 0
  %13 = call i32 @puts(i8* %12)
  %14 = load i32, i32* %1, align 4
  %15 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %16 = call i32 (i8*, ...) @printf(i8* %15, i32 %14)
  br label %label2
label1:
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %18 = add i32 %fail_count, 1
  %19 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.4, i64 0, i64 0
  %20 = call i32 @puts(i8* %19)
  %21 = load i32, i32* %1, align 4
  %22 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %23 = call i32 (i8*, ...) @printf(i8* %22, i32 %21)
  %24 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.5, i64 0, i64 0
  %25 = call i32 @puts(i8* %24)
  %26 = load i32, i32* %0, align 4
  %27 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %28 = call i32 (i8*, ...) @printf(i8* %27, i32 %26)
  br label %label2
label2:
  ret i1 1
}

; Function: print_summary
define i1 @print_summary() personality i32 (...)* @__gxx_personality_v0 {
; Function Attrs: uwtable noinline optnone
entry:
  %0 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.6, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.7, i64 0, i64 0
  %3 = call i32 @puts(i8* %2)
  %4 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %5 = call i32 (i8*, ...) @printf(i8* %4, i32 %pass_count)
  %6 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.8, i64 0, i64 0
  %7 = call i32 @puts(i8* %6)
  %8 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %9 = call i32 (i8*, ...) @printf(i8* %8, i32 %fail_count)
  ret i1 1
}

; Main function entry point

; String constants
@.str.5 = private unnamed_addr constant [7 x i8] c", got \00", align 1
@.str.2 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.4 = private unnamed_addr constant [16 x i8] c"FAIL: Expected \00", align 1
@.str.7 = private unnamed_addr constant [7 x i8] c"Pass: \00", align 1
@.str.1 = private unnamed_addr constant [7 x i8] c"PASS: \00", align 1
@.str.0 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.3 = private unnamed_addr constant [5 x i8] c" == \00", align 1
@.str.6 = private unnamed_addr constant [14 x i8] c"Test Results:\00", align 1
@.str.8 = private unnamed_addr constant [7 x i8] c"Fail: \00", align 1
define i32 @main() {
entry:
  %10 = alloca i32, align 4
  store i32 0, i32* %10, align 4
  ; Variable test_count allocated at %10
  %11 = alloca i32, align 4
  store i32 0, i32* %11, align 4
  ; Variable pass_count allocated at %11
  %12 = alloca i32, align 4
  store i32 0, i32* %12, align 4
  ; Variable fail_count allocated at %12
  %13 = add i32 2, 2
  %14 = call i32 @assert_eq_int(i32 %13, i32 4)
  %15 = sub i32 10, 5
  %16 = call i32 @assert_eq_int(i32 %15, i32 5)
  %17 = mul i32 3, 3
  %18 = call i32 @assert_eq_int(i32 %17, i32 9)
  %19 = call i32 @print_summary()
  ret i32 0
}
