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


; String constants
@.str.0 = private unnamed_addr constant [29 x i8] c"Arrays created successfully!\00", align 1
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
  %6 = alloca [5 x i32]*, align 4
  store [5 x i32]* %0, [5 x i32]** %6, align 4
  ; Variable numbers allocated
  %7 = alloca [0x i32], align 4
  %8 = alloca [0 x i32]*, align 4
  store [0 x i32]* %7, [0 x i32]** %8, align 4
  ; Variable empty allocated
  %9 = alloca [3x i32], align 4
  %10 = getelementptr inbounds [3x i32], [3x i32]* %9, i64 0, i64 0
  store i32 10, i32* %10, align 4
  %11 = getelementptr inbounds [3x i32], [3x i32]* %9, i64 0, i64 1
  store i32 20, i32* %11, align 4
  %12 = getelementptr inbounds [3x i32], [3x i32]* %9, i64 0, i64 2
  store i32 30, i32* %12, align 4
  %13 = alloca [3 x i32]*, align 4
  store [3 x i32]* %9, [3 x i32]** %13, align 4
  ; Variable mixed allocated
  %14 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.0, i64 0, i64 0
  %15 = call i32 @puts(i8* %14)
  %16 = add i32 0, 0
  ; Expression result: %16
  ret i32 42
}

