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
define i8* @test_start(i8* %name) {
entry:
  ; Expression result: %test_count
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = add i32 %test_count, 1
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  ; Expression result: %name
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %3
  ret i32 0
}

define i8* @test_pass(i8* %message) {
entry:
  ; Expression result: %test_passed
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = add i32 %test_passed, 1
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  ; Expression result: %message
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %3
  ret i32 0
}

define i8* @test_fail(i8* %message) {
entry:
  ; Expression result: %test_failed
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = add i32 %test_failed, 1
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  ; Expression result: %message
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %3
  ret i32 0
}

define i8* @assert_eq_string(i8* %actual, i8* %expected) {
entry:
  %0 = icmp eq i32 %actual, %expected
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %1
  %2 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.1, i64 0, i64 0
  %3 = add i32 %actual, %2
  %4 = add i32 %3, %expected
  %5 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.2, i64 0, i64 0
  %6 = add i32 %4, %5
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %7
  br label %label2
label1:
  br label %label2
label2:
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %9
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %10
  %11 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.3, i64 0, i64 0
  %12 = add i32 %actual, %11
  %13 = add i32 %12, %expected
  %14 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.2, i64 0, i64 0
  %15 = add i32 %13, %14
  ; Expression result: %15
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %16
  ret i32 0
}

define i8* @assert_true(i1 %value) {
entry:
  %0 = icmp eq i1 %value, 1
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.4, i64 0, i64 0
  %2 = call i32 @test_pass(i32 %1)
  ; Expression result: %2
  br label %label2
label1:
  br label %label2
label2:
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %3
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %5
  %6 = call i32 @string_from_bool(i32 %value)
  %7 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.5, i64 0, i64 0
  %8 = add i32 %6, %7
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %9
  ret i32 0
}

define i8* @assert_false(i1 %value) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %1 = icmp eq i32 %value, %0
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %1, label %label0, label %label1
label0:
  %2 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.6, i64 0, i64 0
  %3 = call i32 @test_pass(i32 %2)
  ; Expression result: %3
  br label %label2
label1:
  br label %label2
label2:
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %5
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %6
  %7 = call i32 @string_from_bool(i32 %value)
  %8 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.7, i64 0, i64 0
  %9 = add i32 %7, %8
  ; Expression result: %9
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %10
  ret i32 0
}

define i32 @print_test_summary() {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.8, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %6
  %7 = call i32 @string_from_int(i32 %test_count)
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %9
  %10 = call i32 @string_from_int(i32 %test_passed)
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %11
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %12
  %13 = call i32 @string_from_int(i32 %test_failed)
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %14
  %15 = icmp eq i32 %test_failed, 0
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %15, label %label0, label %label1
label0:
  %16 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.9, i64 0, i64 0
  %17 = call i32 @puts(i8* %16)
  %18 = add i32 0, 0
  ; Expression result: %18
  br label %label2
label1:
  br label %label2
label2:
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %20
  %21 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.10, i64 0, i64 0
  %22 = call i32 @puts(i8* %21)
  %23 = add i32 0, 0
  ; Expression result: %23
  ret i32 0
}

define i8* @create_sample_ini() {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.11, i64 0, i64 0
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %3 = add i32 %1, %2
  ; Expression result: %3
  %4 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.12, i64 0, i64 0
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %6 = add i32 %4, %5
  ; Expression result: %6
  %7 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.13, i64 0, i64 0
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %9 = add i32 %7, %8
  ; Expression result: %9
  %10 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.14, i64 0, i64 0
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %12 = add i32 %10, %11
  ; Expression result: %12
  %13 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.15, i64 0, i64 0
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %15 = add i32 %13, %14
  ; Expression result: %15
  %16 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.16, i64 0, i64 0
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %18 = add i32 %16, %17
  ; Expression result: %18
  %19 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.17, i64 0, i64 0
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %21 = add i32 %19, %20
  ; Expression result: %21
  %22 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.18, i64 0, i64 0
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %24 = add i32 %22, %23
  ; Expression result: %24
  %25 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.19, i64 0, i64 0
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %27 = add i32 %25, %26
  ; Expression result: %27
  %28 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.20, i64 0, i64 0
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %30 = add i32 %28, %29
  ; Expression result: %30
  %31 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.14, i64 0, i64 0
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %33 = add i32 %31, %32
  ; Expression result: %33
  %34 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.21, i64 0, i64 0
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %36 = add i32 %34, %35
  ; Expression result: %36
  %37 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.22, i64 0, i64 0
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %39 = add i32 %37, %38
  ; Expression result: %39
  %40 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.23, i64 0, i64 0
  %41 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %42 = add i32 %40, %41
  ; Expression result: %42
  %43 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.24, i64 0, i64 0
  %44 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %45 = add i32 %43, %44
  ; Expression result: %45
  %46 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.25, i64 0, i64 0
  %47 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %48 = add i32 %46, %47
  ; Expression result: %48
  %49 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.14, i64 0, i64 0
  %50 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %51 = add i32 %49, %50
  ; Expression result: %51
  %52 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.26, i64 0, i64 0
  %53 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %54 = add i32 %52, %53
  ; Expression result: %54
  %55 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.27, i64 0, i64 0
  %56 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %57 = add i32 %55, %56
  ; Expression result: %57
  %58 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.28, i64 0, i64 0
  %59 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %60 = add i32 %58, %59
  ; Expression result: %60
  %61 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.29, i64 0, i64 0
  %62 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %63 = add i32 %61, %62
  ; Expression result: %63
  %64 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.30, i64 0, i64 0
  ; Expression result: %64
  %65 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %65
  ret i32 0
}

define i8* @create_sample_env() {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.31, i64 0, i64 0
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %3 = add i32 %1, %2
  ; Expression result: %3
  %4 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.32, i64 0, i64 0
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %6 = add i32 %4, %5
  ; Expression result: %6
  %7 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.33, i64 0, i64 0
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %9 = add i32 %7, %8
  ; Expression result: %9
  %10 = getelementptr inbounds [53 x i8], [53 x i8]* @.str.34, i64 0, i64 0
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %12 = add i32 %10, %11
  ; Expression result: %12
  %13 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.35, i64 0, i64 0
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %15 = add i32 %13, %14
  ; Expression result: %15
  %16 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.36, i64 0, i64 0
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %18 = add i32 %16, %17
  ; Expression result: %18
  %19 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.37, i64 0, i64 0
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %21 = add i32 %19, %20
  ; Expression result: %21
  %22 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.38, i64 0, i64 0
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %24 = add i32 %22, %23
  ; Expression result: %24
  %25 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.14, i64 0, i64 0
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %27 = add i32 %25, %26
  ; Expression result: %27
  %28 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.39, i64 0, i64 0
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %30 = add i32 %28, %29
  ; Expression result: %30
  %31 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.40, i64 0, i64 0
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %33 = add i32 %31, %32
  ; Expression result: %33
  %34 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.41, i64 0, i64 0
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %36 = add i32 %34, %35
  ; Expression result: %36
  %37 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.42, i64 0, i64 0
  ; Expression result: %37
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %38
  ret i32 0
}

