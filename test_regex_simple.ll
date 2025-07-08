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
define i32 @test_simple_regex() {
entry:
  %0 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.1, i64 0, i64 0
  %3 = call i32 @puts(i8* %2)
  %4 = add i32 0, 0
  ; Expression result: %4
  %5 = alloca {i32, i32, i32}, align 4
  %6 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %7 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %8 = icmp eq i32 %6, %7
  %9 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %5, i32 0, i32 0
  store i32 %8, i32* %9, align 4
  %10 = alloca {i1}*, align 4
  store {i1}* %5, {i1}** %10, align 4
  ; Variable result1 allocated
  %11 = load {i1}*, {i1}** %10, align 4
  %12 = call i32 @assert_true(i32 %11)
  ; Expression result: %12
  %13 = alloca {i32, i32, i32}, align 4
  %14 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %15 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.3, i64 0, i64 0
  %16 = icmp eq i32 %14, %15
  %17 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %13, i32 0, i32 0
  store i32 %16, i32* %17, align 4
  %18 = alloca {i1}*, align 4
  store {i1}* %13, {i1}** %18, align 4
  ; Variable result2 allocated
  %19 = load {i1}*, {i1}** %18, align 4
  %20 = call i32 @assert_false(i32 %19)
  ; Expression result: %20
  %21 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %22 = call i32 @string_len(i32 %21)
  %23 = alloca i32, align 4
  store i32 %22, i32* %23, align 4
  ; Variable len1 allocated
  %24 = load i32, i32* %23, align 4
  %25 = call i32 @assert_eq_int(i32 %24, i32 5)
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.4, i64 0, i64 0
  %27 = call i32 @string_len(i32 %26)
  %28 = alloca i32, align 4
  store i32 %27, i32* %28, align 4
  ; Variable len2 allocated
  %29 = load i32, i32* %28, align 4
  %30 = call i32 @assert_eq_int(i32 %29, i32 0)
  ; Expression result: %30
  %31 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %32 = call i32 @string_char_at(i32 %31, i32 0)
  %33 = alloca i32, align 4
  store i32 %32, i32* %33, align 4
  ; Variable char1 allocated
  %34 = load i32, i32* %33, align 4
  %35 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.5, i64 0, i64 0
  %36 = call i32 @assert_eq_string(i32 %34, i32 %35)
  ; Expression result: %36
  %37 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %38 = call i32 @string_char_at(i32 %37, i32 4)
  %39 = alloca i32, align 4
  store i32 %38, i32* %39, align 4
  ; Variable char2 allocated
  %40 = load i32, i32* %39, align 4
  %41 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.6, i64 0, i64 0
  %42 = call i32 @assert_eq_string(i32 %40, i32 %41)
  ; Expression result: %42
  %43 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.7, i64 0, i64 0
  %44 = call i32 @puts(i8* %43)
  %45 = add i32 0, 0
  ; Expression result: %45
  %46 = call i32 @print_test_summary()
  ; Expression result: %46
  ret i32 0
}



; String constants
@.str.3 = private unnamed_addr constant [6 x i8] c"world\00", align 1
@.str.6 = private unnamed_addr constant [2 x i8] c"o\00", align 1
@.str.0 = private unnamed_addr constant [19 x i8] c"Simple Regex Tests\00", align 1
@.str.7 = private unnamed_addr constant [28 x i8] c"Basic regex tests completed\00", align 1
@.str.2 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
@.str.5 = private unnamed_addr constant [2 x i8] c"h\00", align 1
@.str.1 = private unnamed_addr constant [37 x i8] c"Testing basic regex functionality...\00", align 1
@.str.4 = private unnamed_addr constant [1 x i8] c"\00", align 1
define i32 @main() {
  %0 = call i32 @test_simple_regex()
  ret i32 0
}
