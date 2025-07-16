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
@.str.10 = private unnamed_addr constant [63 x i8] c"{\"type\": \"object\", \"properties\": {\"name\": {\"type\": \"string\"}}}\00", align 1
@.str.72 = private unnamed_addr constant [39 x i8] c"Contract testing framework integration\00", align 1
@.str.89 = private unnamed_addr constant [19 x i8] c"{\"type\": \"object\"}\00", align 1
@.str.1 = private unnamed_addr constant [23 x i8] c"contract_test function\00", align 1
@.str.41 = private unnamed_addr constant [44 x i8] c"✅ Contract completeness validation passed\00", align 1
@.str.77 = private unnamed_addr constant [19 x i8] c"{\"type\": \"string\"}\00", align 1
@.str.14 = private unnamed_addr constant [14 x i8] c"/api/v1/users\00", align 1
@.str.67 = private unnamed_addr constant [2 x i8] c"1\00", align 1
@.str.43 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.27 = private unnamed_addr constant [36 x i8] c"✅ Contract regression test passed\00", align 1
@.str.91 = private unnamed_addr constant [23 x i8] c"{\"response_time\": 100}\00", align 1
@.str.17 = private unnamed_addr constant [35 x i8] c"✅ API contract validation passed\00", align 1
@.str.76 = private unnamed_addr constant [9 x i8] c"TestImpl\00", align 1
@.str.34 = private unnamed_addr constant [7 x i8] c"PASSED\00", align 1
@.str.8 = private unnamed_addr constant [43 x i8] c"✅ Interface contract verification passed\00", align 1
@.str.6 = private unnamed_addr constant [14 x i8] c"UserInterface\00", align 1
@.str.32 = private unnamed_addr constant [34 x i8] c"generate_contract_report function\00", align 1
@.str.48 = private unnamed_addr constant [17 x i8] c"{\"name\": \"Test\"}\00", align 1
@.str.71 = private unnamed_addr constant [42 x i8] c"✅ Performance metrics validation passed\00", align 1
@.str.70 = private unnamed_addr constant [31 x i8] c"performance metrics validation\00", align 1
@.str.40 = private unnamed_addr constant [49 x i8] c"[\"create_order\", \"update_order\", \"cancel_order\"]\00", align 1
@.str.18 = private unnamed_addr constant [32 x i8] c"consumer_contract_test function\00", align 1
@.str.92 = private unnamed_addr constant [64 x i8] c"✅ End-to-end contract testing workflow completed successfully\00", align 1
@.str.94 = private unnamed_addr constant [51 x i8] c"📊 Contract testing capabilities fully validated\00", align 1
@.str.75 = private unnamed_addr constant [14 x i8] c"TestInterface\00", align 1
@.str.20 = private unnamed_addr constant [8 x i8] c"UserAPI\00", align 1
@.str.65 = private unnamed_addr constant [30 x i8] c"API endpoint batch validation\00", align 1
@.str.84 = private unnamed_addr constant [37 x i8] c"End-to-end contract testing workflow\00", align 1
@.str.21 = private unnamed_addr constant [46 x i8] c"{\"consumer\": \"WebApp\", \"provider\": \"UserAPI\"}\00", align 1
@.str.25 = private unnamed_addr constant [39 x i8] c"{\"version\": \"1.0\", \"methods\": [\"pay\"]}\00", align 1
@.str.90 = private unnamed_addr constant [3 x i8] c"{}\00", align 1
@.str.31 = private unnamed_addr constant [37 x i8] c"✅ Performance contract test passed\00", align 1
@.str.0 = private unnamed_addr constant [47 x i8] c"Contract Testing Framework comprehensive tests\00", align 1
@.str.51 = private unnamed_addr constant [17 x i8] c"PaymentInterface\00", align 1
@.str.37 = private unnamed_addr constant [41 x i8] c"✅ Contract coverage calculation passed\00", align 1
@.str.64 = private unnamed_addr constant [37 x i8] c"✅ Complex schema validation passed\00", align 1
@.str.88 = private unnamed_addr constant [18 x i8] c"E2EImplementation\00", align 1
@.str.56 = private unnamed_addr constant [39 x i8] c"contract version compatibility testing\00", align 1
@.str.2 = private unnamed_addr constant [51 x i8] c"{\"version\": \"1.0\", \"endpoints\": [\"/api/v1/users\"]}\00", align 1
@.str.23 = private unnamed_addr constant [34 x i8] c"contract_regression_test function\00", align 1
@.str.26 = private unnamed_addr constant [49 x i8] c"{\"version\": \"1.1\", \"methods\": [\"pay\", \"refund\"]}\00", align 1
@.str.55 = private unnamed_addr constant [42 x i8] c"✅ Multiple interface contracts verified\00", align 1
@.str.78 = private unnamed_addr constant [7 x i8] c"\"test\"\00", align 1
@.str.83 = private unnamed_addr constant [36 x i8] c"✅ High contract coverage achieved\00", align 1
@.str.33 = private unnamed_addr constant [26 x i8] c"UserService Contract Test\00", align 1
@.str.45 = private unnamed_addr constant [41 x i8] c"✅ Empty contract specification handled\00", align 1
@.str.4 = private unnamed_addr constant [31 x i8] c"✅ Basic contract test passed\00", align 1
@.str.52 = private unnamed_addr constant [22 x i8] c"NotificationInterface\00", align 1
@.str.93 = private unnamed_addr constant [55 x i8] c"🎉 Contract Testing Framework - All tests completed!\00", align 1
@.str.35 = private unnamed_addr constant [38 x i8] c"✅ Contract report generation passed\00", align 1
@.str.24 = private unnamed_addr constant [15 x i8] c"PaymentService\00", align 1
@.str.44 = private unnamed_addr constant [13 x i8] c"EmptyService\00", align 1
@.str.95 = private unnamed_addr constant [56 x i8] c"🚀 Ready for production contract validation workflows\00", align 1
@.str.9 = private unnamed_addr constant [32 x i8] c"schema_validation_test function\00", align 1
@.str.19 = private unnamed_addr constant [7 x i8] c"WebApp\00", align 1
@.str.57 = private unnamed_addr constant [36 x i8] c"{\"version\": \"1.0\", \"api\": \"stable\"}\00", align 1
@.str.87 = private unnamed_addr constant [13 x i8] c"E2EInterface\00", align 1
@.str.62 = private unnamed_addr constant [136 x i8] c"{\"type\": \"object\", \"properties\": {\"user\": {\"type\": \"object\", \"properties\": {\"id\": {\"type\": \"number\"}, \"profile\": {\"type\": \"object\"}}}}}\00", align 1
@.str.12 = private unnamed_addr constant [34 x i8] c"✅ Schema validation test passed\00", align 1
@.str.86 = private unnamed_addr constant [47 x i8] c"{\"workflow\": \"complete\", \"validation\": \"full\"}\00", align 1
@.str.46 = private unnamed_addr constant [43 x i8] c"schema_validation_test with invalid schema\00", align 1
@.str.54 = private unnamed_addr constant [22 x i8] c"EmailNotificationImpl\00", align 1
@.str.3 = private unnamed_addr constant [12 x i8] c"UserService\00", align 1
@.str.38 = private unnamed_addr constant [40 x i8] c"validate_contract_completeness function\00", align 1
@.str.80 = private unnamed_addr constant [42 x i8] c"Contract coverage comprehensive reporting\00", align 1
@.str.61 = private unnamed_addr constant [26 x i8] c"complex schema validation\00", align 1
@.str.79 = private unnamed_addr constant [52 x i8] c"✅ Contract testing framework integration complete\00", align 1
@.str.81 = private unnamed_addr constant [34 x i8] c"Contract Testing Coverage Report:\00", align 1
@.str.15 = private unnamed_addr constant [47 x i8] c"{\"type\": \"array\", \"items\": {\"type\": \"object\"}}\00", align 1
@.str.47 = private unnamed_addr constant [20 x i8] c"invalid_json_schema\00", align 1
@.str.39 = private unnamed_addr constant [13 x i8] c"OrderService\00", align 1
@.str.30 = private unnamed_addr constant [51 x i8] c"{\"max_response_time\": 100, \"min_throughput\": 1000}\00", align 1
@.str.74 = private unnamed_addr constant [15 x i8] c"{\"test\": true}\00", align 1
@.str.58 = private unnamed_addr constant [36 x i8] c"{\"version\": \"2.0\", \"api\": \"stable\"}\00", align 1
@.str.60 = private unnamed_addr constant [44 x i8] c"✅ Contract version compatibility verified\00", align 1
@.str.5 = private unnamed_addr constant [35 x i8] c"verify_interface_contract function\00", align 1
@.str.16 = private unnamed_addr constant [28 x i8] c"[{\"id\": 1, \"name\": \"John\"}]\00", align 1
@.str.22 = private unnamed_addr constant [41 x i8] c"✅ Consumer-driven contract test passed\00", align 1
@.str.29 = private unnamed_addr constant [14 x i8] c"SearchService\00", align 1
@.str.42 = private unnamed_addr constant [39 x i8] c"contract_test with empty specification\00", align 1
@.str.28 = private unnamed_addr constant [35 x i8] c"performance_contract_test function\00", align 1
@.str.7 = private unnamed_addr constant [16 x i8] c"UserServiceImpl\00", align 1
@.str.50 = private unnamed_addr constant [41 x i8] c"multiple interface contract verification\00", align 1
@.str.11 = private unnamed_addr constant [21 x i8] c"{\"name\": \"John Doe\"}\00", align 1
@.str.59 = private unnamed_addr constant [15 x i8] c"VersionService\00", align 1
@.str.85 = private unnamed_addr constant [15 x i8] c"E2ETestService\00", align 1
@.str.69 = private unnamed_addr constant [41 x i8] c"✅ API endpoint batch validation passed\00", align 1
@.str.13 = private unnamed_addr constant [31 x i8] c"validate_api_contract function\00", align 1
@.str.36 = private unnamed_addr constant [37 x i8] c"calculate_contract_coverage function\00", align 1
@.str.63 = private unnamed_addr constant [80 x i8] c"{\"user\": {\"id\": 123, \"profile\": {\"name\": \"John\", \"email\": \"john@example.com\"}}}\00", align 1
@.str.66 = private unnamed_addr constant [2 x i8] c"0\00", align 1
@.str.49 = private unnamed_addr constant [38 x i8] c"✅ Invalid schema handled gracefully\00", align 1
@.str.68 = private unnamed_addr constant [2 x i8] c"2\00", align 1
@.str.82 = private unnamed_addr constant [2 x i8] c"%\00", align 1
@.str.73 = private unnamed_addr constant [12 x i8] c"TestService\00", align 1
@.str.53 = private unnamed_addr constant [19 x i8] c"PaymentServiceImpl\00", align 1
define i32 @main() {
  %1 = getelementptr inbounds [47 x i8], [47 x i8]* @.str.0, i64 0, i64 0
  %2 = call i32 @test_start(i32 %1)
  %3 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.1, i64 0, i64 0
  %4 = call i32 @test_start(i32 %3)
  %5 = getelementptr inbounds [51 x i8], [51 x i8]* @.str.2, i64 0, i64 0
  %6 = alloca i8*, align 4
  store i8* %5, i8** %6, align 4
  ; Variable contract_spec allocated at %6
  %7 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.3, i64 0, i64 0
  %8 = load i32, i32* %6, align 4
  %9 = call i32 @contract_test(i32 %7, i32 %8)
  %10 = alloca i1, align 4
  store i1 %9, i1* %10, align 4
  ; Variable result allocated at %10
  %11 = load i32, i32* %10, align 4
  %12 = call i32 @assert_true(i32 %11)
  %13 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.4, i64 0, i64 0
  %14 = call i32 @puts(i8* %13)
  %15 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.5, i64 0, i64 0
  %16 = call i32 @test_start(i32 %15)
  %17 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.6, i64 0, i64 0
  %18 = alloca i8*, align 4
  store i8* %17, i8** %18, align 4
  ; Variable interface_spec allocated at %18
  %19 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.7, i64 0, i64 0
  %20 = alloca i8*, align 4
  store i8* %19, i8** %20, align 4
  ; Variable implementation_spec allocated at %20
  %21 = load i32, i32* %18, align 4
  %22 = load i32, i32* %20, align 4
  %23 = call i32 @verify_interface_contract(i32 %21, i32 %22)
  %24 = alloca i1, align 4
  store i1 %23, i1* %24, align 4
  ; Variable interface_result allocated at %24
  %25 = load i32, i32* %24, align 4
  %26 = call i32 @assert_true(i32 %25)
  %27 = getelementptr inbounds [43 x i8], [43 x i8]* @.str.8, i64 0, i64 0
  %28 = call i32 @puts(i8* %27)
  %29 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.9, i64 0, i64 0
  %30 = call i32 @test_start(i32 %29)
  %31 = getelementptr inbounds [63 x i8], [63 x i8]* @.str.10, i64 0, i64 0
  %32 = alloca i8*, align 4
  store i8* %31, i8** %32, align 4
  ; Variable schema allocated at %32
  %33 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.11, i64 0, i64 0
  %34 = alloca i8*, align 4
  store i8* %33, i8** %34, align 4
  ; Variable test_data allocated at %34
  %35 = load i32, i32* %32, align 4
  %36 = load i32, i32* %34, align 4
  %37 = call i32 @schema_validation_test(i32 %35, i32 %36)
  %38 = alloca i1, align 4
  store i1 %37, i1* %38, align 4
  ; Variable schema_result allocated at %38
  %39 = load i32, i32* %38, align 4
  %40 = call i32 @assert_true(i32 %39)
  %41 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.12, i64 0, i64 0
  %42 = call i32 @puts(i8* %41)
  %43 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.13, i64 0, i64 0
  %44 = call i32 @test_start(i32 %43)
  %45 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.14, i64 0, i64 0
  %46 = alloca i8*, align 4
  store i8* %45, i8** %46, align 4
  ; Variable endpoint allocated at %46
  %47 = getelementptr inbounds [47 x i8], [47 x i8]* @.str.15, i64 0, i64 0
  %48 = alloca i8*, align 4
  store i8* %47, i8** %48, align 4
  ; Variable expected_schema allocated at %48
  %49 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.16, i64 0, i64 0
  %50 = alloca i8*, align 4
  store i8* %49, i8** %50, align 4
  ; Variable api_response allocated at %50
  %51 = load i32, i32* %46, align 4
  %52 = load i32, i32* %48, align 4
  %53 = load i32, i32* %50, align 4
  %54 = call i32 @validate_api_contract(i32 %51, i32 %52, i32 %53)
  %55 = alloca i1, align 4
  store i1 %54, i1* %55, align 4
  ; Variable api_result allocated at %55
  %56 = load i32, i32* %55, align 4
  %57 = call i32 @assert_true(i32 %56)
  %58 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.17, i64 0, i64 0
  %59 = call i32 @puts(i8* %58)
  %60 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.18, i64 0, i64 0
  %61 = call i32 @test_start(i32 %60)
  %62 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.19, i64 0, i64 0
  %63 = alloca i8*, align 4
  store i8* %62, i8** %63, align 4
  ; Variable consumer allocated at %63
  %64 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.20, i64 0, i64 0
  %65 = alloca i8*, align 4
  store i8* %64, i8** %65, align 4
  ; Variable provider allocated at %65
  %66 = getelementptr inbounds [46 x i8], [46 x i8]* @.str.21, i64 0, i64 0
  %67 = alloca i8*, align 4
  store i8* %66, i8** %67, align 4
  ; Variable consumer_contract allocated at %67
  %68 = load i32, i32* %63, align 4
  %69 = load i32, i32* %65, align 4
  %70 = load i32, i32* %67, align 4
  %71 = call i32 @consumer_contract_test(i32 %68, i32 %69, i32 %70)
  %72 = alloca i1, align 4
  store i1 %71, i1* %72, align 4
  ; Variable consumer_result allocated at %72
  %73 = load i32, i32* %72, align 4
  %74 = call i32 @assert_true(i32 %73)
  %75 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.22, i64 0, i64 0
  %76 = call i32 @puts(i8* %75)
  %77 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.23, i64 0, i64 0
  %78 = call i32 @test_start(i32 %77)
  %79 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.24, i64 0, i64 0
  %80 = alloca i8*, align 4
  store i8* %79, i8** %80, align 4
  ; Variable service allocated at %80
  %81 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.25, i64 0, i64 0
  %82 = alloca i8*, align 4
  store i8* %81, i8** %82, align 4
  ; Variable old_contract allocated at %82
  %83 = getelementptr inbounds [49 x i8], [49 x i8]* @.str.26, i64 0, i64 0
  %84 = alloca i8*, align 4
  store i8* %83, i8** %84, align 4
  ; Variable new_contract allocated at %84
  %85 = load i32, i32* %80, align 4
  %86 = load i32, i32* %82, align 4
  %87 = load i32, i32* %84, align 4
  %88 = call i32 @contract_regression_test(i32 %85, i32 %86, i32 %87)
  %89 = alloca i1, align 4
  store i1 %88, i1* %89, align 4
  ; Variable regression_result allocated at %89
  %90 = load i32, i32* %89, align 4
  %91 = call i32 @assert_true(i32 %90)
  %92 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.27, i64 0, i64 0
  %93 = call i32 @puts(i8* %92)
  %94 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.28, i64 0, i64 0
  %95 = call i32 @test_start(i32 %94)
  %96 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.29, i64 0, i64 0
  %97 = alloca i8*, align 4
  store i8* %96, i8** %97, align 4
  ; Variable perf_service allocated at %97
  %98 = getelementptr inbounds [51 x i8], [51 x i8]* @.str.30, i64 0, i64 0
  %99 = alloca i8*, align 4
  store i8* %98, i8** %99, align 4
  ; Variable perf_spec allocated at %99
  %100 = load i32, i32* %97, align 4
  %101 = load i32, i32* %99, align 4
  %102 = call i32 @performance_contract_test(i32 %100, i32 %101)
  %103 = alloca i1, align 4
  store i1 %102, i1* %103, align 4
  ; Variable perf_result allocated at %103
  %104 = load i32, i32* %103, align 4
  %105 = call i32 @assert_true(i32 %104)
  %106 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.31, i64 0, i64 0
  %107 = call i32 @puts(i8* %106)
  %108 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.32, i64 0, i64 0
  %109 = call i32 @test_start(i32 %108)
  %110 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.33, i64 0, i64 0
  %111 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.34, i64 0, i64 0
  %112 = call i32 @generate_contract_report(i32 %110, i32 %111)
  %113 = alloca i8*, align 4
  store i8* %112, i8** %113, align 4
  ; Variable report allocated at %113
  %114 = load i32, i32* %113, align 4
  ; Member access: %114.length
  %115 = getelementptr inbounds %struct.object, %struct.object* %114, i32 0, i32 0
  %116 = load i32, i32* %115, align 4
  %117 = icmp sgt i32 %116, 0
  %118 = call i32 @assert_true(i32 %117)
  %119 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.35, i64 0, i64 0
  %120 = call i32 @puts(i8* %119)
  %121 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.36, i64 0, i64 0
  %122 = call i32 @test_start(i32 %121)
  %123 = call i32 @calculate_contract_coverage(i32 10, i32 8)
  %124 = alloca double, align 4
  store double %123, double* %124, align 4
  ; Variable coverage allocated at %124
  %125 = load i32, i32* %124, align 4
  %126 = icmp eq i32 %125, 80
  %127 = call i32 @assert_true(i32 %126)
  %128 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.37, i64 0, i64 0
  %129 = call i32 @puts(i8* %128)
  %130 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.38, i64 0, i64 0
  %131 = call i32 @test_start(i32 %130)
  %132 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.39, i64 0, i64 0
  %133 = alloca i8*, align 4
  store i8* %132, i8** %133, align 4
  ; Variable completeness_service allocated at %133
  %134 = getelementptr inbounds [49 x i8], [49 x i8]* @.str.40, i64 0, i64 0
  %135 = alloca i8*, align 4
  store i8* %134, i8** %135, align 4
  ; Variable contract_list allocated at %135
  %136 = load i32, i32* %133, align 4
  %137 = load i32, i32* %135, align 4
  %138 = call i32 @validate_contract_completeness(i32 %136, i32 %137)
  %139 = alloca i1, align 4
  store i1 %138, i1* %139, align 4
  ; Variable completeness_result allocated at %139
  %140 = load i32, i32* %139, align 4
  %141 = call i32 @assert_true(i32 %140)
  %142 = getelementptr inbounds [44 x i8], [44 x i8]* @.str.41, i64 0, i64 0
  %143 = call i32 @puts(i8* %142)
  %144 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.42, i64 0, i64 0
  %145 = call i32 @test_start(i32 %144)
  %146 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %147 = alloca i8*, align 4
  store i8* %146, i8** %147, align 4
  ; Variable empty_contract allocated at %147
  %148 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.44, i64 0, i64 0
  %149 = load i32, i32* %147, align 4
  %150 = call i32 @contract_test(i32 %148, i32 %149)
  %151 = alloca i1, align 4
  store i1 %150, i1* %151, align 4
  ; Variable empty_result allocated at %151
  %152 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.45, i64 0, i64 0
  %153 = call i32 @puts(i8* %152)
  %154 = getelementptr inbounds [43 x i8], [43 x i8]* @.str.46, i64 0, i64 0
  %155 = call i32 @test_start(i32 %154)
  %156 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.47, i64 0, i64 0
  %157 = alloca i8*, align 4
  store i8* %156, i8** %157, align 4
  ; Variable invalid_schema allocated at %157
  %158 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.48, i64 0, i64 0
  %159 = alloca i8*, align 4
  store i8* %158, i8** %159, align 4
  ; Variable valid_data allocated at %159
  %160 = load i32, i32* %157, align 4
  %161 = load i32, i32* %159, align 4
  %162 = call i32 @schema_validation_test(i32 %160, i32 %161)
  %163 = alloca i1, align 4
  store i1 %162, i1* %163, align 4
  ; Variable invalid_schema_result allocated at %163
  %164 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.49, i64 0, i64 0
  %165 = call i32 @puts(i8* %164)
  %166 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.50, i64 0, i64 0
  %167 = call i32 @test_start(i32 %166)
  %168 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.51, i64 0, i64 0
  %169 = alloca i8*, align 4
  store i8* %168, i8** %169, align 4
  ; Variable interface1 allocated at %169
  %170 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.52, i64 0, i64 0
  %171 = alloca i8*, align 4
  store i8* %170, i8** %171, align 4
  ; Variable interface2 allocated at %171
  %172 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.53, i64 0, i64 0
  %173 = alloca i8*, align 4
  store i8* %172, i8** %173, align 4
  ; Variable impl1 allocated at %173
  %174 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.54, i64 0, i64 0
  %175 = alloca i8*, align 4
  store i8* %174, i8** %175, align 4
  ; Variable impl2 allocated at %175
  %176 = load i32, i32* %169, align 4
  %177 = load i32, i32* %173, align 4
  %178 = call i32 @verify_interface_contract(i32 %176, i32 %177)
  %179 = alloca i1, align 4
  store i1 %178, i1* %179, align 4
  ; Variable multi_result1 allocated at %179
  %180 = load i32, i32* %171, align 4
  %181 = load i32, i32* %175, align 4
  %182 = call i32 @verify_interface_contract(i32 %180, i32 %181)
  %183 = alloca i1, align 4
  store i1 %182, i1* %183, align 4
  ; Variable multi_result2 allocated at %183
  %184 = load i32, i32* %183, align 4
  %185 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %186 = getelementptr inbounds [42 x i8], [42 x i8]* @.str.55, i64 0, i64 0
  %187 = call i32 @puts(i8* %186)
  %188 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.56, i64 0, i64 0
  %189 = call i32 @test_start(i32 %188)
  %190 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.57, i64 0, i64 0
  %191 = alloca i8*, align 4
  store i8* %190, i8** %191, align 4
  ; Variable old_version allocated at %191
  %192 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.58, i64 0, i64 0
  %193 = alloca i8*, align 4
  store i8* %192, i8** %193, align 4
  ; Variable new_version allocated at %193
  %194 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.59, i64 0, i64 0
  %195 = load i32, i32* %191, align 4
  %196 = load i32, i32* %193, align 4
  %197 = call i32 @contract_regression_test(i32 %194, i32 %195, i32 %196)
  %198 = alloca i1, align 4
  store i1 %197, i1* %198, align 4
  ; Variable version_compatibility allocated at %198
  %199 = load i1, i1* %198, align 4
  %200 = call i32 @assert_true(i32 %199)
  %201 = getelementptr inbounds [44 x i8], [44 x i8]* @.str.60, i64 0, i64 0
  %202 = call i32 @puts(i8* %201)
  %203 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.61, i64 0, i64 0
  %204 = call i32 @test_start(i32 %203)
  %205 = getelementptr inbounds [136 x i8], [136 x i8]* @.str.62, i64 0, i64 0
  %206 = alloca i8*, align 4
  store i8* %205, i8** %206, align 4
  ; Variable complex_schema allocated at %206
  %207 = getelementptr inbounds [80 x i8], [80 x i8]* @.str.63, i64 0, i64 0
  %208 = alloca i8*, align 4
  store i8* %207, i8** %208, align 4
  ; Variable complex_data allocated at %208
  %209 = load i32, i32* %206, align 4
  %210 = load i32, i32* %208, align 4
  %211 = call i32 @schema_validation_test(i32 %209, i32 %210)
  %212 = alloca i1, align 4
  store i1 %211, i1* %212, align 4
  ; Variable complex_result allocated at %212
  %213 = load i32, i32* %212, align 4
  %214 = call i32 @assert_true(i32 %213)
  %215 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.64, i64 0, i64 0
  %216 = call i32 @puts(i8* %215)
  %217 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.65, i64 0, i64 0
  %218 = call i32 @test_start(i32 %217)
  %219 = alloca i32, align 4
  store i32 0, i32* %219, align 4
  ; Variable endpoints allocated at %219
  %220 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %221 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %222 = inttoptr i64 0 to [0 x i32]*
  %223 = alloca i32, align 4
  store i32 0, i32* %223, align 4
  ; Variable schemas allocated at %223
  %224 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %225 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %226 = inttoptr i64 0 to [0 x i32]*
  %227 = alloca i32, align 4
  store i32 0, i32* %227, align 4
  ; Variable responses allocated at %227
  %228 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %229 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %230 = inttoptr i64 0 to [0 x i32]*
  %231 = alloca i32, align 4
  store i32 0, i32* %231, align 4
  ; Variable batch_results allocated at %231
  %232 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %233 = alloca i32, align 4
  store i32 0, i32* %233, align 4
  ; Short declaration: i := 0 (i32)
  br label %label0
label0:
  %234 = load i32, i32* %233, align 4
  %235 = icmp slt i32 %234, 3
  br i1 %235, label %label1, label %label3
label1:
  %236 = load i32, i32* %231, align 4
  %237 = inttoptr i64 0 to [0 x i32]*
  %238 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %239 = load i32, i32* %233, align 4
  %240 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %241 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %242 = load i32, i32* %223, align 4
  %243 = inttoptr i64 0 to [0 x i32]*
  %244 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %245 = load i32, i32* %227, align 4
  %246 = inttoptr i64 0 to [0 x i32]*
  %247 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  br label %label2
label2:
  %248 = load i32, i32* %233, align 4
  %249 = add i32 %248, 1
  store i32 %249, i32* %233, align 4
  br label %label0
label3:
  %250 = load i32, i32* %231, align 4
  %251 = alloca i1, align 4
  store i1 %250, i1* %251, align 4
  ; Variable all_passed allocated at %251
  %252 = alloca [1 x i32], align 4
  %253 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.66, i64 0, i64 0
  %254 = getelementptr inbounds [1 x i32], [1 x i32]* %252, i64 0, i64 0
  store i32 %253, i32* %254, align 4
  %255 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %256 = load i32, i32* %231, align 4
  %257 = alloca [1 x i32], align 4
  %258 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.67, i64 0, i64 0
  %259 = getelementptr inbounds [1 x i32], [1 x i32]* %257, i64 0, i64 0
  store i32 %258, i32* %259, align 4
  %260 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %261 = load i32, i32* %231, align 4
  %262 = alloca [1 x i32], align 4
  %263 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.68, i64 0, i64 0
  %264 = getelementptr inbounds [1 x i32], [1 x i32]* %262, i64 0, i64 0
  store i32 %263, i32* %264, align 4
  %265 = load i32, i32* %251, align 4
  %266 = call i32 @assert_true(i32 %265)
  %267 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.69, i64 0, i64 0
  %268 = call i32 @puts(i8* %267)
  %269 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.70, i64 0, i64 0
  %270 = call i32 @test_start(i32 %269)
  %271 = alloca i32, align 4
  store i32 0, i32* %271, align 4
  ; Variable performance_services allocated at %271
  %272 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %273 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %274 = inttoptr i64 0 to [0 x i32]*
  %275 = alloca i32, align 4
  store i32 0, i32* %275, align 4
  ; Variable performance_specs allocated at %275
  %276 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %277 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %278 = inttoptr i64 0 to [0 x i32]*
  %279 = alloca i32, align 4
  store i32 0, i32* %279, align 4
  ; Variable perf_results allocated at %279
  %280 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %281 = alloca i32, align 4
  store i32 0, i32* %281, align 4
  ; Short declaration: j := 0 (i32)
  br label %label4
label4:
  %282 = load i32, i32* %281, align 4
  %283 = icmp slt i32 %282, 2
  br i1 %283, label %label5, label %label7
label5:
  %284 = load i32, i32* %279, align 4
  %285 = inttoptr i64 0 to [0 x i32]*
  %286 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %287 = load i32, i32* %281, align 4
  %288 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %289 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %290 = load i32, i32* %275, align 4
  %291 = inttoptr i64 0 to [0 x i32]*
  %292 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  br label %label6
label6:
  %293 = load i32, i32* %281, align 4
  %294 = add i32 %293, 1
  store i32 %294, i32* %281, align 4
  br label %label4
label7:
  %295 = load i32, i32* %279, align 4
  %296 = alloca i1, align 4
  store i1 %295, i1* %296, align 4
  ; Variable all_perf_passed allocated at %296
  %297 = alloca [1 x i32], align 4
  %298 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.66, i64 0, i64 0
  %299 = getelementptr inbounds [1 x i32], [1 x i32]* %297, i64 0, i64 0
  store i32 %298, i32* %299, align 4
  %300 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %301 = load i32, i32* %279, align 4
  %302 = alloca [1 x i32], align 4
  %303 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.67, i64 0, i64 0
  %304 = getelementptr inbounds [1 x i32], [1 x i32]* %302, i64 0, i64 0
  store i32 %303, i32* %304, align 4
  %305 = load i32, i32* %296, align 4
  %306 = call i32 @assert_true(i32 %305)
  %307 = getelementptr inbounds [42 x i8], [42 x i8]* @.str.71, i64 0, i64 0
  %308 = call i32 @puts(i8* %307)
  %309 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.72, i64 0, i64 0
  %310 = call i32 @test_start(i32 %309)
  %311 = alloca i1, align 4
  store i1 1, i1* %311, align 4
  ; Variable integration_test_passed allocated at %311
  %312 = load i32, i32* %311, align 4
  %313 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %314 = load i32, i32* %311, align 4
  %315 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %316 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.73, i64 0, i64 0
  %317 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.74, i64 0, i64 0
  %318 = call i32 @contract_test(i32 %316, i32 %317)
  %319 = load i32, i32* %311, align 4
  %320 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %321 = load i32, i32* %311, align 4
  %322 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %323 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.75, i64 0, i64 0
  %324 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.76, i64 0, i64 0
  %325 = call i32 @verify_interface_contract(i32 %323, i32 %324)
  %326 = load i32, i32* %311, align 4
  %327 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %328 = load i32, i32* %311, align 4
  %329 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %330 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.77, i64 0, i64 0
  %331 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.78, i64 0, i64 0
  %332 = call i32 @schema_validation_test(i32 %330, i32 %331)
  %333 = load i32, i32* %311, align 4
  %334 = call i32 @assert_true(i32 %333)
  %335 = getelementptr inbounds [52 x i8], [52 x i8]* @.str.79, i64 0, i64 0
  %336 = call i32 @puts(i8* %335)
  %337 = getelementptr inbounds [42 x i8], [42 x i8]* @.str.80, i64 0, i64 0
  %338 = call i32 @test_start(i32 %337)
  %339 = alloca i32, align 4
  store i32 25, i32* %339, align 4
  ; Variable total_contracts allocated at %339
  %340 = alloca i32, align 4
  store i32 23, i32* %340, align 4
  ; Variable tested_contracts allocated at %340
  %341 = load i32, i32* %339, align 4
  %342 = load i32, i32* %340, align 4
  %343 = call i32 @calculate_contract_coverage(i32 %341, i32 %342)
  %344 = alloca double, align 4
  store double %343, double* %344, align 4
  ; Variable final_coverage allocated at %344
  %345 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.81, i64 0, i64 0
  %346 = call i32 @puts(i8* %345)
  %347 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %348 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %349 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %350 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.82, i64 0, i64 0
  %351 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %352 = load i32, i32* %344, align 4
  %353 = icmp sgt i32 %352, 90
  %354 = call i32 @assert_true(i32 %353)
  %355 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.83, i64 0, i64 0
  %356 = call i32 @puts(i8* %355)
  %357 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.84, i64 0, i64 0
  %358 = call i32 @test_start(i32 %357)
  %359 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.85, i64 0, i64 0
  %360 = alloca i8*, align 4
  store i8* %359, i8** %360, align 4
  ; Variable workflow_service allocated at %360
  %361 = getelementptr inbounds [47 x i8], [47 x i8]* @.str.86, i64 0, i64 0
  %362 = alloca i8*, align 4
  store i8* %361, i8** %362, align 4
  ; Variable workflow_contract allocated at %362
  %363 = load i32, i32* %360, align 4
  %364 = load i32, i32* %362, align 4
  %365 = call i32 @contract_test(i32 %363, i32 %364)
  %366 = alloca i1, align 4
  store i1 %365, i1* %366, align 4
  ; Variable step1 allocated at %366
  %367 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.87, i64 0, i64 0
  %368 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.88, i64 0, i64 0
  %369 = call i32 @verify_interface_contract(i32 %367, i32 %368)
  %370 = alloca i1, align 4
  store i1 %369, i1* %370, align 4
  ; Variable step2 allocated at %370
  %371 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.89, i64 0, i64 0
  %372 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.90, i64 0, i64 0
  %373 = call i32 @schema_validation_test(i32 %371, i32 %372)
  %374 = alloca i1, align 4
  store i1 %373, i1* %374, align 4
  ; Variable step3 allocated at %374
  %375 = load i32, i32* %360, align 4
  %376 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.91, i64 0, i64 0
  %377 = call i32 @performance_contract_test(i32 %375, i32 %376)
  %378 = alloca i1, align 4
  store i1 %377, i1* %378, align 4
  ; Variable step4 allocated at %378
  %379 = load i32, i32* %366, align 4
  %380 = alloca i1, align 4
  store i1 %379, i1* %380, align 4
  ; Variable workflow_complete allocated at %380
  %381 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %382 = load i32, i32* %370, align 4
  %383 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %384 = load i32, i32* %374, align 4
  %385 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.43, i64 0, i64 0
  %386 = load i32, i32* %378, align 4
  %387 = load i32, i32* %380, align 4
  %388 = call i32 @assert_true(i32 %387)
  %389 = getelementptr inbounds [64 x i8], [64 x i8]* @.str.92, i64 0, i64 0
  %390 = call i32 @puts(i8* %389)
  %391 = call i32 @print_test_summary()
  %392 = getelementptr inbounds [55 x i8], [55 x i8]* @.str.93, i64 0, i64 0
  %393 = call i32 @puts(i8* %392)
  %394 = getelementptr inbounds [51 x i8], [51 x i8]* @.str.94, i64 0, i64 0
  %395 = call i32 @puts(i8* %394)
  %396 = getelementptr inbounds [56 x i8], [56 x i8]* @.str.95, i64 0, i64 0
  %397 = call i32 @puts(i8* %396)
  ret i32 0
}