define i8* @test_ini_basic_parsing() {
entry:
  %0 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.43, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @create_sample_ini()
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable ini_content allocated
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %5
  %6 = load i32, i32* %4, align 4
  %7 = call i32 @parse_ini(i32 %6)
  %8 = alloca i32, align 4
  store i32 %7, i32* %8, align 4
  ; Variable config allocated
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %9
  %10 = load i32, i32* %8, align 4
  %11 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.44, i64 0, i64 0
  %12 = call i32 @get_value(i32 %10, i32 %11)
  %13 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.45, i64 0, i64 0
  %14 = call i32 @assert_eq_string(i32 %12, i32 %13)
  ; Expression result: %14
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %15
  %16 = load i32, i32* %8, align 4
  %17 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.46, i64 0, i64 0
  %18 = call i32 @get_value(i32 %16, i32 %17)
  %19 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.47, i64 0, i64 0
  %20 = call i32 @assert_eq_string(i32 %18, i32 %19)
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %21
  %22 = load i32, i32* %8, align 4
  %23 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.48, i64 0, i64 0
  %24 = call i32 @get_value(i32 %22, i32 %23)
  %25 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.49, i64 0, i64 0
  %26 = call i32 @assert_eq_string(i32 %24, i32 %25)
  ; Expression result: %26
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %27
  %28 = load i32, i32* %8, align 4
  %29 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.50, i64 0, i64 0
  %30 = call i32 @get_value(i32 %28, i32 %29)
  %31 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.51, i64 0, i64 0
  %32 = call i32 @assert_eq_string(i32 %30, i32 %31)
  ; Expression result: %32
  %33 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %33
  %34 = load i32, i32* %8, align 4
  %35 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.52, i64 0, i64 0
  %36 = call i32 @get_value(i32 %34, i32 %35)
  %37 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.53, i64 0, i64 0
  %38 = call i32 @assert_eq_string(i32 %36, i32 %37)
  ; Expression result: %38
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %39
  %40 = load i32, i32* %8, align 4
  %41 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.54, i64 0, i64 0
  %42 = call i32 @get_value(i32 %40, i32 %41)
  %43 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.55, i64 0, i64 0
  %44 = call i32 @assert_eq_string(i32 %42, i32 %43)
  ; Expression result: %44
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %45
  %46 = load i32, i32* %8, align 4
  %47 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.56, i64 0, i64 0
  %48 = call i32 @get_value(i32 %46, i32 %47)
  %49 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.57, i64 0, i64 0
  %50 = call i32 @assert_eq_string(i32 %48, i32 %49)
  ; Expression result: %50
  %51 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %51
  %52 = load i32, i32* %8, align 4
  %53 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.58, i64 0, i64 0
  %54 = call i32 @get_value(i32 %52, i32 %53)
  %55 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.59, i64 0, i64 0
  %56 = call i32 @assert_eq_string(i32 %54, i32 %55)
  ; Expression result: %56
  %57 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %57
  %58 = load i32, i32* %8, align 4
  %59 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.60, i64 0, i64 0
  %60 = call i32 @get_value(i32 %58, i32 %59)
  %61 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.61, i64 0, i64 0
  %62 = call i32 @assert_eq_string(i32 %60, i32 %61)
  ; Expression result: %62
  %63 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %63
  %64 = load i32, i32* %8, align 4
  %65 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.62, i64 0, i64 0
  %66 = call i32 @get_value(i32 %64, i32 %65)
  %67 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.63, i64 0, i64 0
  %68 = call i32 @assert_eq_string(i32 %66, i32 %67)
  ; Expression result: %68
  %69 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %69
  %70 = load i32, i32* %8, align 4
  %71 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.64, i64 0, i64 0
  %72 = call i32 @get_value(i32 %70, i32 %71)
  %73 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.65, i64 0, i64 0
  %74 = call i32 @assert_eq_string(i32 %72, i32 %73)
  ; Expression result: %74
  %75 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %75
  %76 = load i32, i32* %8, align 4
  %77 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.66, i64 0, i64 0
  %78 = call i32 @get_value(i32 %76, i32 %77)
  %79 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.67, i64 0, i64 0
  %80 = call i32 @assert_eq_string(i32 %78, i32 %79)
  ; Expression result: %80
  %81 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %81
  %82 = load i32, i32* %8, align 4
  %83 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.68, i64 0, i64 0
  %84 = call i32 @get_value(i32 %82, i32 %83)
  %85 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.69, i64 0, i64 0
  %86 = call i32 @assert_eq_string(i32 %84, i32 %85)
  ; Expression result: %86
  %87 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %87
  %88 = load i32, i32* %8, align 4
  %89 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.70, i64 0, i64 0
  %90 = call i32 @get_value(i32 %88, i32 %89)
  %91 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.71, i64 0, i64 0
  %92 = call i32 @assert_eq_string(i32 %90, i32 %91)
  ; Expression result: %92
  %93 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %93
  ret i32 0
}

define i8* @test_ini_edge_cases() {
entry:
  %0 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.72, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  %3 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.73, i64 0, i64 0
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %5 = add i32 %3, %4
  %6 = alloca i8*, align 4
  store i8* %5, i8** %6, align 4
  ; Variable edge_content allocated
  %7 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.14, i64 0, i64 0
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %9 = add i32 %7, %8
  ; Expression result: %9
  %10 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.74, i64 0, i64 0
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %12 = add i32 %10, %11
  ; Expression result: %12
  %13 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.14, i64 0, i64 0
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %15 = add i32 %13, %14
  ; Expression result: %15
  %16 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.75, i64 0, i64 0
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %18 = add i32 %16, %17
  ; Expression result: %18
  %19 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.76, i64 0, i64 0
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %21 = add i32 %19, %20
  ; Expression result: %21
  %22 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.77, i64 0, i64 0
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %24 = add i32 %22, %23
  ; Expression result: %24
  %25 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.78, i64 0, i64 0
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %27 = add i32 %25, %26
  ; Expression result: %27
  %28 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.14, i64 0, i64 0
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %30 = add i32 %28, %29
  ; Expression result: %30
  %31 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.79, i64 0, i64 0
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %33 = add i32 %31, %32
  ; Expression result: %33
  %34 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.80, i64 0, i64 0
  ; Expression result: %34
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %35
  %36 = load i8*, i8** %6, align 4
  %37 = call i32 @parse_ini(i32 %36)
  %38 = alloca i32, align 4
  store i32 %37, i32* %38, align 4
  ; Variable config allocated
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %39
  %40 = load i32, i32* %38, align 4
  %41 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.81, i64 0, i64 0
  %42 = call i32 @get_value(i32 %40, i32 %41)
  %43 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.82, i64 0, i64 0
  %44 = call i32 @assert_eq_string(i32 %42, i32 %43)
  ; Expression result: %44
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %45
  %46 = load i32, i32* %38, align 4
  %47 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.83, i64 0, i64 0
  %48 = call i32 @get_value(i32 %46, i32 %47)
  %49 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.84, i64 0, i64 0
  %50 = call i32 @assert_eq_string(i32 %48, i32 %49)
  ; Expression result: %50
  %51 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %51
  %52 = load i32, i32* %38, align 4
  %53 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.85, i64 0, i64 0
  %54 = call i32 @get_value(i32 %52, i32 %53)
  %55 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.86, i64 0, i64 0
  %56 = call i32 @assert_eq_string(i32 %54, i32 %55)
  ; Expression result: %56
  %57 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %57
  %58 = load i32, i32* %38, align 4
  %59 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.87, i64 0, i64 0
  %60 = call i32 @get_value(i32 %58, i32 %59)
  %61 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.88, i64 0, i64 0
  %62 = call i32 @assert_eq_string(i32 %60, i32 %61)
  ; Expression result: %62
  %63 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %63
  ret i32 0
}

