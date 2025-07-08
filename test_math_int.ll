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
define i8* @test_basic_operations() {
entry:
  %0 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  ; Expression result: 5
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %3
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %4
  ; Expression result: 5
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %6
  %7 = call i32 @abs_int(i32 5)
  %8 = call i32 @assert_eq_int(i32 %7, i32 5)
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %9
  %10 = call i32 @abs_int(i32 0)
  %11 = call i32 @assert_eq_int(i32 %10, i32 0)
  ; Expression result: %11
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %12
  ; Expression result: 2147483647
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  ; Expression result: 2147483647
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %15
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %16
  %17 = call i32 @sign_int(i32 5)
  %18 = call i32 @assert_eq_int(i32 %17, i32 1)
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %19
  ; Expression result: 5
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %22 = sub i32 %21, 1
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %24
  %25 = call i32 @sign_int(i32 0)
  %26 = call i32 @assert_eq_int(i32 %25, i32 0)
  ; Expression result: %26
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %27
  %28 = call i32 @min_int(i32 3, i32 7)
  %29 = call i32 @assert_eq_int(i32 %28, i32 3)
  ; Expression result: %29
  %30 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %30
  %31 = call i32 @max_int(i32 3, i32 7)
  %32 = call i32 @assert_eq_int(i32 %31, i32 7)
  ; Expression result: %32
  %33 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %33
  ; Expression result: 5
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %35 = sub i32 %34, 2
  ; Expression result: %35
  %36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %36
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %38 = sub i32 %37, 5
  ; Expression result: %38
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %39
  %40 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %40
  ; Expression result: 5
  %41 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %42 = sub i32 %41, 2
  ; Expression result: %42
  %43 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %43
  %44 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %45 = sub i32 %44, 2
  ; Expression result: %45
  %46 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %46
  %47 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %47
  %48 = call i32 @clamp_int(i32 5, i32 1, i32 10)
  %49 = call i32 @assert_eq_int(i32 %48, i32 5)
  ; Expression result: %49
  %50 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %50
  ; Expression result: 5
  %51 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %51
  ; Expression result: 1
  %52 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %52
  ; Expression result: 10
  %53 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %53
  %54 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %54
  ; Expression result: 1
  %55 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %55
  %56 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %56
  %57 = call i32 @clamp_int(i32 15, i32 1, i32 10)
  %58 = call i32 @assert_eq_int(i32 %57, i32 10)
  ; Expression result: %58
  %59 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %59
  ret i32 0
}

define i8* @test_power_functions() {
entry:
  %0 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.2, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @pow_int(i32 2, i32 0)
  %4 = call i32 @assert_eq_int(i32 %3, i32 1)
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = call i32 @pow_int(i32 2, i32 1)
  %7 = call i32 @assert_eq_int(i32 %6, i32 2)
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = call i32 @pow_int(i32 2, i32 3)
  %10 = call i32 @assert_eq_int(i32 %9, i32 8)
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = call i32 @pow_int(i32 2, i32 10)
  %13 = call i32 @assert_eq_int(i32 %12, i32 1024)
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  %15 = call i32 @pow_int(i32 5, i32 3)
  %16 = call i32 @assert_eq_int(i32 %15, i32 125)
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  ; Expression result: 2
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %18
  ; Expression result: 3
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %21 = sub i32 %20, 8
  ; Expression result: %21
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  ; Expression result: 2
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %24
  ; Expression result: 4
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %26
  ; Expression result: 16
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %27
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %28
  ; Expression result: 1
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %29
  %30 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %30
  ; Expression result: 0
  %31 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %31
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %32
  ; Expression result: 2
  %33 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %33
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %34
  ; Expression result: 0
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %35
  %36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %36
  %37 = call i32 @sqrt_int(i32 0)
  %38 = call i32 @assert_eq_int(i32 %37, i32 0)
  ; Expression result: %38
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %39
  %40 = call i32 @sqrt_int(i32 1)
  %41 = call i32 @assert_eq_int(i32 %40, i32 1)
  ; Expression result: %41
  %42 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %42
  %43 = call i32 @sqrt_int(i32 4)
  %44 = call i32 @assert_eq_int(i32 %43, i32 2)
  ; Expression result: %44
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %45
  %46 = call i32 @sqrt_int(i32 9)
  %47 = call i32 @assert_eq_int(i32 %46, i32 3)
  ; Expression result: %47
  %48 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %48
  %49 = call i32 @sqrt_int(i32 16)
  %50 = call i32 @assert_eq_int(i32 %49, i32 4)
  ; Expression result: %50
  %51 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %51
  %52 = call i32 @sqrt_int(i32 25)
  %53 = call i32 @assert_eq_int(i32 %52, i32 5)
  ; Expression result: %53
  %54 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %54
  %55 = call i32 @sqrt_int(i32 15)
  %56 = call i32 @assert_eq_int(i32 %55, i32 3)
  ; Expression result: %56
  %57 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %57
  %58 = call i32 @sqrt_int(i32 24)
  %59 = call i32 @assert_eq_int(i32 %58, i32 4)
  ; Expression result: %59
  %60 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %60
  ; Expression result: 1
  %61 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %61
  %62 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %63 = sub i32 %62, 1
  ; Expression result: %63
  %64 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %64
  %65 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %65
  ret i32 0
}

