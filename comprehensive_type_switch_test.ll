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
@.str.7 = private unnamed_addr constant [4 x i8] c"lit\00", align 1
@.str.1 = private unnamed_addr constant [7 x i8] c"normie\00", align 1
@.str.10 = private unnamed_addr constant [10 x i8] c"character\00", align 1
@.str.0 = private unnamed_addr constant [32 x i8] c"Comprehensive type switch tests\00", align 1
@.str.18 = private unnamed_addr constant [14 x i8] c"string: bound\00", align 1
@.str.16 = private unnamed_addr constant [10 x i8] c"integer: \00", align 1
@.str.11 = private unnamed_addr constant [5 x i8] c"test\00", align 1
@.str.14 = private unnamed_addr constant [6 x i8] c"float\00", align 1
@.str.9 = private unnamed_addr constant [4 x i8] c"sip\00", align 1
@.str.3 = private unnamed_addr constant [4 x i8] c"tea\00", align 1
@.str.15 = private unnamed_addr constant [6 x i8] c"bound\00", align 1
@.str.4 = private unnamed_addr constant [7 x i8] c"string\00", align 1
@.str.13 = private unnamed_addr constant [5 x i8] c"meal\00", align 1
@.str.8 = private unnamed_addr constant [8 x i8] c"boolean\00", align 1
@.str.12 = private unnamed_addr constant [17 x i8] c"wildcard matched\00", align 1
@.str.17 = private unnamed_addr constant [9 x i8] c"string: \00", align 1
@.str.5 = private unnamed_addr constant [8 x i8] c"unknown\00", align 1
@.str.2 = private unnamed_addr constant [8 x i8] c"integer\00", align 1
@.str.6 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  %1 = alloca i32, align 4
  store i32 42, i32* %1, align 4
  ; Variable x allocated at %1
  %2 = load i32, i32* %1, align 4
  %3 = call i8* @cursed_get_runtime_type_info(i8* %2)
  %4 = call i1 @cursed_check_type(i8* %3, i8* getelementptr ([7 x i8], [7 x i8]* @str_getelementptr inbounds [7 x i8], [7 x i8]* @.str.1, i64 0, i64 0, i32 0, i32 0))
  br i1 %4, label %typeswitch_arm_0_1, label %typeswitch_check_1
typeswitch_arm_0_1:
  %4 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.2, i64 0, i64 0
  %5 = bitcast i8* %4 to i8*
  br label %typeswitch_exit_0
  %6 = call i1 @cursed_check_type(i8* %3, i8* getelementptr ([4 x i8], [4 x i8]* @str_getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0, i32 0, i32 0))
  br i1 %6, label %typeswitch_arm_1_1, label %typeswitch_check_2
typeswitch_arm_1_1:
  %6 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.4, i64 0, i64 0
  %7 = bitcast i8* %6 to i8*
  br label %typeswitch_exit_0
  %8 = i1 1
  br i1 %8, label %typeswitch_arm_2_1, label %typeswitch_default_1
typeswitch_arm_2_1:
  %8 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.5, i64 0, i64 0
  %9 = bitcast i8* %8 to i8*
  br label %typeswitch_exit_0
typeswitch_default_1:
  ; Default case - should not reach here with proper wildcard
  call void @cursed_panic(i8* getelementptr ([25 x i8], [25 x i8]* @str_typeswitch_panic, i32 0, i32 0))
  unreachable
typeswitch_exit_0:
  %10 = phi i8* [ %10, %typeswitch_arm_0_1 ], [ %10, %typeswitch_arm_1_1 ], [ %10, %typeswitch_arm_2_1 ]
  %11 = alloca i8*, align 4
  store i8* %10, i8** %11, align 4
  ; Variable result1 allocated at %11
  %12 = load i32, i32* %11, align 4
  %12 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.2, i64 0, i64 0
  %13 = call i32 @assert_eq_string(i32 %12, i32 %12)
  %13 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.6, i64 0, i64 0
  %14 = alloca i8*, align 4
  store i8* %13, i8** %14, align 4
  ; Variable y allocated at %14
  %15 = load i32, i32* %14, align 4
  %16 = call i8* @cursed_get_runtime_type_info(i8* %15)
  %17 = call i1 @cursed_check_type(i8* %16, i8* getelementptr ([7 x i8], [7 x i8]* @str_getelementptr inbounds [7 x i8], [7 x i8]* @.str.1, i64 0, i64 0, i32 0, i32 0))
  br i1 %17, label %typeswitch_arm_0_3, label %typeswitch_check_1
