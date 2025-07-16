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
@.str.9 = private unnamed_addr constant [18 x i8] c"Different content\00", align 1
@.str.45 = private unnamed_addr constant [28 x i8] c"Testz framework integration\00", align 1
@.str.58 = private unnamed_addr constant [33 x i8] c"✅ Multiple snapshot management\00", align 1
@.str.24 = private unnamed_addr constant [24 x i8] c"Multiple snapshot tests\00", align 1
@.str.50 = private unnamed_addr constant [46 x i8] c"\0A🎯 Snapshot Testing Framework Test Report:\00", align 1
@.str.55 = private unnamed_addr constant [37 x i8] c"✅ File path building and utilities\00", align 1
@.str.31 = private unnamed_addr constant [35 x i8] c"✅ Multiple snapshot tests passed\00", align 1
@.str.56 = private unnamed_addr constant [34 x i8] c"✅ Edge cases and error handling\00", align 1
@.str.49 = private unnamed_addr constant [44 x i8] c"✅ Testz framework integration test passed\00", align 1
@.str.47 = private unnamed_addr constant [38 x i8] c"Integration test with testz framework\00", align 1
@.str.5 = private unnamed_addr constant [29 x i8] c"Identical content comparison\00", align 1
@.str.27 = private unnamed_addr constant [18 x i8] c"Third test output\00", align 1
@.str.10 = private unnamed_addr constant [35 x i8] c"✅ Content comparison test passed\00", align 1
@.str.32 = private unnamed_addr constant [26 x i8] c"Update mode functionality\00", align 1
@.str.38 = private unnamed_addr constant [11 x i8] c"Edge cases\00", align 1
@.str.16 = private unnamed_addr constant [32 x i8] c"✅ Diff generation test passed\00", align 1
@.str.4 = private unnamed_addr constant [31 x i8] c"✅ Basic snapshot test passed\00", align 1
@.str.15 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.25 = private unnamed_addr constant [18 x i8] c"First test output\00", align 1
@.str.42 = private unnamed_addr constant [26 x i8] c"Special chars: !@#$%^&*()\00", align 1
@.str.22 = private unnamed_addr constant [2 x i8] c"2\00", align 1
@.str.44 = private unnamed_addr constant [27 x i8] c"✅ Edge cases test passed\00", align 1
@.str.46 = private unnamed_addr constant [5 x i8] c"test\00", align 1
@.str.54 = private unnamed_addr constant [43 x i8] c"✅ Content comparison and diff generation\00", align 1
@.str.21 = private unnamed_addr constant [25 x i8] c"String utility functions\00", align 1
@.str.53 = private unnamed_addr constant [43 x i8] c"✅ Basic snapshot creation and comparison\00", align 1
@.str.14 = private unnamed_addr constant [10 x i8] c"diff_test\00", align 1
@.str.35 = private unnamed_addr constant [23 x i8] c"Interface test content\00", align 1
@.str.6 = private unnamed_addr constant [40 x i8] c"✅ Identical content comparison passed\00", align 1
@.str.39 = private unnamed_addr constant [11 x i8] c"empty_test\00", align 1
@.str.33 = private unnamed_addr constant [42 x i8] c"✅ Update mode functionality test passed\00", align 1
@.str.0 = private unnamed_addr constant [33 x i8] c"Snapshot Testing Framework Tests\00", align 1
@.str.2 = private unnamed_addr constant [14 x i8] c"Hello, World!\00", align 1
@.str.8 = private unnamed_addr constant [13 x i8] c"Same content\00", align 1
@.str.28 = private unnamed_addr constant [13 x i8] c"multi_test_1\00", align 1
@.str.57 = private unnamed_addr constant [37 x i8] c"✅ Integration with testz framework\00", align 1
@.str.41 = private unnamed_addr constant [12 x i8] c"Single line\00", align 1
@.str.11 = private unnamed_addr constant [16 x i8] c"Diff generation\00", align 1
@.str.7 = private unnamed_addr constant [33 x i8] c"Content comparison functionality\00", align 1
@.str.36 = private unnamed_addr constant [15 x i8] c"interface_test\00", align 1
@.str.52 = private unnamed_addr constant [50 x i8] c"✅ All 11 test categories completed successfully\00", align 1
@.str.29 = private unnamed_addr constant [13 x i8] c"multi_test_2\00", align 1
@.str.60 = private unnamed_addr constant [53 x i8] c"🚀 Snapshot testing framework is production-ready!\00", align 1
@.str.51 = private unnamed_addr constant [45 x i8] c"============================================\00", align 1
@.str.17 = private unnamed_addr constant [23 x i8] c"Snapshot path building\00", align 1
@.str.40 = private unnamed_addr constant [17 x i8] c"single_line_test\00", align 1
@.str.48 = private unnamed_addr constant [23 x i8] c"testz_integration_test\00", align 1
@.str.19 = private unnamed_addr constant [24 x i8] c".snapshots/my_test.snap\00", align 1
@.str.20 = private unnamed_addr constant [39 x i8] c"✅ Snapshot path building test passed\00", align 1
@.str.23 = private unnamed_addr constant [33 x i8] c"✅ String utilities test passed\00", align 1
@.str.12 = private unnamed_addr constant [7 x i8] c"Line 1\00", align 1
@.str.18 = private unnamed_addr constant [8 x i8] c"my_test\00", align 1
@.str.34 = private unnamed_addr constant [32 x i8] c"Compare with snapshot interface\00", align 1
@.str.59 = private unnamed_addr constant [30 x i8] c"✅ Update mode functionality\00", align 1
@.str.1 = private unnamed_addr constant [29 x i8] c"Basic snapshot functionality\00", align 1
@.str.26 = private unnamed_addr constant [19 x i8] c"Second test output\00", align 1
@.str.43 = private unnamed_addr constant [19 x i8] c"special_chars_test\00", align 1
@.str.30 = private unnamed_addr constant [13 x i8] c"multi_test_3\00", align 1
@.str.37 = private unnamed_addr constant [48 x i8] c"✅ Compare with snapshot interface test passed\00", align 1
@.str.13 = private unnamed_addr constant [7 x i8] c"Line 2\00", align 1
@.str.3 = private unnamed_addr constant [11 x i8] c"basic_test\00", align 1
define i32 @main() {
  %1 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.0, i64 0, i64 0
  %2 = call i32 @test_start(i32 %1)
  %3 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.1, i64 0, i64 0
  %4 = call i32 @test_start(i32 %3)
  %5 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.2, i64 0, i64 0
  %6 = alloca i8*, align 4
  store i8* %5, i8** %6, align 4
  ; Variable test_output allocated at %6
  %7 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.3, i64 0, i64 0
  %8 = load i32, i32* %6, align 4
  %9 = call i32 @snapshot_test(i32 %7, i32 %8)
  %10 = alloca i1, align 4
  store i1 %9, i1* %10, align 4
  ; Variable result allocated at %10
  %11 = load i32, i32* %10, align 4
  %12 = call i32 @assert_true(i32 %11)
  %13 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.4, i64 0, i64 0
  %14 = call i32 @puts(i8* %13)
  %15 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.5, i64 0, i64 0
  %16 = call i32 @test_start(i32 %15)
  %17 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.2, i64 0, i64 0
  %18 = alloca i8*, align 4
  store i8* %17, i8** %18, align 4
  ; Variable same_output allocated at %18
  %19 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.3, i64 0, i64 0
  %20 = load i32, i32* %18, align 4
  %21 = call i32 @snapshot_test(i32 %19, i32 %20)
  %22 = alloca i1, align 4
  store i1 %21, i1* %22, align 4
  ; Variable same_result allocated at %22
  %23 = load i32, i32* %22, align 4
  %24 = call i32 @assert_true(i32 %23)
  %25 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.6, i64 0, i64 0
  %26 = call i32 @puts(i8* %25)
  %27 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.7, i64 0, i64 0
  %28 = call i32 @test_start(i32 %27)
  %29 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.8, i64 0, i64 0
  %30 = alloca i8*, align 4
  store i8* %29, i8** %30, align 4
  ; Variable content1 allocated at %30
  %31 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.8, i64 0, i64 0
  %32 = alloca i8*, align 4
  store i8* %31, i8** %32, align 4
  ; Variable content2 allocated at %32
  %33 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.9, i64 0, i64 0
  %34 = alloca i8*, align 4
  store i8* %33, i8** %34, align 4
  ; Variable content3 allocated at %34
  %35 = load i32, i32* %30, align 4
  %36 = load i32, i32* %32, align 4
  %37 = call i32 @compare_content(i32 %35, i32 %36)
  %38 = call i32 @assert_true(i32 %37)
  %39 = load i32, i32* %30, align 4
  %40 = load i32, i32* %34, align 4
  %41 = call i32 @compare_content(i32 %39, i32 %40)
  %42 = call i32 @assert_false(i32 %41)
  %43 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.10, i64 0, i64 0
  %44 = call i32 @puts(i8* %43)
  %45 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.11, i64 0, i64 0
  %46 = call i32 @test_start(i32 %45)
  %47 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.12, i64 0, i64 0
  %48 = alloca i8*, align 4
  store i8* %47, i8** %48, align 4
  ; Variable original allocated at %48
  %49 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.13, i64 0, i64 0
  %50 = alloca i8*, align 4
  store i8* %49, i8** %50, align 4
  ; Variable modified allocated at %50
  %51 = load i32, i32* %48, align 4
  %52 = load i32, i32* %50, align 4
  %53 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.14, i64 0, i64 0
  %54 = call i32 @generate_diff(i32 %51, i32 %52, i32 %53)
  %55 = alloca i8*, align 4
  store i8* %54, i8** %55, align 4
  ; Variable diff_result allocated at %55
  %56 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.15, i64 0, i64 0
  %57 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.15, i64 0, i64 0
  %58 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.16, i64 0, i64 0
  %59 = call i32 @puts(i8* %58)
  %60 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.17, i64 0, i64 0
  %61 = call i32 @test_start(i32 %60)
  %62 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.18, i64 0, i64 0
  %63 = call i32 @build_snapshot_path(i32 %62)
  %64 = alloca i8*, align 4
  store i8* %63, i8** %64, align 4
  ; Variable path allocated at %64
  %65 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.19, i64 0, i64 0
  %66 = alloca i8*, align 4
  store i8* %65, i8** %66, align 4
  ; Variable expected_path allocated at %66
  %67 = load i32, i32* %64, align 4
  %68 = load i32, i32* %66, align 4
  %69 = call i32 @assert_eq_string(i32 %67, i32 %68)
  %70 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.20, i64 0, i64 0
  %71 = call i32 @puts(i8* %70)
  %72 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.21, i64 0, i64 0
  %73 = call i32 @test_start(i32 %72)
  %74 = call i32 @int_to_string(i32 2)
  %75 = alloca i8*, align 4
  store i8* %74, i8** %75, align 4
  ; Variable num_str allocated at %75
  %76 = load i32, i32* %75, align 4
  %77 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.22, i64 0, i64 0
  %78 = call i32 @assert_eq_string(i32 %76, i32 %77)
  %79 = call i32 @max_int(i32 10, i32 20)
  %80 = alloca i32, align 4
  store i32 %79, i32* %80, align 4
  ; Variable max_result allocated at %80
  %81 = load i32, i32* %80, align 4
  %82 = call i32 @assert_eq_int(i32 %81, i32 20)
  %83 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.23, i64 0, i64 0
  %84 = call i32 @puts(i8* %83)
  %85 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.24, i64 0, i64 0
  %86 = call i32 @test_start(i32 %85)
  %87 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.25, i64 0, i64 0
  %88 = alloca i8*, align 4
  store i8* %87, i8** %88, align 4
  ; Variable output1 allocated at %88
  %89 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.26, i64 0, i64 0
  %90 = alloca i8*, align 4
  store i8* %89, i8** %90, align 4
  ; Variable output2 allocated at %90
  %91 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.27, i64 0, i64 0
  %92 = alloca i8*, align 4
  store i8* %91, i8** %92, align 4
  ; Variable output3 allocated at %92
  %93 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.28, i64 0, i64 0
  %94 = load i32, i32* %88, align 4
  %95 = call i32 @snapshot_test(i32 %93, i32 %94)
  %96 = alloca i1, align 4
  store i1 %95, i1* %96, align 4
  ; Variable result1 allocated at %96
  %97 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.29, i64 0, i64 0
  %98 = load i32, i32* %90, align 4
  %99 = call i32 @snapshot_test(i32 %97, i32 %98)
  %100 = alloca i1, align 4
  store i1 %99, i1* %100, align 4
  ; Variable result2 allocated at %100
  %101 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.30, i64 0, i64 0
  %102 = load i32, i32* %92, align 4
  %103 = call i32 @snapshot_test(i32 %101, i32 %102)
  %104 = alloca i1, align 4
  store i1 %103, i1* %104, align 4
  ; Variable result3 allocated at %104
  %105 = load i32, i32* %96, align 4
  %106 = call i32 @assert_true(i32 %105)
  %107 = load i32, i32* %100, align 4
  %108 = call i32 @assert_true(i32 %107)
  %109 = load i32, i32* %104, align 4
  %110 = call i32 @assert_true(i32 %109)
  %111 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.31, i64 0, i64 0
  %112 = call i32 @puts(i8* %111)
  %113 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.32, i64 0, i64 0
  %114 = call i32 @test_start(i32 %113)
  %115 = alloca i1, align 4
  store i1 %update_mode, i1* %115, align 4
  ; Variable original_update_mode allocated at %115
  %116 = call i32 @update_snapshots()
  %117 = alloca i1, align 4
  store i1 %116, i1* %117, align 4
  ; Variable update_result allocated at %117
  %118 = load i32, i32* %117, align 4
  %119 = call i32 @assert_true(i32 %118)
  %120 = call i32 @assert_true(i32 %update_mode)
  %121 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.15, i64 0, i64 0
  %122 = load i32, i32* %115, align 4
  %123 = getelementptr inbounds [42 x i8], [42 x i8]* @.str.33, i64 0, i64 0
  %124 = call i32 @puts(i8* %123)
  %125 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.34, i64 0, i64 0
  %126 = call i32 @test_start(i32 %125)
  %127 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.35, i64 0, i64 0
  %128 = alloca i8*, align 4
  store i8* %127, i8** %128, align 4
  ; Variable interface_content allocated at %128
  %129 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.36, i64 0, i64 0
  %130 = load i32, i32* %128, align 4
  %131 = call i32 @compare_with_snapshot(i32 %129, i32 %130)
  %132 = alloca i1, align 4
  store i1 %131, i1* %132, align 4
  ; Variable interface_result allocated at %132
  %133 = load i32, i32* %132, align 4
  %134 = call i32 @assert_true(i32 %133)
  %135 = getelementptr inbounds [48 x i8], [48 x i8]* @.str.37, i64 0, i64 0
  %136 = call i32 @puts(i8* %135)
  %137 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.38, i64 0, i64 0
  %138 = call i32 @test_start(i32 %137)
  %139 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.39, i64 0, i64 0
  %140 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.15, i64 0, i64 0
  %141 = call i32 @snapshot_test(i32 %139, i32 %140)
  %142 = alloca i1, align 4
  store i1 %141, i1* %142, align 4
  ; Variable empty_result allocated at %142
  %143 = load i32, i32* %142, align 4
  %144 = call i32 @assert_true(i32 %143)
  %145 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.40, i64 0, i64 0
  %146 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.41, i64 0, i64 0
  %147 = call i32 @snapshot_test(i32 %145, i32 %146)
  %148 = alloca i1, align 4
  store i1 %147, i1* %148, align 4
  ; Variable single_line_result allocated at %148
  %149 = load i32, i32* %148, align 4
  %150 = call i32 @assert_true(i32 %149)
  %151 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.42, i64 0, i64 0
  %152 = alloca i8*, align 4
  store i8* %151, i8** %152, align 4
  ; Variable special_chars allocated at %152
  %153 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.43, i64 0, i64 0
  %154 = load i32, i32* %152, align 4
  %155 = call i32 @snapshot_test(i32 %153, i32 %154)
  %156 = alloca i1, align 4
  store i1 %155, i1* %156, align 4
  ; Variable special_result allocated at %156
  %157 = load i32, i32* %156, align 4
  %158 = call i32 @assert_true(i32 %157)
  %159 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.44, i64 0, i64 0
  %160 = call i32 @puts(i8* %159)
  %161 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.45, i64 0, i64 0
  %162 = call i32 @test_start(i32 %161)
  %163 = call i32 @assert_true(i32 1)
  %164 = call i32 @assert_false(i32 0)
  %165 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.46, i64 0, i64 0
  %166 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.46, i64 0, i64 0
  %167 = call i32 @assert_eq_string(i32 %165, i32 %166)
  %168 = call i32 @assert_eq_int(i32 42, i32 42)
  %169 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.47, i64 0, i64 0
  %170 = alloca i8*, align 4
  store i8* %169, i8** %170, align 4
  ; Variable testz_output allocated at %170
  %171 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.48, i64 0, i64 0
  %172 = load i32, i32* %170, align 4
  %173 = call i32 @snapshot_test(i32 %171, i32 %172)
  %174 = alloca i1, align 4
  store i1 %173, i1* %174, align 4
  ; Variable testz_result allocated at %174
  %175 = load i32, i32* %174, align 4
  %176 = call i32 @assert_true(i32 %175)
  %177 = getelementptr inbounds [44 x i8], [44 x i8]* @.str.49, i64 0, i64 0
  %178 = call i32 @puts(i8* %177)
  %179 = call i32 @print_test_summary()
  %180 = getelementptr inbounds [46 x i8], [46 x i8]* @.str.50, i64 0, i64 0
  %181 = call i32 @puts(i8* %180)
  %182 = getelementptr inbounds [45 x i8], [45 x i8]* @.str.51, i64 0, i64 0
  %183 = call i32 @puts(i8* %182)
  %184 = getelementptr inbounds [50 x i8], [50 x i8]* @.str.52, i64 0, i64 0
  %185 = call i32 @puts(i8* %184)
  %186 = getelementptr inbounds [43 x i8], [43 x i8]* @.str.53, i64 0, i64 0
  %187 = call i32 @puts(i8* %186)
  %188 = getelementptr inbounds [43 x i8], [43 x i8]* @.str.54, i64 0, i64 0
  %189 = call i32 @puts(i8* %188)
  %190 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.55, i64 0, i64 0
  %191 = call i32 @puts(i8* %190)
  %192 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.56, i64 0, i64 0
  %193 = call i32 @puts(i8* %192)
  %194 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.57, i64 0, i64 0
  %195 = call i32 @puts(i8* %194)
  %196 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.58, i64 0, i64 0
  %197 = call i32 @puts(i8* %196)
  %198 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.59, i64 0, i64 0
  %199 = call i32 @puts(i8* %198)
  %200 = getelementptr inbounds [45 x i8], [45 x i8]* @.str.51, i64 0, i64 0
  %201 = call i32 @puts(i8* %200)
  %202 = getelementptr inbounds [53 x i8], [53 x i8]* @.str.60, i64 0, i64 0
  %203 = call i32 @puts(i8* %202)
  ret i32 0
}
