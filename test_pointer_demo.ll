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
@.str.1 = private unnamed_addr constant [24 x i8] c"Value through pointer: \00", align 1
@.str.0 = private unnamed_addr constant [17 x i8] c"Original value: \00", align 1
@.str.2 = private unnamed_addr constant [17 x i8] c"Modified value: \00", align 1
define i32 @main() {
entry:
  %0 = alloca i32, align 4
  store i32 42, i32* %0, align 4
  ; Variable x allocated
  %1 = load i32, i32* %0, align 4
  %2 = alloca i32, align 4
  store i32 %0, i32* %2, align 4
  ; Variable ptr allocated
  %3 = load i32, i32* %2, align 4
  %4 = load i32, i32* %3, align 4
  %5 = alloca i32, align 4
  store i32 %4, i32* %5, align 4
  ; Variable value allocated
  %6 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.0, i64 0, i64 0
  %7 = load i32, i32* %0, align 4
  %8 = call i8* @i32_to_string(i32 %7)
  %9 = call i8* @string_concat(i8* %6, i8* %8)
  %10 = call i32 @puts(i8* %9)
  %11 = add i32 0, 0
  ; Expression result: %11
  %12 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.1, i64 0, i64 0
  %13 = load i32, i32* %5, align 4
  %14 = call i8* @i32_to_string(i32 %13)
  %15 = call i8* @string_concat(i8* %12, i8* %14)
  %16 = call i32 @puts(i8* %15)
  %17 = add i32 0, 0
  ; Expression result: %17
  %18 = load i32, i32* %2, align 4
  %19 = load i32, i32* %18, align 4
  ; Expression result: %19
  %20 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.2, i64 0, i64 0
  %21 = load i32, i32* %0, align 4
  %22 = call i8* @i32_to_string(i32 %21)
  %23 = call i8* @string_concat(i8* %20, i8* %22)
  %24 = call i32 @puts(i8* %23)
  %25 = add i32 0, 0
  ; Expression result: %25
  ret i32 0
}

