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


; String constants
@.str.6 = private unnamed_addr constant [2 x i8] c"p\00", align 1
@.str.1 = private unnamed_addr constant [23 x i8] c"<div>Hello World</div>\00", align 1
@.str.2 = private unnamed_addr constant [4 x i8] c"div\00", align 1
@.str.4 = private unnamed_addr constant [20 x i8] c"create HTML element\00", align 1
@.str.3 = private unnamed_addr constant [12 x i8] c"Hello World\00", align 1
@.str.8 = private unnamed_addr constant [23 x i8] c"set and get attributes\00", align 1
@.str.0 = private unnamed_addr constant [31 x i8] c"parse_html basic functionality\00", align 1
@.str.7 = private unnamed_addr constant [15 x i8] c"Test paragraph\00", align 1
@.str.9 = private unnamed_addr constant [4 x i8] c"img\00", align 1
@.str.5 = private unnamed_addr constant [1 x i8] c"\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  %2 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.1, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable html allocated at %3
  %4 = load i32, i32* %3, align 4
  %5 = call i32 @parse_html(i32 %4)
  %6 = alloca %struct.HTMLElement, align 4
  store %struct.HTMLElement %5, %struct.HTMLElement* %6, align 4
  ; Variable element allocated at %6
  %7 = load i32, i32* %6, align 4
  ; Member access: %7.tag
  %8 = getelementptr inbounds %struct.object, %struct.object* %7, i32 0, i32 0
  %9 = load i32, i32* %8, align 4
  %10 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %11 = call i32 @assert_eq_string(i32 %9, i32 %10)
  %12 = load i32, i32* %6, align 4
  %13 = call i32 @get_text_content(i32 %12)
  %14 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.3, i64 0, i64 0
  %15 = call i32 @assert_eq_string(i32 %13, i32 %14)
  %16 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.4, i64 0, i64 0
  %17 = call i32 @test_start(i32 %16)
  %18 = alloca %struct.HTMLElement, align 4
  store %struct.HTMLElement null, %struct.HTMLElement* %18, align 4
  ; Variable test_element allocated at %18
  %19 = load i32, i32* %18, align 4
  ; Member access: %19.tag
  %20 = getelementptr inbounds %struct.object, %struct.object* %19, i32 0, i32 0
  %21 = load i32, i32* %20, align 4
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  %23 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.6, i64 0, i64 0
  %24 = load i32, i32* %18, align 4
  ; Member access: %24.content
  %25 = getelementptr inbounds %struct.object, %struct.object* %24, i32 0, i32 0
  %26 = load i32, i32* %25, align 4
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  %28 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.7, i64 0, i64 0
  %29 = load i32, i32* %18, align 4
  ; Member access: %29.attributes
  %30 = getelementptr inbounds %struct.object, %struct.object* %29, i32 0, i32 0
  %31 = load i32, i32* %30, align 4
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  %33 = call i32 @make_map()
  %34 = load i32, i32* %18, align 4
  ; Member access: %34.tag
  %35 = getelementptr inbounds %struct.object, %struct.object* %34, i32 0, i32 0
  %36 = load i32, i32* %35, align 4
  %37 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.6, i64 0, i64 0
  %38 = call i32 @assert_eq_string(i32 %36, i32 %37)
  %39 = load i32, i32* %18, align 4
  ; Member access: %39.content
  %40 = getelementptr inbounds %struct.object, %struct.object* %39, i32 0, i32 0
  %41 = load i32, i32* %40, align 4
  %42 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.7, i64 0, i64 0
  %43 = call i32 @assert_eq_string(i32 %41, i32 %42)
  %44 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.8, i64 0, i64 0
  %45 = call i32 @test_start(i32 %44)
  %46 = alloca %struct.HTMLElement, align 4
  store %struct.HTMLElement null, %struct.HTMLElement* %46, align 4
  ; Variable attr_element allocated at %46
  %47 = load i32, i32* %46, align 4
  ; Member access: %47.tag
  %48 = getelementptr inbounds %struct.object, %struct.object* %47, i32 0, i32 0
  %49 = load i32, i32* %48, align 4
  %50 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  %51 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.9, i64 0, i64 0
  %52 = load i32, i32* %46, align 4
  ; Member access: %52.attributes
  %53 = getelementptr inbounds %struct.object, %struct.object* %52, i32 0, i32 0
  %54 = load i32, i32* %53, align 4
  %55 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  %56 = call i32 @make_map()
  ret i32 0
}
