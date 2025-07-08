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

define i8* @test_start(i8* %name) {
entry:
  ; Expression result: %test_count
  %0 = add i32 0, 0 ; placeholder
  ; Expression result: %0
  %1 = add i32 0, 0 ; placeholder
  %2 = add i32 %test_count, %1
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  ; Expression result: %3
  ; Expression result: %name
  %4 = add i32 0, 0 ; placeholder
  ; Expression result: %4
  ret i32 0
}

define i8* @test_pass(i8* %message) {
entry:
  ; Expression result: %test_passed
  %0 = add i32 0, 0 ; placeholder
  ; Expression result: %0
  %1 = add i32 0, 0 ; placeholder
  %2 = add i32 %test_passed, %1
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  ; Expression result: %3
  ; Expression result: %message
  %4 = add i32 0, 0 ; placeholder
  ; Expression result: %4
  ret i32 0
}

define i8* @test_fail(i8* %message) {
entry:
  ; Expression result: %test_failed
  %0 = add i32 0, 0 ; placeholder
  ; Expression result: %0
  %1 = add i32 0, 0 ; placeholder
  %2 = add i32 %test_failed, %1
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  ; Expression result: %3
  ; Expression result: %message
  %4 = add i32 0, 0 ; placeholder
  ; Expression result: %4
  ret i32 0
}

define void @assert_eq_int(i32 %actual, i32 %expected) {
entry:
  %0 = icmp eq i32 %actual, %expected
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %0, label %label0, label %label1
label0:
  %1 = add i32 0, 0 ; placeholder
  ; Expression result: %1
  %2 = add i32 0, 0 ; placeholder
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  ; Expression result: %3
  %4 = add i32 0, 0 ; placeholder
  %5 = add i32 0, 0 ; placeholder
  %6 = add i32 %4, %5
  ; Expression result: %6
  %7 = add i32 0, 0 ; placeholder
  ; Expression result: %7
  br label %label2
label1:
  %8 = add i32 0, 0 ; placeholder
  ; Expression result: %8
  %9 = add i32 0, 0 ; placeholder
  ; Expression result: %9
  %10 = add i32 0, 0 ; placeholder
  ; Expression result: %10
  %11 = add i32 0, 0 ; placeholder
  %12 = add i32 0, 0 ; placeholder
  %13 = add i32 %11, %12
  ; Expression result: %13
  %14 = add i32 0, 0 ; placeholder
  ; Expression result: %14
  br label %label2
label2:
  ret void
}

define void @assert_true(i1 %value) {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = icmp eq i1 %value, %0
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %1, label %label0, label %label1
label0:
  %2 = add i32 0, 0 ; placeholder
  %3 = call i32 @test_pass(i32 %2)
  ; Expression result: %3
  br label %label2
label1:
  %4 = add i32 0, 0 ; placeholder
  ; Expression result: %4
  %5 = add i32 0, 0 ; placeholder
  ; Expression result: %5
  %6 = add i32 0, 0 ; placeholder
  ; Expression result: %6
  %7 = add i32 0, 0 ; placeholder
  ; Expression result: %7
  %8 = add i32 0, 0 ; placeholder
  ; Expression result: %8
  br label %label2
label2:
  ret void
}

define void @print_test_summary() {
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
  ; Expression result: %6
  %7 = add i32 0, 0 ; placeholder
  ; Expression result: %7
  %8 = add i32 0, 0 ; placeholder
  ; Expression result: %8
  %9 = add i32 0, 0 ; placeholder
  ; Expression result: %9
  %10 = add i32 0, 0 ; placeholder
  ; Expression result: %10
  %11 = add i32 0, 0 ; placeholder
  ; Expression result: %11
  %12 = add i32 0, 0 ; placeholder
  ; Expression result: %12
  %13 = add i32 0, 0 ; placeholder
  ; Expression result: %13
  %14 = add i32 0, 0 ; placeholder
  ; Expression result: %14
  %15 = add i32 0, 0 ; placeholder
  %16 = icmp eq i32 %test_failed, %15
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %16, label %label0, label %label1
label0:
  %17 = add i32 0, 0 ; placeholder
  %18 = call i32 @puts(i8* %17)
  %19 = add i32 0, 0
  ; Expression result: %19
  br label %label2
label1:
  %20 = add i32 0, 0 ; placeholder
  %21 = call i32 @puts(i8* %20)
  %22 = add i32 0, 0
  ; Expression result: %22
  br label %label2
label2:
  ret void
}