define i8* @test_ini_stringify() {
entry:
  %0 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.89, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @map_create()
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable config allocated
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %5
  %6 = load i32, i32* %4, align 4
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %7
  %8 = load i32, i32* %4, align 4
  %9 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.90, i64 0, i64 0
  %10 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.91, i64 0, i64 0
  %11 = call i32 @set_value(i32 %8, i32 %9, i32 %10)
  ; Expression result: %11
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %12
  %13 = load i32, i32* %4, align 4
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %14
  %15 = load i32, i32* %4, align 4
  %16 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.48, i64 0, i64 0
  %17 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.49, i64 0, i64 0
  %18 = call i32 @set_value(i32 %15, i32 %16, i32 %17)
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %19
  %20 = load i32, i32* %4, align 4
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %21
  %22 = load i32, i32* %4, align 4
  %23 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.50, i64 0, i64 0
  %24 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.51, i64 0, i64 0
  %25 = call i32 @set_value(i32 %22, i32 %23, i32 %24)
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %26
  %27 = load i32, i32* %4, align 4
  ; Expression result: %27
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %28
  %29 = load i32, i32* %4, align 4
  %30 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.58, i64 0, i64 0
  %31 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.59, i64 0, i64 0
  %32 = call i32 @set_value(i32 %29, i32 %30, i32 %31)
  ; Expression result: %32
  %33 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %33
  %34 = load i32, i32* %4, align 4
  ; Expression result: %34
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %35
  %36 = load i32, i32* %4, align 4
  %37 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.60, i64 0, i64 0
  %38 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.61, i64 0, i64 0
  %39 = call i32 @set_value(i32 %36, i32 %37, i32 %38)
  ; Expression result: %39
  %40 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %40
  %41 = load i32, i32* %4, align 4
  %42 = call i32 @stringify_ini(i32 %41)
  %43 = alloca i32, align 4
  store i32 %42, i32* %43, align 4
  ; Variable ini_output allocated
  %44 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %44
  %45 = load i32, i32* %43, align 4
  %46 = call i32 @parse_ini(i32 %45)
  %47 = alloca i32, align 4
  store i32 %46, i32* %47, align 4
  ; Variable parsed_config allocated
  %48 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %48
  %49 = load i32, i32* %47, align 4
  %50 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.90, i64 0, i64 0
  %51 = call i32 @get_value(i32 %49, i32 %50)
  %52 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.91, i64 0, i64 0
  %53 = call i32 @assert_eq_string(i32 %51, i32 %52)
  ; Expression result: %53
  %54 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %54
  %55 = load i32, i32* %47, align 4
  %56 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.48, i64 0, i64 0
  %57 = call i32 @get_value(i32 %55, i32 %56)
  %58 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.49, i64 0, i64 0
  %59 = call i32 @assert_eq_string(i32 %57, i32 %58)
  ; Expression result: %59
  %60 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %60
  %61 = load i32, i32* %47, align 4
  %62 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.50, i64 0, i64 0
  %63 = call i32 @get_value(i32 %61, i32 %62)
  %64 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.51, i64 0, i64 0
  %65 = call i32 @assert_eq_string(i32 %63, i32 %64)
  ; Expression result: %65
  %66 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %66
  %67 = load i32, i32* %47, align 4
  %68 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.58, i64 0, i64 0
  %69 = call i32 @get_value(i32 %67, i32 %68)
  %70 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.59, i64 0, i64 0
  %71 = call i32 @assert_eq_string(i32 %69, i32 %70)
  ; Expression result: %71
  %72 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %72
  %73 = load i32, i32* %47, align 4
  %74 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.60, i64 0, i64 0
  %75 = call i32 @get_value(i32 %73, i32 %74)
  %76 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.61, i64 0, i64 0
  %77 = call i32 @assert_eq_string(i32 %75, i32 %76)
  ; Expression result: %77
  %78 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %78
  ret i32 0
}

define i8* @test_env_parsing() {
entry:
  %0 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.92, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @create_sample_env()
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable env_content allocated
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %5
  %6 = load i32, i32* %4, align 4
  %7 = call i32 @parse_env(i32 %6)
  %8 = alloca i32, align 4
  store i32 %7, i32* %8, align 4
  ; Variable config allocated
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %9
  %10 = load i32, i32* %8, align 4
  %11 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.93, i64 0, i64 0
  %12 = call i32 @get_value(i32 %10, i32 %11)
  %13 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.94, i64 0, i64 0
  %14 = call i32 @assert_eq_string(i32 %12, i32 %13)
  ; Expression result: %14
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %15
  %16 = load i32, i32* %8, align 4
  %17 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.95, i64 0, i64 0
  %18 = call i32 @get_value(i32 %16, i32 %17)
  %19 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.96, i64 0, i64 0
  %20 = call i32 @assert_eq_string(i32 %18, i32 %19)
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %21
  %22 = load i32, i32* %8, align 4
  %23 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.97, i64 0, i64 0
  %24 = call i32 @get_value(i32 %22, i32 %23)
  %25 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.98, i64 0, i64 0
  %26 = call i32 @assert_eq_string(i32 %24, i32 %25)
  ; Expression result: %26
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %27
  %28 = load i32, i32* %8, align 4
  %29 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.99, i64 0, i64 0
  %30 = call i32 @get_value(i32 %28, i32 %29)
  %31 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.100, i64 0, i64 0
  %32 = call i32 @assert_eq_string(i32 %30, i32 %31)
  ; Expression result: %32
  %33 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %33
  %34 = load i32, i32* %8, align 4
  %35 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.101, i64 0, i64 0
  %36 = call i32 @get_value(i32 %34, i32 %35)
  %37 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.102, i64 0, i64 0
  %38 = call i32 @assert_eq_string(i32 %36, i32 %37)
  ; Expression result: %38
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %39
  %40 = load i32, i32* %8, align 4
  %41 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.103, i64 0, i64 0
  %42 = call i32 @get_value(i32 %40, i32 %41)
  %43 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.104, i64 0, i64 0
  %44 = call i32 @assert_eq_string(i32 %42, i32 %43)
  ; Expression result: %44
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %45
  %46 = load i32, i32* %8, align 4
  %47 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.105, i64 0, i64 0
  %48 = call i32 @get_value(i32 %46, i32 %47)
  %49 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.106, i64 0, i64 0
  %50 = call i32 @assert_eq_string(i32 %48, i32 %49)
  ; Expression result: %50
  %51 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %51
  %52 = load i32, i32* %8, align 4
  %53 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.107, i64 0, i64 0
  %54 = call i32 @get_value(i32 %52, i32 %53)
  %55 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.59, i64 0, i64 0
  %56 = call i32 @assert_eq_string(i32 %54, i32 %55)
  ; Expression result: %56
  %57 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %57
  %58 = load i32, i32* %8, align 4
  %59 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.108, i64 0, i64 0
  %60 = call i32 @get_value(i32 %58, i32 %59)
  %61 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.109, i64 0, i64 0
  %62 = call i32 @assert_eq_string(i32 %60, i32 %61)
  ; Expression result: %62
  %63 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %63
  %64 = load i32, i32* %8, align 4
  %65 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.110, i64 0, i64 0
  %66 = call i32 @get_value(i32 %64, i32 %65)
  %67 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.111, i64 0, i64 0
  %68 = call i32 @assert_eq_string(i32 %66, i32 %67)
  ; Expression result: %68
  %69 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %69
  ret i32 0
}

define i8* @test_env_stringify() {
entry:
  %0 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.112, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @map_create()
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable config allocated
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %5
  %6 = load i32, i32* %4, align 4
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %7
  %8 = load i32, i32* %4, align 4
  %9 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.48, i64 0, i64 0
  %10 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.49, i64 0, i64 0
  %11 = call i32 @set_value(i32 %8, i32 %9, i32 %10)
  ; Expression result: %11
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %12
  %13 = load i32, i32* %4, align 4
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %14
  %15 = load i32, i32* %4, align 4
  %16 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.50, i64 0, i64 0
  %17 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.51, i64 0, i64 0
  %18 = call i32 @set_value(i32 %15, i32 %16, i32 %17)
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %19
  %20 = load i32, i32* %4, align 4
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %21
  %22 = load i32, i32* %4, align 4
  %23 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.58, i64 0, i64 0
  %24 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.59, i64 0, i64 0
  %25 = call i32 @set_value(i32 %22, i32 %23, i32 %24)
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %26
  %27 = load i32, i32* %4, align 4
  ; Expression result: %27
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %28
  %29 = load i32, i32* %4, align 4
  %30 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.113, i64 0, i64 0
  %31 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.114, i64 0, i64 0
  %32 = call i32 @set_value(i32 %29, i32 %30, i32 %31)
  ; Expression result: %32
  %33 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %33
  %34 = load i32, i32* %4, align 4
  %35 = call i32 @stringify_env(i32 %34)
  %36 = alloca i32, align 4
  store i32 %35, i32* %36, align 4
  ; Variable env_output allocated
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %37
  %38 = load i32, i32* %36, align 4
  %39 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.115, i64 0, i64 0
  %40 = call i32 @string_contains(i32 %38, i32 %39)
  %41 = call i32 @assert_true(i32 %40)
  ; Expression result: %41
  %42 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %42
  %43 = load i32, i32* %36, align 4
  %44 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.116, i64 0, i64 0
  %45 = call i32 @string_contains(i32 %43, i32 %44)
  %46 = call i32 @assert_true(i32 %45)
  ; Expression result: %46
  %47 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %47
  %48 = load i32, i32* %36, align 4
  %49 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.117, i64 0, i64 0
  %50 = call i32 @string_contains(i32 %48, i32 %49)
  %51 = call i32 @assert_true(i32 %50)
  ; Expression result: %51
  %52 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %52
  %53 = load i32, i32* %36, align 4
  %54 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.118, i64 0, i64 0
  %55 = call i32 @string_contains(i32 %53, i32 %54)
  %56 = call i32 @assert_true(i32 %55)
  ; Expression result: %56
  %57 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %57
  ret i32 0
}

