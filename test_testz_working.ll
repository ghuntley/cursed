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

define void @assert_true(i1 %value) {
entry:
  %0 = icmp eq i1 %value, 1
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.3, i64 0, i64 0
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
  %6 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.4, i64 0, i64 0
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %7
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
  %3 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.5, i64 0, i64 0
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
  %16 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.6, i64 0, i64 0
  %17 = call i32 @puts(i8* %16)
  %18 = add i32 0, 0
  ; Expression result: %18
  br label %label2
label1:
  %19 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.7, i64 0, i64 0
  %20 = call i32 @puts(i8* %19)
  %21 = add i32 0, 0
  ; Expression result: %21
  br label %label2
label2:
  ret void
}



; String constants
@.str.4 = private unnamed_addr constant [17 x i8] c", expected based\00", align 1
@.str.3 = private unnamed_addr constant [28 x i8] c"assert_true: value is based\00", align 1
@.str.0 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.5 = private unnamed_addr constant [21 x i8] c"=== TEST SUMMARY ===\00", align 1
@.str.7 = private unnamed_addr constant [22 x i8] c"❌ Some tests failed\00", align 1
@.str.2 = private unnamed_addr constant [12 x i8] c", expected \00", align 1
@.str.6 = private unnamed_addr constant [28 x i8] c"🎉 ALL TESTS PASSED! 🎉\00", align 1
@.str.1 = private unnamed_addr constant [5 x i8] c" == \00", align 1
@.str.8 = private unnamed_addr constant [12 x i8] c"Simple test\00", align 1
define i32 @main() {
  %0 = alloca i32, align 4
  store i32 0, i32* %0, align 4
  ; Variable test_count allocated at %0
  %1 = alloca i32, align 4
  store i32 0, i32* %1, align 4
  ; Variable test_passed allocated at %1
  %2 = alloca i32, align 4
  store i32 0, i32* %2, align 4
  ; Variable test_failed allocated at %2
  %3 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %4 = call i32 @test_start(i32 %3)
  %5 = call i32 @assert_eq_int(i32 1, i32 1)
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %9 = call i32 @assert_true(i32 1)
  %10 = call i32 @print_test_summary()
  ret i32 0
}
