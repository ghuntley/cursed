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
define i32 @run_all_stdlib_tests() {
entry:
  %0 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.1, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %7 = call i32 @puts(i8* %6)
  %8 = add i32 0, 0
  ; Expression result: %8
  ; Expression result: %total_tests_run
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %9
  ; Expression result: 0
  ; Expression result: %total_tests_passed
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %10
  ; Expression result: 0
  ; Expression result: %total_tests_failed
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %11
  ; Expression result: 0
  %12 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.3, i64 0, i64 0
  %13 = call i32 @puts(i8* %12)
  %14 = add i32 0, 0
  ; Expression result: %14
  %15 = call i32 @run_basic_math_tests()
  ; Expression result: %15
  %16 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.4, i64 0, i64 0
  %17 = call i32 @puts(i8* %16)
  %18 = add i32 0, 0
  ; Expression result: %18
  %19 = call i32 @run_basic_string_tests()
  ; Expression result: %19
  %20 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.5, i64 0, i64 0
  %21 = call i32 @puts(i8* %20)
  %22 = add i32 0, 0
  ; Expression result: %22
  %23 = call i32 @run_basic_boolean_tests()
  ; Expression result: %23
  %24 = getelementptr inbounds [42 x i8], [42 x i8]* @.str.6, i64 0, i64 0
  %25 = call i32 @puts(i8* %24)
  %26 = add i32 0, 0
  ; Expression result: %26
  %27 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.7, i64 0, i64 0
  %28 = call i32 @puts(i8* %27)
  %29 = add i32 0, 0
  ; Expression result: %29
  %30 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.8, i64 0, i64 0
  %31 = call i32 @puts(i8* %30)
  %32 = add i32 0, 0
  ; Expression result: %32
  %33 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %33
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %34
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %35
  %36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %36
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %37
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %38
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %39
  %40 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %40
  %41 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %41
  %42 = icmp eq i32 %total_tests_failed, 0
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %42, label %label0, label %label1
label0:
  %43 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %44 = call i32 @puts(i8* %43)
  %45 = add i32 0, 0
  ; Expression result: %45
  %46 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.9, i64 0, i64 0
  %47 = call i32 @puts(i8* %46)
  %48 = add i32 0, 0
  ; Expression result: %48
  %49 = getelementptr inbounds [49 x i8], [49 x i8]* @.str.10, i64 0, i64 0
  %50 = call i32 @puts(i8* %49)
  %51 = add i32 0, 0
  ; Expression result: %51
  %52 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %53 = call i32 @puts(i8* %52)
  %54 = add i32 0, 0
  ; Expression result: %54
  %55 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.11, i64 0, i64 0
  %56 = call i32 @puts(i8* %55)
  %57 = add i32 0, 0
  ; Expression result: %57
  %58 = getelementptr inbounds [55 x i8], [55 x i8]* @.str.12, i64 0, i64 0
  %59 = call i32 @puts(i8* %58)
  %60 = add i32 0, 0
  ; Expression result: %60
  %61 = getelementptr inbounds [53 x i8], [53 x i8]* @.str.13, i64 0, i64 0
  %62 = call i32 @puts(i8* %61)
  %63 = add i32 0, 0
  ; Expression result: %63
  %64 = getelementptr inbounds [47 x i8], [47 x i8]* @.str.14, i64 0, i64 0
  %65 = call i32 @puts(i8* %64)
  %66 = add i32 0, 0
  ; Expression result: %66
  %67 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %68 = call i32 @puts(i8* %67)
  %69 = add i32 0, 0
  ; Expression result: %69
  br label %label2
label1:
  %70 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %71 = call i32 @puts(i8* %70)
  %72 = add i32 0, 0
  ; Expression result: %72
  %73 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.15, i64 0, i64 0
  %74 = call i32 @puts(i8* %73)
  %75 = add i32 0, 0
  ; Expression result: %75
  %76 = getelementptr inbounds [48 x i8], [48 x i8]* @.str.16, i64 0, i64 0
  %77 = call i32 @puts(i8* %76)
  %78 = add i32 0, 0
  ; Expression result: %78
  %79 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %80 = call i32 @puts(i8* %79)
  %81 = add i32 0, 0
  ; Expression result: %81
  br label %label2
label2:
  %82 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %82
  ; Expression result: %total_tests_failed
  ret i32 0
}

