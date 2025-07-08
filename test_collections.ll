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
define i32 @test_array_operations() {
entry:
  %0 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @array_new()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable arr allocated
  %4 = load i32, i32* %3, align 4
  %5 = call i32 @array_is_empty(i32 %4)
  %6 = call i32 @assert_true(i32 %5)
  ; Expression result: %6
  %7 = load i32, i32* %3, align 4
  %8 = call i32 @array_len(i32 %7)
  %9 = call i32 @assert_eq_int(i32 %8, i32 0)
  ; Expression result: %9
  %10 = load i32, i32* %3, align 4
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = load i32, i32* %3, align 4
  %13 = call i32 @array_push(i32 %12, i32 1)
  ; Expression result: %13
  %14 = load i32, i32* %3, align 4
  ; Expression result: %14
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %15
  %16 = load i32, i32* %3, align 4
  %17 = call i32 @array_push(i32 %16, i32 2)
  ; Expression result: %17
  %18 = load i32, i32* %3, align 4
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %19
  %20 = load i32, i32* %3, align 4
  %21 = call i32 @array_push(i32 %20, i32 3)
  ; Expression result: %21
  %22 = load i32, i32* %3, align 4
  %23 = call i32 @array_len(i32 %22)
  %24 = call i32 @assert_eq_int(i32 %23, i32 3)
  ; Expression result: %24
  %25 = load i32, i32* %3, align 4
  %26 = call i32 @array_is_empty(i32 %25)
  %27 = call i32 @assert_false(i32 %26)
  ; Expression result: %27
  %28 = load i32, i32* %3, align 4
  %29 = call i32 @array_get(i32 %28, i32 0)
  %30 = call i32 @assert_eq_int(i32 %29, i32 1)
  ; Expression result: %30
  %31 = load i32, i32* %3, align 4
  %32 = call i32 @array_get(i32 %31, i32 1)
  %33 = call i32 @assert_eq_int(i32 %32, i32 2)
  ; Expression result: %33
  %34 = load i32, i32* %3, align 4
  %35 = call i32 @array_get(i32 %34, i32 2)
  %36 = call i32 @assert_eq_int(i32 %35, i32 3)
  ; Expression result: %36
  %37 = load i32, i32* %3, align 4
  ; Expression result: %37
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %38
  %39 = load i32, i32* %3, align 4
  %40 = call i32 @array_set(i32 %39, i32 1, i32 5)
  ; Expression result: %40
  %41 = load i32, i32* %3, align 4
  %42 = call i32 @array_get(i32 %41, i32 1)
  %43 = call i32 @assert_eq_int(i32 %42, i32 5)
  ; Expression result: %43
  %44 = load i32, i32* %3, align 4
  %45 = call i32 @array_pop(i32 %44)
  %46 = alloca i32, align 4
  store i32 %45, i32* %46, align 4
  ; Variable popped allocated
  %47 = load i32, i32* %46, align 4
  %48 = call i32 @assert_eq_int(i32 %47, i32 3)
  ; Expression result: %48
  %49 = load i32, i32* %3, align 4
  %50 = call i32 @array_len(i32 %49)
  %51 = call i32 @assert_eq_int(i32 %50, i32 2)
  ; Expression result: %51
  %52 = load i32, i32* %3, align 4
  ; Expression result: %52
  %53 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %53
  %54 = load i32, i32* %3, align 4
  %55 = call i32 @array_insert(i32 %54, i32 1, i32 4)
  ; Expression result: %55
  %56 = load i32, i32* %3, align 4
  %57 = call i32 @array_len(i32 %56)
  %58 = call i32 @assert_eq_int(i32 %57, i32 3)
  ; Expression result: %58
  %59 = load i32, i32* %3, align 4
  %60 = call i32 @array_get(i32 %59, i32 1)
  %61 = call i32 @assert_eq_int(i32 %60, i32 4)
  ; Expression result: %61
  %62 = load i32, i32* %3, align 4
  %63 = call i32 @array_remove(i32 %62, i32 1)
  %64 = alloca i32, align 4
  store i32 %63, i32* %64, align 4
  ; Variable removed allocated
  %65 = load i32, i32* %64, align 4
  %66 = call i32 @assert_eq_int(i32 %65, i32 4)
  ; Expression result: %66
  %67 = load i32, i32* %3, align 4
  %68 = call i32 @array_len(i32 %67)
  %69 = call i32 @assert_eq_int(i32 %68, i32 2)
  ; Expression result: %69
  ret i32 0
}

define i8* @test_array_search() {
entry:
  %0 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.2, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @array_new()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable arr allocated
  %4 = load i32, i32* %3, align 4
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = load i32, i32* %3, align 4
  %7 = call i32 @array_push(i32 %6, i32 10)
  ; Expression result: %7
  %8 = load i32, i32* %3, align 4
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %9
  %10 = load i32, i32* %3, align 4
  %11 = call i32 @array_push(i32 %10, i32 20)
  ; Expression result: %11
  %12 = load i32, i32* %3, align 4
  ; Expression result: %12
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %13
  %14 = load i32, i32* %3, align 4
  %15 = call i32 @array_push(i32 %14, i32 30)
  ; Expression result: %15
  %16 = load i32, i32* %3, align 4
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = load i32, i32* %3, align 4
  %19 = call i32 @array_push(i32 %18, i32 20)
  ; Expression result: %19
  %20 = load i32, i32* %3, align 4
  %21 = call i32 @array_contains(i32 %20, i32 20)
  %22 = call i32 @assert_true(i32 %21)
  ; Expression result: %22
  %23 = load i32, i32* %3, align 4
  %24 = call i32 @array_contains(i32 %23, i32 10)
  %25 = call i32 @assert_true(i32 %24)
  ; Expression result: %25
  %26 = load i32, i32* %3, align 4
  %27 = call i32 @array_contains(i32 %26, i32 40)
  %28 = call i32 @assert_false(i32 %27)
  ; Expression result: %28
  %29 = load i32, i32* %3, align 4
  %30 = call i32 @array_index_of(i32 %29, i32 20)
  %31 = call i32 @assert_eq_int(i32 %30, i32 1)
  ; Expression result: %31
  %32 = load i32, i32* %3, align 4
  %33 = call i32 @array_index_of(i32 %32, i32 10)
  %34 = call i32 @assert_eq_int(i32 %33, i32 0)
  ; Expression result: %34
  ; Expression result: 1
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %35
  ret i32 0
}

