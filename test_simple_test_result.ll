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
define i32 @test_basic_types() {
entry:
  %0 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = alloca i32, align 4
  store i32 0, i32* %3, align 4
  ; Variable pass_status_value allocated
  %4 = alloca i32, align 4
  store i32 1, i32* %4, align 4
  ; Variable fail_status_value allocated
  %5 = alloca i32, align 4
  store i32 2, i32* %5, align 4
  ; Variable skip_status_value allocated
  %6 = alloca i32, align 4
  store i32 3, i32* %6, align 4
  ; Variable error_status_value allocated
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %9
  %10 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.2, i64 0, i64 0
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %12 = add i32 %10, %11
  ; Expression result: %12
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %13
  %14 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %15 = alloca i8*, align 4
  store i8* %14, i8** %15, align 4
  ; Variable test_name allocated
  %16 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.4, i64 0, i64 0
  %17 = alloca i8*, align 4
  store i8* %16, i8** %17, align 4
  ; Variable assertion_name allocated
  %18 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.5, i64 0, i64 0
  %19 = alloca i8*, align 4
  store i8* %18, i8** %19, align 4
  ; Variable message allocated
  %20 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.6, i64 0, i64 0
  %21 = alloca i8*, align 4
  store i8* %20, i8** %21, align 4
  ; Variable expected_value allocated
  %22 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.6, i64 0, i64 0
  %23 = alloca i8*, align 4
  store i8* %22, i8** %23, align 4
  ; Variable actual_value allocated
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %24
  %25 = load i8*, i8** %15, align 4
  %26 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.7, i64 0, i64 0
  %27 = add i32 %25, %26
  %28 = load i8*, i8** %17, align 4
  %29 = add i32 %27, %28
  %30 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.7, i64 0, i64 0
  %31 = add i32 %29, %30
  %32 = load i8*, i8** %19, align 4
  %33 = add i32 %31, %32
  ; Expression result: %33
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %34
  %35 = alloca {i32, i32, i32}, align 4
  %36 = load i32, i32* %3, align 4
  %37 = icmp eq i32 %36, 0
  %38 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %35, i32 0, i32 0
  store i32 %37, i32* %38, align 4
  %39 = alloca {i1}*, align 4
  store {i1}* %35, {i1}** %39, align 4
  ; Variable is_pass_status allocated
  %40 = alloca {i32, i32, i32}, align 4
  %41 = load i32, i32* %4, align 4
  %42 = icmp eq i32 %41, 1
  %43 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %40, i32 0, i32 0
  store i32 %42, i32* %43, align 4
  %44 = alloca {i1}*, align 4
  store {i1}* %40, {i1}** %44, align 4
  ; Variable is_fail_status allocated
  %45 = load {i1}*, {i1}** %39, align 4
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %45, label %label0, label %label1
label0:
  %46 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.8, i64 0, i64 0
  %47 = call i32 @puts(i8* %46)
  %48 = add i32 0, 0
  ; Expression result: %48
  br label %label2
label1:
  br label %label2
label2:
  %49 = load {i1}*, {i1}** %44, align 4
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %49, label %label3, label %label4
label3:
  %50 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.9, i64 0, i64 0
  %51 = call i32 @puts(i8* %50)
  %52 = add i32 0, 0
  ; Expression result: %52
  br label %label5
label4:
  br label %label5
label5:
  %53 = alloca i32, align 4
  store i32 5, i32* %53, align 4
  ; Variable total_tests allocated
  %54 = alloca i32, align 4
  store i32 4, i32* %54, align 4
  ; Variable passed_tests allocated
  %55 = add i32 0, 0 ; literal placeholder
  %56 = alloca i8*, align 4
  store i8* %55, i8** %56, align 4
  ; Variable success_rate allocated
  %57 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %57
  %58 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %58
  ; Expression result: 80
  %59 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %59
  %60 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %60
  %61 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %61
  %62 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %62
  %63 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %63
  %64 = getelementptr inbounds [49 x i8], [49 x i8]* @.str.11, i64 0, i64 0
  %65 = call i32 @puts(i8* %64)
  %66 = add i32 0, 0
  ; Expression result: %66
  ret i32 0
}

