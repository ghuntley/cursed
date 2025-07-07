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
@.str.8 = private unnamed_addr constant [14 x i8] c"HMAC result: \00", align 1
@.str.0 = private unnamed_addr constant [43 x i8] c"🔐 Testing crypto in compilation mode...\00", align 1
@.str.4 = private unnamed_addr constant [17 x i8] c"Base64 decoded: \00", align 1
@.str.2 = private unnamed_addr constant [14 x i8] c"SHA256 hash: \00", align 1
@.str.7 = private unnamed_addr constant [4 x i8] c"key\00", align 1
@.str.5 = private unnamed_addr constant [16 x i8] c"Random number: \00", align 1
@.str.9 = private unnamed_addr constant [40 x i8] c"🎉 Crypto compilation test completed!\00", align 1
@.str.6 = private unnamed_addr constant [5 x i8] c"data\00", align 1
@.str.1 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
@.str.3 = private unnamed_addr constant [17 x i8] c"Base64 encoded: \00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [43 x i8], [43 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %4 = call i32 @crypto_sha256(i8* %3)
  %5 = alloca i32, align 4
  store i32 %4, i32* %5, align 4
  ; Variable hash allocated
  %6 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.2, i64 0, i64 0
  %7 = load i32, i32* %5, align 4
  %8 = call i8* @i32_to_string(i32 %7)
  %9 = call i8* @string_concat(i8* %6, i8* %8)
  %10 = call i32 @puts(i8* %9)
  %11 = add i32 0, 0
  ; Expression result: %11
  %12 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %13 = call i32 @crypto_base64_encode(i8* %12)
  %14 = alloca i32, align 4
  store i32 %13, i32* %14, align 4
  ; Variable encoded allocated
  %15 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.3, i64 0, i64 0
  %16 = load i32, i32* %14, align 4
  %17 = call i8* @i32_to_string(i32 %16)
  %18 = call i8* @string_concat(i8* %15, i8* %17)
  %19 = call i32 @puts(i8* %18)
  %20 = add i32 0, 0
  ; Expression result: %20
  %21 = load i32, i32* %14, align 4
  %22 = call i32 @crypto_base64_decode(i32 %21)
  %23 = alloca i32, align 4
  store i32 %22, i32* %23, align 4
  ; Variable decoded allocated
  %24 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.4, i64 0, i64 0
  %25 = load i32, i32* %23, align 4
  %26 = call i8* @i32_to_string(i32 %25)
  %27 = call i8* @string_concat(i8* %24, i8* %26)
  %28 = call i32 @puts(i8* %27)
  %29 = add i32 0, 0
  ; Expression result: %29
  %30 = call i32 @crypto_random_int(i32 1, i32 10)
  %31 = alloca i32, align 4
  store i32 %30, i32* %31, align 4
  ; Variable rand_num allocated
  %32 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.5, i64 0, i64 0
  %33 = load i32, i32* %31, align 4
  %34 = call i8* @i32_to_string(i32 %33)
  %35 = call i8* @string_concat(i8* %32, i8* %34)
  %36 = call i32 @puts(i8* %35)
  %37 = add i32 0, 0
  ; Expression result: %37
  %38 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.6, i64 0, i64 0
  %39 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.7, i64 0, i64 0
  %40 = call i32 @crypto_hmac_sha256(i8* %38, i8* %39)
  %41 = alloca i32, align 4
  store i32 %40, i32* %41, align 4
  ; Variable hmac allocated
  %42 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.8, i64 0, i64 0
  %43 = load i32, i32* %41, align 4
  %44 = call i8* @i32_to_string(i32 %43)
  %45 = call i8* @string_concat(i8* %42, i8* %44)
  %46 = call i32 @puts(i8* %45)
  %47 = add i32 0, 0
  ; Expression result: %47
  %48 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.9, i64 0, i64 0
  %49 = call i32 @puts(i8* %48)
  %50 = add i32 0, 0
  ; Expression result: %50
  ret i32 0
}

