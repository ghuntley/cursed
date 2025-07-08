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
define i32 @test_integer_assertions() {
entry:
  %0 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @assert_eq_int(i32 42, i32 42)
  ; Expression result: %2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %3
  ; Expression result: 1
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %4
  ; Expression result: 2
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %6
  ; Expression result: 2
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %7
  ; Expression result: 10
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = call i32 @assert_ne_int(i32 42, i32 43)
  ; Expression result: %9
  %10 = call i32 @assert_ne_int(i32 1, i32 2)
  ; Expression result: %10
  %11 = call i32 @assert_greater_than(i32 5, i32 3)
  ; Expression result: %11
  %12 = call i32 @assert_greater_than(i32 100, i32 42)
  ; Expression result: %12
  %13 = call i32 @assert_less_than(i32 3, i32 5)
  ; Expression result: %13
  %14 = call i32 @assert_less_than(i32 42, i32 100)
  ; Expression result: %14
  %15 = call i32 @assert_in_range(i32 5, i32 1, i32 10)
  ; Expression result: %15
  %16 = call i32 @assert_in_range(i32 42, i32 40, i32 50)
  ; Expression result: %16
  ; Expression result: 5
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  ; Expression result: 5
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %18
  %19 = call i32 @test_end()
  ; Expression result: %19
  ret i32 0
}

define i32 @test_float_assertions() {
entry:
  %0 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.2, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @assert_eq_float(i32 3.14159, i32 3.14159)
  ; Expression result: %2
  %3 = call i32 @assert_eq_float(i32 1, i32 1)
  ; Expression result: %3
  %4 = call i32 @assert_eq_float(i32 0.3, i32 0.3)
  ; Expression result: %4
  %5 = call i32 @assert_eq_float_with_tolerance(i32 3.14, i32 3.141, i32 0.01)
  ; Expression result: %5
  %6 = call i32 @assert_eq_float_with_tolerance(i32 1.001, i32 1.002, i32 0.01)
  ; Expression result: %6
  %7 = call i32 @test_end()
  ; Expression result: %7
  ret i32 0
}