define i32 @test_array_manipulation() {
entry:
  %0 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.3, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @array_new()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable arr allocated
  %4 = load i32, i32* %3, align 4
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = load i32, i32* %3, align 4
  %7 = call i32 @array_push(i32 %6, i32 1)
  ; Expression result: %7
  %8 = load i32, i32* %3, align 4
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %9
  %10 = load i32, i32* %3, align 4
  %11 = call i32 @array_push(i32 %10, i32 2)
  ; Expression result: %11
  %12 = load i32, i32* %3, align 4
  ; Expression result: %12
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %13
  %14 = load i32, i32* %3, align 4
  %15 = call i32 @array_push(i32 %14, i32 3)
  ; Expression result: %15
  %16 = load i32, i32* %3, align 4
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = load i32, i32* %3, align 4
  %19 = call i32 @array_reverse(i32 %18)
  ; Expression result: %19
  %20 = load i32, i32* %3, align 4
  %21 = call i32 @array_get(i32 %20, i32 0)
  %22 = call i32 @assert_eq_int(i32 %21, i32 3)
  ; Expression result: %22
  %23 = load i32, i32* %3, align 4
  %24 = call i32 @array_get(i32 %23, i32 1)
  %25 = call i32 @assert_eq_int(i32 %24, i32 2)
  ; Expression result: %25
  %26 = load i32, i32* %3, align 4
  %27 = call i32 @array_get(i32 %26, i32 2)
  %28 = call i32 @assert_eq_int(i32 %27, i32 1)
  ; Expression result: %28
  %29 = load i32, i32* %3, align 4
  %30 = call i32 @array_slice(i32 %29, i32 1, i32 3)
  %31 = alloca i32, align 4
  store i32 %30, i32* %31, align 4
  ; Variable sliced allocated
  %32 = load i32, i32* %31, align 4
  %33 = call i32 @array_len(i32 %32)
  %34 = call i32 @assert_eq_int(i32 %33, i32 2)
  ; Expression result: %34
  %35 = load i32, i32* %31, align 4
  %36 = call i32 @array_get(i32 %35, i32 0)
  %37 = call i32 @assert_eq_int(i32 %36, i32 2)
  ; Expression result: %37
  %38 = load i32, i32* %31, align 4
  %39 = call i32 @array_get(i32 %38, i32 1)
  %40 = call i32 @assert_eq_int(i32 %39, i32 1)
  ; Expression result: %40
  %41 = call i32 @array_new()
  %42 = alloca i32, align 4
  store i32 %41, i32* %42, align 4
  ; Variable arr2 allocated
  %43 = load i32, i32* %42, align 4
  ; Expression result: %43
  %44 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %44
  %45 = load i32, i32* %42, align 4
  %46 = call i32 @array_push(i32 %45, i32 4)
  ; Expression result: %46
  %47 = load i32, i32* %42, align 4
  ; Expression result: %47
  %48 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %48
  %49 = load i32, i32* %42, align 4
  %50 = call i32 @array_push(i32 %49, i32 5)
  ; Expression result: %50
  %51 = load i32, i32* %3, align 4
  %52 = load i32, i32* %42, align 4
  %53 = call i32 @array_concat(i32 %51, i32 %52)
  %54 = alloca i32, align 4
  store i32 %53, i32* %54, align 4
  ; Variable concatenated allocated
  %55 = load i32, i32* %54, align 4
  %56 = call i32 @array_len(i32 %55)
  %57 = call i32 @assert_eq_int(i32 %56, i32 5)
  ; Expression result: %57
  %58 = load i32, i32* %54, align 4
  %59 = call i32 @array_get(i32 %58, i32 3)
  %60 = call i32 @assert_eq_int(i32 %59, i32 4)
  ; Expression result: %60
  %61 = load i32, i32* %54, align 4
  %62 = call i32 @array_get(i32 %61, i32 4)
  %63 = call i32 @assert_eq_int(i32 %62, i32 5)
  ; Expression result: %63
  %64 = load i32, i32* %3, align 4
  ; Expression result: %64
  %65 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %65
  %66 = load i32, i32* %3, align 4
  %67 = call i32 @array_clear(i32 %66)
  ; Expression result: %67
  %68 = load i32, i32* %3, align 4
  %69 = call i32 @array_is_empty(i32 %68)
  %70 = call i32 @assert_true(i32 %69)
  ; Expression result: %70
  %71 = load i32, i32* %3, align 4
  %72 = call i32 @array_len(i32 %71)
  %73 = call i32 @assert_eq_int(i32 %72, i32 0)
  ; Expression result: %73
  ret i32 0
}

define i32 @test_map_operations() {
entry:
  %0 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.4, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @map_new()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable m allocated
  %4 = load i32, i32* %3, align 4
  %5 = call i32 @map_is_empty(i32 %4)
  %6 = call i32 @assert_true(i32 %5)
  ; Expression result: %6
  %7 = load i32, i32* %3, align 4
  %8 = call i32 @map_len(i32 %7)
  %9 = call i32 @assert_eq_int(i32 %8, i32 0)
  ; Expression result: %9
  %10 = load i32, i32* %3, align 4
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = load i32, i32* %3, align 4
  %13 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.5, i64 0, i64 0
  %14 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.6, i64 0, i64 0
  %15 = call i32 @map_set(i32 %12, i32 %13, i32 %14)
  ; Expression result: %15
  %16 = load i32, i32* %3, align 4
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = load i32, i32* %3, align 4
  %19 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.7, i64 0, i64 0
  %20 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.8, i64 0, i64 0
  %21 = call i32 @map_set(i32 %18, i32 %19, i32 %20)
  ; Expression result: %21
  %22 = load i32, i32* %3, align 4
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  %24 = load i32, i32* %3, align 4
  %25 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.9, i64 0, i64 0
  %26 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.10, i64 0, i64 0
  %27 = call i32 @map_set(i32 %24, i32 %25, i32 %26)
  ; Expression result: %27
  %28 = load i32, i32* %3, align 4
  %29 = call i32 @map_len(i32 %28)
  %30 = call i32 @assert_eq_int(i32 %29, i32 3)
  ; Expression result: %30
  %31 = load i32, i32* %3, align 4
  %32 = call i32 @map_is_empty(i32 %31)
  %33 = call i32 @assert_false(i32 %32)
  ; Expression result: %33
  %34 = load i32, i32* %3, align 4
  %35 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.5, i64 0, i64 0
  %36 = call i32 @map_get(i32 %34, i32 %35)
  %37 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.6, i64 0, i64 0
  %38 = call i32 @assert_eq_string(i32 %36, i32 %37)
  ; Expression result: %38
  %39 = load i32, i32* %3, align 4
  %40 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.7, i64 0, i64 0
  %41 = call i32 @map_get(i32 %39, i32 %40)
  %42 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.8, i64 0, i64 0
  %43 = call i32 @assert_eq_string(i32 %41, i32 %42)
  ; Expression result: %43
  %44 = load i32, i32* %3, align 4
  %45 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.9, i64 0, i64 0
  %46 = call i32 @map_get(i32 %44, i32 %45)
  %47 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.10, i64 0, i64 0
  %48 = call i32 @assert_eq_string(i32 %46, i32 %47)
  ; Expression result: %48
  %49 = load i32, i32* %3, align 4
  %50 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.5, i64 0, i64 0
  %51 = call i32 @map_contains_key(i32 %49, i32 %50)
  %52 = call i32 @assert_true(i32 %51)
  ; Expression result: %52
  %53 = load i32, i32* %3, align 4
  %54 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.7, i64 0, i64 0
  %55 = call i32 @map_contains_key(i32 %53, i32 %54)
  %56 = call i32 @assert_true(i32 %55)
  ; Expression result: %56
  %57 = load i32, i32* %3, align 4
  %58 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.11, i64 0, i64 0
  %59 = call i32 @map_contains_key(i32 %57, i32 %58)
  %60 = call i32 @assert_false(i32 %59)
  ; Expression result: %60
  %61 = load i32, i32* %3, align 4
  %62 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.5, i64 0, i64 0
  %63 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.12, i64 0, i64 0
  %64 = call i32 @map_get_or_default(i32 %61, i32 %62, i32 %63)
  %65 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.6, i64 0, i64 0
  %66 = call i32 @assert_eq_string(i32 %64, i32 %65)
  ; Expression result: %66
  %67 = load i32, i32* %3, align 4
  %68 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.11, i64 0, i64 0
  %69 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.12, i64 0, i64 0
  %70 = call i32 @map_get_or_default(i32 %67, i32 %68, i32 %69)
  %71 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.12, i64 0, i64 0
  %72 = call i32 @assert_eq_string(i32 %70, i32 %71)
  ; Expression result: %72
  %73 = load i32, i32* %3, align 4
  %74 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.7, i64 0, i64 0
  %75 = call i32 @map_remove(i32 %73, i32 %74)
  %76 = alloca i32, align 4
  store i32 %75, i32* %76, align 4
  ; Variable removed allocated
  %77 = load i32, i32* %76, align 4
  %78 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.8, i64 0, i64 0
  %79 = call i32 @assert_eq_string(i32 %77, i32 %78)
  ; Expression result: %79
  %80 = load i32, i32* %3, align 4
  %81 = call i32 @map_len(i32 %80)
  %82 = call i32 @assert_eq_int(i32 %81, i32 2)
  ; Expression result: %82
  %83 = load i32, i32* %3, align 4
  %84 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.7, i64 0, i64 0
  %85 = call i32 @map_contains_key(i32 %83, i32 %84)
  %86 = call i32 @assert_false(i32 %85)
  ; Expression result: %86
  ret i32 0
}

