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
@.str.4 = private unnamed_addr constant [8 x i8] c"Hash 2:\00", align 1
@.str.5 = private unnamed_addr constant [20 x i8] c"SHA-256 test passed\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.6 = private unnamed_addr constant [20 x i8] c"SHA-256 test failed\00", align 1
@.str.2 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
@.str.0 = private unnamed_addr constant [28 x i8] c"Testing crypto functions...\00", align 1
@.str.3 = private unnamed_addr constant [8 x i8] c"Hash 1:\00", align 1
@.str.7 = private unnamed_addr constant [18 x i8] c"Testing completed\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.0, i64 0, i64 0
  ; Converting complex expression to output
  %1 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %2 = call i32 (i8*, ...) @printf(i8* %1, i32 %0)
  %3 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %4 = call i32 @crypto_sha256(i32 %3)
  %5 = alloca i8*, align 4
  store i8* %4, i8** %5, align 4
  ; Variable hash1 allocated at %5
  %6 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %7 = call i32 @crypto_sha256(i32 %6)
  %8 = alloca i8*, align 4
  store i8* %7, i8** %8, align 4
  ; Variable hash2 allocated at %8
  %9 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.3, i64 0, i64 0
  ; Converting complex expression to output
  %10 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %11 = call i32 (i8*, ...) @printf(i8* %10, i32 %9)
  %12 = load i32, i32* %5, align 4
  %13 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %14 = call i32 (i8*, ...) @printf(i8* %13, i32 %12)
  %15 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.4, i64 0, i64 0
  ; Converting complex expression to output
  %16 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %17 = call i32 (i8*, ...) @printf(i8* %16, i32 %15)
  %18 = load i32, i32* %8, align 4
  %19 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %20 = call i32 (i8*, ...) @printf(i8* %19, i32 %18)
  ; DEBUG: generate_if_statement_with_init called
  ; DEBUG: about to process condition
  %21 = load i32, i32* %5, align 4
  %22 = load i32, i32* %8, align 4
  %23 = icmp eq i32 %21, %22
  br i1 %23, label %label0, label %label1
label0:
  %24 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.5, i64 0, i64 0
  ; Converting complex expression to output
  %25 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %26 = call i32 (i8*, ...) @printf(i8* %25, i32 %24)
  br label %label2
label1:
  %27 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.6, i64 0, i64 0
  ; Converting complex expression to output
  %28 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %29 = call i32 (i8*, ...) @printf(i8* %28, i32 %27)
  br label %label2
label2:
  %30 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.7, i64 0, i64 0
  ; Converting complex expression to output
  %31 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %32 = call i32 (i8*, ...) @printf(i8* %31, i32 %30)
  ret i32 0
}
