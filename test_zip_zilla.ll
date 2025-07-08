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

define i8* @test_deflate_compression() {
entry:
  %0 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.1, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable original_data allocated
  %4 = load i8*, i8** %3, align 4
  %5 = call i32 @deflate_compress(i32 %4, i32 6)
  %6 = alloca i32, align 4
  store i32 %5, i32* %6, align 4
  ; Variable compressed allocated
  %7 = load i32, i32* %6, align 4
  %8 = call i32 @deflate_decompress(i32 %7)
  %9 = alloca i32, align 4
  store i32 %8, i32* %9, align 4
  ; Variable decompressed allocated
  %10 = load i32, i32* %9, align 4
  %11 = load i8*, i8** %3, align 4
  %12 = call i32 @assert_eq_string(i32 %10, i32 %11)
  ; Expression result: %12
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %13
  %14 = load i8*, i8** %3, align 4
  ; Member access: %14.length
  %15 = getelementptr inbounds %struct.object, %struct.object* %14, i32 0, i32 0
  %16 = load i32, i32* %15, align 4
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %17
  ret i32 0
}

define i32 @test_deflate_empty_data() {
entry:
  %0 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.3, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable original_data allocated
  %4 = load i8*, i8** %3, align 4
  %5 = call i32 @deflate_compress(i32 %4, i32 6)
  %6 = alloca i32, align 4
  store i32 %5, i32* %6, align 4
  ; Variable compressed allocated
  %7 = load i32, i32* %6, align 4
  %8 = call i32 @deflate_decompress(i32 %7)
  %9 = alloca i32, align 4
  store i32 %8, i32* %9, align 4
  ; Variable decompressed allocated
  %10 = load i32, i32* %9, align 4
  %11 = load i8*, i8** %3, align 4
  %12 = call i32 @assert_eq_string(i32 %10, i32 %11)
  ; Expression result: %12
  ret i32 0
}

define i8* @test_deflate_repetitive_data() {
entry:
  %0 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.4, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.5, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable original_data allocated
  %4 = load i8*, i8** %3, align 4
  %5 = call i32 @deflate_compress(i32 %4, i32 6)
  %6 = alloca i32, align 4
  store i32 %5, i32* %6, align 4
  ; Variable compressed allocated
  %7 = load i32, i32* %6, align 4
  %8 = call i32 @deflate_decompress(i32 %7)
  %9 = alloca i32, align 4
  store i32 %8, i32* %9, align 4
  ; Variable decompressed allocated
  %10 = load i32, i32* %9, align 4
  %11 = load i8*, i8** %3, align 4
  %12 = call i32 @assert_eq_string(i32 %10, i32 %11)
  ; Expression result: %12
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %13
  %14 = load i8*, i8** %3, align 4
  ; Member access: %14.length
  %15 = getelementptr inbounds %struct.object, %struct.object* %14, i32 0, i32 0
  %16 = load i32, i32* %15, align 4
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  ; Expression result: %17
  ret i32 0
}

define i32 @test_deflate_mixed_data() {
entry:
  %0 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.6, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.7, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable original_data allocated
  %4 = load i8*, i8** %3, align 4
  %5 = call i32 @deflate_compress(i32 %4, i32 6)
  %6 = alloca i32, align 4
  store i32 %5, i32* %6, align 4
  ; Variable compressed allocated
  %7 = load i32, i32* %6, align 4
  %8 = call i32 @deflate_decompress(i32 %7)
  %9 = alloca i32, align 4
  store i32 %8, i32* %9, align 4
  ; Variable decompressed allocated
  %10 = load i32, i32* %9, align 4
  %11 = load i8*, i8** %3, align 4
  %12 = call i32 @assert_eq_string(i32 %10, i32 %11)
  ; Expression result: %12
  ret i32 0
}

define i32 @test_deflate_compression_levels() {
entry:
  %0 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.8, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.9, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable original_data allocated
  %4 = load i8*, i8** %3, align 4
  %5 = call i32 @deflate_compress(i32 %4, i32 1)
  %6 = alloca i32, align 4
  store i32 %5, i32* %6, align 4
  ; Variable compressed_1 allocated
  %7 = load i8*, i8** %3, align 4
  %8 = call i32 @deflate_compress(i32 %7, i32 9)
  %9 = alloca i32, align 4
  store i32 %8, i32* %9, align 4
  ; Variable compressed_9 allocated
  %10 = load i32, i32* %6, align 4
  %11 = call i32 @deflate_decompress(i32 %10)
  %12 = alloca i32, align 4
  store i32 %11, i32* %12, align 4
  ; Variable decompressed_1 allocated
  %13 = load i32, i32* %9, align 4
  %14 = call i32 @deflate_decompress(i32 %13)
  %15 = alloca i32, align 4
  store i32 %14, i32* %15, align 4
  ; Variable decompressed_9 allocated
  %16 = load i32, i32* %12, align 4
  %17 = load i8*, i8** %3, align 4
  %18 = call i32 @assert_eq_string(i32 %16, i32 %17)
  ; Expression result: %18
  %19 = load i32, i32* %15, align 4
  %20 = load i8*, i8** %3, align 4
  %21 = call i32 @assert_eq_string(i32 %19, i32 %20)
  ; Expression result: %21
  ret i32 0
}