define i8* @test_gcd_lcm() {
entry:
  %0 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.3, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @gcd(i32 12, i32 18)
  %4 = call i32 @assert_eq_int(i32 %3, i32 6)
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = call i32 @gcd(i32 48, i32 18)
  %7 = call i32 @assert_eq_int(i32 %6, i32 6)
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = call i32 @gcd(i32 17, i32 13)
  %10 = call i32 @assert_eq_int(i32 %9, i32 1)
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = call i32 @gcd(i32 0, i32 5)
  %13 = call i32 @assert_eq_int(i32 %12, i32 5)
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  %15 = call i32 @gcd(i32 5, i32 0)
  %16 = call i32 @assert_eq_int(i32 %15, i32 5)
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  ; Expression result: 12
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %18
  ; Expression result: 18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  ; Expression result: 6
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %21
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %22
  ; Expression result: 18
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %24
  ; Expression result: 6
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %26
  %27 = call i32 @lcm(i32 12, i32 18)
  %28 = call i32 @assert_eq_int(i32 %27, i32 36)
  ; Expression result: %28
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %29
  %30 = call i32 @lcm(i32 4, i32 6)
  %31 = call i32 @assert_eq_int(i32 %30, i32 12)
  ; Expression result: %31
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %32
  %33 = call i32 @lcm(i32 17, i32 13)
  %34 = call i32 @assert_eq_int(i32 %33, i32 221)
  ; Expression result: %34
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %35
  %36 = call i32 @lcm(i32 0, i32 5)
  %37 = call i32 @assert_eq_int(i32 %36, i32 0)
  ; Expression result: %37
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %38
  %39 = call i32 @lcm(i32 5, i32 0)
  %40 = call i32 @assert_eq_int(i32 %39, i32 0)
  ; Expression result: %40
  %41 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %41
  %42 = call i32 @gcd_extended(i32 48, i32 18)
  %43 = alloca i32, align 4
  store i32 %42, i32* %43, align 4
  ; Variable gcd_result allocated
  %44 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %44
  %45 = alloca [1x i32], align 4
  %46 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.4, i64 0, i64 0
  %47 = getelementptr inbounds [1x i32], [1x i32]* %45, i64 0, i64 0
  store i32 %46, i32* %47, align 4
  ; Expression result: %45
  %48 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %48
  ; Expression result: 6
  %49 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %49
  %50 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %50
  %51 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %51
  %52 = load i32, i32* %43, align 4
  ; Expression result: %52
  %53 = alloca [1x i32], align 4
  %54 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.5, i64 0, i64 0
  %55 = getelementptr inbounds [1x i32], [1x i32]* %53, i64 0, i64 0
  store i32 %54, i32* %55, align 4
  %56 = add i32 %53, 18
  %57 = load i32, i32* %43, align 4
  %58 = mul i32 %56, %57
  ; Expression result: %58
  %59 = alloca [1x i32], align 4
  %60 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.6, i64 0, i64 0
  %61 = getelementptr inbounds [1x i32], [1x i32]* %59, i64 0, i64 0
  store i32 %60, i32* %61, align 4
  ; Expression result: %59
  %62 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %62
  ; Expression result: 6
  %63 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %63
  %64 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %64
  ret i32 0
}

define i8* @test_factorial_combinatorics() {
entry:
  %0 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.7, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @factorial(i32 0)
  %4 = call i32 @assert_eq_int(i32 %3, i32 1)
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = call i32 @factorial(i32 1)
  %7 = call i32 @assert_eq_int(i32 %6, i32 1)
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = call i32 @factorial(i32 2)
  %10 = call i32 @assert_eq_int(i32 %9, i32 2)
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = call i32 @factorial(i32 3)
  %13 = call i32 @assert_eq_int(i32 %12, i32 6)
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  %15 = call i32 @factorial(i32 4)
  %16 = call i32 @assert_eq_int(i32 %15, i32 24)
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = call i32 @factorial(i32 5)
  %19 = call i32 @assert_eq_int(i32 %18, i32 120)
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  %21 = call i32 @factorial(i32 6)
  %22 = call i32 @assert_eq_int(i32 %21, i32 720)
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  ; Expression result: 1
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %24
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %25
  ; Expression result: 0
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %26
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %27
  %28 = call i32 @combination(i32 5, i32 0)
  %29 = call i32 @assert_eq_int(i32 %28, i32 1)
  ; Expression result: %29
  %30 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %30
  %31 = call i32 @combination(i32 5, i32 1)
  %32 = call i32 @assert_eq_int(i32 %31, i32 5)
  ; Expression result: %32
  %33 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %33
  %34 = call i32 @combination(i32 5, i32 2)
  %35 = call i32 @assert_eq_int(i32 %34, i32 10)
  ; Expression result: %35
  %36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %36
  %37 = call i32 @combination(i32 5, i32 3)
  %38 = call i32 @assert_eq_int(i32 %37, i32 10)
  ; Expression result: %38
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %39
  %40 = call i32 @combination(i32 5, i32 4)
  %41 = call i32 @assert_eq_int(i32 %40, i32 5)
  ; Expression result: %41
  %42 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %42
  %43 = call i32 @combination(i32 5, i32 5)
  %44 = call i32 @assert_eq_int(i32 %43, i32 1)
  ; Expression result: %44
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %45
  %46 = call i32 @combination(i32 10, i32 3)
  %47 = call i32 @assert_eq_int(i32 %46, i32 120)
  ; Expression result: %47
  %48 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %48
  %49 = call i32 @combination(i32 5, i32 6)
  %50 = call i32 @assert_eq_int(i32 %49, i32 0)
  ; Expression result: %50
  %51 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %51
  %52 = call i32 @permutation(i32 5, i32 0)
  %53 = call i32 @assert_eq_int(i32 %52, i32 1)
  ; Expression result: %53
  %54 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %54
  %55 = call i32 @permutation(i32 5, i32 1)
  %56 = call i32 @assert_eq_int(i32 %55, i32 5)
  ; Expression result: %56
  %57 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %57
  %58 = call i32 @permutation(i32 5, i32 2)
  %59 = call i32 @assert_eq_int(i32 %58, i32 20)
  ; Expression result: %59
  %60 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %60
  %61 = call i32 @permutation(i32 5, i32 3)
  %62 = call i32 @assert_eq_int(i32 %61, i32 60)
  ; Expression result: %62
  %63 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %63
  %64 = call i32 @permutation(i32 5, i32 5)
  %65 = call i32 @assert_eq_int(i32 %64, i32 120)
  ; Expression result: %65
  %66 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %66
  %67 = call i32 @permutation(i32 5, i32 6)
  %68 = call i32 @assert_eq_int(i32 %67, i32 0)
  ; Expression result: %68
  %69 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %69
  ret i32 0
}