define i32 @test_map_collections() {
entry:
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.13, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @map_new()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable m allocated
  %4 = load i32, i32* %3, align 4
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = load i32, i32* %3, align 4
  %7 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.14, i64 0, i64 0
  %8 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.15, i64 0, i64 0
  %9 = call i32 @map_set(i32 %6, i32 %7, i32 %8)
  ; Expression result: %9
  %10 = load i32, i32* %3, align 4
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = load i32, i32* %3, align 4
  %13 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.16, i64 0, i64 0
  %14 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.17, i64 0, i64 0
  %15 = call i32 @map_set(i32 %12, i32 %13, i32 %14)
  ; Expression result: %15
  %16 = load i32, i32* %3, align 4
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = load i32, i32* %3, align 4
  %19 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.18, i64 0, i64 0
  %20 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.19, i64 0, i64 0
  %21 = call i32 @map_set(i32 %18, i32 %19, i32 %20)
  ; Expression result: %21
  %22 = load i32, i32* %3, align 4
  %23 = call i32 @map_keys(i32 %22)
  %24 = alloca i32, align 4
  store i32 %23, i32* %24, align 4
  ; Variable keys allocated
  %25 = load i32, i32* %24, align 4
  %26 = call i32 @len(i32 %25)
  %27 = call i32 @assert_eq_int(i32 %26, i32 3)
  ; Expression result: %27
  %28 = load i32, i32* %24, align 4
  %29 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.14, i64 0, i64 0
  %30 = call i32 @array_contains(i32 %28, i32 %29)
  %31 = call i32 @assert_true(i32 %30)
  ; Expression result: %31
  %32 = load i32, i32* %24, align 4
  %33 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.16, i64 0, i64 0
  %34 = call i32 @array_contains(i32 %32, i32 %33)
  %35 = call i32 @assert_true(i32 %34)
  ; Expression result: %35
  %36 = load i32, i32* %24, align 4
  %37 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.18, i64 0, i64 0
  %38 = call i32 @array_contains(i32 %36, i32 %37)
  %39 = call i32 @assert_true(i32 %38)
  ; Expression result: %39
  %40 = load i32, i32* %3, align 4
  %41 = call i32 @map_values(i32 %40)
  %42 = alloca i32, align 4
  store i32 %41, i32* %42, align 4
  ; Variable values allocated
  %43 = load i32, i32* %42, align 4
  %44 = call i32 @len(i32 %43)
  %45 = call i32 @assert_eq_int(i32 %44, i32 3)
  ; Expression result: %45
  %46 = load i32, i32* %42, align 4
  %47 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.15, i64 0, i64 0
  %48 = call i32 @array_contains(i32 %46, i32 %47)
  %49 = call i32 @assert_true(i32 %48)
  ; Expression result: %49
  %50 = load i32, i32* %42, align 4
  %51 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.17, i64 0, i64 0
  %52 = call i32 @array_contains(i32 %50, i32 %51)
  %53 = call i32 @assert_true(i32 %52)
  ; Expression result: %53
  %54 = load i32, i32* %42, align 4
  %55 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.19, i64 0, i64 0
  %56 = call i32 @array_contains(i32 %54, i32 %55)
  %57 = call i32 @assert_true(i32 %56)
  ; Expression result: %57
  %58 = call i32 @map_new()
  %59 = alloca i32, align 4
  store i32 %58, i32* %59, align 4
  ; Variable m2 allocated
  %60 = load i32, i32* %59, align 4
  ; Expression result: %60
  %61 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %61
  %62 = load i32, i32* %59, align 4
  %63 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.20, i64 0, i64 0
  %64 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.21, i64 0, i64 0
  %65 = call i32 @map_set(i32 %62, i32 %63, i32 %64)
  ; Expression result: %65
  %66 = load i32, i32* %59, align 4
  ; Expression result: %66
  %67 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %67
  %68 = load i32, i32* %59, align 4
  %69 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.16, i64 0, i64 0
  %70 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.22, i64 0, i64 0
  %71 = call i32 @map_set(i32 %68, i32 %69, i32 %70)
  ; Expression result: %71
  %72 = load i32, i32* %3, align 4
  %73 = load i32, i32* %59, align 4
  %74 = call i32 @map_merge(i32 %72, i32 %73)
  %75 = alloca i32, align 4
  store i32 %74, i32* %75, align 4
  ; Variable merged allocated
  %76 = load i32, i32* %75, align 4
  %77 = call i32 @map_len(i32 %76)
  %78 = call i32 @assert_eq_int(i32 %77, i32 4)
  ; Expression result: %78
  %79 = load i32, i32* %75, align 4
  %80 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.20, i64 0, i64 0
  %81 = call i32 @map_get(i32 %79, i32 %80)
  %82 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.21, i64 0, i64 0
  %83 = call i32 @assert_eq_string(i32 %81, i32 %82)
  ; Expression result: %83
  %84 = load i32, i32* %75, align 4
  %85 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.16, i64 0, i64 0
  %86 = call i32 @map_get(i32 %84, i32 %85)
  %87 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.22, i64 0, i64 0
  %88 = call i32 @assert_eq_string(i32 %86, i32 %87)
  ; Expression result: %88
  ret i32 0
}

define i32 @test_set_operations() {
entry:
  %0 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.23, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @set_new()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable s allocated
  %4 = load i32, i32* %3, align 4
  %5 = call i32 @set_is_empty(i32 %4)
  %6 = call i32 @assert_true(i32 %5)
  ; Expression result: %6
  %7 = load i32, i32* %3, align 4
  %8 = call i32 @set_len(i32 %7)
  %9 = call i32 @assert_eq_int(i32 %8, i32 0)
  ; Expression result: %9
  %10 = load i32, i32* %3, align 4
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = load i32, i32* %3, align 4
  %13 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.24, i64 0, i64 0
  %14 = call i32 @set_add(i32 %12, i32 %13)
  ; Expression result: %14
  %15 = load i32, i32* %3, align 4
  ; Expression result: %15
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %16
  %17 = load i32, i32* %3, align 4
  %18 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.25, i64 0, i64 0
  %19 = call i32 @set_add(i32 %17, i32 %18)
  ; Expression result: %19
  %20 = load i32, i32* %3, align 4
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %21
  %22 = load i32, i32* %3, align 4
  %23 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.26, i64 0, i64 0
  %24 = call i32 @set_add(i32 %22, i32 %23)
  ; Expression result: %24
  %25 = load i32, i32* %3, align 4
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %26
  %27 = load i32, i32* %3, align 4
  %28 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.24, i64 0, i64 0
  %29 = call i32 @set_add(i32 %27, i32 %28)
  ; Expression result: %29
  %30 = load i32, i32* %3, align 4
  %31 = call i32 @set_len(i32 %30)
  %32 = call i32 @assert_eq_int(i32 %31, i32 3)
  ; Expression result: %32
  %33 = load i32, i32* %3, align 4
  %34 = call i32 @set_is_empty(i32 %33)
  %35 = call i32 @assert_false(i32 %34)
  ; Expression result: %35
  %36 = load i32, i32* %3, align 4
  %37 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.24, i64 0, i64 0
  %38 = call i32 @set_contains(i32 %36, i32 %37)
  %39 = call i32 @assert_true(i32 %38)
  ; Expression result: %39
  %40 = load i32, i32* %3, align 4
  %41 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.25, i64 0, i64 0
  %42 = call i32 @set_contains(i32 %40, i32 %41)
  %43 = call i32 @assert_true(i32 %42)
  ; Expression result: %43
  %44 = load i32, i32* %3, align 4
  %45 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.26, i64 0, i64 0
  %46 = call i32 @set_contains(i32 %44, i32 %45)
  %47 = call i32 @assert_true(i32 %46)
  ; Expression result: %47
  %48 = load i32, i32* %3, align 4
  %49 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.27, i64 0, i64 0
  %50 = call i32 @set_contains(i32 %48, i32 %49)
  %51 = call i32 @assert_false(i32 %50)
  ; Expression result: %51
  %52 = load i32, i32* %3, align 4
  %53 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.25, i64 0, i64 0
  %54 = call i32 @set_remove(i32 %52, i32 %53)
  %55 = call i32 @assert_true(i32 %54)
  ; Expression result: %55
  %56 = load i32, i32* %3, align 4
  %57 = call i32 @set_len(i32 %56)
  %58 = call i32 @assert_eq_int(i32 %57, i32 2)
  ; Expression result: %58
  %59 = load i32, i32* %3, align 4
  %60 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.25, i64 0, i64 0
  %61 = call i32 @set_contains(i32 %59, i32 %60)
  %62 = call i32 @assert_false(i32 %61)
  ; Expression result: %62
  %63 = load i32, i32* %3, align 4
  %64 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.11, i64 0, i64 0
  %65 = call i32 @set_remove(i32 %63, i32 %64)
  %66 = call i32 @assert_false(i32 %65)
  ; Expression result: %66
  ret i32 0
}

