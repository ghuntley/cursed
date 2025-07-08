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
@.str.19 = private unnamed_addr constant [2 x i8] c"a\00", align 1
@.str.16 = private unnamed_addr constant [8 x i8] c"file.md\00", align 1
@.str.24 = private unnamed_addr constant [4 x i8] c"xyz\00", align 1
@.str.20 = private unnamed_addr constant [12 x i8] c"hello world\00", align 1
@.str.6 = private unnamed_addr constant [6 x i8] c"local\00", align 1
@.str.11 = private unnamed_addr constant [5 x i8] c".txt\00", align 1
@.str.3 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.26 = private unnamed_addr constant [9 x i8] c"universe\00", align 1
@.str.21 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
@.str.27 = private unnamed_addr constant [24 x i8] c"hello universe universe\00", align 1
@.str.1 = private unnamed_addr constant [14 x i8] c"usr/local/bin\00", align 1
@.str.5 = private unnamed_addr constant [2 x i8] c"1\00", align 1
@.str.13 = private unnamed_addr constant [19 x i8] c"usr/./local/../bin\00", align 1
@.str.8 = private unnamed_addr constant [4 x i8] c"bin\00", align 1
@.str.15 = private unnamed_addr constant [6 x i8] c"*.txt\00", align 1
@.str.0 = private unnamed_addr constant [21 x i8] c"Pathing Module Tests\00", align 1
@.str.23 = private unnamed_addr constant [6 x i8] c"lo wo\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"0\00", align 1
@.str.14 = private unnamed_addr constant [8 x i8] c"usr/bin\00", align 1
@.str.4 = private unnamed_addr constant [4 x i8] c"usr\00", align 1
@.str.7 = private unnamed_addr constant [2 x i8] c"2\00", align 1
@.str.17 = private unnamed_addr constant [6 x i8] c"a,b,c\00", align 1
@.str.9 = private unnamed_addr constant [23 x i8] c"usr/local/bin/file.txt\00", align 1
@.str.25 = private unnamed_addr constant [18 x i8] c"hello world world\00", align 1
@.str.12 = private unnamed_addr constant [15 x i8] c"/usr/local/bin\00", align 1
@.str.18 = private unnamed_addr constant [2 x i8] c",\00", align 1
@.str.22 = private unnamed_addr constant [6 x i8] c"world\00", align 1
@.str.10 = private unnamed_addr constant [9 x i8] c"file.txt\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  %2 = inttoptr i64 0 to [0 x i32]*
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable parts allocated at %3
  %4 = load i32, i32* %3, align 4
  %5 = call i32 @path_join(i32 %4)
  %6 = alloca i8*, align 4
  store i8* %5, i8** %6, align 4
  ; Variable result allocated at %6
  %7 = load i32, i32* %6, align 4
  %8 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.1, i64 0, i64 0
  %9 = call i32 @assert_eq_string(i32 %7, i32 %8)
  %10 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.1, i64 0, i64 0
  %11 = call i32 @path_split(i32 %10)
  %12 = alloca i32, align 4
  store i32 %11, i32* %12, align 4
  ; Variable split_parts allocated at %12
  %13 = load i32, i32* %12, align 4
  ; Member access: %13.length
  %14 = getelementptr inbounds %struct.object, %struct.object* %13, i32 0, i32 0
  %15 = load i32, i32* %14, align 4
  %16 = call i32 @assert_eq_int(i32 %15, i32 3)
  %17 = alloca [1 x i32], align 4
  %18 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.2, i64 0, i64 0
  %19 = getelementptr inbounds [1 x i32], [1 x i32]* %17, i64 0, i64 0
  store i32 %18, i32* %19, align 4
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %21 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.4, i64 0, i64 0
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %23 = alloca [1 x i32], align 4
  %24 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.5, i64 0, i64 0
  %25 = getelementptr inbounds [1 x i32], [1 x i32]* %23, i64 0, i64 0
  store i32 %24, i32* %25, align 4
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %27 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.6, i64 0, i64 0
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %29 = alloca [1 x i32], align 4
  %30 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.7, i64 0, i64 0
  %31 = getelementptr inbounds [1 x i32], [1 x i32]* %29, i64 0, i64 0
  store i32 %30, i32* %31, align 4
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %33 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.8, i64 0, i64 0
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %35 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.9, i64 0, i64 0
  %36 = call i32 @path_basename(i32 %35)
  %37 = alloca i8*, align 4
  store i8* %36, i8** %37, align 4
  ; Variable basename allocated at %37
  %38 = load i32, i32* %37, align 4
  %39 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.10, i64 0, i64 0
  %40 = call i32 @assert_eq_string(i32 %38, i32 %39)
  %41 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.9, i64 0, i64 0
  %42 = call i32 @path_dirname(i32 %41)
  %43 = alloca i8*, align 4
  store i8* %42, i8** %43, align 4
  ; Variable dirname allocated at %43
  %44 = load i32, i32* %43, align 4
  %45 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.1, i64 0, i64 0
  %46 = call i32 @assert_eq_string(i32 %44, i32 %45)
  %47 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.10, i64 0, i64 0
  %48 = call i32 @path_ext(i32 %47)
  %49 = alloca i8*, align 4
  store i8* %48, i8** %49, align 4
  ; Variable ext allocated at %49
  %50 = load i32, i32* %49, align 4
  %51 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.11, i64 0, i64 0
  %52 = call i32 @assert_eq_string(i32 %50, i32 %51)
  %53 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.12, i64 0, i64 0
  %54 = call i32 @path_is_abs(i32 %53)
  %55 = call i32 @assert_true(i32 %54)
  %56 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.1, i64 0, i64 0
  %57 = call i32 @path_is_abs(i32 %56)
  %58 = call i32 @assert_false(i32 %57)
  %59 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.13, i64 0, i64 0
  %60 = call i32 @path_clean(i32 %59)
  %61 = alloca i8*, align 4
  store i8* %60, i8** %61, align 4
  ; Variable clean allocated at %61
  %62 = load i32, i32* %61, align 4
  %63 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.14, i64 0, i64 0
  %64 = call i32 @assert_eq_string(i32 %62, i32 %63)
  %65 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.15, i64 0, i64 0
  %66 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.10, i64 0, i64 0
  %67 = call i32 @path_match(i32 %65, i32 %66)
  %68 = call i32 @assert_true(i32 %67)
  %69 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.15, i64 0, i64 0
  %70 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.16, i64 0, i64 0
  %71 = call i32 @path_match(i32 %69, i32 %70)
  %72 = call i32 @assert_false(i32 %71)
  %73 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.17, i64 0, i64 0
  %74 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.18, i64 0, i64 0
  %75 = call i32 @string_split(i32 %73, i32 %74)
  %76 = alloca i32, align 4
  store i32 %75, i32* %76, align 4
  ; Variable str_parts allocated at %76
  %77 = load i32, i32* %76, align 4
  ; Member access: %77.length
  %78 = getelementptr inbounds %struct.object, %struct.object* %77, i32 0, i32 0
  %79 = load i32, i32* %78, align 4
  %80 = call i32 @assert_eq_int(i32 %79, i32 3)
  %81 = alloca [1 x i32], align 4
  %82 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.2, i64 0, i64 0
  %83 = getelementptr inbounds [1 x i32], [1 x i32]* %81, i64 0, i64 0
  store i32 %82, i32* %83, align 4
  %84 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %85 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.19, i64 0, i64 0
  %86 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %87 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.20, i64 0, i64 0
  %88 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.21, i64 0, i64 0
  %89 = call i32 @string_starts_with(i32 %87, i32 %88)
  %90 = call i32 @assert_true(i32 %89)
  %91 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.20, i64 0, i64 0
  %92 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %93 = call i32 @string_starts_with(i32 %91, i32 %92)
  %94 = call i32 @assert_false(i32 %93)
  %95 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.20, i64 0, i64 0
  %96 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %97 = call i32 @string_ends_with(i32 %95, i32 %96)
  %98 = call i32 @assert_true(i32 %97)
  %99 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.20, i64 0, i64 0
  %100 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.21, i64 0, i64 0
  %101 = call i32 @string_ends_with(i32 %99, i32 %100)
  %102 = call i32 @assert_false(i32 %101)
  %103 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.20, i64 0, i64 0
  %104 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.23, i64 0, i64 0
  %105 = call i32 @string_contains(i32 %103, i32 %104)
  %106 = call i32 @assert_true(i32 %105)
  %107 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.20, i64 0, i64 0
  %108 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.24, i64 0, i64 0
  %109 = call i32 @string_contains(i32 %107, i32 %108)
  %110 = call i32 @assert_false(i32 %109)
  %111 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.25, i64 0, i64 0
  %112 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.22, i64 0, i64 0
  %113 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.26, i64 0, i64 0
  %114 = call i32 @string_replace_all(i32 %111, i32 %112, i32 %113)
  %115 = alloca i8*, align 4
  store i8* %114, i8** %115, align 4
  ; Variable replaced allocated at %115
  %116 = load i32, i32* %115, align 4
  %117 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.27, i64 0, i64 0
  %118 = call i32 @assert_eq_string(i32 %116, i32 %117)
  %119 = call i32 @print_test_summary()
  ret i32 0
}
