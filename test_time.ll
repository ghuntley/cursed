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
define i8* @test_current_time() {
entry:
  %0 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @time_now()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable now_seconds allocated
  %4 = call i32 @time_now_millis()
  %5 = alloca i32, align 4
  store i32 %4, i32* %5, align 4
  ; Variable now_millis allocated
  %6 = call i32 @time_now_micros()
  %7 = alloca i32, align 4
  store i32 %6, i32* %7, align 4
  ; Variable now_micros allocated
  %8 = call i32 @time_now_nanos()
  %9 = alloca i32, align 4
  store i32 %8, i32* %9, align 4
  ; Variable now_nanos allocated
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %10
  ; Expression result: 0
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %12
  ; Expression result: 0
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  ; Expression result: 0
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %15
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %16
  ; Expression result: 0
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %18
  %19 = load i32, i32* %3, align 4
  %20 = mul i32 %19, 1000
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %21
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %22
  %23 = load i32, i32* %5, align 4
  %24 = mul i32 %23, 1000
  ; Expression result: %24
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %25
  %26 = call i32 @time_now()
  %27 = alloca i32, align 4
  store i32 %26, i32* %27, align 4
  ; Variable later_seconds allocated
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %28
  %29 = load i32, i32* %3, align 4
  ; Expression result: %29
  %30 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %30
  ret i32 0
}

define i8* @test_time_creation() {
entry:
  %0 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.2, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = alloca i32, align 4
  store i32 1609459200, i32* %2, align 4
  ; Variable timestamp allocated
  %3 = load i32, i32* %2, align 4
  %4 = call i32 @time_from_timestamp(i32 %3)
  %5 = alloca i32, align 4
  store i32 %4, i32* %5, align 4
  ; Variable dt allocated
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = alloca i32, align 4
  store i32 1609459200000, i32* %9, align 4
  ; Variable millis allocated
  %10 = load i32, i32* %9, align 4
  %11 = call i32 @time_from_millis(i32 %10)
  %12 = alloca i32, align 4
  store i32 %11, i32* %12, align 4
  ; Variable dt_millis allocated
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %15
  %16 = call i32 @time_create(i32 2021, i32 1, i32 1, i32 0, i32 0, i32 0)
  %17 = alloca i32, align 4
  store i32 %16, i32* %17, align 4
  ; Variable created_time allocated
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  %21 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.3, i64 0, i64 0
  %22 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.4, i64 0, i64 0
  %23 = call i32 @time_parse(i32 %21, i32 %22)
  %24 = alloca i32, align 4
  store i32 %23, i32* %24, align 4
  ; Variable parsed_time allocated
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %26
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %27
  ret i32 0
}

define i8* @test_time_components() {
entry:
  %0 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.5, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @time_create(i32 2021, i32 12, i32 25, i32 15, i32 30, i32 45)
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable test_time allocated
  %4 = load i32, i32* %3, align 4
  %5 = call i32 @time_year(i32 %4)
  %6 = call i32 @assert_eq_int(i32 %5, i32 2021)
  ; Expression result: %6
  %7 = load i32, i32* %3, align 4
  %8 = call i32 @time_month(i32 %7)
  %9 = call i32 @assert_eq_int(i32 %8, i32 12)
  ; Expression result: %9
  %10 = load i32, i32* %3, align 4
  %11 = call i32 @time_day(i32 %10)
  %12 = call i32 @assert_eq_int(i32 %11, i32 25)
  ; Expression result: %12
  %13 = load i32, i32* %3, align 4
  %14 = call i32 @time_hour(i32 %13)
  %15 = call i32 @assert_eq_int(i32 %14, i32 15)
  ; Expression result: %15
  %16 = load i32, i32* %3, align 4
  %17 = call i32 @time_minute(i32 %16)
  %18 = call i32 @assert_eq_int(i32 %17, i32 30)
  ; Expression result: %18
  %19 = load i32, i32* %3, align 4
  %20 = call i32 @time_second(i32 %19)
  %21 = call i32 @assert_eq_int(i32 %20, i32 45)
  ; Expression result: %21
  %22 = load i32, i32* %3, align 4
  %23 = call i32 @time_weekday(i32 %22)
  %24 = alloca i32, align 4
  store i32 %23, i32* %24, align 4
  ; Variable weekday allocated
  %25 = load i32, i32* %3, align 4
  %26 = call i32 @time_day_of_year(i32 %25)
  %27 = alloca i32, align 4
  store i32 %26, i32* %27, align 4
  ; Variable day_of_year allocated
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %28
  ; Expression result: 0
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %29
  %30 = load i32, i32* %24, align 4
  ; Expression result: %30
  %31 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %31
  ; Expression result: 6
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %32
  %33 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %33
  ; Expression result: 1
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %34
  %35 = load i32, i32* %27, align 4
  ; Expression result: %35
  %36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %36
  ; Expression result: 366
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %37
  ret i32 0
}