define i8* @test_config_access() {
entry:
  %0 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.119, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @map_create()
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable config allocated
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %5
  %6 = load i32, i32* %4, align 4
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %7
  %8 = load i32, i32* %4, align 4
  %9 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.48, i64 0, i64 0
  %10 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.49, i64 0, i64 0
  %11 = call i32 @set_value(i32 %8, i32 %9, i32 %10)
  ; Expression result: %11
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %12
  %13 = load i32, i32* %4, align 4
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %14
  %15 = load i32, i32* %4, align 4
  %16 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.50, i64 0, i64 0
  %17 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.51, i64 0, i64 0
  %18 = call i32 @set_value(i32 %15, i32 %16, i32 %17)
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %19
  %20 = load i32, i32* %4, align 4
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %21
  %22 = load i32, i32* %4, align 4
  %23 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.120, i64 0, i64 0
  %24 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.47, i64 0, i64 0
  %25 = call i32 @set_value(i32 %22, i32 %23, i32 %24)
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %26
  %27 = load i32, i32* %4, align 4
  ; Expression result: %27
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %28
  %29 = load i32, i32* %4, align 4
  %30 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.113, i64 0, i64 0
  %31 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.121, i64 0, i64 0
  %32 = call i32 @set_value(i32 %29, i32 %30, i32 %31)
  ; Expression result: %32
  %33 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %33
  %34 = load i32, i32* %4, align 4
  %35 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.48, i64 0, i64 0
  %36 = call i32 @get_value(i32 %34, i32 %35)
  %37 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.49, i64 0, i64 0
  %38 = call i32 @assert_eq_string(i32 %36, i32 %37)
  ; Expression result: %38
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %39
  %40 = load i32, i32* %4, align 4
  %41 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.50, i64 0, i64 0
  %42 = call i32 @get_value(i32 %40, i32 %41)
  %43 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.51, i64 0, i64 0
  %44 = call i32 @assert_eq_string(i32 %42, i32 %43)
  ; Expression result: %44
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %45
  %46 = load i32, i32* %4, align 4
  %47 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.122, i64 0, i64 0
  %48 = call i32 @get_value(i32 %46, i32 %47)
  %49 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %50 = call i32 @assert_eq_string(i32 %48, i32 %49)
  ; Expression result: %50
  %51 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %51
  %52 = load i32, i32* %4, align 4
  %53 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.48, i64 0, i64 0
  %54 = call i32 @has_key(i32 %52, i32 %53)
  %55 = call i32 @assert_true(i32 %54)
  ; Expression result: %55
  %56 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %56
  %57 = load i32, i32* %4, align 4
  %58 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.120, i64 0, i64 0
  %59 = call i32 @has_key(i32 %57, i32 %58)
  %60 = call i32 @assert_true(i32 %59)
  ; Expression result: %60
  %61 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %61
  %62 = load i32, i32* %4, align 4
  %63 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.123, i64 0, i64 0
  %64 = call i32 @has_key(i32 %62, i32 %63)
  %65 = call i32 @assert_false(i32 %64)
  ; Expression result: %65
  %66 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %66
  %67 = load i32, i32* %4, align 4
  %68 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.48, i64 0, i64 0
  %69 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.124, i64 0, i64 0
  %70 = call i32 @get_default(i32 %67, i32 %68, i32 %69)
  %71 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.49, i64 0, i64 0
  %72 = call i32 @assert_eq_string(i32 %70, i32 %71)
  ; Expression result: %72
  %73 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %73
  %74 = load i32, i32* %4, align 4
  %75 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.122, i64 0, i64 0
  %76 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.124, i64 0, i64 0
  %77 = call i32 @get_default(i32 %74, i32 %75, i32 %76)
  %78 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.124, i64 0, i64 0
  %79 = call i32 @assert_eq_string(i32 %77, i32 %78)
  ; Expression result: %79
  %80 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %80
  %81 = load i32, i32* %4, align 4
  %82 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.125, i64 0, i64 0
  %83 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.126, i64 0, i64 0
  %84 = call i32 @set_value(i32 %81, i32 %82, i32 %83)
  %85 = alloca i32, align 4
  store i32 %84, i32* %85, align 4
  ; Variable updated_config allocated
  %86 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %86
  %87 = load i32, i32* %85, align 4
  %88 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.125, i64 0, i64 0
  %89 = call i32 @get_value(i32 %87, i32 %88)
  %90 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.126, i64 0, i64 0
  %91 = call i32 @assert_eq_string(i32 %89, i32 %90)
  ; Expression result: %91
  %92 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %92
  ret i32 0
}

define i8* @test_section_access() {
entry:
  %0 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.127, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @map_create()
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable config allocated
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %5
  %6 = load i32, i32* %4, align 4
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %7
  %8 = load i32, i32* %4, align 4
  %9 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.48, i64 0, i64 0
  %10 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.49, i64 0, i64 0
  %11 = call i32 @set_value(i32 %8, i32 %9, i32 %10)
  ; Expression result: %11
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %12
  %13 = load i32, i32* %4, align 4
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %14
  %15 = load i32, i32* %4, align 4
  %16 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.50, i64 0, i64 0
  %17 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.51, i64 0, i64 0
  %18 = call i32 @set_value(i32 %15, i32 %16, i32 %17)
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %19
  %20 = load i32, i32* %4, align 4
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %21
  %22 = load i32, i32* %4, align 4
  %23 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.52, i64 0, i64 0
  %24 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.53, i64 0, i64 0
  %25 = call i32 @set_value(i32 %22, i32 %23, i32 %24)
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %26
  %27 = load i32, i32* %4, align 4
  ; Expression result: %27
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %28
  %29 = load i32, i32* %4, align 4
  %30 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.58, i64 0, i64 0
  %31 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.59, i64 0, i64 0
  %32 = call i32 @set_value(i32 %29, i32 %30, i32 %31)
  ; Expression result: %32
  %33 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %33
  %34 = load i32, i32* %4, align 4
  ; Expression result: %34
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %35
  %36 = load i32, i32* %4, align 4
  %37 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.60, i64 0, i64 0
  %38 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.61, i64 0, i64 0
  %39 = call i32 @set_value(i32 %36, i32 %37, i32 %38)
  ; Expression result: %39
  %40 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %40
  %41 = load i32, i32* %4, align 4
  %42 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.128, i64 0, i64 0
  %43 = call i32 @get_section(i32 %41, i32 %42)
  %44 = alloca i32, align 4
  store i32 %43, i32* %44, align 4
  ; Variable db_section allocated
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %45
  %46 = load i32, i32* %44, align 4
  %47 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.129, i64 0, i64 0
  %48 = call i32 @get_value(i32 %46, i32 %47)
  %49 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.49, i64 0, i64 0
  %50 = call i32 @assert_eq_string(i32 %48, i32 %49)
  ; Expression result: %50
  %51 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %51
  %52 = load i32, i32* %44, align 4
  %53 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.130, i64 0, i64 0
  %54 = call i32 @get_value(i32 %52, i32 %53)
  %55 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.51, i64 0, i64 0
  %56 = call i32 @assert_eq_string(i32 %54, i32 %55)
  ; Expression result: %56
  %57 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %57
  %58 = load i32, i32* %44, align 4
  %59 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.131, i64 0, i64 0
  %60 = call i32 @get_value(i32 %58, i32 %59)
  %61 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.53, i64 0, i64 0
  %62 = call i32 @assert_eq_string(i32 %60, i32 %61)
  ; Expression result: %62
  %63 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %63
  %64 = load i32, i32* %44, align 4
  %65 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.58, i64 0, i64 0
  %66 = call i32 @has_key(i32 %64, i32 %65)
  %67 = call i32 @assert_false(i32 %66)
  ; Expression result: %67
  %68 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %68
  %69 = load i32, i32* %4, align 4
  %70 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.132, i64 0, i64 0
  %71 = call i32 @get_section(i32 %69, i32 %70)
  %72 = alloca i32, align 4
  store i32 %71, i32* %72, align 4
  ; Variable server_section allocated
  %73 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %73
  %74 = load i32, i32* %72, align 4
  %75 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.129, i64 0, i64 0
  %76 = call i32 @get_value(i32 %74, i32 %75)
  %77 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.59, i64 0, i64 0
  %78 = call i32 @assert_eq_string(i32 %76, i32 %77)
  ; Expression result: %78
  %79 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %79
  %80 = load i32, i32* %72, align 4
  %81 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.130, i64 0, i64 0
  %82 = call i32 @get_value(i32 %80, i32 %81)
  %83 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.61, i64 0, i64 0
  %84 = call i32 @assert_eq_string(i32 %82, i32 %83)
  ; Expression result: %84
  %85 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %85
  %86 = load i32, i32* %72, align 4
  %87 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.48, i64 0, i64 0
  %88 = call i32 @has_key(i32 %86, i32 %87)
  %89 = call i32 @assert_false(i32 %88)
  ; Expression result: %89
  %90 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %90
  ret i32 0
}