typeswitch_arm_0_3:
  %17 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.2, i64 0, i64 0
  %18 = bitcast i8* %17 to i8*
  br label %typeswitch_exit_2
  %19 = call i1 @cursed_check_type(i8* %16, i8* getelementptr ([4 x i8], [4 x i8]* @str_getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0, i32 0, i32 0))
  br i1 %19, label %typeswitch_arm_1_3, label %typeswitch_check_2
typeswitch_arm_1_3:
  %19 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.4, i64 0, i64 0
  %20 = bitcast i8* %19 to i8*
  br label %typeswitch_exit_2
  %21 = i1 1
  br i1 %21, label %typeswitch_arm_2_3, label %typeswitch_default_3
typeswitch_arm_2_3:
  %21 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.5, i64 0, i64 0
  %22 = bitcast i8* %21 to i8*
  br label %typeswitch_exit_2
typeswitch_default_3:
  ; Default case - should not reach here with proper wildcard
  call void @cursed_panic(i8* getelementptr ([25 x i8], [25 x i8]* @str_typeswitch_panic, i32 0, i32 0))
  unreachable
typeswitch_exit_2:
  %23 = phi i8* [ %23, %typeswitch_arm_0_3 ], [ %23, %typeswitch_arm_1_3 ], [ %23, %typeswitch_arm_2_3 ]
  %24 = alloca i8*, align 4
  store i8* %23, i8** %24, align 4
  ; Variable result2 allocated at %24
  %25 = load i32, i32* %24, align 4
  %25 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.4, i64 0, i64 0
  %26 = call i32 @assert_eq_string(i32 %25, i32 %25)
  %27 = alloca i1, align 4
  store i1 1, i1* %27, align 4
  ; Variable z allocated at %27
  %28 = load i32, i32* %27, align 4
  %29 = call i8* @cursed_get_runtime_type_info(i8* %28)
  %30 = call i1 @cursed_check_type(i8* %29, i8* getelementptr ([4 x i8], [4 x i8]* @str_getelementptr inbounds [4 x i8], [4 x i8]* @.str.7, i64 0, i64 0, i32 0, i32 0))
  br i1 %30, label %typeswitch_arm_0_5, label %typeswitch_check_1
typeswitch_arm_0_5:
  %30 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.8, i64 0, i64 0
  %31 = bitcast i8* %30 to i8*
  br label %typeswitch_exit_4
  %32 = i1 1
  br i1 %32, label %typeswitch_arm_1_5, label %typeswitch_default_5
typeswitch_arm_1_5:
  %32 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.5, i64 0, i64 0
  %33 = bitcast i8* %32 to i8*
  br label %typeswitch_exit_4
typeswitch_default_5:
  ; Default case - should not reach here with proper wildcard
  call void @cursed_panic(i8* getelementptr ([25 x i8], [25 x i8]* @str_typeswitch_panic, i32 0, i32 0))
  unreachable
typeswitch_exit_4:
  %34 = phi i8* [ %34, %typeswitch_arm_0_5 ], [ %34, %typeswitch_arm_1_5 ]
  %35 = alloca i8*, align 4
  store i8* %34, i8** %35, align 4
  ; Variable result3 allocated at %35
  %36 = load i32, i32* %35, align 4
  %36 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.8, i64 0, i64 0
  %37 = call i32 @assert_eq_string(i32 %36, i32 %36)
  %37 = alloca i8, align 4
  store i8 65, i8* %37, align 4
  ; Variable ch allocated at %37
  %38 = load i32, i32* %37, align 4
  %39 = call i8* @cursed_get_runtime_type_info(i8* %38)
  %40 = call i1 @cursed_check_type(i8* %39, i8* getelementptr ([4 x i8], [4 x i8]* @str_getelementptr inbounds [4 x i8], [4 x i8]* @.str.9, i64 0, i64 0, i32 0, i32 0))
  br i1 %40, label %typeswitch_arm_0_7, label %typeswitch_check_1
typeswitch_arm_0_7:
  %40 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.10, i64 0, i64 0
  %41 = bitcast i8* %40 to i8*
  br label %typeswitch_exit_6
  %42 = i1 1
  br i1 %42, label %typeswitch_arm_1_7, label %typeswitch_default_7
typeswitch_arm_1_7:
  %42 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.5, i64 0, i64 0
  %43 = bitcast i8* %42 to i8*
  br label %typeswitch_exit_6
typeswitch_default_7:
  ; Default case - should not reach here with proper wildcard
  call void @cursed_panic(i8* getelementptr ([25 x i8], [25 x i8]* @str_typeswitch_panic, i32 0, i32 0))
  unreachable
