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
@.str.86 = private unnamed_addr constant [49 x i8] c"🎉 All WASM Mood tests completed successfully!\00", align 1
@.str.27 = private unnamed_addr constant [12 x i8] c"test_export\00", align 1
@.str.90 = private unnamed_addr constant [30 x i8] c"🏃 Total runtimes created: \00", align 1
@.str.43 = private unnamed_addr constant [42 x i8] c"✅ Invalid module validation test passed\00", align 1
@.str.28 = private unnamed_addr constant [27 x i8] c"✅ Add export test passed\00", align 1
@.str.8 = private unnamed_addr constant [35 x i8] c"✅ Source compilation test passed\00", align 1
@.str.52 = private unnamed_addr constant [22 x i8] c"wasm_memory_isolation\00", align 1
@.str.12 = private unnamed_addr constant [33 x i8] c"✅ Runtime creation test passed\00", align 1
@.str.39 = private unnamed_addr constant [19 x i8] c"wasm_wat_to_module\00", align 1
@.str.70 = private unnamed_addr constant [28 x i8] c"✅ Module info test passed\00", align 1
@.str.44 = private unnamed_addr constant [28 x i8] c"wasm_memory_bounds_checking\00", align 1
@.str.45 = private unnamed_addr constant [39 x i8] c"✅ Memory bounds checking test passed\00", align 1
@.str.14 = private unnamed_addr constant [31 x i8] c"✅ Module loading test passed\00", align 1
@.str.36 = private unnamed_addr constant [19 x i8] c"wasm_module_to_wat\00", align 1
@.str.58 = private unnamed_addr constant [19 x i8] c"slay cleanup() { }\00", align 1
@.str.15 = private unnamed_addr constant [19 x i8] c"wasm_call_function\00", align 1
@.str.5 = private unnamed_addr constant [25 x i8] c"wasm_compile_from_source\00", align 1
@.str.32 = private unnamed_addr constant [18 x i8] c"wasm_list_exports\00", align 1
@.str.65 = private unnamed_addr constant [25 x i8] c"wasm_end_to_end_workflow\00", align 1
@.str.13 = private unnamed_addr constant [17 x i8] c"wasm_load_module\00", align 1
@.str.41 = private unnamed_addr constant [41 x i8] c"✅ WAT to module conversion test passed\00", align 1
@.str.47 = private unnamed_addr constant [51 x i8] c"slay add(a normie, b normie) normie { damn a + b }\00", align 1
@.str.76 = private unnamed_addr constant [35 x i8] c"✅ Runtime statistics test passed\00", align 1
@.str.49 = private unnamed_addr constant [50 x i8] c"✅ Function execution with arguments test passed\00", align 1
@.str.57 = private unnamed_addr constant [22 x i8] c"wasm_resource_cleanup\00", align 1
@.str.40 = private unnamed_addr constant [55 x i8] c"(module (func $main nop) (export \"main\" (func $main)))\00", align 1
@.str.55 = private unnamed_addr constant [49 x i8] c"slay compute(x normie) normie { damn x * x + x }\00", align 1
@.str.21 = private unnamed_addr constant [29 x i8] c"✅ Memory write test passed\00", align 1
@.str.82 = private unnamed_addr constant [4 x i8] c"f64\00", align 1
@.str.59 = private unnamed_addr constant [33 x i8] c"✅ Resource cleanup test passed\00", align 1
@.str.61 = private unnamed_addr constant [41 x i8] c"✅ Binary format validation test passed\00", align 1
@.str.78 = private unnamed_addr constant [10 x i8] c"No errors\00", align 1
@.str.81 = private unnamed_addr constant [4 x i8] c"i32\00", align 1
@.str.63 = private unnamed_addr constant [24 x i8] c"slay main() { damn 42 }\00", align 1
@.str.3 = private unnamed_addr constant [21 x i8] c"wasm_validate_module\00", align 1
@.str.79 = private unnamed_addr constant [31 x i8] c"✅ Error handling test passed\00", align 1
@.str.26 = private unnamed_addr constant [16 x i8] c"wasm_add_export\00", align 1
@.str.16 = private unnamed_addr constant [5 x i8] c"main\00", align 1
@.str.31 = private unnamed_addr constant [27 x i8] c"✅ Add import test passed\00", align 1
@.str.53 = private unnamed_addr constant [33 x i8] c"✅ Memory isolation test passed\00", align 1
@.str.50 = private unnamed_addr constant [24 x i8] c"wasm_multiple_instances\00", align 1
@.str.6 = private unnamed_addr constant [42 x i8] c"slay main() { vibez.spill(\"Hello WASM\") }\00", align 1
@.str.67 = private unnamed_addr constant [10 x i8] c"calculate\00", align 1
@.str.2 = private unnamed_addr constant [38 x i8] c"✅ Empty module creation test passed\00", align 1
@.str.0 = private unnamed_addr constant [30 x i8] c"WASM Mood Comprehensive Tests\00", align 1
@.str.17 = private unnamed_addr constant [33 x i8] c"✅ Function calling test passed\00", align 1
@.str.42 = private unnamed_addr constant [31 x i8] c"wasm_invalid_module_validation\00", align 1
@.str.4 = private unnamed_addr constant [34 x i8] c"✅ Module validation test passed\00", align 1
@.str.9 = private unnamed_addr constant [21 x i8] c"wasm_optimize_module\00", align 1
@.str.83 = private unnamed_addr constant [31 x i8] c"✅ Value creation test passed\00", align 1
@.str.25 = private unnamed_addr constant [28 x i8] c"✅ Memory free test passed\00", align 1
@.str.89 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.64 = private unnamed_addr constant [40 x i8] c"✅ Compilation performance test passed\00", align 1
@.str.77 = private unnamed_addr constant [20 x i8] c"wasm_error_handling\00", align 1
@.str.72 = private unnamed_addr constant [29 x i8] c"✅ Runtime info test passed\00", align 1
@.str.11 = private unnamed_addr constant [20 x i8] c"wasm_create_runtime\00", align 1
@.str.88 = private unnamed_addr constant [30 x i8] c"📊 Total modules compiled: \00", align 1
@.str.29 = private unnamed_addr constant [16 x i8] c"wasm_add_import\00", align 1
@.str.68 = private unnamed_addr constant [36 x i8] c"✅ End-to-end workflow test passed\00", align 1
@.str.38 = private unnamed_addr constant [41 x i8] c"✅ Module to WAT conversion test passed\00", align 1
@.str.74 = private unnamed_addr constant [39 x i8] c"✅ Compilation statistics test passed\00", align 1
@.str.84 = private unnamed_addr constant [22 x i8] c"wasm_helper_functions\00", align 1
@.str.18 = private unnamed_addr constant [18 x i8] c"wasm_alloc_memory\00", align 1
@.str.85 = private unnamed_addr constant [33 x i8] c"✅ Helper functions test passed\00", align 1
@.str.7 = private unnamed_addr constant [5 x i8] c"wasm\00", align 1
@.str.35 = private unnamed_addr constant [29 x i8] c"✅ List imports test passed\00", align 1
@.str.51 = private unnamed_addr constant [35 x i8] c"✅ Multiple instances test passed\00", align 1
@.str.24 = private unnamed_addr constant [17 x i8] c"wasm_free_memory\00", align 1
@.str.69 = private unnamed_addr constant [17 x i8] c"wasm_module_info\00", align 1
@.str.33 = private unnamed_addr constant [29 x i8] c"✅ List exports test passed\00", align 1
@.str.54 = private unnamed_addr constant [25 x i8] c"wasm_optimization_levels\00", align 1
@.str.19 = private unnamed_addr constant [34 x i8] c"✅ Memory allocation test passed\00", align 1
@.str.60 = private unnamed_addr constant [30 x i8] c"wasm_binary_format_validation\00", align 1
@.str.62 = private unnamed_addr constant [29 x i8] c"wasm_compilation_performance\00", align 1
@.str.30 = private unnamed_addr constant [12 x i8] c"test_import\00", align 1
@.str.48 = private unnamed_addr constant [4 x i8] c"add\00", align 1
@.str.71 = private unnamed_addr constant [18 x i8] c"wasm_runtime_info\00", align 1
@.str.80 = private unnamed_addr constant [20 x i8] c"wasm_value_creation\00", align 1
@.str.66 = private unnamed_addr constant [61 x i8] c"slay calculate(a normie, b normie) normie { damn a + b * 2 }\00", align 1
@.str.46 = private unnamed_addr constant [34 x i8] c"wasm_function_execution_with_args\00", align 1
@.str.22 = private unnamed_addr constant [17 x i8] c"wasm_read_memory\00", align 1
@.str.20 = private unnamed_addr constant [18 x i8] c"wasm_write_memory\00", align 1
@.str.1 = private unnamed_addr constant [25 x i8] c"wasm_create_empty_module\00", align 1
@.str.56 = private unnamed_addr constant [36 x i8] c"✅ Optimization levels test passed\00", align 1
@.str.10 = private unnamed_addr constant [36 x i8] c"✅ Module optimization test passed\00", align 1
@.str.73 = private unnamed_addr constant [23 x i8] c"wasm_compilation_stats\00", align 1
@.str.75 = private unnamed_addr constant [19 x i8] c"wasm_runtime_stats\00", align 1
@.str.87 = private unnamed_addr constant [45 x i8] c"✅ WebAssembly support is production-ready!\00", align 1
@.str.37 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.23 = private unnamed_addr constant [28 x i8] c"✅ Memory read test passed\00", align 1
@.str.34 = private unnamed_addr constant [18 x i8] c"wasm_list_imports\00", align 1
define i32 @main() {
  %1 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.0, i64 0, i64 0
  %2 = call i32 @test_start(i32 %1)
  %3 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.1, i64 0, i64 0
  %4 = call i32 @test_start(i32 %3)
  %5 = call i32 @wasm_create_empty_module()
  %6 = alloca i32, align 4
  store i32 %5, i32* %6, align 4
  ; Variable empty_module allocated at %6
  %7 = load i32, i32* %6, align 4
  %8 = icmp sgt i32 %7, 0
  %9 = call i32 @assert_true(i32 %8)
  %10 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.2, i64 0, i64 0
  %11 = call i32 @puts(i8* %10)
  %12 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.3, i64 0, i64 0
  %13 = call i32 @test_start(i32 %12)
  %14 = call i32 @wasm_create_empty_module()
  %15 = alloca i32, align 4
  store i32 %14, i32* %15, align 4
  ; Variable test_module allocated at %15
  %16 = load i32, i32* %15, align 4
  %17 = call i32 @wasm_validate_module(i32 %16)
  %18 = alloca i1, align 4
  store i1 %17, i1* %18, align 4
  ; Variable is_valid allocated at %18
  %19 = load i32, i32* %18, align 4
  %20 = call i32 @assert_false(i32 %19)
  %21 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.4, i64 0, i64 0
  %22 = call i32 @puts(i8* %21)
  %23 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.5, i64 0, i64 0
  %24 = call i32 @test_start(i32 %23)
  %25 = getelementptr inbounds [42 x i8], [42 x i8]* @.str.6, i64 0, i64 0
  %26 = alloca i8*, align 4
  store i8* %25, i8** %26, align 4
  ; Variable simple_source allocated at %26
  %27 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.7, i64 0, i64 0
  %28 = call i32 @wasm_create_compile_options(i32 0, i32 %27, i32 1)
  %29 = alloca i32, align 4
  store i32 %28, i32* %29, align 4
  ; Variable compile_options allocated at %29
  %30 = load i32, i32* %26, align 4
  %31 = load i32, i32* %29, align 4
  %32 = call i32 @wasm_compile_from_source(i32 %30, i32 %31)
  %33 = alloca i32, align 4
  store i32 %32, i32* %33, align 4
  ; Variable compiled_module allocated at %33
  %34 = load i32, i32* %33, align 4
  %35 = icmp sgt i32 %34, 0
  %36 = call i32 @assert_true(i32 %35)
  %37 = load i32, i32* %33, align 4
  %38 = call i32 @wasm_validate_module(i32 %37)
  %39 = alloca i1, align 4
  store i1 %38, i1* %39, align 4
  ; Variable compiled_valid allocated at %39
  %40 = load i32, i32* %39, align 4
  %41 = call i32 @assert_true(i32 %40)
  %42 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.8, i64 0, i64 0
  %43 = call i32 @puts(i8* %42)
  %44 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.9, i64 0, i64 0
  %45 = call i32 @test_start(i32 %44)
  %46 = load i32, i32* %33, align 4
  %47 = call i32 @wasm_optimize_module(i32 %46, i32 2)
  %48 = alloca i32, align 4
  store i32 %47, i32* %48, align 4
  ; Variable optimized_module allocated at %48
  %49 = load i32, i32* %48, align 4
  %50 = icmp sgt i32 %49, 0
  %51 = call i32 @assert_true(i32 %50)
  %52 = load i32, i32* %48, align 4
  %53 = call i32 @wasm_validate_module(i32 %52)
  %54 = alloca i1, align 4
  store i1 %53, i1* %54, align 4
  ; Variable optimized_valid allocated at %54
  %55 = load i32, i32* %54, align 4
  %56 = call i32 @assert_true(i32 %55)
  %57 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.10, i64 0, i64 0
  %58 = call i32 @puts(i8* %57)
  %59 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.11, i64 0, i64 0
  %60 = call i32 @test_start(i32 %59)
  %61 = call i32 @wasm_create_config(i32 1048576, i32 10, i32 1)
  %62 = alloca i32, align 4
  store i32 %61, i32* %62, align 4
  ; Variable runtime_config allocated at %62
  %63 = load i32, i32* %62, align 4
  %64 = call i32 @wasm_create_runtime(i32 %63)
  %65 = alloca i32, align 4
  store i32 %64, i32* %65, align 4
  ; Variable runtime allocated at %65
  %66 = load i32, i32* %65, align 4
  %67 = icmp sgt i32 %66, 0
  %68 = call i32 @assert_true(i32 %67)
  %69 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.12, i64 0, i64 0
  %70 = call i32 @puts(i8* %69)
  %71 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.13, i64 0, i64 0
  %72 = call i32 @test_start(i32 %71)
  %73 = load i32, i32* %65, align 4
  %74 = load i32, i32* %33, align 4
  %75 = call i32 @wasm_load_module(i32 %73, i32 %74)
  %76 = alloca i32, align 4
  store i32 %75, i32* %76, align 4
  ; Variable instance allocated at %76
  %77 = load i32, i32* %76, align 4
  %78 = icmp sgt i32 %77, 0
  %79 = call i32 @assert_true(i32 %78)
  %80 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.14, i64 0, i64 0
  %81 = call i32 @puts(i8* %80)
  %82 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.15, i64 0, i64 0
  %83 = call i32 @test_start(i32 %82)
  %84 = alloca i32, align 4
  store i32 0, i32* %84, align 4
  ; Variable args allocated at %84
  %85 = load i32, i32* %76, align 4
  %86 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.16, i64 0, i64 0
  %87 = load i32, i32* %84, align 4
  %88 = call i32 @wasm_call_function(i32 %85, i32 %86, i32 %87)
  %89 = alloca i32, align 4
  store i32 %88, i32* %89, align 4
  ; Variable result allocated at %89
  %90 = load i32, i32* %89, align 4
  %91 = call i32 @assert_eq_int(i32 %90, i32 42)
  %92 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.17, i64 0, i64 0
  %93 = call i32 @puts(i8* %92)
  %94 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.18, i64 0, i64 0
  %95 = call i32 @test_start(i32 %94)
  %96 = call i32 @wasm_alloc_memory(i32 131072)
  %97 = alloca i32, align 4
  store i32 %96, i32* %97, align 4
  ; Variable memory allocated at %97
  %98 = load i32, i32* %97, align 4
  %99 = icmp sgt i32 %98, 0
  %100 = call i32 @assert_true(i32 %99)
  %101 = load i32, i32* %97, align 4
  %102 = call i32 @wasm_get_memory_size(i32 %101)
  %103 = alloca i32, align 4
  store i32 %102, i32* %103, align 4
  ; Variable memory_size allocated at %103
  %104 = load i32, i32* %103, align 4
  %105 = call i32 @assert_eq_int(i32 %104, i32 131072)
  %106 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.19, i64 0, i64 0
  %107 = call i32 @puts(i8* %106)
  %108 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.20, i64 0, i64 0
  %109 = call i32 @test_start(i32 %108)
  %110 = load i32, i32* %97, align 4
  %111 = call i32 @wasm_write_memory(i32 %110, i32 1000, i32 4)
  %112 = alloca i1, align 4
  store i1 %111, i1* %112, align 4
  ; Variable write_success allocated at %112
  %113 = load i32, i32* %112, align 4
  %114 = call i32 @assert_true(i32 %113)
  %115 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.21, i64 0, i64 0
  %116 = call i32 @puts(i8* %115)
  %117 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.22, i64 0, i64 0
  %118 = call i32 @test_start(i32 %117)
  %119 = load i32, i32* %97, align 4
  %120 = call i32 @wasm_read_memory(i32 %119, i32 1000, i32 4)
  %121 = alloca i32, align 4
  store i32 %120, i32* %121, align 4
  ; Variable read_result allocated at %121
  %122 = load i32, i32* %121, align 4
  %123 = call i32 @assert_eq_int(i32 %122, i32 4)
  %124 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.23, i64 0, i64 0
  %125 = call i32 @puts(i8* %124)
  %126 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.24, i64 0, i64 0
  %127 = call i32 @test_start(i32 %126)
  %128 = load i32, i32* %97, align 4
  %129 = call i32 @wasm_free_memory(i32 %128)
  %130 = alloca i1, align 4
  store i1 %129, i1* %130, align 4
  ; Variable free_success allocated at %130
  %131 = load i32, i32* %130, align 4
  %132 = call i32 @assert_true(i32 %131)
  %133 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.25, i64 0, i64 0
  %134 = call i32 @puts(i8* %133)
  %135 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.26, i64 0, i64 0
  %136 = call i32 @test_start(i32 %135)
  %137 = load i32, i32* %33, align 4
  %138 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.27, i64 0, i64 0
  %139 = call i32 @wasm_add_export(i32 %137, i32 %138, i32 1)
  %140 = alloca i1, align 4
  store i1 %139, i1* %140, align 4
  ; Variable export_success allocated at %140
  %141 = load i32, i32* %140, align 4
  %142 = call i32 @assert_true(i32 %141)
  %143 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.28, i64 0, i64 0
  %144 = call i32 @puts(i8* %143)
  %145 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.29, i64 0, i64 0
  %146 = call i32 @test_start(i32 %145)
  %147 = load i32, i32* %33, align 4
  %148 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.30, i64 0, i64 0
  %149 = call i32 @wasm_add_import(i32 %147, i32 %148, i32 2)
  %150 = alloca i1, align 4
  store i1 %149, i1* %150, align 4
  ; Variable import_success allocated at %150
  %151 = load i32, i32* %150, align 4
  %152 = call i32 @assert_true(i32 %151)
  %153 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.31, i64 0, i64 0
  %154 = call i32 @puts(i8* %153)
  %155 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.32, i64 0, i64 0
  %156 = call i32 @test_start(i32 %155)
  %157 = load i32, i32* %33, align 4
  %158 = call i32 @wasm_list_exports(i32 %157)
  %159 = alloca i32, align 4
  store i32 %158, i32* %159, align 4
  ; Variable exports_count allocated at %159
  %160 = load i32, i32* %159, align 4
  %161 = call i32 @assert_eq_int(i32 %160, i32 2)
  %162 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.33, i64 0, i64 0
  %163 = call i32 @puts(i8* %162)
  %164 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.34, i64 0, i64 0
  %165 = call i32 @test_start(i32 %164)
  %166 = load i32, i32* %33, align 4
  %167 = call i32 @wasm_list_imports(i32 %166)
  %168 = alloca i32, align 4
  store i32 %167, i32* %168, align 4
  ; Variable imports_count allocated at %168
  %169 = load i32, i32* %168, align 4
  %170 = call i32 @assert_eq_int(i32 %169, i32 1)
  %171 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.35, i64 0, i64 0
  %172 = call i32 @puts(i8* %171)
  %173 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.36, i64 0, i64 0
  %174 = call i32 @test_start(i32 %173)
  %175 = load i32, i32* %33, align 4
  %176 = call i32 @wasm_module_to_wat(i32 %175)
  %177 = alloca i8*, align 4
  store i8* %176, i8** %177, align 4
  ; Variable wat_output allocated at %177
  %178 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.37, i64 0, i64 0
  %179 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.37, i64 0, i64 0
  %180 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.38, i64 0, i64 0
  %181 = call i32 @puts(i8* %180)
  %182 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.39, i64 0, i64 0
  %183 = call i32 @test_start(i32 %182)
  %184 = getelementptr inbounds [55 x i8], [55 x i8]* @.str.40, i64 0, i64 0
  %185 = alloca i8*, align 4
  store i8* %184, i8** %185, align 4
  ; Variable simple_wat allocated at %185
  %186 = load i32, i32* %185, align 4
  %187 = call i32 @wasm_wat_to_module(i32 %186)
  %188 = alloca i32, align 4
  store i32 %187, i32* %188, align 4
  ; Variable wat_module allocated at %188
  %189 = load i32, i32* %188, align 4
  %190 = icmp sgt i32 %189, 0
  %191 = call i32 @assert_true(i32 %190)
  %192 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.41, i64 0, i64 0
  %193 = call i32 @puts(i8* %192)
  %194 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.42, i64 0, i64 0
  %195 = call i32 @test_start(i32 %194)
  %196 = alloca i32, align 4
  store i32 999, i32* %196, align 4
  ; Variable invalid_module allocated at %196
  %197 = load i32, i32* %196, align 4
  %198 = call i32 @wasm_validate_module(i32 %197)
  %199 = alloca i1, align 4
  store i1 %198, i1* %199, align 4
  ; Variable invalid_result allocated at %199
  %200 = load i32, i32* %199, align 4
  %201 = call i32 @assert_false(i32 %200)
  %202 = getelementptr inbounds [42 x i8], [42 x i8]* @.str.43, i64 0, i64 0
  %203 = call i32 @puts(i8* %202)
  %204 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.44, i64 0, i64 0
  %205 = call i32 @test_start(i32 %204)
  %206 = call i32 @wasm_alloc_memory(i32 1024)
  %207 = alloca i32, align 4
  store i32 %206, i32* %207, align 4
  ; Variable small_memory allocated at %207
  %208 = load i32, i32* %207, align 4
  %209 = call i32 @wasm_write_memory(i32 %208, i32 1022, i32 2)
  %210 = alloca i1, align 4
  store i1 %209, i1* %210, align 4
  ; Variable bounds_write allocated at %210
  %211 = load i32, i32* %210, align 4
  %212 = call i32 @assert_true(i32 %211)
  %213 = load i32, i32* %207, align 4
  %214 = call i32 @wasm_write_memory(i32 %213, i32 1023, i32 2)
  %215 = alloca i1, align 4
  store i1 %214, i1* %215, align 4
  ; Variable oob_write allocated at %215
  %216 = load i32, i32* %215, align 4
  %217 = call i32 @assert_false(i32 %216)
  %218 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.45, i64 0, i64 0
  %219 = call i32 @puts(i8* %218)
  %220 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.46, i64 0, i64 0
  %221 = call i32 @test_start(i32 %220)
  %222 = getelementptr inbounds [51 x i8], [51 x i8]* @.str.47, i64 0, i64 0
  %223 = load i32, i32* %29, align 4
  %224 = call i32 @wasm_compile_from_source(i32 %222, i32 %223)
  %225 = alloca i32, align 4
  store i32 %224, i32* %225, align 4
  ; Variable math_module allocated at %225
  %226 = load i32, i32* %65, align 4
  %227 = load i32, i32* %225, align 4
  %228 = call i32 @wasm_load_module(i32 %226, i32 %227)
  %229 = alloca i32, align 4
  store i32 %228, i32* %229, align 4
  ; Variable math_instance allocated at %229
  %230 = alloca i32, align 4
  store i32 2, i32* %230, align 4
  ; Variable math_args allocated at %230
  %231 = load i32, i32* %229, align 4
  %232 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.48, i64 0, i64 0
  %233 = load i32, i32* %230, align 4
  %234 = call i32 @wasm_call_function(i32 %231, i32 %232, i32 %233)
  %235 = alloca i32, align 4
  store i32 %234, i32* %235, align 4
  ; Variable math_result allocated at %235
  %236 = load i32, i32* %235, align 4
  %237 = call i32 @assert_eq_int(i32 %236, i32 42)
  %238 = getelementptr inbounds [50 x i8], [50 x i8]* @.str.49, i64 0, i64 0
  %239 = call i32 @puts(i8* %238)
  %240 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.50, i64 0, i64 0
  %241 = call i32 @test_start(i32 %240)
  %242 = load i32, i32* %65, align 4
  %243 = load i32, i32* %33, align 4
  %244 = call i32 @wasm_load_module(i32 %242, i32 %243)
  %245 = alloca i32, align 4
  store i32 %244, i32* %245, align 4
  ; Variable instance1 allocated at %245
  %246 = load i32, i32* %65, align 4
  %247 = load i32, i32* %33, align 4
  %248 = call i32 @wasm_load_module(i32 %246, i32 %247)
  %249 = alloca i32, align 4
  store i32 %248, i32* %249, align 4
  ; Variable instance2 allocated at %249
  %250 = load i32, i32* %245, align 4
  %251 = icmp sgt i32 %250, 0
  %252 = call i32 @assert_true(i32 %251)
  %253 = load i32, i32* %249, align 4
  %254 = icmp sgt i32 %253, 0
  %255 = call i32 @assert_true(i32 %254)
  %256 = load i32, i32* %245, align 4
  %257 = load i32, i32* %249, align 4
  %258 = call i32 @assert_ne_int(i32 %256, i32 %257)
  %259 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.51, i64 0, i64 0
  %260 = call i32 @puts(i8* %259)
  %261 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.52, i64 0, i64 0
  %262 = call i32 @test_start(i32 %261)
  %263 = call i32 @wasm_alloc_memory(i32 4096)
  %264 = alloca i32, align 4
  store i32 %263, i32* %264, align 4
  ; Variable mem1 allocated at %264
  %265 = call i32 @wasm_alloc_memory(i32 4096)
  %266 = alloca i32, align 4
  store i32 %265, i32* %266, align 4
  ; Variable mem2 allocated at %266
  %267 = load i32, i32* %264, align 4
  %268 = call i32 @wasm_write_memory(i32 %267, i32 0, i32 2)
  %269 = load i32, i32* %266, align 4
  %270 = call i32 @wasm_write_memory(i32 %269, i32 0, i32 2)
  %271 = load i32, i32* %264, align 4
  %272 = call i32 @wasm_read_memory(i32 %271, i32 0, i32 2)
  %273 = alloca i32, align 4
  store i32 %272, i32* %273, align 4
  ; Variable read1 allocated at %273
  %274 = load i32, i32* %266, align 4
  %275 = call i32 @wasm_read_memory(i32 %274, i32 0, i32 2)
  %276 = alloca i32, align 4
  store i32 %275, i32* %276, align 4
  ; Variable read2 allocated at %276
  %277 = load i32, i32* %273, align 4
  %278 = call i32 @assert_eq_int(i32 %277, i32 2)
  %279 = load i32, i32* %276, align 4
  %280 = call i32 @assert_eq_int(i32 %279, i32 2)
  %281 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.53, i64 0, i64 0
  %282 = call i32 @puts(i8* %281)
  %283 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.54, i64 0, i64 0
  %284 = call i32 @test_start(i32 %283)
  %285 = getelementptr inbounds [49 x i8], [49 x i8]* @.str.55, i64 0, i64 0
  %286 = alloca i8*, align 4
  store i8* %285, i8** %286, align 4
  ; Variable source_for_opt allocated at %286
  %287 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.7, i64 0, i64 0
  %288 = call i32 @wasm_create_compile_options(i32 0, i32 %287, i32 1)
  %289 = alloca i32, align 4
  store i32 %288, i32* %289, align 4
  ; Variable opt0_options allocated at %289
  %290 = load i32, i32* %286, align 4
  %291 = load i32, i32* %289, align 4
  %292 = call i32 @wasm_compile_from_source(i32 %290, i32 %291)
  %293 = alloca i32, align 4
  store i32 %292, i32* %293, align 4
  ; Variable opt0_module allocated at %293
  %294 = load i32, i32* %293, align 4
  %295 = call i32 @wasm_optimize_module(i32 %294, i32 3)
  %296 = alloca i32, align 4
  store i32 %295, i32* %296, align 4
  ; Variable opt3_module allocated at %296
  %297 = load i32, i32* %293, align 4
  %298 = call i32 @wasm_validate_module(i32 %297)
  %299 = call i32 @assert_true(i32 %298)
  %300 = load i32, i32* %296, align 4
  %301 = call i32 @wasm_validate_module(i32 %300)
  %302 = call i32 @assert_true(i32 %301)
  %303 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.56, i64 0, i64 0
  %304 = call i32 @puts(i8* %303)
  %305 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.57, i64 0, i64 0
  %306 = call i32 @test_start(i32 %305)
  %307 = call i32 @wasm_create_config(i32 65536, i32 2, i32 0)
  %308 = alloca i32, align 4
  store i32 %307, i32* %308, align 4
  ; Variable resource_config allocated at %308
  %309 = load i32, i32* %308, align 4
  %310 = call i32 @wasm_create_runtime(i32 %309)
  %311 = alloca i32, align 4
  store i32 %310, i32* %311, align 4
  ; Variable resource_runtime allocated at %311
  %312 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.58, i64 0, i64 0
  %313 = load i32, i32* %29, align 4
  %314 = call i32 @wasm_compile_from_source(i32 %312, i32 %313)
  %315 = alloca i32, align 4
  store i32 %314, i32* %315, align 4
  ; Variable cleanup_module allocated at %315
  %316 = load i32, i32* %311, align 4
  %317 = load i32, i32* %315, align 4
  %318 = call i32 @wasm_load_module(i32 %316, i32 %317)
  %319 = alloca i32, align 4
  store i32 %318, i32* %319, align 4
  ; Variable cleanup_instance allocated at %319
  %320 = load i32, i32* %319, align 4
  %321 = icmp sgt i32 %320, 0
  %322 = call i32 @assert_true(i32 %321)
  %323 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.59, i64 0, i64 0
  %324 = call i32 @puts(i8* %323)
  %325 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.60, i64 0, i64 0
  %326 = call i32 @test_start(i32 %325)
  %327 = call i32 @wasm_create_empty_module()
  %328 = alloca i32, align 4
  store i32 %327, i32* %328, align 4
  ; Variable format_module allocated at %328
  %329 = load i32, i32* %328, align 4
  %330 = call i32 @wasm_validate_bytecode(i32 %329)
  %331 = alloca i1, align 4
  store i1 %330, i1* %331, align 4
  ; Variable format_valid allocated at %331
  %332 = load i32, i32* %331, align 4
  %333 = call i32 @assert_true(i32 %332)
  %334 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.61, i64 0, i64 0
  %335 = call i32 @puts(i8* %334)
  %336 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.62, i64 0, i64 0
  %337 = call i32 @test_start(i32 %336)
  %338 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.63, i64 0, i64 0
  %339 = alloca i8*, align 4
  store i8* %338, i8** %339, align 4
  ; Variable large_source allocated at %339
  %340 = load i32, i32* %339, align 4
  %341 = call i32 @wasm_benchmark_compilation(i32 %340, i32 10)
  %342 = alloca i32, align 4
  store i32 %341, i32* %342, align 4
  ; Variable benchmark_time allocated at %342
  %343 = load i32, i32* %342, align 4
  %344 = icmp sgt i32 %343, 0
  %345 = call i32 @assert_true(i32 %344)
  %346 = load i32, i32* %342, align 4
  %347 = icmp slt i32 %346, 1000
  %348 = call i32 @assert_true(i32 %347)
  %349 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.64, i64 0, i64 0
  %350 = call i32 @puts(i8* %349)
  %351 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.65, i64 0, i64 0
  %352 = call i32 @test_start(i32 %351)
  %353 = getelementptr inbounds [61 x i8], [61 x i8]* @.str.66, i64 0, i64 0
  %354 = alloca i8*, align 4
  store i8* %353, i8** %354, align 4
  ; Variable workflow_source allocated at %354
  %355 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.7, i64 0, i64 0
  %356 = call i32 @wasm_create_compile_options(i32 2, i32 %355, i32 0)
  %357 = alloca i32, align 4
  store i32 %356, i32* %357, align 4
  ; Variable workflow_options allocated at %357
  %358 = load i32, i32* %354, align 4
  %359 = load i32, i32* %357, align 4
  %360 = call i32 @wasm_compile_from_source(i32 %358, i32 %359)
  %361 = alloca i32, align 4
  store i32 %360, i32* %361, align 4
  ; Variable workflow_module allocated at %361
  %362 = load i32, i32* %361, align 4
  %363 = call i32 @wasm_validate_module(i32 %362)
  %364 = call i32 @assert_true(i32 %363)
  %365 = load i32, i32* %62, align 4
  %366 = call i32 @wasm_create_runtime(i32 %365)
  %367 = alloca i32, align 4
  store i32 %366, i32* %367, align 4
  ; Variable workflow_runtime allocated at %367
  %368 = load i32, i32* %367, align 4
  %369 = load i32, i32* %361, align 4
  %370 = call i32 @wasm_load_module(i32 %368, i32 %369)
  %371 = alloca i32, align 4
  store i32 %370, i32* %371, align 4
  ; Variable workflow_instance allocated at %371
  %372 = load i32, i32* %371, align 4
  %373 = icmp sgt i32 %372, 0
  %374 = call i32 @assert_true(i32 %373)
  %375 = alloca i32, align 4
  store i32 2, i32* %375, align 4
  ; Variable calc_args allocated at %375
  %376 = load i32, i32* %371, align 4
  %377 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.67, i64 0, i64 0
  %378 = load i32, i32* %375, align 4
  %379 = call i32 @wasm_call_function(i32 %376, i32 %377, i32 %378)
  %380 = alloca i32, align 4
  store i32 %379, i32* %380, align 4
  ; Variable calc_result allocated at %380
  %381 = load i32, i32* %380, align 4
  %382 = call i32 @assert_eq_int(i32 %381, i32 42)
  %383 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.68, i64 0, i64 0
  %384 = call i32 @puts(i8* %383)
  %385 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.69, i64 0, i64 0
  %386 = call i32 @test_start(i32 %385)
  %387 = load i32, i32* %33, align 4
  %388 = call i32 @wasm_get_module_info(i32 %387)
  %389 = alloca i32, align 4
  store i32 %388, i32* %389, align 4
  ; Variable module_info allocated at %389
  %390 = load i32, i32* %389, align 4
  %391 = call i32 @assert_eq_int(i32 %390, i32 1)
  %392 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.70, i64 0, i64 0
  %393 = call i32 @puts(i8* %392)
  %394 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.71, i64 0, i64 0
  %395 = call i32 @test_start(i32 %394)
  %396 = load i32, i32* %65, align 4
  %397 = call i32 @wasm_get_runtime_info(i32 %396)
  %398 = alloca i32, align 4
  store i32 %397, i32* %398, align 4
  ; Variable runtime_info allocated at %398
  %399 = load i32, i32* %398, align 4
  %400 = call i32 @assert_eq_int(i32 %399, i32 1)
  %401 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.72, i64 0, i64 0
  %402 = call i32 @puts(i8* %401)
  %403 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.73, i64 0, i64 0
  %404 = call i32 @test_start(i32 %403)
  %405 = call i32 @wasm_get_compilation_stats()
  %406 = alloca i32, align 4
  store i32 %405, i32* %406, align 4
  ; Variable comp_stats allocated at %406
  %407 = load i32, i32* %406, align 4
  %408 = icmp sgt i32 %407, 0
  %409 = call i32 @assert_true(i32 %408)
  %410 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.74, i64 0, i64 0
  %411 = call i32 @puts(i8* %410)
  %412 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.75, i64 0, i64 0
  %413 = call i32 @test_start(i32 %412)
  %414 = call i32 @wasm_get_runtime_stats()
  %415 = alloca i32, align 4
  store i32 %414, i32* %415, align 4
  ; Variable runtime_stats allocated at %415
  %416 = load i32, i32* %415, align 4
  %417 = icmp sgt i32 %416, 0
  %418 = call i32 @assert_true(i32 %417)
  %419 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.76, i64 0, i64 0
  %420 = call i32 @puts(i8* %419)
  %421 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.77, i64 0, i64 0
  %422 = call i32 @test_start(i32 %421)
  %423 = call i32 @wasm_get_last_error()
  %424 = alloca i8*, align 4
  store i8* %423, i8** %424, align 4
  ; Variable error_msg allocated at %424
  %425 = load i32, i32* %424, align 4
  %426 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.78, i64 0, i64 0
  %427 = call i32 @assert_eq_string(i32 %425, i32 %426)
  %428 = call i32 @wasm_clear_errors()
  %429 = alloca i1, align 4
  store i1 %428, i1* %429, align 4
  ; Variable clear_result allocated at %429
  %430 = load i32, i32* %429, align 4
  %431 = call i32 @assert_true(i32 %430)
  %432 = call i32 @wasm_set_error_handler(i32 1)
  %433 = alloca i1, align 4
  store i1 %432, i1* %433, align 4
  ; Variable handler_result allocated at %433
  %434 = load i32, i32* %433, align 4
  %435 = call i32 @assert_true(i32 %434)
  %436 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.79, i64 0, i64 0
  %437 = call i32 @puts(i8* %436)
  %438 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.80, i64 0, i64 0
  %439 = call i32 @test_start(i32 %438)
  %440 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.81, i64 0, i64 0
  %441 = call i32 @wasm_create_value(i32 %440, i32 100)
  %442 = alloca i32, align 4
  store i32 %441, i32* %442, align 4
  ; Variable int_value allocated at %442
  %443 = load i32, i32* %442, align 4
  %444 = call i32 @assert_eq_int(i32 %443, i32 100)
  %445 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.82, i64 0, i64 0
  %446 = call i32 @wasm_create_value(i32 %445, i32 42)
  %447 = alloca i32, align 4
  store i32 %446, i32* %447, align 4
  ; Variable float_value allocated at %447
  %448 = load i32, i32* %447, align 4
  %449 = call i32 @assert_eq_int(i32 %448, i32 42)
  %450 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.83, i64 0, i64 0
  %451 = call i32 @puts(i8* %450)
  %452 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.84, i64 0, i64 0
  %453 = call i32 @test_start(i32 %452)
  %454 = call i32 @wasm_create_empty_instance()
  %455 = alloca i32, align 4
  store i32 %454, i32* %455, align 4
  ; Variable empty_instance allocated at %455
  %456 = load i32, i32* %455, align 4
  %457 = call i32 @assert_eq_int(i32 %456, i32 0)
  %458 = call i32 @wasm_create_empty_value()
  %459 = alloca i32, align 4
  store i32 %458, i32* %459, align 4
  ; Variable empty_value allocated at %459
  %460 = load i32, i32* %459, align 4
  %461 = call i32 @assert_eq_int(i32 %460, i32 0)
  %462 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.85, i64 0, i64 0
  %463 = call i32 @puts(i8* %462)
  %464 = call i32 @print_test_summary()
  %465 = getelementptr inbounds [49 x i8], [49 x i8]* @.str.86, i64 0, i64 0
  %466 = call i32 @puts(i8* %465)
  %467 = getelementptr inbounds [45 x i8], [45 x i8]* @.str.87, i64 0, i64 0
  %468 = call i32 @puts(i8* %467)
  %469 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.88, i64 0, i64 0
  %470 = call i32 @puts(i8* %469)
  %471 = call i32 @wasm_get_compilation_stats()
  ; Converting complex expression to output
  %472 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.89, i64 0, i64 0
  %473 = call i32 (i8*, ...) @printf(i8* %472, i32 %471)
  %474 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.90, i64 0, i64 0
  %475 = call i32 @puts(i8* %474)
  %476 = call i32 @wasm_get_runtime_stats()
  ; Converting complex expression to output
  %477 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.89, i64 0, i64 0
  %478 = call i32 (i8*, ...) @printf(i8* %477, i32 %476)
  ret i32 0
}