define i32 @test_struct_like_behavior() {
entry:
  %0 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.12, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.13, i64 0, i64 0
  %4 = alloca i8*, align 4
  store i8* %3, i8** %4, align 4
  ; Variable test_name allocated
  %5 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.4, i64 0, i64 0
  %6 = alloca i8*, align 4
  store i8* %5, i8** %6, align 4
  ; Variable assertion_name allocated
  %7 = alloca i32, align 4
  store i32 0, i32* %7, align 4
  ; Variable status allocated
  %8 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.14, i64 0, i64 0
  %9 = alloca i8*, align 4
  store i8* %8, i8** %9, align 4
  ; Variable message allocated
  %10 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.6, i64 0, i64 0
  %11 = alloca i8*, align 4
  store i8* %10, i8** %11, align 4
  ; Variable expected allocated
  %12 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.6, i64 0, i64 0
  %13 = alloca i8*, align 4
  store i8* %12, i8** %13, align 4
  ; Variable actual allocated
  %14 = alloca i32, align 4
  store i32 25, i32* %14, align 4
  ; Variable execution_time allocated
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %15
  %16 = load i8*, i8** %4, align 4
  %17 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.15, i64 0, i64 0
  %18 = add i32 %16, %17
  %19 = load i8*, i8** %6, align 4
  %20 = add i32 %18, %19
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %21
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %24
  %25 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.16, i64 0, i64 0
  %26 = load i8*, i8** %9, align 4
  %27 = add i32 %25, %26
  ; Expression result: %27
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %28
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %29
  %30 = load i8*, i8** %11, align 4
  %31 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.17, i64 0, i64 0
  %32 = add i32 %30, %31
  %33 = load i8*, i8** %13, align 4
  %34 = add i32 %32, %33
  ; Expression result: %34
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %35
  %36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %36
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %37
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %38
  %39 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.18, i64 0, i64 0
  ; Expression result: %39
  %40 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %40
  %41 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.19, i64 0, i64 0
  %42 = alloca i8*, align 4
  store i8* %41, i8** %42, align 4
  ; Variable suite_name allocated
  %43 = alloca i32, align 4
  store i32 3, i32* %43, align 4
  ; Variable total_count allocated
  %44 = alloca i32, align 4
  store i32 2, i32* %44, align 4
  ; Variable passed_count allocated
  %45 = alloca i32, align 4
  store i32 1, i32* %45, align 4
  ; Variable failed_count allocated
  %46 = alloca i32, align 4
  store i32 0, i32* %46, align 4
  ; Variable skipped_count allocated
  %47 = alloca i32, align 4
  store i32 0, i32* %47, align 4
  ; Variable error_count allocated
  %48 = add i32 0, 0 ; literal placeholder
  %49 = alloca i8*, align 4
  store i8* %48, i8** %49, align 4
  ; Variable suite_success_rate allocated
  %50 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %50
  %51 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %51
  ; Expression result: 66.67
  %52 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %52
  %53 = load i8*, i8** %42, align 4
  ; Expression result: %53
  %54 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %54
  %55 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %55
  %56 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %56
  %57 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %57
  %58 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.20, i64 0, i64 0
  %59 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %60 = add i32 %58, %59
  ; Expression result: %60
  %61 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %61
  %62 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.21, i64 0, i64 0
  %63 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %64 = add i32 %62, %63
  ; Expression result: %64
  %65 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %65
  %66 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %66
  %67 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %67
  %68 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %68
  %69 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %69
  %70 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %70
  %71 = getelementptr inbounds [43 x i8], [43 x i8]* @.str.22, i64 0, i64 0
  %72 = call i32 @puts(i8* %71)
  %73 = add i32 0, 0
  ; Expression result: %73
  ret i32 0
}

