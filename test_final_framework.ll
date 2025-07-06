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

define i32 @test_pass(i8* %message) {
entry:
  %0 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.0, i64 0, i64 0
  %1 = call i8* @string_concat(i8* %0, i8* %message)
  %2 = call i32 @puts(i8* %1)
  %3 = add i32 0, 0
  ; Expression result: %3
  ret i32 0
}

define i32 @test_fail(i8* %message) {
entry:
  %0 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.1, i64 0, i64 0
  %1 = call i8* @string_concat(i8* %0, i8* %message)
  %2 = call i32 @puts(i8* %1)
  %3 = add i32 0, 0
  ; Expression result: %3
  ret i32 0
}

define i32 @test_start(i8* %name) {
entry:
  %0 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.2, i64 0, i64 0
  %1 = call i8* @string_concat(i8* %0, i8* %name)
  %2 = call i32 @puts(i8* %1)
  %3 = add i32 0, 0
  ; Expression result: %3
  ret i32 0
}

define void @assert_eq(i32 %actual, i32 %expected) {
entry:
  %0 = icmp eq i32 %actual, %expected
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.3, i64 0, i64 0
  %2 = call i8* @i32_to_string(i32 %actual)
  %3 = call i8* @string_concat(i8* %1, i8* %2)
  %4 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.4, i64 0, i64 0
  %5 = call i8* @string_concat(i8* %3, i8* %4)
  %6 = add i32 %5, %expected
  %7 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.5, i64 0, i64 0
  %8 = call i8* @string_concat(i8* %6, i8* %7)
  %9 = call i32 @test_pass(i8* %8)
  ; Expression result: %9
  br label %label2
label1:
  %10 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.6, i64 0, i64 0
  %11 = call i8* @i32_to_string(i32 %actual)
  %12 = call i8* @string_concat(i8* %10, i8* %11)
  %13 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.7, i64 0, i64 0
  %14 = call i8* @string_concat(i8* %12, i8* %13)
  %15 = add i32 %14, %expected
  %16 = call i32 @test_fail(i8* %15)
  ; Expression result: %16
  br label %label2
label2:
  ret void
}

define void @assert_eq_string(i8* %actual, i8* %expected) {
entry:
  %0 = icmp eq i32 %actual, %expected
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.8, i64 0, i64 0
  %2 = call i8* @string_concat(i8* %1, i8* %actual)
  %3 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.9, i64 0, i64 0
  %4 = call i8* @string_concat(i8* %2, i8* %3)
  %5 = add i32 %4, %expected
  %6 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.10, i64 0, i64 0
  %7 = call i8* @string_concat(i8* %5, i8* %6)
  %8 = call i32 @test_pass(i8* %7)
  ; Expression result: %8
  br label %label2
label1:
  %9 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.11, i64 0, i64 0
  %10 = call i8* @string_concat(i8* %9, i8* %actual)
  %11 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.12, i64 0, i64 0
  %12 = call i8* @string_concat(i8* %10, i8* %11)
  %13 = add i32 %12, %expected
  %14 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.13, i64 0, i64 0
  %15 = call i8* @string_concat(i8* %13, i8* %14)
  %16 = call i32 @test_fail(i8* %15)
  ; Expression result: %16
  br label %label2
label2:
  ret void
}

define void @assert_true(i1 %value) {
entry:
  %0 = icmp eq i1 %value, 1
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.14, i64 0, i64 0
  %2 = call i8* @string_concat(i8* %1, i8* %value)
  %3 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.5, i64 0, i64 0
  %4 = call i8* @string_concat(i8* %2, i8* %3)
  %5 = call i32 @test_pass(i8* %4)
  ; Expression result: %5
  br label %label2
label1:
  %6 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.15, i64 0, i64 0
  %7 = call i8* @string_concat(i8* %6, i8* %value)
  %8 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.16, i64 0, i64 0
  %9 = call i8* @string_concat(i8* %7, i8* %8)
  %10 = call i32 @test_fail(i8* %9)
  ; Expression result: %10
  br label %label2
label2:
  ret void
}

