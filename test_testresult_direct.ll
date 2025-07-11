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
declare i8* @cursed_create_structured_error()
declare i8* @cursed_set_error_message(i8*, i8*)
declare i8* @cursed_set_error_code(i8*, i32)
declare i8* @cursed_set_error_details(i8*, i8*)
declare i8* @cursed_set_error_field(i8*, i8*, i8*)
declare i8* @cursed_get_error_field(i8*, i8*)
declare i32 @cursed_get_error_code(i8*)
declare i8* @cursed_get_error_message(i8*)
declare i8* @cursed_get_error_details(i8*)
declare void @cursed_enhanced_try_begin(i64)
declare void @cursed_enhanced_try_end(i64)
declare i8* @cursed_get_panic_context(i64)
declare i8* @cursed_extract_panic_value(i8*)
declare i8* @cursed_extract_stack_trace(i8*)
declare void @cursed_clear_panic_context(i64)
declare void @cursed_register_panic_handler(i64, i8*)
declare i8* @cursed_handle_panic(i64, i8*)
declare void @cursed_propagate_error_context(i64, i64)
declare i8* @cursed_get_goroutine_error_context(i64)
declare void @cursed_clear_goroutine_error_context(i64)
declare i8* @cursed_create_enhanced_context(i8*, i64)
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
  ; Expression result: %current_test_name
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  ; Expression result: %name
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %3
  ; Expression result: %name
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %4
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

define void @assert_eq_int(i32 %actual, i32 %expected) {
entry:
  %0 = icmp eq i32 %actual, %expected
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %3
  %4 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.1, i64 0, i64 0
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %6 = add i32 %4, %5
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %7
  br label %label2
label1:
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %9
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %10
  %11 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.2, i64 0, i64 0
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %13 = add i32 %11, %12
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %14
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
  %1 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %1
  %2 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.3, i64 0, i64 0
  %3 = add i32 %actual, %2
  %4 = add i32 %3, %expected
  %5 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.4, i64 0, i64 0
  %6 = add i32 %4, %5
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %7
  br label %label2
label1:
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.5, i64 0, i64 0
  %10 = add i32 %actual, %9
  %11 = add i32 %10, %expected
  %12 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.4, i64 0, i64 0
  %13 = add i32 %11, %12
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
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
  %1 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.6, i64 0, i64 0
  %2 = call i32 @test_pass(i32 %1)
  ; Expression result: %2
  br label %label2
label1:
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %3
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %5
  %6 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.7, i64 0, i64 0
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %7
  br label %label2
label2:
  ret void
}

define void @assert_false(i1 %value) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %1 = icmp eq i32 %value, %0
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %1, label %label0, label %label1
label0:
  %2 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.8, i64 0, i64 0
  %3 = call i32 @test_pass(i32 %2)
  ; Expression result: %3
  br label %label2
label1:
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %5
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %6
  %7 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.9, i64 0, i64 0
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %8
  br label %label2
label2:
  ret void
}

define void @print_test_summary() {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.10, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %9
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %11
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %12
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %14
  %15 = icmp eq i32 %test_failed, 0
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %15, label %label0, label %label1
label0:
  %16 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.11, i64 0, i64 0
  %17 = call i32 @puts(i8* %16)
  %18 = add i32 0, 0
  ; Expression result: %18
  br label %label2
label1:
  %19 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.12, i64 0, i64 0
  %20 = call i32 @puts(i8* %19)
  %21 = add i32 0, 0
  ; Expression result: %21
  br label %label2
label2:
  ret void
}

define i32 @test_testresult_system() {
entry:
  %0 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.13, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  ; Expression result: 2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %3
  ; Expression result: 4
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %4
  %5 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.14, i64 0, i64 0
  %6 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.14, i64 0, i64 0
  %7 = call i32 @assert_eq_string(i32 %5, i32 %6)
  ; Expression result: %7
  %8 = call i32 @assert_true(i32 1)
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %10 = call i32 @assert_false(i32 %9)
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %11
  ; Expression result: 3
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %12
  ; Expression result: 7
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %14
  ; Expression result: 7
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %15
  ; Expression result: 42
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %17
  ; Expression result: 4
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %18
  ; Expression result: 5
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %20
  %21 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.15, i64 0, i64 0
  ; Expression result: %21
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %22
  %23 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.16, i64 0, i64 0
  ; Expression result: %23
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %24
  %25 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.17, i64 0, i64 0
  %26 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.17, i64 0, i64 0
  %27 = call i32 @assert_eq_string(i32 %25, i32 %26)
  ; Expression result: %27
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %28
  ; Expression result: 1
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %29
  %30 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %30
  %31 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %31
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %32
  %33 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.18, i64 0, i64 0
  %34 = call i32 @puts(i8* %33)
  %35 = add i32 0, 0
  ; Expression result: %35
  ret i32 0
}


; String constants
@.str.2 = private unnamed_addr constant [12 x i8] c", expected \00", align 1
@.str.8 = private unnamed_addr constant [27 x i8] c"assert_false: value is cap\00", align 1
@.str.18 = private unnamed_addr constant [37 x i8] c"✓ TestResult system test completed\00", align 1
@.str.19 = private unnamed_addr constant [32 x i8] c"Starting TestResult System Test\00", align 1
@.str.20 = private unnamed_addr constant [32 x i8] c"TestResult System Test Complete\00", align 1
@.str.4 = private unnamed_addr constant [2 x i8] c"\"\00", align 1
@.str.5 = private unnamed_addr constant [14 x i8] c"\", expected \"\00", align 1
@.str.14 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
@.str.12 = private unnamed_addr constant [22 x i8] c"❌ Some tests failed\00", align 1
@.str.15 = private unnamed_addr constant [6 x i8] c"world\00", align 1
@.str.3 = private unnamed_addr constant [7 x i8] c"\" == \"\00", align 1
@.str.9 = private unnamed_addr constant [15 x i8] c", expected cap\00", align 1
@.str.13 = private unnamed_addr constant [23 x i8] c"TestResult System Test\00", align 1
@.str.7 = private unnamed_addr constant [17 x i8] c", expected based\00", align 1
@.str.17 = private unnamed_addr constant [5 x i8] c"test\00", align 1
@.str.1 = private unnamed_addr constant [5 x i8] c" == \00", align 1
@.str.6 = private unnamed_addr constant [28 x i8] c"assert_true: value is based\00", align 1
@.str.0 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.11 = private unnamed_addr constant [28 x i8] c"🎉 ALL TESTS PASSED! 🎉\00", align 1
@.str.16 = private unnamed_addr constant [11 x i8] c"helloworld\00", align 1
@.str.10 = private unnamed_addr constant [21 x i8] c"=== TEST SUMMARY ===\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.19, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %3
  ; Expression result: 45
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %4
  %5 = call i32 @test_testresult_system()
  ; Expression result: %5
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %6
  ; Expression result: 45
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %7
  %8 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.20, i64 0, i64 0
  %9 = call i32 @puts(i8* %8)
  %10 = add i32 0, 0
  ; Expression result: %10
  %11 = call i32 @print_test_summary()
  ; Expression result: %11
  ret i32 0
}

