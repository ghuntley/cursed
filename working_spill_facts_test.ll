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

define i8* @spill_basic(i8* %message) {
entry:
  %0 = call i32 @puts(i8* %message)
  %1 = add i32 0, 0
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  ; Expression result: %message
  ret i32 0
}

define i8* @spill_format(i8* %format, i8* %value) {
entry:
  %0 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.1, i64 0, i64 0
  %1 = add i32 %format, %0
  %2 = add i32 %1, %value
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable formatted allocated
  %4 = load i8*, i8** %3, align 4
  %5 = call i32 @puts(i8* %4)
  %6 = add i32 0, 0
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %7
  %8 = load i8*, i8** %3, align 4
  ; Expression result: %8
  ret i32 0
}

define i8* @spill_color(i8* %color, i8* %message) {
entry:
  %0 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.2, i64 0, i64 0
  %1 = add i32 %0, %color
  %2 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.3, i64 0, i64 0
  %3 = add i32 %1, %2
  %4 = add i32 %3, %message
  %5 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.4, i64 0, i64 0
  %6 = add i32 %4, %5
  %7 = alloca i8*, align 4
  store i8* %6, i8** %7, align 4
  ; Variable colorized allocated
  %8 = load i8*, i8** %7, align 4
  %9 = call i32 @puts(i8* %8)
  %10 = add i32 0, 0
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %11
  %12 = load i8*, i8** %7, align 4
  ; Expression result: %12
  ret i32 0
}

define i8* @spill_genz(i8* %message) {
entry:
  %0 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.5, i64 0, i64 0
  %1 = add i32 %message, %0
  %2 = alloca i8*, align 4
  store i8* %1, i8** %2, align 4
  ; Variable genZ allocated
  %3 = load i8*, i8** %2, align 4
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %6
  %7 = load i8*, i8** %2, align 4
  ; Expression result: %7
  ret i32 0
}



; String constants
@.str.1 = private unnamed_addr constant [2 x i8] c" \00", align 1
@.str.8 = private unnamed_addr constant [14 x i8] c"Hello, world!\00", align 1
@.str.9 = private unnamed_addr constant [9 x i8] c"User: %s\00", align 1
@.str.10 = private unnamed_addr constant [6 x i8] c"Alice\00", align 1
@.str.6 = private unnamed_addr constant [35 x i8] c"Testing spill_facts functionality:\00", align 1
@.str.3 = private unnamed_addr constant [2 x i8] c"}\00", align 1
@.str.7 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"{\00", align 1
@.str.4 = private unnamed_addr constant [9 x i8] c"{/color}\00", align 1
@.str.13 = private unnamed_addr constant [12 x i8] c"This is lit\00", align 1
@.str.11 = private unnamed_addr constant [4 x i8] c"red\00", align 1
@.str.14 = private unnamed_addr constant [21 x i8] c"All tests completed!\00", align 1
@.str.0 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.12 = private unnamed_addr constant [16 x i8] c"Warning message\00", align 1
@.str.5 = private unnamed_addr constant [12 x i8] c" fr fr 🔥\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.6, i64 0, i64 0
  ; Converting complex expression to output
  %1 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.7, i64 0, i64 0
  %2 = call i32 (i8*, ...) @printf(i8* %1, i32 %0)
  %3 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.8, i64 0, i64 0
  %4 = call i32 @spill_basic(i32 %3)
  %5 = alloca i8*, align 4
  store i8* %4, i8** %5, align 4
  ; Variable test1 allocated at %5
  %6 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.9, i64 0, i64 0
  %7 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.10, i64 0, i64 0
  %8 = call i32 @spill_format(i32 %6, i32 %7)
  %9 = alloca i8*, align 4
  store i8* %8, i8** %9, align 4
  ; Variable test2 allocated at %9
  %10 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.11, i64 0, i64 0
  %11 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.12, i64 0, i64 0
  %12 = call i32 @spill_color(i32 %10, i32 %11)
  %13 = alloca i8*, align 4
  store i8* %12, i8** %13, align 4
  ; Variable test3 allocated at %13
  %14 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.13, i64 0, i64 0
  %15 = call i32 @spill_genz(i32 %14)
  %16 = alloca i8*, align 4
  store i8* %15, i8** %16, align 4
  ; Variable test4 allocated at %16
  %17 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.14, i64 0, i64 0
  ; Converting complex expression to output
  %18 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.7, i64 0, i64 0
  %19 = call i32 (i8*, ...) @printf(i8* %18, i32 %17)
  ret i32 0
}