define i32 @test_set_operations_advanced() {
entry:
  %0 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.28, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @set_new()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable s1 allocated
  %4 = load i32, i32* %3, align 4
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = load i32, i32* %3, align 4
  %7 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.29, i64 0, i64 0
  %8 = call i32 @set_add(i32 %6, i32 %7)
  ; Expression result: %8
  %9 = load i32, i32* %3, align 4
  ; Expression result: %9
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %10
  %11 = load i32, i32* %3, align 4
  %12 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.30, i64 0, i64 0
  %13 = call i32 @set_add(i32 %11, i32 %12)
  ; Expression result: %13
  %14 = load i32, i32* %3, align 4
  ; Expression result: %14
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %15
  %16 = load i32, i32* %3, align 4
  %17 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.31, i64 0, i64 0
  %18 = call i32 @set_add(i32 %16, i32 %17)
  ; Expression result: %18
  %19 = call i32 @set_new()
  %20 = alloca i32, align 4
  store i32 %19, i32* %20, align 4
  ; Variable s2 allocated
  %21 = load i32, i32* %20, align 4
  ; Expression result: %21
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %22
  %23 = load i32, i32* %20, align 4
  %24 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.30, i64 0, i64 0
  %25 = call i32 @set_add(i32 %23, i32 %24)
  ; Expression result: %25
  %26 = load i32, i32* %20, align 4
  ; Expression result: %26
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %27
  %28 = load i32, i32* %20, align 4
  %29 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.31, i64 0, i64 0
  %30 = call i32 @set_add(i32 %28, i32 %29)
  ; Expression result: %30
  %31 = load i32, i32* %20, align 4
  ; Expression result: %31
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %32
  %33 = load i32, i32* %20, align 4
  %34 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.32, i64 0, i64 0
  %35 = call i32 @set_add(i32 %33, i32 %34)
  ; Expression result: %35
  %36 = load i32, i32* %3, align 4
  %37 = load i32, i32* %20, align 4
  %38 = call i32 @set_union(i32 %36, i32 %37)
  %39 = alloca i32, align 4
  store i32 %38, i32* %39, align 4
  ; Variable union_set allocated
  %40 = load i32, i32* %39, align 4
  %41 = call i32 @set_len(i32 %40)
  %42 = call i32 @assert_eq_int(i32 %41, i32 4)
  ; Expression result: %42
  %43 = load i32, i32* %39, align 4
  %44 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.29, i64 0, i64 0
  %45 = call i32 @set_contains(i32 %43, i32 %44)
  %46 = call i32 @assert_true(i32 %45)
  ; Expression result: %46
  %47 = load i32, i32* %39, align 4
  %48 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.30, i64 0, i64 0
  %49 = call i32 @set_contains(i32 %47, i32 %48)
  %50 = call i32 @assert_true(i32 %49)
  ; Expression result: %50
  %51 = load i32, i32* %39, align 4
  %52 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.31, i64 0, i64 0
  %53 = call i32 @set_contains(i32 %51, i32 %52)
  %54 = call i32 @assert_true(i32 %53)
  ; Expression result: %54
  %55 = load i32, i32* %39, align 4
  %56 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.32, i64 0, i64 0
  %57 = call i32 @set_contains(i32 %55, i32 %56)
  %58 = call i32 @assert_true(i32 %57)
  ; Expression result: %58
  %59 = load i32, i32* %3, align 4
  %60 = load i32, i32* %20, align 4
  %61 = call i32 @set_intersection(i32 %59, i32 %60)
  %62 = alloca i32, align 4
  store i32 %61, i32* %62, align 4
  ; Variable intersection_set allocated
  %63 = load i32, i32* %62, align 4
  %64 = call i32 @set_len(i32 %63)
  %65 = call i32 @assert_eq_int(i32 %64, i32 2)
  ; Expression result: %65
  %66 = load i32, i32* %62, align 4
  %67 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.30, i64 0, i64 0
  %68 = call i32 @set_contains(i32 %66, i32 %67)
  %69 = call i32 @assert_true(i32 %68)
  ; Expression result: %69
  %70 = load i32, i32* %62, align 4
  %71 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.31, i64 0, i64 0
  %72 = call i32 @set_contains(i32 %70, i32 %71)
  %73 = call i32 @assert_true(i32 %72)
  ; Expression result: %73
  %74 = load i32, i32* %62, align 4
  %75 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.29, i64 0, i64 0
  %76 = call i32 @set_contains(i32 %74, i32 %75)
  %77 = call i32 @assert_false(i32 %76)
  ; Expression result: %77
  %78 = load i32, i32* %62, align 4
  %79 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.32, i64 0, i64 0
  %80 = call i32 @set_contains(i32 %78, i32 %79)
  %81 = call i32 @assert_false(i32 %80)
  ; Expression result: %81
  %82 = load i32, i32* %3, align 4
  %83 = load i32, i32* %20, align 4
  %84 = call i32 @set_difference(i32 %82, i32 %83)
  %85 = alloca i32, align 4
  store i32 %84, i32* %85, align 4
  ; Variable difference_set allocated
  %86 = load i32, i32* %85, align 4
  %87 = call i32 @set_len(i32 %86)
  %88 = call i32 @assert_eq_int(i32 %87, i32 1)
  ; Expression result: %88
  %89 = load i32, i32* %85, align 4
  %90 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.29, i64 0, i64 0
  %91 = call i32 @set_contains(i32 %89, i32 %90)
  %92 = call i32 @assert_true(i32 %91)
  ; Expression result: %92
  %93 = load i32, i32* %85, align 4
  %94 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.30, i64 0, i64 0
  %95 = call i32 @set_contains(i32 %93, i32 %94)
  %96 = call i32 @assert_false(i32 %95)
  ; Expression result: %96
  %97 = load i32, i32* %85, align 4
  %98 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.31, i64 0, i64 0
  %99 = call i32 @set_contains(i32 %97, i32 %98)
  %100 = call i32 @assert_false(i32 %99)
  ; Expression result: %100
  %101 = call i32 @set_new()
  %102 = alloca i32, align 4
  store i32 %101, i32* %102, align 4
  ; Variable subset allocated
  %103 = load i32, i32* %102, align 4
  ; Expression result: %103
  %104 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %104
  %105 = load i32, i32* %102, align 4
  %106 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.29, i64 0, i64 0
  %107 = call i32 @set_add(i32 %105, i32 %106)
  ; Expression result: %107
  %108 = load i32, i32* %102, align 4
  ; Expression result: %108
  %109 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %109
  %110 = load i32, i32* %102, align 4
  %111 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.30, i64 0, i64 0
  %112 = call i32 @set_add(i32 %110, i32 %111)
  ; Expression result: %112
  %113 = load i32, i32* %102, align 4
  %114 = load i32, i32* %3, align 4
  %115 = call i32 @set_is_subset(i32 %113, i32 %114)
  %116 = call i32 @assert_true(i32 %115)
  ; Expression result: %116
  %117 = load i32, i32* %3, align 4
  %118 = load i32, i32* %102, align 4
  %119 = call i32 @set_is_superset(i32 %117, i32 %118)
  %120 = call i32 @assert_true(i32 %119)
  ; Expression result: %120
  %121 = load i32, i32* %3, align 4
  %122 = load i32, i32* %102, align 4
  %123 = call i32 @set_is_subset(i32 %121, i32 %122)
  %124 = call i32 @assert_false(i32 %123)
  ; Expression result: %124
  %125 = load i32, i32* %102, align 4
  %126 = load i32, i32* %3, align 4
  %127 = call i32 @set_is_superset(i32 %125, i32 %126)
  %128 = call i32 @assert_false(i32 %127)
  ; Expression result: %128
  ret i32 0
}