define i32 @test_report_generation() {
entry:
  %0 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.23, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = alloca i32, align 4
  store i32 10, i32* %3, align 4
  ; Variable report_total_tests allocated
  %4 = alloca i32, align 4
  store i32 8, i32* %4, align 4
  ; Variable report_passed_tests allocated
  %5 = alloca i32, align 4
  store i32 2, i32* %5, align 4
  ; Variable report_failed_tests allocated
  %6 = alloca i32, align 4
  store i32 0, i32* %6, align 4
  ; Variable report_skipped_tests allocated
  %7 = alloca i32, align 4
  store i32 0, i32* %7, align 4
  ; Variable report_error_tests allocated
  %8 = add i32 0, 0 ; literal placeholder
  %9 = alloca i8*, align 4
  store i8* %8, i8** %9, align 4
  ; Variable report_success_rate allocated
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  ; Expression result: 80
  %12 = alloca i32, align 4
  store i32 1500, i32* %12, align 4
  ; Variable report_execution_time allocated
  %13 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.24, i64 0, i64 0
  %14 = alloca i8*, align 4
  store i8* %13, i8** %14, align 4
  ; Variable report_output allocated
  %15 = load i8*, i8** %14, align 4
  ; Expression result: %15
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %16
  %17 = load i8*, i8** %14, align 4
  %18 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.25, i64 0, i64 0
  %19 = add i32 %17, %18
  ; Expression result: %19
  %20 = load i8*, i8** %14, align 4
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %21
  %22 = load i8*, i8** %14, align 4
  %23 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.26, i64 0, i64 0
  %24 = add i32 %22, %23
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %26 = add i32 %24, %25
  ; Expression result: %26
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %27
  %28 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.27, i64 0, i64 0
  ; Expression result: %28
  %29 = load i8*, i8** %14, align 4
  ; Expression result: %29
  %30 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %30
  %31 = load i8*, i8** %14, align 4
  %32 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.28, i64 0, i64 0
  %33 = add i32 %31, %32
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %35 = add i32 %33, %34
  ; Expression result: %35
  %36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %36
  %37 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.27, i64 0, i64 0
  ; Expression result: %37
  %38 = load i8*, i8** %14, align 4
  ; Expression result: %38
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %39
  %40 = load i8*, i8** %14, align 4
  %41 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.29, i64 0, i64 0
  %42 = add i32 %40, %41
  %43 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %44 = add i32 %42, %43
  ; Expression result: %44
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %45
  %46 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.27, i64 0, i64 0
  ; Expression result: %46
  %47 = load i8*, i8** %14, align 4
  ; Expression result: %47
  %48 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %48
  %49 = load i8*, i8** %14, align 4
  %50 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.30, i64 0, i64 0
  %51 = add i32 %49, %50
  %52 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %53 = add i32 %51, %52
  ; Expression result: %53
  %54 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %54
  %55 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.31, i64 0, i64 0
  ; Expression result: %55
  %56 = load i8*, i8** %14, align 4
  ; Expression result: %56
  %57 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %57
  %58 = load i8*, i8** %14, align 4
  %59 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.32, i64 0, i64 0
  %60 = add i32 %58, %59
  %61 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %62 = add i32 %60, %61
  ; Expression result: %62
  %63 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %63
  %64 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.33, i64 0, i64 0
  ; Expression result: %64
  %65 = load i32, i32* %5, align 4
  %66 = icmp eq i32 %65, 0
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %66, label %label0, label %label1
label0:
  %67 = load i8*, i8** %14, align 4
  ; Expression result: %67
  %68 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %68
  %69 = load i8*, i8** %14, align 4
  %70 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.34, i64 0, i64 0
  %71 = add i32 %69, %70
  ; Expression result: %71
  br label %label2
label1:
  %72 = load i8*, i8** %14, align 4
  ; Expression result: %72
  %73 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %73
  %74 = load i8*, i8** %14, align 4
  %75 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.35, i64 0, i64 0
  %76 = add i32 %74, %75
  ; Expression result: %76
  br label %label2
label2:
  %77 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.36, i64 0, i64 0
  %78 = call i32 @puts(i8* %77)
  %79 = add i32 0, 0
  ; Expression result: %79
  %80 = load i8*, i8** %14, align 4
  %81 = call i32 @puts(i8* %80)
  %82 = add i32 0, 0
  ; Expression result: %82
  %83 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.37, i64 0, i64 0
  %84 = alloca i8*, align 4
  store i8* %83, i8** %84, align 4
  ; Variable json_report allocated
  %85 = load i8*, i8** %84, align 4
  ; Expression result: %85
  %86 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %86
  %87 = load i8*, i8** %84, align 4
  %88 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.38, i64 0, i64 0
  %89 = add i32 %87, %88
  %90 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %91 = add i32 %89, %90
  ; Expression result: %91
  %92 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %92
  %93 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.39, i64 0, i64 0
  ; Expression result: %93
  %94 = load i8*, i8** %84, align 4
  ; Expression result: %94
  %95 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %95
  %96 = load i8*, i8** %84, align 4
  %97 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.40, i64 0, i64 0
  %98 = add i32 %96, %97
  %99 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %100 = add i32 %98, %99
  ; Expression result: %100
  %101 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %101
  %102 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.39, i64 0, i64 0
  ; Expression result: %102
  %103 = load i8*, i8** %84, align 4
  ; Expression result: %103
  %104 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %104
  %105 = load i8*, i8** %84, align 4
  %106 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.41, i64 0, i64 0
  %107 = add i32 %105, %106
  %108 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %109 = add i32 %107, %108
  ; Expression result: %109
  %110 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %110
  %111 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.39, i64 0, i64 0
  ; Expression result: %111
  %112 = load i8*, i8** %84, align 4
  ; Expression result: %112
  %113 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %113
  %114 = load i8*, i8** %84, align 4
  %115 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.42, i64 0, i64 0
  %116 = add i32 %114, %115
  %117 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %118 = add i32 %116, %117
  ; Expression result: %118
  %119 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %119
  %120 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.39, i64 0, i64 0
  ; Expression result: %120
  %121 = load i8*, i8** %84, align 4
  ; Expression result: %121
  %122 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %122
  %123 = load i8*, i8** %84, align 4
  %124 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.43, i64 0, i64 0
  %125 = add i32 %123, %124
  %126 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %127 = add i32 %125, %126
  ; Expression result: %127
  %128 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %128
  %129 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.27, i64 0, i64 0
  ; Expression result: %129
  %130 = load i8*, i8** %84, align 4
  ; Expression result: %130
  %131 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %131
  %132 = load i8*, i8** %84, align 4
  %133 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.44, i64 0, i64 0
  %134 = add i32 %132, %133
  ; Expression result: %134
  %135 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.45, i64 0, i64 0
  %136 = call i32 @puts(i8* %135)
  %137 = add i32 0, 0
  ; Expression result: %137
  %138 = load i8*, i8** %84, align 4
  %139 = call i32 @puts(i8* %138)
  %140 = add i32 0, 0
  ; Expression result: %140
  %141 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.46, i64 0, i64 0
  %142 = call i32 @puts(i8* %141)
  %143 = add i32 0, 0
  ; Expression result: %143
  ret i32 0
}