define i8* @test_time_formatting() {
entry:
  %0 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.6, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @time_create(i32 2021, i32 6, i32 15, i32 9, i32 30, i32 0)
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable test_time allocated
  %4 = load i32, i32* %3, align 4
  %5 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.7, i64 0, i64 0
  %6 = call i32 @time_format(i32 %4, i32 %5)
  %7 = alloca i32, align 4
  store i32 %6, i32* %7, align 4
  ; Variable formatted allocated
  %8 = load i32, i32* %7, align 4
  %9 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.8, i64 0, i64 0
  %10 = call i32 @string_contains(i32 %8, i32 %9)
  %11 = call i32 @assert_true(i32 %10)
  ; Expression result: %11
  %12 = load i32, i32* %7, align 4
  %13 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.9, i64 0, i64 0
  %14 = call i32 @string_contains(i32 %12, i32 %13)
  %15 = call i32 @assert_true(i32 %14)
  ; Expression result: %15
  %16 = load i32, i32* %7, align 4
  %17 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.10, i64 0, i64 0
  %18 = call i32 @string_contains(i32 %16, i32 %17)
  %19 = call i32 @assert_true(i32 %18)
  ; Expression result: %19
  %20 = load i32, i32* %3, align 4
  %21 = call i32 @time_to_string(i32 %20)
  %22 = alloca i32, align 4
  store i32 %21, i32* %22, align 4
  ; Variable time_str allocated
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  ; Expression result: 0
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %24
  %25 = load i32, i32* %3, align 4
  %26 = call i32 @time_to_iso8601(i32 %25)
  %27 = alloca i32, align 4
  store i32 %26, i32* %27, align 4
  ; Variable iso_str allocated
  %28 = load i32, i32* %27, align 4
  %29 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.11, i64 0, i64 0
  %30 = call i32 @string_contains(i8* %28, i32 %29)
  %31 = call i32 @assert_true(i32 %30)
  ; Expression result: %31
  %32 = load i32, i32* %27, align 4
  %33 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.8, i64 0, i64 0
  %34 = call i32 @string_contains(i8* %32, i32 %33)
  %35 = call i32 @assert_true(i32 %34)
  ; Expression result: %35
  %36 = load i32, i32* %3, align 4
  %37 = call i32 @time_to_rfc3339(i32 %36)
  %38 = alloca i32, align 4
  store i32 %37, i32* %38, align 4
  ; Variable rfc_str allocated
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %39
  ; Expression result: 0
  %40 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %40
  ret i32 0
}