define i8* @test_basic_math() {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = add i32 0, 0 ; placeholder
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  ; Expression result: %3
  %4 = add i32 0, 0 ; placeholder
  ; Expression result: %4
  %5 = add i32 0, 0 ; placeholder
  ; Expression result: %5
  %6 = add i32 0, 0 ; placeholder
  ; Expression result: %6
  %7 = add i32 0, 0 ; placeholder
  ; Expression result: %7
  %8 = add i32 0, 0 ; placeholder
  ; Expression result: %8
  %9 = add i32 0, 0 ; placeholder
  ; Expression result: %9
  %10 = add i32 0, 0 ; placeholder
  ; Expression result: %10
  %11 = add i32 0, 0 ; placeholder
  ; Expression result: %11
  %12 = add i32 0, 0 ; placeholder
  ; Expression result: %12
  %13 = add i32 0, 0 ; placeholder
  ; Expression result: %13
  %14 = add i32 0, 0 ; placeholder
  ; Expression result: %14
  %15 = add i32 0, 0 ; placeholder
  ; Expression result: %15
  %16 = add i32 0, 0 ; placeholder
  ; Expression result: %16
  %17 = add i32 0, 0 ; placeholder
  ; Expression result: %17
  %18 = add i32 0, 0 ; placeholder
  ; Expression result: %18
  %19 = add i32 0, 0 ; placeholder
  ; Expression result: %19
  %20 = add i32 0, 0 ; placeholder
  ; Expression result: %20
  %21 = add i32 0, 0 ; placeholder
  ; Expression result: %21
  %22 = add i32 0, 0 ; placeholder
  ; Expression result: %22
  %23 = add i32 0, 0 ; placeholder
  ; Expression result: %23
  %24 = add i32 0, 0 ; placeholder
  ; Expression result: %24
  %25 = add i32 0, 0 ; placeholder
  ; Expression result: %25
  %26 = add i32 0, 0 ; placeholder
  ; Expression result: %26
  %27 = add i32 0, 0 ; placeholder
  ; Expression result: %27
  %28 = add i32 0, 0 ; placeholder
  ; Expression result: %28
  %29 = add i32 0, 0 ; placeholder
  ; Expression result: %29
  %30 = add i32 0, 0 ; placeholder
  ; Expression result: %30
  %31 = add i32 0, 0 ; placeholder
  ; Expression result: %31
  %32 = add i32 0, 0 ; placeholder
  ; Expression result: %32
  %33 = add i32 0, 0 ; placeholder
  ; Expression result: %33
  ret i32 0
}