define i8* @test_config_merging() {
entry:
  %0 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.133, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @map_create()
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable base_config allocated
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %5
  %6 = load i32, i32* %4, align 4
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %7
  %8 = load i32, i32* %4, align 4
  %9 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.48, i64 0, i64 0
  %10 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.49, i64 0, i64 0
  %11 = call i32 @set_value(i32 %8, i32 %9, i32 %10)
  ; Expression result: %11
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %12
  %13 = load i32, i32* %4, align 4
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %14
  %15 = load i32, i32* %4, align 4
  %16 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.50, i64 0, i64 0
  %17 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.51, i64 0, i64 0
  %18 = call i32 @set_value(i32 %15, i32 %16, i32 %17)
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %19
  %20 = load i32, i32* %4, align 4
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %21
  %22 = load i32, i32* %4, align 4
  %23 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.58, i64 0, i64 0
  %24 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.59, i64 0, i64 0
  %25 = call i32 @set_value(i32 %22, i32 %23, i32 %24)
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %26
  %27 = load i32, i32* %4, align 4
  ; Expression result: %27
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %28
  %29 = load i32, i32* %4, align 4
  %30 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.60, i64 0, i64 0
  %31 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.61, i64 0, i64 0
  %32 = call i32 @set_value(i32 %29, i32 %30, i32 %31)
  ; Expression result: %32
  %33 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %33
  %34 = call i32 @map_create()
  %35 = alloca i32, align 4
  store i32 %34, i32* %35, align 4
  ; Variable override_config allocated
  %36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %36
  %37 = load i32, i32* %35, align 4
  ; Expression result: %37
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %38
  %39 = load i32, i32* %35, align 4
  %40 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.48, i64 0, i64 0
  %41 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.134, i64 0, i64 0
  %42 = call i32 @set_value(i32 %39, i32 %40, i32 %41)
  ; Expression result: %42
  %43 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %43
  %44 = load i32, i32* %35, align 4
  ; Expression result: %44
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %45
  %46 = load i32, i32* %35, align 4
  %47 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.60, i64 0, i64 0
  %48 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.135, i64 0, i64 0
  %49 = call i32 @set_value(i32 %46, i32 %47, i32 %48)
  ; Expression result: %49
  %50 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %50
  %51 = load i32, i32* %35, align 4
  ; Expression result: %51
  %52 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %52
  %53 = load i32, i32* %35, align 4
  %54 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.125, i64 0, i64 0
  %55 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.126, i64 0, i64 0
  %56 = call i32 @set_value(i32 %53, i32 %54, i32 %55)
  ; Expression result: %56
  %57 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %57
  %58 = load i32, i32* %4, align 4
  %59 = load i32, i32* %35, align 4
  %60 = call i32 @merge_configs(i32 %58, i32 %59)
  %61 = alloca i32, align 4
  store i32 %60, i32* %61, align 4
  ; Variable merged_config allocated
  %62 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %62
  %63 = load i32, i32* %61, align 4
  %64 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.48, i64 0, i64 0
  %65 = call i32 @get_value(i32 %63, i32 %64)
  %66 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.134, i64 0, i64 0
  %67 = call i32 @assert_eq_string(i32 %65, i32 %66)
  ; Expression result: %67
  %68 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %68
  %69 = load i32, i32* %61, align 4
  %70 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.60, i64 0, i64 0
  %71 = call i32 @get_value(i32 %69, i32 %70)
  %72 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.135, i64 0, i64 0
  %73 = call i32 @assert_eq_string(i32 %71, i32 %72)
  ; Expression result: %73
  %74 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %74
  %75 = load i32, i32* %61, align 4
  %76 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.50, i64 0, i64 0
  %77 = call i32 @get_value(i32 %75, i32 %76)
  %78 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.51, i64 0, i64 0
  %79 = call i32 @assert_eq_string(i32 %77, i32 %78)
  ; Expression result: %79
  %80 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %80
  %81 = load i32, i32* %61, align 4
  %82 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.58, i64 0, i64 0
  %83 = call i32 @get_value(i32 %81, i32 %82)
  %84 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.59, i64 0, i64 0
  %85 = call i32 @assert_eq_string(i32 %83, i32 %84)
  ; Expression result: %85
  %86 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %86
  %87 = load i32, i32* %61, align 4
  %88 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.125, i64 0, i64 0
  %89 = call i32 @get_value(i32 %87, i32 %88)
  %90 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.126, i64 0, i64 0
  %91 = call i32 @assert_eq_string(i32 %89, i32 %90)
  ; Expression result: %91
  %92 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %92
  ret i32 0
}

define i8* @test_schema_validation() {
entry:
  %0 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.136, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @map_create()
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable config allocated
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %5
  %6 = load i32, i32* %4, align 4
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %7
  %8 = load i32, i32* %4, align 4
  %9 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.48, i64 0, i64 0
  %10 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.49, i64 0, i64 0
  %11 = call i32 @set_value(i32 %8, i32 %9, i32 %10)
  ; Expression result: %11
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %12
  %13 = load i32, i32* %4, align 4
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %14
  %15 = load i32, i32* %4, align 4
  %16 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.50, i64 0, i64 0
  %17 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.51, i64 0, i64 0
  %18 = call i32 @set_value(i32 %15, i32 %16, i32 %17)
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %19
  %20 = load i32, i32* %4, align 4
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %21
  %22 = load i32, i32* %4, align 4
  %23 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.58, i64 0, i64 0
  %24 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.59, i64 0, i64 0
  %25 = call i32 @set_value(i32 %22, i32 %23, i32 %24)
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %26
  %27 = call i32 @map_create()
  %28 = alloca i32, align 4
  store i32 %27, i32* %28, align 4
  ; Variable schema allocated
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %29
  %30 = alloca [0x i32], align 4
  %31 = alloca [0 x i32]*, align 4
  store [0 x i32]* %30, [0 x i32]** %31, align 4
  ; Variable required_keys allocated
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %32
  %33 = load i32, i32* %28, align 4
  ; Expression result: %33
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %34
  %35 = load i32, i32* %28, align 4
  %36 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.137, i64 0, i64 0
  %37 = load [0 x i32]*, [0 x i32]** %31, align 4
  %38 = call i32 @map_set(i32 %35, i32 %36, i32 %37)
  ; Expression result: %38
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %39
  %40 = load i32, i32* %4, align 4
  %41 = load i32, i32* %28, align 4
  %42 = call i32 @validate_schema(i32 %40, i32 %41)
  %43 = call i32 @assert_true(i32 %42)
  ; Expression result: %43
  %44 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %44
  %45 = call i32 @map_create()
  %46 = alloca i32, align 4
  store i32 %45, i32* %46, align 4
  ; Variable invalid_config allocated
  %47 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %47
  %48 = load i32, i32* %46, align 4
  ; Expression result: %48
  %49 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %49
  %50 = load i32, i32* %46, align 4
  %51 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.48, i64 0, i64 0
  %52 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.49, i64 0, i64 0
  %53 = call i32 @set_value(i32 %50, i32 %51, i32 %52)
  ; Expression result: %53
  %54 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %54
  %55 = load i32, i32* %46, align 4
  %56 = load i32, i32* %28, align 4
  %57 = call i32 @validate_schema(i32 %55, i32 %56)
  %58 = call i32 @assert_false(i32 %57)
  ; Expression result: %58
  %59 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %59
  ret i32 0
}

