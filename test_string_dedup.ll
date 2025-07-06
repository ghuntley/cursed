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
@_ZTI11CursedError = constant { i8*, i8* } { i8* null, i8* getelementptr inbounds ([14 x i8], [14 x i8]* @_ZTS11CursedError, i32 0, i32 0) }
@_ZTS11CursedError = constant [14 x i8] c"11CursedError\00"



; String constants
@.str.0 = private unnamed_addr constant [6 x i8] c"Hello\00", align 1
@.str.1 = private unnamed_addr constant [6 x i8] c"World\00", align 1
@.str.2 = private unnamed_addr constant [5 x i8] c"%d\n\00", align 1
define i32 @main() {
  %1 = getelementptr inbounds ([6 x i8], [6 x i8]* @.str.0, i64 0, i64 0)
  ; Variable: msg1 = %1
  %2 = getelementptr inbounds ([6 x i8], [6 x i8]* @.str.0, i64 0, i64 0)
  ; Variable: msg2 = %2
  %3 = getelementptr inbounds ([6 x i8], [6 x i8]* @.str.1, i64 0, i64 0)
  ; Variable: msg3 = %3
  %4 = getelementptr inbounds ([6 x i8], [6 x i8]* @.str.0, i64 0, i64 0)
  ; Variable: msg4 = %4
  %5 = load i32, i32* %1, align 4
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.str.2, i64 0, i64 0), i32 %5)
  %6 = load i32, i32* %2, align 4
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.str.2, i64 0, i64 0), i32 %6)
  %7 = load i32, i32* %3, align 4
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.str.2, i64 0, i64 0), i32 %7)
  %8 = load i32, i32* %4, align 4
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.str.2, i64 0, i64 0), i32 %8)
  ret i32 0
}
