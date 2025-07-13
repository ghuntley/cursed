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
declare void @cursed_channel_send(i8*, i8*)
declare i8* @cursed_channel_receive(i8*)
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



; String constants
@.str.23 = private unnamed_addr constant [39 x i8] c"❌ FAIL: Integer to string conversion\00", align 1
@.str.38 = private unnamed_addr constant [37 x i8] c"✅ PASS: Integer implements Numeric\00", align 1
@.str.7 = private unnamed_addr constant [28 x i8] c"❌ FAIL: Integer type name\00", align 1
@.str.12 = private unnamed_addr constant [28 x i8] c"❌ FAIL: Integer type size\00", align 1
@.str.63 = private unnamed_addr constant [24 x i8] c"Test 9: Method metadata\00", align 1
@.str.25 = private unnamed_addr constant [40 x i8] c"❌ FAIL: Integer to boolean conversion\00", align 1
@.str.37 = private unnamed_addr constant [38 x i8] c"❌ FAIL: Integer implements Stringer\00", align 1
@.str.49 = private unnamed_addr constant [29 x i8] c"❌ FAIL: Zero boolean value\00", align 1
@.str.28 = private unnamed_addr constant [24 x i8] c"Test 4: Method checking\00", align 1
@.str.29 = private unnamed_addr constant [7 x i8] c"string\00", align 1
@.str.35 = private unnamed_addr constant [33 x i8] c"Test 5: Interface implementation\00", align 1
@.str.46 = private unnamed_addr constant [29 x i8] c"✅ PASS: Zero integer value\00", align 1
@.str.48 = private unnamed_addr constant [29 x i8] c"✅ PASS: Zero boolean value\00", align 1
@.str.11 = private unnamed_addr constant [33 x i8] c"✅ PASS: Integer type size is 4\00", align 1
@.str.73 = private unnamed_addr constant [48 x i8] c"✅ PASS: Reflection demo executed successfully\00", align 1
@.str.74 = private unnamed_addr constant [26 x i8] c"❌ FAIL: Reflection demo\00", align 1
@.str.52 = private unnamed_addr constant [28 x i8] c"✅ PASS: Zero string value\00", align 1
@.str.57 = private unnamed_addr constant [32 x i8] c"✅ PASS: String to int parsing\00", align 1
@.str.64 = private unnamed_addr constant [4 x i8] c"int\00", align 1
@.str.19 = private unnamed_addr constant [28 x i8] c"❌ FAIL: Boolean type kind\00", align 1
@.str.43 = private unnamed_addr constant [45 x i8] c"✅ PASS: Deep equality for different values\00", align 1
@.str.54 = private unnamed_addr constant [23 x i8] c"Test 8: String parsing\00", align 1
@.str.2 = private unnamed_addr constant [38 x i8] c"=====================================\00", align 1
@.str.76 = private unnamed_addr constant [37 x i8] c"All core reflection features tested!\00", align 1
@.str.67 = private unnamed_addr constant [13 x i8] c"string() tea\00", align 1
@.str.21 = private unnamed_addr constant [3 x i8] c"42\00", align 1
@.str.9 = private unnamed_addr constant [41 x i8] c"✅ PASS: Integer type kind is 'integer'\00", align 1
@.str.26 = private unnamed_addr constant [38 x i8] c"✅ PASS: Integer to float conversion\00", align 1
@.str.6 = private unnamed_addr constant [40 x i8] c"✅ PASS: Integer type name is 'normie'\00", align 1
@.str.16 = private unnamed_addr constant [28 x i8] c"❌ FAIL: Boolean type name\00", align 1
@.str.18 = private unnamed_addr constant [41 x i8] c"✅ PASS: Boolean type kind is 'boolean'\00", align 1
@.str.22 = private unnamed_addr constant [39 x i8] c"✅ PASS: Integer to string conversion\00", align 1
@.str.0 = private unnamed_addr constant [36 x i8] c"CURSED Reflection System Test Suite\00", align 1
@.str.10 = private unnamed_addr constant [28 x i8] c"❌ FAIL: Integer type kind\00", align 1
@.str.5 = private unnamed_addr constant [7 x i8] c"normie\00", align 1
@.str.58 = private unnamed_addr constant [32 x i8] c"❌ FAIL: String to int parsing\00", align 1
@.str.62 = private unnamed_addr constant [33 x i8] c"❌ FAIL: String to bool parsing\00", align 1
@.str.47 = private unnamed_addr constant [29 x i8] c"❌ FAIL: Zero integer value\00", align 1
@.str.66 = private unnamed_addr constant [23 x i8] c"❌ FAIL: Method count\00", align 1
@.str.65 = private unnamed_addr constant [28 x i8] c"✅ PASS: Method count is 4\00", align 1
@.str.32 = private unnamed_addr constant [36 x i8] c"❌ FAIL: Integer has string method\00", align 1
@.str.61 = private unnamed_addr constant [33 x i8] c"✅ PASS: String to bool parsing\00", align 1
@.str.75 = private unnamed_addr constant [39 x i8] c"CURSED Reflection System Test Complete\00", align 1
@.str.50 = private unnamed_addr constant [27 x i8] c"✅ PASS: Zero float value\00", align 1
@.str.30 = private unnamed_addr constant [8 x i8] c"unknown\00", align 1
@.str.15 = private unnamed_addr constant [37 x i8] c"✅ PASS: Boolean type name is 'lit'\00", align 1
@.str.77 = private unnamed_addr constant [36 x i8] c"Module is ready for production use.\00", align 1
@.str.69 = private unnamed_addr constant [27 x i8] c"❌ FAIL: Method signature\00", align 1
@.str.42 = private unnamed_addr constant [40 x i8] c"❌ FAIL: Deep equality for same values\00", align 1
@.str.27 = private unnamed_addr constant [38 x i8] c"❌ FAIL: Integer to float conversion\00", align 1
@.str.44 = private unnamed_addr constant [45 x i8] c"❌ FAIL: Deep equality for different values\00", align 1
@.str.34 = private unnamed_addr constant [47 x i8] c"❌ FAIL: Integer does not have unknown method\00", align 1
@.str.33 = private unnamed_addr constant [47 x i8] c"✅ PASS: Integer does not have unknown method\00", align 1
@.str.53 = private unnamed_addr constant [28 x i8] c"❌ FAIL: Zero string value\00", align 1
@.str.56 = private unnamed_addr constant [5 x i8] c"true\00", align 1
@.str.4 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.59 = private unnamed_addr constant [34 x i8] c"✅ PASS: String to float parsing\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.70 = private unnamed_addr constant [29 x i8] c"✅ PASS: Method return type\00", align 1
@.str.17 = private unnamed_addr constant [8 x i8] c"boolean\00", align 1
@.str.55 = private unnamed_addr constant [5 x i8] c"3.14\00", align 1
@.str.41 = private unnamed_addr constant [40 x i8] c"✅ PASS: Deep equality for same values\00", align 1
@.str.36 = private unnamed_addr constant [38 x i8] c"✅ PASS: Integer implements Stringer\00", align 1
@.str.14 = private unnamed_addr constant [4 x i8] c"lit\00", align 1
@.str.31 = private unnamed_addr constant [36 x i8] c"✅ PASS: Integer has string method\00", align 1
@.str.72 = private unnamed_addr constant [25 x i8] c"Test 10: Reflection demo\00", align 1
@.str.40 = private unnamed_addr constant [22 x i8] c"Test 6: Deep equality\00", align 1
@.str.39 = private unnamed_addr constant [37 x i8] c"❌ FAIL: Integer implements Numeric\00", align 1
@.str.8 = private unnamed_addr constant [8 x i8] c"integer\00", align 1
@.str.3 = private unnamed_addr constant [32 x i8] c"Test 1: Integer type reflection\00", align 1
@.str.24 = private unnamed_addr constant [40 x i8] c"✅ PASS: Integer to boolean conversion\00", align 1
@.str.60 = private unnamed_addr constant [34 x i8] c"❌ FAIL: String to float parsing\00", align 1
@.str.68 = private unnamed_addr constant [27 x i8] c"✅ PASS: Method signature\00", align 1
@.str.20 = private unnamed_addr constant [25 x i8] c"Test 3: Type conversions\00", align 1
@.str.51 = private unnamed_addr constant [27 x i8] c"❌ FAIL: Zero float value\00", align 1
@.str.45 = private unnamed_addr constant [20 x i8] c"Test 7: Zero values\00", align 1
@.str.13 = private unnamed_addr constant [32 x i8] c"Test 2: Boolean type reflection\00", align 1
@.str.71 = private unnamed_addr constant [29 x i8] c"❌ FAIL: Method return type\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.0, i64 0, i64 0
  ; Converting complex expression to output
  %1 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %2 = call i32 (i8*, ...) @printf(i8* %1, i32 %0)
  %3 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.2, i64 0, i64 0
  ; Converting complex expression to output
  %4 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %5 = call i32 (i8*, ...) @printf(i8* %4, i32 %3)
  %6 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.3, i64 0, i64 0
  ; Converting complex expression to output
  %7 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %8 = call i32 (i8*, ...) @printf(i8* %7, i32 %6)
  %9 = alloca i32, align 4
  store i32 42, i32* %9, align 4
  ; Variable int_val allocated at %9
  %10 = load i32, i32* %9, align 4
  %11 = call i32 @get_type_name_int(i32 %10)
  %12 = alloca i8*, align 4
  store i8* %11, i8** %12, align 4
  ; Variable type_name allocated at %12
  %13 = load i32, i32* %9, align 4
  %14 = call i32 @get_type_kind_int(i32 %13)
  %15 = alloca i8*, align 4
  store i8* %14, i8** %15, align 4
  ; Variable type_kind allocated at %15
  %16 = load i32, i32* %9, align 4
  %17 = call i32 @get_type_size_int(i32 %16)
  %18 = alloca i32, align 4
  store i32 %17, i32* %18, align 4
  ; Variable type_size allocated at %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %20 = load i32, i32* %12, align 4
  %21 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.5, i64 0, i64 0
  %22 = icmp eq i32 %20, %21
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %24 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.6, i64 0, i64 0
  ; Converting complex expression to output
  %25 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %26 = call i32 (i8*, ...) @printf(i8* %25, i32 %24)
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %30 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.7, i64 0, i64 0
  ; Converting complex expression to output
  %31 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %32 = call i32 (i8*, ...) @printf(i8* %31, i32 %30)
  %33 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %35 = load i32, i32* %15, align 4
  %36 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.8, i64 0, i64 0
  %37 = icmp eq i32 %35, %36
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %39 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.9, i64 0, i64 0
  ; Converting complex expression to output
  %40 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %41 = call i32 (i8*, ...) @printf(i8* %40, i32 %39)
  %42 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %43 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %44 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %45 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.10, i64 0, i64 0
  ; Converting complex expression to output
  %46 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %47 = call i32 (i8*, ...) @printf(i8* %46, i32 %45)
  %48 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %49 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %50 = load i32, i32* %18, align 4
  %51 = icmp eq i32 %50, 4
  %52 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %53 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.11, i64 0, i64 0
  ; Converting complex expression to output
  %54 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %55 = call i32 (i8*, ...) @printf(i8* %54, i32 %53)
  %56 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %57 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %58 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %59 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.12, i64 0, i64 0
  ; Converting complex expression to output
  %60 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %61 = call i32 (i8*, ...) @printf(i8* %60, i32 %59)
  %62 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %63 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.13, i64 0, i64 0
  ; Converting complex expression to output
  %64 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %65 = call i32 (i8*, ...) @printf(i8* %64, i32 %63)
  %66 = alloca i1, align 4
  store i1 1, i1* %66, align 4
  ; Variable bool_val allocated at %66
  %67 = load i32, i32* %66, align 4
  %68 = call i32 @get_type_name_bool(i32 %67)
  %69 = alloca i8*, align 4
  store i8* %68, i8** %69, align 4
  ; Variable bool_type_name allocated at %69
  %70 = load i32, i32* %66, align 4
  %71 = call i32 @get_type_kind_bool(i32 %70)
  %72 = alloca i8*, align 4
  store i8* %71, i8** %72, align 4
  ; Variable bool_type_kind allocated at %72
  %73 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %74 = load i32, i32* %69, align 4
  %75 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.14, i64 0, i64 0
  %76 = icmp eq i32 %74, %75
  %77 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %78 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.15, i64 0, i64 0
  ; Converting complex expression to output
  %79 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %80 = call i32 (i8*, ...) @printf(i8* %79, i32 %78)
  %81 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %82 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %83 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %84 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.16, i64 0, i64 0
  ; Converting complex expression to output
  %85 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %86 = call i32 (i8*, ...) @printf(i8* %85, i32 %84)
  %87 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %88 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %89 = load i32, i32* %72, align 4
  %90 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.17, i64 0, i64 0
  %91 = icmp eq i32 %89, %90
  %92 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %93 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.18, i64 0, i64 0
  ; Converting complex expression to output
  %94 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %95 = call i32 (i8*, ...) @printf(i8* %94, i32 %93)
  %96 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %97 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %98 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %99 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.19, i64 0, i64 0
  ; Converting complex expression to output
  %100 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %101 = call i32 (i8*, ...) @printf(i8* %100, i32 %99)
  %102 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %103 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.20, i64 0, i64 0
  ; Converting complex expression to output
  %104 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %105 = call i32 (i8*, ...) @printf(i8* %104, i32 %103)
  %106 = load i32, i32* %9, align 4
  %107 = call i32 @convert_int_to_string(i32 %106)
  %108 = alloca i8*, align 4
  store i8* %107, i8** %108, align 4
  ; Variable str_result allocated at %108
  %109 = load i32, i32* %9, align 4
  %110 = call i32 @convert_int_to_bool(i32 %109)
  %111 = alloca i1, align 4
  store i1 %110, i1* %111, align 4
  ; Variable bool_result allocated at %111
  %112 = load i32, i32* %9, align 4
  %113 = call i32 @convert_int_to_float(i32 %112)
  %114 = alloca double, align 4
  store double %113, double* %114, align 4
  ; Variable float_result allocated at %114
  %115 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %116 = load i32, i32* %108, align 4
  %117 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.21, i64 0, i64 0
  %118 = icmp eq i32 %116, %117
  %119 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %120 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.22, i64 0, i64 0
  ; Converting complex expression to output
  %121 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %122 = call i32 (i8*, ...) @printf(i8* %121, i32 %120)
  %123 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %124 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %125 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %126 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.23, i64 0, i64 0
  ; Converting complex expression to output
  %127 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %128 = call i32 (i8*, ...) @printf(i8* %127, i32 %126)
  %129 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %130 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %131 = load i32, i32* %111, align 4
  %132 = icmp eq i32 %131, 1
  %133 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %134 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.24, i64 0, i64 0
  ; Converting complex expression to output
  %135 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %136 = call i32 (i8*, ...) @printf(i8* %135, i32 %134)
  %137 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %138 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %139 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %140 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.25, i64 0, i64 0
  ; Converting complex expression to output
  %141 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %142 = call i32 (i8*, ...) @printf(i8* %141, i32 %140)
  %143 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %144 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %145 = load i32, i32* %114, align 4
  %146 = icmp eq i32 %145, 42
  %147 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %148 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.26, i64 0, i64 0
  ; Converting complex expression to output
  %149 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %150 = call i32 (i8*, ...) @printf(i8* %149, i32 %148)
  %151 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %152 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %153 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %154 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.27, i64 0, i64 0
  ; Converting complex expression to output
  %155 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %156 = call i32 (i8*, ...) @printf(i8* %155, i32 %154)
  %157 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %158 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.28, i64 0, i64 0
  ; Converting complex expression to output
  %159 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %160 = call i32 (i8*, ...) @printf(i8* %159, i32 %158)
  %161 = load i32, i32* %9, align 4
  %162 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.29, i64 0, i64 0
  %163 = call i32 @has_method_int(i32 %161, i32 %162)
  %164 = alloca i1, align 4
  store i1 %163, i1* %164, align 4
  ; Variable has_string_method allocated at %164
  %165 = load i32, i32* %9, align 4
  %166 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.30, i64 0, i64 0
  %167 = call i32 @has_method_int(i32 %165, i32 %166)
  %168 = alloca i1, align 4
  store i1 %167, i1* %168, align 4
  ; Variable has_unknown_method allocated at %168
  %169 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %170 = load i32, i32* %164, align 4
  %171 = icmp eq i32 %170, 1
  %172 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %173 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.31, i64 0, i64 0
  ; Converting complex expression to output
  %174 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %175 = call i32 (i8*, ...) @printf(i8* %174, i32 %173)
  %176 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %177 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %178 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %179 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.32, i64 0, i64 0
  ; Converting complex expression to output
  %180 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %181 = call i32 (i8*, ...) @printf(i8* %180, i32 %179)
  %182 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %183 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %184 = load i32, i32* %168, align 4
  %185 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %186 = icmp eq i32 %184, %185
  %187 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %188 = getelementptr inbounds [47 x i8], [47 x i8]* @.str.33, i64 0, i64 0
  ; Converting complex expression to output
  %189 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %190 = call i32 (i8*, ...) @printf(i8* %189, i32 %188)
  %191 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %192 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %193 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %194 = getelementptr inbounds [47 x i8], [47 x i8]* @.str.34, i64 0, i64 0
  ; Converting complex expression to output
  %195 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %196 = call i32 (i8*, ...) @printf(i8* %195, i32 %194)
  %197 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %198 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.35, i64 0, i64 0
  ; Converting complex expression to output
  %199 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %200 = call i32 (i8*, ...) @printf(i8* %199, i32 %198)
  %201 = load i32, i32* %9, align 4
  %202 = call i32 @implements_stringer_int(i32 %201)
  %203 = alloca i1, align 4
  store i1 %202, i1* %203, align 4
  ; Variable implements_stringer allocated at %203
  %204 = load i32, i32* %9, align 4
  %205 = call i32 @implements_numeric_int(i32 %204)
  %206 = alloca i1, align 4
  store i1 %205, i1* %206, align 4
  ; Variable implements_numeric allocated at %206
  %207 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %208 = load i32, i32* %203, align 4
  %209 = icmp eq i32 %208, 1
  %210 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %211 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.36, i64 0, i64 0
  ; Converting complex expression to output
  %212 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %213 = call i32 (i8*, ...) @printf(i8* %212, i32 %211)
  %214 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %215 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %216 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %217 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.37, i64 0, i64 0
  ; Converting complex expression to output
  %218 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %219 = call i32 (i8*, ...) @printf(i8* %218, i32 %217)
  %220 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %221 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %222 = load i32, i32* %206, align 4
  %223 = icmp eq i32 %222, 1
  %224 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %225 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.38, i64 0, i64 0
  ; Converting complex expression to output
  %226 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %227 = call i32 (i8*, ...) @printf(i8* %226, i32 %225)
  %228 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %229 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %230 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %231 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.39, i64 0, i64 0
  ; Converting complex expression to output
  %232 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %233 = call i32 (i8*, ...) @printf(i8* %232, i32 %231)
  %234 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %235 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.40, i64 0, i64 0
  ; Converting complex expression to output
  %236 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %237 = call i32 (i8*, ...) @printf(i8* %236, i32 %235)
  %238 = alloca i32, align 4
  store i32 42, i32* %238, align 4
  ; Variable other_int allocated at %238
  %239 = alloca i32, align 4
  store i32 24, i32* %239, align 4
  ; Variable different_int allocated at %239
  %240 = load i32, i32* %9, align 4
  %241 = load i32, i32* %238, align 4
  %242 = call i32 @deep_equal_int(i32 %240, i32 %241)
  %243 = alloca i1, align 4
  store i1 %242, i1* %243, align 4
  ; Variable equal allocated at %243
  %244 = load i32, i32* %9, align 4
  %245 = load i32, i32* %239, align 4
  %246 = call i32 @deep_equal_int(i32 %244, i32 %245)
  %247 = alloca i1, align 4
  store i1 %246, i1* %247, align 4
  ; Variable not_equal allocated at %247
  %248 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %249 = load i32, i32* %243, align 4
  %250 = icmp eq i32 %249, 1
  %251 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %252 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.41, i64 0, i64 0
  ; Converting complex expression to output
  %253 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %254 = call i32 (i8*, ...) @printf(i8* %253, i32 %252)
  %255 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %256 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %257 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %258 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.42, i64 0, i64 0
  ; Converting complex expression to output
  %259 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %260 = call i32 (i8*, ...) @printf(i8* %259, i32 %258)
  %261 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %262 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %263 = load i32, i32* %247, align 4
  %264 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %265 = icmp eq i32 %263, %264
  %266 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %267 = getelementptr inbounds [45 x i8], [45 x i8]* @.str.43, i64 0, i64 0
  ; Converting complex expression to output
  %268 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %269 = call i32 (i8*, ...) @printf(i8* %268, i32 %267)
  %270 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %271 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %272 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %273 = getelementptr inbounds [45 x i8], [45 x i8]* @.str.44, i64 0, i64 0
  ; Converting complex expression to output
  %274 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %275 = call i32 (i8*, ...) @printf(i8* %274, i32 %273)
  %276 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %277 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.45, i64 0, i64 0
  ; Converting complex expression to output
  %278 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %279 = call i32 (i8*, ...) @printf(i8* %278, i32 %277)
  %280 = call i32 @get_zero_int()
  %281 = alloca i32, align 4
  store i32 %280, i32* %281, align 4
  ; Variable zero_int allocated at %281
  %282 = call i32 @get_zero_bool()
  %283 = alloca i1, align 4
  store i1 %282, i1* %283, align 4
  ; Variable zero_bool allocated at %283
  %284 = call i32 @get_zero_float()
  %285 = alloca double, align 4
  store double %284, double* %285, align 4
  ; Variable zero_float allocated at %285
  %286 = call i32 @get_zero_string()
  %287 = alloca i8*, align 4
  store i8* %286, i8** %287, align 4
  ; Variable zero_str allocated at %287
  %288 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %289 = load i32, i32* %281, align 4
  %290 = icmp eq i32 %289, 0
  %291 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %292 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.46, i64 0, i64 0
  ; Converting complex expression to output
  %293 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %294 = call i32 (i8*, ...) @printf(i8* %293, i32 %292)
  %295 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %296 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %297 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %298 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.47, i64 0, i64 0
  ; Converting complex expression to output
  %299 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %300 = call i32 (i8*, ...) @printf(i8* %299, i32 %298)
  %301 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %302 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %303 = load i32, i32* %283, align 4
  %304 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %305 = icmp eq i32 %303, %304
  %306 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %307 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.48, i64 0, i64 0
  ; Converting complex expression to output
  %308 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %309 = call i32 (i8*, ...) @printf(i8* %308, i32 %307)
  %310 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %311 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %312 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %313 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.49, i64 0, i64 0
  ; Converting complex expression to output
  %314 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %315 = call i32 (i8*, ...) @printf(i8* %314, i32 %313)
  %316 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %317 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %318 = load i32, i32* %285, align 4
  %319 = icmp eq i32 %318, 0
  %320 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %321 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.50, i64 0, i64 0
  ; Converting complex expression to output
  %322 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %323 = call i32 (i8*, ...) @printf(i8* %322, i32 %321)
  %324 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %325 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %326 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %327 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.51, i64 0, i64 0
  ; Converting complex expression to output
  %328 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %329 = call i32 (i8*, ...) @printf(i8* %328, i32 %327)
  %330 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %331 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %332 = load i32, i32* %287, align 4
  %333 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %334 = icmp eq i32 %332, %333
  %335 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %336 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.52, i64 0, i64 0
  ; Converting complex expression to output
  %337 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %338 = call i32 (i8*, ...) @printf(i8* %337, i32 %336)
  %339 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %340 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %341 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %342 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.53, i64 0, i64 0
  ; Converting complex expression to output
  %343 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %344 = call i32 (i8*, ...) @printf(i8* %343, i32 %342)
  %345 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %346 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.54, i64 0, i64 0
  ; Converting complex expression to output
  %347 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %348 = call i32 (i8*, ...) @printf(i8* %347, i32 %346)
  %349 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.21, i64 0, i64 0
  %350 = call i32 @parse_string_to_int(i32 %349)
  %351 = alloca i32, align 4
  store i32 %350, i32* %351, align 4
  ; Variable parsed_int allocated at %351
  %352 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.55, i64 0, i64 0
  %353 = call i32 @parse_string_to_float(i32 %352)
  %354 = alloca double, align 4
  store double %353, double* %354, align 4
  ; Variable parsed_float allocated at %354
  %355 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.56, i64 0, i64 0
  %356 = call i32 @parse_string_to_bool(i32 %355)
  %357 = alloca i1, align 4
  store i1 %356, i1* %357, align 4
  ; Variable parsed_bool allocated at %357
  %358 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %359 = load i32, i32* %351, align 4
  %360 = icmp eq i32 %359, 42
  %361 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %362 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.57, i64 0, i64 0
  ; Converting complex expression to output
  %363 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %364 = call i32 (i8*, ...) @printf(i8* %363, i32 %362)
  %365 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %366 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %367 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %368 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.58, i64 0, i64 0
  ; Converting complex expression to output
  %369 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %370 = call i32 (i8*, ...) @printf(i8* %369, i32 %368)
  %371 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %372 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %373 = load i32, i32* %354, align 4
  %374 = icmp eq i32 %373, 3.14
  %375 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %376 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.59, i64 0, i64 0
  ; Converting complex expression to output
  %377 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %378 = call i32 (i8*, ...) @printf(i8* %377, i32 %376)
  %379 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %380 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %381 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %382 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.60, i64 0, i64 0
  ; Converting complex expression to output
  %383 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %384 = call i32 (i8*, ...) @printf(i8* %383, i32 %382)
  %385 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %386 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %387 = load i32, i32* %357, align 4
  %388 = icmp eq i32 %387, 1
  %389 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %390 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.61, i64 0, i64 0
  ; Converting complex expression to output
  %391 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %392 = call i32 (i8*, ...) @printf(i8* %391, i32 %390)
  %393 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %394 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %395 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %396 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.62, i64 0, i64 0
  ; Converting complex expression to output
  %397 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %398 = call i32 (i8*, ...) @printf(i8* %397, i32 %396)
  %399 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %400 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.63, i64 0, i64 0
  ; Converting complex expression to output
  %401 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %402 = call i32 (i8*, ...) @printf(i8* %401, i32 %400)
  %403 = call i32 @get_method_count()
  %404 = alloca i32, align 4
  store i32 %403, i32* %404, align 4
  ; Variable method_count allocated at %404
  %405 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.29, i64 0, i64 0
  %406 = call i32 @get_method_signature(i32 %405)
  %407 = alloca i8*, align 4
  store i8* %406, i8** %407, align 4
  ; Variable signature allocated at %407
  %408 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.64, i64 0, i64 0
  %409 = call i32 @get_method_return_type(i32 %408)
  %410 = alloca i8*, align 4
  store i8* %409, i8** %410, align 4
  ; Variable return_type allocated at %410
  %411 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %412 = load i32, i32* %404, align 4
  %413 = icmp eq i32 %412, 4
  %414 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %415 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.65, i64 0, i64 0
  ; Converting complex expression to output
  %416 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %417 = call i32 (i8*, ...) @printf(i8* %416, i32 %415)
  %418 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %419 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %420 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %421 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.66, i64 0, i64 0
  ; Converting complex expression to output
  %422 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %423 = call i32 (i8*, ...) @printf(i8* %422, i32 %421)
  %424 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %425 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %426 = load i32, i32* %407, align 4
  %427 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.67, i64 0, i64 0
  %428 = icmp eq i32 %426, %427
  %429 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %430 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.68, i64 0, i64 0
  ; Converting complex expression to output
  %431 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %432 = call i32 (i8*, ...) @printf(i8* %431, i32 %430)
  %433 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %434 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %435 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %436 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.69, i64 0, i64 0
  ; Converting complex expression to output
  %437 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %438 = call i32 (i8*, ...) @printf(i8* %437, i32 %436)
  %439 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %440 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %441 = load i32, i32* %410, align 4
  %442 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.5, i64 0, i64 0
  %443 = icmp eq i32 %441, %442
  %444 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %445 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.70, i64 0, i64 0
  ; Converting complex expression to output
  %446 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %447 = call i32 (i8*, ...) @printf(i8* %446, i32 %445)
  %448 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %449 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %450 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %451 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.71, i64 0, i64 0
  ; Converting complex expression to output
  %452 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %453 = call i32 (i8*, ...) @printf(i8* %452, i32 %451)
  %454 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %455 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.72, i64 0, i64 0
  ; Converting complex expression to output
  %456 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %457 = call i32 (i8*, ...) @printf(i8* %456, i32 %455)
  %458 = call i32 @reflection_demo()
  %459 = alloca i1, align 4
  store i1 %458, i1* %459, align 4
  ; Variable demo_result allocated at %459
  %460 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %461 = load i32, i32* %459, align 4
  %462 = icmp eq i32 %461, 1
  %463 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %464 = getelementptr inbounds [48 x i8], [48 x i8]* @.str.73, i64 0, i64 0
  ; Converting complex expression to output
  %465 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %466 = call i32 (i8*, ...) @printf(i8* %465, i32 %464)
  %467 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %468 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %469 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %470 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.74, i64 0, i64 0
  ; Converting complex expression to output
  %471 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %472 = call i32 (i8*, ...) @printf(i8* %471, i32 %470)
  %473 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %474 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.2, i64 0, i64 0
  ; Converting complex expression to output
  %475 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %476 = call i32 (i8*, ...) @printf(i8* %475, i32 %474)
  %477 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.75, i64 0, i64 0
  ; Converting complex expression to output
  %478 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %479 = call i32 (i8*, ...) @printf(i8* %478, i32 %477)
  %480 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.76, i64 0, i64 0
  ; Converting complex expression to output
  %481 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %482 = call i32 (i8*, ...) @printf(i8* %481, i32 %480)
  %483 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.77, i64 0, i64 0
  ; Converting complex expression to output
  %484 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %485 = call i32 (i8*, ...) @printf(i8* %484, i32 %483)
  ret i32 0
}
