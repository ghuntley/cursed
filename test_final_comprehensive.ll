; CURSED Language - Advanced LLVM Compilation
target triple = "x86_64-unknown-linux-gnu"


; Runtime function declarations
declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)
declare i8* @malloc(i64)
declare void @free(i8*)
declare i64 @strlen(i8*)
declare i8* @strcpy(i8*, i8*)

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
@_ZTI11CursedError = constant { i8*, i8* } { i8* null, i8* getelementptr inbounds ([14 x i8], [14 x i8]* @_ZTS11CursedError, i32 0, i32 0) }
@_ZTS11CursedError = constant [14 x i8] c"11CursedError\00"

define i8* @createPerson(i8* %name, i32 %age) {
entry:
  %0 = add i32 0, 0 ; placeholder
  ret i32 %0
}


; String constants
@.str.0 = private unnamed_addr constant [6 x i8] c"Alice\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"Bob\00", align 1
@.str.2 = private unnamed_addr constant [8 x i8] c"Charlie\00", align 1
@.str.3 = private unnamed_addr constant [28 x i8] c"Array type parsing: WORKING\00", align 1
@.str.4 = private unnamed_addr constant [28 x i8] c"Struct definitions: WORKING\00", align 1
@.str.5 = private unnamed_addr constant [30 x i8] c"Function definitions: WORKING\00", align 1
@.str.6 = private unnamed_addr constant [5 x i8] c"Test\00", align 1
@.str.7 = private unnamed_addr constant [25 x i8] c"Struct literals: WORKING\00", align 1
define i32 @main() {
entry:
  %0 = alloca [5x i32], align 4
  %1 = getelementptr inbounds [5x i32], [5x i32]* %0, i64 0, i64 0
  store i32 1, i32* %1, align 4
  %2 = getelementptr inbounds [5x i32], [5x i32]* %0, i64 0, i64 1
  store i32 2, i32* %2, align 4
  %3 = getelementptr inbounds [5x i32], [5x i32]* %0, i64 0, i64 2
  store i32 3, i32* %3, align 4
  %4 = getelementptr inbounds [5x i32], [5x i32]* %0, i64 0, i64 3
  store i32 4, i32* %4, align 4
  %5 = getelementptr inbounds [5x i32], [5x i32]* %0, i64 0, i64 4
  store i32 5, i32* %5, align 4
  %6 = alloca i8*, align 4
  store i8* %0, i8** %6, align 4
  ; Variable numbers allocated
  %7 = alloca [3x i32], align 4
  %8 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.0, i64 0, i64 0
  %9 = getelementptr inbounds [3x i32], [3x i32]* %7, i64 0, i64 0
  store i32 %8, i32* %9, align 4
  %10 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %11 = getelementptr inbounds [3x i32], [3x i32]* %7, i64 0, i64 1
  store i32 %10, i32* %11, align 4
  %12 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.2, i64 0, i64 0
  %13 = getelementptr inbounds [3x i32], [3x i32]* %7, i64 0, i64 2
  store i32 %12, i32* %13, align 4
  %14 = alloca i8*, align 4
  store i8* %7, i8** %14, align 4
  ; Variable names allocated
  %15 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.3, i64 0, i64 0
  %16 = call i32 @puts(i8* %15)
  %17 = add i32 0, 0
  ; Expression result: %17
  %18 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.4, i64 0, i64 0
  %19 = call i32 @puts(i8* %18)
  %20 = add i32 0, 0
  ; Expression result: %20
  %21 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.5, i64 0, i64 0
  %22 = call i32 @puts(i8* %21)
  %23 = add i32 0, 0
  ; Expression result: %23
  %24 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.6, i64 0, i64 0
  %25 = call i32 @createPerson(i8* %24, i32 25)
  %26 = alloca i8*, align 4
  store i8* %25, i8** %26, align 4
  ; Variable person allocated
  %27 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.7, i64 0, i64 0
  %28 = call i32 @puts(i8* %27)
  %29 = add i32 0, 0
  ; Expression result: %29
  ret i32 42
}

