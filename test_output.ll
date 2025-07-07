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
@.str.2 = private unnamed_addr constant [40 x i8] c"🎉 Basic self-hosting test completed!\00", align 1
@.str.3 = private unnamed_addr constant [56 x i8] c"The CURSED compiler can parse and execute basic syntax.\00", align 1
@.str.0 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.1 = private unnamed_addr constant [30 x i8] c"Self-hosting test successful!\00", align 1
define i32 @main() {
  %0 = alloca i32, align 4
  store i32 null, i32* %0, align 4
  ; Variable message allocated at %0
  %1 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %3 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.1, i64 0, i64 0
  %4 = alloca i32, align 4
  store i32 null, i32* %4, align 4
  ; Variable count allocated at %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %7 = alloca i32, align 4
  store i32 null, i32* %7, align 4
  ; Variable flag allocated at %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %20 = alloca i32, align 4
  store i32 null, i32* %20, align 4
  ; Variable small_num allocated at %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %30 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %31 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %33 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.2, i64 0, i64 0
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %36 = getelementptr inbounds [56 x i8], [56 x i8]* @.str.3, i64 0, i64 0
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ret i32 0
}