define i8* @test_start(i8* %name) {
entry:
  ; Expression result: %total_tests_run
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %0
  %1 = add i32 %total_tests_run, 1
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %2
  ; Expression result: %name
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %3
  ret i32 0
}

define i8* @test_pass(i8* %message) {
entry:
  ; Expression result: %total_tests_passed
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %0
  %1 = add i32 %total_tests_passed, 1
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %2
  ; Expression result: %message
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %3
  ret i32 0
}

define i8* @test_fail(i8* %message) {
entry:
  ; Expression result: %total_tests_failed
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %0
  %1 = add i32 %total_tests_failed, 1
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %2
  ; Expression result: %message
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %3
  ret i32 0
}

define void @assert_eq_int(i32 %actual, i32 %expected) {
entry:
  %0 = icmp eq i32 %actual, %expected
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %3
  %4 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.17, i64 0, i64 0
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %6 = add i32 %4, %5
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %7
  br label %label2
label1:
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %9
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %10
  %11 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.18, i64 0, i64 0
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %13 = add i32 %11, %12
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %14
  br label %label2
label2:
  ret void
}

define void @assert_true(i1 %value) {
entry:
  %0 = icmp eq i1 %value, 1
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.19, i64 0, i64 0
  %2 = call i32 @test_pass(i32 %1)
  ; Expression result: %2
  br label %label2
label1:
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %3
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %5
  %6 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.20, i64 0, i64 0
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %7
  br label %label2
label2:
  ret void
}

define void @assert_eq_string(i8* %actual, i8* %expected) {
entry:
  %0 = icmp eq i32 %actual, %expected
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %1
  %2 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.21, i64 0, i64 0
  %3 = add i32 %actual, %2
  %4 = add i32 %3, %expected
  %5 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.22, i64 0, i64 0
  %6 = add i32 %4, %5
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %7
  br label %label2
label1:
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.23, i64 0, i64 0
  %10 = add i32 %actual, %9
  %11 = add i32 %10, %expected
  %12 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.22, i64 0, i64 0
  %13 = add i32 %11, %12
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %14
  br label %label2
label2:
  ret void
}

define i8* @run_basic_math_tests() {
entry:
  %0 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.24, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %2
  ; Expression result: 1
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %3
  ; Expression result: 2
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %5
  ; Expression result: 5
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %6
  ; Expression result: 5
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %8
  ; Expression result: 7
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %9
  ; Expression result: 42
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %11
  ; Expression result: 4
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %12
  ; Expression result: 5
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %13
  %14 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.25, i64 0, i64 0
  %15 = call i32 @test_start(i32 %14)
  ; Expression result: %15
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %16
  ; Expression result: 5
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %17
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %18
  ; Expression result: 8
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %20
  ; Expression result: 7
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %21
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %22
  ; Expression result: 9
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %23
  %24 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.26, i64 0, i64 0
  %25 = call i32 @test_start(i32 %24)
  ; Expression result: %25
  %26 = alloca i32, align 4
  store i32 15, i32* %26, align 4
  ; Variable a allocated
  %27 = alloca i32, align 4
  store i32 3, i32* %27, align 4
  ; Variable b allocated
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %28
  %29 = load i32, i32* %27, align 4
  ; Expression result: %29
  %30 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %30
  ; Expression result: 18
  %31 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %31
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %32
  %33 = load i32, i32* %27, align 4
  ; Expression result: %33
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %34
  ; Expression result: 12
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %35
  %36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %36
  %37 = load i32, i32* %27, align 4
  ; Expression result: %37
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %38
  ; Expression result: 45
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %39
  %40 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %40
  %41 = load i32, i32* %27, align 4
  ; Expression result: %41
  %42 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %42
  ; Expression result: 5
  %43 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %43
  ret i32 0
}

