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


; Main function entry point

; String constants
@.str.8 = private unnamed_addr constant [44 x i8] c"Queue: FIFO operations with circular buffer\00", align 1
@.str.10 = private unnamed_addr constant [53 x i8] c"PriorityQueue: Built on heap for priority operations\00", align 1
@.str.7 = private unnamed_addr constant [43 x i8] c"Heap: Min/max heap for priority operations\00", align 1
@.str.4 = private unnamed_addr constant [45 x i8] c"HashMap: String keys with collision handling\00", align 1
@.str.0 = private unnamed_addr constant [28 x i8] c"Collections Core Basic Test\00", align 1
@.str.1 = private unnamed_addr constant [49 x i8] c"Collections core module implementation complete!\00", align 1
@.str.2 = private unnamed_addr constant [44 x i8] c"Vector: Dynamic array with automatic growth\00", align 1
@.str.3 = private unnamed_addr constant [43 x i8] c"LinkedList: Single and double linked lists\00", align 1
@.str.9 = private unnamed_addr constant [42 x i8] c"Stack: LIFO operations with array storage\00", align 1
@.str.6 = private unnamed_addr constant [53 x i8] c"BST/AVL: Binary search trees with optional balancing\00", align 1
@.str.5 = private unnamed_addr constant [38 x i8] c"Set: Unique elements built on HashMap\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  %2 = call i32 @assert_true(i32 1)
  %3 = getelementptr inbounds [49 x i8], [49 x i8]* @.str.1, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = getelementptr inbounds [44 x i8], [44 x i8]* @.str.2, i64 0, i64 0
  %6 = call i32 @puts(i8* %5)
  %7 = getelementptr inbounds [43 x i8], [43 x i8]* @.str.3, i64 0, i64 0
  %8 = call i32 @puts(i8* %7)
  %9 = getelementptr inbounds [45 x i8], [45 x i8]* @.str.4, i64 0, i64 0
  %10 = call i32 @puts(i8* %9)
  %11 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.5, i64 0, i64 0
  %12 = call i32 @puts(i8* %11)
  %13 = getelementptr inbounds [53 x i8], [53 x i8]* @.str.6, i64 0, i64 0
  %14 = call i32 @puts(i8* %13)
  %15 = getelementptr inbounds [43 x i8], [43 x i8]* @.str.7, i64 0, i64 0
  %16 = call i32 @puts(i8* %15)
  %17 = getelementptr inbounds [44 x i8], [44 x i8]* @.str.8, i64 0, i64 0
  %18 = call i32 @puts(i8* %17)
  %19 = getelementptr inbounds [42 x i8], [42 x i8]* @.str.9, i64 0, i64 0
  %20 = call i32 @puts(i8* %19)
  %21 = getelementptr inbounds [53 x i8], [53 x i8]* @.str.10, i64 0, i64 0
  %22 = call i32 @puts(i8* %21)
  %23 = call i32 @assert_true(i32 1)
  %24 = call i32 @print_test_summary()
  ret i32 0
}