define void @assert_false(i1 %value) {
entry:
  %0 = icmp eq i1 %value, 0
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.17, i64 0, i64 0
  %2 = call i8* @string_concat(i8* %1, i8* %value)
  %3 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.5, i64 0, i64 0
  %4 = call i8* @string_concat(i8* %2, i8* %3)
  %5 = call i32 @test_pass(i8* %4)
  ; Expression result: %5
  br label %label2
label1:
  %6 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.18, i64 0, i64 0
  %7 = call i8* @string_concat(i8* %6, i8* %value)
  %8 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.19, i64 0, i64 0
  %9 = call i8* @string_concat(i8* %7, i8* %8)
  %10 = call i32 @test_fail(i8* %9)
  ; Expression result: %10
  br label %label2
label2:
  ret void
}

define void @assert_ne(i32 %actual, i32 %expected) {
entry:
  %0 = icmp ne i32 %actual, %expected
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.20, i64 0, i64 0
  %2 = call i8* @i32_to_string(i32 %actual)
  %3 = call i8* @string_concat(i8* %1, i8* %2)
  %4 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.4, i64 0, i64 0
  %5 = call i8* @string_concat(i8* %3, i8* %4)
  %6 = add i32 %5, %expected
  %7 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.5, i64 0, i64 0
  %8 = call i8* @string_concat(i8* %6, i8* %7)
  %9 = call i32 @test_pass(i8* %8)
  ; Expression result: %9
  br label %label2
label1:
  %10 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.21, i64 0, i64 0
  %11 = call i8* @i32_to_string(i32 %actual)
  %12 = call i8* @string_concat(i8* %10, i8* %11)
  %13 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.22, i64 0, i64 0
  %14 = call i8* @string_concat(i8* %12, i8* %13)
  %15 = add i32 %14, %expected
  %16 = call i32 @test_fail(i8* %15)
  ; Expression result: %16
  br label %label2
label2:
  ret void
}

define void @assert_greater(i32 %actual, i32 %expected) {
entry:
  %0 = icmp sgt i32 %actual, %expected
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.23, i64 0, i64 0
  %2 = call i8* @i32_to_string(i32 %actual)
  %3 = call i8* @string_concat(i8* %1, i8* %2)
  %4 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.4, i64 0, i64 0
  %5 = call i8* @string_concat(i8* %3, i8* %4)
  %6 = add i32 %5, %expected
  %7 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.5, i64 0, i64 0
  %8 = call i8* @string_concat(i8* %6, i8* %7)
  %9 = call i32 @test_pass(i8* %8)
  ; Expression result: %9
  br label %label2
label1:
  %10 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.24, i64 0, i64 0
  %11 = call i8* @i32_to_string(i32 %actual)
  %12 = call i8* @string_concat(i8* %10, i8* %11)
  %13 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.25, i64 0, i64 0
  %14 = call i8* @string_concat(i8* %12, i8* %13)
  %15 = add i32 %14, %expected
  %16 = call i32 @test_fail(i8* %15)
  ; Expression result: %16
  br label %label2
label2:
  ret void
}

define void @assert_less(i32 %actual, i32 %expected) {
entry:
  %0 = icmp slt i32 %actual, %expected
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.26, i64 0, i64 0
  %2 = call i8* @i32_to_string(i32 %actual)
  %3 = call i8* @string_concat(i8* %1, i8* %2)
  %4 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.4, i64 0, i64 0
  %5 = call i8* @string_concat(i8* %3, i8* %4)
  %6 = add i32 %5, %expected
  %7 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.5, i64 0, i64 0
  %8 = call i8* @string_concat(i8* %6, i8* %7)
  %9 = call i32 @test_pass(i8* %8)
  ; Expression result: %9
  br label %label2
label1:
  %10 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.27, i64 0, i64 0
  %11 = call i8* @i32_to_string(i32 %actual)
  %12 = call i8* @string_concat(i8* %10, i8* %11)
  %13 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.28, i64 0, i64 0
  %14 = call i8* @string_concat(i8* %12, i8* %13)
  %15 = add i32 %14, %expected
  %16 = call i32 @test_fail(i8* %15)
  ; Expression result: %16
  br label %label2
label2:
  ret void
}