define i32 @test_time_arithmetic() {
entry:
  %0 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.12, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @time_create(i32 2021, i32 6, i32 15, i32 12, i32 0, i32 0)
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable base_time allocated
  %4 = load i32, i32* %3, align 4
  %5 = call i32 @time_add_years(i32 %4, i32 1)
  %6 = alloca i32, align 4
  store i32 %5, i32* %6, align 4
  ; Variable plus_year allocated
  %7 = load i32, i32* %6, align 4
  %8 = call i32 @time_year(i32 %7)
  %9 = call i32 @assert_eq_int(i32 %8, i32 2022)
  ; Expression result: %9
  %10 = load i32, i32* %6, align 4
  %11 = call i32 @time_month(i32 %10)
  %12 = call i32 @assert_eq_int(i32 %11, i32 6)
  ; Expression result: %12
  %13 = load i32, i32* %6, align 4
  %14 = call i32 @time_day(i32 %13)
  %15 = call i32 @assert_eq_int(i32 %14, i32 15)
  ; Expression result: %15
  %16 = load i32, i32* %3, align 4
  %17 = call i32 @time_add_months(i32 %16, i32 3)
  %18 = alloca i32, align 4
  store i32 %17, i32* %18, align 4
  ; Variable plus_month allocated
  %19 = load i32, i32* %18, align 4
  %20 = call i32 @time_year(i32 %19)
  %21 = call i32 @assert_eq_int(i32 %20, i32 2021)
  ; Expression result: %21
  %22 = load i32, i32* %18, align 4
  %23 = call i32 @time_month(i32 %22)
  %24 = call i32 @assert_eq_int(i32 %23, i32 9)
  ; Expression result: %24
  %25 = load i32, i32* %18, align 4
  %26 = call i32 @time_day(i32 %25)
  %27 = call i32 @assert_eq_int(i32 %26, i32 15)
  ; Expression result: %27
  %28 = load i32, i32* %3, align 4
  %29 = call i32 @time_add_days(i32 %28, i32 10)
  %30 = alloca i32, align 4
  store i32 %29, i32* %30, align 4
  ; Variable plus_days allocated
  %31 = load i32, i32* %30, align 4
  %32 = call i32 @time_year(i32 %31)
  %33 = call i32 @assert_eq_int(i32 %32, i32 2021)
  ; Expression result: %33
  %34 = load i32, i32* %30, align 4
  %35 = call i32 @time_month(i32 %34)
  %36 = call i32 @assert_eq_int(i32 %35, i32 6)
  ; Expression result: %36
  %37 = load i32, i32* %30, align 4
  %38 = call i32 @time_day(i32 %37)
  %39 = call i32 @assert_eq_int(i32 %38, i32 25)
  ; Expression result: %39
  %40 = load i32, i32* %3, align 4
  %41 = call i32 @time_add_hours(i32 %40, i32 5)
  %42 = alloca i32, align 4
  store i32 %41, i32* %42, align 4
  ; Variable plus_hours allocated
  %43 = load i32, i32* %42, align 4
  %44 = call i32 @time_hour(i32 %43)
  %45 = call i32 @assert_eq_int(i32 %44, i32 17)
  ; Expression result: %45
  %46 = load i32, i32* %3, align 4
  %47 = call i32 @time_add_minutes(i32 %46, i32 30)
  %48 = alloca i32, align 4
  store i32 %47, i32* %48, align 4
  ; Variable plus_minutes allocated
  %49 = load i32, i32* %48, align 4
  %50 = call i32 @time_minute(i32 %49)
  %51 = call i32 @assert_eq_int(i32 %50, i32 30)
  ; Expression result: %51
  %52 = load i32, i32* %3, align 4
  %53 = call i32 @time_add_seconds(i32 %52, i32 45)
  %54 = alloca i32, align 4
  store i32 %53, i32* %54, align 4
  ; Variable plus_seconds allocated
  %55 = load i32, i32* %54, align 4
  %56 = call i32 @time_second(i32 %55)
  %57 = call i32 @assert_eq_int(i32 %56, i32 45)
  ; Expression result: %57
  ret i32 0
}

define i8* @test_time_differences() {
entry:
  %0 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.13, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @time_create(i32 2021, i32 6, i32 15, i32 12, i32 0, i32 0)
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable time1 allocated
  %4 = call i32 @time_create(i32 2021, i32 6, i32 16, i32 15, i32 30, i32 45)
  %5 = alloca i32, align 4
  store i32 %4, i32* %5, align 4
  ; Variable time2 allocated
  %6 = load i32, i32* %5, align 4
  %7 = load i32, i32* %3, align 4
  %8 = call i32 @time_diff_days(i32 %6, i32 %7)
  %9 = alloca i32, align 4
  store i32 %8, i32* %9, align 4
  ; Variable diff_days allocated
  %10 = load i32, i32* %9, align 4
  %11 = call i32 @assert_eq_int(i32 %10, i32 1)
  ; Expression result: %11
  %12 = load i32, i32* %5, align 4
  %13 = load i32, i32* %3, align 4
  %14 = call i32 @time_diff_hours(i32 %12, i32 %13)
  %15 = alloca i32, align 4
  store i32 %14, i32* %15, align 4
  ; Variable diff_hours allocated
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %16
  ; Expression result: 27
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = load i32, i32* %15, align 4
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %19
  ; Expression result: 28
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  %21 = load i32, i32* %5, align 4
  %22 = load i32, i32* %3, align 4
  %23 = call i32 @time_diff_minutes(i32 %21, i32 %22)
  %24 = alloca i32, align 4
  store i32 %23, i32* %24, align 4
  ; Variable diff_minutes allocated
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %25
  ; Expression result: 1650
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %26
  %27 = load i32, i32* %24, align 4
  ; Expression result: %27
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %28
  ; Expression result: 1700
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %29
  %30 = load i32, i32* %5, align 4
  %31 = load i32, i32* %3, align 4
  %32 = call i32 @time_diff_seconds(i32 %30, i32 %31)
  %33 = alloca i32, align 4
  store i32 %32, i32* %33, align 4
  ; Variable diff_seconds allocated
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %34
  ; Expression result: 99000
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %35
  %36 = load i32, i32* %5, align 4
  %37 = load i32, i32* %3, align 4
  %38 = call i32 @time_subtract(i32 %36, i32 %37)
  %39 = alloca i32, align 4
  store i32 %38, i32* %39, align 4
  ; Variable duration_diff allocated
  %40 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %40
  %41 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %41
  %42 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %42
  ret i32 0
}

