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
@.str.3 = private unnamed_addr constant [8 x i8] c"Charlie\00", align 1
@.str.0 = private unnamed_addr constant [16 x i8] c"Testing maps...\00", align 1
@.str.2 = private unnamed_addr constant [4 x i8] c"Bob\00", align 1
@.str.1 = private unnamed_addr constant [6 x i8] c"Alice\00", align 1
@.str.4 = private unnamed_addr constant [25 x i8] c"Map created successfully\00", align 1
@.str.5 = private unnamed_addr constant [34 x i8] c"Trying to access Alice's score...\00", align 1
@.str.6 = private unnamed_addr constant [25 x i8] c"All map tests completed!\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = alloca %struct.map, align 8
  ; Map with 3 entries
  %4 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  ; Map entry 0: %4 -> 95
  %5 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  ; Map entry 1: %5 -> 87
  %6 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.3, i64 0, i64 0
  ; Map entry 2: %6 -> 92
  %7 = alloca i32, align 4
  store i32 %3, i32* %7, align 4
  ; Variable scores allocated
  %8 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.4, i64 0, i64 0
  %9 = call i32 @puts(i8* %8)
  %10 = add i32 0, 0
  ; Expression result: %10
  %11 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.5, i64 0, i64 0
  %12 = call i32 @puts(i8* %11)
  %13 = add i32 0, 0
  ; Expression result: %13
  %14 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.6, i64 0, i64 0
  %15 = call i32 @puts(i8* %14)
  %16 = add i32 0, 0
  ; Expression result: %16
  ret i32 0
}