define i32 @test_queue_operations() {
entry:
  %0 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.33, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @queue_new()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable q allocated
  %4 = load i32, i32* %3, align 4
  %5 = call i32 @queue_is_empty(i32 %4)
  %6 = call i32 @assert_true(i32 %5)
  ; Expression result: %6
  %7 = load i32, i32* %3, align 4
  %8 = call i32 @queue_len(i32 %7)
  %9 = call i32 @assert_eq_int(i32 %8, i32 0)
  ; Expression result: %9
  %10 = load i32, i32* %3, align 4
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = load i32, i32* %3, align 4
  %13 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.34, i64 0, i64 0
  %14 = call i32 @queue_enqueue(i32 %12, i32 %13)
  ; Expression result: %14
  %15 = load i32, i32* %3, align 4
  ; Expression result: %15
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %16
  %17 = load i32, i32* %3, align 4
  %18 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.35, i64 0, i64 0
  %19 = call i32 @queue_enqueue(i32 %17, i32 %18)
  ; Expression result: %19
  %20 = load i32, i32* %3, align 4
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %21
  %22 = load i32, i32* %3, align 4
  %23 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.36, i64 0, i64 0
  %24 = call i32 @queue_enqueue(i32 %22, i32 %23)
  ; Expression result: %24
  %25 = load i32, i32* %3, align 4
  %26 = call i32 @queue_len(i32 %25)
  %27 = call i32 @assert_eq_int(i32 %26, i32 3)
  ; Expression result: %27
  %28 = load i32, i32* %3, align 4
  %29 = call i32 @queue_is_empty(i32 %28)
  %30 = call i32 @assert_false(i32 %29)
  ; Expression result: %30
  %31 = load i32, i32* %3, align 4
  %32 = call i32 @queue_front(i32 %31)
  %33 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.34, i64 0, i64 0
  %34 = call i32 @assert_eq_string(i32 %32, i32 %33)
  ; Expression result: %34
  %35 = load i32, i32* %3, align 4
  %36 = call i32 @queue_back(i32 %35)
  %37 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.36, i64 0, i64 0
  %38 = call i32 @assert_eq_string(i32 %36, i32 %37)
  ; Expression result: %38
  %39 = load i32, i32* %3, align 4
  %40 = call i32 @queue_dequeue(i32 %39)
  %41 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.34, i64 0, i64 0
  %42 = call i32 @assert_eq_string(i32 %40, i32 %41)
  ; Expression result: %42
  %43 = load i32, i32* %3, align 4
  %44 = call i32 @queue_len(i32 %43)
  %45 = call i32 @assert_eq_int(i32 %44, i32 2)
  ; Expression result: %45
  %46 = load i32, i32* %3, align 4
  %47 = call i32 @queue_front(i32 %46)
  %48 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.35, i64 0, i64 0
  %49 = call i32 @assert_eq_string(i32 %47, i32 %48)
  ; Expression result: %49
  %50 = load i32, i32* %3, align 4
  %51 = call i32 @queue_dequeue(i32 %50)
  %52 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.35, i64 0, i64 0
  %53 = call i32 @assert_eq_string(i32 %51, i32 %52)
  ; Expression result: %53
  %54 = load i32, i32* %3, align 4
  %55 = call i32 @queue_dequeue(i32 %54)
  %56 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.36, i64 0, i64 0
  %57 = call i32 @assert_eq_string(i32 %55, i32 %56)
  ; Expression result: %57
  %58 = load i32, i32* %3, align 4
  %59 = call i32 @queue_is_empty(i32 %58)
  %60 = call i32 @assert_true(i32 %59)
  ; Expression result: %60
  ret i32 0
}

define i32 @test_stack_operations() {
entry:
  %0 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.37, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @stack_new()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable s allocated
  %4 = load i32, i32* %3, align 4
  %5 = call i32 @stack_is_empty(i32 %4)
  %6 = call i32 @assert_true(i32 %5)
  ; Expression result: %6
  %7 = load i32, i32* %3, align 4
  %8 = call i32 @stack_len(i32 %7)
  %9 = call i32 @assert_eq_int(i32 %8, i32 0)
  ; Expression result: %9
  %10 = load i32, i32* %3, align 4
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = load i32, i32* %3, align 4
  %13 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.38, i64 0, i64 0
  %14 = call i32 @stack_push(i32 %12, i32 %13)
  ; Expression result: %14
  %15 = load i32, i32* %3, align 4
  ; Expression result: %15
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %16
  %17 = load i32, i32* %3, align 4
  %18 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.39, i64 0, i64 0
  %19 = call i32 @stack_push(i32 %17, i32 %18)
  ; Expression result: %19
  %20 = load i32, i32* %3, align 4
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %21
  %22 = load i32, i32* %3, align 4
  %23 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.40, i64 0, i64 0
  %24 = call i32 @stack_push(i32 %22, i32 %23)
  ; Expression result: %24
  %25 = load i32, i32* %3, align 4
  %26 = call i32 @stack_len(i32 %25)
  %27 = call i32 @assert_eq_int(i32 %26, i32 3)
  ; Expression result: %27
  %28 = load i32, i32* %3, align 4
  %29 = call i32 @stack_is_empty(i32 %28)
  %30 = call i32 @assert_false(i32 %29)
  ; Expression result: %30
  %31 = load i32, i32* %3, align 4
  %32 = call i32 @stack_peek(i32 %31)
  %33 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.40, i64 0, i64 0
  %34 = call i32 @assert_eq_string(i32 %32, i32 %33)
  ; Expression result: %34
  %35 = load i32, i32* %3, align 4
  %36 = call i32 @stack_len(i32 %35)
  %37 = call i32 @assert_eq_int(i32 %36, i32 3)
  ; Expression result: %37
  %38 = load i32, i32* %3, align 4
  %39 = call i32 @stack_pop(i32 %38)
  %40 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.40, i64 0, i64 0
  %41 = call i32 @assert_eq_string(i32 %39, i32 %40)
  ; Expression result: %41
  %42 = load i32, i32* %3, align 4
  %43 = call i32 @stack_len(i32 %42)
  %44 = call i32 @assert_eq_int(i32 %43, i32 2)
  ; Expression result: %44
  %45 = load i32, i32* %3, align 4
  %46 = call i32 @stack_peek(i32 %45)
  %47 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.39, i64 0, i64 0
  %48 = call i32 @assert_eq_string(i32 %46, i32 %47)
  ; Expression result: %48
  %49 = load i32, i32* %3, align 4
  %50 = call i32 @stack_pop(i32 %49)
  %51 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.39, i64 0, i64 0
  %52 = call i32 @assert_eq_string(i32 %50, i32 %51)
  ; Expression result: %52
  %53 = load i32, i32* %3, align 4
  %54 = call i32 @stack_pop(i32 %53)
  %55 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.38, i64 0, i64 0
  %56 = call i32 @assert_eq_string(i32 %54, i32 %55)
  ; Expression result: %56
  %57 = load i32, i32* %3, align 4
  %58 = call i32 @stack_is_empty(i32 %57)
  %59 = call i32 @assert_true(i32 %58)
  ; Expression result: %59
  ret i32 0
}