define i32 @test_duration_operations() {
entry:
  %0 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.14, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @duration_from_seconds(i32 3600)
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable dur_sec allocated
  %4 = call i32 @duration_from_millis(i32 3600000)
  %5 = alloca i32, align 4
  store i32 %4, i32* %5, align 4
  ; Variable dur_millis allocated
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %9
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = load i32, i32* %3, align 4
  %13 = call i32 @duration_to_seconds(i32 %12)
  %14 = call i32 @assert_eq_int(i32 %13, i32 3600)
  ; Expression result: %14
  %15 = load i32, i32* %5, align 4
  %16 = call i32 @duration_to_millis(i32 %15)
  %17 = call i32 @assert_eq_int(i32 %16, i32 3600000)
  ; Expression result: %17
  %18 = call i32 @duration_from_seconds(i32 1800)
  %19 = alloca i32, align 4
  store i32 %18, i32* %19, align 4
  ; Variable dur1 allocated
  %20 = call i32 @duration_from_seconds(i32 1200)
  %21 = alloca i32, align 4
  store i32 %20, i32* %21, align 4
  ; Variable dur2 allocated
  %22 = load i32, i32* %19, align 4
  %23 = load i32, i32* %21, align 4
  %24 = call i32 @duration_add(i32 %22, i32 %23)
  %25 = alloca i32, align 4
  store i32 %24, i32* %25, align 4
  ; Variable dur_sum allocated
  %26 = load i32, i32* %19, align 4
  %27 = load i32, i32* %21, align 4
  %28 = call i32 @duration_subtract(i32 %26, i32 %27)
  %29 = alloca i32, align 4
  store i32 %28, i32* %29, align 4
  ; Variable dur_diff allocated
  %30 = load i32, i32* %25, align 4
  %31 = call i32 @duration_to_seconds(i32 %30)
  %32 = call i32 @assert_eq_int(i32 %31, i32 3000)
  ; Expression result: %32
  %33 = load i32, i32* %29, align 4
  %34 = call i32 @duration_to_seconds(i32 %33)
  %35 = call i32 @assert_eq_int(i32 %34, i32 600)
  ; Expression result: %35
  ret i32 0
}

define i8* @test_timezone_operations() {
entry:
  %0 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.15, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @time_utc()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable utc_time allocated
  %4 = call i32 @time_local()
  %5 = alloca i32, align 4
  store i32 %4, i32* %5, align 4
  ; Variable local_time allocated
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %9
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = call i32 @time_create(i32 2021, i32 6, i32 15, i32 12, i32 0, i32 0)
  %13 = alloca i32, align 4
  store i32 %12, i32* %13, align 4
  ; Variable test_time allocated
  %14 = load i32, i32* %13, align 4
  %15 = call i32 @time_to_utc(i32 %14)
  %16 = alloca i32, align 4
  store i32 %15, i32* %16, align 4
  ; Variable utc_converted allocated
  %17 = load i32, i32* %13, align 4
  %18 = call i32 @time_to_local(i32 %17)
  %19 = alloca i32, align 4
  store i32 %18, i32* %19, align 4
  ; Variable local_converted allocated
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %21
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %24
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %25
  %26 = call i32 @time_timezone_offset()
  %27 = alloca i32, align 4
  store i32 %26, i32* %27, align 4
  ; Variable offset allocated
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %29 = sub i32 %28, 12
  %30 = mul i32 %29, 3600
  ; Expression result: %30
  %31 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %31
  %32 = load i32, i32* %27, align 4
  ; Expression result: %32
  %33 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %33
  %34 = mul i32 12, 3600
  ; Expression result: %34
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %35
  ret i32 0
}