define i8* @test_math_types() {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = add i32 0, 0 ; placeholder
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable a allocated
  %4 = add i32 0, 0 ; placeholder
  %5 = alloca i32, align 4
  store i32 %4, i32* %5, align 4
  ; Variable b allocated
  %6 = add i32 0, 0 ; placeholder
  ; Expression result: %6
  %7 = load i32, i32* %5, align 4
  ; Expression result: %7
  %8 = add i32 0, 0 ; placeholder
  ; Expression result: %8
  %9 = add i32 0, 0 ; placeholder
  ; Expression result: %9
  %10 = add i32 0, 0 ; placeholder
  ; Expression result: %10
  %11 = add i32 0, 0 ; placeholder
  ; Expression result: %11
  %12 = load i32, i32* %5, align 4
  ; Expression result: %12
  %13 = add i32 0, 0 ; placeholder
  ; Expression result: %13
  %14 = add i32 0, 0 ; placeholder
  ; Expression result: %14
  %15 = add i32 0, 0 ; placeholder
  ; Expression result: %15
  %16 = add i32 0, 0 ; placeholder
  ; Expression result: %16
  %17 = load i32, i32* %5, align 4
  ; Expression result: %17
  %18 = add i32 0, 0 ; placeholder
  ; Expression result: %18
  %19 = add i32 0, 0 ; placeholder
  ; Expression result: %19
  %20 = add i32 0, 0 ; placeholder
  ; Expression result: %20
  %21 = add i32 0, 0 ; placeholder
  %22 = alloca i32, align 4
  store i32 %21, i32* %22, align 4
  ; Variable x allocated
  %23 = add i32 0, 0 ; placeholder
  %24 = alloca i8*, align 4
  store i8* %23, i8** %24, align 4
  ; Variable y allocated
  %25 = add i32 0, 0 ; placeholder
  ; Expression result: %25
  %26 = add i32 0, 0 ; placeholder
  ; Expression result: %26
  %27 = add i32 0, 0 ; placeholder
  ; Expression result: %27
  %28 = add i32 0, 0 ; placeholder
  ; Expression result: %28
  %29 = add i32 0, 0 ; placeholder
  ; Expression result: %29
  %30 = add i32 0, 0 ; placeholder
  ; Expression result: %30
  ret i32 0
}

define i8* @test_boolean_logic() {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = add i32 0, 0 ; placeholder
  %3 = alloca i1, align 4
  store i1 %2, i1* %3, align 4
  ; Variable true_val allocated
  %4 = add i32 0, 0 ; placeholder
  %5 = alloca i8*, align 4
  store i8* %4, i8** %5, align 4
  ; Variable false_val allocated
  %6 = load i1, i1* %3, align 4
  %7 = call i32 @assert_true(i32 %6)
  ; Expression result: %7
  %8 = load i8*, i8** %5, align 4
  ; Expression result: %8
  %9 = add i32 0, 0 ; placeholder
  ; Expression result: %9
  %10 = add i32 0, 0 ; placeholder
  ; Expression result: %10
  %11 = load i1, i1* %3, align 4
  ; Expression result: %11
  %12 = add i32 0, 0 ; placeholder
  ; Expression result: %12
  %13 = add i32 0, 0 ; placeholder
  ; Expression result: %13
  %14 = load i8*, i8** %5, align 4
  ; Expression result: %14
  %15 = add i32 0, 0 ; placeholder
  ; Expression result: %15
  %16 = add i32 0, 0 ; placeholder
  ; Expression result: %16
  %17 = load i8*, i8** %5, align 4
  ; Expression result: %17
  %18 = add i32 0, 0 ; placeholder
  ; Expression result: %18
  %19 = add i32 0, 0 ; placeholder
  ; Expression result: %19
  ret i32 0
}

define void @run_simple_math_tests() {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = call i32 @test_basic_math()
  ; Expression result: %6
  %7 = call i32 @test_math_types()
  ; Expression result: %7
  %8 = call i32 @test_boolean_logic()
  ; Expression result: %8
  %9 = call i32 @print_test_summary()
  ; Expression result: %9
  %10 = add i32 0, 0 ; placeholder
  %11 = icmp sgt i32 %test_failed, %10
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %11, label %label0, label %label1
label0:
  %12 = add i32 0, 0 ; placeholder
  ; Expression result: %12
  %13 = add i32 0, 0 ; placeholder
  ; Expression result: %13
  br label %label2
label1:
  %14 = add i32 0, 0 ; placeholder
  ; Expression result: %14
  %15 = add i32 0, 0 ; placeholder
  ; Expression result: %15
  br label %label2
label2:
  ret void
}


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
  %3 = call i32 @run_simple_math_tests()
  ret i32 0
}
