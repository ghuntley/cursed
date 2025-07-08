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
define i32 @test_basic_output() {
entry:
  %0 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  %3 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %3
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %4
  %5 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.3, i64 0, i64 0
  %6 = call i32 @test_pass(i32 %5)
  ; Expression result: %6
  %7 = call i32 @spill_int(i32 42)
  ; Expression result: %7
  %8 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.4, i64 0, i64 0
  %9 = call i32 @test_pass(i32 %8)
  ; Expression result: %9
  %10 = call i32 @spill_float(i32 3.14159)
  ; Expression result: %10
  %11 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.5, i64 0, i64 0
  %12 = call i32 @test_pass(i32 %11)
  ; Expression result: %12
  %13 = call i32 @spill_bool(i32 1)
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %15 = call i32 @spill_bool(i32 %14)
  ; Expression result: %15
  %16 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.6, i64 0, i64 0
  %17 = call i32 @test_pass(i32 %16)
  ; Expression result: %17
  %18 = call i32 @spill_char(i32 88)
  ; Expression result: %18
  %19 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.7, i64 0, i64 0
  %20 = call i32 @test_pass(i32 %19)
  ; Expression result: %20
  ret i32 0
}

define i32 @test_println_functions() {
entry:
  %0 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.8, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.9, i64 0, i64 0
  %3 = call i32 @println(i32 %2)
  ; Expression result: %3
  %4 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.10, i64 0, i64 0
  %5 = call i32 @test_pass(i32 %4)
  ; Expression result: %5
  %6 = call i32 @println_int(i32 123)
  ; Expression result: %6
  %7 = call i32 @println_float(i32 2.718)
  ; Expression result: %7
  %8 = call i32 @println_bool(i32 1)
  ; Expression result: %8
  %9 = call i32 @println_char(i32 89)
  ; Expression result: %9
  %10 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.11, i64 0, i64 0
  %11 = call i32 @test_pass(i32 %10)
  ; Expression result: %11
  ret i32 0
}

define i32 @test_format_functions() {
entry:
  %0 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.12, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @format_int(i32 42)
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable int_str allocated
  %4 = load i32, i32* %3, align 4
  %5 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.13, i64 0, i64 0
  %6 = call i32 @assert_eq_string(i8* %4, i32 %5)
  ; Expression result: %6
  %7 = call i32 @format_float(i32 3.14)
  %8 = alloca i32, align 4
  store i32 %7, i32* %8, align 4
  ; Variable float_str allocated
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %9
  ; Expression result: 0
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %10
  %11 = call i32 @format_bool(i32 1)
  %12 = alloca i32, align 4
  store i32 %11, i32* %12, align 4
  ; Variable bool_str_true allocated
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %14 = call i32 @format_bool(i32 %13)
  %15 = alloca i32, align 4
  store i32 %14, i32* %15, align 4
  ; Variable bool_str_false allocated
  %16 = load i32, i32* %12, align 4
  %17 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.14, i64 0, i64 0
  %18 = call i32 @assert_eq_string(i8* %16, i32 %17)
  ; Expression result: %18
  %19 = load i32, i32* %15, align 4
  %20 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.15, i64 0, i64 0
  %21 = call i32 @assert_eq_string(i8* %19, i32 %20)
  ; Expression result: %21
  %22 = call i32 @format_char(i32 90)
  %23 = alloca i32, align 4
  store i32 %22, i32* %23, align 4
  ; Variable char_str allocated
  %24 = load i32, i32* %23, align 4
  %25 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.16, i64 0, i64 0
  %26 = call i32 @assert_eq_string(i8* %24, i32 %25)
  ; Expression result: %26
  %27 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.17, i64 0, i64 0
  %28 = call i32 @test_pass(i32 %27)
  ; Expression result: %28
  ret i32 0
}