define i32 @test_time_validation() {
entry:
  %0 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.16, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @time_is_leap_year(i32 2020)
  %3 = call i32 @assert_true(i32 %2)
  ; Expression result: %3
  %4 = call i32 @time_is_leap_year(i32 2021)
  %5 = call i32 @assert_false(i32 %4)
  ; Expression result: %5
  %6 = call i32 @time_is_leap_year(i32 2000)
  %7 = call i32 @assert_true(i32 %6)
  ; Expression result: %7
  %8 = call i32 @time_is_leap_year(i32 1900)
  %9 = call i32 @assert_false(i32 %8)
  ; Expression result: %9
  %10 = call i32 @time_days_in_month(i32 2021, i32 1)
  %11 = call i32 @assert_eq_int(i32 %10, i32 31)
  ; Expression result: %11
  %12 = call i32 @time_days_in_month(i32 2021, i32 2)
  %13 = call i32 @assert_eq_int(i32 %12, i32 28)
  ; Expression result: %13
  %14 = call i32 @time_days_in_month(i32 2020, i32 2)
  %15 = call i32 @assert_eq_int(i32 %14, i32 29)
  ; Expression result: %15
  %16 = call i32 @time_days_in_month(i32 2021, i32 4)
  %17 = call i32 @assert_eq_int(i32 %16, i32 30)
  ; Expression result: %17
  %18 = call i32 @time_is_valid_date(i32 2021, i32 6, i32 15)
  %19 = call i32 @assert_true(i32 %18)
  ; Expression result: %19
  %20 = call i32 @time_is_valid_date(i32 2020, i32 2, i32 29)
  %21 = call i32 @assert_true(i32 %20)
  ; Expression result: %21
  %22 = call i32 @time_is_valid_date(i32 2021, i32 2, i32 29)
  %23 = call i32 @assert_false(i32 %22)
  ; Expression result: %23
  %24 = call i32 @time_is_valid_date(i32 2021, i32 13, i32 1)
  %25 = call i32 @assert_false(i32 %24)
  ; Expression result: %25
  %26 = call i32 @time_is_valid_date(i32 2021, i32 6, i32 32)
  %27 = call i32 @assert_false(i32 %26)
  ; Expression result: %27
  %28 = call i32 @time_create(i32 2021, i32 6, i32 14, i32 12, i32 0, i32 0)
  %29 = alloca i32, align 4
  store i32 %28, i32* %29, align 4
  ; Variable monday allocated
  %30 = call i32 @time_create(i32 2021, i32 6, i32 19, i32 12, i32 0, i32 0)
  %31 = alloca i32, align 4
  store i32 %30, i32* %31, align 4
  ; Variable saturday allocated
  %32 = call i32 @time_create(i32 2021, i32 6, i32 20, i32 12, i32 0, i32 0)
  %33 = alloca i32, align 4
  store i32 %32, i32* %33, align 4
  ; Variable sunday allocated
  %34 = load i32, i32* %29, align 4
  %35 = call i32 @time_is_weekend(i32 %34)
  %36 = call i32 @assert_false(i32 %35)
  ; Expression result: %36
  %37 = load i32, i32* %31, align 4
  %38 = call i32 @time_is_weekend(i32 %37)
  %39 = call i32 @assert_true(i32 %38)
  ; Expression result: %39
  %40 = load i32, i32* %33, align 4
  %41 = call i32 @time_is_weekend(i32 %40)
  %42 = call i32 @assert_true(i32 %41)
  ; Expression result: %42
  ret i32 0
}

define i32 @test_time_constants() {
entry:
  %0 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.17, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @time_seconds_per_minute()
  %3 = call i32 @assert_eq_int(i32 %2, i32 60)
  ; Expression result: %3
  %4 = call i32 @time_minutes_per_hour()
  %5 = call i32 @assert_eq_int(i32 %4, i32 60)
  ; Expression result: %5
  %6 = call i32 @time_hours_per_day()
  %7 = call i32 @assert_eq_int(i32 %6, i32 24)
  ; Expression result: %7
  %8 = call i32 @time_days_per_week()
  %9 = call i32 @assert_eq_int(i32 %8, i32 7)
  ; Expression result: %9
  %10 = call i32 @time_months_per_year()
  %11 = call i32 @assert_eq_int(i32 %10, i32 12)
  ; Expression result: %11
  %12 = call i32 @time_millis_per_second()
  %13 = call i32 @assert_eq_int(i32 %12, i32 1000)
  ; Expression result: %13
  %14 = call i32 @time_micros_per_second()
  %15 = call i32 @assert_eq_int(i32 %14, i32 1000000)
  ; Expression result: %15
  %16 = call i32 @time_nanos_per_second()
  %17 = call i32 @assert_eq_int(i32 %16, i32 1000000000)
  ; Expression result: %17
  ret i32 0
}

define i8* @test_time_sleep() {
entry:
  %0 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.18, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @time_now_millis()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable start_time allocated
  %4 = call i32 @time_sleep_millis(i32 10)
  ; Expression result: %4
  %5 = call i32 @time_now_millis()
  %6 = alloca i32, align 4
  store i32 %5, i32* %6, align 4
  ; Variable end_time allocated
  %7 = load i32, i32* %6, align 4
  %8 = load i32, i32* %3, align 4
  %9 = sub i32 %7, %8
  %10 = alloca i32, align 4
  store i32 %9, i32* %10, align 4
  ; Variable elapsed allocated
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  ; Expression result: 5
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %12
  %13 = load i32, i32* %10, align 4
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  ; Expression result: 50
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %15
  %16 = call i32 @time_now_micros()
  %17 = alloca i32, align 4
  store i32 %16, i32* %17, align 4
  ; Variable start_micros allocated
  %18 = call i32 @time_sleep_micros(i32 1000)
  ; Expression result: %18
  %19 = call i32 @time_now_micros()
  %20 = alloca i32, align 4
  store i32 %19, i32* %20, align 4
  ; Variable end_micros allocated
  %21 = load i32, i32* %20, align 4
  %22 = load i32, i32* %17, align 4
  %23 = sub i32 %21, %22
  %24 = alloca i32, align 4
  store i32 %23, i32* %24, align 4
  ; Variable elapsed_micros allocated
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %25
  ; Expression result: 500
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %26
  ret i32 0
}