define i8* @test_type_conversion() {
entry:
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.138, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @map_create()
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable config allocated
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %5
  %6 = load i32, i32* %4, align 4
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %7
  %8 = load i32, i32* %4, align 4
  %9 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.46, i64 0, i64 0
  %10 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.47, i64 0, i64 0
  %11 = call i32 @set_value(i32 %8, i32 %9, i32 %10)
  ; Expression result: %11
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %12
  %13 = load i32, i32* %4, align 4
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %14
  %15 = load i32, i32* %4, align 4
  %16 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.139, i64 0, i64 0
  %17 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.96, i64 0, i64 0
  %18 = call i32 @set_value(i32 %15, i32 %16, i32 %17)
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %19
  %20 = load i32, i32* %4, align 4
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %21
  %22 = load i32, i32* %4, align 4
  %23 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.130, i64 0, i64 0
  %24 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.61, i64 0, i64 0
  %25 = call i32 @set_value(i32 %22, i32 %23, i32 %24)
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %26
  %27 = load i32, i32* %4, align 4
  ; Expression result: %27
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %28
  %29 = load i32, i32* %4, align 4
  %30 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.140, i64 0, i64 0
  %31 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.141, i64 0, i64 0
  %32 = call i32 @set_value(i32 %29, i32 %30, i32 %31)
  ; Expression result: %32
  %33 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %33
  %34 = load i32, i32* %4, align 4
  ; Expression result: %34
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %35
  %36 = load i32, i32* %4, align 4
  %37 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.131, i64 0, i64 0
  %38 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.142, i64 0, i64 0
  %39 = call i32 @set_value(i32 %36, i32 %37, i32 %38)
  ; Expression result: %39
  %40 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %40
  %41 = load i32, i32* %4, align 4
  %42 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.46, i64 0, i64 0
  %43 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %44 = call i32 @get_bool_value(i32 %41, i32 %42, i32 %43)
  %45 = call i32 @assert_true(i32 %44)
  ; Expression result: %45
  %46 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %46
  %47 = load i32, i32* %4, align 4
  %48 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.139, i64 0, i64 0
  %49 = call i32 @get_bool_value(i32 %47, i32 %48, i32 1)
  %50 = call i32 @assert_false(i32 %49)
  ; Expression result: %50
  %51 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %51
  %52 = load i32, i32* %4, align 4
  %53 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.122, i64 0, i64 0
  %54 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %55 = call i32 @get_bool_value(i32 %52, i32 %53, i32 %54)
  %56 = call i32 @assert_false(i32 %55)
  ; Expression result: %56
  %57 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %57
  %58 = load i32, i32* %4, align 4
  %59 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.130, i64 0, i64 0
  %60 = call i32 @get_int_value(i32 %58, i32 %59, i32 0)
  %61 = alloca i32, align 4
  store i32 %60, i32* %61, align 4
  ; Variable port_val allocated
  %62 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %62
  %63 = load i32, i32* %61, align 4
  %64 = call i32 @string_from_int(i32 %63)
  %65 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.61, i64 0, i64 0
  %66 = call i32 @assert_eq_string(i32 %64, i32 %65)
  ; Expression result: %66
  %67 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %67
  %68 = load i32, i32* %4, align 4
  %69 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.122, i64 0, i64 0
  %70 = call i32 @get_int_value(i32 %68, i32 %69, i32 9999)
  %71 = alloca i32, align 4
  store i32 %70, i32* %71, align 4
  ; Variable default_val allocated
  %72 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %72
  %73 = load i32, i32* %71, align 4
  %74 = call i32 @string_from_int(i32 %73)
  %75 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.143, i64 0, i64 0
  %76 = call i32 @assert_eq_string(i32 %74, i32 %75)
  ; Expression result: %76
  %77 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %77
  ret i32 0
}

