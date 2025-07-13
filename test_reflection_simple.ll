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
@.str.5 = private unnamed_addr constant [19 x i8] c"Integer to string:\00", align 1
@.str.7 = private unnamed_addr constant [19 x i8] c"Boolean type name:\00", align 1
@.str.9 = private unnamed_addr constant [15 x i8] c"Deep equality:\00", align 1
@.str.8 = private unnamed_addr constant [19 x i8] c"Boolean to string:\00", align 1
@.str.10 = private unnamed_addr constant [13 x i8] c"Demo result:\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.3 = private unnamed_addr constant [19 x i8] c"Integer type kind:\00", align 1
@.str.2 = private unnamed_addr constant [19 x i8] c"Integer type name:\00", align 1
@.str.11 = private unnamed_addr constant [29 x i8] c"Reflection testing complete!\00", align 1
@.str.6 = private unnamed_addr constant [18 x i8] c"Integer to float:\00", align 1
@.str.4 = private unnamed_addr constant [19 x i8] c"Integer type size:\00", align 1
@.str.0 = private unnamed_addr constant [36 x i8] c"Testing CURSED Reflection System...\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.0, i64 0, i64 0
  ; Converting complex expression to output
  %1 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %2 = call i32 (i8*, ...) @printf(i8* %1, i32 %0)
  %3 = alloca i32, align 4
  store i32 42, i32* %3, align 4
  ; Variable int_val allocated at %3
  %4 = load i32, i32* %3, align 4
  %5 = call i32 @get_type_name_int(i32 %4)
  %6 = alloca i8*, align 4
  store i8* %5, i8** %6, align 4
  ; Variable type_name allocated at %6
  %7 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.2, i64 0, i64 0
  ; Converting complex expression to output
  %8 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %9 = call i32 (i8*, ...) @printf(i8* %8, i32 %7)
  %10 = load i32, i32* %6, align 4
  %11 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %12 = call i32 (i8*, ...) @printf(i8* %11, i32 %10)
  %13 = load i32, i32* %3, align 4
  %14 = call i32 @get_type_kind_int(i32 %13)
  %15 = alloca i8*, align 4
  store i8* %14, i8** %15, align 4
  ; Variable type_kind allocated at %15
  %16 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.3, i64 0, i64 0
  ; Converting complex expression to output
  %17 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %18 = call i32 (i8*, ...) @printf(i8* %17, i32 %16)
  %19 = load i32, i32* %15, align 4
  %20 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %21 = call i32 (i8*, ...) @printf(i8* %20, i32 %19)
  %22 = load i32, i32* %3, align 4
  %23 = call i32 @get_type_size_int(i32 %22)
  %24 = alloca i32, align 4
  store i32 %23, i32* %24, align 4
  ; Variable type_size allocated at %24
  %25 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.4, i64 0, i64 0
  ; Converting complex expression to output
  %26 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %27 = call i32 (i8*, ...) @printf(i8* %26, i32 %25)
  %28 = load i32, i32* %24, align 4
  %29 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %30 = call i32 (i8*, ...) @printf(i8* %29, i32 %28)
  %31 = load i32, i32* %3, align 4
  %32 = call i32 @convert_int_to_string(i32 %31)
  %33 = alloca i8*, align 4
  store i8* %32, i8** %33, align 4
  ; Variable str_result allocated at %33
  %34 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.5, i64 0, i64 0
  ; Converting complex expression to output
  %35 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %36 = call i32 (i8*, ...) @printf(i8* %35, i32 %34)
  %37 = load i32, i32* %33, align 4
  %38 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %39 = call i32 (i8*, ...) @printf(i8* %38, i32 %37)
  %40 = load i32, i32* %3, align 4
  %41 = call i32 @convert_int_to_float(i32 %40)
  %42 = alloca double, align 4
  store double %41, double* %42, align 4
  ; Variable float_result allocated at %42
  %43 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.6, i64 0, i64 0
  ; Converting complex expression to output
  %44 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %45 = call i32 (i8*, ...) @printf(i8* %44, i32 %43)
  %46 = load i32, i32* %42, align 4
  %47 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %48 = call i32 (i8*, ...) @printf(i8* %47, i32 %46)
  %49 = alloca i1, align 4
  store i1 1, i1* %49, align 4
  ; Variable bool_val allocated at %49
  %50 = load i32, i32* %49, align 4
  %51 = call i32 @get_type_name_bool(i32 %50)
  %52 = alloca i8*, align 4
  store i8* %51, i8** %52, align 4
  ; Variable bool_type_name allocated at %52
  %53 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.7, i64 0, i64 0
  ; Converting complex expression to output
  %54 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %55 = call i32 (i8*, ...) @printf(i8* %54, i32 %53)
  %56 = load i32, i32* %52, align 4
  %57 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %58 = call i32 (i8*, ...) @printf(i8* %57, i32 %56)
  %59 = load i32, i32* %49, align 4
  %60 = call i32 @convert_bool_to_string(i32 %59)
  %61 = alloca i8*, align 4
  store i8* %60, i8** %61, align 4
  ; Variable bool_str allocated at %61
  %62 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.8, i64 0, i64 0
  ; Converting complex expression to output
  %63 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %64 = call i32 (i8*, ...) @printf(i8* %63, i32 %62)
  %65 = load i32, i32* %61, align 4
  %66 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %67 = call i32 (i8*, ...) @printf(i8* %66, i32 %65)
  %68 = alloca i32, align 4
  store i32 42, i32* %68, align 4
  ; Variable other_int allocated at %68
  %69 = load i32, i32* %3, align 4
  %70 = load i32, i32* %68, align 4
  %71 = call i32 @deep_equal_int(i32 %69, i32 %70)
  %72 = alloca i1, align 4
  store i1 %71, i1* %72, align 4
  ; Variable equal allocated at %72
  %73 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.9, i64 0, i64 0
  ; Converting complex expression to output
  %74 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %75 = call i32 (i8*, ...) @printf(i8* %74, i32 %73)
  %76 = load i32, i32* %72, align 4
  %77 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %78 = call i32 (i8*, ...) @printf(i8* %77, i32 %76)
  %79 = call i32 @reflection_demo()
  %80 = alloca i1, align 4
  store i1 %79, i1* %80, align 4
  ; Variable demo_result allocated at %80
  %81 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.10, i64 0, i64 0
  ; Converting complex expression to output
  %82 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %83 = call i32 (i8*, ...) @printf(i8* %82, i32 %81)
  %84 = load i32, i32* %80, align 4
  %85 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %86 = call i32 (i8*, ...) @printf(i8* %85, i32 %84)
  %87 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.11, i64 0, i64 0
  ; Converting complex expression to output
  %88 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %89 = call i32 (i8*, ...) @printf(i8* %88, i32 %87)
  ret i32 0
}