define i32 @test_time_benchmarking() {
entry:
  %0 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.19, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  ; Nested function skipped
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  ; Expression result: %sum
  ret i32 0
}

define i32 @test_time_edge_cases() {
entry:
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.20, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @time_from_timestamp(i32 0)
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable epoch allocated
  %4 = load i32, i32* %3, align 4
  %5 = call i32 @time_year(i32 %4)
  %6 = call i32 @assert_eq_int(i32 %5, i32 1970)
  ; Expression result: %6
  %7 = load i32, i32* %3, align 4
  %8 = call i32 @time_month(i32 %7)
  %9 = call i32 @assert_eq_int(i32 %8, i32 1)
  ; Expression result: %9
  %10 = load i32, i32* %3, align 4
  %11 = call i32 @time_day(i32 %10)
  %12 = call i32 @assert_eq_int(i32 %11, i32 1)
  ; Expression result: %12
  %13 = call i32 @time_create(i32 2100, i32 12, i32 31, i32 23, i32 59, i32 59)
  %14 = alloca i32, align 4
  store i32 %13, i32* %14, align 4
  ; Variable future allocated
  %15 = load i32, i32* %14, align 4
  %16 = call i32 @time_year(i32 %15)
  %17 = call i32 @assert_eq_int(i32 %16, i32 2100)
  ; Expression result: %17
  %18 = load i32, i32* %14, align 4
  %19 = call i32 @time_month(i32 %18)
  %20 = call i32 @assert_eq_int(i32 %19, i32 12)
  ; Expression result: %20
  %21 = load i32, i32* %14, align 4
  %22 = call i32 @time_day(i32 %21)
  %23 = call i32 @assert_eq_int(i32 %22, i32 31)
  ; Expression result: %23
  %24 = call i32 @time_create(i32 2020, i32 2, i32 29, i32 12, i32 0, i32 0)
  %25 = alloca i32, align 4
  store i32 %24, i32* %25, align 4
  ; Variable leap_day allocated
  %26 = load i32, i32* %25, align 4
  %27 = call i32 @time_year(i32 %26)
  %28 = call i32 @assert_eq_int(i32 %27, i32 2020)
  ; Expression result: %28
  %29 = load i32, i32* %25, align 4
  %30 = call i32 @time_month(i32 %29)
  %31 = call i32 @assert_eq_int(i32 %30, i32 2)
  ; Expression result: %31
  %32 = load i32, i32* %25, align 4
  %33 = call i32 @time_day(i32 %32)
  %34 = call i32 @assert_eq_int(i32 %33, i32 29)
  ; Expression result: %34
  %35 = call i32 @time_create(i32 2021, i32 12, i32 31, i32 23, i32 59, i32 59)
  %36 = alloca i32, align 4
  store i32 %35, i32* %36, align 4
  ; Variable end_of_year allocated
  %37 = load i32, i32* %36, align 4
  %38 = call i32 @time_add_seconds(i32 %37, i32 1)
  %39 = alloca i32, align 4
  store i32 %38, i32* %39, align 4
  ; Variable next_second allocated
  %40 = load i32, i32* %39, align 4
  %41 = call i32 @time_year(i32 %40)
  %42 = call i32 @assert_eq_int(i32 %41, i32 2022)
  ; Expression result: %42
  %43 = load i32, i32* %39, align 4
  %44 = call i32 @time_month(i32 %43)
  %45 = call i32 @assert_eq_int(i32 %44, i32 1)
  ; Expression result: %45
  %46 = load i32, i32* %39, align 4
  %47 = call i32 @time_day(i32 %46)
  %48 = call i32 @assert_eq_int(i32 %47, i32 1)
  ; Expression result: %48
  %49 = load i32, i32* %39, align 4
  %50 = call i32 @time_hour(i32 %49)
  %51 = call i32 @assert_eq_int(i32 %50, i32 0)
  ; Expression result: %51
  %52 = load i32, i32* %39, align 4
  %53 = call i32 @time_minute(i32 %52)
  %54 = call i32 @assert_eq_int(i32 %53, i32 0)
  ; Expression result: %54
  %55 = load i32, i32* %39, align 4
  %56 = call i32 @time_second(i32 %55)
  %57 = call i32 @assert_eq_int(i32 %56, i32 0)
  ; Expression result: %57
  ret i32 0
}

