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

define i1 @testFunction() {
entry:
  %1 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.0, i64 0, i64 0
  %2 = call i32 @puts(i8* %1)
  %3 = add i32 0, 0
  ; Expression result: %3
  ret i1 1
}

; String constants
@.str.4 = private unnamed_addr constant [36 x i8] c"Float test: Area of circle (r=5) = \00", align 1
@.str.3 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.7 = private unnamed_addr constant [22 x i8] c"Boolean test - true: \00", align 1
@.str.10 = private unnamed_addr constant [34 x i8] c"=== WebAssembly test complete ===\00", align 1
@.str.1 = private unnamed_addr constant [46 x i8] c"=== CURSED WebAssembly Comprehensive Test ===\00", align 1
@.str.0 = private unnamed_addr constant [26 x i8] c"Function call successful!\00", align 1
@.str.6 = private unnamed_addr constant [13 x i8] c"WebAssembly!\00", align 1
@.str.2 = private unnamed_addr constant [25 x i8] c"Integer test: 10 + 20 = \00", align 1
@.str.8 = private unnamed_addr constant [23 x i8] c"Boolean test - false: \00", align 1
@.str.5 = private unnamed_addr constant [12 x i8] c"Hello from \00", align 1
@.str.9 = private unnamed_addr constant [20 x i8] c"Function returned: \00", align 1
define i32 @main() {
  %1 = getelementptr inbounds [46 x i8], [46 x i8]* @.str.1, i64 0, i64 0
  %2 = call i32 @puts(i8* %1)
  %3 = alloca i32, align 4
  store i32 10, i32* %3, align 4
  ; Variable a allocated at %3
  %4 = alloca i32, align 4
  store i32 20, i32* %4, align 4
  ; Variable b allocated at %4
  %5 = load i32, i32* %3, align 4
  %6 = load i32, i32* %4, align 4
  %7 = add i32 %5, %6
  %8 = alloca i32, align 4
  store i32 %7, i32* %8, align 4
  ; Variable sum allocated at %8
  %9 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.2, i64 0, i64 0
  %10 = call i32 @puts(i8* %9)
  %11 = load i32, i32* %8, align 4
  %12 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %13 = call i32 (i8*, ...) @printf(i8* %12, i32 %11)
  %14 = alloca double, align 4
  store double 3.14159, double* %14, align 4
  ; Variable pi allocated at %14
  %15 = alloca double, align 4
  store double 5, double* %15, align 4
  ; Variable radius allocated at %15
  %16 = load i32, i32* %14, align 4
  %17 = load i32, i32* %15, align 4
  %18 = mul i32 %16, %17
  %19 = load i32, i32* %15, align 4
  %20 = mul i32 %18, %19
  %21 = alloca double, align 4
  store double %20, double* %21, align 4
  ; Variable area allocated at %21
  %22 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.4, i64 0, i64 0
  %23 = call i32 @puts(i8* %22)
  %24 = load i32, i32* %21, align 4
  %25 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %26 = call i32 (i8*, ...) @printf(i8* %25, i32 %24)
  %27 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.5, i64 0, i64 0
  %28 = alloca i8*, align 4
  store i8* %27, i8** %28, align 4
  ; Variable greeting allocated at %28
  %29 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.6, i64 0, i64 0
  %30 = alloca i8*, align 4
  store i8* %29, i8** %30, align 4
  ; Variable target allocated at %30
  %31 = load i8*, i8** %28, align 4
  %32 = call i32 @puts(i8* %31)
  %33 = load i32, i32* %30, align 4
  %34 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %35 = call i32 (i8*, ...) @printf(i8* %34, i32 %33)
  %36 = alloca i1, align 4
  store i1 1, i1* %36, align 4
  ; Variable flag1 allocated at %36
  %37 = alloca i1, align 4
  store i1 0, i1* %37, align 4
  ; Variable flag2 allocated at %37
  %38 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.7, i64 0, i64 0
  %39 = call i32 @puts(i8* %38)
  %40 = load i1, i1* %36, align 4
  %41 = zext i1 %40 to i32
  %42 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %43 = call i32 (i8*, ...) @printf(i8* %42, i32 %41)
  %44 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.8, i64 0, i64 0
  %45 = call i32 @puts(i8* %44)
  %46 = load i1, i1* %37, align 4
  %47 = zext i1 %46 to i32
  %48 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %49 = call i32 (i8*, ...) @printf(i8* %48, i32 %47)
  %50 = call i32 @testFunction()
  %51 = alloca i1, align 4
  store i1 %50, i1* %51, align 4
  ; Variable result allocated at %51
  %52 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.9, i64 0, i64 0
  %53 = call i32 @puts(i8* %52)
  %54 = load i32, i32* %51, align 4
  %55 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %56 = call i32 (i8*, ...) @printf(i8* %55, i32 %54)
  %57 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.10, i64 0, i64 0
  %58 = call i32 @puts(i8* %57)
  ret i32 0
}