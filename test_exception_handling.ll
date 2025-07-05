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
@_ZTI11CursedError = external constant i8*



; String constants
@.str.0 = private unnamed_addr constant [21 x i8] c"This is a test error\00", align 1
@.str.1 = private unnamed_addr constant [14 x i8] c"Never reached\00", align 1
@.str.0 = private unnamed_addr constant [22 x i8] c"No exception thrown: \00", align 1
@.str.0 = private unnamed_addr constant [19 x i8] c"Caught exception: \00", align 1
@.str.4 = private unnamed_addr constant [30 x i8] c"Program continues after catch\00", align 1
define i32 @main() {
  ; Panic (yeet_error) statement with exception throwing
  %1 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.0, i64 0, i64 0
  %exception_alloc = call i8* @__cxa_allocate_exception(i64 8)
  %exception_cast = bitcast i8* %exception_alloc to i8**
  store i8* %1, i8** %exception_cast
  call void @__cxa_throw(i8* %exception_alloc, i8* @_ZTI11CursedError, i8* null)
  unreachable
  %2 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.1, i64 0, i64 0
  ret i8* %2
  %4 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.0, i64 0, i64 0
  %5 = add i32 %4, %result
  %3 = call i32 @println(i32 %5)
  %7 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.0, i64 0, i64 0
  %8 = add i32 %7, %error
  %6 = call i32 @println(i32 %8)
  %10 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.4, i64 0, i64 0
  %9 = call i32 @println(i32 %10)
  ret i32 0
}
