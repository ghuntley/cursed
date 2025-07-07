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

define i32 @test_basic_spill() {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = add i32 0, 0 ; placeholder
  %7 = call i32 @puts(i8* %6)
  %8 = add i32 0, 0
  ; Expression result: %8
  %9 = add i32 0, 0 ; placeholder
  %10 = call i32 @puts(i8* %9)
  %11 = add i32 0, 0
  ; Expression result: %11
  %12 = add i32 0, 0 ; placeholder
  %13 = call i32 @puts(i8* %12)
  %14 = add i32 0, 0
  ; Expression result: %14
  %15 = add i32 0, 0 ; placeholder
  %16 = call i32 @puts(i8* %15)
  %17 = add i32 0, 0
  ; Expression result: %17
  %18 = add i32 0, 0 ; placeholder
  %19 = call i32 @puts(i8* %18)
  %20 = add i32 0, 0
  ; Expression result: %20
  %21 = add i32 0, 0 ; placeholder
  %22 = call i32 @puts(i8* %21)
  %23 = add i32 0, 0
  ; Expression result: %23
  %24 = add i32 0, 0 ; placeholder
  %25 = call i32 @puts(i8* %24)
  %26 = add i32 0, 0
  ; Expression result: %26
  %27 = add i32 0, 0 ; placeholder
  %28 = call i32 @puts(i8* %27)
  %29 = add i32 0, 0
  ; Expression result: %29
  %30 = add i32 0, 0 ; placeholder
  %31 = call i32 @puts(i8* %30)
  %32 = add i32 0, 0
  ; Expression result: %32
  ret i32 0
}

define i32 @test_format_functions() {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = add i32 0, 0 ; placeholder
  %7 = alloca i32, align 4
  store i32 %6, i32* %7, align 4
  ; Variable int_val allocated
  %8 = add i32 0, 0 ; placeholder
  %9 = alloca i8*, align 4
  store i8* %8, i8** %9, align 4
  ; Variable int_str allocated
  %10 = add i32 0, 0 ; placeholder
  %11 = call i32 @puts(i8* %10)
  %12 = add i32 0, 0
  ; Expression result: %12
  %13 = load i8*, i8** %9, align 4
  %14 = call i32 @puts(i8* %13)
  %15 = add i32 0, 0
  ; Expression result: %15
  %16 = add i32 0, 0 ; placeholder
  %17 = call i32 @puts(i8* %16)
  %18 = add i32 0, 0
  ; Expression result: %18
  %19 = add i32 0, 0 ; placeholder
  %20 = alloca i1, align 4
  store i1 %19, i1* %20, align 4
  ; Variable bool_val allocated
  %21 = add i32 0, 0 ; placeholder
  %22 = alloca i8*, align 4
  store i8* %21, i8** %22, align 4
  ; Variable bool_str allocated
  %23 = add i32 0, 0 ; placeholder
  %24 = call i32 @puts(i8* %23)
  %25 = add i32 0, 0
  ; Expression result: %25
  %26 = load i8*, i8** %22, align 4
  %27 = call i32 @puts(i8* %26)
  %28 = add i32 0, 0
  ; Expression result: %28
  %29 = add i32 0, 0 ; placeholder
  %30 = call i32 @puts(i8* %29)
  %31 = add i32 0, 0
  ; Expression result: %31
  ret i32 0
}

define i32 @test_utility_functions() {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = add i32 0, 0 ; placeholder
  %7 = alloca i8*, align 4
  store i8* %6, i8** %7, align 4
  ; Variable stars allocated
  %8 = add i32 0, 0 ; placeholder
  %9 = call i32 @puts(i8* %8)
  %10 = add i32 0, 0
  ; Expression result: %10
  %11 = load i8*, i8** %7, align 4
  %12 = call i32 @puts(i8* %11)
  %13 = add i32 0, 0
  ; Expression result: %13
  %14 = add i32 0, 0 ; placeholder
  %15 = call i32 @puts(i8* %14)
  %16 = add i32 0, 0
  ; Expression result: %16
  %17 = add i32 0, 0 ; placeholder
  %18 = alloca i8*, align 4
  store i8* %17, i8** %18, align 4
  ; Variable padded allocated
  %19 = add i32 0, 0 ; placeholder
  %20 = call i32 @puts(i8* %19)
  %21 = add i32 0, 0
  ; Expression result: %21
  %22 = load i8*, i8** %18, align 4
  %23 = call i32 @puts(i8* %22)
  %24 = add i32 0, 0
  ; Expression result: %24
  %25 = add i32 0, 0 ; placeholder
  %26 = call i32 @puts(i8* %25)
  %27 = add i32 0, 0
  ; Expression result: %27
  ret i32 0
}