typeswitch_exit_6:
  %44 = phi i8* [ %44, %typeswitch_arm_0_7 ], [ %44, %typeswitch_arm_1_7 ]
  %45 = alloca i8*, align 4
  store i8* %44, i8** %45, align 4
  ; Variable result4 allocated at %45
  %46 = load i32, i32* %45, align 4
  %46 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.10, i64 0, i64 0
  %47 = call i32 @assert_eq_string(i32 %46, i32 %46)
  %47 = alloca i32, align 4
  store i32 100, i32* %47, align 4
  ; Variable value allocated at %47
  %48 = load i32, i32* %47, align 4
  %49 = call i8* @cursed_get_runtime_type_info(i8* %48)
  %50 = call i1 @cursed_check_type(i8* %49, i8* getelementptr ([7 x i8], [7 x i8]* @str_getelementptr inbounds [7 x i8], [7 x i8]* @.str.1, i64 0, i64 0, i32 0, i32 0))
  br i1 %50, label %typeswitch_arm_0_9, label %typeswitch_check_1
typeswitch_arm_0_9:
  %51 = bitcast i8* %48 to i8*
  %51 = load i32, i32* %51, align 4
  %52 = mul i32 %51, 2
  %53 = bitcast i8* %52 to i8*
  br label %typeswitch_exit_8
  %54 = i1 1
  br i1 %54, label %typeswitch_arm_1_9, label %typeswitch_default_9
typeswitch_arm_1_9:
  %54 = bitcast i8* 0 to i8*
  br label %typeswitch_exit_8
typeswitch_default_9:
  ; Default case - should not reach here with proper wildcard
  call void @cursed_panic(i8* getelementptr ([25 x i8], [25 x i8]* @str_typeswitch_panic, i32 0, i32 0))
  unreachable
typeswitch_exit_8:
  %55 = phi i8* [ %55, %typeswitch_arm_0_9 ], [ %55, %typeswitch_arm_1_9 ]
  %56 = alloca i32, align 4
  store i32 %55, i32* %56, align 4
  ; Variable doubled allocated at %56
  %57 = load i32, i32* %56, align 4
  %57 = call i32 @assert_eq_int(i32 %57, i32 200)
  %57 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.11, i64 0, i64 0
  %58 = alloca i8*, align 4
  store i8* %57, i8** %58, align 4
  ; Variable any_value allocated at %58
  %59 = load i32, i32* %58, align 4
  %60 = call i8* @cursed_get_runtime_type_info(i8* %59)
  %61 = call i1 @cursed_check_type(i8* %60, i8* getelementptr ([7 x i8], [7 x i8]* @str_getelementptr inbounds [7 x i8], [7 x i8]* @.str.1, i64 0, i64 0, i32 0, i32 0))
  br i1 %61, label %typeswitch_arm_0_11, label %typeswitch_check_1
typeswitch_arm_0_11:
  %61 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.2, i64 0, i64 0
  %62 = bitcast i8* %61 to i8*
  br label %typeswitch_exit_10
  %63 = i1 1
  br i1 %63, label %typeswitch_arm_1_11, label %typeswitch_default_11
typeswitch_arm_1_11:
  %63 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.12, i64 0, i64 0
  %64 = bitcast i8* %63 to i8*
  br label %typeswitch_exit_10
typeswitch_default_11:
  ; Default case - should not reach here with proper wildcard
  call void @cursed_panic(i8* getelementptr ([25 x i8], [25 x i8]* @str_typeswitch_panic, i32 0, i32 0))
  unreachable
typeswitch_exit_10:
  %65 = phi i8* [ %65, %typeswitch_arm_0_11 ], [ %65, %typeswitch_arm_1_11 ]
  %66 = alloca i8*, align 4
  store i8* %65, i8** %66, align 4
  ; Variable wildcard_result allocated at %66
  %67 = load i32, i32* %66, align 4
  %67 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.12, i64 0, i64 0
  %68 = call i32 @assert_eq_string(i32 %67, i32 %67)
  %68 = alloca double, align 4
  store double 3.14, double* %68, align 4
  ; Variable float_val allocated at %68
  %69 = load i32, i32* %68, align 4
  %70 = call i8* @cursed_get_runtime_type_info(i8* %69)
  %71 = call i1 @cursed_check_type(i8* %70, i8* getelementptr ([7 x i8], [7 x i8]* @str_getelementptr inbounds [7 x i8], [7 x i8]* @.str.1, i64 0, i64 0, i32 0, i32 0))
  br i1 %71, label %typeswitch_arm_0_13, label %typeswitch_check_1
