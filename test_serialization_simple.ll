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

; CURSED runtime functions
declare void @cursed_panic(i8*, i64)
declare i8* @cursed_alloc(i64)
declare void @cursed_free(i8*)
declare i32 @cursed_goroutine_spawn(i8*)
declare void @cursed_channel_send(i8*, i8*)
declare i8* @cursed_channel_receive(i8*)

; Exception handling declarations
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
declare i8* @malloc(i32)
declare void @free(i8*)
@error_msg_default = private unnamed_addr constant [13 x i8] c"Error occurred\00"
define i32 @test_simple_serialization() {
entry:
  %0 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [45 x i8], [45 x i8]* @.str.1, i64 0, i64 0
  %3 = call i32 @puts(i8* %2)
  %4 = add i32 0, 0
  ; Expression result: %4
  %5 = alloca i32, align 4
  store i32 42, i32* %5, align 4
  ; Variable int1 allocated
  %6 = alloca i32, align 4
  store i32 123, i32* %6, align 4
  ; Variable int2 allocated
  %7 = load i32, i32* %5, align 4
  %8 = call i32 @assert_eq_int(i32 %7, i32 42)
  ; Expression result: %8
  %9 = load i32, i32* %6, align 4
  %10 = call i32 @assert_eq_int(i32 %9, i32 123)
  ; Expression result: %10
  %11 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %12 = alloca i8*, align 4
  store i8* %11, i8** %12, align 4
  ; Variable str1 allocated
  %13 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %14 = alloca i8*, align 4
  store i8* %13, i8** %14, align 4
  ; Variable str2 allocated
  %15 = load i8*, i8** %12, align 4
  %16 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %17 = call i32 @assert_eq_string(i8* %15, i32 %16)
  ; Expression result: %17
  %18 = load i8*, i8** %14, align 4
  %19 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %20 = call i32 @assert_eq_string(i8* %18, i32 %19)
  ; Expression result: %20
  %21 = alloca i1, align 4
  store i1 1, i1* %21, align 4
  ; Variable bool1 allocated
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %23 = alloca i8*, align 4
  store i8* %22, i8** %23, align 4
  ; Variable bool2 allocated
  %24 = load i1, i1* %21, align 4
  %25 = call i32 @assert_true(i32 %24)
  ; Expression result: %25
  %26 = load i8*, i8** %23, align 4
  %27 = call i32 @assert_false(i32 %26)
  ; Expression result: %27
  %28 = add i32 0, 0 ; literal placeholder
  %29 = alloca i8*, align 4
  store i8* %28, i8** %29, align 4
  ; Variable float1 allocated
  %30 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  ; Expression result: %30
  %31 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  ; Expression result: %31
  ; Expression result: 3.14
  %32 = add i32 0, 0 ; literal placeholder
  %33 = alloca i8*, align 4
  store i8* %32, i8** %33, align 4
  ; Variable float2 allocated
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  ; Expression result: %34
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  ; Expression result: %35
  ; Expression result: 2.71
  %36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  ; Expression result: %36
  ; Expression result: 3
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  ; Expression result: %37
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  ; Expression result: %38
  ; Expression result: 2
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  ; Expression result: %39
  %40 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.5, i64 0, i64 0
  %41 = alloca i8*, align 4
  store i8* %40, i8** %41, align 4
  ; Variable char_a allocated
  %42 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.6, i64 0, i64 0
  %43 = alloca i8*, align 4
  store i8* %42, i8** %43, align 4
  ; Variable char_b allocated
  %44 = load i8*, i8** %41, align 4
  %45 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.5, i64 0, i64 0
  %46 = call i32 @assert_eq_string(i32 %44, i32 %45)
  ; Expression result: %46
  %47 = load i8*, i8** %43, align 4
  %48 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.6, i64 0, i64 0
  %49 = call i32 @assert_eq_string(i32 %47, i32 %48)
  ; Expression result: %49
  %50 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.7, i64 0, i64 0
  %51 = call i32 @puts(i8* %50)
  %52 = add i32 0, 0
  ; Expression result: %52
  %53 = call i32 @print_test_summary()
  ; Expression result: %53
  ret i32 0
}



; String constants
@.str.4 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.2 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
@.str.1 = private unnamed_addr constant [45 x i8] c"Testing basic serialization functionality...\00", align 1
@.str.0 = private unnamed_addr constant [27 x i8] c"Simple Serialization Tests\00", align 1
@.str.3 = private unnamed_addr constant [6 x i8] c"world\00", align 1
@.str.7 = private unnamed_addr constant [36 x i8] c"Basic serialization tests completed\00", align 1
@.str.6 = private unnamed_addr constant [2 x i8] c"B\00", align 1
@.str.5 = private unnamed_addr constant [2 x i8] c"A\00", align 1
define i32 @main() {
  %0 = call i32 @test_simple_serialization()
  ret i32 0
}