; String constants
@.str.7 = private unnamed_addr constant [4 x i8] c" - \00", align 1
@.str.27 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str.20 = private unnamed_addr constant [12 x i8] c" | Passed: \00", align 1
@.str.42 = private unnamed_addr constant [19 x i8] c"  \"success_rate\": \00", align 1
@.str.39 = private unnamed_addr constant [3 x i8] c",\0A\00", align 1
@.str.40 = private unnamed_addr constant [19 x i8] c"  \"passed_tests\": \00", align 1
@.str.43 = private unnamed_addr constant [21 x i8] c"  \"execution_time\": \00", align 1
@.str.50 = private unnamed_addr constant [37 x i8] c"✅ All basic functionality verified\00", align 1
@.str.13 = private unnamed_addr constant [14 x i8] c"test_addition\00", align 1
@.str.1 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.4 = private unnamed_addr constant [10 x i8] c"assert_eq\00", align 1
@.str.6 = private unnamed_addr constant [2 x i8] c"4\00", align 1
@.str.8 = private unnamed_addr constant [28 x i8] c"✓ Pass status check works\00", align 1
@.str.10 = private unnamed_addr constant [2 x i8] c"%\00", align 1
@.str.11 = private unnamed_addr constant [49 x i8] c"✓ Basic TestResult type functionality verified\00", align 1
@.str.15 = private unnamed_addr constant [15 x i8] c" | Assertion: \00", align 1
@.str.17 = private unnamed_addr constant [12 x i8] c" | Actual: \00", align 1
@.str.36 = private unnamed_addr constant [18 x i8] c"Generated Report:\00", align 1
@.str.37 = private unnamed_addr constant [3 x i8] c"{\0A\00", align 1
@.str.44 = private unnamed_addr constant [2 x i8] c"}\00", align 1
@.str.33 = private unnamed_addr constant [4 x i8] c"ms\0A\00", align 1
@.str.31 = private unnamed_addr constant [3 x i8] c"%\0A\00", align 1
@.str.32 = private unnamed_addr constant [17 x i8] c"Execution Time: \00", align 1
@.str.28 = private unnamed_addr constant [9 x i8] c"Passed: \00", align 1
@.str.41 = private unnamed_addr constant [19 x i8] c"  \"failed_tests\": \00", align 1
@.str.47 = private unnamed_addr constant [43 x i8] c"TestResult System Basic Functionality Test\00", align 1
@.str.34 = private unnamed_addr constant [29 x i8] c"🎉 ALL TESTS PASSED! 🎉\0A\00", align 1
@.str.26 = private unnamed_addr constant [14 x i8] c"Total Tests: \00", align 1
@.str.29 = private unnamed_addr constant [9 x i8] c"Failed: \00", align 1
@.str.35 = private unnamed_addr constant [23 x i8] c"❌ Some tests failed\0A\00", align 1
@.str.3 = private unnamed_addr constant [10 x i8] c"test_math\00", align 1
@.str.46 = private unnamed_addr constant [40 x i8] c"✓ Report generation working correctly\00", align 1
@.str.24 = private unnamed_addr constant [20 x i8] c"CURSED Test Report\0A\00", align 1
@.str.0 = private unnamed_addr constant [34 x i8] c"Testing basic TestResult types...\00", align 1
@.str.45 = private unnamed_addr constant [13 x i8] c"JSON Report:\00", align 1
@.str.16 = private unnamed_addr constant [13 x i8] c" | Message: \00", align 1
@.str.2 = private unnamed_addr constant [8 x i8] c", Fail=\00", align 1
@.str.48 = private unnamed_addr constant [53 x i8] c"====================================================\00", align 1
@.str.18 = private unnamed_addr constant [3 x i8] c"ms\00", align 1
@.str.25 = private unnamed_addr constant [20 x i8] c"==================\0A\00", align 1
@.str.30 = private unnamed_addr constant [15 x i8] c"Success Rate: \00", align 1
@.str.21 = private unnamed_addr constant [12 x i8] c" | Failed: \00", align 1
@.str.14 = private unnamed_addr constant [21 x i8] c"Addition test passed\00", align 1
@.str.5 = private unnamed_addr constant [10 x i8] c"2 + 2 = 4\00", align 1
@.str.19 = private unnamed_addr constant [11 x i8] c"math_tests\00", align 1
@.str.23 = private unnamed_addr constant [29 x i8] c"Testing report generation...\00", align 1
@.str.49 = private unnamed_addr constant [38 x i8] c"TestResult System Basic Test Complete\00", align 1
@.str.38 = private unnamed_addr constant [18 x i8] c"  \"total_tests\": \00", align 1
@.str.9 = private unnamed_addr constant [28 x i8] c"✓ Fail status check works\00", align 1
@.str.12 = private unnamed_addr constant [32 x i8] c"Testing struct-like behavior...\00", align 1
@.str.22 = private unnamed_addr constant [43 x i8] c"✓ Struct-like behavior working correctly\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [43 x i8], [43 x i8]* @.str.47, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [53 x i8], [53 x i8]* @.str.48, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = call i32 @test_basic_types()
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %8 = call i32 @puts(i8* %7)
  %9 = add i32 0, 0
  ; Expression result: %9
  %10 = call i32 @test_struct_like_behavior()
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %12 = call i32 @puts(i8* %11)
  %13 = add i32 0, 0
  ; Expression result: %13
  %14 = call i32 @test_report_generation()
  ; Expression result: %14
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %16 = call i32 @puts(i8* %15)
  %17 = add i32 0, 0
  ; Expression result: %17
  %18 = getelementptr inbounds [53 x i8], [53 x i8]* @.str.48, i64 0, i64 0
  %19 = call i32 @puts(i8* %18)
  %20 = add i32 0, 0
  ; Expression result: %20
  %21 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.49, i64 0, i64 0
  %22 = call i32 @puts(i8* %21)
  %23 = add i32 0, 0
  ; Expression result: %23
  %24 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.50, i64 0, i64 0
  %25 = call i32 @puts(i8* %24)
  %26 = add i32 0, 0
  ; Expression result: %26
  ret i32 0
}