define i32 @test_utility_functions() {
entry:
  %0 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.41, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @range(i32 1, i32 5)
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable range_arr allocated
  %4 = load i32, i32* %3, align 4
  %5 = call i32 @len(i32 %4)
  %6 = call i32 @assert_eq_int(i32 %5, i32 4)
  ; Expression result: %6
  %7 = alloca [1x i32], align 4
  %8 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.42, i64 0, i64 0
  %9 = getelementptr inbounds [1x i32], [1x i32]* %7, i64 0, i64 0
  store i32 %8, i32* %9, align 4
  ; Expression result: %7
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %10
  ; Expression result: 1
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = alloca [1x i32], align 4
  %13 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.43, i64 0, i64 0
  %14 = getelementptr inbounds [1x i32], [1x i32]* %12, i64 0, i64 0
  store i32 %13, i32* %14, align 4
  ; Expression result: %12
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %15
  ; Expression result: 2
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %16
  %17 = alloca [1x i32], align 4
  %18 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.44, i64 0, i64 0
  %19 = getelementptr inbounds [1x i32], [1x i32]* %17, i64 0, i64 0
  store i32 %18, i32* %19, align 4
  ; Expression result: %17
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  ; Expression result: 3
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %21
  %22 = alloca [1x i32], align 4
  %23 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.45, i64 0, i64 0
  %24 = getelementptr inbounds [1x i32], [1x i32]* %22, i64 0, i64 0
  store i32 %23, i32* %24, align 4
  ; Expression result: %22
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %25
  ; Expression result: 4
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %26
  %27 = call i32 @range_step(i32 0, i32 10, i32 2)
  %28 = alloca i32, align 4
  store i32 %27, i32* %28, align 4
  ; Variable range_step_arr allocated
  %29 = load i32, i32* %28, align 4
  %30 = call i32 @len(i32 %29)
  %31 = call i32 @assert_eq_int(i32 %30, i32 5)
  ; Expression result: %31
  %32 = alloca [1x i32], align 4
  %33 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.42, i64 0, i64 0
  %34 = getelementptr inbounds [1x i32], [1x i32]* %32, i64 0, i64 0
  store i32 %33, i32* %34, align 4
  ; Expression result: %32
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %35
  ; Expression result: 0
  %36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %36
  %37 = alloca [1x i32], align 4
  %38 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.43, i64 0, i64 0
  %39 = getelementptr inbounds [1x i32], [1x i32]* %37, i64 0, i64 0
  store i32 %38, i32* %39, align 4
  ; Expression result: %37
  %40 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %40
  ; Expression result: 2
  %41 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %41
  %42 = alloca [1x i32], align 4
  %43 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.44, i64 0, i64 0
  %44 = getelementptr inbounds [1x i32], [1x i32]* %42, i64 0, i64 0
  store i32 %43, i32* %44, align 4
  ; Expression result: %42
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %45
  ; Expression result: 4
  %46 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %46
  %47 = alloca [1x i32], align 4
  %48 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.45, i64 0, i64 0
  %49 = getelementptr inbounds [1x i32], [1x i32]* %47, i64 0, i64 0
  store i32 %48, i32* %49, align 4
  ; Expression result: %47
  %50 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %50
  ; Expression result: 6
  %51 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %51
  %52 = alloca [1x i32], align 4
  %53 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.46, i64 0, i64 0
  %54 = getelementptr inbounds [1x i32], [1x i32]* %52, i64 0, i64 0
  store i32 %53, i32* %54, align 4
  ; Expression result: %52
  %55 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %55
  ; Expression result: 8
  %56 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %56
  %57 = alloca [7x i32], align 4
  %58 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.43, i64 0, i64 0
  %59 = getelementptr inbounds [7x i32], [7x i32]* %57, i64 0, i64 0
  store i32 %58, i32* %59, align 4
  %60 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.44, i64 0, i64 0
  %61 = getelementptr inbounds [7x i32], [7x i32]* %57, i64 0, i64 1
  store i32 %60, i32* %61, align 4
  %62 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.44, i64 0, i64 0
  %63 = getelementptr inbounds [7x i32], [7x i32]* %57, i64 0, i64 2
  store i32 %62, i32* %63, align 4
  %64 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.45, i64 0, i64 0
  %65 = getelementptr inbounds [7x i32], [7x i32]* %57, i64 0, i64 3
  store i32 %64, i32* %65, align 4
  %66 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.45, i64 0, i64 0
  %67 = getelementptr inbounds [7x i32], [7x i32]* %57, i64 0, i64 4
  store i32 %66, i32* %67, align 4
  %68 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.45, i64 0, i64 0
  %69 = getelementptr inbounds [7x i32], [7x i32]* %57, i64 0, i64 5
  store i32 %68, i32* %69, align 4
  %70 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.46, i64 0, i64 0
  %71 = getelementptr inbounds [7x i32], [7x i32]* %57, i64 0, i64 6
  store i32 %70, i32* %71, align 4
  %72 = alloca [7 x i32]*, align 4
  store [7 x i32]* %57, [7 x i32]** %72, align 4
  ; Variable arr_with_dupes allocated
  %73 = load [7 x i32]*, [7 x i32]** %72, align 4
  %74 = call i32 @unique(i32 %73)
  %75 = alloca i32, align 4
  store i32 %74, i32* %75, align 4
  ; Variable unique_arr allocated
  %76 = load i32, i32* %75, align 4
  %77 = call i32 @len(i32 %76)
  %78 = call i32 @assert_eq_int(i32 %77, i32 4)
  ; Expression result: %78
  %79 = load i32, i32* %75, align 4
  %80 = call i32 @array_contains(i32 %79, i32 1)
  %81 = call i32 @assert_true(i32 %80)
  ; Expression result: %81
  %82 = load i32, i32* %75, align 4
  %83 = call i32 @array_contains(i32 %82, i32 2)
  %84 = call i32 @assert_true(i32 %83)
  ; Expression result: %84
  %85 = load i32, i32* %75, align 4
  %86 = call i32 @array_contains(i32 %85, i32 3)
  %87 = call i32 @assert_true(i32 %86)
  ; Expression result: %87
  %88 = load i32, i32* %75, align 4
  %89 = call i32 @array_contains(i32 %88, i32 4)
  %90 = call i32 @assert_true(i32 %89)
  ; Expression result: %90
  %91 = alloca [7x i32], align 4
  %92 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.43, i64 0, i64 0
  %93 = getelementptr inbounds [7x i32], [7x i32]* %91, i64 0, i64 0
  store i32 %92, i32* %93, align 4
  %94 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.44, i64 0, i64 0
  %95 = getelementptr inbounds [7x i32], [7x i32]* %91, i64 0, i64 1
  store i32 %94, i32* %95, align 4
  %96 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.44, i64 0, i64 0
  %97 = getelementptr inbounds [7x i32], [7x i32]* %91, i64 0, i64 2
  store i32 %96, i32* %97, align 4
  %98 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.45, i64 0, i64 0
  %99 = getelementptr inbounds [7x i32], [7x i32]* %91, i64 0, i64 3
  store i32 %98, i32* %99, align 4
  %100 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.44, i64 0, i64 0
  %101 = getelementptr inbounds [7x i32], [7x i32]* %91, i64 0, i64 4
  store i32 %100, i32* %101, align 4
  %102 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.46, i64 0, i64 0
  %103 = getelementptr inbounds [7x i32], [7x i32]* %91, i64 0, i64 5
  store i32 %102, i32* %103, align 4
  %104 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.44, i64 0, i64 0
  %105 = getelementptr inbounds [7x i32], [7x i32]* %91, i64 0, i64 6
  store i32 %104, i32* %105, align 4
  %106 = alloca [7 x i32]*, align 4
  store [7 x i32]* %91, [7 x i32]** %106, align 4
  ; Variable count_arr allocated
  %107 = load [7 x i32]*, [7 x i32]** %106, align 4
  %108 = call i32 @count_occurrences(i32 %107, i32 2)
  %109 = call i32 @assert_eq_int(i32 %108, i32 4)
  ; Expression result: %109
  %110 = load [7 x i32]*, [7 x i32]** %106, align 4
  %111 = call i32 @count_occurrences(i32 %110, i32 1)
  %112 = call i32 @assert_eq_int(i32 %111, i32 1)
  ; Expression result: %112
  %113 = load [7 x i32]*, [7 x i32]** %106, align 4
  %114 = call i32 @count_occurrences(i32 %113, i32 5)
  %115 = call i32 @assert_eq_int(i32 %114, i32 0)
  ; Expression result: %115
  ret i32 0
}