define i32 @test_basic_math() {
entry:
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.29, i64 0, i64 0
  %1 = call i32 @test_start(i8* %0)
  ; Expression result: %1
  %2 = add i32 2, 2
  %3 = call i32 @assert_eq(i8* %2, i32 4)
  ; Expression result: %3
  %4 = mul i32 3, 7
  %5 = call i32 @assert_eq(i32 %4, i32 21)
  ; Expression result: %5
  %6 = sub i32 10, 3
  %7 = call i32 @assert_eq(i32 %6, i32 7)
  ; Expression result: %7
  %8 = sdiv i32 20, 4
  %9 = call i32 @assert_eq(i32 %8, i32 5)
  ; Expression result: %9
  ret i32 0
}

define i32 @test_string_operations() {
entry:
  %0 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.30, i64 0, i64 0
  %1 = call i32 @test_start(i8* %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.31, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable greeting allocated
  %4 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.32, i64 0, i64 0
  %5 = alloca i8*, align 4
  store i8* %4, i8** %5, align 4
  ; Variable target allocated
  %6 = load i8*, i8** %3, align 4
  %7 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.33, i64 0, i64 0
  %8 = call i8* @string_concat(i8* %6, i8* %7)
  %9 = load i8*, i8** %5, align 4
  %10 = add i32 %8, %9
  %11 = alloca i8*, align 4
  store i8* %10, i8** %11, align 4
  ; Variable full_greeting allocated
  %12 = load i8*, i8** %11, align 4
  %13 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.34, i64 0, i64 0
  %14 = call i32 @assert_eq_string(i32 %12, i8* %13)
  ; Expression result: %14
  %15 = load i8*, i8** %3, align 4
  %16 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.31, i64 0, i64 0
  %17 = call i32 @assert_eq_string(i32 %15, i8* %16)
  ; Expression result: %17
  %18 = load i8*, i8** %5, align 4
  %19 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.32, i64 0, i64 0
  %20 = call i32 @assert_eq_string(i32 %18, i8* %19)
  ; Expression result: %20
  ret i32 0
}

define i32 @test_boolean_logic() {
entry:
  %0 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.35, i64 0, i64 0
  %1 = call i32 @test_start(i8* %0)
  ; Expression result: %1
  %2 = call i32 @assert_true(i1 1)
  ; Expression result: %2
  %3 = call i32 @assert_false(i1 0)
  ; Expression result: %3
  %4 = icmp sgt i32 5, 3
  %5 = call i32 @assert_true(i32 %4)
  ; Expression result: %5
  %6 = icmp sgt i32 3, 5
  %7 = call i32 @assert_false(i32 %6)
  ; Expression result: %7
  %8 = and i1 1, 1
  %9 = call i32 @assert_true(i32 %8)
  ; Expression result: %9
  %10 = and i1 0, 1
  %11 = call i32 @assert_false(i32 %10)
  ; Expression result: %11
  %12 = or i1 1, 0
  %13 = call i32 @assert_true(i32 %12)
  ; Expression result: %13
  %14 = or i1 0, 0
  %15 = call i32 @assert_false(i32 %14)
  ; Expression result: %15
  ret i32 0
}

define i32 @test_comparisons() {
entry:
  %0 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.36, i64 0, i64 0
  %1 = call i32 @test_start(i8* %0)
  ; Expression result: %1
  %2 = alloca i32, align 4
  store i32 10, i32* %2, align 4
  ; Variable x allocated
  %3 = alloca i32, align 4
  store i32 5, i32* %3, align 4
  ; Variable y allocated
  %4 = load i32, i32* %2, align 4
  %5 = load i32, i32* %3, align 4
  %6 = call i32 @assert_greater(i32 %4, i32 %5)
  ; Expression result: %6
  %7 = load i32, i32* %3, align 4
  %8 = load i32, i32* %2, align 4
  %9 = call i32 @assert_less(i32 %7, i32 %8)
  ; Expression result: %9
  %10 = load i32, i32* %2, align 4
  %11 = load i32, i32* %3, align 4
  %12 = call i32 @assert_ne(i32 %10, i32 %11)
  ; Expression result: %12
  %13 = load i32, i32* %2, align 4
  %14 = call i32 @assert_eq(i32 %13, i32 10)
  ; Expression result: %14
  %15 = load i32, i32* %3, align 4
  %16 = call i32 @assert_eq(i32 %15, i32 5)
  ; Expression result: %16
  ret i32 0
}

define i32 @test_variables() {
entry:
  %0 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.37, i64 0, i64 0
  %1 = call i32 @test_start(i8* %0)
  ; Expression result: %1
  %2 = alloca i32, align 4
  store i32 42, i32* %2, align 4
  ; Variable number allocated
  %3 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.38, i64 0, i64 0
  %4 = alloca i8*, align 4
  store i8* %3, i8** %4, align 4
  ; Variable text allocated
  %5 = alloca i1, align 4
  store i1 1, i1* %5, align 4
  ; Variable flag allocated
  %6 = load i32, i32* %2, align 4
  %7 = call i32 @assert_eq(i32 %6, i32 42)
  ; Expression result: %7
  %8 = load i8*, i8** %4, align 4
  %9 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.38, i64 0, i64 0
  %10 = call i32 @assert_eq_string(i8* %8, i8* %9)
  ; Expression result: %10
  %11 = load i1, i1* %5, align 4
  %12 = call i32 @assert_true(i32 %11)
  ; Expression result: %12
  ret i32 0
}

define i32 @test_arithmetic() {
entry:
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.39, i64 0, i64 0
  %1 = call i32 @test_start(i8* %0)
  ; Expression result: %1
  %2 = alloca i32, align 4
  store i32 15, i32* %2, align 4
  ; Variable a allocated
  %3 = alloca i32, align 4
  store i32 3, i32* %3, align 4
  ; Variable b allocated
  %4 = load i32, i32* %2, align 4
  %5 = load i32, i32* %3, align 4
  %6 = add i32 %4, %5
  %7 = call i32 @assert_eq(i8* %6, i32 18)
  ; Expression result: %7
  %8 = load i32, i32* %2, align 4
  %9 = load i32, i32* %3, align 4
  %10 = sub i32 %8, %9
  %11 = call i32 @assert_eq(i32 %10, i32 12)
  ; Expression result: %11
  %12 = load i32, i32* %2, align 4
  %13 = load i32, i32* %3, align 4
  %14 = mul i32 %12, %13
  %15 = call i32 @assert_eq(i32 %14, i32 45)
  ; Expression result: %15
  %16 = load i32, i32* %2, align 4
  %17 = load i32, i32* %3, align 4
  %18 = sdiv i32 %16, %17
  %19 = call i32 @assert_eq(i32 %18, i32 5)
  ; Expression result: %19
  ret i32 0
}

define i32 @test_function_calls() {
entry:
  %0 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.40, i64 0, i64 0
  %1 = call i32 @test_start(i8* %0)
  ; Expression result: %1
  %2 = call i32 @add_two(i32 5, i32 3)
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable result1 allocated
  %4 = load i32, i32* %3, align 4
  %5 = call i32 @assert_eq(i32 %4, i32 8)
  ; Expression result: %5
  %6 = call i32 @multiply_by_three(i32 4)
  %7 = alloca i32, align 4
  store i32 %6, i32* %7, align 4
  ; Variable result2 allocated
  %8 = load i32, i32* %7, align 4
  %9 = call i32 @assert_eq(i32 %8, i32 12)
  ; Expression result: %9
  %10 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.41, i64 0, i64 0
  %11 = call i32 @say_hello(i8* %10)
  %12 = alloca i32, align 4
  store i32 %11, i32* %12, align 4
  ; Variable greeting allocated
  %13 = load i32, i32* %12, align 4
  %14 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.42, i64 0, i64 0
  %15 = call i32 @assert_eq_string(i32 %13, i8* %14)
  ; Expression result: %15
  ret i32 0
}

define void @test_conditional_logic() {
entry:
  %0 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.43, i64 0, i64 0
  %1 = call i32 @test_start(i8* %0)
  ; Expression result: %1
  %2 = icmp sgt i32 10, 5
  %3 = alloca i1, align 4
  store i1 %2, i1* %3, align 4
  ; Variable condition1 allocated
  %4 = icmp slt i32 3, 2
  %5 = alloca i1, align 4
  store i1 %4, i1* %5, align 4
  ; Variable condition2 allocated
  %6 = load i1, i1* %3, align 4
  %7 = call i32 @assert_true(i32 %6)
  ; Expression result: %7
  %8 = load i1, i1* %5, align 4
  %9 = call i32 @assert_false(i32 %8)
  ; Expression result: %9
  %10 = load i1, i1* %3, align 4
  br i1 %10, label %label0, label %label1
label0:
  %11 = call i32 @assert_true(i1 1)
  ; Expression result: %11
  br label %label2
label1:
  %12 = call i32 @assert_true(i1 0)
  ; Expression result: %12
  br label %label2
label2:
  ret void
}

define i32 @test_failing_cases() {
entry:
  %0 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.44, i64 0, i64 0
  %1 = call i32 @test_start(i8* %0)
  ; Expression result: %1
  %2 = add i32 2, 2
  %3 = call i32 @assert_eq(i8* %2, i32 5)
  ; Expression result: %3
  %4 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.45, i64 0, i64 0
  %5 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.46, i64 0, i64 0
  %6 = call i32 @assert_eq_string(i8* %4, i8* %5)
  ; Expression result: %6
  %7 = call i32 @assert_true(i1 0)
  ; Expression result: %7
  %8 = call i32 @assert_false(i1 1)
  ; Expression result: %8
  ret i32 0
}

define i32 @add_two(i32 %a, i32 %b) {
entry:
  %0 = add i32 %a, %b
  ret i32 %0
}

define i32 @multiply_by_three(i32 %n) {
entry:
  %0 = mul i32 %n, 3
  ret i32 %0
}

define i8* @say_hello(i8* %name) {
entry:
  %0 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.47, i64 0, i64 0
  %1 = call i8* @string_concat(i8* %0, i8* %name)
  %2 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.48, i64 0, i64 0
  %3 = call i8* @string_concat(i8* %1, i8* %2)
  ret i8* %3
}

define i32 @run_all_passing_tests() {
entry:
  %0 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.49, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = call i32 @test_basic_math()
  ; Expression result: %3
  %4 = call i32 @test_string_operations()
  ; Expression result: %4
  %5 = call i32 @test_boolean_logic()
  ; Expression result: %5
  %6 = call i32 @test_comparisons()
  ; Expression result: %6
  %7 = call i32 @test_variables()
  ; Expression result: %7
  %8 = call i32 @test_arithmetic()
  ; Expression result: %8
  %9 = call i32 @test_function_calls()
  ; Expression result: %9
  %10 = call i32 @test_conditional_logic()
  ; Expression result: %10
  ret i32 0
}

define i32 @run_failing_tests() {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.50, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.51, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = call i32 @test_failing_cases()
  ; Expression result: %6
  ret i32 0
}

define i32 @demo_assertions() {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.50, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.52, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.53, i64 0, i64 0
  %7 = call i32 @test_start(i8* %6)
  ; Expression result: %7
  %8 = call i32 @assert_eq(i32 42, i32 42)
  ; Expression result: %8
  %9 = call i32 @assert_ne(i32 42, i32 24)
  ; Expression result: %9
  %10 = call i32 @assert_greater(i32 10, i32 5)
  ; Expression result: %10
  %11 = call i32 @assert_less(i32 5, i32 10)
  ; Expression result: %11
  %12 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.45, i64 0, i64 0
  %13 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.45, i64 0, i64 0
  %14 = call i32 @assert_eq_string(i8* %12, i8* %13)
  ; Expression result: %14
  %15 = call i32 @assert_true(i1 1)
  ; Expression result: %15
  %16 = call i32 @assert_false(i1 0)
  ; Expression result: %16
  ret i32 0
}


; String constants
@.str.4 = private unnamed_addr constant [3 x i8] c", \00", align 1
@.str.46 = private unnamed_addr constant [8 x i8] c"goodbye\00", align 1
@.str.9 = private unnamed_addr constant [5 x i8] c"\", \"\00", align 1
@.str.41 = private unnamed_addr constant [7 x i8] c"CURSED\00", align 1
@.str.31 = private unnamed_addr constant [6 x i8] c"Hello\00", align 1
@.str.7 = private unnamed_addr constant [12 x i8] c", expected \00", align 1
@.str.21 = private unnamed_addr constant [23 x i8] c"assert_ne failed: got \00", align 1
@.str.39 = private unnamed_addr constant [16 x i8] c"test_arithmetic\00", align 1
@.str.10 = private unnamed_addr constant [3 x i8] c"\")\00", align 1
@.str.15 = private unnamed_addr constant [25 x i8] c"assert_true failed: got \00", align 1
@.str.33 = private unnamed_addr constant [2 x i8] c" \00", align 1
@.str.45 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
@.str.48 = private unnamed_addr constant [2 x i8] c"!\00", align 1
@.str.30 = private unnamed_addr constant [23 x i8] c"test_string_operations\00", align 1
@.str.43 = private unnamed_addr constant [23 x i8] c"test_conditional_logic\00", align 1
@.str.19 = private unnamed_addr constant [15 x i8] c", expected cap\00", align 1
@.str.35 = private unnamed_addr constant [19 x i8] c"test_boolean_logic\00", align 1
@.str.59 = private unnamed_addr constant [41 x i8] c"- Basic assertions (eq, ne, true, false)\00", align 1
@.str.57 = private unnamed_addr constant [25 x i8] c"=== TESTING COMPLETE ===\00", align 1
@.str.55 = private unnamed_addr constant [41 x i8] c"========================================\00", align 1
@.str.16 = private unnamed_addr constant [17 x i8] c", expected based\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"assert_eq failed: got \00", align 1
@.str.17 = private unnamed_addr constant [14 x i8] c"assert_false(\00", align 1
@.str.63 = private unnamed_addr constant [22 x i8] c"- Pass/fail reporting\00", align 1
@.str.56 = private unnamed_addr constant [53 x i8] c"Compatible with interpretation and compilation modes\00", align 1
@.str.50 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.26 = private unnamed_addr constant [13 x i8] c"assert_less(\00", align 1
@.str.11 = private unnamed_addr constant [31 x i8] c"assert_eq_string failed: got \"\00", align 1
@.str.32 = private unnamed_addr constant [6 x i8] c"World\00", align 1
@.str.40 = private unnamed_addr constant [20 x i8] c"test_function_calls\00", align 1
@.str.47 = private unnamed_addr constant [8 x i8] c"Hello, \00", align 1
@.str.44 = private unnamed_addr constant [19 x i8] c"test_failing_cases\00", align 1
@.str.29 = private unnamed_addr constant [16 x i8] c"test_basic_math\00", align 1
@.str.13 = private unnamed_addr constant [2 x i8] c"\"\00", align 1
@.str.42 = private unnamed_addr constant [15 x i8] c"Hello, CURSED!\00", align 1
@.str.58 = private unnamed_addr constant [37 x i8] c"Framework successfully demonstrates:\00", align 1
@.str.54 = private unnamed_addr constant [41 x i8] c"CURSED Testing Framework - Final Version\00", align 1
@.str.25 = private unnamed_addr constant [5 x i8] c" <= \00", align 1
@.str.28 = private unnamed_addr constant [5 x i8] c" >= \00", align 1
@.str.34 = private unnamed_addr constant [12 x i8] c"Hello World\00", align 1
@.str.64 = private unnamed_addr constant [34 x i8] c"Ready for use in CURSED projects!\00", align 1
@.str.62 = private unnamed_addr constant [19 x i8] c"- Function testing\00", align 1
@.str.0 = private unnamed_addr constant [13 x i8] c"  ✓ PASS: \00", align 1
@.str.61 = private unnamed_addr constant [22 x i8] c"- Numeric comparisons\00", align 1
@.str.23 = private unnamed_addr constant [16 x i8] c"assert_greater(\00", align 1
@.str.49 = private unnamed_addr constant [34 x i8] c"=== RUNNING ALL PASSING TESTS ===\00", align 1
@.str.12 = private unnamed_addr constant [14 x i8] c"\", expected \"\00", align 1
@.str.36 = private unnamed_addr constant [17 x i8] c"test_comparisons\00", align 1
@.str.51 = private unnamed_addr constant [34 x i8] c"=== TESTING FAILURE REPORTING ===\00", align 1
@.str.22 = private unnamed_addr constant [16 x i8] c", expected not \00", align 1
@.str.5 = private unnamed_addr constant [2 x i8] c")\00", align 1
@.str.1 = private unnamed_addr constant [13 x i8] c"  ✗ FAIL: \00", align 1
@.str.53 = private unnamed_addr constant [16 x i8] c"demo_assertions\00", align 1
@.str.60 = private unnamed_addr constant [17 x i8] c"- String testing\00", align 1
@.str.38 = private unnamed_addr constant [5 x i8] c"test\00", align 1
@.str.24 = private unnamed_addr constant [24 x i8] c"assert_greater failed: \00", align 1
@.str.14 = private unnamed_addr constant [13 x i8] c"assert_true(\00", align 1
@.str.2 = private unnamed_addr constant [15 x i8] c"Running test: \00", align 1
@.str.27 = private unnamed_addr constant [21 x i8] c"assert_less failed: \00", align 1
@.str.3 = private unnamed_addr constant [11 x i8] c"assert_eq(\00", align 1
@.str.8 = private unnamed_addr constant [19 x i8] c"assert_eq_string(\"\00", align 1
@.str.52 = private unnamed_addr constant [23 x i8] c"=== ASSERTION DEMO ===\00", align 1
@.str.18 = private unnamed_addr constant [26 x i8] c"assert_false failed: got \00", align 1
@.str.20 = private unnamed_addr constant [11 x i8] c"assert_ne(\00", align 1
@.str.37 = private unnamed_addr constant [15 x i8] c"test_variables\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.54, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.55, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = getelementptr inbounds [53 x i8], [53 x i8]* @.str.56, i64 0, i64 0
  %7 = call i32 @puts(i8* %6)
  %8 = add i32 0, 0
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.50, i64 0, i64 0
  %10 = call i32 @puts(i8* %9)
  %11 = add i32 0, 0
  ; Expression result: %11
  %12 = call i32 @demo_assertions()
  ; Expression result: %12
  %13 = call i32 @run_all_passing_tests()
  ; Expression result: %13
  %14 = call i32 @run_failing_tests()
  ; Expression result: %14
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.50, i64 0, i64 0
  %16 = call i32 @puts(i8* %15)
  %17 = add i32 0, 0
  ; Expression result: %17
  %18 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.57, i64 0, i64 0
  %19 = call i32 @puts(i8* %18)
  %20 = add i32 0, 0
  ; Expression result: %20
  %21 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.58, i64 0, i64 0
  %22 = call i32 @puts(i8* %21)
  %23 = add i32 0, 0
  ; Expression result: %23
  %24 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.59, i64 0, i64 0
  %25 = call i32 @puts(i8* %24)
  %26 = add i32 0, 0
  ; Expression result: %26
  %27 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.60, i64 0, i64 0
  %28 = call i32 @puts(i8* %27)
  %29 = add i32 0, 0
  ; Expression result: %29
  %30 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.61, i64 0, i64 0
  %31 = call i32 @puts(i8* %30)
  %32 = add i32 0, 0
  ; Expression result: %32
  %33 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.62, i64 0, i64 0
  %34 = call i32 @puts(i8* %33)
  %35 = add i32 0, 0
  ; Expression result: %35
  %36 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.63, i64 0, i64 0
  %37 = call i32 @puts(i8* %36)
  %38 = add i32 0, 0
  ; Expression result: %38
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.50, i64 0, i64 0
  %40 = call i32 @puts(i8* %39)
  %41 = add i32 0, 0
  ; Expression result: %41
  %42 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.64, i64 0, i64 0
  %43 = call i32 @puts(i8* %42)
  %44 = add i32 0, 0
  ; Expression result: %44
  ret i32 0
}

