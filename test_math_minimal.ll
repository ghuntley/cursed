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


; String constants
@.str.1 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.4 = private unnamed_addr constant [34 x i8] c"Math test completed successfully!\00", align 1
@.str.3 = private unnamed_addr constant [11 x i8] c"abs(-10): \00", align 1
@.str.2 = private unnamed_addr constant [11 x i8] c"PI value: \00", align 1
@.str.0 = private unnamed_addr constant [23 x i8] c"Testing math module...\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %3
  %4 = add i32 0, 0 ; literal placeholder
  %5 = alloca i8*, align 4
  store i8* %4, i8** %5, align 4
  ; Variable pi_val allocated
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %7
  %8 = call i32 @math_pi()
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %9
  %10 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.2, i64 0, i64 0
  %11 = call i32 @puts(i8* %10)
  %12 = add i32 0, 0
  ; Expression result: %12
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %13
  %14 = load i8*, i8** %5, align 4
  %15 = call i32 @puts(i8* %14)
  %16 = add i32 0, 0
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = add i32 0, 0 ; literal placeholder
  %19 = alloca i8*, align 4
  store i8* %18, i8** %19, align 4
  ; Variable abs_val allocated
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %21
  ; Expression result: 10
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  %24 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.3, i64 0, i64 0
  %25 = call i32 @puts(i8* %24)
  %26 = add i32 0, 0
  ; Expression result: %26
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %27
  %28 = load i8*, i8** %19, align 4
  %29 = call i32 @puts(i8* %28)
  %30 = add i32 0, 0
  ; Expression result: %30
  %31 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %31
  %32 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.4, i64 0, i64 0
  %33 = call i32 @puts(i8* %32)
  %34 = add i32 0, 0
  ; Expression result: %34
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %35
  ret i32 0
}

