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



; String constants
@.str.2 = private unnamed_addr constant [23 x i8] c"<div>Hello World</div>\00", align 1
@.str.12 = private unnamed_addr constant [20 x i8] c"https://example.com\00", align 1
@.str.8 = private unnamed_addr constant [2 x i8] c"<\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.0 = private unnamed_addr constant [29 x i8] c"=== HTML Processing Demo ===\00", align 1
@.str.6 = private unnamed_addr constant [2 x i8] c"p\00", align 1
@.str.13 = private unnamed_addr constant [5 x i8] c"Link\00", align 1
@.str.11 = private unnamed_addr constant [39 x i8] c"<a href='https://example.com'>Link</a>\00", align 1
@.str.9 = private unnamed_addr constant [2 x i8] c">\00", align 1
@.str.3 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.10 = private unnamed_addr constant [3 x i8] c"</\00", align 1
@.str.14 = private unnamed_addr constant [24 x i8] c"=== Demo completed! ===\00", align 1
@.str.4 = private unnamed_addr constant [4 x i8] c"div\00", align 1
@.str.5 = private unnamed_addr constant [12 x i8] c"Hello World\00", align 1
@.str.7 = private unnamed_addr constant [20 x i8] c"This is a paragraph\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.0, i64 0, i64 0
  ; Converting complex expression to output
  %1 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %2 = call i32 (i8*, ...) @printf(i8* %1, i32 %0)
  %3 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.2, i64 0, i64 0
  %4 = alloca i8*, align 4
  store i8* %3, i8** %4, align 4
  ; Variable html_content allocated at %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %6 = load i32, i32* %4, align 4
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %8 = alloca i32, align 4
  store i32 1, i32* %8, align 4
  ; Variable tag_start allocated at %8
  %9 = alloca i32, align 4
  store i32 4, i32* %9, align 4
  ; Variable tag_end allocated at %9
  %10 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.4, i64 0, i64 0
  %11 = alloca i8*, align 4
  store i8* %10, i8** %11, align 4
  ; Variable tag_name allocated at %11
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %13 = load i32, i32* %11, align 4
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %15 = alloca i32, align 4
  store i32 5, i32* %15, align 4
  ; Variable content_start allocated at %15
  %16 = alloca i32, align 4
  store i32 16, i32* %16, align 4
  ; Variable content_end allocated at %16
  %17 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.5, i64 0, i64 0
  %18 = alloca i8*, align 4
  store i8* %17, i8** %18, align 4
  ; Variable content allocated at %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %20 = load i32, i32* %18, align 4
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %22 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.6, i64 0, i64 0
  %23 = alloca i8*, align 4
  store i8* %22, i8** %23, align 4
  ; Variable new_tag allocated at %23
  %24 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.7, i64 0, i64 0
  %25 = alloca i8*, align 4
  store i8* %24, i8** %25, align 4
  ; Variable new_content allocated at %25
  %26 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.8, i64 0, i64 0
  %27 = load i32, i32* %23, align 4
  %28 = add i32 %26, %27
  %29 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.9, i64 0, i64 0
  %30 = add i32 %28, %29
  %31 = load i32, i32* %25, align 4
  %32 = add i32 %30, %31
  %33 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.10, i64 0, i64 0
  %34 = add i32 %32, %33
  %35 = load i32, i32* %23, align 4
  %36 = add i32 %34, %35
  %37 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.9, i64 0, i64 0
  %38 = add i32 %36, %37
  %39 = alloca i8*, align 4
  store i8* %38, i8** %39, align 4
  ; Variable generated_html allocated at %39
  %40 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %41 = load i32, i32* %39, align 4
  %42 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %43 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.11, i64 0, i64 0
  %44 = alloca i8*, align 4
  store i8* %43, i8** %44, align 4
  ; Variable element_with_attrs allocated at %44
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %46 = load i32, i32* %44, align 4
  %47 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %48 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.12, i64 0, i64 0
  %49 = alloca i8*, align 4
  store i8* %48, i8** %49, align 4
  ; Variable href_value allocated at %49
  %50 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.13, i64 0, i64 0
  %51 = alloca i8*, align 4
  store i8* %50, i8** %51, align 4
  ; Variable link_text allocated at %51
  %52 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %53 = load i32, i32* %49, align 4
  %54 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %55 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %56 = load i32, i32* %51, align 4
  %57 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %58 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.14, i64 0, i64 0
  ; Converting complex expression to output
  %59 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %60 = call i32 (i8*, ...) @printf(i8* %59, i32 %58)
  ret i32 0
}