define i32 @test_array_conversion() {
entry:
  %0 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.47, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @set_new()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable s allocated
  %4 = load i32, i32* %3, align 4
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = load i32, i32* %3, align 4
  %7 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.48, i64 0, i64 0
  %8 = call i32 @set_add(i32 %6, i32 %7)
  ; Expression result: %8
  %9 = load i32, i32* %3, align 4
  ; Expression result: %9
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %10
  %11 = load i32, i32* %3, align 4
  %12 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.49, i64 0, i64 0
  %13 = call i32 @set_add(i32 %11, i32 %12)
  ; Expression result: %13
  %14 = load i32, i32* %3, align 4
  ; Expression result: %14
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %15
  %16 = load i32, i32* %3, align 4
  %17 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.50, i64 0, i64 0
  %18 = call i32 @set_add(i32 %16, i32 %17)
  ; Expression result: %18
  %19 = load i32, i32* %3, align 4
  %20 = call i32 @set_to_array(i32 %19)
  %21 = alloca i32, align 4
  store i32 %20, i32* %21, align 4
  ; Variable arr allocated
  %22 = load i32, i32* %21, align 4
  %23 = call i32 @len(i32 %22)
  %24 = call i32 @assert_eq_int(i32 %23, i32 3)
  ; Expression result: %24
  %25 = load i32, i32* %21, align 4
  %26 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.48, i64 0, i64 0
  %27 = call i32 @array_contains(i32 %25, i32 %26)
  %28 = call i32 @assert_true(i32 %27)
  ; Expression result: %28
  %29 = load i32, i32* %21, align 4
  %30 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.49, i64 0, i64 0
  %31 = call i32 @array_contains(i32 %29, i32 %30)
  %32 = call i32 @assert_true(i32 %31)
  ; Expression result: %32
  %33 = load i32, i32* %21, align 4
  %34 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.50, i64 0, i64 0
  %35 = call i32 @array_contains(i32 %33, i32 %34)
  %36 = call i32 @assert_true(i32 %35)
  ; Expression result: %36
  %37 = alloca [0x i32], align 4
  %38 = alloca [0 x i32]*, align 4
  store [0 x i32]* %37, [0 x i32]** %38, align 4
  ; Variable arr2 allocated
  %39 = load [0 x i32]*, [0 x i32]** %38, align 4
  %40 = call i32 @set_from_array(i32 %39)
  %41 = alloca i32, align 4
  store i32 %40, i32* %41, align 4
  ; Variable s2 allocated
  %42 = load i32, i32* %41, align 4
  %43 = call i32 @set_len(i32 %42)
  %44 = call i32 @assert_eq_int(i32 %43, i32 3)
  ; Expression result: %44
  %45 = load i32, i32* %41, align 4
  %46 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.29, i64 0, i64 0
  %47 = call i32 @set_contains(i32 %45, i32 %46)
  %48 = call i32 @assert_true(i32 %47)
  ; Expression result: %48
  %49 = load i32, i32* %41, align 4
  %50 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.30, i64 0, i64 0
  %51 = call i32 @set_contains(i32 %49, i32 %50)
  %52 = call i32 @assert_true(i32 %51)
  ; Expression result: %52
  %53 = load i32, i32* %41, align 4
  %54 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.31, i64 0, i64 0
  %55 = call i32 @set_contains(i32 %53, i32 %54)
  %56 = call i32 @assert_true(i32 %55)
  ; Expression result: %56
  ret i32 0
}

define i32 @test_collections_edge_cases() {
entry:
  %0 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.51, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @array_new()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable empty_arr allocated
  ; Expression result: 1
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %4
  %5 = load i32, i32* %3, align 4
  %6 = call i32 @array_contains(i32 %5, i32 1)
  %7 = call i32 @assert_false(i32 %6)
  ; Expression result: %7
  %8 = call i32 @map_new()
  %9 = alloca i32, align 4
  store i32 %8, i32* %9, align 4
  ; Variable empty_map allocated
  %10 = load i32, i32* %9, align 4
  %11 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.52, i64 0, i64 0
  %12 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.12, i64 0, i64 0
  %13 = call i32 @map_get_or_default(i32 %10, i32 %11, i32 %12)
  %14 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.12, i64 0, i64 0
  %15 = call i32 @assert_eq_string(i32 %13, i32 %14)
  ; Expression result: %15
  %16 = load i32, i32* %9, align 4
  %17 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.52, i64 0, i64 0
  %18 = call i32 @map_contains_key(i32 %16, i32 %17)
  %19 = call i32 @assert_false(i32 %18)
  ; Expression result: %19
  %20 = call i32 @set_new()
  %21 = alloca i32, align 4
  store i32 %20, i32* %21, align 4
  ; Variable empty_set allocated
  %22 = load i32, i32* %21, align 4
  %23 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.53, i64 0, i64 0
  %24 = call i32 @set_contains(i32 %22, i32 %23)
  %25 = call i32 @assert_false(i32 %24)
  ; Expression result: %25
  %26 = load i32, i32* %21, align 4
  %27 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.53, i64 0, i64 0
  %28 = call i32 @set_remove(i32 %26, i32 %27)
  %29 = call i32 @assert_false(i32 %28)
  ; Expression result: %29
  %30 = call i32 @array_new()
  %31 = alloca i32, align 4
  store i32 %30, i32* %31, align 4
  ; Variable single_arr allocated
  %32 = load i32, i32* %31, align 4
  ; Expression result: %32
  %33 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %33
  %34 = load i32, i32* %31, align 4
  %35 = call i32 @array_push(i32 %34, i32 42)
  ; Expression result: %35
  %36 = load i32, i32* %31, align 4
  %37 = call i32 @array_pop(i32 %36)
  %38 = call i32 @assert_eq_int(i32 %37, i32 42)
  ; Expression result: %38
  %39 = load i32, i32* %31, align 4
  %40 = call i32 @array_is_empty(i32 %39)
  %41 = call i32 @assert_true(i32 %40)
  ; Expression result: %41
  ret i32 0
}

define i32 @test_collections_clear() {
entry:
  %0 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.54, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @array_new()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable arr allocated
  %4 = load i32, i32* %3, align 4
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  %6 = load i32, i32* %3, align 4
  %7 = call i32 @array_push(i32 %6, i32 1)
  ; Expression result: %7
  %8 = load i32, i32* %3, align 4
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %9
  %10 = load i32, i32* %3, align 4
  %11 = call i32 @array_push(i32 %10, i32 2)
  ; Expression result: %11
  %12 = load i32, i32* %3, align 4
  ; Expression result: %12
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %13
  %14 = load i32, i32* %3, align 4
  %15 = call i32 @array_clear(i32 %14)
  ; Expression result: %15
  %16 = load i32, i32* %3, align 4
  %17 = call i32 @array_is_empty(i32 %16)
  %18 = call i32 @assert_true(i32 %17)
  ; Expression result: %18
  %19 = call i32 @map_new()
  %20 = alloca i32, align 4
  store i32 %19, i32* %20, align 4
  ; Variable m allocated
  %21 = load i32, i32* %20, align 4
  ; Expression result: %21
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %22
  %23 = load i32, i32* %20, align 4
  %24 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.52, i64 0, i64 0
  %25 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.55, i64 0, i64 0
  %26 = call i32 @map_set(i32 %23, i32 %24, i32 %25)
  ; Expression result: %26
  %27 = load i32, i32* %20, align 4
  ; Expression result: %27
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %28
  %29 = load i32, i32* %20, align 4
  %30 = call i32 @map_clear(i32 %29)
  ; Expression result: %30
  %31 = load i32, i32* %20, align 4
  %32 = call i32 @map_is_empty(i32 %31)
  %33 = call i32 @assert_true(i32 %32)
  ; Expression result: %33
  %34 = call i32 @set_new()
  %35 = alloca i32, align 4
  store i32 %34, i32* %35, align 4
  ; Variable s allocated
  %36 = load i32, i32* %35, align 4
  ; Expression result: %36
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %37
  %38 = load i32, i32* %35, align 4
  %39 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.53, i64 0, i64 0
  %40 = call i32 @set_add(i32 %38, i32 %39)
  ; Expression result: %40
  %41 = load i32, i32* %35, align 4
  ; Expression result: %41
  %42 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %42
  %43 = load i32, i32* %35, align 4
  %44 = call i32 @set_clear(i32 %43)
  ; Expression result: %44
  %45 = load i32, i32* %35, align 4
  %46 = call i32 @set_is_empty(i32 %45)
  %47 = call i32 @assert_true(i32 %46)
  ; Expression result: %47
  %48 = call i32 @queue_new()
  %49 = alloca i32, align 4
  store i32 %48, i32* %49, align 4
  ; Variable q allocated
  %50 = load i32, i32* %49, align 4
  ; Expression result: %50
  %51 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %51
  %52 = load i32, i32* %49, align 4
  %53 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.53, i64 0, i64 0
  %54 = call i32 @queue_enqueue(i32 %52, i32 %53)
  ; Expression result: %54
  %55 = load i32, i32* %49, align 4
  ; Expression result: %55
  %56 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %56
  %57 = load i32, i32* %49, align 4
  %58 = call i32 @queue_clear(i32 %57)
  ; Expression result: %58
  %59 = load i32, i32* %49, align 4
  %60 = call i32 @queue_is_empty(i32 %59)
  %61 = call i32 @assert_true(i32 %60)
  ; Expression result: %61
  %62 = call i32 @stack_new()
  %63 = alloca i32, align 4
  store i32 %62, i32* %63, align 4
  ; Variable st allocated
  %64 = load i32, i32* %63, align 4
  ; Expression result: %64
  %65 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %65
  %66 = load i32, i32* %63, align 4
  %67 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.53, i64 0, i64 0
  %68 = call i32 @stack_push(i32 %66, i32 %67)
  ; Expression result: %68
  %69 = load i32, i32* %63, align 4
  ; Expression result: %69
  %70 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %70
  %71 = load i32, i32* %63, align 4
  %72 = call i32 @stack_clear(i32 %71)
  ; Expression result: %72
  %73 = load i32, i32* %63, align 4
  %74 = call i32 @stack_is_empty(i32 %73)
  %75 = call i32 @assert_true(i32 %74)
  ; Expression result: %75
  ret i32 0
}