define i32 @test_sprintf_function() {
entry:
  %0 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.18, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.19, i64 0, i64 0
  %3 = alloca [0x i32], align 4
  %4 = call i32 @sprintf(i32 %2, i32 %3)
  %5 = alloca i32, align 4
  store i32 %4, i32* %5, align 4
  ; Variable result allocated
  %6 = load i32, i32* %5, align 4
  %7 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.20, i64 0, i64 0
  %8 = call i32 @assert_eq_string(i32 %6, i32 %7)
  ; Expression result: %8
  %9 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.21, i64 0, i64 0
  %10 = alloca [0x i32], align 4
  %11 = call i32 @sprintf(i32 %9, i32 %10)
  %12 = alloca i32, align 4
  store i32 %11, i32* %12, align 4
  ; Variable result2 allocated
  %13 = load i32, i32* %12, align 4
  %14 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.22, i64 0, i64 0
  %15 = call i32 @assert_eq_string(i32 %13, i32 %14)
  ; Expression result: %15
  %16 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.23, i64 0, i64 0
  %17 = alloca [0x i32], align 4
  %18 = call i32 @sprintf(i32 %16, i32 %17)
  %19 = alloca i32, align 4
  store i32 %18, i32* %19, align 4
  ; Variable result3 allocated
  %20 = load i32, i32* %19, align 4
  %21 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.24, i64 0, i64 0
  %22 = call i32 @assert_eq_string(i32 %20, i32 %21)
  ; Expression result: %22
  %23 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.25, i64 0, i64 0
  %24 = call i32 @test_pass(i32 %23)
  ; Expression result: %24
  ret i32 0
}

define i32 @test_printf_functions() {
entry:
  %0 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.26, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.27, i64 0, i64 0
  %3 = alloca [0x i32], align 4
  %4 = call i32 @printf(i32 %2, i32 %3)
  ; Expression result: %4
  %5 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.28, i64 0, i64 0
  %6 = call i32 @test_pass(i32 %5)
  ; Expression result: %6
  %7 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.29, i64 0, i64 0
  %8 = alloca [0x i32], align 4
  %9 = call i32 @printfln(i32 %7, i32 %8)
  ; Expression result: %9
  %10 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.30, i64 0, i64 0
  %11 = call i32 @test_pass(i32 %10)
  ; Expression result: %11
  ret i32 0
}