define i8* @test_fibonacci() {
entry:
  %0 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.8, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @fibonacci(i32 0)
  %4 = call i32 @assert_eq_int(i32 %3, i32 0)
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = call i32 @fibonacci(i32 1)
  %7 = call i32 @assert_eq_int(i32 %6, i32 1)
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = call i32 @fibonacci(i32 2)
  %10 = call i32 @assert_eq_int(i32 %9, i32 1)
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = call i32 @fibonacci(i32 3)
  %13 = call i32 @assert_eq_int(i32 %12, i32 2)
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  %15 = call i32 @fibonacci(i32 4)
  %16 = call i32 @assert_eq_int(i32 %15, i32 3)
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = call i32 @fibonacci(i32 5)
  %19 = call i32 @assert_eq_int(i32 %18, i32 5)
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  %21 = call i32 @fibonacci(i32 6)
  %22 = call i32 @assert_eq_int(i32 %21, i32 8)
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  %24 = call i32 @fibonacci(i32 7)
  %25 = call i32 @assert_eq_int(i32 %24, i32 13)
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %26
  %27 = call i32 @fibonacci(i32 8)
  %28 = call i32 @assert_eq_int(i32 %27, i32 21)
  ; Expression result: %28
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %29
  %30 = call i32 @fibonacci(i32 9)
  %31 = call i32 @assert_eq_int(i32 %30, i32 34)
  ; Expression result: %31
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %32
  %33 = call i32 @fibonacci(i32 10)
  %34 = call i32 @assert_eq_int(i32 %33, i32 55)
  ; Expression result: %34
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %35
  ; Expression result: 1
  %36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %36
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %37
  ; Expression result: 0
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %38
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %39
  %40 = call i32 @fibonacci(i32 15)
  %41 = call i32 @assert_eq_int(i32 %40, i32 610)
  ; Expression result: %41
  %42 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %42
  %43 = call i32 @fibonacci(i32 20)
  %44 = call i32 @assert_eq_int(i32 %43, i32 6765)
  ; Expression result: %44
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %45
  ret i32 0
}

