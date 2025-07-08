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


; String constants
@.str.2 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.20 = private unnamed_addr constant [4 x i8] c"101\00", align 1
@.str.18 = private unnamed_addr constant [7 x i8] c"longer\00", align 1
@.str.34 = private unnamed_addr constant [11 x i8] c"NoNumbers!\00", align 1
@.str.29 = private unnamed_addr constant [7 x i8] c"abcdef\00", align 1
@.str.23 = private unnamed_addr constant [5 x i8] c"fail\00", align 1
@.str.24 = private unnamed_addr constant [11 x i8] c"private123\00", align 1
@.str.30 = private unnamed_addr constant [12 x i8] c"StrongP@ss1\00", align 1
@.str.5 = private unnamed_addr constant [7 x i8] c"secret\00", align 1
@.str.15 = private unnamed_addr constant [5 x i8] c"same\00", align 1
@.str.8 = private unnamed_addr constant [10 x i8] c"different\00", align 1
@.str.16 = private unnamed_addr constant [8 x i8] c"strings\00", align 1
@.str.19 = private unnamed_addr constant [3 x i8] c"72\00", align 1
@.str.31 = private unnamed_addr constant [5 x i8] c"weak\00", align 1
@.str.33 = private unnamed_addr constant [16 x i8] c"NOLOWERCASE123!\00", align 1
@.str.7 = private unnamed_addr constant [4 x i8] c"ABC\00", align 1
@.str.3 = private unnamed_addr constant [5 x i8] c"test\00", align 1
@.str.36 = private unnamed_addr constant [12 x i8] c"secret data\00", align 1
@.str.12 = private unnamed_addr constant [18 x i8] c"important message\00", align 1
@.str.26 = private unnamed_addr constant [19 x i8] c"tampered|signature\00", align 1
@.str.9 = private unnamed_addr constant [11 x i8] c"mypassword\00", align 1
@.str.17 = private unnamed_addr constant [6 x i8] c"short\00", align 1
@.str.22 = private unnamed_addr constant [4 x i8] c"111\00", align 1
@.str.6 = private unnamed_addr constant [12 x i8] c"Hello World\00", align 1
@.str.10 = private unnamed_addr constant [11 x i8] c"randomsalt\00", align 1
@.str.13 = private unnamed_addr constant [11 x i8] c"secret key\00", align 1
@.str.28 = private unnamed_addr constant [13 x i8] c"confidential\00", align 1
@.str.35 = private unnamed_addr constant [18 x i8] c"NoSpecialChars123\00", align 1
@.str.11 = private unnamed_addr constant [14 x i8] c"wrongpassword\00", align 1
@.str.14 = private unnamed_addr constant [18 x i8] c"different message\00", align 1
@.str.32 = private unnamed_addr constant [16 x i8] c"nouppercase123!\00", align 1
@.str.1 = private unnamed_addr constant [12 x i8] c"hello world\00", align 1
@.str.21 = private unnamed_addr constant [4 x i8] c"108\00", align 1
@.str.4 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
@.str.25 = private unnamed_addr constant [10 x i8] c"public123\00", align 1
@.str.27 = private unnamed_addr constant [10 x i8] c"streamkey\00", align 1
@.str.0 = private unnamed_addr constant [32 x i8] c"Pure CURSED Crypto Module Tests\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  %2 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.1, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable data allocated at %3
  %4 = load i32, i32* %3, align 4
  %5 = call i32 @hash_simple(i32 %4)
  %6 = alloca i32, align 4
  store i32 %5, i32* %6, align 4
  ; Variable hash1 allocated at %6
  %7 = load i32, i32* %3, align 4
  %8 = call i32 @hash_djb2(i32 %7)
  %9 = alloca i32, align 4
  store i32 %8, i32* %9, align 4
  ; Variable hash2 allocated at %9
  %10 = load i32, i32* %3, align 4
  %11 = call i32 @hash_sdbm(i32 %10)
  %12 = alloca i32, align 4
  store i32 %11, i32* %12, align 4
  ; Variable hash3 allocated at %12
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %20 = load i32, i32* %9, align 4
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %22 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.3, i64 0, i64 0
  %23 = call i32 @hash_simple(i32 %22)
  %24 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.3, i64 0, i64 0
  %25 = call i32 @hash_simple(i32 %24)
  %26 = call i32 @assert_eq_int(i32 %23, i32 %25)
  %27 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.4, i64 0, i64 0
  %28 = alloca i8*, align 4
  store i8* %27, i8** %28, align 4
  ; Variable plaintext allocated at %28
  %29 = load i32, i32* %28, align 4
  %30 = call i32 @caesar_encrypt(i32 %29, i32 3)
  %31 = alloca i8*, align 4
  store i8* %30, i8** %31, align 4
  ; Variable encrypted allocated at %31
  %32 = load i32, i32* %31, align 4
  %33 = call i32 @caesar_decrypt(i32 %32, i32 3)
  %34 = alloca i8*, align 4
  store i8* %33, i8** %34, align 4
  ; Variable decrypted allocated at %34
  %35 = load i32, i32* %34, align 4
  %36 = load i32, i32* %28, align 4
  %37 = call i32 @assert_eq_string(i32 %35, i32 %36)
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %39 = load i32, i32* %28, align 4
  %40 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %41 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.5, i64 0, i64 0
  %42 = alloca i8*, align 4
  store i8* %41, i8** %42, align 4
  ; Variable key allocated at %42
  %43 = load i32, i32* %28, align 4
  %44 = load i32, i32* %42, align 4
  %45 = call i32 @xor_encrypt(i32 %43, i32 %44)
  %46 = alloca i8*, align 4
  store i8* %45, i8** %46, align 4
  ; Variable xor_encrypted allocated at %46
  %47 = load i32, i32* %46, align 4
  %48 = load i32, i32* %42, align 4
  %49 = call i32 @xor_decrypt(i32 %47, i32 %48)
  %50 = alloca i8*, align 4
  store i8* %49, i8** %50, align 4
  ; Variable xor_decrypted allocated at %50
  %51 = load i32, i32* %50, align 4
  %52 = load i32, i32* %28, align 4
  %53 = call i32 @assert_eq_string(i32 %51, i32 %52)
  %54 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %55 = load i32, i32* %28, align 4
  %56 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %57 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.6, i64 0, i64 0
  %58 = alloca i8*, align 4
  store i8* %57, i8** %58, align 4
  ; Variable original allocated at %58
  %59 = load i32, i32* %58, align 4
  %60 = call i32 @base64_encode(i32 %59)
  %61 = alloca i8*, align 4
  store i8* %60, i8** %61, align 4
  ; Variable encoded allocated at %61
  %62 = load i32, i32* %61, align 4
  %63 = call i32 @base64_decode(i32 %62)
  %64 = alloca i8*, align 4
  store i8* %63, i8** %64, align 4
  ; Variable decoded allocated at %64
  %65 = load i32, i32* %64, align 4
  %66 = load i32, i32* %58, align 4
  %67 = call i32 @assert_eq_string(i32 %65, i32 %66)
  %68 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %69 = load i32, i32* %58, align 4
  %70 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %71 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %72 = load i32, i32* %58, align 4
  %73 = call i32 @string_length(i32 %72)
  %74 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %75 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.7, i64 0, i64 0
  %76 = call i32 @hex_encode(i32 %75)
  %77 = alloca i8*, align 4
  store i8* %76, i8** %77, align 4
  ; Variable hex_encoded allocated at %77
  %78 = load i32, i32* %77, align 4
  %79 = call i32 @hex_decode(i32 %78)
  %80 = alloca i8*, align 4
  store i8* %79, i8** %80, align 4
  ; Variable hex_decoded allocated at %80
  %81 = load i32, i32* %80, align 4
  %82 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.7, i64 0, i64 0
  %83 = call i32 @assert_eq_string(i32 %81, i32 %82)
  %84 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %85 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %86 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.3, i64 0, i64 0
  %87 = call i32 @crc32_hash(i32 %86)
  %88 = alloca i32, align 4
  store i32 %87, i32* %88, align 4
  ; Variable crc1 allocated at %88
  %89 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.3, i64 0, i64 0
  %90 = call i32 @crc32_hash(i32 %89)
  %91 = alloca i32, align 4
  store i32 %90, i32* %91, align 4
  ; Variable crc2 allocated at %91
  %92 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.8, i64 0, i64 0
  %93 = call i32 @crc32_hash(i32 %92)
  %94 = alloca i32, align 4
  store i32 %93, i32* %94, align 4
  ; Variable crc3 allocated at %94
  %95 = load i32, i32* %88, align 4
  %96 = load i32, i32* %91, align 4
  %97 = call i32 @assert_eq_int(i32 %95, i32 %96)
  %98 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %99 = load i32, i32* %94, align 4
  %100 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %101 = call i32 @crypto_seed_random(i32 12345)
  %102 = call i32 @crypto_random()
  %103 = alloca i32, align 4
  store i32 %102, i32* %103, align 4
  ; Variable rand1 allocated at %103
  %104 = call i32 @crypto_random()
  %105 = alloca i32, align 4
  store i32 %104, i32* %105, align 4
  ; Variable rand2 allocated at %105
  %106 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %107 = load i32, i32* %105, align 4
  %108 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %109 = call i32 @crypto_random_bytes(i32 10)
  %110 = alloca i32, align 4
  store i32 %109, i32* %110, align 4
  ; Variable random_bytes allocated at %110
  %111 = load i32, i32* %110, align 4
  ; Member access: %111.length
  %112 = getelementptr inbounds %struct.object, %struct.object* %111, i32 0, i32 0
  %113 = load i32, i32* %112, align 4
  %114 = call i32 @assert_eq_int(i32 %113, i32 10)
  %115 = call i32 @crypto_random_string(i32 8)
  %116 = alloca i8*, align 4
  store i8* %115, i8** %116, align 4
  ; Variable random_string allocated at %116
  %117 = load i32, i32* %116, align 4
  %118 = call i32 @string_length(i32 %117)
  %119 = call i32 @assert_eq_int(i32 %118, i32 8)
  %120 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.9, i64 0, i64 0
  %121 = alloca i8*, align 4
  store i8* %120, i8** %121, align 4
  ; Variable password allocated at %121
  %122 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.10, i64 0, i64 0
  %123 = alloca i8*, align 4
  store i8* %122, i8** %123, align 4
  ; Variable salt allocated at %123
  %124 = load i32, i32* %121, align 4
  %125 = load i32, i32* %123, align 4
  %126 = call i32 @password_hash(i32 %124, i32 %125)
  %127 = alloca i8*, align 4
  store i8* %126, i8** %127, align 4
  ; Variable hash allocated at %127
  %128 = load i32, i32* %121, align 4
  %129 = load i32, i32* %123, align 4
  %130 = load i32, i32* %127, align 4
  %131 = call i32 @password_verify(i32 %128, i32 %129, i32 %130)
  %132 = call i32 @assert_true(i32 %131)
  %133 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.11, i64 0, i64 0
  %134 = load i32, i32* %123, align 4
  %135 = load i32, i32* %127, align 4
  %136 = call i32 @password_verify(i32 %133, i32 %134, i32 %135)
  %137 = call i32 @assert_false(i32 %136)
  %138 = load i32, i32* %121, align 4
  %139 = load i32, i32* %123, align 4
  %140 = call i32 @pbkdf2_simple(i32 %138, i32 %139, i32 100)
  %141 = alloca i8*, align 4
  store i8* %140, i8** %141, align 4
  ; Variable pbkdf_result allocated at %141
  %142 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %143 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %144 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.12, i64 0, i64 0
  %145 = alloca i8*, align 4
  store i8* %144, i8** %145, align 4
  ; Variable message allocated at %145
  %146 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.13, i64 0, i64 0
  %147 = alloca i8*, align 4
  store i8* %146, i8** %147, align 4
  ; Variable hmac_key allocated at %147
  %148 = load i32, i32* %147, align 4
  %149 = load i32, i32* %145, align 4
  %150 = call i32 @hmac_simple(i32 %148, i32 %149)
  %151 = alloca i8*, align 4
  store i8* %150, i8** %151, align 4
  ; Variable mac allocated at %151
  %152 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %153 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %154 = load i32, i32* %147, align 4
  %155 = load i32, i32* %145, align 4
  %156 = call i32 @hmac_simple(i32 %154, i32 %155)
  %157 = alloca i8*, align 4
  store i8* %156, i8** %157, align 4
  ; Variable mac2 allocated at %157
  %158 = load i32, i32* %151, align 4
  %159 = load i32, i32* %157, align 4
  %160 = call i32 @assert_eq_string(i32 %158, i32 %159)
  %161 = load i32, i32* %147, align 4
  %162 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.14, i64 0, i64 0
  %163 = call i32 @hmac_simple(i32 %161, i32 %162)
  %164 = alloca i8*, align 4
  store i8* %163, i8** %164, align 4
  ; Variable different_mac allocated at %164
  %165 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %166 = load i32, i32* %164, align 4
  %167 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %168 = load i32, i32* %121, align 4
  %169 = load i32, i32* %123, align 4
  %170 = call i32 @derive_key(i32 %168, i32 %169, i32 32)
  %171 = alloca i8*, align 4
  store i8* %170, i8** %171, align 4
  ; Variable derived_key allocated at %171
  %172 = load i32, i32* %171, align 4
  %173 = call i32 @string_length(i32 %172)
  %174 = call i32 @assert_eq_int(i32 %173, i32 32)
  %175 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.15, i64 0, i64 0
  %176 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.15, i64 0, i64 0
  %177 = call i32 @secure_compare(i32 %175, i32 %176)
  %178 = call i32 @assert_true(i32 %177)
  %179 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.8, i64 0, i64 0
  %180 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.16, i64 0, i64 0
  %181 = call i32 @secure_compare(i32 %179, i32 %180)
  %182 = call i32 @assert_false(i32 %181)
  %183 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.17, i64 0, i64 0
  %184 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.18, i64 0, i64 0
  %185 = call i32 @secure_compare(i32 %183, i32 %184)
  %186 = call i32 @assert_false(i32 %185)
  %187 = alloca [5 x i32], align 4
  %188 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.19, i64 0, i64 0
  %189 = getelementptr inbounds [5 x i32], [5 x i32]* %187, i64 0, i64 0
  store i32 %188, i32* %189, align 4
  %190 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.20, i64 0, i64 0
  %191 = getelementptr inbounds [5 x i32], [5 x i32]* %187, i64 0, i64 1
  store i32 %190, i32* %191, align 4
  %192 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.21, i64 0, i64 0
  %193 = getelementptr inbounds [5 x i32], [5 x i32]* %187, i64 0, i64 2
  store i32 %192, i32* %193, align 4
  %194 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.21, i64 0, i64 0
  %195 = getelementptr inbounds [5 x i32], [5 x i32]* %187, i64 0, i64 3
  store i32 %194, i32* %195, align 4
  %196 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.22, i64 0, i64 0
  %197 = getelementptr inbounds [5 x i32], [5 x i32]* %187, i64 0, i64 4
  store i32 %196, i32* %197, align 4
  %198 = alloca i32, align 4
  store i32 %187, i32* %198, align 4
  ; Variable test_bytes allocated at %198
  %199 = load i32, i32* %198, align 4
  %200 = call i32 @crypto_bytes_to_hex(i32 %199)
  %201 = alloca i8*, align 4
  store i8* %200, i8** %201, align 4
  ; Variable bytes_hex allocated at %201
  %202 = load i32, i32* %201, align 4
  %203 = call i32 @crypto_hex_to_bytes(i32 %202)
  %204 = alloca i32, align 4
  store i32 %203, i32* %204, align 4
  ; Variable hex_bytes allocated at %204
  %205 = load i32, i32* %198, align 4
  ; Member access: %205.length
  %206 = getelementptr inbounds %struct.object, %struct.object* %205, i32 0, i32 0
  %207 = load i32, i32* %206, align 4
  %208 = load i32, i32* %204, align 4
  ; Member access: %208.length
  %209 = getelementptr inbounds %struct.object, %struct.object* %208, i32 0, i32 0
  %210 = load i32, i32* %209, align 4
  %211 = call i32 @assert_eq_int(i32 %207, i32 %210)
  %212 = call i32 @crypto_generate_salt(i32 16)
  %213 = alloca i8*, align 4
  store i8* %212, i8** %213, align 4
  ; Variable salt_generated allocated at %213
  %214 = load i32, i32* %213, align 4
  %215 = call i32 @string_length(i32 %214)
  %216 = call i32 @assert_eq_int(i32 %215, i32 16)
  %217 = call i32 @crypto_generate_nonce(i32 12)
  %218 = alloca i8*, align 4
  store i8* %217, i8** %218, align 4
  ; Variable nonce allocated at %218
  %219 = load i32, i32* %218, align 4
  %220 = call i32 @string_length(i32 %219)
  %221 = call i32 @assert_eq_int(i32 %220, i32 12)
  %222 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.3, i64 0, i64 0
  %223 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.3, i64 0, i64 0
  %224 = call i32 @crypto_constant_time_compare(i32 %222, i32 %223)
  %225 = call i32 @assert_true(i32 %224)
  %226 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.3, i64 0, i64 0
  %227 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.23, i64 0, i64 0
  %228 = call i32 @crypto_constant_time_compare(i32 %226, i32 %227)
  %229 = call i32 @assert_false(i32 %228)
  %230 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.24, i64 0, i64 0
  %231 = alloca i8*, align 4
  store i8* %230, i8** %231, align 4
  ; Variable private_key allocated at %231
  %232 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.25, i64 0, i64 0
  %233 = alloca i8*, align 4
  store i8* %232, i8** %233, align 4
  ; Variable public_key allocated at %233
  %234 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.4, i64 0, i64 0
  %235 = load i32, i32* %231, align 4
  %236 = call i32 @sign_message(i32 %234, i32 %235)
  %237 = alloca i8*, align 4
  store i8* %236, i8** %237, align 4
  ; Variable signed_msg allocated at %237
  %238 = load i32, i32* %237, align 4
  %239 = load i32, i32* %233, align 4
  %240 = call i32 @verify_signature(i32 %238, i32 %239)
  %241 = call i32 @assert_true(i32 %240)
  %242 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.26, i64 0, i64 0
  %243 = load i32, i32* %233, align 4
  %244 = call i32 @verify_signature(i32 %242, i32 %243)
  %245 = call i32 @assert_false(i32 %244)
  %246 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.27, i64 0, i64 0
  %247 = alloca i8*, align 4
  store i8* %246, i8** %247, align 4
  ; Variable rc4_key allocated at %247
  %248 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.28, i64 0, i64 0
  %249 = alloca i8*, align 4
  store i8* %248, i8** %249, align 4
  ; Variable rc4_plaintext allocated at %249
  %250 = load i32, i32* %249, align 4
  %251 = load i32, i32* %247, align 4
  %252 = call i32 @rc4_crypt(i32 %250, i32 %251)
  %253 = alloca i8*, align 4
  store i8* %252, i8** %253, align 4
  ; Variable rc4_ciphertext allocated at %253
  %254 = load i32, i32* %253, align 4
  %255 = load i32, i32* %247, align 4
  %256 = call i32 @rc4_crypt(i32 %254, i32 %255)
  %257 = alloca i8*, align 4
  store i8* %256, i8** %257, align 4
  ; Variable rc4_decrypted allocated at %257
  %258 = load i32, i32* %257, align 4
  %259 = load i32, i32* %249, align 4
  %260 = call i32 @assert_eq_string(i32 %258, i32 %259)
  %261 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %262 = load i32, i32* %249, align 4
  %263 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %264 = call i32 @crypto_random_password(i32 12)
  %265 = alloca i8*, align 4
  store i8* %264, i8** %265, align 4
  ; Variable random_password allocated at %265
  %266 = load i32, i32* %265, align 4
  %267 = call i32 @string_length(i32 %266)
  %268 = call i32 @assert_eq_int(i32 %267, i32 12)
  %269 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.29, i64 0, i64 0
  %270 = call i32 @crypto_entropy_estimate(i32 %269)
  %271 = alloca i32, align 4
  store i32 %270, i32* %271, align 4
  ; Variable entropy allocated at %271
  %272 = load i32, i32* %271, align 4
  %273 = call i32 @assert_eq_int(i32 %272, i32 6)
  %274 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.30, i64 0, i64 0
  %275 = call i32 @crypto_is_strong_password(i32 %274)
  %276 = call i32 @assert_true(i32 %275)
  %277 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.31, i64 0, i64 0
  %278 = call i32 @crypto_is_strong_password(i32 %277)
  %279 = call i32 @assert_false(i32 %278)
  %280 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.32, i64 0, i64 0
  %281 = call i32 @crypto_is_strong_password(i32 %280)
  %282 = call i32 @assert_false(i32 %281)
  %283 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.33, i64 0, i64 0
  %284 = call i32 @crypto_is_strong_password(i32 %283)
  %285 = call i32 @assert_false(i32 %284)
  %286 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.34, i64 0, i64 0
  %287 = call i32 @crypto_is_strong_password(i32 %286)
  %288 = call i32 @assert_false(i32 %287)
  %289 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.35, i64 0, i64 0
  %290 = call i32 @crypto_is_strong_password(i32 %289)
  %291 = call i32 @assert_false(i32 %290)
  %292 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.36, i64 0, i64 0
  %293 = alloca i8*, align 4
  store i8* %292, i8** %293, align 4
  ; Variable sensitive allocated at %293
  %294 = load i32, i32* %293, align 4
  %295 = call i32 @crypto_wipe_string(i32 %294)
  %296 = alloca i8*, align 4
  store i8* %295, i8** %296, align 4
  ; Variable wiped allocated at %296
  %297 = load i32, i32* %296, align 4
  %298 = call i32 @string_length(i32 %297)
  %299 = load i32, i32* %293, align 4
  %300 = call i32 @string_length(i32 %299)
  %301 = call i32 @assert_eq_int(i32 %298, i32 %300)
  %302 = call i32 @print_test_summary()
  ret i32 0
}
