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

define i32 @connect_db() {
entry:
  %0 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  ret i32 0
}

define i8* @build_query() {
entry:
  %0 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.1, i64 0, i64 0
  %1 = alloca i8*, align 4
  store i8* %0, i8** %1, align 4
  ; Variable query allocated
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %2
  %3 = load i8*, i8** %1, align 4
  ; Expression result: %3
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %4
  ret i32 0
}



; String constants
@.str.4 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.2 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.3 = private unnamed_addr constant [21 x i8] c"SQL test starting...\00", align 1
@.str.1 = private unnamed_addr constant [20 x i8] c"SELECT * FROM users\00", align 1
@.str.5 = private unnamed_addr constant [20 x i8] c"SQL test completed!\00", align 1
@.str.0 = private unnamed_addr constant [19 x i8] c"Database connected\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.3, i64 0, i64 0
  ; Converting complex expression to output
  %1 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.4, i64 0, i64 0
  %2 = call i32 (i8*, ...) @printf(i8* %1, i32 %0)
  %3 = call i32 @connect_db()
  %4 = call i32 @build_query()
  %5 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.5, i64 0, i64 0
  ; Converting complex expression to output
  %6 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.4, i64 0, i64 0
  %7 = call i32 (i8*, ...) @printf(i8* %6, i32 %5)
  ret i32 0
}