define i32 @test_color_functions() {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = add i32 0, 0 ; placeholder
  %7 = alloca i8*, align 4
  store i8* %6, i8** %7, align 4
  ; Variable red_text allocated
  %8 = add i32 0, 0 ; placeholder
  %9 = alloca i8*, align 4
  store i8* %8, i8** %9, align 4
  ; Variable green_text allocated
  %10 = add i32 0, 0 ; placeholder
  %11 = alloca i8*, align 4
  store i8* %10, i8** %11, align 4
  ; Variable blue_text allocated
  %12 = add i32 0, 0 ; placeholder
  %13 = call i32 @puts(i8* %12)
  %14 = add i32 0, 0
  ; Expression result: %14
  %15 = load i8*, i8** %7, align 4
  %16 = call i32 @puts(i8* %15)
  %17 = add i32 0, 0
  ; Expression result: %17
  %18 = add i32 0, 0 ; placeholder
  %19 = call i32 @puts(i8* %18)
  %20 = add i32 0, 0
  ; Expression result: %20
  %21 = load i8*, i8** %9, align 4
  %22 = call i32 @puts(i8* %21)
  %23 = add i32 0, 0
  ; Expression result: %23
  %24 = add i32 0, 0 ; placeholder
  %25 = call i32 @puts(i8* %24)
  %26 = add i32 0, 0
  ; Expression result: %26
  %27 = load i8*, i8** %11, align 4
  %28 = call i32 @puts(i8* %27)
  %29 = add i32 0, 0
  ; Expression result: %29
  %30 = add i32 0, 0 ; placeholder
  %31 = call i32 @puts(i8* %30)
  %32 = add i32 0, 0
  ; Expression result: %32
  ret i32 0
}

define i32 @test_debug_functions() {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = add i32 0, 0 ; placeholder
  %7 = alloca i8*, align 4
  store i8* %6, i8** %7, align 4
  ; Variable debug_msg allocated
  %8 = load i8*, i8** %7, align 4
  %9 = call i32 @puts(i8* %8)
  %10 = add i32 0, 0
  ; Expression result: %10
  %11 = add i32 0, 0 ; placeholder
  %12 = call i32 @puts(i8* %11)
  %13 = add i32 0, 0
  ; Expression result: %13
  %14 = add i32 0, 0 ; placeholder
  %15 = alloca i8*, align 4
  store i8* %14, i8** %15, align 4
  ; Variable info_msg allocated
  %16 = load i8*, i8** %15, align 4
  %17 = call i32 @puts(i8* %16)
  %18 = add i32 0, 0
  ; Expression result: %18
  %19 = add i32 0, 0 ; placeholder
  %20 = call i32 @puts(i8* %19)
  %21 = add i32 0, 0
  ; Expression result: %21
  %22 = add i32 0, 0 ; placeholder
  %23 = alloca i8*, align 4
  store i8* %22, i8** %23, align 4
  ; Variable error_msg allocated
  %24 = load i8*, i8** %23, align 4
  %25 = call i32 @puts(i8* %24)
  %26 = add i32 0, 0
  ; Expression result: %26
  %27 = add i32 0, 0 ; placeholder
  %28 = call i32 @puts(i8* %27)
  %29 = add i32 0, 0
  ; Expression result: %29
  ret i32 0
}