define i32 @run_all_collections_tests() {
entry:
  %0 = getelementptr inbounds [46 x i8], [46 x i8]* @.str.56, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [42 x i8], [42 x i8]* @.str.57, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = call i32 @test_array_operations()
  ; Expression result: %6
  %7 = call i32 @test_array_search()
  ; Expression result: %7
  %8 = call i32 @test_array_manipulation()
  ; Expression result: %8
  %9 = call i32 @test_map_operations()
  ; Expression result: %9
  %10 = call i32 @test_map_collections()
  ; Expression result: %10
  %11 = call i32 @test_set_operations()
  ; Expression result: %11
  %12 = call i32 @test_set_operations_advanced()
  ; Expression result: %12
  %13 = call i32 @test_queue_operations()
  ; Expression result: %13
  %14 = call i32 @test_stack_operations()
  ; Expression result: %14
  %15 = call i32 @test_utility_functions()
  ; Expression result: %15
  %16 = call i32 @test_array_conversion()
  ; Expression result: %16
  %17 = call i32 @test_collections_edge_cases()
  ; Expression result: %17
  %18 = call i32 @test_collections_clear()
  ; Expression result: %18
  %19 = call i32 @run_all_hashmap_tests()
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
@.str.18 = private unnamed_addr constant [5 x i8] c"city\00", align 1
@.str.31 = private unnamed_addr constant [2 x i8] c"c\00", align 1
@.str.1 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.26 = private unnamed_addr constant [7 x i8] c"cherry\00", align 1
@.str.9 = private unnamed_addr constant [5 x i8] c"key3\00", align 1
@.str.45 = private unnamed_addr constant [2 x i8] c"3\00", align 1
@.str.7 = private unnamed_addr constant [5 x i8] c"key2\00", align 1
@.str.32 = private unnamed_addr constant [2 x i8] c"d\00", align 1
@.str.0 = private unnamed_addr constant [17 x i8] c"Array Operations\00", align 1
@.str.27 = private unnamed_addr constant [7 x i8] c"orange\00", align 1
@.str.48 = private unnamed_addr constant [2 x i8] c"x\00", align 1
@.str.36 = private unnamed_addr constant [6 x i8] c"third\00", align 1
@.str.22 = private unnamed_addr constant [3 x i8] c"31\00", align 1
@.str.49 = private unnamed_addr constant [2 x i8] c"y\00", align 1
@.str.25 = private unnamed_addr constant [7 x i8] c"banana\00", align 1
@.str.21 = private unnamed_addr constant [4 x i8] c"USA\00", align 1
@.str.53 = private unnamed_addr constant [5 x i8] c"item\00", align 1
@.str.3 = private unnamed_addr constant [19 x i8] c"Array Manipulation\00", align 1
@.str.10 = private unnamed_addr constant [7 x i8] c"value3\00", align 1
@.str.19 = private unnamed_addr constant [9 x i8] c"New York\00", align 1
@.str.5 = private unnamed_addr constant [5 x i8] c"key1\00", align 1
@.str.43 = private unnamed_addr constant [2 x i8] c"1\00", align 1
@.str.47 = private unnamed_addr constant [17 x i8] c"Array Conversion\00", align 1
@.str.42 = private unnamed_addr constant [2 x i8] c"0\00", align 1
@.str.24 = private unnamed_addr constant [6 x i8] c"apple\00", align 1
@.str.30 = private unnamed_addr constant [2 x i8] c"b\00", align 1
@.str.14 = private unnamed_addr constant [5 x i8] c"name\00", align 1
@.str.41 = private unnamed_addr constant [18 x i8] c"Utility Functions\00", align 1
@.str.57 = private unnamed_addr constant [42 x i8] c"=========================================\00", align 1
@.str.6 = private unnamed_addr constant [7 x i8] c"value1\00", align 1
@.str.56 = private unnamed_addr constant [46 x i8] c"📦 Running CURSED Collections Library Tests\00", align 1
@.str.11 = private unnamed_addr constant [12 x i8] c"nonexistent\00", align 1
@.str.33 = private unnamed_addr constant [17 x i8] c"Queue Operations\00", align 1
@.str.8 = private unnamed_addr constant [7 x i8] c"value2\00", align 1
@.str.40 = private unnamed_addr constant [4 x i8] c"top\00", align 1
@.str.4 = private unnamed_addr constant [15 x i8] c"Map Operations\00", align 1
@.str.46 = private unnamed_addr constant [2 x i8] c"4\00", align 1
@.str.34 = private unnamed_addr constant [6 x i8] c"first\00", align 1
@.str.39 = private unnamed_addr constant [7 x i8] c"middle\00", align 1
@.str.2 = private unnamed_addr constant [24 x i8] c"Array Search Operations\00", align 1
@.str.52 = private unnamed_addr constant [4 x i8] c"key\00", align 1
@.str.51 = private unnamed_addr constant [23 x i8] c"Collections Edge Cases\00", align 1
@.str.35 = private unnamed_addr constant [7 x i8] c"second\00", align 1
@.str.23 = private unnamed_addr constant [15 x i8] c"Set Operations\00", align 1
@.str.12 = private unnamed_addr constant [8 x i8] c"default\00", align 1
@.str.55 = private unnamed_addr constant [6 x i8] c"value\00", align 1
@.str.15 = private unnamed_addr constant [6 x i8] c"Alice\00", align 1
@.str.16 = private unnamed_addr constant [4 x i8] c"age\00", align 1
@.str.29 = private unnamed_addr constant [2 x i8] c"a\00", align 1
@.str.50 = private unnamed_addr constant [2 x i8] c"z\00", align 1
@.str.37 = private unnamed_addr constant [17 x i8] c"Stack Operations\00", align 1
@.str.17 = private unnamed_addr constant [3 x i8] c"30\00", align 1
@.str.38 = private unnamed_addr constant [7 x i8] c"bottom\00", align 1
@.str.54 = private unnamed_addr constant [29 x i8] c"Collections Clear Operations\00", align 1
@.str.28 = private unnamed_addr constant [24 x i8] c"Set Advanced Operations\00", align 1
@.str.44 = private unnamed_addr constant [2 x i8] c"2\00", align 1
@.str.13 = private unnamed_addr constant [16 x i8] c"Map Collections\00", align 1
@.str.20 = private unnamed_addr constant [8 x i8] c"country\00", align 1
define i32 @main() {
  %0 = call i32 @run_all_collections_tests()
  ret i32 0
}