define i8* @test_prime_numbers() {
entry:
  %0 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.9, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @is_prime(i32 0)
  %4 = call i32 @assert_false(i32 %3)
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = call i32 @is_prime(i32 1)
  %7 = call i32 @assert_false(i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = call i32 @is_prime(i32 2)
  %10 = call i32 @assert_true(i32 %9)
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = call i32 @is_prime(i32 3)
  %13 = call i32 @assert_true(i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  %15 = call i32 @is_prime(i32 4)
  %16 = call i32 @assert_false(i32 %15)
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = call i32 @is_prime(i32 5)
  %19 = call i32 @assert_true(i32 %18)
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  %21 = call i32 @is_prime(i32 6)
  %22 = call i32 @assert_false(i32 %21)
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  %24 = call i32 @is_prime(i32 7)
  %25 = call i32 @assert_true(i32 %24)
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %26
  %27 = call i32 @is_prime(i32 8)
  %28 = call i32 @assert_false(i32 %27)
  ; Expression result: %28
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %29
  %30 = call i32 @is_prime(i32 9)
  %31 = call i32 @assert_false(i32 %30)
  ; Expression result: %31
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %32
  %33 = call i32 @is_prime(i32 10)
  %34 = call i32 @assert_false(i32 %33)
  ; Expression result: %34
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %35
  %36 = call i32 @is_prime(i32 11)
  %37 = call i32 @assert_true(i32 %36)
  ; Expression result: %37
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %38
  %39 = call i32 @is_prime(i32 13)
  %40 = call i32 @assert_true(i32 %39)
  ; Expression result: %40
  %41 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %41
  %42 = call i32 @is_prime(i32 17)
  %43 = call i32 @assert_true(i32 %42)
  ; Expression result: %43
  %44 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %44
  %45 = call i32 @is_prime(i32 19)
  %46 = call i32 @assert_true(i32 %45)
  ; Expression result: %46
  %47 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %47
  %48 = call i32 @is_prime(i32 23)
  %49 = call i32 @assert_true(i32 %48)
  ; Expression result: %49
  %50 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %50
  %51 = call i32 @is_prime(i32 25)
  %52 = call i32 @assert_false(i32 %51)
  ; Expression result: %52
  %53 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %53
  %54 = call i32 @is_prime(i32 27)
  %55 = call i32 @assert_false(i32 %54)
  ; Expression result: %55
  %56 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %56
  %57 = call i32 @is_prime(i32 29)
  %58 = call i32 @assert_true(i32 %57)
  ; Expression result: %58
  %59 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %59
  %60 = call i32 @is_prime(i32 31)
  %61 = call i32 @assert_true(i32 %60)
  ; Expression result: %61
  %62 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %62
  %63 = call i32 @next_prime(i32 1)
  %64 = call i32 @assert_eq_int(i32 %63, i32 2)
  ; Expression result: %64
  %65 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %65
  %66 = call i32 @next_prime(i32 2)
  %67 = call i32 @assert_eq_int(i32 %66, i32 3)
  ; Expression result: %67
  %68 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %68
  %69 = call i32 @next_prime(i32 3)
  %70 = call i32 @assert_eq_int(i32 %69, i32 5)
  ; Expression result: %70
  %71 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %71
  %72 = call i32 @next_prime(i32 10)
  %73 = call i32 @assert_eq_int(i32 %72, i32 11)
  ; Expression result: %73
  %74 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %74
  %75 = call i32 @next_prime(i32 13)
  %76 = call i32 @assert_eq_int(i32 %75, i32 17)
  ; Expression result: %76
  %77 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %77
  %78 = call i32 @next_prime(i32 20)
  %79 = call i32 @assert_eq_int(i32 %78, i32 23)
  ; Expression result: %79
  %80 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %80
  %81 = call i32 @prime_factors(i32 2)
  %82 = alloca i32, align 4
  store i32 %81, i32* %82, align 4
  ; Variable factors2 allocated
  %83 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %83
  %84 = load i32, i32* %82, align 4
  ; Member access: %84.length
  %85 = getelementptr inbounds %struct.object, %struct.object* %84, i32 0, i32 0
  %86 = load i32, i32* %85, align 4
  %87 = call i32 @assert_eq_int(i32 %86, i32 1)
  ; Expression result: %87
  %88 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %88
  %89 = alloca [1x i32], align 4
  %90 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.4, i64 0, i64 0
  %91 = getelementptr inbounds [1x i32], [1x i32]* %89, i64 0, i64 0
  store i32 %90, i32* %91, align 4
  ; Expression result: %89
  %92 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %92
  ; Expression result: 2
  %93 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %93
  %94 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %94
  %95 = call i32 @prime_factors(i32 12)
  %96 = alloca i32, align 4
  store i32 %95, i32* %96, align 4
  ; Variable factors12 allocated
  %97 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %97
  %98 = load i32, i32* %96, align 4
  ; Member access: %98.length
  %99 = getelementptr inbounds %struct.object, %struct.object* %98, i32 0, i32 0
  %100 = load i32, i32* %99, align 4
  %101 = call i32 @assert_eq_int(i32 %100, i32 3)
  ; Expression result: %101
  %102 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %102
  %103 = alloca [1x i32], align 4
  %104 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.4, i64 0, i64 0
  %105 = getelementptr inbounds [1x i32], [1x i32]* %103, i64 0, i64 0
  store i32 %104, i32* %105, align 4
  ; Expression result: %103
  %106 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %106
  ; Expression result: 2
  %107 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %107
  %108 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %108
  %109 = alloca [1x i32], align 4
  %110 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.5, i64 0, i64 0
  %111 = getelementptr inbounds [1x i32], [1x i32]* %109, i64 0, i64 0
  store i32 %110, i32* %111, align 4
  ; Expression result: %109
  %112 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %112
  ; Expression result: 2
  %113 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %113
  %114 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %114
  %115 = alloca [1x i32], align 4
  %116 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.6, i64 0, i64 0
  %117 = getelementptr inbounds [1x i32], [1x i32]* %115, i64 0, i64 0
  store i32 %116, i32* %117, align 4
  ; Expression result: %115
  %118 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %118
  ; Expression result: 3
  %119 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %119
  %120 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %120
  %121 = call i32 @prime_factors(i32 30)
  %122 = alloca i32, align 4
  store i32 %121, i32* %122, align 4
  ; Variable factors30 allocated
  %123 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %123
  %124 = load i32, i32* %122, align 4
  ; Member access: %124.length
  %125 = getelementptr inbounds %struct.object, %struct.object* %124, i32 0, i32 0
  %126 = load i32, i32* %125, align 4
  %127 = call i32 @assert_eq_int(i32 %126, i32 3)
  ; Expression result: %127
  %128 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %128
  %129 = alloca [1x i32], align 4
  %130 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.4, i64 0, i64 0
  %131 = getelementptr inbounds [1x i32], [1x i32]* %129, i64 0, i64 0
  store i32 %130, i32* %131, align 4
  ; Expression result: %129
  %132 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %132
  ; Expression result: 2
  %133 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %133
  %134 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %134
  %135 = alloca [1x i32], align 4
  %136 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.5, i64 0, i64 0
  %137 = getelementptr inbounds [1x i32], [1x i32]* %135, i64 0, i64 0
  store i32 %136, i32* %137, align 4
  ; Expression result: %135
  %138 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %138
  ; Expression result: 3
  %139 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %139
  %140 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %140
  %141 = alloca [1x i32], align 4
  %142 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.6, i64 0, i64 0
  %143 = getelementptr inbounds [1x i32], [1x i32]* %141, i64 0, i64 0
  store i32 %142, i32* %143, align 4
  ; Expression result: %141
  %144 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %144
  ; Expression result: 5
  %145 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %145
  %146 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %146
  ret i32 0
}

define i8* @test_modular_arithmetic() {
entry:
  %0 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.10, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @mod_add(i32 5, i32 3, i32 7)
  %4 = call i32 @assert_eq_int(i32 %3, i32 1)
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = call i32 @mod_add(i32 10, i32 15, i32 12)
  %7 = call i32 @assert_eq_int(i32 %6, i32 1)
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = call i32 @mod_sub(i32 5, i32 3, i32 7)
  %10 = call i32 @assert_eq_int(i32 %9, i32 2)
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = call i32 @mod_sub(i32 3, i32 5, i32 7)
  %13 = call i32 @assert_eq_int(i32 %12, i32 5)
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  %15 = call i32 @mod_mul(i32 5, i32 3, i32 7)
  %16 = call i32 @assert_eq_int(i32 %15, i32 1)
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = call i32 @mod_mul(i32 6, i32 7, i32 11)
  %19 = call i32 @assert_eq_int(i32 %18, i32 9)
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  %21 = call i32 @mod_pow(i32 2, i32 3, i32 5)
  %22 = call i32 @assert_eq_int(i32 %21, i32 3)
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  %24 = call i32 @mod_pow(i32 3, i32 4, i32 7)
  %25 = call i32 @assert_eq_int(i32 %24, i32 4)
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %26
  %27 = call i32 @mod_pow(i32 5, i32 0, i32 13)
  %28 = call i32 @assert_eq_int(i32 %27, i32 1)
  ; Expression result: %28
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %29
  %30 = call i32 @mod_inverse(i32 3, i32 7)
  %31 = call i32 @assert_eq_int(i32 %30, i32 5)
  ; Expression result: %31
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %32
  %33 = call i32 @mod_inverse(i32 5, i32 7)
  %34 = call i32 @assert_eq_int(i32 %33, i32 3)
  ; Expression result: %34
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %35
  ; Expression result: 1
  %36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %36
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %37
  ret i32 0
}

define i8* @test_number_theory() {
entry:
  %0 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.11, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @euler_totient(i32 1)
  %4 = call i32 @assert_eq_int(i32 %3, i32 1)
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = call i32 @euler_totient(i32 2)
  %7 = call i32 @assert_eq_int(i32 %6, i32 1)
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = call i32 @euler_totient(i32 3)
  %10 = call i32 @assert_eq_int(i32 %9, i32 2)
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = call i32 @euler_totient(i32 4)
  %13 = call i32 @assert_eq_int(i32 %12, i32 2)
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  %15 = call i32 @euler_totient(i32 5)
  %16 = call i32 @assert_eq_int(i32 %15, i32 4)
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = call i32 @euler_totient(i32 6)
  %19 = call i32 @assert_eq_int(i32 %18, i32 2)
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  %21 = call i32 @euler_totient(i32 7)
  %22 = call i32 @assert_eq_int(i32 %21, i32 6)
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  %24 = call i32 @euler_totient(i32 8)
  %25 = call i32 @assert_eq_int(i32 %24, i32 4)
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %26
  %27 = call i32 @euler_totient(i32 9)
  %28 = call i32 @assert_eq_int(i32 %27, i32 6)
  ; Expression result: %28
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %29
  %30 = call i32 @euler_totient(i32 10)
  %31 = call i32 @assert_eq_int(i32 %30, i32 4)
  ; Expression result: %31
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %32
  %33 = alloca [3x i32], align 4
  %34 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.6, i64 0, i64 0
  %35 = getelementptr inbounds [3x i32], [3x i32]* %33, i64 0, i64 0
  store i32 %34, i32* %35, align 4
  %36 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.12, i64 0, i64 0
  %37 = getelementptr inbounds [3x i32], [3x i32]* %33, i64 0, i64 1
  store i32 %36, i32* %37, align 4
  %38 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.6, i64 0, i64 0
  %39 = getelementptr inbounds [3x i32], [3x i32]* %33, i64 0, i64 2
  store i32 %38, i32* %39, align 4
  %40 = alloca [3 x i32]*, align 4
  store [3 x i32]* %33, [3 x i32]** %40, align 4
  ; Variable remainders allocated
  %41 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %41
  %42 = alloca [3x i32], align 4
  %43 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.12, i64 0, i64 0
  %44 = getelementptr inbounds [3x i32], [3x i32]* %42, i64 0, i64 0
  store i32 %43, i32* %44, align 4
  %45 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.13, i64 0, i64 0
  %46 = getelementptr inbounds [3x i32], [3x i32]* %42, i64 0, i64 1
  store i32 %45, i32* %46, align 4
  %47 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.14, i64 0, i64 0
  %48 = getelementptr inbounds [3x i32], [3x i32]* %42, i64 0, i64 2
  store i32 %47, i32* %48, align 4
  %49 = alloca [3 x i32]*, align 4
  store [3 x i32]* %42, [3 x i32]** %49, align 4
  ; Variable moduli allocated
  %50 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %50
  %51 = load [3 x i32]*, [3 x i32]** %40, align 4
  %52 = load [3 x i32]*, [3 x i32]** %49, align 4
  %53 = call i32 @chinese_remainder(i32 %51, i32 %52)
  %54 = alloca i32, align 4
  store i32 %53, i32* %54, align 4
  ; Variable result allocated
  %55 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %55
  %56 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %56
  ; Expression result: 3
  %57 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %57
  ; Expression result: 2
  %58 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %58
  %59 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %59
  %60 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %60
  ; Expression result: 5
  %61 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %61
  ; Expression result: 3
  %62 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %62
  %63 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %63
  %64 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %64
  ; Expression result: 7
  %65 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %65
  ; Expression result: 2
  %66 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %66
  %67 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %67
  ret i32 0
}

define i8* @test_bitwise_operations() {
entry:
  %0 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.15, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @popcount(i32 0)
  %4 = call i32 @assert_eq_int(i32 %3, i32 0)
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = call i32 @popcount(i32 1)
  %7 = call i32 @assert_eq_int(i32 %6, i32 1)
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = call i32 @popcount(i32 3)
  %10 = call i32 @assert_eq_int(i32 %9, i32 2)
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = call i32 @popcount(i32 7)
  %13 = call i32 @assert_eq_int(i32 %12, i32 3)
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  %15 = call i32 @popcount(i32 15)
  %16 = call i32 @assert_eq_int(i32 %15, i32 4)
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = call i32 @leading_zeros(i32 0)
  %19 = call i32 @assert_eq_int(i32 %18, i32 32)
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  %21 = call i32 @leading_zeros(i32 1)
  %22 = call i32 @assert_eq_int(i32 %21, i32 31)
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  %24 = call i32 @leading_zeros(i32 2)
  %25 = call i32 @assert_eq_int(i32 %24, i32 30)
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %26
  %27 = call i32 @leading_zeros(i32 4)
  %28 = call i32 @assert_eq_int(i32 %27, i32 29)
  ; Expression result: %28
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %29
  %30 = call i32 @trailing_zeros(i32 0)
  %31 = call i32 @assert_eq_int(i32 %30, i32 32)
  ; Expression result: %31
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %32
  %33 = call i32 @trailing_zeros(i32 1)
  %34 = call i32 @assert_eq_int(i32 %33, i32 0)
  ; Expression result: %34
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %35
  %36 = call i32 @trailing_zeros(i32 2)
  %37 = call i32 @assert_eq_int(i32 %36, i32 1)
  ; Expression result: %37
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %38
  %39 = call i32 @trailing_zeros(i32 4)
  %40 = call i32 @assert_eq_int(i32 %39, i32 2)
  ; Expression result: %40
  %41 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %41
  %42 = call i32 @trailing_zeros(i32 8)
  %43 = call i32 @assert_eq_int(i32 %42, i32 3)
  ; Expression result: %43
  %44 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %44
  %45 = call i32 @reverse_bits(i32 0)
  %46 = call i32 @assert_eq_int(i32 %45, i32 0)
  ; Expression result: %46
  %47 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %47
  ; Expression result: 2147483648
  %48 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %48
  %49 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %49
  ; Expression result: 2147483648
  %50 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %50
  %51 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %51
  ; Expression result: 1
  %52 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %52
  %53 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %53
  ret i32 0
}

define i8* @test_digital_operations() {
entry:
  %0 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.16, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @digital_root(i32 0)
  %4 = call i32 @assert_eq_int(i32 %3, i32 0)
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = call i32 @digital_root(i32 9)
  %7 = call i32 @assert_eq_int(i32 %6, i32 9)
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = call i32 @digital_root(i32 10)
  %10 = call i32 @assert_eq_int(i32 %9, i32 1)
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = call i32 @digital_root(i32 11)
  %13 = call i32 @assert_eq_int(i32 %12, i32 2)
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  %15 = call i32 @digital_root(i32 38)
  %16 = call i32 @assert_eq_int(i32 %15, i32 2)
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = call i32 @digital_root(i32 123)
  %19 = call i32 @assert_eq_int(i32 %18, i32 6)
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  %21 = call i32 @digit_sum(i32 0)
  %22 = call i32 @assert_eq_int(i32 %21, i32 0)
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  %24 = call i32 @digit_sum(i32 123)
  %25 = call i32 @assert_eq_int(i32 %24, i32 6)
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %26
  %27 = call i32 @digit_sum(i32 456)
  %28 = call i32 @assert_eq_int(i32 %27, i32 15)
  ; Expression result: %28
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %29
  %30 = call i32 @digit_sum(i32 999)
  %31 = call i32 @assert_eq_int(i32 %30, i32 27)
  ; Expression result: %31
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %32
  %33 = call i32 @digit_product(i32 0)
  %34 = call i32 @assert_eq_int(i32 %33, i32 0)
  ; Expression result: %34
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %35
  %36 = call i32 @digit_product(i32 123)
  %37 = call i32 @assert_eq_int(i32 %36, i32 6)
  ; Expression result: %37
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %38
  %39 = call i32 @digit_product(i32 456)
  %40 = call i32 @assert_eq_int(i32 %39, i32 120)
  ; Expression result: %40
  %41 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %41
  %42 = call i32 @digit_product(i32 505)
  %43 = call i32 @assert_eq_int(i32 %42, i32 0)
  ; Expression result: %43
  %44 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %44
  ret i32 0
}

define i8* @test_perfect_numbers() {
entry:
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.17, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @is_perfect(i32 1)
  %4 = call i32 @assert_false(i32 %3)
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = call i32 @is_perfect(i32 2)
  %7 = call i32 @assert_false(i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = call i32 @is_perfect(i32 5)
  %10 = call i32 @assert_false(i32 %9)
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = call i32 @is_perfect(i32 6)
  %13 = call i32 @assert_true(i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  %15 = call i32 @is_perfect(i32 10)
  %16 = call i32 @assert_false(i32 %15)
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = call i32 @is_perfect(i32 28)
  %19 = call i32 @assert_true(i32 %18)
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  %21 = call i32 @sum_proper_divisors(i32 6)
  %22 = call i32 @assert_eq_int(i32 %21, i32 6)
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  %24 = call i32 @sum_proper_divisors(i32 12)
  %25 = call i32 @assert_eq_int(i32 %24, i32 16)
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %26
  %27 = call i32 @sum_proper_divisors(i32 28)
  %28 = call i32 @assert_eq_int(i32 %27, i32 28)
  ; Expression result: %28
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %29
  %30 = call i32 @is_abundant(i32 6)
  %31 = call i32 @assert_false(i32 %30)
  ; Expression result: %31
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %32
  %33 = call i32 @is_abundant(i32 12)
  %34 = call i32 @assert_true(i32 %33)
  ; Expression result: %34
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %35
  %36 = call i32 @is_abundant(i32 18)
  %37 = call i32 @assert_true(i32 %36)
  ; Expression result: %37
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %38
  %39 = call i32 @is_deficient(i32 1)
  %40 = call i32 @assert_true(i32 %39)
  ; Expression result: %40
  %41 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %41
  %42 = call i32 @is_deficient(i32 2)
  %43 = call i32 @assert_true(i32 %42)
  ; Expression result: %43
  %44 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %44
  %45 = call i32 @is_deficient(i32 4)
  %46 = call i32 @assert_true(i32 %45)
  ; Expression result: %46
  %47 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %47
  %48 = call i32 @is_deficient(i32 8)
  %49 = call i32 @assert_true(i32 %48)
  ; Expression result: %49
  %50 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %50
  %51 = call i32 @is_deficient(i32 6)
  %52 = call i32 @assert_false(i32 %51)
  ; Expression result: %52
  %53 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %53
  %54 = call i32 @is_deficient(i32 12)
  %55 = call i32 @assert_false(i32 %54)
  ; Expression result: %55
  %56 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %56
  ret i32 0
}

define i8* @test_collatz() {
entry:
  %0 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.18, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @collatz_length(i32 1)
  %4 = call i32 @assert_eq_int(i32 %3, i32 0)
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = call i32 @collatz_length(i32 2)
  %7 = call i32 @assert_eq_int(i32 %6, i32 1)
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = call i32 @collatz_length(i32 3)
  %10 = call i32 @assert_eq_int(i32 %9, i32 7)
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = call i32 @collatz_length(i32 4)
  %13 = call i32 @assert_eq_int(i32 %12, i32 2)
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  %15 = call i32 @collatz_length(i32 5)
  %16 = call i32 @assert_eq_int(i32 %15, i32 5)
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = call i32 @collatz_length(i32 6)
  %19 = call i32 @assert_eq_int(i32 %18, i32 8)
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  %21 = call i32 @collatz_length(i32 7)
  %22 = call i32 @assert_eq_int(i32 %21, i32 16)
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  %24 = call i32 @collatz_length(i32 0)
  %25 = call i32 @assert_eq_int(i32 %24, i32 0)
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %26
  ; Expression result: 1
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %27
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %28
  ; Expression result: 0
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %29
  %30 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %30
  ret i32 0
}

define i8* @test_base_conversion() {
entry:
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.19, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @to_base(i32 0, i32 2)
  %4 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.4, i64 0, i64 0
  %5 = call i32 @assert_eq_string(i32 %3, i32 %4)
  ; Expression result: %5
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %6
  %7 = call i32 @to_base(i32 5, i32 2)
  %8 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.20, i64 0, i64 0
  %9 = call i32 @assert_eq_string(i32 %7, i32 %8)
  ; Expression result: %9
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %10
  %11 = call i32 @to_base(i32 10, i32 2)
  %12 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.21, i64 0, i64 0
  %13 = call i32 @assert_eq_string(i32 %11, i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  %15 = call i32 @to_base(i32 255, i32 2)
  %16 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.22, i64 0, i64 0
  %17 = call i32 @assert_eq_string(i32 %15, i32 %16)
  ; Expression result: %17
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %18
  %19 = call i32 @to_base(i32 10, i32 10)
  %20 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.23, i64 0, i64 0
  %21 = call i32 @assert_eq_string(i32 %19, i32 %20)
  ; Expression result: %21
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %22
  %23 = call i32 @to_base(i32 255, i32 16)
  %24 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.24, i64 0, i64 0
  %25 = call i32 @assert_eq_string(i32 %23, i32 %24)
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %26
  %27 = call i32 @to_base(i32 26, i32 26)
  %28 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.23, i64 0, i64 0
  %29 = call i32 @assert_eq_string(i32 %27, i32 %28)
  ; Expression result: %29
  %30 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %30
  %31 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.4, i64 0, i64 0
  %32 = call i32 @from_base(i32 %31, i32 2)
  %33 = call i32 @assert_eq_int(i32 %32, i32 0)
  ; Expression result: %33
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %34
  %35 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.20, i64 0, i64 0
  %36 = call i32 @from_base(i32 %35, i32 2)
  %37 = call i32 @assert_eq_int(i32 %36, i32 5)
  ; Expression result: %37
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %38
  %39 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.21, i64 0, i64 0
  %40 = call i32 @from_base(i32 %39, i32 2)
  %41 = call i32 @assert_eq_int(i32 %40, i32 10)
  ; Expression result: %41
  %42 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %42
  %43 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.22, i64 0, i64 0
  %44 = call i32 @from_base(i32 %43, i32 2)
  %45 = call i32 @assert_eq_int(i32 %44, i32 255)
  ; Expression result: %45
  %46 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %46
  %47 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.23, i64 0, i64 0
  %48 = call i32 @from_base(i32 %47, i32 10)
  %49 = call i32 @assert_eq_int(i32 %48, i32 10)
  ; Expression result: %49
  %50 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %50
  %51 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.24, i64 0, i64 0
  %52 = call i32 @from_base(i32 %51, i32 16)
  %53 = call i32 @assert_eq_int(i32 %52, i32 255)
  ; Expression result: %53
  %54 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %54
  %55 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.25, i64 0, i64 0
  %56 = call i32 @from_base(i32 %55, i32 16)
  %57 = call i32 @assert_eq_int(i32 %56, i32 255)
  ; Expression result: %57
  %58 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %58
  %59 = call i32 @to_base(i32 42, i32 2)
  %60 = call i32 @from_base(i32 %59, i32 2)
  %61 = call i32 @assert_eq_int(i32 %60, i32 42)
  ; Expression result: %61
  %62 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %62
  %63 = call i32 @to_base(i32 100, i32 8)
  %64 = call i32 @from_base(i32 %63, i32 8)
  %65 = call i32 @assert_eq_int(i32 %64, i32 100)
  ; Expression result: %65
  %66 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %66
  %67 = call i32 @to_base(i32 255, i32 16)
  %68 = call i32 @from_base(i32 %67, i32 16)
  %69 = call i32 @assert_eq_int(i32 %68, i32 255)
  ; Expression result: %69
  %70 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %70
  ret i32 0
}

define i8* @test_edge_cases() {
entry:
  %0 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.26, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @factorial(i32 10)
  %4 = call i32 @assert_eq_int(i32 %3, i32 3628800)
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = call i32 @factorial(i32 12)
  %7 = call i32 @assert_eq_int(i32 %6, i32 479001600)
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = call i32 @fibonacci(i32 25)
  %10 = call i32 @assert_eq_int(i32 %9, i32 75025)
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = call i32 @fibonacci(i32 30)
  %13 = call i32 @assert_eq_int(i32 %12, i32 832040)
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  %15 = call i32 @pow_int(i32 2, i32 20)
  %16 = call i32 @assert_eq_int(i32 %15, i32 1048576)
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = call i32 @pow_int(i32 3, i32 10)
  %19 = call i32 @assert_eq_int(i32 %18, i32 59049)
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  %21 = call i32 @gcd(i32 1071, i32 462)
  %22 = call i32 @assert_eq_int(i32 %21, i32 21)
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  %24 = call i32 @gcd(i32 123456, i32 789012)
  %25 = call i32 @assert_eq_int(i32 %24, i32 12)
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %26
  %27 = call i32 @sqrt_int(i32 1000000)
  %28 = call i32 @assert_eq_int(i32 %27, i32 1000)
  ; Expression result: %28
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %29
  %30 = call i32 @sqrt_int(i32 999999)
  %31 = call i32 @assert_eq_int(i32 %30, i32 999)
  ; Expression result: %31
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %32
  %33 = call i32 @sqrt_int(i32 1000001)
  %34 = call i32 @assert_eq_int(i32 %33, i32 1000)
  ; Expression result: %34
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %35
  ret i32 0
}


; String constants
@.str.9 = private unnamed_addr constant [14 x i8] c"Prime Numbers\00", align 1
@.str.25 = private unnamed_addr constant [3 x i8] c"ff\00", align 1
@.str.17 = private unnamed_addr constant [16 x i8] c"Perfect Numbers\00", align 1
@.str.20 = private unnamed_addr constant [4 x i8] c"101\00", align 1
@.str.23 = private unnamed_addr constant [3 x i8] c"10\00", align 1
@.str.26 = private unnamed_addr constant [29 x i8] c"Edge Cases and Large Numbers\00", align 1
@.str.1 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.8 = private unnamed_addr constant [19 x i8] c"Fibonacci Sequence\00", align 1
@.str.18 = private unnamed_addr constant [19 x i8] c"Collatz Conjecture\00", align 1
@.str.22 = private unnamed_addr constant [9 x i8] c"11111111\00", align 1
@.str.7 = private unnamed_addr constant [28 x i8] c"Factorial and Combinatorics\00", align 1
@.str.12 = private unnamed_addr constant [2 x i8] c"3\00", align 1
@.str.0 = private unnamed_addr constant [25 x i8] c"Basic Integer Operations\00", align 1
@.str.2 = private unnamed_addr constant [24 x i8] c"Integer Power Functions\00", align 1
@.str.10 = private unnamed_addr constant [19 x i8] c"Modular Arithmetic\00", align 1
@.str.21 = private unnamed_addr constant [5 x i8] c"1010\00", align 1
@.str.5 = private unnamed_addr constant [2 x i8] c"1\00", align 1
@.str.15 = private unnamed_addr constant [19 x i8] c"Bitwise Operations\00", align 1
@.str.11 = private unnamed_addr constant [14 x i8] c"Number Theory\00", align 1
@.str.4 = private unnamed_addr constant [2 x i8] c"0\00", align 1
@.str.3 = private unnamed_addr constant [12 x i8] c"GCD and LCM\00", align 1
@.str.13 = private unnamed_addr constant [2 x i8] c"5\00", align 1
@.str.16 = private unnamed_addr constant [19 x i8] c"Digital Operations\00", align 1
@.str.6 = private unnamed_addr constant [2 x i8] c"2\00", align 1
@.str.19 = private unnamed_addr constant [16 x i8] c"Base Conversion\00", align 1
@.str.24 = private unnamed_addr constant [3 x i8] c"FF\00", align 1
@.str.14 = private unnamed_addr constant [2 x i8] c"7\00", align 1
define i32 @main() {
entry:
  %0 = call i32 @test_basic_operations()
  ; Expression result: %0
  %1 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %1
  %2 = call i32 @test_power_functions()
  ; Expression result: %2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %3
  %4 = call i32 @test_gcd_lcm()
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = call i32 @test_factorial_combinatorics()
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %7
  %8 = call i32 @test_fibonacci()
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %9
  %10 = call i32 @test_prime_numbers()
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = call i32 @test_modular_arithmetic()
  ; Expression result: %12
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %13
  %14 = call i32 @test_number_theory()
  ; Expression result: %14
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %15
  %16 = call i32 @test_bitwise_operations()
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = call i32 @test_digital_operations()
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %19
  %20 = call i32 @test_perfect_numbers()
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %21
  %22 = call i32 @test_collatz()
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  %24 = call i32 @test_base_conversion()
  ; Expression result: %24
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %25
  %26 = call i32 @test_edge_cases()
  ; Expression result: %26
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %27
  %28 = call i32 @print_test_summary()
  ; Expression result: %28
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %29
  ret i32 0
}

