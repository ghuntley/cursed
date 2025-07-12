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
@error_msg_default = private unnamed_addr constant [15 x i8] c"Error occurred\00"


; String constants
@.str.7 = private unnamed_addr constant [4 x i8] c"pi:\00", align 1
@.str.13 = private unnamed_addr constant [26 x i8] c"=== All tests passed! ===\00", align 1
@.str.3 = private unnamed_addr constant [7 x i8] c"CURSED\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.10 = private unnamed_addr constant [15 x i8] c"converted_int:\00", align 1
@.str.4 = private unnamed_addr constant [3 x i8] c"x:\00", align 1
@.str.6 = private unnamed_addr constant [6 x i8] c"flag:\00", align 1
@.str.12 = private unnamed_addr constant [3 x i8] c"y:\00", align 1
@.str.9 = private unnamed_addr constant [17 x i8] c"converted_float:\00", align 1
@.str.0 = private unnamed_addr constant [36 x i8] c"=== Type Alias Integration Test ===\00", align 1
@.str.8 = private unnamed_addr constant [33 x i8] c"2. Type conversion with aliases:\00", align 1
@.str.5 = private unnamed_addr constant [6 x i8] c"name:\00", align 1
@.str.11 = private unnamed_addr constant [24 x i8] c"3. Nested type aliases:\00", align 1
@.str.2 = private unnamed_addr constant [32 x i8] c"1. Basic type alias assignment:\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.0, i64 0, i64 0
  ; Converting complex expression to output
  %1 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %2 = call i32 (i8*, ...) @printf(i8* %1, i32 %0)
  %3 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.2, i64 0, i64 0
  ; Converting complex expression to output
  %4 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %5 = call i32 (i8*, ...) @printf(i8* %4, i32 %3)
  %6 = alloca %struct.MyInt, align 4
  store %struct.MyInt 42, %struct.MyInt* %6, align 4
  ; Variable x allocated at %6
  %7 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.3, i64 0, i64 0
  %8 = alloca %struct.MyString, align 4
  store %struct.MyString %7, %struct.MyString* %8, align 4
  ; Variable name allocated at %8
  %9 = alloca %struct.MyBool, align 4
  store %struct.MyBool 1, %struct.MyBool* %9, align 4
  ; Variable flag allocated at %9
  %10 = alloca %struct.MyFloat, align 4
  store %struct.MyFloat 3.14159, %struct.MyFloat* %10, align 4
  ; Variable pi allocated at %10
  %11 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.4, i64 0, i64 0
  ; Converting complex expression to output
  %12 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %13 = call i32 (i8*, ...) @printf(i8* %12, i32 %11)
  %14 = load i32, i32* %6, align 4
  %15 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %16 = call i32 (i8*, ...) @printf(i8* %15, i32 %14)
  %17 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.5, i64 0, i64 0
  ; Converting complex expression to output
  %18 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %19 = call i32 (i8*, ...) @printf(i8* %18, i32 %17)
  %20 = load i32, i32* %8, align 4
  %21 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %22 = call i32 (i8*, ...) @printf(i8* %21, i32 %20)
  %23 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.6, i64 0, i64 0
  ; Converting complex expression to output
  %24 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %25 = call i32 (i8*, ...) @printf(i8* %24, i32 %23)
  %26 = load i1, i1* %9, align 4
  %27 = zext i1 %26 to i32
  %28 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %29 = call i32 (i8*, ...) @printf(i8* %28, i32 %27)
  %30 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.7, i64 0, i64 0
  ; Converting complex expression to output
  %31 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %32 = call i32 (i8*, ...) @printf(i8* %31, i32 %30)
  %33 = load i32, i32* %10, align 4
  %34 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %35 = call i32 (i8*, ...) @printf(i8* %34, i32 %33)
  %36 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.8, i64 0, i64 0
  ; Converting complex expression to output
  %37 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %38 = call i32 (i8*, ...) @printf(i8* %37, i32 %36)
  %39 = alloca %struct.MyFloat, align 4
  store %struct.MyFloat 100, %struct.MyFloat* %39, align 4
  ; Variable converted_float allocated at %39
  %40 = alloca %struct.MyInt, align 4
  store %struct.MyInt 99, %struct.MyInt* %40, align 4
  ; Variable converted_int allocated at %40
  %41 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.9, i64 0, i64 0
  ; Converting complex expression to output
  %42 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %43 = call i32 (i8*, ...) @printf(i8* %42, i32 %41)
  %44 = load i32, i32* %39, align 4
  %45 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %46 = call i32 (i8*, ...) @printf(i8* %45, i32 %44)
  %47 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.10, i64 0, i64 0
  ; Converting complex expression to output
  %48 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %49 = call i32 (i8*, ...) @printf(i8* %48, i32 %47)
  %50 = load i32, i32* %40, align 4
  %51 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %52 = call i32 (i8*, ...) @printf(i8* %51, i32 %50)
  %53 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.11, i64 0, i64 0
  ; Converting complex expression to output
  %54 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %55 = call i32 (i8*, ...) @printf(i8* %54, i32 %53)
  %56 = alloca %struct.AnotherInt, align 4
  store %struct.AnotherInt 24, %struct.AnotherInt* %56, align 4
  ; Variable y allocated at %56
  %57 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.12, i64 0, i64 0
  ; Converting complex expression to output
  %58 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %59 = call i32 (i8*, ...) @printf(i8* %58, i32 %57)
  %60 = load i32, i32* %56, align 4
  %61 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %62 = call i32 (i8*, ...) @printf(i8* %61, i32 %60)
  %63 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.13, i64 0, i64 0
  ; Converting complex expression to output
  %64 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %65 = call i32 (i8*, ...) @printf(i8* %64, i32 %63)
  ret i32 0
}