define i32 @test_string_assertions() {
entry:
  %0 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.3, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.4, i64 0, i64 0
  %3 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.4, i64 0, i64 0
  %4 = call i32 @assert_eq_string(i32 %2, i32 %3)
  ; Expression result: %4
  %5 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.5, i64 0, i64 0
  %6 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.5, i64 0, i64 0
  %7 = call i32 @assert_eq_string(i32 %5, i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %10 = call i32 @assert_eq_string(i32 %8, i32 %9)
  ; Expression result: %10
  %11 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.4, i64 0, i64 0
  %12 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.6, i64 0, i64 0
  %13 = call i32 @assert_ne_string(i32 %11, i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.5, i64 0, i64 0
  %15 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.7, i64 0, i64 0
  %16 = call i32 @assert_ne_string(i32 %14, i32 %15)
  ; Expression result: %16
  %17 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %18 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.6, i64 0, i64 0
  %19 = call i32 @assert_string_contains(i32 %17, i32 %18)
  ; Expression result: %19
  %20 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.9, i64 0, i64 0
  %21 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.5, i64 0, i64 0
  %22 = call i32 @assert_string_contains(i32 %20, i32 %21)
  ; Expression result: %22
  %23 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %24 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.4, i64 0, i64 0
  %25 = call i32 @assert_string_starts_with(i32 %23, i32 %24)
  ; Expression result: %25
  %26 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %27 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.6, i64 0, i64 0
  %28 = call i32 @assert_string_ends_with(i32 %26, i32 %27)
  ; Expression result: %28
  %29 = call i32 @test_end()
  ; Expression result: %29
  ret i32 0
}

define i32 @test_boolean_assertions() {
entry:
  %0 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.10, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @assert_true(i32 1)
  ; Expression result: %2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %4 = call i32 @assert_false(i32 %3)
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  ; Expression result: 3
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %7
  ; Expression result: 5
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = call i32 @assert_eq_bool(i32 1, i32 1)
  ; Expression result: %9
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %12 = call i32 @assert_eq_bool(i32 %10, i32 %11)
  ; Expression result: %12
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %13
  ; Expression result: 3
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  ; Expression result: 1
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %15
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %16
  ; Expression result: 5
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %19
  %20 = call i32 @test_end()
  ; Expression result: %20
  ret i32 0
}

define i32 @test_nil_assertions() {
entry:
  %0 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.11, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.12, i64 0, i64 0
  %3 = call i32 @assert_nil(i32 %2)
  ; Expression result: %3
  %4 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.13, i64 0, i64 0
  %5 = call i32 @assert_not_nil(i32 %4)
  ; Expression result: %5
  %6 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.4, i64 0, i64 0
  %7 = call i32 @assert_not_nil(i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.14, i64 0, i64 0
  %9 = call i32 @assert_not_nil(i32 %8)
  ; Expression result: %9
  %10 = call i32 @test_end()
  ; Expression result: %10
  ret i32 0
}

define i32 @test_test_suites() {
entry:
  %0 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.15, i64 0, i64 0
  %1 = call i32 @suite_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.16, i64 0, i64 0
  %3 = call i32 @test_start(i32 %2)
  ; Expression result: %3
  %4 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.15, i64 0, i64 0
  %5 = call i32 @assert_eq_string(i8* %current_suite_name, i32 %4)
  ; Expression result: %5
  %6 = call i32 @assert_greater_than(i32 %suite_count, i32 0)
  ; Expression result: %6
  %7 = call i32 @test_end()
  ; Expression result: %7
  %8 = call i32 @suite_end()
  ; Expression result: %8
  ret i32 0
}

define i32 @test_performance_testing() {
entry:
  %0 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.17, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @benchmark_start()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable start_time allocated
  %4 = alloca i32, align 4
  store i32 0, i32* %4, align 4
  ; Variable result allocated
  %6 = load i32, i32* %5, align 4
  %7 = icmp slt i32 %6, 1000
  %13 = add i32 1, 0 ; increment placeholder
  %5 = alloca i32, align 4
  store i32 0, i32* %5, align 4
  ; Short declaration: i := 0
  br label %label0
label0:
  br i1 %7, label %label1, label %label3
label1:
  %8 = load i32, i32* %4, align 4
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %9
  %10 = load i32, i32* %4, align 4
  %11 = load i32, i32* %5, align 4
  %12 = add i32 %10, %11
  ; Expression result: %12
  br label %label2
label2:
  br label %label0
label3:
  %14 = load i32, i32* %3, align 4
  %15 = call i32 @benchmark_end(i32 %14)
  ; Expression result: %15
  %16 = load i32, i32* %4, align 4
  %17 = call i32 @assert_eq_int(i32 %16, i32 499500)
  ; Expression result: %17
  %18 = call i32 @test_end()
  ; Expression result: %18
  ret i32 0
}

define i32 @test_mock_functions() {
entry:
  %0 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.18, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.19, i64 0, i64 0
  %3 = call i32 @create_mock(i32 %2)
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable mock allocated
  %5 = load i32, i32* %4, align 4
  %6 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.20, i64 0, i64 0
  %7 = call i32 @mock_return(i32 %5, i32 %6)
  ; Expression result: %7
  %8 = load i32, i32* %4, align 4
  ; Member access: %8.name
  %9 = getelementptr inbounds %struct.object, %struct.object* %8, i32 0, i32 0
  %10 = load i32, i32* %9, align 4
  %11 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.19, i64 0, i64 0
  %12 = call i32 @assert_eq_string(i32 %10, i32 %11)
  ; Expression result: %12
  %13 = load i32, i32* %4, align 4
  ; Member access: %13.return_value
  %14 = getelementptr inbounds %struct.object, %struct.object* %13, i32 0, i32 0
  %15 = load i32, i32* %14, align 4
  %16 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.20, i64 0, i64 0
  %17 = call i32 @assert_eq_string(i32 %15, i32 %16)
  ; Expression result: %17
  %18 = load i32, i32* %4, align 4
  ; Member access: %18.call_count
  %19 = getelementptr inbounds %struct.object, %struct.object* %18, i32 0, i32 0
  %20 = load i32, i32* %19, align 4
  %21 = call i32 @assert_eq_int(i32 %20, i32 0)
  ; Expression result: %21
  %22 = load i32, i32* %4, align 4
  ; Member access: %22.should_throw
  %23 = getelementptr inbounds %struct.object, %struct.object* %22, i32 0, i32 0
  %24 = load i32, i32* %23, align 4
  %25 = call i32 @assert_false(i32 %24)
  ; Expression result: %25
  %26 = load i32, i32* %4, align 4
  %27 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.21, i64 0, i64 0
  %28 = call i32 @mock_throw(i32 %26, i32 %27)
  ; Expression result: %28
  %29 = load i32, i32* %4, align 4
  ; Member access: %29.should_throw
  %30 = getelementptr inbounds %struct.object, %struct.object* %29, i32 0, i32 0
  %31 = load i32, i32* %30, align 4
  %32 = call i32 @assert_true(i32 %31)
  ; Expression result: %32
  %33 = load i32, i32* %4, align 4
  ; Member access: %33.throw_message
  %34 = getelementptr inbounds %struct.object, %struct.object* %33, i32 0, i32 0
  %35 = load i32, i32* %34, align 4
  %36 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.21, i64 0, i64 0
  %37 = call i32 @assert_eq_string(i32 %35, i32 %36)
  ; Expression result: %37
  %38 = call i32 @test_end()
  ; Expression result: %38
  ret i32 0
}

define i32 @test_error_handling() {
entry:
  %0 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.22, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.23, i64 0, i64 0
  %3 = call i32 @assert_throws(i32 %2)
  ; Expression result: %3
  %4 = call i32 @assert_no_throw()
  ; Expression result: %4
  %5 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.24, i64 0, i64 0
  %6 = call i32 @expect_panic(i32 %5)
  ; Expression result: %6
  %7 = call i32 @test_end()
  ; Expression result: %7
  ret i32 0
}

define i32 @test_array_assertions() {
entry:
  %0 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.25, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = alloca [5x i32], align 4
  %3 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.26, i64 0, i64 0
  %4 = getelementptr inbounds [5x i32], [5x i32]* %2, i64 0, i64 0
  store i32 %3, i32* %4, align 4
  %5 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.27, i64 0, i64 0
  %6 = getelementptr inbounds [5x i32], [5x i32]* %2, i64 0, i64 1
  store i32 %5, i32* %6, align 4
  %7 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.28, i64 0, i64 0
  %8 = getelementptr inbounds [5x i32], [5x i32]* %2, i64 0, i64 2
  store i32 %7, i32* %8, align 4
  %9 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.29, i64 0, i64 0
  %10 = getelementptr inbounds [5x i32], [5x i32]* %2, i64 0, i64 3
  store i32 %9, i32* %10, align 4
  %11 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.30, i64 0, i64 0
  %12 = getelementptr inbounds [5x i32], [5x i32]* %2, i64 0, i64 4
  store i32 %11, i32* %12, align 4
  %13 = alloca [5 x i32]*, align 4
  store [5 x i32]* %2, [5 x i32]** %13, align 4
  ; Variable test_array allocated
  %14 = alloca [5x i32], align 4
  %15 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.26, i64 0, i64 0
  %16 = getelementptr inbounds [5x i32], [5x i32]* %14, i64 0, i64 0
  store i32 %15, i32* %16, align 4
  %17 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.27, i64 0, i64 0
  %18 = getelementptr inbounds [5x i32], [5x i32]* %14, i64 0, i64 1
  store i32 %17, i32* %18, align 4
  %19 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.28, i64 0, i64 0
  %20 = getelementptr inbounds [5x i32], [5x i32]* %14, i64 0, i64 2
  store i32 %19, i32* %20, align 4
  %21 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.29, i64 0, i64 0
  %22 = getelementptr inbounds [5x i32], [5x i32]* %14, i64 0, i64 3
  store i32 %21, i32* %22, align 4
  %23 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.30, i64 0, i64 0
  %24 = getelementptr inbounds [5x i32], [5x i32]* %14, i64 0, i64 4
  store i32 %23, i32* %24, align 4
  %25 = alloca [5 x i32]*, align 4
  store [5 x i32]* %14, [5 x i32]** %25, align 4
  ; Variable expected_array allocated
  %26 = load [5 x i32]*, [5 x i32]** %13, align 4
  %27 = load [5 x i32]*, [5 x i32]** %25, align 4
  %28 = call i32 @assert_array_eq_int(i32 %26, i32 %27)
  ; Expression result: %28
  %29 = load [5 x i32]*, [5 x i32]** %13, align 4
  %30 = call i32 @assert_array_contains_int(i32 %29, i32 3)
  ; Expression result: %30
  %31 = load [5 x i32]*, [5 x i32]** %13, align 4
  %32 = call i32 @assert_array_not_contains_int(i32 %31, i32 10)
  ; Expression result: %32
  %33 = load [5 x i32]*, [5 x i32]** %13, align 4
  %34 = call i32 @assert_array_length(i32 %33, i32 5)
  ; Expression result: %34
  %35 = call i32 @test_end()
  ; Expression result: %35
  ret i32 0
}

define i32 @test_configuration() {
entry:
  %0 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.31, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @create_default_config()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable config allocated
  %4 = load i32, i32* %3, align 4
  ; Member access: %4.timeout
  %5 = getelementptr inbounds %struct.object, %struct.object* %4, i32 0, i32 0
  %6 = load i32, i32* %5, align 4
  %7 = call i32 @assert_eq_int(i32 %6, i32 5000)
  ; Expression result: %7
  %8 = load i32, i32* %3, align 4
  ; Member access: %8.verbose
  %9 = getelementptr inbounds %struct.object, %struct.object* %8, i32 0, i32 0
  %10 = load i32, i32* %9, align 4
  %11 = call i32 @assert_true(i32 %10)
  ; Expression result: %11
  %12 = load i32, i32* %3, align 4
  ; Member access: %12.fail_fast
  %13 = getelementptr inbounds %struct.object, %struct.object* %12, i32 0, i32 0
  %14 = load i32, i32* %13, align 4
  %15 = call i32 @assert_false(i32 %14)
  ; Expression result: %15
  %16 = load i32, i32* %3, align 4
  ; Member access: %16.parallel
  %17 = getelementptr inbounds %struct.object, %struct.object* %16, i32 0, i32 0
  %18 = load i32, i32* %17, align 4
  %19 = call i32 @assert_false(i32 %18)
  ; Expression result: %19
  %20 = load i32, i32* %3, align 4
  ; Member access: %20.test_dir
  %21 = getelementptr inbounds %struct.object, %struct.object* %20, i32 0, i32 0
  %22 = load i32, i32* %21, align 4
  %23 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.32, i64 0, i64 0
  %24 = call i32 @assert_eq_string(i32 %22, i32 %23)
  ; Expression result: %24
  %25 = load i32, i32* %3, align 4
  ; Member access: %25.pattern
  %26 = getelementptr inbounds %struct.object, %struct.object* %25, i32 0, i32 0
  %27 = load i32, i32* %26, align 4
  %28 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.33, i64 0, i64 0
  %29 = call i32 @assert_eq_string(i32 %27, i32 %28)
  ; Expression result: %29
  %30 = load i32, i32* %3, align 4
  ; Member access: %30.output_format
  %31 = getelementptr inbounds %struct.object, %struct.object* %30, i32 0, i32 0
  %32 = load i32, i32* %31, align 4
  %33 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.34, i64 0, i64 0
  %34 = call i32 @assert_eq_string(i32 %32, i32 %33)
  ; Expression result: %34
  %35 = load i32, i32* %3, align 4
  ; Member access: %35.coverage_enabled
  %36 = getelementptr inbounds %struct.object, %struct.object* %35, i32 0, i32 0
  %37 = load i32, i32* %36, align 4
  %38 = call i32 @assert_false(i32 %37)
  ; Expression result: %38
  %39 = load i32, i32* %3, align 4
  %40 = call i32 @set_test_config(i32 %39)
  ; Expression result: %40
  %41 = call i32 @test_end()
  ; Expression result: %41
  ret i32 0
}

define i32 @test_reporting_formats() {
entry:
  %0 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.35, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.36, i64 0, i64 0
  %3 = call i32 @puts(i8* %2)
  %4 = add i32 0, 0
  ; Expression result: %4
  %5 = call i32 @generate_json_report()
  ; Expression result: %5
  %6 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.37, i64 0, i64 0
  %7 = call i32 @puts(i8* %6)
  %8 = add i32 0, 0
  ; Expression result: %8
  %9 = call i32 @generate_xml_report()
  ; Expression result: %9
  %10 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.38, i64 0, i64 0
  %11 = call i32 @puts(i8* %10)
  %12 = add i32 0, 0
  ; Expression result: %12
  %13 = call i32 @generate_html_report()
  ; Expression result: %13
  %14 = call i32 @test_end()
  ; Expression result: %14
  ret i32 0
}

define i32 @test_filtering() {
entry:
  %0 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.39, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.40, i64 0, i64 0
  %3 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.33, i64 0, i64 0
  %4 = call i32 @should_run_test(i32 %2, i32 %3)
  %5 = call i32 @assert_true(i32 %4)
  ; Expression result: %5
  %6 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.41, i64 0, i64 0
  %7 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.42, i64 0, i64 0
  %8 = call i32 @should_run_test(i32 %6, i32 %7)
  %9 = call i32 @assert_true(i32 %8)
  ; Expression result: %9
  %10 = call i32 @test_end()
  ; Expression result: %10
  ret i32 0
}

define i32 @test_edge_cases() {
entry:
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.43, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @assert_eq_int(i32 0, i32 0)
  ; Expression result: %2
  ; Expression result: 1
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %4 = sub i32 %3, 1
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = call i32 @assert_eq_int(i32 2147483647, i32 2147483647)
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %9 = call i32 @assert_eq_string(i32 %7, i32 %8)
  ; Expression result: %9
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %11 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.44, i64 0, i64 0
  %12 = call i32 @assert_ne_string(i32 %10, i32 %11)
  ; Expression result: %12
  %13 = call i32 @assert_eq_float(i32 0, i32 0)
  ; Expression result: %13
  %14 = call i32 @assert_eq_float(i32 1, i32 1)
  ; Expression result: %14
  %15 = call i32 @test_end()
  ; Expression result: %15
  ret i32 0
}

define i32 @test_skipping() {
entry:
  %0 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.45, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [53 x i8], [53 x i8]* @.str.46, i64 0, i64 0
  %3 = call i32 @test_skip(i32 %2)
  ; Expression result: %3
  %4 = call i32 @test_end()
  ; Expression result: %4
  ret i32 0
}

define i32 @test_state_management() {
entry:
  %0 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.47, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = alloca i32, align 4
  store i32 %test_count, i32* %2, align 4
  ; Variable initial_count allocated
  %3 = alloca i32, align 4
  store i32 %test_passed, i32* %3, align 4
  ; Variable initial_passed allocated
  %4 = alloca i32, align 4
  store i32 %test_failed, i32* %4, align 4
  ; Variable initial_failed allocated
  %5 = call i32 @assert_eq_int(i32 42, i32 42)
  ; Expression result: %5
  %6 = load i32, i32* %3, align 4
  %7 = call i32 @assert_greater_than(i32 %test_passed, i32 %6)
  ; Expression result: %7
  %8 = load i32, i32* %2, align 4
  %9 = call i32 @assert_greater_than(i32 %test_count, i32 %8)
  ; Expression result: %9
  %10 = call i32 @test_end()
  ; Expression result: %10
  ret i32 0
}


; String constants
@.str.43 = private unnamed_addr constant [16 x i8] c"test_edge_cases\00", align 1
@.str.49 = private unnamed_addr constant [38 x i8] c"=== Running Basic Assertion Tests ===\00", align 1
@.str.16 = private unnamed_addr constant [25 x i8] c"test_suite_functionality\00", align 1
@.str.12 = private unnamed_addr constant [7 x i8] c"cringe\00", align 1
@.str.35 = private unnamed_addr constant [23 x i8] c"test_reporting_formats\00", align 1
@.str.32 = private unnamed_addr constant [7 x i8] c"tests/\00", align 1
@.str.29 = private unnamed_addr constant [2 x i8] c"4\00", align 1
@.str.0 = private unnamed_addr constant [24 x i8] c"test_integer_assertions\00", align 1
@.str.45 = private unnamed_addr constant [14 x i8] c"test_skipping\00", align 1
@.str.48 = private unnamed_addr constant [62 x i8] c"🧪 Starting CURSED Enhanced Testing Framework Self-Tests...\00", align 1
@.str.36 = private unnamed_addr constant [32 x i8] c"Testing JSON report generation:\00", align 1
@.str.38 = private unnamed_addr constant [32 x i8] c"Testing HTML report generation:\00", align 1
@.str.53 = private unnamed_addr constant [39 x i8] c"=== Running State Management Tests ===\00", align 1
@.str.54 = private unnamed_addr constant [60 x i8] c"🎯 Enhanced CURSED Testing Framework validation complete!\00", align 1
@.str.17 = private unnamed_addr constant [25 x i8] c"test_performance_testing\00", align 1
@.str.41 = private unnamed_addr constant [8 x i8] c"my_test\00", align 1
@.str.1 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.4 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
@.str.46 = private unnamed_addr constant [53 x i8] c"This test is intentionally skipped for demonstration\00", align 1
@.str.11 = private unnamed_addr constant [20 x i8] c"test_nil_assertions\00", align 1
@.str.2 = private unnamed_addr constant [22 x i8] c"test_float_assertions\00", align 1
@.str.33 = private unnamed_addr constant [7 x i8] c"test_*\00", align 1
@.str.37 = private unnamed_addr constant [31 x i8] c"Testing XML report generation:\00", align 1
@.str.58 = private unnamed_addr constant [13 x i8] c"HTML Report:\00", align 1
@.str.28 = private unnamed_addr constant [2 x i8] c"3\00", align 1
@.str.51 = private unnamed_addr constant [50 x i8] c"=== Running Configuration and Reporting Tests ===\00", align 1
@.str.6 = private unnamed_addr constant [6 x i8] c"world\00", align 1
@.str.30 = private unnamed_addr constant [2 x i8] c"5\00", align 1
@.str.39 = private unnamed_addr constant [15 x i8] c"test_filtering\00", align 1
@.str.52 = private unnamed_addr constant [32 x i8] c"=== Running Edge Case Tests ===\00", align 1
@.str.14 = private unnamed_addr constant [3 x i8] c"42\00", align 1
@.str.25 = private unnamed_addr constant [22 x i8] c"test_array_assertions\00", align 1
@.str.40 = private unnamed_addr constant [13 x i8] c"test_example\00", align 1
@.str.8 = private unnamed_addr constant [12 x i8] c"hello world\00", align 1
@.str.59 = private unnamed_addr constant [46 x i8] c"❌ Some tests failed - returning exit code 1\00", align 1
@.str.60 = private unnamed_addr constant [45 x i8] c"✅ All tests passed - returning exit code 0\00", align 1
@.str.42 = private unnamed_addr constant [7 x i8] c"*test*\00", align 1
@.str.26 = private unnamed_addr constant [2 x i8] c"1\00", align 1
@.str.20 = private unnamed_addr constant [14 x i8] c"mocked_result\00", align 1
@.str.50 = private unnamed_addr constant [39 x i8] c"=== Running Advanced Feature Tests ===\00", align 1
@.str.56 = private unnamed_addr constant [13 x i8] c"JSON Report:\00", align 1
@.str.27 = private unnamed_addr constant [2 x i8] c"2\00", align 1
@.str.15 = private unnamed_addr constant [24 x i8] c"Advanced Features Suite\00", align 1
@.str.9 = private unnamed_addr constant [19 x i8] c"CURSED programming\00", align 1
@.str.21 = private unnamed_addr constant [13 x i8] c"Mocked error\00", align 1
@.str.10 = private unnamed_addr constant [24 x i8] c"test_boolean_assertions\00", align 1
@.str.34 = private unnamed_addr constant [8 x i8] c"console\00", align 1
@.str.57 = private unnamed_addr constant [12 x i8] c"XML Report:\00", align 1
@.str.31 = private unnamed_addr constant [19 x i8] c"test_configuration\00", align 1
@.str.5 = private unnamed_addr constant [7 x i8] c"CURSED\00", align 1
@.str.44 = private unnamed_addr constant [2 x i8] c" \00", align 1
@.str.55 = private unnamed_addr constant [35 x i8] c"=== Alternative Output Formats ===\00", align 1
@.str.13 = private unnamed_addr constant [11 x i8] c"not cringe\00", align 1
@.str.18 = private unnamed_addr constant [20 x i8] c"test_mock_functions\00", align 1
@.str.19 = private unnamed_addr constant [14 x i8] c"test_function\00", align 1
@.str.47 = private unnamed_addr constant [22 x i8] c"test_state_management\00", align 1
@.str.23 = private unnamed_addr constant [23 x i8] c"Expected error message\00", align 1
@.str.22 = private unnamed_addr constant [20 x i8] c"test_error_handling\00", align 1
@.str.7 = private unnamed_addr constant [7 x i8] c"cursed\00", align 1
@.str.24 = private unnamed_addr constant [15 x i8] c"risky_function\00", align 1
@.str.3 = private unnamed_addr constant [23 x i8] c"test_string_assertions\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [62 x i8], [62 x i8]* @.str.48, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = call i32 @reset_test_state()
  ; Expression result: %6
  %7 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.49, i64 0, i64 0
  %8 = call i32 @puts(i8* %7)
  %9 = add i32 0, 0
  ; Expression result: %9
  %10 = call i32 @test_integer_assertions()
  ; Expression result: %10
  %11 = call i32 @test_float_assertions()
  ; Expression result: %11
  %12 = call i32 @test_string_assertions()
  ; Expression result: %12
  %13 = call i32 @test_boolean_assertions()
  ; Expression result: %13
  %14 = call i32 @test_nil_assertions()
  ; Expression result: %14
  %15 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.50, i64 0, i64 0
  %16 = call i32 @puts(i8* %15)
  %17 = add i32 0, 0
  ; Expression result: %17
  %18 = call i32 @test_test_suites()
  ; Expression result: %18
  %19 = call i32 @test_performance_testing()
  ; Expression result: %19
  %20 = call i32 @test_mock_functions()
  ; Expression result: %20
  %21 = call i32 @test_error_handling()
  ; Expression result: %21
  %22 = call i32 @test_array_assertions()
  ; Expression result: %22
  %23 = getelementptr inbounds [50 x i8], [50 x i8]* @.str.51, i64 0, i64 0
  %24 = call i32 @puts(i8* %23)
  %25 = add i32 0, 0
  ; Expression result: %25
  %26 = call i32 @test_configuration()
  ; Expression result: %26
  %27 = call i32 @test_reporting_formats()
  ; Expression result: %27
  %28 = call i32 @test_filtering()
  ; Expression result: %28
  %29 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.52, i64 0, i64 0
  %30 = call i32 @puts(i8* %29)
  %31 = add i32 0, 0
  ; Expression result: %31
  %32 = call i32 @test_edge_cases()
  ; Expression result: %32
  %33 = call i32 @test_skipping()
  ; Expression result: %33
  %34 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.53, i64 0, i64 0
  %35 = call i32 @puts(i8* %34)
  %36 = add i32 0, 0
  ; Expression result: %36
  %37 = call i32 @test_state_management()
  ; Expression result: %37
  %38 = call i32 @print_test_summary()
  ; Expression result: %38
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %40 = call i32 @puts(i8* %39)
  %41 = add i32 0, 0
  ; Expression result: %41
  %42 = getelementptr inbounds [60 x i8], [60 x i8]* @.str.54, i64 0, i64 0
  %43 = call i32 @puts(i8* %42)
  %44 = add i32 0, 0
  ; Expression result: %44
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %46 = call i32 @puts(i8* %45)
  %47 = add i32 0, 0
  ; Expression result: %47
  %48 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.55, i64 0, i64 0
  %49 = call i32 @puts(i8* %48)
  %50 = add i32 0, 0
  ; Expression result: %50
  %51 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.56, i64 0, i64 0
  %52 = call i32 @puts(i8* %51)
  %53 = add i32 0, 0
  ; Expression result: %53
  %54 = call i32 @generate_json_report()
  ; Expression result: %54
  %55 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %56 = call i32 @puts(i8* %55)
  %57 = add i32 0, 0
  ; Expression result: %57
  %58 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.57, i64 0, i64 0
  %59 = call i32 @puts(i8* %58)
  %60 = add i32 0, 0
  ; Expression result: %60
  %61 = call i32 @generate_xml_report()
  ; Expression result: %61
  %62 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %63 = call i32 @puts(i8* %62)
  %64 = add i32 0, 0
  ; Expression result: %64
  %65 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.58, i64 0, i64 0
  %66 = call i32 @puts(i8* %65)
  %67 = add i32 0, 0
  ; Expression result: %67
  %68 = call i32 @generate_html_report()
  ; Expression result: %68
  %69 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %70 = call i32 @puts(i8* %69)
  %71 = add i32 0, 0
  ; Expression result: %71
  %72 = icmp sgt i32 %test_failed, 0
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %72, label %label0, label %label1
label0:
  %73 = getelementptr inbounds [46 x i8], [46 x i8]* @.str.59, i64 0, i64 0
  %74 = call i32 @puts(i8* %73)
  %75 = add i32 0, 0
  ; Expression result: %75
  %76 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %76
  ; Expression result: 1
  br label %label2
label1:
  %77 = getelementptr inbounds [45 x i8], [45 x i8]* @.str.60, i64 0, i64 0
  %78 = call i32 @puts(i8* %77)
  %79 = add i32 0, 0
  ; Expression result: %79
  %80 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %80
  ; Expression result: 0
  br label %label2
label2:
  ret i32 0
}

