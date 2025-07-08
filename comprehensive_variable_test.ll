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
@.str.6 = private unnamed_addr constant [15 x i8] c"Boolean (lit):\00", align 1
@.str.14 = private unnamed_addr constant [13 x i8] c"Direct char:\00", align 1
@.str.7 = private unnamed_addr constant [17 x i8] c"Character (sip):\00", align 1
@.str.3 = private unnamed_addr constant [16 x i8] c"Integer (drip):\00", align 1
@.str.10 = private unnamed_addr constant [14 x i8] c"Direct float:\00", align 1
@.str.12 = private unnamed_addr constant [7 x i8] c"direct\00", align 1
@.str.8 = private unnamed_addr constant [21 x i8] c"=== Literal Test ===\00", align 1
@.str.13 = private unnamed_addr constant [13 x i8] c"Direct bool:\00", align 1
@.str.1 = private unnamed_addr constant [30 x i8] c"=== Variable Display Test ===\00", align 1
@.str.2 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.9 = private unnamed_addr constant [12 x i8] c"Direct int:\00", align 1
@.str.11 = private unnamed_addr constant [15 x i8] c"Direct string:\00", align 1
@.str.4 = private unnamed_addr constant [14 x i8] c"Float (drip):\00", align 1
@.str.0 = private unnamed_addr constant [12 x i8] c"hello world\00", align 1
@.str.5 = private unnamed_addr constant [14 x i8] c"String (tea):\00", align 1
define i32 @main() {
  %0 = alloca double, align 4
  store double 42, double* %0, align 4
  ; Variable int_var allocated at %0
  %1 = alloca double, align 4
  store double 3.14, double* %1, align 4
  ; Variable float_var allocated at %1
  %2 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.0, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable string_var allocated at %3
  %4 = alloca i1, align 4
  store i1 1, i1* %4, align 4
  ; Variable bool_var allocated at %4
  %5 = alloca i8, align 4
  store i8 88, i8* %5, align 4
  ; Variable char_var allocated at %5
  %6 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.1, i64 0, i64 0
  ; Converting complex expression to output
  %7 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %8 = call i32 (i8*, ...) @printf(i8* %7, i32 %6)
  %9 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.3, i64 0, i64 0
  ; Converting complex expression to output
  %10 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %11 = call i32 (i8*, ...) @printf(i8* %10, i32 %9)
  %12 = load i32, i32* %0, align 4
  %13 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %14 = call i32 (i8*, ...) @printf(i8* %13, i32 %12)
  %15 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.4, i64 0, i64 0
  ; Converting complex expression to output
  %16 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %17 = call i32 (i8*, ...) @printf(i8* %16, i32 %15)
  %18 = load i32, i32* %1, align 4
  %19 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %20 = call i32 (i8*, ...) @printf(i8* %19, i32 %18)
  %21 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.5, i64 0, i64 0
  ; Converting complex expression to output
  %22 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %23 = call i32 (i8*, ...) @printf(i8* %22, i32 %21)
  %24 = load i32, i32* %3, align 4
  %25 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %26 = call i32 (i8*, ...) @printf(i8* %25, i32 %24)
  %27 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.6, i64 0, i64 0
  ; Converting complex expression to output
  %28 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %29 = call i32 (i8*, ...) @printf(i8* %28, i32 %27)
  %30 = load i32, i32* %4, align 4
  %31 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %32 = call i32 (i8*, ...) @printf(i8* %31, i32 %30)
  %33 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.7, i64 0, i64 0
  ; Converting complex expression to output
  %34 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %35 = call i32 (i8*, ...) @printf(i8* %34, i32 %33)
  %36 = load i32, i32* %5, align 4
  %37 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %38 = call i32 (i8*, ...) @printf(i8* %37, i32 %36)
  %39 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.8, i64 0, i64 0
  ; Converting complex expression to output
  %40 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %41 = call i32 (i8*, ...) @printf(i8* %40, i32 %39)
  %42 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.9, i64 0, i64 0
  ; Converting complex expression to output
  %43 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %44 = call i32 (i8*, ...) @printf(i8* %43, i32 %42)
  ; Converting complex expression to output
  %45 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %46 = call i32 (i8*, ...) @printf(i8* %45, i32 99)
  %47 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.10, i64 0, i64 0
  ; Converting complex expression to output
  %48 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %49 = call i32 (i8*, ...) @printf(i8* %48, i32 %47)
  ; Converting complex expression to output
  %50 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %51 = call i32 (i8*, ...) @printf(i8* %50, i32 2.71)
  %52 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.11, i64 0, i64 0
  ; Converting complex expression to output
  %53 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %54 = call i32 (i8*, ...) @printf(i8* %53, i32 %52)
  %55 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.12, i64 0, i64 0
  ; Converting complex expression to output
  %56 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %57 = call i32 (i8*, ...) @printf(i8* %56, i32 %55)
  %58 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.13, i64 0, i64 0
  ; Converting complex expression to output
  %59 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %60 = call i32 (i8*, ...) @printf(i8* %59, i32 %58)
  ; Converting complex expression to output
  %61 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %62 = call i32 (i8*, ...) @printf(i8* %61, i32 1)
  %63 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.14, i64 0, i64 0
  ; Converting complex expression to output
  %64 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %65 = call i32 (i8*, ...) @printf(i8* %64, i32 %63)
  ; Converting complex expression to output
  %66 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %67 = call i32 (i8*, ...) @printf(i8* %66, i32 89)
  ret i32 0
}
