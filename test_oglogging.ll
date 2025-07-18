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


; Interface value creation runtime function
declare i8* @cursed_create_interface_value(i8*, i8*, i8*)

; Interface value creation wrapper
define i8* @create_interface_value(i8* %vtable_ptr, i8* %data_ptr, i8* %type_name) {
entry:
    %interface_value = call i8* @cursed_create_interface_value(i8* %vtable_ptr, i8* %data_ptr, i8* %type_name)
    ret i8* %interface_value
}


; Method dispatch runtime function
declare i8* @cursed_dispatch_method(i8*, i8*, i8*, i32)

; Method dispatch wrapper with optimization
define i8* @dispatch_interface_method(i8* %interface_value, i8* %method_name, i8* %args, i32 %arg_count) {
entry:
    ; Extract vtable from interface value
    %interface_ptr = bitcast i8* %interface_value to {i8*, i8*}*
    %vtable_ptr_ptr = getelementptr {i8*, i8*}, {i8*, i8*}* %interface_ptr, i32 0, i32 0
    %vtable_ptr = load i8*, i8** %vtable_ptr_ptr
    
    ; Extract data pointer
    %data_ptr_ptr = getelementptr {i8*, i8*}, {i8*, i8*}* %interface_ptr, i32 0, i32 1
    %data_ptr = load i8*, i8** %data_ptr_ptr
    
    ; Dispatch method call
    %result = call i8* @cursed_dispatch_method(i8* %vtable_ptr, i8* %method_name, i8* %args, i32 %arg_count)
    ret i8* %result
}


; Interface type checking runtime function
declare i1 @cursed_implements_interface(i8*, i8*)

; Interface type checking wrapper
define i1 @check_interface_implementation(i8* %type_name, i8* %interface_name) {
entry:
    %result = call i1 @cursed_implements_interface(i8* %type_name, i8* %interface_name)
    ret i1 %result
}


; Runtime vtable lookup
declare i8* @cursed_runtime_get_vtable(i8*, i8*)

define i8* @get_vtable_runtime(i8* %type_name, i8* %interface_name) {
entry:
    %vtable = call i8* @cursed_runtime_get_vtable(i8* %type_name, i8* %interface_name)
    ret i8* %vtable
}


; Main function entry point

; String constants
@.str.13 = private unnamed_addr constant [19 x i8] c"✅ Constants work\00", align 1
@.str.1 = private unnamed_addr constant [23 x i8] c"Basic log message test\00", align 1
@.str.7 = private unnamed_addr constant [24 x i8] c"✅ All log levels work\00", align 1
@.str.16 = private unnamed_addr constant [20 x i8] c"✅ Edge cases work\00", align 1
@.str.3 = private unnamed_addr constant [14 x i8] c"Debug message\00", align 1
@.str.15 = private unnamed_addr constant [30 x i8] c"Test with special chars: 🚀\00", align 1
@.str.10 = private unnamed_addr constant [7 x i8] c" WARN=\00", align 1
@.str.0 = private unnamed_addr constant [28 x i8] c"Testing oglogging module...\00", align 1
@.str.12 = private unnamed_addr constant [8 x i8] c" FATAL=\00", align 1
@.str.14 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.11 = private unnamed_addr constant [8 x i8] c" ERROR=\00", align 1
@.str.6 = private unnamed_addr constant [14 x i8] c"Error message\00", align 1
@.str.4 = private unnamed_addr constant [13 x i8] c"Info message\00", align 1
@.str.8 = private unnamed_addr constant [7 x i8] c"DEBUG=\00", align 1
@.str.9 = private unnamed_addr constant [7 x i8] c" INFO=\00", align 1
@.str.5 = private unnamed_addr constant [16 x i8] c"Warning message\00", align 1
@.str.17 = private unnamed_addr constant [36 x i8] c"🎉 oglogging module tests passed!\00", align 1
@.str.2 = private unnamed_addr constant [24 x i8] c"✅ Basic logging works\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %1 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.1, i64 0, i64 0
  %2 = call i32 @Spill(i32 %1)
  %2 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.2, i64 0, i64 0
  %3 = call i32 @puts(i8* %2)
  %3 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.3, i64 0, i64 0
  %4 = call i32 @Debug(i32 %3)
  %4 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.4, i64 0, i64 0
  %5 = call i32 @Info(i32 %4)
  %5 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.5, i64 0, i64 0
  %6 = call i32 @Warn(i32 %5)
  %6 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.6, i64 0, i64 0
  %7 = call i32 @Error(i32 %6)
  %7 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.7, i64 0, i64 0
  %8 = call i32 @puts(i8* %7)
  %8 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.8, i64 0, i64 0
  %9 = add i32 %8, %DEBUG
  %10 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.9, i64 0, i64 0
  %11 = add i32 %9, %10
  %12 = add i32 %11, %INFO
  %13 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.10, i64 0, i64 0
  %14 = add i32 %12, %13
  %15 = add i32 %14, %WARN
  %16 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.11, i64 0, i64 0
  %17 = add i32 %15, %16
  %18 = add i32 %17, %ERROR
  %19 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.12, i64 0, i64 0
  %20 = add i32 %18, %19
  %21 = add i32 %20, %FATAL
  %22 = call i32 @puts(i8* %21)
  %22 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.13, i64 0, i64 0
  %23 = call i32 @puts(i8* %22)
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.14, i64 0, i64 0
  %24 = call i32 @Spill(i32 %23)
  %24 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.15, i64 0, i64 0
  %25 = call i32 @Debug(i32 %24)
  %25 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.16, i64 0, i64 0
  %26 = call i32 @puts(i8* %25)
  %26 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.17, i64 0, i64 0
  %27 = call i32 @puts(i8* %26)
  ret i32 0
}
