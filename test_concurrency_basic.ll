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


; String constants
@.str.0 = private unnamed_addr constant [37 x i8] c"🚀 Starting basic concurrency test\00", align 1
@.str.2 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.3 = private unnamed_addr constant [27 x i8] c"Variable assignment test: \00", align 1
@.str.4 = private unnamed_addr constant [18 x i8] c"Arithmetic test: \00", align 1
@.str.6 = private unnamed_addr constant [14 x i8] c"String test: \00", align 1
@.str.10 = private unnamed_addr constant [30 x i8] c"✅ All basic tests completed\00", align 1
@.str.5 = private unnamed_addr constant [25 x i8] c"Concurrency test passed!\00", align 1
@.str.7 = private unnamed_addr constant [15 x i8] c"Boolean test: \00", align 1
@.str.8 = private unnamed_addr constant [26 x i8] c"Conditional test: SUCCESS\00", align 1
@.str.9 = private unnamed_addr constant [25 x i8] c"Conditional test: FAILED\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.0, i64 0, i64 0
  ; Converting complex expression to output
  %1 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %2 = call i32 (i8*, ...) @printf(i8* %1, i32 %0)
  %3 = alloca i32, align 4
  store i32 null, i32* %3, align 4
  ; Variable result allocated at %3
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %5 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.3, i64 0, i64 0
  ; Converting complex expression to output
  %6 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %7 = call i32 (i8*, ...) @printf(i8* %6, i32 %5)
  %8 = load i32, i32* %3, align 4
  %9 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %10 = call i32 (i8*, ...) @printf(i8* %9, i32 %8)
  %11 = alloca i32, align 4
  store i32 null, i32* %11, align 4
  ; Variable sum allocated at %11
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %13 = add i32 10, 20
  %14 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.4, i64 0, i64 0
  ; Converting complex expression to output
  %15 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %16 = call i32 (i8*, ...) @printf(i8* %15, i32 %14)
  %17 = load i32, i32* %11, align 4
  %18 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %19 = call i32 (i8*, ...) @printf(i8* %18, i32 %17)
  %20 = alloca i32, align 4
  store i32 null, i32* %20, align 4
  ; Variable message allocated at %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %22 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.5, i64 0, i64 0
  %23 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.6, i64 0, i64 0
  ; Converting complex expression to output
  %24 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %25 = call i32 (i8*, ...) @printf(i8* %24, i32 %23)
  %26 = load i32, i32* %20, align 4
  %27 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %28 = call i32 (i8*, ...) @printf(i8* %27, i32 %26)
  %29 = alloca i32, align 4
  store i32 null, i32* %29, align 4
  ; Variable flag allocated at %29
  %30 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %31 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.7, i64 0, i64 0
  ; Converting complex expression to output
  %32 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %33 = call i32 (i8*, ...) @printf(i8* %32, i32 %31)
  %34 = load i1, i1* %29, align 4
  %35 = zext i1 %34 to i32
  %36 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %37 = call i32 (i8*, ...) @printf(i8* %36, i32 %35)
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %39 = load i1, i1* %29, align 4
  %40 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %41 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.8, i64 0, i64 0
  ; Converting complex expression to output
  %42 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %43 = call i32 (i8*, ...) @printf(i8* %42, i32 %41)
  %44 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %46 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %47 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.9, i64 0, i64 0
  ; Converting complex expression to output
  %48 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %49 = call i32 (i8*, ...) @printf(i8* %48, i32 %47)
  %50 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %51 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.10, i64 0, i64 0
  ; Converting complex expression to output
  %52 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %53 = call i32 (i8*, ...) @printf(i8* %52, i32 %51)
  ret i32 0
}