define i32 @test_formatted_output() {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = add i32 0, 0 ; placeholder
  %7 = alloca i8*, align 4
  store i8* %6, i8** %7, align 4
  ; Variable separator allocated
  %8 = load i8*, i8** %7, align 4
  %9 = call i32 @puts(i8* %8)
  %10 = add i32 0, 0
  ; Expression result: %10
  %11 = add i32 0, 0 ; placeholder
  %12 = alloca i8*, align 4
  store i8* %11, i8** %12, align 4
  ; Variable header allocated
  %13 = load i8*, i8** %12, align 4
  %14 = call i32 @puts(i8* %13)
  %15 = add i32 0, 0
  ; Expression result: %15
  %16 = load i8*, i8** %7, align 4
  %17 = call i32 @puts(i8* %16)
  %18 = add i32 0, 0
  ; Expression result: %18
  %19 = add i32 0, 0 ; placeholder
  %20 = alloca i8*, align 4
  store i8* %19, i8** %20, align 4
  ; Variable row allocated
  %21 = load i8*, i8** %20, align 4
  %22 = call i32 @puts(i8* %21)
  %23 = add i32 0, 0
  ; Expression result: %23
  %24 = load i8*, i8** %7, align 4
  %25 = call i32 @puts(i8* %24)
  %26 = add i32 0, 0
  ; Expression result: %26
  ret i32 0
}

define i32 @test_sprintf_simulation() {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = add i32 0, 0 ; placeholder
  %7 = alloca i8*, align 4
  store i8* %6, i8** %7, align 4
  ; Variable result allocated
  %8 = add i32 0, 0 ; placeholder
  %9 = call i32 @puts(i8* %8)
  %10 = add i32 0, 0
  ; Expression result: %10
  %11 = load i8*, i8** %7, align 4
  %12 = call i32 @puts(i8* %11)
  %13 = add i32 0, 0
  ; Expression result: %13
  %14 = add i32 0, 0 ; placeholder
  %15 = call i32 @puts(i8* %14)
  %16 = add i32 0, 0
  ; Expression result: %16
  %17 = add i32 0, 0 ; placeholder
  %18 = alloca i8*, align 4
  store i8* %17, i8** %18, align 4
  ; Variable math_result allocated
  %19 = add i32 0, 0 ; placeholder
  %20 = call i32 @puts(i8* %19)
  %21 = add i32 0, 0
  ; Expression result: %21
  %22 = load i8*, i8** %18, align 4
  %23 = call i32 @puts(i8* %22)
  %24 = add i32 0, 0
  ; Expression result: %24
  %25 = add i32 0, 0 ; placeholder
  %26 = call i32 @puts(i8* %25)
  %27 = add i32 0, 0
  ; Expression result: %27
  ret i32 0
}

define i32 @test_integration() {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = add i32 0, 0 ; placeholder
  %7 = alloca i8*, align 4
  store i8* %6, i8** %7, align 4
  ; Variable name allocated
  %8 = add i32 0, 0 ; placeholder
  %9 = alloca i8*, align 4
  store i8* %8, i8** %9, align 4
  ; Variable version allocated
  %10 = add i32 0, 0 ; placeholder
  %11 = alloca i8*, align 4
  store i8* %10, i8** %11, align 4
  ; Variable status allocated
  %12 = add i32 0, 0 ; placeholder
  %13 = call i32 @puts(i8* %12)
  %14 = add i32 0, 0
  ; Expression result: %14
  %15 = add i32 0, 0 ; placeholder
  %16 = call i32 @puts(i8* %15)
  %17 = add i32 0, 0
  ; Expression result: %17
  %18 = add i32 0, 0 ; placeholder
  %19 = call i32 @puts(i8* %18)
  %20 = add i32 0, 0
  ; Expression result: %20
  %21 = add i32 0, 0 ; placeholder
  %22 = call i32 @puts(i8* %21)
  %23 = add i32 0, 0
  ; Expression result: %23
  %24 = load i8*, i8** %7, align 4
  %25 = call i32 @puts(i8* %24)
  %26 = add i32 0, 0
  ; Expression result: %26
  %27 = add i32 0, 0 ; placeholder
  %28 = call i32 @puts(i8* %27)
  %29 = add i32 0, 0
  ; Expression result: %29
  %30 = add i32 0, 0 ; placeholder
  %31 = call i32 @puts(i8* %30)
  %32 = add i32 0, 0
  ; Expression result: %32
  %33 = load i8*, i8** %9, align 4
  %34 = call i32 @puts(i8* %33)
  %35 = add i32 0, 0
  ; Expression result: %35
  %36 = add i32 0, 0 ; placeholder
  %37 = call i32 @puts(i8* %36)
  %38 = add i32 0, 0
  ; Expression result: %38
  %39 = add i32 0, 0 ; placeholder
  %40 = call i32 @puts(i8* %39)
  %41 = add i32 0, 0
  ; Expression result: %41
  %42 = load i8*, i8** %11, align 4
  %43 = call i32 @puts(i8* %42)
  %44 = add i32 0, 0
  ; Expression result: %44
  %45 = add i32 0, 0 ; placeholder
  %46 = call i32 @puts(i8* %45)
  %47 = add i32 0, 0
  ; Expression result: %47
  %48 = add i32 0, 0 ; placeholder
  %49 = call i32 @puts(i8* %48)
  %50 = add i32 0, 0
  ; Expression result: %50
  ret i32 0
}

