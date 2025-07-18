; WebAssembly target module
target triple = "wasm32-unknown-unknown"
target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"

; WebAssembly runtime functions
declare void @cursed_print(i8*)
declare void @cursed_print_int(i32)
declare void @cursed_print_float(float)
declare i8* @__wasm_malloc(i32)
declare void @__wasm_free(i8*)

; CURSED Language - Advanced LLVM Compilation

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

; Function: add_numbers
define i32 @add_numbers(i32 %arg_0, i32 %arg_1) {
entry:
  %0 = alloca i32, align 4
  store i32 %arg_0, i32* %0, align 4
  %1 = alloca i32, align 4
  store i32 %arg_1, i32* %1, align 4
  %1 = load i32, i32* %0, align 4
  %2 = load i32, i32* %1, align 4
  %3 = add i32 %1, %2
  ret i32 %3
}

; Main function entry point

; String constants
@.str.3 = private unnamed_addr constant [7 x i8] c"Text: \00", align 1
@.str.9 = private unnamed_addr constant [17 x i8] c"Loop iteration: \00", align 1
@.str.10 = private unnamed_addr constant [2 x i8] c"1\00", align 1
@.str.11 = private unnamed_addr constant [2 x i8] c"2\00", align 1
@.str.12 = private unnamed_addr constant [2 x i8] c"3\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"Sum is greater than 10\00", align 1
@.str.5 = private unnamed_addr constant [6 x i8] c"Sum: \00", align 1
@.str.2 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.7 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.13 = private unnamed_addr constant [29 x i8] c"WASM advanced test complete!\00", align 1
@.str.8 = private unnamed_addr constant [32 x i8] c"Sum is less than or equal to 10\00", align 1
@.str.1 = private unnamed_addr constant [9 x i8] c"Number: \00", align 1
@.str.0 = private unnamed_addr constant [10 x i8] c"WASM Test\00", align 1
@.str.4 = private unnamed_addr constant [9 x i8] c"Result: \00", align 1
define i32 @main() {
entry:
  %3 = alloca i32, align 4
  store i32 42, i32* %3, align 4
  ; Variable number allocated at %3
  %3 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.0, i64 0, i64 0
  %4 = alloca i8*, align 4
  store i8* %3, i8** %4, align 4
  ; Variable text allocated at %4
  %4 = load i32, i32* %3, align 4
  %5 = add i32 %4, 10
  %6 = alloca i32, align 4
  store i32 %5, i32* %6, align 4
  ; Variable result allocated at %6
  %6 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.1, i64 0, i64 0
  %7 = call i32 @puts(i8* %6)
  %8 = load i32, i32* %3, align 4
  %9 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %10 = call i32 (i8*, ...) @printf(i8* %9, i32 %8)
  %10 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.3, i64 0, i64 0
  %11 = call i32 @puts(i8* %10)
  %12 = load i32, i32* %4, align 4
  %13 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %14 = call i32 (i8*, ...) @printf(i8* %13, i32 %12)
  %14 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.4, i64 0, i64 0
  %15 = call i32 @puts(i8* %14)
  %16 = load i32, i32* %6, align 4
  %17 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %18 = call i32 (i8*, ...) @printf(i8* %17, i32 %16)
  %17 = call i32 @add_numbers(i32 5, i32 7)
  %18 = alloca i32, align 4
  store i32 %17, i32* %18, align 4
  ; Variable sum allocated at %18
  %18 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.5, i64 0, i64 0
  %19 = call i32 @puts(i8* %18)
  %20 = load i32, i32* %18, align 4
  %21 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %22 = call i32 (i8*, ...) @printf(i8* %21, i32 %20)
  ; DEBUG: generate_if_statement_with_init called
  ; DEBUG: about to process condition
  %22 = load i32, i32* %18, align 4
  %23 = icmp sgt i32 %22, 10
  br i1 %23, label %label0, label %label1
label0:
  %23 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.6, i64 0, i64 0
  %24 = call i32 @puts(i8* %23)
  br label %label2
label1:
  br label %label2
label2:
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.7, i64 0, i64 0
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.7, i64 0, i64 0
  %24 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.8, i64 0, i64 0
  %25 = call i32 @puts(i8* %24)
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.7, i64 0, i64 0
  %25 = alloca i32, align 4
  store i32 0, i32* %25, align 4
  ; Short declaration: i := 0 (i32)
  br label %label3
label3:
  %25 = load i32, i32* %25, align 4
  %26 = icmp slt i32 %25, 3
  br i1 %26, label %label4, label %label6
label4:
  %26 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.9, i64 0, i64 0
  %27 = call i32 @puts(i8* %26)
  %28 = load i32, i32* %25, align 4
  %29 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %30 = call i32 (i8*, ...) @printf(i8* %29, i32 %28)
  br label %label5
label5:
  %31 = load i32, i32* %25, align 4
  %32 = add i32 %31, 1
  store i32 %32, i32* %25, align 4
  br label %label3
label6:
  %32 = alloca i32, align 4
  store i32 0, i32* %32, align 4
  ; Variable numbers allocated at %32
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.7, i64 0, i64 0
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.7, i64 0, i64 0
  %32 = alloca [3 x i32], align 4
  %33 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.10, i64 0, i64 0
  %34 = getelementptr inbounds [3 x i32], [3 x i32]* %32, i64 0, i64 0
  store i32 %33, i32* %34, align 4
  %35 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.11, i64 0, i64 0
  %36 = getelementptr inbounds [3 x i32], [3 x i32]* %32, i64 0, i64 1
  store i32 %35, i32* %36, align 4
  %37 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.12, i64 0, i64 0
  %38 = getelementptr inbounds [3 x i32], [3 x i32]* %32, i64 0, i64 2
  store i32 %37, i32* %38, align 4
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.7, i64 0, i64 0
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.7, i64 0, i64 0
  %37 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.13, i64 0, i64 0
  %38 = call i32 @puts(i8* %37)
  ret i32 0
}