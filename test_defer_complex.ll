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
declare void @cursed_panic(i8*, i64)
declare i8* @cursed_alloc(i64)
declare void @cursed_free(i8*)
declare i32 @cursed_goroutine_spawn(i8*)
declare i32 @cursed_channel_send(i8*, i64)
declare i32 @cursed_channel_receive(i8*, i64*)
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
declare i8* @cursed_link_error_context(i8*, i8*)
declare i8* @cursed_capture_stack_trace()
declare i64 @cursed_get_current_goroutine_id()
declare i64 @time(i64*)
declare i8* @cursed_propagate_with_context(i8*, i8*)
@error_msg_default = private unnamed_addr constant [15 x i8] c"Error occurred\00"

; Module Declarations from Imports
; mod module declarations
declare void @mod_init()
declare void @mod_cleanup()

define i32 @cleanup_one() {
entry:
  %0 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  ret i32 0
}

define i32 @cleanup_two() {
entry:
  %0 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.1, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  ret i32 0
}

define i32 @test_multiple_defers() {
entry:
  %0 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.2, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  ; Defer statement - expression stored for cleanup
  ; Defer statement - expression stored for cleanup
  %3 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.3, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  ; Defer statement - expression stored for cleanup
  %6 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.4, i64 0, i64 0
  %7 = call i32 @puts(i8* %6)
  %8 = add i32 0, 0
  ; Expression result: %8
  ; Executing deferred expressions in LIFO order
  ; Executing deferred expression
  ; Deferred expression completed
  ; Executing deferred expression
  ; Deferred expression completed
  ; Executing deferred expression
  ; Deferred expression completed
  ret i32 0
}

  %9 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.5, i64 0, i64 0
  %10 = call i32 @puts(i8* %9)
  %11 = add i32 0, 0
  %12 = call i32 @cleanup_two()
  %13 = call i32 @cleanup_one()
define i32 @test_nested_defer() {
entry:
  %0 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.6, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  ; Defer statement - expression stored for cleanup
  %4 = load i32, i32* %3, align 4
  %5 = icmp slt i32 %4, 2
  %9 = add i32 1, 0 ; increment placeholder
  %3 = alloca i32, align 4
  store i32 0, i32* %3, align 4
  ; Short declaration: i := 0
  br label %label0
label0:
  br i1 %5, label %label1, label %label3
label1:
  %6 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.7, i64 0, i64 0
  %7 = call i32 @puts(i8* %6)
  %8 = add i32 0, 0
  ; Expression result: %8
  ; Defer statement - expression stored for cleanup
  br label %label2
label2:
  br label %label0
label3:
  %10 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.8, i64 0, i64 0
  %11 = call i32 @puts(i8* %10)
  %12 = add i32 0, 0
  ; Expression result: %12
  ; Executing deferred expressions in LIFO order
  ; Executing deferred expression
  ; Deferred expression completed
  ; Executing deferred expression
  ; Deferred expression completed
  ret i32 0
}

  %13 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.9, i64 0, i64 0
  %14 = call i32 @puts(i8* %13)
  %15 = add i32 0, 0
  %16 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.10, i64 0, i64 0
  %17 = call i32 @puts(i8* %16)
  %18 = add i32 0, 0


; String constants
@.str.3 = private unnamed_addr constant [12 x i8] c"test middle\00", align 1
@.str.8 = private unnamed_addr constant [19 x i8] c"outer function end\00", align 1
@.str.0 = private unnamed_addr constant [21 x i8] c"cleanup_one executed\00", align 1
@.str.10 = private unnamed_addr constant [12 x i8] c"outer defer\00", align 1
@.str.11 = private unnamed_addr constant [30 x i8] c"defer statement functionality\00", align 1
@.str.2 = private unnamed_addr constant [13 x i8] c"test started\00", align 1
@.str.1 = private unnamed_addr constant [21 x i8] c"cleanup_two executed\00", align 1
@.str.7 = private unnamed_addr constant [15 x i8] c"loop iteration\00", align 1
@.str.9 = private unnamed_addr constant [11 x i8] c"loop defer\00", align 1
@.str.4 = private unnamed_addr constant [12 x i8] c"test ending\00", align 1
@.str.5 = private unnamed_addr constant [13 x i8] c"inline defer\00", align 1
@.str.6 = private unnamed_addr constant [21 x i8] c"outer function start\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.11, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  %2 = call i32 @test_multiple_defers()
  %3 = call i32 @test_nested_defer()
  %4 = call i32 @print_test_summary()
  ret i32 0
}