define i32 @test_zip_create_basic() {
entry:
  %0 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.10, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = alloca [0x i32], align 4
  %3 = alloca [0 x i32]*, align 4
  store [0 x i32]* %2, [0 x i32]** %3, align 4
  ; Variable files allocated
  %4 = alloca [0x i32], align 4
  %5 = alloca [0 x i32]*, align 4
  store [0 x i32]* %4, [0 x i32]** %5, align 4
  ; Variable contents allocated
  %6 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.11, i64 0, i64 0
  %7 = load [0 x i32]*, [0 x i32]** %3, align 4
  %8 = load [0 x i32]*, [0 x i32]** %5, align 4
  %9 = call i32 @zip_create(i32 %6, i32 %7, i32 %8)
  %10 = alloca i32, align 4
  store i32 %9, i32* %10, align 4
  ; Variable result allocated
  %11 = load i32, i32* %10, align 4
  %12 = call i32 @assert_true(i32 %11)
  ; Expression result: %12
  ret i32 0
}

define i32 @test_zip_create_empty() {
entry:
  %0 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.12, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = alloca [0x i32], align 4
  %3 = alloca [0 x i32]*, align 4
  store [0 x i32]* %2, [0 x i32]** %3, align 4
  ; Variable files allocated
  %4 = alloca [0x i32], align 4
  %5 = alloca [0 x i32]*, align 4
  store [0 x i32]* %4, [0 x i32]** %5, align 4
  ; Variable contents allocated
  %6 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.13, i64 0, i64 0
  %7 = load [0 x i32]*, [0 x i32]** %3, align 4
  %8 = load [0 x i32]*, [0 x i32]** %5, align 4
  %9 = call i32 @zip_create(i32 %6, i32 %7, i32 %8)
  %10 = alloca i32, align 4
  store i32 %9, i32* %10, align 4
  ; Variable result allocated
  %11 = load i32, i32* %10, align 4
  %12 = call i32 @assert_true(i32 %11)
  ; Expression result: %12
  ret i32 0
}

define i32 @test_zip_create_mismatched_arrays() {
entry:
  %0 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.14, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = alloca [0x i32], align 4
  %3 = alloca [0 x i32]*, align 4
  store [0 x i32]* %2, [0 x i32]** %3, align 4
  ; Variable files allocated
  %4 = alloca [0x i32], align 4
  %5 = alloca [0 x i32]*, align 4
  store [0 x i32]* %4, [0 x i32]** %5, align 4
  ; Variable contents allocated
  %6 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.11, i64 0, i64 0
  %7 = load [0 x i32]*, [0 x i32]** %3, align 4
  %8 = load [0 x i32]*, [0 x i32]** %5, align 4
  %9 = call i32 @zip_create(i32 %6, i32 %7, i32 %8)
  %10 = alloca i32, align 4
  store i32 %9, i32* %10, align 4
  ; Variable result allocated
  %11 = load i32, i32* %10, align 4
  %12 = call i32 @assert_false(i32 %11)
  ; Expression result: %12
  ret i32 0
}

define void @test_zip_extract_basic() {
entry:
  %0 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.15, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.2, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable archive_data allocated
  ret void
}



; String constants
@.str.1 = private unnamed_addr constant [24 x i8] c"hello world hello world\00", align 1
@.str.4 = private unnamed_addr constant [24 x i8] c"deflate repetitive data\00", align 1
@.str.11 = private unnamed_addr constant [9 x i8] c"test.zip\00", align 1
@.str.3 = private unnamed_addr constant [19 x i8] c"deflate empty data\00", align 1
@.str.13 = private unnamed_addr constant [10 x i8] c"empty.zip\00", align 1
@.str.8 = private unnamed_addr constant [27 x i8] c"deflate compression levels\00", align 1
@.str.9 = private unnamed_addr constant [33 x i8] c"test data for compression levels\00", align 1
@.str.14 = private unnamed_addr constant [29 x i8] c"zip create mismatched arrays\00", align 1
@.str.5 = private unnamed_addr constant [23 x i8] c"aaaaaaaaaaaaaaaaaaaaaa\00", align 1
@.str.12 = private unnamed_addr constant [17 x i8] c"zip create empty\00", align 1
@.str.6 = private unnamed_addr constant [19 x i8] c"deflate mixed data\00", align 1
@.str.0 = private unnamed_addr constant [26 x i8] c"deflate compression basic\00", align 1
@.str.10 = private unnamed_addr constant [17 x i8] c"zip create basic\00", align 1
@.str.15 = private unnamed_addr constant [18 x i8] c"zip extract basic\00", align 1
@.str.7 = private unnamed_addr constant [37 x i8] c"abcdefghijklmnopqrstuvwxyz0123456789\00", align 1
@.str.2 = private unnamed_addr constant [1 x i8] c"\00", align 1
define i32 @main() {
  ret i32 0
}