typeswitch_arm_0_13:
  %71 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.2, i64 0, i64 0
  %72 = bitcast i8* %71 to i8*
  br label %typeswitch_exit_12
  %73 = call i1 @cursed_check_type(i8* %70, i8* getelementptr ([4 x i8], [4 x i8]* @str_getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0, i32 0, i32 0))
  br i1 %73, label %typeswitch_arm_1_13, label %typeswitch_check_2
typeswitch_arm_1_13:
  %73 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.4, i64 0, i64 0
  %74 = bitcast i8* %73 to i8*
  br label %typeswitch_exit_12
  %75 = call i1 @cursed_check_type(i8* %70, i8* getelementptr ([5 x i8], [5 x i8]* @str_getelementptr inbounds [5 x i8], [5 x i8]* @.str.13, i64 0, i64 0, i32 0, i32 0))
  br i1 %75, label %typeswitch_arm_2_13, label %typeswitch_check_3
typeswitch_arm_2_13:
  %75 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.14, i64 0, i64 0
  %76 = bitcast i8* %75 to i8*
  br label %typeswitch_exit_12
  %77 = i1 1
  br i1 %77, label %typeswitch_arm_3_13, label %typeswitch_default_13
typeswitch_arm_3_13:
  %77 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.5, i64 0, i64 0
  %78 = bitcast i8* %77 to i8*
  br label %typeswitch_exit_12
typeswitch_default_13:
  ; Default case - should not reach here with proper wildcard
  call void @cursed_panic(i8* getelementptr ([25 x i8], [25 x i8]* @str_typeswitch_panic, i32 0, i32 0))
  unreachable
typeswitch_exit_12:
  %79 = phi i8* [ %79, %typeswitch_arm_0_13 ], [ %79, %typeswitch_arm_1_13 ], [ %79, %typeswitch_arm_2_13 ], [ %79, %typeswitch_arm_3_13 ]
  %80 = alloca i8*, align 4
  store i8* %79, i8** %80, align 4
  ; Variable float_result allocated at %80
  %81 = load i32, i32* %80, align 4
  %81 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.14, i64 0, i64 0
  %82 = call i32 @assert_eq_string(i32 %81, i32 %81)
  %82 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.15, i64 0, i64 0
  %83 = alloca i8*, align 4
  store i8* %82, i8** %83, align 4
  ; Variable str_val allocated at %83
  %84 = load i32, i32* %83, align 4
  %85 = call i8* @cursed_get_runtime_type_info(i8* %84)
  %86 = call i1 @cursed_check_type(i8* %85, i8* getelementptr ([7 x i8], [7 x i8]* @str_getelementptr inbounds [7 x i8], [7 x i8]* @.str.1, i64 0, i64 0, i32 0, i32 0))
  br i1 %86, label %typeswitch_arm_0_15, label %typeswitch_check_1
typeswitch_arm_0_15:
  %87 = bitcast i8* %84 to i8*
  %87 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.16, i64 0, i64 0
  %88 = load i32, i32* %87, align 4
  %89 = add i32 %87, %88
  %90 = bitcast i8* %89 to i8*
  br label %typeswitch_exit_14
  %91 = call i1 @cursed_check_type(i8* %85, i8* getelementptr ([4 x i8], [4 x i8]* @str_getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0, i32 0, i32 0))
  br i1 %91, label %typeswitch_arm_1_15, label %typeswitch_check_2
typeswitch_arm_1_15:
  %92 = bitcast i8* %84 to i8*
  %92 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.17, i64 0, i64 0
  %93 = load i32, i32* %92, align 4
  %94 = add i32 %92, %93
  %95 = bitcast i8* %94 to i8*
  br label %typeswitch_exit_14
  %96 = i1 1
  br i1 %96, label %typeswitch_arm_2_15, label %typeswitch_default_15
typeswitch_arm_2_15:
  %96 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.5, i64 0, i64 0
  %97 = bitcast i8* %96 to i8*
  br label %typeswitch_exit_14
typeswitch_default_15:
  ; Default case - should not reach here with proper wildcard
  call void @cursed_panic(i8* getelementptr ([25 x i8], [25 x i8]* @str_typeswitch_panic, i32 0, i32 0))
  unreachable
typeswitch_exit_14:
  %98 = phi i8* [ %98, %typeswitch_arm_0_15 ], [ %98, %typeswitch_arm_1_15 ], [ %98, %typeswitch_arm_2_15 ]
  %99 = alloca i8*, align 4
  store i8* %98, i8** %99, align 4
  ; Variable bound_result allocated at %99
  %100 = load i32, i32* %99, align 4
  %100 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.18, i64 0, i64 0
  %101 = call i32 @assert_eq_string(i32 %100, i32 %100)
  %102 = call i32 @print_test_summary()
  ret i32 0
}