define i32 @run_basic_string_tests() {
entry:
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.27, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.28, i64 0, i64 0
  %3 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.28, i64 0, i64 0
  %4 = call i32 @assert_eq_string(i32 %2, i32 %3)
  ; Expression result: %4
  %5 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.29, i64 0, i64 0
  %6 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.29, i64 0, i64 0
  %7 = call i32 @assert_eq_string(i32 %5, i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %10 = call i32 @assert_eq_string(i32 %8, i32 %9)
  ; Expression result: %10
  %11 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.30, i64 0, i64 0
  %12 = call i32 @test_start(i32 %11)
  ; Expression result: %12
  %13 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.31, i64 0, i64 0
  %14 = alloca i8*, align 4
  store i8* %13, i8** %14, align 4
  ; Variable greeting allocated
  %15 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.32, i64 0, i64 0
  %16 = alloca i8*, align 4
  store i8* %15, i8** %16, align 4
  ; Variable name allocated
  %17 = load i8*, i8** %14, align 4
  %18 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.31, i64 0, i64 0
  %19 = call i32 @assert_eq_string(i32 %17, i32 %18)
  ; Expression result: %19
  %20 = load i8*, i8** %16, align 4
  %21 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.32, i64 0, i64 0
  %22 = call i32 @assert_eq_string(i8* %20, i32 %21)
  ; Expression result: %22
  %23 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.33, i64 0, i64 0
  %24 = call i32 @test_start(i32 %23)
  ; Expression result: %24
  %25 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.31, i64 0, i64 0
  %26 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.34, i64 0, i64 0
  %27 = add i32 %25, %26
  %28 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.35, i64 0, i64 0
  %29 = add i32 %27, %28
  %30 = alloca i8*, align 4
  store i8* %29, i8** %30, align 4
  ; Variable combined allocated
  %31 = load i8*, i8** %30, align 4
  %32 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.36, i64 0, i64 0
  %33 = call i32 @assert_eq_string(i32 %31, i32 %32)
  ; Expression result: %33
  ret i32 0
}

define i8* @run_basic_boolean_tests() {
entry:
  %0 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.37, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = alloca i1, align 4
  store i1 1, i1* %2, align 4
  ; Variable true_val allocated
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %4 = alloca i8*, align 4
  store i8* %3, i8** %4, align 4
  ; Variable false_val allocated
  %5 = load i1, i1* %2, align 4
  %6 = call i32 @assert_true(i32 %5)
  ; Expression result: %6
  %7 = load i8*, i8** %4, align 4
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.38, i64 0, i64 0
  %10 = call i32 @test_start(i32 %9)
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %11
  ; Expression result: 1
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %12
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %14
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %15
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %17
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %20
  %21 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.39, i64 0, i64 0
  %22 = call i32 @test_start(i32 %21)
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %23
  ; Expression result: 3
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %24
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %25
  ; Expression result: 7
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %26
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %27
  ; Expression result: 4
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %28
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %29
  ; Expression result: 9
  %30 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %30
  ret i32 0
}



; String constants
@.str.9 = private unnamed_addr constant [35 x i8] c"🎉 ALL STDLIB TESTS PASSED! 🎉\00", align 1
@.str.11 = private unnamed_addr constant [16 x i8] c"Tested modules:\00", align 1
@.str.19 = private unnamed_addr constant [32 x i8] c"assert_true: condition is based\00", align 1
@.str.22 = private unnamed_addr constant [2 x i8] c"\"\00", align 1
@.str.32 = private unnamed_addr constant [7 x i8] c"CURSED\00", align 1
@.str.36 = private unnamed_addr constant [12 x i8] c"Hello World\00", align 1
@.str.6 = private unnamed_addr constant [42 x i8] c"\0A========================================\00", align 1
@.str.27 = private unnamed_addr constant [16 x i8] c"String Equality\00", align 1
@.str.38 = private unnamed_addr constant [14 x i8] c"Boolean Logic\00", align 1
@.str.18 = private unnamed_addr constant [12 x i8] c", expected \00", align 1
@.str.8 = private unnamed_addr constant [41 x i8] c"========================================\00", align 1
@.str.23 = private unnamed_addr constant [14 x i8] c"\", expected \"\00", align 1
@.str.0 = private unnamed_addr constant [40 x i8] c"🚀 CURSED Standard Library Test Suite\00", align 1
@.str.26 = private unnamed_addr constant [16 x i8] c"Mixed Type Math\00", align 1
@.str.34 = private unnamed_addr constant [2 x i8] c" \00", align 1
@.str.12 = private unnamed_addr constant [55 x i8] c"  ✓ Math      - Mathematical functions and constants\00", align 1
@.str.10 = private unnamed_addr constant [49 x i8] c"The CURSED standard library is fully functional!\00", align 1
@.str.14 = private unnamed_addr constant [47 x i8] c"  ✓ Boolean   - Boolean logic and operations\00", align 1
@.str.1 = private unnamed_addr constant [38 x i8] c"=====================================\00", align 1
@.str.17 = private unnamed_addr constant [5 x i8] c" == \00", align 1
@.str.25 = private unnamed_addr constant [17 x i8] c"Math Comparisons\00", align 1
@.str.29 = private unnamed_addr constant [6 x i8] c"world\00", align 1
@.str.7 = private unnamed_addr constant [31 x i8] c"📊 FINAL STDLIB TEST SUMMARY\00", align 1
@.str.24 = private unnamed_addr constant [17 x i8] c"Basic Arithmetic\00", align 1
@.str.33 = private unnamed_addr constant [18 x i8] c"String Operations\00", align 1
@.str.20 = private unnamed_addr constant [17 x i8] c", expected based\00", align 1
@.str.35 = private unnamed_addr constant [6 x i8] c"World\00", align 1
@.str.3 = private unnamed_addr constant [28 x i8] c"Running Basic Math Tests...\00", align 1
@.str.31 = private unnamed_addr constant [6 x i8] c"Hello\00", align 1
@.str.13 = private unnamed_addr constant [53 x i8] c"  ✓ String    - String manipulation and processing\00", align 1
@.str.4 = private unnamed_addr constant [25 x i8] c"\0ARunning String Tests...\00", align 1
@.str.15 = private unnamed_addr constant [29 x i8] c"❌ SOME STDLIB TESTS FAILED\00", align 1
@.str.16 = private unnamed_addr constant [48 x i8] c"Please check the test output above for details.\00", align 1
@.str.21 = private unnamed_addr constant [7 x i8] c"\" == \"\00", align 1
@.str.28 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
@.str.39 = private unnamed_addr constant [20 x i8] c"Boolean Comparisons\00", align 1
@.str.5 = private unnamed_addr constant [26 x i8] c"\0ARunning Boolean Tests...\00", align 1
@.str.37 = private unnamed_addr constant [15 x i8] c"Boolean Values\00", align 1
@.str.30 = private unnamed_addr constant [17 x i8] c"String Variables\00", align 1
@.str.2 = private unnamed_addr constant [1 x i8] c"\00", align 1
define i32 @main() {
  %0 = alloca i32, align 4
  store i32 0, i32* %0, align 4
  ; Variable total_tests_run allocated at %0
  %1 = alloca i32, align 4
  store i32 0, i32* %1, align 4
  ; Variable total_tests_passed allocated at %1
  %2 = alloca i32, align 4
  store i32 0, i32* %2, align 4
  ; Variable total_tests_failed allocated at %2
  %3 = call i32 @run_all_stdlib_tests()
  ret i32 0
}
