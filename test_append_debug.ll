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
@.str.4 = private unnamed_addr constant [23 x i8] c"Original array length:\00", align 1
@.str.5 = private unnamed_addr constant [11 x i8] c"New array:\00", align 1
@.str.0 = private unnamed_addr constant [2 x i8] c"1\00", align 1
@.str.1 = private unnamed_addr constant [2 x i8] c"2\00", align 1
@.str.6 = private unnamed_addr constant [18 x i8] c"New array length:\00", align 1
@.str.3 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.2 = private unnamed_addr constant [16 x i8] c"Original array:\00", align 1
define i32 @main() {
  %0 = alloca [2 x i32], align 4
  %1 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.0, i64 0, i64 0
  %2 = getelementptr inbounds [2 x i32], [2 x i32]* %0, i64 0, i64 0
  store i32 %1, i32* %2, align 4
  %3 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.1, i64 0, i64 0
  %4 = getelementptr inbounds [2 x i32], [2 x i32]* %0, i64 0, i64 1
  store i32 %3, i32* %4, align 4
  %5 = alloca i32, align 4
  store i32 %0, i32* %5, align 4
  ; Variable arr allocated at %5
  %6 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.2, i64 0, i64 0
  ; Converting complex expression to output
  %7 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %8 = call i32 (i8*, ...) @printf(i8* %7, i32 %6)
  %9 = load i32, i32* %5, align 4
  %10 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %11 = call i32 (i8*, ...) @printf(i8* %10, i32 %9)
  %12 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.4, i64 0, i64 0
  ; Converting complex expression to output
  %13 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %14 = call i32 (i8*, ...) @printf(i8* %13, i32 %12)
  %15 = load i32, i32* %5, align 4
  %16 = call i32 @len(i32 %15)
  ; Converting complex expression to output
  %17 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %18 = call i32 (i8*, ...) @printf(i8* %17, i32 %16)
  %19 = load i32, i32* %5, align 4
  %20 = call i32 @append(i32 %19, i32 3)
  %21 = alloca i32, align 4
  store i32 %20, i32* %21, align 4
  ; Variable new_arr allocated at %21
  %22 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.5, i64 0, i64 0
  ; Converting complex expression to output
  %23 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %24 = call i32 (i8*, ...) @printf(i8* %23, i32 %22)
  %25 = load i32, i32* %21, align 4
  %26 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %27 = call i32 (i8*, ...) @printf(i8* %26, i32 %25)
  %28 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.6, i64 0, i64 0
  ; Converting complex expression to output
  %29 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %30 = call i32 (i8*, ...) @printf(i8* %29, i32 %28)
  %31 = load i32, i32* %21, align 4
  %32 = call i32 @len(i32 %31)
  ; Converting complex expression to output
  %33 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %34 = call i32 (i8*, ...) @printf(i8* %33, i32 %32)
  ret i32 0
}