define i32 @test_time_parsing_edge_cases() {
entry:
  %0 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.21, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.22, i64 0, i64 0
  %3 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.23, i64 0, i64 0
  %4 = call i32 @time_parse(i32 %2, i32 %3)
  %5 = alloca i32, align 4
  store i32 %4, i32* %5, align 4
  ; Variable iso_date allocated
  %6 = load i32, i32* %5, align 4
  %7 = call i32 @time_year(i32 %6)
  %8 = call i32 @assert_eq_int(i32 %7, i32 2021)
  ; Expression result: %8
  %9 = load i32, i32* %5, align 4
  %10 = call i32 @time_month(i32 %9)
  %11 = call i32 @assert_eq_int(i32 %10, i32 6)
  ; Expression result: %11
  %12 = load i32, i32* %5, align 4
  %13 = call i32 @time_day(i32 %12)
  %14 = call i32 @assert_eq_int(i32 %13, i32 15)
  ; Expression result: %14
  %15 = load i32, i32* %5, align 4
  %16 = call i32 @time_hour(i32 %15)
  %17 = call i32 @assert_eq_int(i32 %16, i32 14)
  ; Expression result: %17
  %18 = load i32, i32* %5, align 4
  %19 = call i32 @time_minute(i32 %18)
  %20 = call i32 @assert_eq_int(i32 %19, i32 30)
  ; Expression result: %20
  %21 = load i32, i32* %5, align 4
  %22 = call i32 @time_second(i32 %21)
  %23 = call i32 @assert_eq_int(i32 %22, i32 0)
  ; Expression result: %23
  %24 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.24, i64 0, i64 0
  %25 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.25, i64 0, i64 0
  %26 = call i32 @time_parse(i32 %24, i32 %25)
  %27 = alloca i32, align 4
  store i32 %26, i32* %27, align 4
  ; Variable us_date allocated
  %28 = load i32, i32* %27, align 4
  %29 = call i32 @time_year(i32 %28)
  %30 = call i32 @assert_eq_int(i32 %29, i32 2021)
  ; Expression result: %30
  %31 = load i32, i32* %27, align 4
  %32 = call i32 @time_month(i32 %31)
  %33 = call i32 @assert_eq_int(i32 %32, i32 6)
  ; Expression result: %33
  %34 = load i32, i32* %27, align 4
  %35 = call i32 @time_day(i32 %34)
  %36 = call i32 @assert_eq_int(i32 %35, i32 15)
  ; Expression result: %36
  %37 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.26, i64 0, i64 0
  %38 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.27, i64 0, i64 0
  %39 = call i32 @time_parse(i32 %37, i32 %38)
  %40 = alloca i32, align 4
  store i32 %39, i32* %40, align 4
  ; Variable time_only allocated
  %41 = load i32, i32* %40, align 4
  %42 = call i32 @time_hour(i32 %41)
  %43 = call i32 @assert_eq_int(i32 %42, i32 14)
  ; Expression result: %43
  %44 = load i32, i32* %40, align 4
  %45 = call i32 @time_minute(i32 %44)
  %46 = call i32 @assert_eq_int(i32 %45, i32 30)
  ; Expression result: %46
  %47 = load i32, i32* %40, align 4
  %48 = call i32 @time_second(i32 %47)
  %49 = call i32 @assert_eq_int(i32 %48, i32 45)
  ; Expression result: %49
  ret i32 0
}

define i32 @run_all_time_tests() {
entry:
  %0 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.28, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.29, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = call i32 @test_current_time()
  ; Expression result: %6
  %7 = call i32 @test_time_creation()
  ; Expression result: %7
  %8 = call i32 @test_time_components()
  ; Expression result: %8
  %9 = call i32 @test_time_formatting()
  ; Expression result: %9
  %10 = call i32 @test_time_arithmetic()
  ; Expression result: %10
  %11 = call i32 @test_time_differences()
  ; Expression result: %11
  %12 = call i32 @test_duration_operations()
  ; Expression result: %12
  %13 = call i32 @test_timezone_operations()
  ; Expression result: %13
  %14 = call i32 @test_time_validation()
  ; Expression result: %14
  %15 = call i32 @test_time_constants()
  ; Expression result: %15
  %16 = call i32 @test_time_sleep()
  ; Expression result: %16
  %17 = call i32 @test_time_benchmarking()
  ; Expression result: %17
  %18 = call i32 @test_time_edge_cases()
  ; Expression result: %18
  %19 = call i32 @test_time_parsing_edge_cases()
  ; Expression result: %19
  %20 = call i32 @print_test_summary()
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %21
  %22 = call i32 @run_all_tests()
  ; Expression result: %22
  ret i32 0
}



