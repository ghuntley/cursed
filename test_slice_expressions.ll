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
@.str.2 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.6 = private unnamed_addr constant [12 x i8] c"Slice [:]: \00", align 1
@.str.3 = private unnamed_addr constant [14 x i8] c"Slice [1:4]: \00", align 1
@.str.5 = private unnamed_addr constant [13 x i8] c"Slice [:3]: \00", align 1
@.str.1 = private unnamed_addr constant [17 x i8] c"Original array: \00", align 1
@.str.0 = private unnamed_addr constant [26 x i8] c"Testing slice expressions\00", align 1
@.str.4 = private unnamed_addr constant [13 x i8] c"Slice [2:]: \00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = alloca [5 x i32], align 4
  %3 = getelementptr inbounds [5 x i32], [5 x i32]* %2, i64 0, i64 0
  store i32 1, i32* %3, align 4
  %4 = getelementptr inbounds [5 x i32], [5 x i32]* %2, i64 0, i64 1
  store i32 2, i32* %4, align 4
  %5 = getelementptr inbounds [5 x i32], [5 x i32]* %2, i64 0, i64 2
  store i32 3, i32* %5, align 4
  %6 = getelementptr inbounds [5 x i32], [5 x i32]* %2, i64 0, i64 3
  store i32 4, i32* %6, align 4
  %7 = getelementptr inbounds [5 x i32], [5 x i32]* %2, i64 0, i64 4
  store i32 5, i32* %7, align 4
  %8 = alloca i32, align 4
  store i32 %2, i32* %8, align 4
  ; Variable numbers allocated at %8
  %9 = load i32, i32* %8, align 4
  %10 = sub i32 4, 1
  %11 = alloca [0 x i32], i32 %10, align 4
  ; Slice compilation: copying elements from %9 to %11 (start: 1, end: 4)
  %12 = alloca i32, align 4
  store i32 %11, i32* %12, align 4
  ; Variable slice1 allocated at %12
  %13 = load i32, i32* %8, align 4
  %14 = load i32, i32* getelementptr inbounds ([0 x i32], [0 x i32]* %13, i32 0, i32 -1), align 4 ; array length placeholder
  %15 = sub i32 %14, 2
  %16 = alloca [0 x i32], i32 %15, align 4
  ; Slice compilation: copying elements from %13 to %16 (start: 2, end: %14)
  %17 = alloca i32, align 4
  store i32 %16, i32* %17, align 4
  ; Variable slice2 allocated at %17
  %18 = load i32, i32* %8, align 4
  %19 = sub i32 3, 0
  %20 = alloca [0 x i32], i32 %19, align 4
  ; Slice compilation: copying elements from %18 to %20 (start: 0, end: 3)
  %21 = alloca i32, align 4
  store i32 %20, i32* %21, align 4
  ; Variable slice3 allocated at %21
  %22 = load i32, i32* %8, align 4
  %23 = load i32, i32* getelementptr inbounds ([0 x i32], [0 x i32]* %22, i32 0, i32 -1), align 4 ; array length placeholder
  %24 = sub i32 %23, 0
  %25 = alloca [0 x i32], i32 %24, align 4
  ; Slice compilation: copying elements from %22 to %25 (start: 0, end: %23)
  %26 = alloca i32, align 4
  store i32 %25, i32* %26, align 4
  ; Variable slice4 allocated at %26
  %27 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.1, i64 0, i64 0
  %28 = call i32 @puts(i8* %27)
  %29 = load i32, i32* %8, align 4
  %30 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %31 = call i32 (i8*, ...) @printf(i8* %30, i32 %29)
  %32 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.3, i64 0, i64 0
  %33 = call i32 @puts(i8* %32)
  %34 = load i32, i32* %12, align 4
  %35 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %36 = call i32 (i8*, ...) @printf(i8* %35, i32 %34)
  %37 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.4, i64 0, i64 0
  %38 = call i32 @puts(i8* %37)
  %39 = load i32, i32* %17, align 4
  %40 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %41 = call i32 (i8*, ...) @printf(i8* %40, i32 %39)
  %42 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.5, i64 0, i64 0
  %43 = call i32 @puts(i8* %42)
  %44 = load i32, i32* %21, align 4
  %45 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %46 = call i32 (i8*, ...) @printf(i8* %45, i32 %44)
  %47 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.6, i64 0, i64 0
  %48 = call i32 @puts(i8* %47)
  %49 = load i32, i32* %26, align 4
  %50 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %51 = call i32 (i8*, ...) @printf(i8* %50, i32 %49)
  ret i32 0
}