define i8* @test_empty_config() {
entry:
  %0 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.144, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %4 = alloca i8*, align 4
  store i8* %3, i8** %4, align 4
  ; Variable empty_ini allocated
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %5
  %6 = load i8*, i8** %4, align 4
  %7 = call i32 @parse_ini(i32 %6)
  %8 = alloca i32, align 4
  store i32 %7, i32* %8, align 4
  ; Variable config allocated
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %9
  %10 = load i32, i32* %8, align 4
  %11 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.145, i64 0, i64 0
  %12 = call i32 @has_key(i32 %10, i32 %11)
  %13 = call i32 @assert_false(i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %14
  %15 = load i32, i32* %8, align 4
  %16 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.145, i64 0, i64 0
  %17 = call i32 @get_value(i32 %15, i32 %16)
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %19 = call i32 @assert_eq_string(i32 %17, i32 %18)
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %20
  %21 = load i32, i32* %8, align 4
  %22 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.145, i64 0, i64 0
  %23 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.124, i64 0, i64 0
  %24 = call i32 @get_default(i32 %21, i32 %22, i32 %23)
  %25 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.124, i64 0, i64 0
  %26 = call i32 @assert_eq_string(i32 %24, i32 %25)
  ; Expression result: %26
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %27
  ret i32 0
}

define i8* @test_malformed_config() {
entry:
  %0 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.146, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  %3 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.147, i64 0, i64 0
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %5 = add i32 %3, %4
  %6 = alloca i8*, align 4
  store i8* %5, i8** %6, align 4
  ; Variable malformed_ini allocated
  %7 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.148, i64 0, i64 0
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %9 = add i32 %7, %8
  ; Expression result: %9
  %10 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.149, i64 0, i64 0
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %12 = add i32 %10, %11
  ; Expression result: %12
  %13 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.150, i64 0, i64 0
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %15 = add i32 %13, %14
  ; Expression result: %15
  %16 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.151, i64 0, i64 0
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %17
  %18 = load i8*, i8** %6, align 4
  %19 = call i32 @parse_ini(i32 %18)
  %20 = alloca i32, align 4
  store i32 %19, i32* %20, align 4
  ; Variable config allocated
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %21
  %22 = load i32, i32* %20, align 4
  %23 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.152, i64 0, i64 0
  %24 = call i32 @get_value(i32 %22, i32 %23)
  %25 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.153, i64 0, i64 0
  %26 = call i32 @assert_eq_string(i32 %24, i32 %25)
  ; Expression result: %26
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %27
  ret i32 0
}


; String constants
@.str.11 = private unnamed_addr constant [28 x i8] c"# Sample INI configuration\0A\00", align 1
@.str.19 = private unnamed_addr constant [16 x i8] c"username=admin\0A\00", align 1
@.str.75 = private unnamed_addr constant [23 x i8] c"[section_with_spaces]\0A\00", align 1
@.str.13 = private unnamed_addr constant [12 x i8] c"debug=true\0A\00", align 1
@.str.39 = private unnamed_addr constant [28 x i8] c"# Web server configuration\0A\00", align 1
@.str.105 = private unnamed_addr constant [8 x i8] c"TIMEOUT\00", align 1
@.str.72 = private unnamed_addr constant [15 x i8] c"INI Edge Cases\00", align 1
@.str.38 = private unnamed_addr constant [12 x i8] c"TIMEOUT=60\0A\00", align 1
@.str.34 = private unnamed_addr constant [53 x i8] c"export DATABASE_URL=postgres://localhost:5432/myapp\0A\00", align 1
@.str.4 = private unnamed_addr constant [28 x i8] c"assert_true: value is based\00", align 1
@.str.127 = private unnamed_addr constant [15 x i8] c"Section Access\00", align 1
@.str.18 = private unnamed_addr constant [12 x i8] c"name=myapp\0A\00", align 1
@.str.58 = private unnamed_addr constant [12 x i8] c"server.host\00", align 1
@.str.80 = private unnamed_addr constant [19 x i8] c"global_after=test\0A\00", align 1
@.str.130 = private unnamed_addr constant [5 x i8] c"port\00", align 1
@.str.17 = private unnamed_addr constant [11 x i8] c"port=5432\0A\00", align 1
@.str.146 = private unnamed_addr constant [24 x i8] c"Malformed Configuration\00", align 1
@.str.151 = private unnamed_addr constant [23 x i8] c"valid_key=valid_value\0A\00", align 1
@.str.51 = private unnamed_addr constant [5 x i8] c"5432\00", align 1
@.str.87 = private unnamed_addr constant [13 x i8] c"global_after\00", align 1
@.str.152 = private unnamed_addr constant [24 x i8] c"valid_section.valid_key\00", align 1
@.str.49 = private unnamed_addr constant [10 x i8] c"localhost\00", align 1
@.str.35 = private unnamed_addr constant [31 x i8] c"API_KEY=\"sk-1234567890abcdef\"\0A\00", align 1
@.str.42 = private unnamed_addr constant [19 x i8] c"WEB_CORS_ORIGIN=*\0A\00", align 1
@.str.86 = private unnamed_addr constant [7 x i8] c"simple\00", align 1
@.str.128 = private unnamed_addr constant [9 x i8] c"database\00", align 1
@.str.59 = private unnamed_addr constant [8 x i8] c"0.0.0.0\00", align 1
@.str.7 = private unnamed_addr constant [15 x i8] c", expected cap\00", align 1
@.str.145 = private unnamed_addr constant [8 x i8] c"any_key\00", align 1
@.str.20 = private unnamed_addr constant [22 x i8] c"password=\"secret123\"\0A\00", align 1
@.str.45 = private unnamed_addr constant [13 x i8] c"global_value\00", align 1
@.str.79 = private unnamed_addr constant [24 x i8] c"# Global after section\0A\00", align 1
@.str.21 = private unnamed_addr constant [10 x i8] c"[server]\0A\00", align 1
@.str.121 = private unnamed_addr constant [8 x i8] c"TestApp\00", align 1
@.str.141 = private unnamed_addr constant [5 x i8] c"30.5\00", align 1
@.str.93 = private unnamed_addr constant [9 x i8] c"NODE_ENV\00", align 1
@.str.12 = private unnamed_addr constant [25 x i8] c"global_key=global_value\0A\00", align 1
@.str.15 = private unnamed_addr constant [12 x i8] c"[database]\0A\00", align 1
@.str.43 = private unnamed_addr constant [18 x i8] c"INI Basic Parsing\00", align 1
@.str.94 = private unnamed_addr constant [11 x i8] c"production\00", align 1
@.str.131 = private unnamed_addr constant [5 x i8] c"name\00", align 1
@.str.112 = private unnamed_addr constant [22 x i8] c"Environment Stringify\00", align 1
@.str.76 = private unnamed_addr constant [37 x i8] c"key_with_spaces = value with spaces\0A\00", align 1
@.str.63 = private unnamed_addr constant [2 x i8] c"4\00", align 1
@.str.10 = private unnamed_addr constant [22 x i8] c"❌ Some tests failed\00", align 1
@.str.154 = private unnamed_addr constant [49 x i8] c"Running CURSED Configuration Management Tests...\00", align 1
@.str.83 = private unnamed_addr constant [33 x i8] c"section_with_spaces.quoted_value\00", align 1
@.str.116 = private unnamed_addr constant [19 x i8] c"DATABASE_PORT=5432\00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c", expected based\00", align 1
@.str.70 = private unnamed_addr constant [15 x i8] c"logging.rotate\00", align 1
@.str.103 = private unnamed_addr constant [8 x i8] c"WORKERS\00", align 1
@.str.1 = private unnamed_addr constant [7 x i8] c"\" == \"\00", align 1
@.str.36 = private unnamed_addr constant [36 x i8] c"REDIS_URL='redis://localhost:6379'\0A\00", align 1
@.str.84 = private unnamed_addr constant [14 x i8] c"quoted string\00", align 1
@.str.137 = private unnamed_addr constant [9 x i8] c"required\00", align 1
@.str.140 = private unnamed_addr constant [8 x i8] c"timeout\00", align 1
@.str.123 = private unnamed_addr constant [16 x i8] c"nonexistent.key\00", align 1
@.str.54 = private unnamed_addr constant [18 x i8] c"database.username\00", align 1
@.str.89 = private unnamed_addr constant [14 x i8] c"INI Stringify\00", align 1
@.str.82 = private unnamed_addr constant [18 x i8] c"value with spaces\00", align 1
@.str.106 = private unnamed_addr constant [3 x i8] c"60\00", align 1
@.str.138 = private unnamed_addr constant [16 x i8] c"Type Conversion\00", align 1
@.str.44 = private unnamed_addr constant [11 x i8] c"global_key\00", align 1
@.str.153 = private unnamed_addr constant [12 x i8] c"valid_value\00", align 1
@.str.109 = private unnamed_addr constant [5 x i8] c"3000\00", align 1
@.str.29 = private unnamed_addr constant [23 x i8] c"file=/var/log/app.log\0A\00", align 1
@.str.55 = private unnamed_addr constant [6 x i8] c"admin\00", align 1
@.str.99 = private unnamed_addr constant [8 x i8] c"API_KEY\00", align 1
@.str.33 = private unnamed_addr constant [13 x i8] c"DEBUG=false\0A\00", align 1
@.str.150 = private unnamed_addr constant [17 x i8] c"[valid_section]\0A\00", align 1
@.str.155 = private unnamed_addr constant [41 x i8] c"Configuration Management Tests Complete!\00", align 1
@.str.78 = private unnamed_addr constant [23 x i8] c"unquoted_value=simple\0A\00", align 1
@.str.14 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.114 = private unnamed_addr constant [7 x i8] c"My App\00", align 1
@.str.111 = private unnamed_addr constant [2 x i8] c"*\00", align 1
@.str.115 = private unnamed_addr constant [24 x i8] c"DATABASE_HOST=localhost\00", align 1
@.str.23 = private unnamed_addr constant [11 x i8] c"port=8080\0A\00", align 1
@.str.0 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"\"\00", align 1
@.str.122 = private unnamed_addr constant [12 x i8] c"nonexistent\00", align 1
@.str.139 = private unnamed_addr constant [8 x i8] c"enabled\00", align 1
@.str.67 = private unnamed_addr constant [5 x i8] c"info\00", align 1
@.str.71 = private unnamed_addr constant [6 x i8] c"daily\00", align 1
@.str.27 = private unnamed_addr constant [11 x i8] c"[logging]\0A\00", align 1
@.str.98 = private unnamed_addr constant [32 x i8] c"postgres://localhost:5432/myapp\00", align 1
@.str.100 = private unnamed_addr constant [20 x i8] c"sk-1234567890abcdef\00", align 1
@.str.108 = private unnamed_addr constant [9 x i8] c"WEB_PORT\00", align 1
@.str.6 = private unnamed_addr constant [27 x i8] c"assert_false: value is cap\00", align 1
@.str.41 = private unnamed_addr constant [15 x i8] c"WEB_PORT=3000\0A\00", align 1
@.str.3 = private unnamed_addr constant [14 x i8] c"\", expected \"\00", align 1
@.str.143 = private unnamed_addr constant [5 x i8] c"9999\00", align 1
@.str.60 = private unnamed_addr constant [12 x i8] c"server.port\00", align 1
@.str.129 = private unnamed_addr constant [5 x i8] c"host\00", align 1
@.str.69 = private unnamed_addr constant [17 x i8] c"/var/log/app.log\00", align 1
@.str.102 = private unnamed_addr constant [23 x i8] c"redis://localhost:6379\00", align 1
@.str.104 = private unnamed_addr constant [2 x i8] c"8\00", align 1
@.str.118 = private unnamed_addr constant [18 x i8] c"APP_NAME=\"My App\"\00", align 1
@.str.50 = private unnamed_addr constant [14 x i8] c"database.port\00", align 1
@.str.57 = private unnamed_addr constant [10 x i8] c"secret123\00", align 1
@.str.142 = private unnamed_addr constant [6 x i8] c"MyApp\00", align 1
@.str.135 = private unnamed_addr constant [5 x i8] c"9090\00", align 1
@.str.124 = private unnamed_addr constant [8 x i8] c"default\00", align 1
@.str.107 = private unnamed_addr constant [9 x i8] c"WEB_HOST\00", align 1
@.str.47 = private unnamed_addr constant [5 x i8] c"true\00", align 1
@.str.133 = private unnamed_addr constant [22 x i8] c"Configuration Merging\00", align 1
@.str.24 = private unnamed_addr constant [11 x i8] c"workers=4\0A\00", align 1
@.str.32 = private unnamed_addr constant [21 x i8] c"NODE_ENV=production\0A\00", align 1
@.str.95 = private unnamed_addr constant [6 x i8] c"DEBUG\00", align 1
@.str.28 = private unnamed_addr constant [12 x i8] c"level=info\0A\00", align 1
@.str.77 = private unnamed_addr constant [30 x i8] c"quoted_value=\"quoted string\"\0A\00", align 1
@.str.56 = private unnamed_addr constant [18 x i8] c"database.password\00", align 1
@.str.147 = private unnamed_addr constant [34 x i8] c"[section_without_closing_bracket\0A\00", align 1
@.str.144 = private unnamed_addr constant [20 x i8] c"Empty Configuration\00", align 1
@.str.52 = private unnamed_addr constant [14 x i8] c"database.name\00", align 1
@.str.125 = private unnamed_addr constant [8 x i8] c"new.key\00", align 1
@.str.66 = private unnamed_addr constant [14 x i8] c"logging.level\00", align 1
@.str.96 = private unnamed_addr constant [6 x i8] c"false\00", align 1
@.str.110 = private unnamed_addr constant [16 x i8] c"WEB_CORS_ORIGIN\00", align 1
@.str.40 = private unnamed_addr constant [18 x i8] c"WEB_HOST=0.0.0.0\0A\00", align 1
@.str.16 = private unnamed_addr constant [16 x i8] c"host=localhost\0A\00", align 1
@.str.26 = private unnamed_addr constant [26 x i8] c"; Comment with semicolon\0A\00", align 1
@.str.9 = private unnamed_addr constant [28 x i8] c"🎉 ALL TESTS PASSED! 🎉\00", align 1
@.str.30 = private unnamed_addr constant [14 x i8] c"rotate=daily\0A\00", align 1
@.str.31 = private unnamed_addr constant [25 x i8] c"# Environment variables\0A\00", align 1
@.str.73 = private unnamed_addr constant [21 x i8] c"# Comment only file\0A\00", align 1
@.str.88 = private unnamed_addr constant [5 x i8] c"test\00", align 1
@.str.101 = private unnamed_addr constant [10 x i8] c"REDIS_URL\00", align 1
@.str.136 = private unnamed_addr constant [18 x i8] c"Schema Validation\00", align 1
@.str.81 = private unnamed_addr constant [36 x i8] c"section_with_spaces.key_with_spaces\00", align 1
@.str.8 = private unnamed_addr constant [21 x i8] c"=== TEST SUMMARY ===\00", align 1
@.str.113 = private unnamed_addr constant [9 x i8] c"app.name\00", align 1
@.str.117 = private unnamed_addr constant [20 x i8] c"SERVER_HOST=0.0.0.0\00", align 1
@.str.119 = private unnamed_addr constant [31 x i8] c"Configuration Access Functions\00", align 1
@.str.149 = private unnamed_addr constant [20 x i8] c"=value_without_key\0A\00", align 1
@.str.148 = private unnamed_addr constant [20 x i8] c"key_without_equals\0A\00", align 1
@.str.22 = private unnamed_addr constant [14 x i8] c"host=0.0.0.0\0A\00", align 1
@.str.25 = private unnamed_addr constant [12 x i8] c"timeout=30\0A\00", align 1
@.str.53 = private unnamed_addr constant [6 x i8] c"myapp\00", align 1
@.str.62 = private unnamed_addr constant [15 x i8] c"server.workers\00", align 1
@.str.120 = private unnamed_addr constant [13 x i8] c"server.debug\00", align 1
@.str.85 = private unnamed_addr constant [35 x i8] c"section_with_spaces.unquoted_value\00", align 1
@.str.92 = private unnamed_addr constant [29 x i8] c"Environment Variable Parsing\00", align 1
@.str.91 = private unnamed_addr constant [6 x i8] c"value\00", align 1
@.str.126 = private unnamed_addr constant [10 x i8] c"new_value\00", align 1
@.str.134 = private unnamed_addr constant [15 x i8] c"db.example.com\00", align 1
@.str.97 = private unnamed_addr constant [13 x i8] c"DATABASE_URL\00", align 1
@.str.68 = private unnamed_addr constant [13 x i8] c"logging.file\00", align 1
@.str.132 = private unnamed_addr constant [7 x i8] c"server\00", align 1
@.str.74 = private unnamed_addr constant [17 x i8] c"[empty_section]\0A\00", align 1
@.str.90 = private unnamed_addr constant [7 x i8] c"global\00", align 1
@.str.37 = private unnamed_addr constant [11 x i8] c"WORKERS=8\0A\00", align 1
@.str.48 = private unnamed_addr constant [14 x i8] c"database.host\00", align 1
@.str.64 = private unnamed_addr constant [15 x i8] c"server.timeout\00", align 1
@.str.46 = private unnamed_addr constant [6 x i8] c"debug\00", align 1
@.str.65 = private unnamed_addr constant [3 x i8] c"30\00", align 1
@.str.61 = private unnamed_addr constant [5 x i8] c"8080\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [49 x i8], [49 x i8]* @.str.154, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %3
  %4 = call i32 @test_ini_basic_parsing()
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %5
  %6 = call i32 @test_ini_edge_cases()
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %7
  %8 = call i32 @test_ini_stringify()
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %9
  %10 = call i32 @test_env_parsing()
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %11
  %12 = call i32 @test_env_stringify()
  ; Expression result: %12
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %13
  %14 = call i32 @test_config_access()
  ; Expression result: %14
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %15
  %16 = call i32 @test_section_access()
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %17
  %18 = call i32 @test_config_merging()
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %19
  %20 = call i32 @test_schema_validation()
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %21
  %22 = call i32 @test_type_conversion()
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %23
  %24 = call i32 @test_empty_config()
  ; Expression result: %24
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %25
  %26 = call i32 @test_malformed_config()
  ; Expression result: %26
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %27
  %28 = call i32 @print_test_summary()
  ; Expression result: %28
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %29
  %30 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.155, i64 0, i64 0
  %31 = call i32 @puts(i8* %30)
  %32 = add i32 0, 0
  ; Expression result: %32
  %33 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %33
  ret i32 0
}