; String constants
@.str.9 = private unnamed_addr constant [3 x i8] c"06\00", align 1
@.str.6 = private unnamed_addr constant [26 x i8] c"Time Formatting Functions\00", align 1
@.str.15 = private unnamed_addr constant [20 x i8] c"Timezone Operations\00", align 1
@.str.21 = private unnamed_addr constant [24 x i8] c"Time Parsing Edge Cases\00", align 1
@.str.24 = private unnamed_addr constant [11 x i8] c"06/15/2021\00", align 1
@.str.1 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.13 = private unnamed_addr constant [26 x i8] c"Time Difference Functions\00", align 1
@.str.28 = private unnamed_addr constant [38 x i8] c"⏰ Running CURSED Time Library Tests\00", align 1
@.str.23 = private unnamed_addr constant [18 x i8] c"%Y-%m-%dT%H:%M:%S\00", align 1
@.str.27 = private unnamed_addr constant [9 x i8] c"%H:%M:%S\00", align 1
@.str.5 = private unnamed_addr constant [26 x i8] c"Time Component Extraction\00", align 1
@.str.3 = private unnamed_addr constant [11 x i8] c"2021-01-01\00", align 1
@.str.8 = private unnamed_addr constant [5 x i8] c"2021\00", align 1
@.str.2 = private unnamed_addr constant [24 x i8] c"Time Creation Functions\00", align 1
@.str.26 = private unnamed_addr constant [9 x i8] c"14:30:45\00", align 1
@.str.4 = private unnamed_addr constant [9 x i8] c"%Y-%m-%d\00", align 1
@.str.12 = private unnamed_addr constant [26 x i8] c"Time Arithmetic Functions\00", align 1
@.str.30 = private unnamed_addr constant [2 x i8] c"0\00", align 1
@.str.18 = private unnamed_addr constant [21 x i8] c"Time Sleep Functions\00", align 1
@.str.10 = private unnamed_addr constant [3 x i8] c"15\00", align 1
@.str.22 = private unnamed_addr constant [20 x i8] c"2021-06-15T14:30:00\00", align 1
@.str.29 = private unnamed_addr constant [35 x i8] c"==================================\00", align 1
@.str.17 = private unnamed_addr constant [15 x i8] c"Time Constants\00", align 1
@.str.16 = private unnamed_addr constant [26 x i8] c"Time Validation Functions\00", align 1
@.str.0 = private unnamed_addr constant [23 x i8] c"Current Time Functions\00", align 1
@.str.25 = private unnamed_addr constant [9 x i8] c"%m/%d/%Y\00", align 1
@.str.31 = private unnamed_addr constant [2 x i8] c"1\00", align 1
@.str.7 = private unnamed_addr constant [18 x i8] c"%Y-%m-%d %H:%M:%S\00", align 1
@.str.14 = private unnamed_addr constant [20 x i8] c"Duration Operations\00", align 1
@.str.19 = private unnamed_addr constant [28 x i8] c"Time Benchmarking Functions\00", align 1
@.str.11 = private unnamed_addr constant [2 x i8] c"T\00", align 1
@.str.20 = private unnamed_addr constant [16 x i8] c"Time Edge Cases\00", align 1
define i32 @main() {
  %0 = call i32 @time_benchmark(i32 %test_function)
  %1 = alloca %struct.duration, align 4
  store %struct.duration %0, %struct.duration* %1, align 4
  ; Variable benchmark_duration allocated at %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %7 = call i32 @time_measure(i32 %test_function)
  %8 = alloca i32, align 4
  store i32 %7, i32* %8, align 4
  ; Variable measurement allocated at %8
  %9 = load i32, i32* %8, align 4
  %10 = call i32 @len(i32 %9)
  %11 = call i32 @assert_eq_int(i32 %10, i32 2)
  %12 = alloca [1 x i32], align 4
  %13 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.30, i64 0, i64 0
  %14 = getelementptr inbounds [1 x i32], [1 x i32]* %12, i64 0, i64 0
  store i32 %13, i32* %14, align 4
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %17 = alloca [1 x i32], align 4
  %18 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.31, i64 0, i64 0
  %19 = getelementptr inbounds [1 x i32], [1 x i32]* %17, i64 0, i64 0
  store i32 %18, i32* %19, align 4
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %24 = call i32 @run_all_time_tests()
  ret i32 0
}
