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
@.str.11 = private unnamed_addr constant [26 x i8] c"All math tests completed!\00", align 1
@.str.0 = private unnamed_addr constant [26 x i8] c"Testing math functions...\00", align 1
@.str.5 = private unnamed_addr constant [15 x i8] c"min(4, 9) = %f\00", align 1
@.str.4 = private unnamed_addr constant [15 x i8] c"max(4, 9) = %f\00", align 1
@.str.6 = private unnamed_addr constant [15 x i8] c"pow(2, 3) = %f\00", align 1
@.str.7 = private unnamed_addr constant [12 x i8] c"sin(0) = %f\00", align 1
@.str.9 = private unnamed_addr constant [16 x i8] c"floor(3.7) = %f\00", align 1
@.str.10 = private unnamed_addr constant [15 x i8] c"ceil(3.2) = %f\00", align 1
@.str.1 = private unnamed_addr constant [13 x i8] c"sqrt(9) = %f\00", align 1
@.str.2 = private unnamed_addr constant [6 x i8] c"%d\\0A\00", align 1
@.str.3 = private unnamed_addr constant [13 x i8] c"abs(-5) = %f\00", align 1
@.str.8 = private unnamed_addr constant [12 x i8] c"cos(0) = %f\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = alloca double, align 4
  store double 4, double* %3, align 4
  ; Variable x allocated
  %4 = alloca double, align 4
  store double 9, double* %4, align 4
  ; Variable y allocated
  %5 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.1, i64 0, i64 0
  %6 = call i32 @puts(i8* %5)
  %7 = load double, double* %4, align 4
  %8 = call i32 @math_sqrt(i32 %7)
  %9 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %10 = call i32 (i8*, ...) @printf(i8* %9, i32 %8)
  %11 = add i32 0, 0
  ; Expression result: %11
  %12 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.3, i64 0, i64 0
  %13 = call i32 @puts(i8* %12)
  %14 = sub i32 0, 5
  %15 = call i32 @math_abs(i32 %14)
  %16 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %17 = call i32 (i8*, ...) @printf(i8* %16, i32 %15)
  %18 = add i32 0, 0
  ; Expression result: %18
  %19 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.4, i64 0, i64 0
  %20 = call i32 @puts(i8* %19)
  %21 = load double, double* %3, align 4
  %22 = load double, double* %4, align 4
  %23 = call i32 @math_max(i32 %21, i32 %22)
  %24 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %25 = call i32 (i8*, ...) @printf(i8* %24, i32 %23)
  %26 = add i32 0, 0
  ; Expression result: %26
  %27 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.5, i64 0, i64 0
  %28 = call i32 @puts(i8* %27)
  %29 = load double, double* %3, align 4
  %30 = load double, double* %4, align 4
  %31 = call i32 @math_min(i32 %29, i32 %30)
  %32 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %33 = call i32 (i8*, ...) @printf(i8* %32, i32 %31)
  %34 = add i32 0, 0
  ; Expression result: %34
  %35 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.6, i64 0, i64 0
  %36 = call i32 @puts(i8* %35)
  %37 = call i32 @math_pow(i32 2, i32 3)
  %38 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %39 = call i32 (i8*, ...) @printf(i8* %38, i32 %37)
  %40 = add i32 0, 0
  ; Expression result: %40
  %41 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.7, i64 0, i64 0
  %42 = call i32 @puts(i8* %41)
  %43 = call i32 @math_sin(i32 0)
  %44 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %45 = call i32 (i8*, ...) @printf(i8* %44, i32 %43)
  %46 = add i32 0, 0
  ; Expression result: %46
  %47 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %48 = call i32 @puts(i8* %47)
  %49 = call i32 @math_cos(i32 0)
  %50 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %51 = call i32 (i8*, ...) @printf(i8* %50, i32 %49)
  %52 = add i32 0, 0
  ; Expression result: %52
  %53 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.9, i64 0, i64 0
  %54 = call i32 @puts(i8* %53)
  %55 = call i32 @math_floor(i32 3.7)
  %56 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %57 = call i32 (i8*, ...) @printf(i8* %56, i32 %55)
  %58 = add i32 0, 0
  ; Expression result: %58
  %59 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.10, i64 0, i64 0
  %60 = call i32 @puts(i8* %59)
  %61 = call i32 @math_ceil(i32 3.2)
  %62 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %63 = call i32 (i8*, ...) @printf(i8* %62, i32 %61)
  %64 = add i32 0, 0
  ; Expression result: %64
  %65 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.11, i64 0, i64 0
  %66 = call i32 @puts(i8* %65)
  %67 = add i32 0, 0
  ; Expression result: %67
  ret i32 0
}

