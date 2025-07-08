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

declare i8* @cursed_error_init(i8*, i8*)
declare i8* @cursed_create_error(i8*)
declare i1 @cursed_is_error(i8*)
declare void @cursed_propagate_error(i8*)
declare void @cursed_try_begin()
declare void @cursed_try_end()
declare i8* @cursed_get_panic_value()
declare i8* @malloc(i32)
declare void @free(i8*)
@error_msg_default = private unnamed_addr constant [13 x i8] c"Error occurred\00"
define i8* @sha256_hash(i8* %data) {
entry:
  %0 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.0, i64 0, i64 0
  %1 = add i32 %0, %data
  %2 = alloca i8*, align 4
  store i8* %1, i8** %2, align 4
  ; Variable result allocated
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %3
  %4 = load i8*, i8** %2, align 4
  ; Expression result: %4
  ret i32 0
}

define i8* @crc32_hash(i8* %data) {
entry:
  %0 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.2, i64 0, i64 0
  %1 = add i32 %0, %data
  %2 = alloca i8*, align 4
  store i8* %1, i8** %2, align 4
  ; Variable result allocated
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %3
  %4 = load i8*, i8** %2, align 4
  ; Expression result: %4
  ret i32 0
}

define i32 @test_basic() {
entry:
  %0 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.3, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.4, i64 0, i64 0
  %4 = call i32 @sha256_hash(i32 %3)
  %5 = alloca i32, align 4
  store i32 %4, i32* %5, align 4
  ; Variable test1 allocated
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %6
  %7 = load i32, i32* %5, align 4
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.4, i64 0, i64 0
  %10 = call i32 @crc32_hash(i32 %9)
  %11 = alloca i32, align 4
  store i32 %10, i32* %11, align 4
  ; Variable test2 allocated
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %12
  %13 = load i32, i32* %11, align 4
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  %15 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.5, i64 0, i64 0
  %16 = call i32 @puts(i8* %15)
  %17 = add i32 0, 0
  ; Expression result: %17
  ret i32 0
}



; String constants
@.str.2 = private unnamed_addr constant [7 x i8] c"crc32_\00", align 1
@.str.4 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
@.str.0 = private unnamed_addr constant [8 x i8] c"sha256_\00", align 1
@.str.1 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.3 = private unnamed_addr constant [32 x i8] c"Testing basic hash functions...\00", align 1
@.str.5 = private unnamed_addr constant [23 x i8] c"Basic tests completed!\00", align 1
define i32 @main() {
  %0 = call i32 @test_basic()
  ret i32 0
}