define i32 @main() {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = add i32 0, 0 ; placeholder
  %7 = call i32 @puts(i8* %6)
  %8 = add i32 0, 0
  ; Expression result: %8
  %9 = add i32 0, 0 ; placeholder
  %10 = call i32 @puts(i8* %9)
  %11 = add i32 0, 0
  ; Expression result: %11
  %12 = call i32 @test_basic_spill()
  ; Expression result: %12
  %13 = add i32 0, 0 ; placeholder
  %14 = call i32 @puts(i8* %13)
  %15 = add i32 0, 0
  ; Expression result: %15
  %16 = call i32 @test_format_functions()
  ; Expression result: %16
  %17 = add i32 0, 0 ; placeholder
  %18 = call i32 @puts(i8* %17)
  %19 = add i32 0, 0
  ; Expression result: %19
  %20 = call i32 @test_utility_functions()
  ; Expression result: %20
  %21 = add i32 0, 0 ; placeholder
  %22 = call i32 @puts(i8* %21)
  %23 = add i32 0, 0
  ; Expression result: %23
  %24 = call i32 @test_color_functions()
  ; Expression result: %24
  %25 = add i32 0, 0 ; placeholder
  %26 = call i32 @puts(i8* %25)
  %27 = add i32 0, 0
  ; Expression result: %27
  %28 = call i32 @test_debug_functions()
  ; Expression result: %28
  %29 = add i32 0, 0 ; placeholder
  %30 = call i32 @puts(i8* %29)
  %31 = add i32 0, 0
  ; Expression result: %31
  %32 = call i32 @test_formatted_output()
  ; Expression result: %32
  %33 = add i32 0, 0 ; placeholder
  %34 = call i32 @puts(i8* %33)
  %35 = add i32 0, 0
  ; Expression result: %35
  %36 = call i32 @test_sprintf_simulation()
  ; Expression result: %36
  %37 = add i32 0, 0 ; placeholder
  %38 = call i32 @puts(i8* %37)
  %39 = add i32 0, 0
  ; Expression result: %39
  %40 = call i32 @test_integration()
  ; Expression result: %40
  %41 = add i32 0, 0 ; placeholder
  %42 = call i32 @puts(i8* %41)
  %43 = add i32 0, 0
  ; Expression result: %43
  %44 = add i32 0, 0 ; placeholder
  %45 = call i32 @puts(i8* %44)
  %46 = add i32 0, 0
  ; Expression result: %46
  %47 = add i32 0, 0 ; placeholder
  %48 = call i32 @puts(i8* %47)
  %49 = add i32 0, 0
  ; Expression result: %49
  %50 = add i32 0, 0 ; placeholder
  %51 = call i32 @puts(i8* %50)
  %52 = add i32 0, 0
  ; Expression result: %52
  ret i32 0
}