define i32 @test_type_safe_formatting() {
entry:
  %0 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.31, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.32, i64 0, i64 0
  %3 = call i32 @format_with_type(i32 42, i32 %2)
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable int_result allocated
  %5 = load i32, i32* %4, align 4
  %6 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.13, i64 0, i64 0
  %7 = call i32 @assert_eq_string(i32 %5, i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.33, i64 0, i64 0
  %9 = call i32 @format_with_type(i32 123, i32 %8)
  %10 = alloca i32, align 4
  store i32 %9, i32* %10, align 4
  ; Variable float_result allocated
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  ; Expression result: 0
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %12
  %13 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.34, i64 0, i64 0
  %14 = call i32 @format_with_type(i32 1, i32 %13)
  %15 = alloca i32, align 4
  store i32 %14, i32* %15, align 4
  ; Variable bool_result allocated
  %16 = load i32, i32* %15, align 4
  %17 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.14, i64 0, i64 0
  %18 = call i32 @assert_eq_string(i32 %16, i32 %17)
  ; Expression result: %18
  %19 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.35, i64 0, i64 0
  %20 = call i32 @test_pass(i32 %19)
  ; Expression result: %20
  ret i32 0
}

define i32 @test_debug_functions() {
entry:
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.36, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.37, i64 0, i64 0
  %3 = call i32 @debug_print(i32 %2)
  ; Expression result: %3
  %4 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.38, i64 0, i64 0
  %5 = call i32 @test_pass(i32 %4)
  ; Expression result: %5
  %6 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.39, i64 0, i64 0
  %7 = call i32 @debug_print_int(i32 %6, i32 100)
  ; Expression result: %7
  %8 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.40, i64 0, i64 0
  %9 = call i32 @debug_print_float(i32 %8, i32 3.14159)
  ; Expression result: %9
  %10 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.41, i64 0, i64 0
  %11 = call i32 @debug_print_bool(i32 %10, i32 1)
  ; Expression result: %11
  %12 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.42, i64 0, i64 0
  %13 = call i32 @test_pass(i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.43, i64 0, i64 0
  %15 = call i32 @info_print(i32 %14)
  ; Expression result: %15
  %16 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.44, i64 0, i64 0
  %17 = call i32 @error_print(i32 %16)
  ; Expression result: %17
  %18 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.45, i64 0, i64 0
  %19 = call i32 @warning_print(i32 %18)
  ; Expression result: %19
  %20 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.46, i64 0, i64 0
  %21 = call i32 @test_pass(i32 %20)
  ; Expression result: %21
  ret i32 0
}

define i32 @test_utility_functions() {
entry:
  %0 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.47, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @repeat_char(i32 42, i32 5)
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable repeated allocated
  %4 = load i32, i32* %3, align 4
  %5 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.48, i64 0, i64 0
  %6 = call i32 @assert_eq_string(i32 %4, i32 %5)
  ; Expression result: %6
  %7 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.49, i64 0, i64 0
  %8 = call i32 @pad_left(i32 %7, i32 8, i32 48)
  %9 = alloca i32, align 4
  store i32 %8, i32* %9, align 4
  ; Variable padded_left allocated
  %10 = load i32, i32* %9, align 4
  %11 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.50, i64 0, i64 0
  %12 = call i32 @assert_eq_string(i32 %10, i32 %11)
  ; Expression result: %12
  %13 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.49, i64 0, i64 0
  %14 = call i32 @pad_right(i32 %13, i32 8, i32 45)
  %15 = alloca i32, align 4
  store i32 %14, i32* %15, align 4
  ; Variable padded_right allocated
  %16 = load i32, i32* %15, align 4
  %17 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.51, i64 0, i64 0
  %18 = call i32 @assert_eq_string(i32 %16, i32 %17)
  ; Expression result: %18
  %19 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.52, i64 0, i64 0
  %20 = call i32 @center_text(i32 %19, i32 6, i32 32)
  %21 = alloca i32, align 4
  store i32 %20, i32* %21, align 4
  ; Variable centered allocated
  %22 = load i32, i32* %21, align 4
  %23 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.53, i64 0, i64 0
  %24 = call i32 @assert_eq_string(i32 %22, i32 %23)
  ; Expression result: %24
  %25 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.54, i64 0, i64 0
  %26 = call i32 @test_pass(i32 %25)
  ; Expression result: %26
  ret i32 0
}

define i32 @test_formatted_output() {
entry:
  %0 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.55, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @print_separator(i32 20, i32 45)
  ; Expression result: %2
  %3 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.56, i64 0, i64 0
  %4 = call i32 @test_pass(i32 %3)
  ; Expression result: %4
  %5 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.57, i64 0, i64 0
  %6 = call i32 @print_header(i32 %5, i32 30)
  ; Expression result: %6
  %7 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.58, i64 0, i64 0
  %8 = call i32 @test_pass(i32 %7)
  ; Expression result: %8
  %9 = add i32 0, 0 ; literal placeholder
  %10 = alloca i8*, align 4
  store i8* %9, i8** %10, align 4
  ; Variable columns allocated
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %12
  %13 = alloca [0x i32], align 4
  ; Expression result: %13
  %14 = load i8*, i8** %10, align 4
  %15 = call i32 @print_row(i32 %14, i32 30)
  ; Expression result: %15
  %16 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.59, i64 0, i64 0
  %17 = call i32 @test_pass(i32 %16)
  ; Expression result: %17
  ret i32 0
}

define i32 @test_number_formatting() {
entry:
  %0 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.60, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @format_int_padded(i32 42, i32 5)
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable padded_int allocated
  %4 = load i32, i32* %3, align 4
  %5 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.61, i64 0, i64 0
  %6 = call i32 @assert_eq_string(i32 %4, i32 %5)
  ; Expression result: %6
  %7 = call i32 @format_float_precision(i32 3.14159, i32 2)
  %8 = alloca i32, align 4
  store i32 %7, i32* %8, align 4
  ; Variable float_precision allocated
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %9
  ; Expression result: 0
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %10
  %11 = call i32 @format_percentage(i32 0.75)
  %12 = alloca i32, align 4
  store i32 %11, i32* %12, align 4
  ; Variable percentage allocated
  %13 = load i32, i32* %12, align 4
  %14 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.62, i64 0, i64 0
  %15 = call i32 @string_contains(i32 %13, i32 %14)
  %16 = call i32 @assert_true(i32 %15)
  ; Expression result: %16
  %17 = load i32, i32* %12, align 4
  %18 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.63, i64 0, i64 0
  %19 = call i32 @string_contains(i32 %17, i32 %18)
  %20 = call i32 @assert_true(i32 %19)
  ; Expression result: %20
  %21 = getelementptr inbounds [43 x i8], [43 x i8]* @.str.64, i64 0, i64 0
  %22 = call i32 @test_pass(i32 %21)
  ; Expression result: %22
  ret i32 0
}

define i32 @test_color_functions() {
entry:
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.65, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.66, i64 0, i64 0
  %3 = call i32 @color_red(i32 %2)
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable red_text allocated
  %5 = load i32, i32* %4, align 4
  %6 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.66, i64 0, i64 0
  %7 = call i32 @string_contains(i8* %5, i32 %6)
  %8 = call i32 @assert_true(i32 %7)
  ; Expression result: %8
  %9 = load i32, i32* %4, align 4
  %10 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.67, i64 0, i64 0
  %11 = call i32 @string_contains(i8* %9, i32 %10)
  %12 = call i32 @assert_true(i32 %11)
  ; Expression result: %12
  %13 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.68, i64 0, i64 0
  %14 = call i32 @color_green(i32 %13)
  %15 = alloca i32, align 4
  store i32 %14, i32* %15, align 4
  ; Variable green_text allocated
  %16 = load i32, i32* %15, align 4
  %17 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.68, i64 0, i64 0
  %18 = call i32 @string_contains(i8* %16, i32 %17)
  %19 = call i32 @assert_true(i32 %18)
  ; Expression result: %19
  %20 = load i32, i32* %15, align 4
  %21 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.69, i64 0, i64 0
  %22 = call i32 @string_contains(i8* %20, i32 %21)
  %23 = call i32 @assert_true(i32 %22)
  ; Expression result: %23
  %24 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.70, i64 0, i64 0
  %25 = call i32 @color_blue(i32 %24)
  %26 = alloca i32, align 4
  store i32 %25, i32* %26, align 4
  ; Variable blue_text allocated
  %27 = load i32, i32* %26, align 4
  %28 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.70, i64 0, i64 0
  %29 = call i32 @string_contains(i8* %27, i32 %28)
  %30 = call i32 @assert_true(i32 %29)
  ; Expression result: %30
  %31 = load i32, i32* %26, align 4
  %32 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.71, i64 0, i64 0
  %33 = call i32 @string_contains(i8* %31, i32 %32)
  %34 = call i32 @assert_true(i32 %33)
  ; Expression result: %34
  %35 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.72, i64 0, i64 0
  %36 = call i32 @test_pass(i32 %35)
  ; Expression result: %36
  ret i32 0
}

define i32 @test_message_functions() {
entry:
  %0 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.73, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.74, i64 0, i64 0
  %3 = call i32 @success_print(i32 %2)
  ; Expression result: %3
  %4 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.75, i64 0, i64 0
  %5 = call i32 @error_print_colored(i32 %4)
  ; Expression result: %5
  %6 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.76, i64 0, i64 0
  %7 = call i32 @warning_print_colored(i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.77, i64 0, i64 0
  %9 = call i32 @info_print_colored(i32 %8)
  ; Expression result: %9
  %10 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.78, i64 0, i64 0
  %11 = call i32 @test_pass(i32 %10)
  ; Expression result: %11
  ret i32 0
}

define i32 @test_integration_formatting() {
entry:
  %0 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.79, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.80, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable name allocated
  %4 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.81, i64 0, i64 0
  %5 = alloca i8*, align 4
  store i8* %4, i8** %5, align 4
  ; Variable version allocated
  %6 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.82, i64 0, i64 0
  %7 = alloca i8*, align 4
  store i8* %6, i8** %7, align 4
  ; Variable status allocated
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %9 = call i32 @println(i32 %8)
  ; Expression result: %9
  %10 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.83, i64 0, i64 0
  %11 = call i32 @print_header(i32 %10, i32 40)
  ; Expression result: %11
  %12 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.84, i64 0, i64 0
  %13 = alloca [0x i32], align 4
  %14 = call i32 @sprintf(i32 %12, i32 %13)
  %15 = alloca i32, align 4
  store i32 %14, i32* %15, align 4
  ; Variable name_line allocated
  %16 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.85, i64 0, i64 0
  %17 = alloca [0x i32], align 4
  %18 = call i32 @sprintf(i32 %16, i32 %17)
  %19 = alloca i32, align 4
  store i32 %18, i32* %19, align 4
  ; Variable version_line allocated
  %20 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.86, i64 0, i64 0
  %21 = alloca [0x i32], align 4
  %22 = call i32 @sprintf(i32 %20, i32 %21)
  %23 = alloca i32, align 4
  store i32 %22, i32* %23, align 4
  ; Variable status_line allocated
  %24 = load i32, i32* %15, align 4
  %25 = call i32 @println(i8* %24)
  ; Expression result: %25
  %26 = load i32, i32* %19, align 4
  %27 = call i32 @println(i32 %26)
  ; Expression result: %27
  %28 = load i32, i32* %23, align 4
  %29 = call i32 @println(i32 %28)
  ; Expression result: %29
  %30 = call i32 @print_separator(i32 40, i32 61)
  ; Expression result: %30
  %31 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.87, i64 0, i64 0
  %32 = call i32 @test_pass(i32 %31)
  ; Expression result: %32
  ret i32 0
}

define i32 @test_mixed_type_output() {
entry:
  %0 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.88, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  %3 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.89, i64 0, i64 0
  ; Expression result: %3
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %4
  %5 = call i32 @spill_int(i32 42)
  ; Expression result: %5
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %6
  %7 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.90, i64 0, i64 0
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = call i32 @spill_float(i32 3.14)
  ; Expression result: %9
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %10
  %11 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.91, i64 0, i64 0
  ; Expression result: %11
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %12
  %13 = call i32 @spill_bool(i32 1)
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  %15 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.92, i64 0, i64 0
  ; Expression result: %15
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %16
  %17 = call i32 @spill_char(i32 88)
  ; Expression result: %17
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %19 = call i32 @println(i32 %18)
  ; Expression result: %19
  %20 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.93, i64 0, i64 0
  %21 = call i32 @test_pass(i32 %20)
  ; Expression result: %21
  ret i32 0
}

define i32 @run_all_tests() {
entry:
  %0 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.94, i64 0, i64 0
  %1 = call i32 @println(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.95, i64 0, i64 0
  %3 = call i32 @println(i32 %2)
  ; Expression result: %3
  %4 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.94, i64 0, i64 0
  %5 = call i32 @println(i32 %4)
  ; Expression result: %5
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %7 = call i32 @println(i32 %6)
  ; Expression result: %7
  %8 = call i32 @test_basic_output()
  ; Expression result: %8
  %9 = call i32 @test_println_functions()
  ; Expression result: %9
  %10 = call i32 @test_format_functions()
  ; Expression result: %10
  %11 = call i32 @test_sprintf_function()
  ; Expression result: %11
  %12 = call i32 @test_printf_functions()
  ; Expression result: %12
  %13 = call i32 @test_type_safe_formatting()
  ; Expression result: %13
  %14 = call i32 @test_debug_functions()
  ; Expression result: %14
  %15 = call i32 @test_utility_functions()
  ; Expression result: %15
  %16 = call i32 @test_formatted_output()
  ; Expression result: %16
  %17 = call i32 @test_number_formatting()
  ; Expression result: %17
  %18 = call i32 @test_color_functions()
  ; Expression result: %18
  %19 = call i32 @test_message_functions()
  ; Expression result: %19
  %20 = call i32 @test_integration_formatting()
  ; Expression result: %20
  %21 = call i32 @test_mixed_type_output()
  ; Expression result: %21
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %23 = call i32 @println(i32 %22)
  ; Expression result: %23
  %24 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.94, i64 0, i64 0
  %25 = call i32 @println(i32 %24)
  ; Expression result: %25
  %26 = call i32 @print_test_summary()
  ; Expression result: %26
  %27 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.94, i64 0, i64 0
  %28 = call i32 @println(i32 %27)
  ; Expression result: %28
  ret i32 0
}



; String constants
@.str.6 = private unnamed_addr constant [26 x i8] c"spill_bool function works\00", align 1
@.str.22 = private unnamed_addr constant [17 x i8] c"The answer is 42\00", align 1
@.str.32 = private unnamed_addr constant [4 x i8] c"int\00", align 1
@.str.74 = private unnamed_addr constant [33 x i8] c"Operation completed successfully\00", align 1
@.str.14 = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.str.16 = private unnamed_addr constant [2 x i8] c"Z\00", align 1
@.str.15 = private unnamed_addr constant [4 x i8] c"cap\00", align 1
@.str.8 = private unnamed_addr constant [18 x i8] c"Println functions\00", align 1
@.str.56 = private unnamed_addr constant [31 x i8] c"print_separator function works\00", align 1
@.str.35 = private unnamed_addr constant [37 x i8] c"Type-safe formatting works correctly\00", align 1
@.str.19 = private unnamed_addr constant [11 x i8] c"Hello, {}!\00", align 1
@.str.66 = private unnamed_addr constant [9 x i8] c"Red text\00", align 1
@.str.69 = private unnamed_addr constant [8 x i8] c" 33[32m\00", align 1
@.str.0 = private unnamed_addr constant [21 x i8] c"Basic spill function\00", align 1
@.str.57 = private unnamed_addr constant [12 x i8] c"Test Header\00", align 1
@.str.70 = private unnamed_addr constant [10 x i8] c"Blue text\00", align 1
@.str.90 = private unnamed_addr constant [9 x i8] c" Float: \00", align 1
@.str.92 = private unnamed_addr constant [13 x i8] c" Character: \00", align 1
@.str.79 = private unnamed_addr constant [23 x i8] c"Integration formatting\00", align 1
@.str.84 = private unnamed_addr constant [9 x i8] c"Name: {}\00", align 1
@.str.2 = private unnamed_addr constant [15 x i8] c"Hello, CURSED!\00", align 1
@.str.9 = private unnamed_addr constant [15 x i8] c"This is a line\00", align 1
@.str.75 = private unnamed_addr constant [17 x i8] c"This is an error\00", align 1
@.str.85 = private unnamed_addr constant [12 x i8] c"Version: {}\00", align 1
@.str.94 = private unnamed_addr constant [41 x i8] c"========================================\00", align 1
@.str.81 = private unnamed_addr constant [6 x i8] c"1.0.0\00", align 1
@.str.87 = private unnamed_addr constant [39 x i8] c"Integration formatting works correctly\00", align 1
@.str.65 = private unnamed_addr constant [16 x i8] c"Color functions\00", align 1
@.str.89 = private unnamed_addr constant [10 x i8] c"Integer: \00", align 1
@.str.17 = private unnamed_addr constant [36 x i8] c"All format functions work correctly\00", align 1
@.str.49 = private unnamed_addr constant [5 x i8] c"test\00", align 1
@.str.82 = private unnamed_addr constant [6 x i8] c"Ready\00", align 1
@.str.67 = private unnamed_addr constant [8 x i8] c" 33[31m\00", align 1
@.str.83 = private unnamed_addr constant [14 x i8] c"System Status\00", align 1
@.str.72 = private unnamed_addr constant [31 x i8] c"Color functions work correctly\00", align 1
@.str.47 = private unnamed_addr constant [18 x i8] c"Utility functions\00", align 1
@.str.40 = private unnamed_addr constant [3 x i8] c"pi\00", align 1
@.str.41 = private unnamed_addr constant [5 x i8] c"flag\00", align 1
@.str.1 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.91 = private unnamed_addr constant [11 x i8] c" Boolean: \00", align 1
@.str.43 = private unnamed_addr constant [20 x i8] c"Information message\00", align 1
@.str.46 = private unnamed_addr constant [33 x i8] c"All message level functions work\00", align 1
@.str.54 = private unnamed_addr constant [37 x i8] c"All utility functions work correctly\00", align 1
@.str.24 = private unnamed_addr constant [10 x i8] c"2 + 3 = 5\00", align 1
@.str.76 = private unnamed_addr constant [18 x i8] c"This is a warning\00", align 1
@.str.63 = private unnamed_addr constant [2 x i8] c"%\00", align 1
@.str.77 = private unnamed_addr constant [20 x i8] c"This is information\00", align 1
@.str.37 = private unnamed_addr constant [24 x i8] c"This is a debug message\00", align 1
@.str.68 = private unnamed_addr constant [11 x i8] c"Green text\00", align 1
@.str.78 = private unnamed_addr constant [37 x i8] c"All message functions work correctly\00", align 1
@.str.27 = private unnamed_addr constant [22 x i8] c"Formatted output: {}\0A\00", align 1
@.str.88 = private unnamed_addr constant [18 x i8] c"Mixed type output\00", align 1
@.str.33 = private unnamed_addr constant [6 x i8] c"float\00", align 1
@.str.5 = private unnamed_addr constant [27 x i8] c"spill_float function works\00", align 1
@.str.42 = private unnamed_addr constant [31 x i8] c"All debug print functions work\00", align 1
@.str.4 = private unnamed_addr constant [25 x i8] c"spill_int function works\00", align 1
@.str.71 = private unnamed_addr constant [8 x i8] c" 33[34m\00", align 1
@.str.73 = private unnamed_addr constant [18 x i8] c"Message functions\00", align 1
@.str.13 = private unnamed_addr constant [3 x i8] c"42\00", align 1
@.str.29 = private unnamed_addr constant [19 x i8] c"Formatted line: {}\00", align 1
@.str.7 = private unnamed_addr constant [26 x i8] c"spill_char function works\00", align 1
@.str.26 = private unnamed_addr constant [17 x i8] c"printf functions\00", align 1
@.str.50 = private unnamed_addr constant [9 x i8] c"0000test\00", align 1
@.str.25 = private unnamed_addr constant [33 x i8] c"sprintf function works correctly\00", align 1
@.str.39 = private unnamed_addr constant [9 x i8] c"test_var\00", align 1
@.str.45 = private unnamed_addr constant [16 x i8] c"Warning message\00", align 1
@.str.51 = private unnamed_addr constant [9 x i8] c"test----\00", align 1
@.str.11 = private unnamed_addr constant [27 x i8] c"All println functions work\00", align 1
@.str.28 = private unnamed_addr constant [22 x i8] c"printf function works\00", align 1
@.str.30 = private unnamed_addr constant [24 x i8] c"printfln function works\00", align 1
@.str.55 = private unnamed_addr constant [17 x i8] c"Formatted output\00", align 1
@.str.44 = private unnamed_addr constant [14 x i8] c"Error message\00", align 1
@.str.60 = private unnamed_addr constant [18 x i8] c"Number formatting\00", align 1
@.str.62 = private unnamed_addr constant [3 x i8] c"75\00", align 1
@.str.3 = private unnamed_addr constant [21 x i8] c"spill function works\00", align 1
@.str.64 = private unnamed_addr constant [43 x i8] c"Number formatting functions work correctly\00", align 1
@.str.18 = private unnamed_addr constant [17 x i8] c"sprintf function\00", align 1
@.str.52 = private unnamed_addr constant [3 x i8] c"hi\00", align 1
@.str.59 = private unnamed_addr constant [25 x i8] c"print_row function works\00", align 1
@.str.36 = private unnamed_addr constant [16 x i8] c"Debug functions\00", align 1
@.str.61 = private unnamed_addr constant [6 x i8] c"00042\00", align 1
@.str.80 = private unnamed_addr constant [7 x i8] c"CURSED\00", align 1
@.str.53 = private unnamed_addr constant [7 x i8] c"  hi  \00", align 1
@.str.86 = private unnamed_addr constant [11 x i8] c"Status: {}\00", align 1
@.str.20 = private unnamed_addr constant [14 x i8] c"Hello, World!\00", align 1
@.str.95 = private unnamed_addr constant [28 x i8] c"         VIBEZ MODULE TESTS\00", align 1
@.str.93 = private unnamed_addr constant [34 x i8] c"Mixed type output works correctly\00", align 1
@.str.31 = private unnamed_addr constant [21 x i8] c"Type-safe formatting\00", align 1
@.str.48 = private unnamed_addr constant [6 x i8] c"*****\00", align 1
@.str.10 = private unnamed_addr constant [23 x i8] c"println function works\00", align 1
@.str.38 = private unnamed_addr constant [27 x i8] c"debug_print function works\00", align 1
@.str.12 = private unnamed_addr constant [17 x i8] c"Format functions\00", align 1
@.str.23 = private unnamed_addr constant [16 x i8] c"{0} + {1} = {2}\00", align 1
@.str.21 = private unnamed_addr constant [17 x i8] c"The answer is {}\00", align 1
@.str.58 = private unnamed_addr constant [28 x i8] c"print_header function works\00", align 1
@.str.34 = private unnamed_addr constant [5 x i8] c"bool\00", align 1
define i32 @main() {
  %0 = call i32 @run_all_tests()
  ret i32 0
}
