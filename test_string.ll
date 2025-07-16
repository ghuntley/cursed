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
; mod module declarations


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



; String constants
@.str.65 = private unnamed_addr constant [6 x i8] c"Hello\00", align 1
@.str.34 = private unnamed_addr constant [4 x i8] c"123\00", align 1
@.str.77 = private unnamed_addr constant [10 x i8] c"2 + 3 = 5\00", align 1
@.str.41 = private unnamed_addr constant [8 x i8] c"test456\00", align 1
@.str.68 = private unnamed_addr constant [23 x i8] c"string_format function\00", align 1
@.str.86 = private unnamed_addr constant [29 x i8] c"string_char_code_at function\00", align 1
@.str.27 = private unnamed_addr constant [4 x i8] c"xyz\00", align 1
@.str.51 = private unnamed_addr constant [30 x i8] c"string_replace_first function\00", align 1
@.str.1 = private unnamed_addr constant [23 x i8] c"string_length function\00", align 1
@.str.72 = private unnamed_addr constant [29 x i8] c"string_format_three function\00", align 1
@.str.70 = private unnamed_addr constant [6 x i8] c"World\00", align 1
@.str.4 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.13 = private unnamed_addr constant [4 x i8] c"cba\00", align 1
@.str.25 = private unnamed_addr constant [12 x i8] c"test string\00", align 1
@.str.87 = private unnamed_addr constant [48 x i8] c"🎉 CURSED String Library v2.0 Tests Complete!\00", align 1
@.str.47 = private unnamed_addr constant [8 x i8] c"  hello\00", align 1
@.str.56 = private unnamed_addr constant [17 x i8] c"demo string test\00", align 1
@.str.19 = private unnamed_addr constant [4 x i8] c"ABC\00", align 1
@.str.7 = private unnamed_addr constant [7 x i8] c" world\00", align 1
@.str.20 = private unnamed_addr constant [25 x i8] c"string_to_lower function\00", align 1
@.str.38 = private unnamed_addr constant [25 x i8] c"string_is_alpha function\00", align 1
@.str.42 = private unnamed_addr constant [7 x i8] c"hello!\00", align 1
@.str.89 = private unnamed_addr constant [52 x i8] c"🚀 Production-ready string manipulation available\00", align 1
@.str.29 = private unnamed_addr constant [28 x i8] c"string_starts_with function\00", align 1
@.str.60 = private unnamed_addr constant [12 x i8] c"hi world hi\00", align 1
@.str.63 = private unnamed_addr constant [2 x i8] c"b\00", align 1
@.str.11 = private unnamed_addr constant [24 x i8] c"string_reverse function\00", align 1
@.str.15 = private unnamed_addr constant [5 x i8] c"tset\00", align 1
@.str.10 = private unnamed_addr constant [8 x i8] c"testing\00", align 1
@.str.23 = private unnamed_addr constant [12 x i8] c"programming\00", align 1
@.str.40 = private unnamed_addr constant [32 x i8] c"string_is_alphanumeric function\00", align 1
@.str.73 = private unnamed_addr constant [13 x i8] c"{} + {} = {}\00", align 1
@.str.9 = private unnamed_addr constant [4 x i8] c"ing\00", align 1
@.str.78 = private unnamed_addr constant [25 x i8] c"string_pad_left function\00", align 1
@.str.55 = private unnamed_addr constant [5 x i8] c"demo\00", align 1
@.str.64 = private unnamed_addr constant [36 x i8] c"string_compare_ignore_case function\00", align 1
@.str.80 = private unnamed_addr constant [2 x i8] c" \00", align 1
@.str.85 = private unnamed_addr constant [24 x i8] c"string_char_at function\00", align 1
@.str.76 = private unnamed_addr constant [2 x i8] c"5\00", align 1
@.str.22 = private unnamed_addr constant [6 x i8] c"world\00", align 1
@.str.52 = private unnamed_addr constant [9 x i8] c"universe\00", align 1
@.str.45 = private unnamed_addr constant [8 x i8] c" world \00", align 1
@.str.67 = private unnamed_addr constant [23 x i8] c"string_substr function\00", align 1
@.str.30 = private unnamed_addr constant [5 x i8] c"prog\00", align 1
@.str.28 = private unnamed_addr constant [25 x i8] c"string_index_of function\00", align 1
@.str.18 = private unnamed_addr constant [5 x i8] c"TEST\00", align 1
@.str.21 = private unnamed_addr constant [25 x i8] c"string_contains function\00", align 1
@.str.82 = private unnamed_addr constant [26 x i8] c"string_pad_right function\00", align 1
@.str.8 = private unnamed_addr constant [12 x i8] c"hello world\00", align 1
@.str.14 = private unnamed_addr constant [6 x i8] c"olleh\00", align 1
@.str.17 = private unnamed_addr constant [6 x i8] c"HELLO\00", align 1
@.str.0 = private unnamed_addr constant [47 x i8] c"CURSED String Library v2.0 Comprehensive Tests\00", align 1
@.str.50 = private unnamed_addr constant [7 x i8] c"world \00", align 1
@.str.36 = private unnamed_addr constant [2 x i8] c"0\00", align 1
@.str.31 = private unnamed_addr constant [26 x i8] c"string_ends_with function\00", align 1
@.str.32 = private unnamed_addr constant [5 x i8] c"ming\00", align 1
@.str.33 = private unnamed_addr constant [27 x i8] c"string_is_numeric function\00", align 1
@.str.39 = private unnamed_addr constant [9 x i8] c"hello123\00", align 1
@.str.62 = private unnamed_addr constant [24 x i8] c"string_compare function\00", align 1
@.str.71 = private unnamed_addr constant [14 x i8] c"Hello, World!\00", align 1
@.str.83 = private unnamed_addr constant [9 x i8] c"test0000\00", align 1
@.str.75 = private unnamed_addr constant [2 x i8] c"3\00", align 1
@.str.35 = private unnamed_addr constant [3 x i8] c"42\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"string_concat function\00", align 1
@.str.3 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
@.str.43 = private unnamed_addr constant [21 x i8] c"string_trim function\00", align 1
@.str.66 = private unnamed_addr constant [26 x i8] c"string_substring function\00", align 1
@.str.26 = private unnamed_addr constant [7 x i8] c"string\00", align 1
@.str.57 = private unnamed_addr constant [28 x i8] c"string_replace_all function\00", align 1
@.str.74 = private unnamed_addr constant [2 x i8] c"2\00", align 1
@.str.12 = private unnamed_addr constant [4 x i8] c"abc\00", align 1
@.str.49 = private unnamed_addr constant [8 x i8] c"hello  \00", align 1
@.str.58 = private unnamed_addr constant [18 x i8] c"hello world hello\00", align 1
@.str.84 = private unnamed_addr constant [11 x i8] c"hello     \00", align 1
@.str.88 = private unnamed_addr constant [50 x i8] c"✅ All 25+ string operations tested successfully\00", align 1
@.str.5 = private unnamed_addr constant [2 x i8] c"a\00", align 1
@.str.44 = private unnamed_addr constant [10 x i8] c"  hello  \00", align 1
@.str.59 = private unnamed_addr constant [3 x i8] c"hi\00", align 1
@.str.46 = private unnamed_addr constant [26 x i8] c"string_trim_left function\00", align 1
@.str.69 = private unnamed_addr constant [11 x i8] c"Hello, {}!\00", align 1
@.str.53 = private unnamed_addr constant [15 x i8] c"hello universe\00", align 1
@.str.61 = private unnamed_addr constant [17 x i8] c"demo string demo\00", align 1
@.str.79 = private unnamed_addr constant [9 x i8] c"0000test\00", align 1
@.str.81 = private unnamed_addr constant [11 x i8] c"     hello\00", align 1
@.str.48 = private unnamed_addr constant [27 x i8] c"string_trim_right function\00", align 1
@.str.54 = private unnamed_addr constant [17 x i8] c"test string test\00", align 1
@.str.16 = private unnamed_addr constant [25 x i8] c"string_to_upper function\00", align 1
@.str.24 = private unnamed_addr constant [5 x i8] c"gram\00", align 1
@.str.37 = private unnamed_addr constant [5 x i8] c"12a3\00", align 1
@.str.2 = private unnamed_addr constant [5 x i8] c"test\00", align 1
define i32 @main() {
  %1 = getelementptr inbounds [47 x i8], [47 x i8]* @.str.0, i64 0, i64 0
  %2 = call i32 @test_start(i32 %1)
  %3 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.1, i64 0, i64 0
  %4 = call i32 @test_start(i32 %3)
  %5 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.2, i64 0, i64 0
  %6 = call i32 @string_length(i32 %5)
  %7 = call i32 @assert_eq_int(i32 %6, i32 4)
  %8 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %9 = call i32 @string_length(i32 %8)
  %10 = call i32 @assert_eq_int(i32 %9, i32 5)
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %12 = call i32 @string_length(i32 %11)
  %13 = call i32 @assert_eq_int(i32 %12, i32 0)
  %14 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.5, i64 0, i64 0
  %15 = call i32 @string_length(i32 %14)
  %16 = call i32 @assert_eq_int(i32 %15, i32 1)
  %17 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.6, i64 0, i64 0
  %18 = call i32 @test_start(i32 %17)
  %19 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %20 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.7, i64 0, i64 0
  %21 = call i32 @string_concat(i32 %19, i32 %20)
  %22 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %23 = call i32 @assert_eq_string(i32 %21, i32 %22)
  %24 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.2, i64 0, i64 0
  %25 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.9, i64 0, i64 0
  %26 = call i32 @string_concat(i32 %24, i32 %25)
  %27 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.10, i64 0, i64 0
  %28 = call i32 @assert_eq_string(i32 %26, i32 %27)
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %30 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %31 = call i32 @string_concat(i32 %29, i32 %30)
  %32 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %33 = call i32 @assert_eq_string(i32 %31, i32 %32)
  %34 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.11, i64 0, i64 0
  %35 = call i32 @test_start(i32 %34)
  %36 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.12, i64 0, i64 0
  %37 = call i32 @string_reverse(i32 %36)
  %38 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.13, i64 0, i64 0
  %39 = call i32 @assert_eq_string(i32 %37, i32 %38)
  %40 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %41 = call i32 @string_reverse(i32 %40)
  %42 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.14, i64 0, i64 0
  %43 = call i32 @assert_eq_string(i32 %41, i32 %42)
  %44 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.2, i64 0, i64 0
  %45 = call i32 @string_reverse(i32 %44)
  %46 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.15, i64 0, i64 0
  %47 = call i32 @assert_eq_string(i32 %45, i32 %46)
  %48 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.16, i64 0, i64 0
  %49 = call i32 @test_start(i32 %48)
  %50 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %51 = call i32 @string_to_upper(i32 %50)
  %52 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.17, i64 0, i64 0
  %53 = call i32 @assert_eq_string(i32 %51, i32 %52)
  %54 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.2, i64 0, i64 0
  %55 = call i32 @string_to_upper(i32 %54)
  %56 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.18, i64 0, i64 0
  %57 = call i32 @assert_eq_string(i32 %55, i32 %56)
  %58 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.12, i64 0, i64 0
  %59 = call i32 @string_to_upper(i32 %58)
  %60 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.19, i64 0, i64 0
  %61 = call i32 @assert_eq_string(i32 %59, i32 %60)
  %62 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.20, i64 0, i64 0
  %63 = call i32 @test_start(i32 %62)
  %64 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.17, i64 0, i64 0
  %65 = call i32 @string_to_lower(i32 %64)
  %66 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %67 = call i32 @assert_eq_string(i32 %65, i32 %66)
  %68 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.18, i64 0, i64 0
  %69 = call i32 @string_to_lower(i32 %68)
  %70 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.2, i64 0, i64 0
  %71 = call i32 @assert_eq_string(i32 %69, i32 %70)
  %72 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.19, i64 0, i64 0
  %73 = call i32 @string_to_lower(i32 %72)
  %74 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.12, i64 0, i64 0
  %75 = call i32 @assert_eq_string(i32 %73, i32 %74)
  %76 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.21, i64 0, i64 0
  %77 = call i32 @test_start(i32 %76)
  %78 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %79 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %80 = call i32 @string_contains(i32 %78, i32 %79)
  %81 = call i32 @assert_true(i32 %80)
  %82 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.23, i64 0, i64 0
  %83 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.24, i64 0, i64 0
  %84 = call i32 @string_contains(i32 %82, i32 %83)
  %85 = call i32 @assert_true(i32 %84)
  %86 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.25, i64 0, i64 0
  %87 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.26, i64 0, i64 0
  %88 = call i32 @string_contains(i32 %86, i32 %87)
  %89 = call i32 @assert_true(i32 %88)
  %90 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %91 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.27, i64 0, i64 0
  %92 = call i32 @string_contains(i32 %90, i32 %91)
  %93 = call i32 @assert_false(i32 %92)
  %94 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.28, i64 0, i64 0
  %95 = call i32 @test_start(i32 %94)
  %96 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %97 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %98 = call i32 @string_index_of(i32 %96, i32 %97)
  %99 = call i32 @assert_eq_int(i32 %98, i32 6)
  %100 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.23, i64 0, i64 0
  %101 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.24, i64 0, i64 0
  %102 = call i32 @string_index_of(i32 %100, i32 %101)
  %103 = call i32 @assert_eq_int(i32 %102, i32 3)
  %104 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %105 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.29, i64 0, i64 0
  %106 = call i32 @test_start(i32 %105)
  %107 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %108 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %109 = call i32 @string_starts_with(i32 %107, i32 %108)
  %110 = call i32 @assert_true(i32 %109)
  %111 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.23, i64 0, i64 0
  %112 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.30, i64 0, i64 0
  %113 = call i32 @string_starts_with(i32 %111, i32 %112)
  %114 = call i32 @assert_true(i32 %113)
  %115 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %116 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %117 = call i32 @string_starts_with(i32 %115, i32 %116)
  %118 = call i32 @assert_false(i32 %117)
  %119 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.31, i64 0, i64 0
  %120 = call i32 @test_start(i32 %119)
  %121 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %122 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %123 = call i32 @string_ends_with(i32 %121, i32 %122)
  %124 = call i32 @assert_true(i32 %123)
  %125 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.23, i64 0, i64 0
  %126 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.32, i64 0, i64 0
  %127 = call i32 @string_ends_with(i32 %125, i32 %126)
  %128 = call i32 @assert_true(i32 %127)
  %129 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %130 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %131 = call i32 @string_ends_with(i32 %129, i32 %130)
  %132 = call i32 @assert_false(i32 %131)
  %133 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.33, i64 0, i64 0
  %134 = call i32 @test_start(i32 %133)
  %135 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.34, i64 0, i64 0
  %136 = call i32 @string_is_numeric(i32 %135)
  %137 = call i32 @assert_true(i32 %136)
  %138 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.35, i64 0, i64 0
  %139 = call i32 @string_is_numeric(i32 %138)
  %140 = call i32 @assert_true(i32 %139)
  %141 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.36, i64 0, i64 0
  %142 = call i32 @string_is_numeric(i32 %141)
  %143 = call i32 @assert_true(i32 %142)
  %144 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.12, i64 0, i64 0
  %145 = call i32 @string_is_numeric(i32 %144)
  %146 = call i32 @assert_false(i32 %145)
  %147 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.37, i64 0, i64 0
  %148 = call i32 @string_is_numeric(i32 %147)
  %149 = call i32 @assert_false(i32 %148)
  %150 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.38, i64 0, i64 0
  %151 = call i32 @test_start(i32 %150)
  %152 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %153 = call i32 @string_is_alpha(i32 %152)
  %154 = call i32 @assert_true(i32 %153)
  %155 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.19, i64 0, i64 0
  %156 = call i32 @string_is_alpha(i32 %155)
  %157 = call i32 @assert_true(i32 %156)
  %158 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.34, i64 0, i64 0
  %159 = call i32 @string_is_alpha(i32 %158)
  %160 = call i32 @assert_false(i32 %159)
  %161 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.39, i64 0, i64 0
  %162 = call i32 @string_is_alpha(i32 %161)
  %163 = call i32 @assert_false(i32 %162)
  %164 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.40, i64 0, i64 0
  %165 = call i32 @test_start(i32 %164)
  %166 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.39, i64 0, i64 0
  %167 = call i32 @string_is_alphanumeric(i32 %166)
  %168 = call i32 @assert_true(i32 %167)
  %169 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.41, i64 0, i64 0
  %170 = call i32 @string_is_alphanumeric(i32 %169)
  %171 = call i32 @assert_true(i32 %170)
  %172 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %173 = call i32 @string_is_alphanumeric(i32 %172)
  %174 = call i32 @assert_true(i32 %173)
  %175 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.34, i64 0, i64 0
  %176 = call i32 @string_is_alphanumeric(i32 %175)
  %177 = call i32 @assert_true(i32 %176)
  %178 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.42, i64 0, i64 0
  %179 = call i32 @string_is_alphanumeric(i32 %178)
  %180 = call i32 @assert_false(i32 %179)
  %181 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.43, i64 0, i64 0
  %182 = call i32 @test_start(i32 %181)
  %183 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.44, i64 0, i64 0
  %184 = call i32 @string_trim(i32 %183)
  %185 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %186 = call i32 @assert_eq_string(i32 %184, i32 %185)
  %187 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.45, i64 0, i64 0
  %188 = call i32 @string_trim(i32 %187)
  %189 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %190 = call i32 @assert_eq_string(i32 %188, i32 %189)
  %191 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %192 = call i32 @string_trim(i32 %191)
  %193 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %194 = call i32 @assert_eq_string(i32 %192, i32 %193)
  %195 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.46, i64 0, i64 0
  %196 = call i32 @test_start(i32 %195)
  %197 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.47, i64 0, i64 0
  %198 = call i32 @string_trim_left(i32 %197)
  %199 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %200 = call i32 @assert_eq_string(i32 %198, i32 %199)
  %201 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.7, i64 0, i64 0
  %202 = call i32 @string_trim_left(i32 %201)
  %203 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %204 = call i32 @assert_eq_string(i32 %202, i32 %203)
  %205 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.48, i64 0, i64 0
  %206 = call i32 @test_start(i32 %205)
  %207 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.49, i64 0, i64 0
  %208 = call i32 @string_trim_right(i32 %207)
  %209 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %210 = call i32 @assert_eq_string(i32 %208, i32 %209)
  %211 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.50, i64 0, i64 0
  %212 = call i32 @string_trim_right(i32 %211)
  %213 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %214 = call i32 @assert_eq_string(i32 %212, i32 %213)
  %215 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.51, i64 0, i64 0
  %216 = call i32 @test_start(i32 %215)
  %217 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %218 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %219 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.52, i64 0, i64 0
  %220 = call i32 @string_replace_first(i32 %217, i32 %218, i32 %219)
  %221 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.53, i64 0, i64 0
  %222 = call i32 @assert_eq_string(i32 %220, i32 %221)
  %223 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.54, i64 0, i64 0
  %224 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.2, i64 0, i64 0
  %225 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.55, i64 0, i64 0
  %226 = call i32 @string_replace_first(i32 %223, i32 %224, i32 %225)
  %227 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.56, i64 0, i64 0
  %228 = call i32 @assert_eq_string(i32 %226, i32 %227)
  %229 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.57, i64 0, i64 0
  %230 = call i32 @test_start(i32 %229)
  %231 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.58, i64 0, i64 0
  %232 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %233 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.59, i64 0, i64 0
  %234 = call i32 @string_replace_all(i32 %231, i32 %232, i32 %233)
  %235 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.60, i64 0, i64 0
  %236 = call i32 @assert_eq_string(i32 %234, i32 %235)
  %237 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.54, i64 0, i64 0
  %238 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.2, i64 0, i64 0
  %239 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.55, i64 0, i64 0
  %240 = call i32 @string_replace_all(i32 %237, i32 %238, i32 %239)
  %241 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.61, i64 0, i64 0
  %242 = call i32 @assert_eq_string(i32 %240, i32 %241)
  %243 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.62, i64 0, i64 0
  %244 = call i32 @test_start(i32 %243)
  %245 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %246 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %247 = call i32 @string_compare(i32 %245, i32 %246)
  %248 = call i32 @assert_eq_int(i32 %247, i32 0)
  %249 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %250 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.63, i64 0, i64 0
  %251 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.5, i64 0, i64 0
  %252 = call i32 @string_compare(i32 %250, i32 %251)
  %253 = call i32 @assert_eq_int(i32 %252, i32 1)
  %254 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.64, i64 0, i64 0
  %255 = call i32 @test_start(i32 %254)
  %256 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.65, i64 0, i64 0
  %257 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %258 = call i32 @string_compare_ignore_case(i32 %256, i32 %257)
  %259 = call i32 @assert_eq_int(i32 %258, i32 0)
  %260 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %261 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.66, i64 0, i64 0
  %262 = call i32 @test_start(i32 %261)
  %263 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %264 = call i32 @string_substring(i32 %263, i32 0, i32 5)
  %265 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %266 = call i32 @assert_eq_string(i32 %264, i32 %265)
  %267 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %268 = call i32 @string_substring(i32 %267, i32 6, i32 11)
  %269 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %270 = call i32 @assert_eq_string(i32 %268, i32 %269)
  %271 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.67, i64 0, i64 0
  %272 = call i32 @test_start(i32 %271)
  %273 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %274 = call i32 @string_substr(i32 %273, i32 0, i32 5)
  %275 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %276 = call i32 @assert_eq_string(i32 %274, i32 %275)
  %277 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %278 = call i32 @string_substr(i32 %277, i32 6, i32 5)
  %279 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %280 = call i32 @assert_eq_string(i32 %278, i32 %279)
  %281 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.68, i64 0, i64 0
  %282 = call i32 @test_start(i32 %281)
  %283 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.69, i64 0, i64 0
  %284 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.70, i64 0, i64 0
  %285 = call i32 @string_format(i32 %283, i32 %284)
  %286 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.71, i64 0, i64 0
  %287 = call i32 @assert_eq_string(i32 %285, i32 %286)
  %288 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.72, i64 0, i64 0
  %289 = call i32 @test_start(i32 %288)
  %290 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.73, i64 0, i64 0
  %291 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.74, i64 0, i64 0
  %292 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.75, i64 0, i64 0
  %293 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.76, i64 0, i64 0
  %294 = call i32 @string_format_three(i32 %290, i32 %291, i32 %292, i32 %293)
  %295 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.77, i64 0, i64 0
  %296 = call i32 @assert_eq_string(i32 %294, i32 %295)
  %297 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.78, i64 0, i64 0
  %298 = call i32 @test_start(i32 %297)
  %299 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.2, i64 0, i64 0
  %300 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.36, i64 0, i64 0
  %301 = call i32 @string_pad_left(i32 %299, i32 8, i32 %300)
  %302 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.79, i64 0, i64 0
  %303 = call i32 @assert_eq_string(i32 %301, i32 %302)
  %304 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %305 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.80, i64 0, i64 0
  %306 = call i32 @string_pad_left(i32 %304, i32 10, i32 %305)
  %307 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.81, i64 0, i64 0
  %308 = call i32 @assert_eq_string(i32 %306, i32 %307)
  %309 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.82, i64 0, i64 0
  %310 = call i32 @test_start(i32 %309)
  %311 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.2, i64 0, i64 0
  %312 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.36, i64 0, i64 0
  %313 = call i32 @string_pad_right(i32 %311, i32 8, i32 %312)
  %314 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.83, i64 0, i64 0
  %315 = call i32 @assert_eq_string(i32 %313, i32 %314)
  %316 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %317 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.80, i64 0, i64 0
  %318 = call i32 @string_pad_right(i32 %316, i32 10, i32 %317)
  %319 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.84, i64 0, i64 0
  %320 = call i32 @assert_eq_string(i32 %318, i32 %319)
  %321 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.85, i64 0, i64 0
  %322 = call i32 @test_start(i32 %321)
  %323 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %324 = call i32 @string_char_at(i32 %323, i32 0)
  %325 = call i32 @assert_eq_string(i32 %324, i32 104)
  %326 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %327 = call i32 @string_char_at(i32 %326, i32 1)
  %328 = call i32 @assert_eq_string(i32 %327, i32 101)
  %329 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.86, i64 0, i64 0
  %330 = call i32 @test_start(i32 %329)
  %331 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %332 = call i32 @string_char_code_at(i32 %331, i32 0)
  %333 = call i32 @assert_eq_int(i32 %332, i32 104)
  %334 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %335 = call i32 @string_char_code_at(i32 %334, i32 1)
  %336 = call i32 @assert_eq_int(i32 %335, i32 101)
  %337 = call i32 @print_test_summary()
  %338 = getelementptr inbounds [48 x i8], [48 x i8]* @.str.87, i64 0, i64 0
  %339 = call i32 @puts(i8* %338)
  %340 = getelementptr inbounds [50 x i8], [50 x i8]* @.str.88, i64 0, i64 0
  %341 = call i32 @puts(i8* %340)
  %342 = getelementptr inbounds [52 x i8], [52 x i8]* @.str.89, i64 0, i64 0
  %343 = call i32 @puts(i8* %342)
  ret i32 0
}
