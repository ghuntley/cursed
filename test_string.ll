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
define i32 @test_string_length() {
entry:
  %0 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %3 = call i32 @string_len(i32 %2)
  %4 = call i32 @assert_eq_int(i32 %3, i32 5)
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %6 = call i32 @string_len(i32 %5)
  %7 = call i32 @assert_eq_int(i32 %6, i32 0)
  ; Expression result: %7
  %8 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.3, i64 0, i64 0
  %9 = call i32 @string_len(i32 %8)
  %10 = call i32 @assert_eq_int(i32 %9, i32 6)
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %12 = call i32 @string_is_empty(i32 %11)
  %13 = call i32 @assert_true(i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %15 = call i32 @string_is_empty(i32 %14)
  %16 = call i32 @assert_false(i32 %15)
  ; Expression result: %16
  ret i32 0
}

define i32 @test_string_case_conversion() {
entry:
  %0 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.4, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %3 = call i32 @string_to_upper(i32 %2)
  %4 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.5, i64 0, i64 0
  %5 = call i32 @assert_eq_string(i32 %3, i32 %4)
  ; Expression result: %5
  %6 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.3, i64 0, i64 0
  %7 = call i32 @string_to_upper(i32 %6)
  %8 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.3, i64 0, i64 0
  %9 = call i32 @assert_eq_string(i32 %7, i32 %8)
  ; Expression result: %9
  %10 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.6, i64 0, i64 0
  %11 = call i32 @string_to_upper(i32 %10)
  %12 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.7, i64 0, i64 0
  %13 = call i32 @assert_eq_string(i32 %11, i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.5, i64 0, i64 0
  %15 = call i32 @string_to_lower(i32 %14)
  %16 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %17 = call i32 @assert_eq_string(i32 %15, i32 %16)
  ; Expression result: %17
  %18 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.8, i64 0, i64 0
  %19 = call i32 @string_to_lower(i32 %18)
  %20 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.8, i64 0, i64 0
  %21 = call i32 @assert_eq_string(i32 %19, i32 %20)
  ; Expression result: %21
  %22 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.6, i64 0, i64 0
  %23 = call i32 @string_to_lower(i32 %22)
  %24 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.9, i64 0, i64 0
  %25 = call i32 @assert_eq_string(i32 %23, i32 %24)
  ; Expression result: %25
  %26 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %27 = call i32 @string_capitalize(i32 %26)
  %28 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.10, i64 0, i64 0
  %29 = call i32 @assert_eq_string(i32 %27, i32 %28)
  ; Expression result: %29
  %30 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.5, i64 0, i64 0
  %31 = call i32 @string_capitalize(i32 %30)
  %32 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.10, i64 0, i64 0
  %33 = call i32 @assert_eq_string(i32 %31, i32 %32)
  ; Expression result: %33
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %35 = call i32 @string_capitalize(i32 %34)
  %36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %37 = call i32 @assert_eq_string(i32 %35, i32 %36)
  ; Expression result: %37
  ret i32 0
}

define i32 @test_string_trimming() {
entry:
  %0 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.11, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.12, i64 0, i64 0
  %3 = call i32 @string_trim(i32 %2)
  %4 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %5 = call i32 @assert_eq_string(i32 %3, i32 %4)
  ; Expression result: %5
  %6 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.13, i64 0, i64 0
  %7 = call i32 @string_trim(i32 %6)
  %8 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.14, i64 0, i64 0
  %9 = call i32 @assert_eq_string(i32 %7, i32 %8)
  ; Expression result: %9
  %10 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.15, i64 0, i64 0
  %11 = call i32 @string_trim(i32 %10)
  %12 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.15, i64 0, i64 0
  %13 = call i32 @assert_eq_string(i32 %11, i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.12, i64 0, i64 0
  %15 = call i32 @string_trim_start(i32 %14)
  %16 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.16, i64 0, i64 0
  %17 = call i32 @assert_eq_string(i32 %15, i32 %16)
  ; Expression result: %17
  %18 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.17, i64 0, i64 0
  %19 = call i32 @string_trim_start(i32 %18)
  %20 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.14, i64 0, i64 0
  %21 = call i32 @assert_eq_string(i32 %19, i32 %20)
  ; Expression result: %21
  %22 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.12, i64 0, i64 0
  %23 = call i32 @string_trim_end(i32 %22)
  %24 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.18, i64 0, i64 0
  %25 = call i32 @assert_eq_string(i32 %23, i32 %24)
  ; Expression result: %25
  %26 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.19, i64 0, i64 0
  %27 = call i32 @string_trim_end(i32 %26)
  %28 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.14, i64 0, i64 0
  %29 = call i32 @assert_eq_string(i32 %27, i32 %28)
  ; Expression result: %29
  ret i32 0
}

define i32 @test_string_search() {
entry:
  %0 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.20, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %3 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %4 = call i32 @string_contains(i32 %2, i32 %3)
  %5 = call i32 @assert_true(i32 %4)
  ; Expression result: %5
  %6 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %7 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %8 = call i32 @string_contains(i32 %6, i32 %7)
  %9 = call i32 @assert_true(i32 %8)
  ; Expression result: %9
  %10 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %11 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.23, i64 0, i64 0
  %12 = call i32 @string_contains(i32 %10, i32 %11)
  %13 = call i32 @assert_false(i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %15 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %16 = call i32 @string_starts_with(i32 %14, i32 %15)
  %17 = call i32 @assert_true(i32 %16)
  ; Expression result: %17
  %18 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %20 = call i32 @string_starts_with(i32 %18, i32 %19)
  %21 = call i32 @assert_true(i32 %20)
  ; Expression result: %21
  %22 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %23 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %24 = call i32 @string_starts_with(i32 %22, i32 %23)
  %25 = call i32 @assert_false(i32 %24)
  ; Expression result: %25
  %26 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %27 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %28 = call i32 @string_ends_with(i32 %26, i32 %27)
  %29 = call i32 @assert_true(i32 %28)
  ; Expression result: %29
  %30 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %31 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %32 = call i32 @string_ends_with(i32 %30, i32 %31)
  %33 = call i32 @assert_true(i32 %32)
  ; Expression result: %33
  %34 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %35 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %36 = call i32 @string_ends_with(i32 %34, i32 %35)
  %37 = call i32 @assert_false(i32 %36)
  ; Expression result: %37
  ret i32 0
}

define i32 @test_string_indexing() {
entry:
  %0 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.24, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %3 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %4 = call i32 @string_index_of(i32 %2, i32 %3)
  %5 = call i32 @assert_eq_int(i32 %4, i32 6)
  ; Expression result: %5
  %6 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %7 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %8 = call i32 @string_index_of(i32 %6, i32 %7)
  %9 = call i32 @assert_eq_int(i32 %8, i32 0)
  ; Expression result: %9
  ; Expression result: 1
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %10
  %11 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.25, i64 0, i64 0
  %12 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %13 = call i32 @string_last_index_of(i32 %11, i32 %12)
  %14 = call i32 @assert_eq_int(i32 %13, i32 6)
  ; Expression result: %14
  ; Expression result: 1
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %15
  %16 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.26, i64 0, i64 0
  %17 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %18 = call i32 @string_count_occurrences(i32 %16, i32 %17)
  %19 = call i32 @assert_eq_int(i32 %18, i32 3)
  ; Expression result: %19
  %20 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %21 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.27, i64 0, i64 0
  %22 = call i32 @string_count_occurrences(i32 %20, i32 %21)
  %23 = call i32 @assert_eq_int(i32 %22, i32 3)
  ; Expression result: %23
  %24 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %25 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.23, i64 0, i64 0
  %26 = call i32 @string_count_occurrences(i32 %24, i32 %25)
  %27 = call i32 @assert_eq_int(i32 %26, i32 0)
  ; Expression result: %27
  ret i32 0
}

define i32 @test_string_slicing() {
entry:
  %0 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.28, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %3 = call i32 @string_slice(i32 %2, i32 0, i32 5)
  %4 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %5 = call i32 @assert_eq_string(i32 %3, i32 %4)
  ; Expression result: %5
  %6 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %7 = call i32 @string_slice(i32 %6, i32 6, i32 11)
  %8 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %9 = call i32 @assert_eq_string(i32 %7, i32 %8)
  ; Expression result: %9
  %10 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %11 = call i32 @string_slice(i32 %10, i32 2, i32 8)
  %12 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.29, i64 0, i64 0
  %13 = call i32 @assert_eq_string(i32 %11, i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %15 = call i32 @string_substring(i32 %14, i32 0, i32 5)
  %16 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %17 = call i32 @assert_eq_string(i32 %15, i32 %16)
  ; Expression result: %17
  %18 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %19 = call i32 @string_substring(i32 %18, i32 6, i32 5)
  %20 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %21 = call i32 @assert_eq_string(i32 %19, i32 %20)
  ; Expression result: %21
  %22 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %23 = call i32 @string_substring(i32 %22, i32 2, i32 6)
  %24 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.29, i64 0, i64 0
  %25 = call i32 @assert_eq_string(i32 %23, i32 %24)
  ; Expression result: %25
  %26 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %27 = call i32 @string_char_at(i32 %26, i32 0)
  %28 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.30, i64 0, i64 0
  %29 = call i32 @assert_eq_string(i32 %27, i32 %28)
  ; Expression result: %29
  %30 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %31 = call i32 @string_char_at(i32 %30, i32 4)
  %32 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.31, i64 0, i64 0
  %33 = call i32 @assert_eq_string(i32 %31, i32 %32)
  ; Expression result: %33
  %34 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %35 = call i32 @string_char_at(i32 %34, i32 2)
  %36 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.27, i64 0, i64 0
  %37 = call i32 @assert_eq_string(i32 %35, i32 %36)
  ; Expression result: %37
  ret i32 0
}

define i8* @test_string_splitting() {
entry:
  %0 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.32, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.33, i64 0, i64 0
  %3 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.34, i64 0, i64 0
  %4 = call i32 @string_split(i32 %2, i32 %3)
  %5 = alloca i32, align 4
  store i32 %4, i32* %5, align 4
  ; Variable parts allocated
  %6 = load i32, i32* %5, align 4
  %7 = call i32 @len(i32 %6)
  %8 = call i32 @assert_eq_int(i32 %7, i32 3)
  ; Expression result: %8
  %9 = alloca [1x i32], align 4
  %10 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.35, i64 0, i64 0
  %11 = getelementptr inbounds [1x i32], [1x i32]* %9, i64 0, i64 0
  store i32 %10, i32* %11, align 4
  ; Expression result: %9
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %12
  %13 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.36, i64 0, i64 0
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %14
  %15 = alloca [1x i32], align 4
  %16 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.37, i64 0, i64 0
  %17 = getelementptr inbounds [1x i32], [1x i32]* %15, i64 0, i64 0
  store i32 %16, i32* %17, align 4
  ; Expression result: %15
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %18
  %19 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.38, i64 0, i64 0
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %20
  %21 = alloca [1x i32], align 4
  %22 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.39, i64 0, i64 0
  %23 = getelementptr inbounds [1x i32], [1x i32]* %21, i64 0, i64 0
  store i32 %22, i32* %23, align 4
  ; Expression result: %21
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %24
  %25 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.40, i64 0, i64 0
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %26
  %27 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.41, i64 0, i64 0
  %28 = call i32 @string_split_lines(i32 %27)
  %29 = alloca i32, align 4
  store i32 %28, i32* %29, align 4
  ; Variable lines allocated
  %30 = load i32, i32* %29, align 4
  %31 = call i32 @len(i32 %30)
  %32 = call i32 @assert_eq_int(i32 %31, i32 3)
  ; Expression result: %32
  %33 = alloca [1x i32], align 4
  %34 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.35, i64 0, i64 0
  %35 = getelementptr inbounds [1x i32], [1x i32]* %33, i64 0, i64 0
  store i32 %34, i32* %35, align 4
  ; Expression result: %33
  %36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %36
  %37 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.42, i64 0, i64 0
  ; Expression result: %37
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %38
  %39 = alloca [1x i32], align 4
  %40 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.37, i64 0, i64 0
  %41 = getelementptr inbounds [1x i32], [1x i32]* %39, i64 0, i64 0
  store i32 %40, i32* %41, align 4
  ; Expression result: %39
  %42 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %42
  %43 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.43, i64 0, i64 0
  ; Expression result: %43
  %44 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %44
  %45 = alloca [1x i32], align 4
  %46 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.39, i64 0, i64 0
  %47 = getelementptr inbounds [1x i32], [1x i32]* %45, i64 0, i64 0
  store i32 %46, i32* %47, align 4
  ; Expression result: %45
  %48 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %48
  %49 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.44, i64 0, i64 0
  ; Expression result: %49
  %50 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %50
  %51 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.45, i64 0, i64 0
  %52 = call i32 @string_split_whitespace(i32 %51)
  %53 = alloca i32, align 4
  store i32 %52, i32* %53, align 4
  ; Variable words allocated
  %54 = load i32, i32* %53, align 4
  %55 = call i32 @len(i32 %54)
  %56 = call i32 @assert_eq_int(i32 %55, i32 3)
  ; Expression result: %56
  %57 = alloca [1x i32], align 4
  %58 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.35, i64 0, i64 0
  %59 = getelementptr inbounds [1x i32], [1x i32]* %57, i64 0, i64 0
  store i32 %58, i32* %59, align 4
  ; Expression result: %57
  %60 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %60
  %61 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %61
  %62 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %62
  %63 = alloca [1x i32], align 4
  %64 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.37, i64 0, i64 0
  %65 = getelementptr inbounds [1x i32], [1x i32]* %63, i64 0, i64 0
  store i32 %64, i32* %65, align 4
  ; Expression result: %63
  %66 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %66
  %67 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  ; Expression result: %67
  %68 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %68
  %69 = alloca [1x i32], align 4
  %70 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.39, i64 0, i64 0
  %71 = getelementptr inbounds [1x i32], [1x i32]* %69, i64 0, i64 0
  store i32 %70, i32* %71, align 4
  ; Expression result: %69
  %72 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %72
  %73 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.14, i64 0, i64 0
  ; Expression result: %73
  %74 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %74
  ret i32 0
}

define i32 @test_string_replacement() {
entry:
  %0 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.46, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.25, i64 0, i64 0
  %3 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %4 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.47, i64 0, i64 0
  %5 = call i32 @string_replace(i32 %2, i32 %3, i32 %4)
  %6 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.48, i64 0, i64 0
  %7 = call i32 @assert_eq_string(i32 %5, i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %9 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.23, i64 0, i64 0
  %10 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.49, i64 0, i64 0
  %11 = call i32 @string_replace(i32 %8, i32 %9, i32 %10)
  %12 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %13 = call i32 @assert_eq_string(i32 %11, i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.25, i64 0, i64 0
  %15 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %16 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.47, i64 0, i64 0
  %17 = call i32 @string_replace_all(i32 %14, i32 %15, i32 %16)
  %18 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.50, i64 0, i64 0
  %19 = call i32 @assert_eq_string(i32 %17, i32 %18)
  ; Expression result: %19
  %20 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %21 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.27, i64 0, i64 0
  %22 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.51, i64 0, i64 0
  %23 = call i32 @string_replace_all(i32 %20, i32 %21, i32 %22)
  %24 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.52, i64 0, i64 0
  %25 = call i32 @assert_eq_string(i32 %23, i32 %24)
  ; Expression result: %25
  %26 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.49, i64 0, i64 0
  %27 = call i32 @string_repeat(i32 %26, i32 3)
  %28 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.53, i64 0, i64 0
  %29 = call i32 @assert_eq_string(i32 %27, i32 %28)
  ; Expression result: %29
  %30 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.51, i64 0, i64 0
  %31 = call i32 @string_repeat(i32 %30, i32 0)
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %33 = call i32 @assert_eq_string(i32 %31, i32 %32)
  ; Expression result: %33
  %34 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.14, i64 0, i64 0
  %35 = call i32 @string_repeat(i32 %34, i32 1)
  %36 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.14, i64 0, i64 0
  %37 = call i32 @assert_eq_string(i32 %35, i32 %36)
  ; Expression result: %37
  ret i32 0
}

define i32 @test_string_padding() {
entry:
  %0 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.54, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %3 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.55, i64 0, i64 0
  %4 = call i32 @string_pad_left(i32 %2, i32 10, i32 %3)
  %5 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.56, i64 0, i64 0
  %6 = call i32 @assert_eq_string(i32 %4, i32 %5)
  ; Expression result: %6
  %7 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %8 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.35, i64 0, i64 0
  %9 = call i32 @string_pad_left(i32 %7, i32 8, i32 %8)
  %10 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.57, i64 0, i64 0
  %11 = call i32 @assert_eq_string(i32 %9, i32 %10)
  ; Expression result: %11
  %12 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %13 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.51, i64 0, i64 0
  %14 = call i32 @string_pad_left(i32 %12, i32 5, i32 %13)
  %15 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %16 = call i32 @assert_eq_string(i32 %14, i32 %15)
  ; Expression result: %16
  %17 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %18 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.55, i64 0, i64 0
  %19 = call i32 @string_pad_right(i32 %17, i32 10, i32 %18)
  %20 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.58, i64 0, i64 0
  %21 = call i32 @assert_eq_string(i32 %19, i32 %20)
  ; Expression result: %21
  %22 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %23 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.35, i64 0, i64 0
  %24 = call i32 @string_pad_right(i32 %22, i32 8, i32 %23)
  %25 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.59, i64 0, i64 0
  %26 = call i32 @assert_eq_string(i32 %24, i32 %25)
  ; Expression result: %26
  %27 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %28 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.51, i64 0, i64 0
  %29 = call i32 @string_pad_right(i32 %27, i32 5, i32 %28)
  %30 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %31 = call i32 @assert_eq_string(i32 %29, i32 %30)
  ; Expression result: %31
  %32 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %33 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.55, i64 0, i64 0
  %34 = call i32 @string_pad_center(i32 %32, i32 9, i32 %33)
  %35 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.12, i64 0, i64 0
  %36 = call i32 @assert_eq_string(i32 %34, i32 %35)
  ; Expression result: %36
  %37 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.47, i64 0, i64 0
  %38 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.51, i64 0, i64 0
  %39 = call i32 @string_pad_center(i32 %37, i32 6, i32 %38)
  %40 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.60, i64 0, i64 0
  %41 = call i32 @assert_eq_string(i32 %39, i32 %40)
  ; Expression result: %41
  ret i32 0
}

define i32 @test_string_validation() {
entry:
  %0 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.61, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.62, i64 0, i64 0
  %3 = call i32 @string_is_numeric(i32 %2)
  %4 = call i32 @assert_true(i32 %3)
  ; Expression result: %4
  %5 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.63, i64 0, i64 0
  %6 = call i32 @string_is_numeric(i32 %5)
  %7 = call i32 @assert_true(i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.64, i64 0, i64 0
  %9 = call i32 @string_is_numeric(i32 %8)
  %10 = call i32 @assert_true(i32 %9)
  ; Expression result: %10
  %11 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.49, i64 0, i64 0
  %12 = call i32 @string_is_numeric(i32 %11)
  %13 = call i32 @assert_false(i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.65, i64 0, i64 0
  %15 = call i32 @string_is_numeric(i32 %14)
  %16 = call i32 @assert_false(i32 %15)
  ; Expression result: %16
  %17 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %18 = call i32 @string_is_alpha(i32 %17)
  %19 = call i32 @assert_true(i32 %18)
  ; Expression result: %19
  %20 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.5, i64 0, i64 0
  %21 = call i32 @string_is_alpha(i32 %20)
  %22 = call i32 @assert_true(i32 %21)
  ; Expression result: %22
  %23 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.66, i64 0, i64 0
  %24 = call i32 @string_is_alpha(i32 %23)
  %25 = call i32 @assert_false(i32 %24)
  ; Expression result: %25
  %26 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.62, i64 0, i64 0
  %27 = call i32 @string_is_alpha(i32 %26)
  %28 = call i32 @assert_false(i32 %27)
  ; Expression result: %28
  %29 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.66, i64 0, i64 0
  %30 = call i32 @string_is_alphanumeric(i32 %29)
  %31 = call i32 @assert_true(i32 %30)
  ; Expression result: %31
  %32 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.67, i64 0, i64 0
  %33 = call i32 @string_is_alphanumeric(i32 %32)
  %34 = call i32 @assert_true(i32 %33)
  ; Expression result: %34
  %35 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.68, i64 0, i64 0
  %36 = call i32 @string_is_alphanumeric(i32 %35)
  %37 = call i32 @assert_false(i32 %36)
  ; Expression result: %37
  %38 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.69, i64 0, i64 0
  %39 = call i32 @string_is_alphanumeric(i32 %38)
  %40 = call i32 @assert_false(i32 %39)
  ; Expression result: %40
  %41 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.70, i64 0, i64 0
  %42 = call i32 @string_is_whitespace(i32 %41)
  %43 = call i32 @assert_true(i32 %42)
  ; Expression result: %43
  %44 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.71, i64 0, i64 0
  %45 = call i32 @string_is_whitespace(i32 %44)
  %46 = call i32 @assert_true(i32 %45)
  ; Expression result: %46
  %47 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %48 = call i32 @string_is_whitespace(i32 %47)
  %49 = call i32 @assert_false(i32 %48)
  ; Expression result: %49
  %50 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.12, i64 0, i64 0
  %51 = call i32 @string_is_whitespace(i32 %50)
  %52 = call i32 @assert_false(i32 %51)
  ; Expression result: %52
  ret i32 0
}

define i32 @test_string_conversion() {
entry:
  %0 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.72, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.62, i64 0, i64 0
  %3 = call i32 @string_to_int(i32 %2)
  %4 = call i32 @assert_eq_int(i32 %3, i32 123)
  ; Expression result: %4
  ; Expression result: 456
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %5
  %6 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.35, i64 0, i64 0
  %7 = call i32 @string_to_int(i32 %6)
  %8 = call i32 @assert_eq_int(i32 %7, i32 0)
  ; Expression result: %8
  %9 = alloca {i32, i32, i32}, align 4
  %10 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.63, i64 0, i64 0
  %11 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %9, i32 0, i32 0
  store i32 %10, i32* %11, align 4
  ; Expression result: %9
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %12
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %13
  %14 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.63, i64 0, i64 0
  ; Expression result: %14
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %15
  %16 = alloca {i32, i32, i32}, align 4
  %17 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.73, i64 0, i64 0
  %18 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %16, i32 0, i32 0
  store i32 %17, i32* %18, align 4
  ; Expression result: %16
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %20
  %21 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.73, i64 0, i64 0
  ; Expression result: %21
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %22
  %23 = alloca {i32, i32, i32}, align 4
  %24 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.74, i64 0, i64 0
  %25 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %23, i32 0, i32 0
  store i32 %24, i32* %25, align 4
  ; Expression result: %23
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %26
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %27
  %28 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.74, i64 0, i64 0
  ; Expression result: %28
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %29
  %30 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.75, i64 0, i64 0
  %31 = call i32 @string_to_bool(i32 %30)
  %32 = call i32 @assert_true(i32 %31)
  ; Expression result: %32
  %33 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.76, i64 0, i64 0
  %34 = call i32 @string_to_bool(i32 %33)
  %35 = call i32 @assert_true(i32 %34)
  ; Expression result: %35
  %36 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.77, i64 0, i64 0
  %37 = call i32 @string_to_bool(i32 %36)
  %38 = call i32 @assert_false(i32 %37)
  ; Expression result: %38
  %39 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.78, i64 0, i64 0
  %40 = call i32 @string_to_bool(i32 %39)
  %41 = call i32 @assert_false(i32 %40)
  ; Expression result: %41
  %42 = call i32 @string_from_int(i32 123)
  %43 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.62, i64 0, i64 0
  %44 = call i32 @assert_eq_string(i32 %42, i32 %43)
  ; Expression result: %44
  ; Expression result: 456
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %45
  %46 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %46
  %47 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.79, i64 0, i64 0
  ; Expression result: %47
  %48 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %48
  %49 = call i32 @string_from_bool(i32 1)
  %50 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.75, i64 0, i64 0
  %51 = call i32 @assert_eq_string(i32 %49, i32 %50)
  ; Expression result: %51
  %52 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %53 = call i32 @string_from_bool(i32 %52)
  %54 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.77, i64 0, i64 0
  %55 = call i32 @assert_eq_string(i32 %53, i32 %54)
  ; Expression result: %55
  ret i32 0
}

define i8* @test_string_utilities() {
entry:
  %0 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.80, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %3 = call i32 @string_reverse(i32 %2)
  %4 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.81, i64 0, i64 0
  %5 = call i32 @assert_eq_string(i32 %3, i32 %4)
  ; Expression result: %5
  %6 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.49, i64 0, i64 0
  %7 = call i32 @string_reverse(i32 %6)
  %8 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.82, i64 0, i64 0
  %9 = call i32 @assert_eq_string(i32 %7, i32 %8)
  ; Expression result: %9
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %11 = call i32 @string_reverse(i32 %10)
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %13 = call i32 @assert_eq_string(i32 %11, i32 %12)
  ; Expression result: %13
  %14 = alloca [0x i32], align 4
  %15 = alloca [0 x i32]*, align 4
  store [0 x i32]* %14, [0 x i32]** %15, align 4
  ; Variable words allocated
  %16 = load [0 x i32]*, [0 x i32]** %15, align 4
  %17 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.55, i64 0, i64 0
  %18 = call i32 @string_join(i32 %16, i32 %17)
  %19 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.83, i64 0, i64 0
  %20 = call i32 @assert_eq_string(i32 %18, i32 %19)
  ; Expression result: %20
  %21 = load [0 x i32]*, [0 x i32]** %15, align 4
  %22 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.34, i64 0, i64 0
  %23 = call i32 @string_join(i32 %21, i32 %22)
  %24 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.84, i64 0, i64 0
  %25 = call i32 @assert_eq_string(i32 %23, i32 %24)
  ; Expression result: %25
  %26 = load [0 x i32]*, [0 x i32]** %15, align 4
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %28 = call i32 @string_join(i32 %26, i32 %27)
  %29 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.85, i64 0, i64 0
  %30 = call i32 @assert_eq_string(i32 %28, i32 %29)
  ; Expression result: %30
  %31 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %32 = call i32 @string_hash(i32 %31)
  %33 = alloca i32, align 4
  store i32 %32, i32* %33, align 4
  ; Variable hash1 allocated
  %34 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %35 = call i32 @string_hash(i32 %34)
  %36 = alloca i32, align 4
  store i32 %35, i32* %36, align 4
  ; Variable hash2 allocated
  %37 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %38 = call i32 @string_hash(i32 %37)
  %39 = alloca i32, align 4
  store i32 %38, i32* %39, align 4
  ; Variable hash3 allocated
  %40 = load i32, i32* %33, align 4
  %41 = load i32, i32* %36, align 4
  %42 = call i32 @assert_eq_int(i32 %40, i32 %41)
  ; Expression result: %42
  %43 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %43
  %44 = load i32, i32* %39, align 4
  ; Expression result: %44
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %45
  ret i32 0
}

define i8* @test_string_distance() {
entry:
  %0 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.86, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %3 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %4 = call i32 @string_levenshtein_distance(i32 %2, i32 %3)
  %5 = call i32 @assert_eq_int(i32 %4, i32 0)
  ; Expression result: %5
  %6 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %7 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.87, i64 0, i64 0
  %8 = call i32 @string_levenshtein_distance(i32 %6, i32 %7)
  %9 = call i32 @assert_eq_int(i32 %8, i32 1)
  ; Expression result: %9
  %10 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %12 = call i32 @string_levenshtein_distance(i32 %10, i32 %11)
  %13 = call i32 @assert_eq_int(i32 %12, i32 5)
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %15 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %16 = call i32 @string_levenshtein_distance(i32 %14, i32 %15)
  %17 = call i32 @assert_eq_int(i32 %16, i32 5)
  ; Expression result: %17
  %18 = alloca {i32, i32, i32}, align 4
  %19 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %20 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %18, i32 0, i32 0
  store i32 %19, i32* %20, align 4
  %21 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %22 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %18, i32 0, i32 1
  store i32 %21, i32* %22, align 4
  ; Expression result: %18
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %23
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %24
  %25 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.88, i64 0, i64 0
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %26
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %27
  ; Expression result: 0
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %28
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %29
  ; Expression result: 1
  %30 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %30
  ret i32 0
}

define i32 @test_string_regex() {
entry:
  %0 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.89, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.90, i64 0, i64 0
  %3 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.62, i64 0, i64 0
  %4 = call i32 @regex_match(i32 %2, i32 %3)
  %5 = call i32 @assert_true(i32 %4)
  ; Expression result: %5
  %6 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.91, i64 0, i64 0
  %7 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %8 = call i32 @regex_match(i32 %6, i32 %7)
  %9 = call i32 @assert_true(i32 %8)
  ; Expression result: %9
  %10 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.90, i64 0, i64 0
  %11 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %12 = call i32 @regex_match(i32 %10, i32 %11)
  %13 = call i32 @assert_false(i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.90, i64 0, i64 0
  %15 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.92, i64 0, i64 0
  %16 = call i32 @regex_find(i32 %14, i32 %15)
  %17 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.62, i64 0, i64 0
  %18 = call i32 @assert_eq_string(i32 %16, i32 %17)
  ; Expression result: %18
  %19 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.91, i64 0, i64 0
  %20 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.93, i64 0, i64 0
  %21 = call i32 @regex_find(i32 %19, i32 %20)
  %22 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.49, i64 0, i64 0
  %23 = call i32 @assert_eq_string(i32 %21, i32 %22)
  ; Expression result: %23
  %24 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.90, i64 0, i64 0
  %25 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.92, i64 0, i64 0
  %26 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.94, i64 0, i64 0
  %27 = call i32 @regex_replace(i32 %24, i32 %25, i32 %26)
  %28 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.95, i64 0, i64 0
  %29 = call i32 @assert_eq_string(i32 %27, i32 %28)
  ; Expression result: %29
  %30 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.91, i64 0, i64 0
  %31 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.93, i64 0, i64 0
  %32 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.96, i64 0, i64 0
  %33 = call i32 @regex_replace(i32 %30, i32 %31, i32 %32)
  %34 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.97, i64 0, i64 0
  %35 = call i32 @assert_eq_string(i32 %33, i32 %34)
  ; Expression result: %35
  ret i32 0
}

define i32 @test_string_edge_cases() {
entry:
  %0 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.98, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %3 = call i32 @string_trim(i32 %2)
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %5 = call i32 @assert_eq_string(i32 %3, i32 %4)
  ; Expression result: %5
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %7 = call i32 @string_to_upper(i32 %6)
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %9 = call i32 @assert_eq_string(i32 %7, i32 %8)
  ; Expression result: %9
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %11 = call i32 @string_reverse(i32 %10)
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %13 = call i32 @assert_eq_string(i32 %11, i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.36, i64 0, i64 0
  %15 = call i32 @string_to_upper(i32 %14)
  %16 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.99, i64 0, i64 0
  %17 = call i32 @assert_eq_string(i32 %15, i32 %16)
  ; Expression result: %17
  %18 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.51, i64 0, i64 0
  %19 = call i32 @string_reverse(i32 %18)
  %20 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.51, i64 0, i64 0
  %21 = call i32 @assert_eq_string(i32 %19, i32 %20)
  ; Expression result: %21
  %22 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.100, i64 0, i64 0
  %23 = call i32 @string_len(i32 %22)
  %24 = call i32 @assert_eq_int(i32 %23, i32 1)
  ; Expression result: %24
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %25
  ; Expression result: 1
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %26
  %27 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.101, i64 0, i64 0
  %28 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.102, i64 0, i64 0
  %29 = call i32 @string_contains(i32 %27, i32 %28)
  %30 = call i32 @assert_true(i32 %29)
  ; Expression result: %30
  ret i32 0
}

define i32 @run_all_string_tests() {
entry:
  %0 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.103, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.104, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = call i32 @test_string_length()
  ; Expression result: %6
  %7 = call i32 @test_string_case_conversion()
  ; Expression result: %7
  %8 = call i32 @test_string_trimming()
  ; Expression result: %8
  %9 = call i32 @test_string_search()
  ; Expression result: %9
  %10 = call i32 @test_string_indexing()
  ; Expression result: %10
  %11 = call i32 @test_string_slicing()
  ; Expression result: %11
  %12 = call i32 @test_string_splitting()
  ; Expression result: %12
  %13 = call i32 @test_string_replacement()
  ; Expression result: %13
  %14 = call i32 @test_string_padding()
  ; Expression result: %14
  %15 = call i32 @test_string_validation()
  ; Expression result: %15
  %16 = call i32 @test_string_conversion()
  ; Expression result: %16
  %17 = call i32 @test_string_utilities()
  ; Expression result: %17
  %18 = call i32 @test_string_distance()
  ; Expression result: %18
  %19 = call i32 @test_string_regex()
  ; Expression result: %19
  %20 = call i32 @test_string_edge_cases()
  ; Expression result: %20
  %21 = call i32 @print_test_summary()
  ; Expression result: %21
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %22
  %23 = call i32 @run_all_tests()
  ; Expression result: %23
  ret i32 0
}



; String constants
@.str.98 = private unnamed_addr constant [18 x i8] c"String Edge Cases\00", align 1
@.str.67 = private unnamed_addr constant [7 x i8] c"ABC123\00", align 1
@.str.39 = private unnamed_addr constant [2 x i8] c"2\00", align 1
@.str.61 = private unnamed_addr constant [28 x i8] c"String Validation Functions\00", align 1
@.str.75 = private unnamed_addr constant [5 x i8] c"true\00", align 1
@.str.7 = private unnamed_addr constant [6 x i8] c"MIXED\00", align 1
@.str.23 = private unnamed_addr constant [4 x i8] c"xyz\00", align 1
@.str.70 = private unnamed_addr constant [4 x i8] c"   \00", align 1
@.str.14 = private unnamed_addr constant [5 x i8] c"test\00", align 1
@.str.26 = private unnamed_addr constant [18 x i8] c"hello hello hello\00", align 1
@.str.43 = private unnamed_addr constant [6 x i8] c"line2\00", align 1
@.str.27 = private unnamed_addr constant [2 x i8] c"l\00", align 1
@.str.62 = private unnamed_addr constant [4 x i8] c"123\00", align 1
@.str.64 = private unnamed_addr constant [5 x i8] c"-123\00", align 1
@.str.77 = private unnamed_addr constant [6 x i8] c"false\00", align 1
@.str.83 = private unnamed_addr constant [17 x i8] c"hello world test\00", align 1
@.str.102 = private unnamed_addr constant [5 x i8] c"🌍\00", align 1
@.str.11 = private unnamed_addr constant [26 x i8] c"String Trimming Functions\00", align 1
@.str.85 = private unnamed_addr constant [15 x i8] c"helloworldtest\00", align 1
@.str.103 = private unnamed_addr constant [41 x i8] c"📝 Running CURSED String Library Tests\00", align 1
@.str.32 = private unnamed_addr constant [27 x i8] c"String Splitting Functions\00", align 1
@.str.89 = private unnamed_addr constant [23 x i8] c"String Regex Functions\00", align 1
@.str.10 = private unnamed_addr constant [6 x i8] c"Hello\00", align 1
@.str.44 = private unnamed_addr constant [6 x i8] c"line3\00", align 1
@.str.42 = private unnamed_addr constant [6 x i8] c"line1\00", align 1
@.str.52 = private unnamed_addr constant [12 x i8] c"hexxo worxd\00", align 1
@.str.96 = private unnamed_addr constant [4 x i8] c"YYY\00", align 1
@.str.71 = private unnamed_addr constant [4 x i8] c"\09\0A\0D\00", align 1
@.str.15 = private unnamed_addr constant [10 x i8] c"no-spaces\00", align 1
@.str.18 = private unnamed_addr constant [8 x i8] c"  hello\00", align 1
@.str.78 = private unnamed_addr constant [4 x i8] c"cap\00", align 1
@.str.4 = private unnamed_addr constant [23 x i8] c"String Case Conversion\00", align 1
@.str.3 = private unnamed_addr constant [7 x i8] c"CURSED\00", align 1
@.str.50 = private unnamed_addr constant [6 x i8] c"hi hi\00", align 1
@.str.80 = private unnamed_addr constant [25 x i8] c"String Utility Functions\00", align 1
@.str.87 = private unnamed_addr constant [6 x i8] c"hallo\00", align 1
@.str.46 = private unnamed_addr constant [29 x i8] c"String Replacement Functions\00", align 1
@.str.20 = private unnamed_addr constant [24 x i8] c"String Search Functions\00", align 1
@.str.81 = private unnamed_addr constant [6 x i8] c"olleh\00", align 1
@.str.51 = private unnamed_addr constant [2 x i8] c"x\00", align 1
@.str.91 = private unnamed_addr constant [7 x i8] c"[a-z]+\00", align 1
@.str.12 = private unnamed_addr constant [10 x i8] c"  hello  \00", align 1
@.str.25 = private unnamed_addr constant [12 x i8] c"hello hello\00", align 1
@.str.97 = private unnamed_addr constant [10 x i8] c"123YYY456\00", align 1
@.str.101 = private unnamed_addr constant [11 x i8] c"Hello 🌍\00", align 1
@.str.104 = private unnamed_addr constant [39 x i8] c"======================================\00", align 1
@.str.49 = private unnamed_addr constant [4 x i8] c"abc\00", align 1
@.str.45 = private unnamed_addr constant [20 x i8] c"hello   world\09\0Atest\00", align 1
@.str.22 = private unnamed_addr constant [6 x i8] c"world\00", align 1
@.str.73 = private unnamed_addr constant [8 x i8] c"-456.78\00", align 1
@.str.74 = private unnamed_addr constant [4 x i8] c"0.0\00", align 1
@.str.76 = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.str.36 = private unnamed_addr constant [2 x i8] c"a\00", align 1
@.str.38 = private unnamed_addr constant [2 x i8] c"b\00", align 1
@.str.33 = private unnamed_addr constant [6 x i8] c"a,b,c\00", align 1
@.str.19 = private unnamed_addr constant [7 x i8] c"test\0D\0A\00", align 1
@.str.35 = private unnamed_addr constant [2 x i8] c"0\00", align 1
@.str.53 = private unnamed_addr constant [10 x i8] c"abcabcabc\00", align 1
@.str.21 = private unnamed_addr constant [12 x i8] c"hello world\00", align 1
@.str.24 = private unnamed_addr constant [26 x i8] c"String Indexing Functions\00", align 1
@.str.63 = private unnamed_addr constant [7 x i8] c"123.45\00", align 1
@.str.86 = private unnamed_addr constant [26 x i8] c"String Distance Functions\00", align 1
@.str.57 = private unnamed_addr constant [9 x i8] c"000hello\00", align 1
@.str.92 = private unnamed_addr constant [10 x i8] c"abc123def\00", align 1
@.str.55 = private unnamed_addr constant [2 x i8] c" \00", align 1
@.str.2 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.72 = private unnamed_addr constant [28 x i8] c"String Conversion Functions\00", align 1
@.str.69 = private unnamed_addr constant [8 x i8] c"123-456\00", align 1
@.str.94 = private unnamed_addr constant [4 x i8] c"XXX\00", align 1
@.str.58 = private unnamed_addr constant [11 x i8] c"hello     \00", align 1
@.str.5 = private unnamed_addr constant [6 x i8] c"HELLO\00", align 1
@.str.28 = private unnamed_addr constant [25 x i8] c"String Slicing Functions\00", align 1
@.str.13 = private unnamed_addr constant [9 x i8] c"\09\0Atest\0D\0A\00", align 1
@.str.16 = private unnamed_addr constant [8 x i8] c"hello  \00", align 1
@.str.82 = private unnamed_addr constant [4 x i8] c"cba\00", align 1
@.str.88 = private unnamed_addr constant [4 x i8] c"1.0\00", align 1
@.str.9 = private unnamed_addr constant [6 x i8] c"mixed\00", align 1
@.str.0 = private unnamed_addr constant [24 x i8] c"String Length Functions\00", align 1
@.str.66 = private unnamed_addr constant [9 x i8] c"hello123\00", align 1
@.str.8 = private unnamed_addr constant [7 x i8] c"cursed\00", align 1
@.str.30 = private unnamed_addr constant [2 x i8] c"h\00", align 1
@.str.31 = private unnamed_addr constant [2 x i8] c"o\00", align 1
@.str.90 = private unnamed_addr constant [4 x i8] c"\\d+\00", align 1
@.str.95 = private unnamed_addr constant [10 x i8] c"abcXXXdef\00", align 1
@.str.99 = private unnamed_addr constant [2 x i8] c"A\00", align 1
@.str.60 = private unnamed_addr constant [7 x i8] c"xxhixx\00", align 1
@.str.79 = private unnamed_addr constant [5 x i8] c"-456\00", align 1
@.str.29 = private unnamed_addr constant [7 x i8] c"llo wo\00", align 1
@.str.41 = private unnamed_addr constant [18 x i8] c"line1\0Aline2\0Aline3\00", align 1
@.str.54 = private unnamed_addr constant [25 x i8] c"String Padding Functions\00", align 1
@.str.84 = private unnamed_addr constant [17 x i8] c"hello,world,test\00", align 1
@.str.34 = private unnamed_addr constant [2 x i8] c",\00", align 1
@.str.68 = private unnamed_addr constant [7 x i8] c"hello!\00", align 1
@.str.48 = private unnamed_addr constant [9 x i8] c"hi hello\00", align 1
@.str.93 = private unnamed_addr constant [10 x i8] c"123abc456\00", align 1
@.str.37 = private unnamed_addr constant [2 x i8] c"1\00", align 1
@.str.56 = private unnamed_addr constant [11 x i8] c"     hello\00", align 1
@.str.65 = private unnamed_addr constant [7 x i8] c"123abc\00", align 1
@.str.17 = private unnamed_addr constant [7 x i8] c"\09\0Atest\00", align 1
@.str.47 = private unnamed_addr constant [3 x i8] c"hi\00", align 1
@.str.6 = private unnamed_addr constant [6 x i8] c"MiXeD\00", align 1
@.str.100 = private unnamed_addr constant [2 x i8] c"y\00", align 1
@.str.1 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
@.str.40 = private unnamed_addr constant [2 x i8] c"c\00", align 1
@.str.59 = private unnamed_addr constant [9 x i8] c"hello000\00", align 1
define i32 @main() {
  %0 = call i32 @run_all_string_tests()
  ret i32 0
}
