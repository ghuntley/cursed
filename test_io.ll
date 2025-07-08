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
define i32 @test_console_io() {
entry:
  %0 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.1, i64 0, i64 0
  %3 = call i32 @print(i32 %2)
  ; Expression result: %3
  %4 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.2, i64 0, i64 0
  %5 = call i32 @println(i32 %4)
  ; Expression result: %5
  %6 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.3, i64 0, i64 0
  %7 = call i32 @eprint(i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.4, i64 0, i64 0
  %9 = call i32 @eprintln(i32 %8)
  ; Expression result: %9
  %10 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.5, i64 0, i64 0
  %11 = alloca [0x i32], align 4
  %12 = call i32 @printf(i32 %10, i32 %11)
  ; Expression result: %12
  %13 = call i32 @assert_true(i32 1)
  ; Expression result: %13
  ret i32 0
}

define i32 @test_file_operations() {
entry:
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.6, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.7, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable test_file allocated
  %4 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.8, i64 0, i64 0
  %5 = alloca i8*, align 4
  store i8* %4, i8** %5, align 4
  ; Variable test_content allocated
  %6 = load i8*, i8** %3, align 4
  %7 = load i8*, i8** %5, align 4
  %8 = call i32 @write_file(i32 %6, i32 %7)
  %9 = alloca i32, align 4
  store i32 %8, i32* %9, align 4
  ; Variable write_success allocated
  %10 = load i32, i32* %9, align 4
  %11 = call i32 @assert_true(i32 %10)
  ; Expression result: %11
  %12 = load i8*, i8** %3, align 4
  %13 = call i32 @file_exists(i32 %12)
  %14 = call i32 @assert_true(i32 %13)
  ; Expression result: %14
  %15 = load i8*, i8** %3, align 4
  %16 = call i32 @is_file(i32 %15)
  %17 = call i32 @assert_true(i32 %16)
  ; Expression result: %17
  %18 = load i8*, i8** %3, align 4
  %19 = call i32 @is_directory(i32 %18)
  %20 = call i32 @assert_false(i32 %19)
  ; Expression result: %20
  %21 = load i8*, i8** %3, align 4
  %22 = call i32 @read_file(i32 %21)
  %23 = alloca i32, align 4
  store i32 %22, i32* %23, align 4
  ; Variable read_content allocated
  %24 = load i32, i32* %23, align 4
  %25 = load i8*, i8** %5, align 4
  %26 = call i32 @assert_eq_string(i32 %24, i32 %25)
  ; Expression result: %26
  %27 = load i8*, i8** %3, align 4
  %28 = call i32 @file_size(i32 %27)
  %29 = alloca i32, align 4
  store i32 %28, i32* %29, align 4
  ; Variable file_size_bytes allocated
  %30 = load i32, i32* %29, align 4
  %31 = load i8*, i8** %5, align 4
  %32 = call i32 @string_len(i32 %31)
  %33 = call i32 @assert_eq_int(i32 %30, i32 %32)
  ; Expression result: %33
  %34 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.9, i64 0, i64 0
  %35 = alloca i8*, align 4
  store i8* %34, i8** %35, align 4
  ; Variable append_content allocated
  %36 = load i8*, i8** %3, align 4
  %37 = load i8*, i8** %35, align 4
  %38 = call i32 @append_file(i32 %36, i32 %37)
  %39 = alloca i32, align 4
  store i32 %38, i32* %39, align 4
  ; Variable append_success allocated
  %40 = load i32, i32* %39, align 4
  %41 = call i32 @assert_true(i32 %40)
  ; Expression result: %41
  %42 = load i8*, i8** %3, align 4
  %43 = call i32 @read_file(i32 %42)
  %44 = alloca i32, align 4
  store i32 %43, i32* %44, align 4
  ; Variable full_content allocated
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %45
  %46 = load i8*, i8** %35, align 4
  ; Expression result: %46
  %47 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %47
  %48 = load i8*, i8** %3, align 4
  %49 = call i32 @delete_file(i32 %48)
  %50 = alloca i32, align 4
  store i32 %49, i32* %50, align 4
  ; Variable delete_success allocated
  %51 = load i32, i32* %50, align 4
  %52 = call i32 @assert_true(i32 %51)
  ; Expression result: %52
  %53 = load i8*, i8** %3, align 4
  %54 = call i32 @file_exists(i32 %53)
  %55 = call i32 @assert_false(i32 %54)
  ; Expression result: %55
  ret i32 0
}

define i32 @test_file_copy_move() {
entry:
  %0 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.11, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.12, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable source_file allocated
  %4 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.13, i64 0, i64 0
  %5 = alloca i8*, align 4
  store i8* %4, i8** %5, align 4
  ; Variable dest_file allocated
  %6 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.14, i64 0, i64 0
  %7 = alloca i8*, align 4
  store i8* %6, i8** %7, align 4
  ; Variable move_file allocated
  %8 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.15, i64 0, i64 0
  %9 = alloca i8*, align 4
  store i8* %8, i8** %9, align 4
  ; Variable test_content allocated
  %10 = load i8*, i8** %3, align 4
  %11 = load i8*, i8** %9, align 4
  %12 = call i32 @write_file(i32 %10, i32 %11)
  %13 = call i32 @assert_true(i32 %12)
  ; Expression result: %13
  %14 = load i8*, i8** %3, align 4
  %15 = load i8*, i8** %5, align 4
  %16 = call i32 @copy_file(i32 %14, i32 %15)
  %17 = call i32 @assert_true(i32 %16)
  ; Expression result: %17
  %18 = load i8*, i8** %3, align 4
  %19 = call i32 @file_exists(i32 %18)
  %20 = call i32 @assert_true(i32 %19)
  ; Expression result: %20
  %21 = load i8*, i8** %5, align 4
  %22 = call i32 @file_exists(i32 %21)
  %23 = call i32 @assert_true(i32 %22)
  ; Expression result: %23
  %24 = load i8*, i8** %5, align 4
  %25 = call i32 @read_file(i32 %24)
  %26 = load i8*, i8** %9, align 4
  %27 = call i32 @assert_eq_string(i32 %25, i32 %26)
  ; Expression result: %27
  %28 = load i8*, i8** %5, align 4
  %29 = load i8*, i8** %7, align 4
  %30 = call i32 @move_file(i32 %28, i32 %29)
  %31 = call i32 @assert_true(i32 %30)
  ; Expression result: %31
  %32 = load i8*, i8** %5, align 4
  %33 = call i32 @file_exists(i32 %32)
  %34 = call i32 @assert_false(i32 %33)
  ; Expression result: %34
  %35 = load i8*, i8** %7, align 4
  %36 = call i32 @file_exists(i32 %35)
  %37 = call i32 @assert_true(i32 %36)
  ; Expression result: %37
  %38 = load i8*, i8** %7, align 4
  %39 = call i32 @read_file(i32 %38)
  %40 = load i8*, i8** %9, align 4
  %41 = call i32 @assert_eq_string(i32 %39, i32 %40)
  ; Expression result: %41
  %42 = load i8*, i8** %3, align 4
  %43 = call i32 @delete_file(i32 %42)
  %44 = call i32 @assert_true(i32 %43)
  ; Expression result: %44
  %45 = load i8*, i8** %7, align 4
  %46 = call i32 @delete_file(i32 %45)
  %47 = call i32 @assert_true(i32 %46)
  ; Expression result: %47
  ret i32 0
}

define i8* @test_binary_file_operations() {
entry:
  %0 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.16, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.17, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable binary_file allocated
  %4 = alloca [9x i32], align 4
  %5 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.18, i64 0, i64 0
  %6 = getelementptr inbounds [9x i32], [9x i32]* %4, i64 0, i64 0
  store i32 %5, i32* %6, align 4
  %7 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.19, i64 0, i64 0
  %8 = getelementptr inbounds [9x i32], [9x i32]* %4, i64 0, i64 1
  store i32 %7, i32* %8, align 4
  %9 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.20, i64 0, i64 0
  %10 = getelementptr inbounds [9x i32], [9x i32]* %4, i64 0, i64 2
  store i32 %9, i32* %10, align 4
  %11 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.20, i64 0, i64 0
  %12 = getelementptr inbounds [9x i32], [9x i32]* %4, i64 0, i64 3
  store i32 %11, i32* %12, align 4
  %13 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.21, i64 0, i64 0
  %14 = getelementptr inbounds [9x i32], [9x i32]* %4, i64 0, i64 4
  store i32 %13, i32* %14, align 4
  %15 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.22, i64 0, i64 0
  %16 = getelementptr inbounds [9x i32], [9x i32]* %4, i64 0, i64 5
  store i32 %15, i32* %16, align 4
  %17 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.23, i64 0, i64 0
  %18 = getelementptr inbounds [9x i32], [9x i32]* %4, i64 0, i64 6
  store i32 %17, i32* %18, align 4
  %19 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.24, i64 0, i64 0
  %20 = getelementptr inbounds [9x i32], [9x i32]* %4, i64 0, i64 7
  store i32 %19, i32* %20, align 4
  %21 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.25, i64 0, i64 0
  %22 = getelementptr inbounds [9x i32], [9x i32]* %4, i64 0, i64 8
  store i32 %21, i32* %22, align 4
  %23 = alloca [9 x i32]*, align 4
  store [9 x i32]* %4, [9 x i32]** %23, align 4
  ; Variable test_bytes allocated
  %24 = load i8*, i8** %3, align 4
  %25 = load [9 x i32]*, [9 x i32]** %23, align 4
  %26 = call i32 @write_file_bytes(i32 %24, i32 %25)
  %27 = call i32 @assert_true(i32 %26)
  ; Expression result: %27
  %28 = load i8*, i8** %3, align 4
  %29 = call i32 @read_file_bytes(i32 %28)
  %30 = alloca i32, align 4
  store i32 %29, i32* %30, align 4
  ; Variable read_bytes allocated
  %31 = load i32, i32* %30, align 4
  %32 = call i32 @len(i32 %31)
  %33 = load [9 x i32]*, [9 x i32]** %23, align 4
  %34 = call i32 @len(i32 %33)
  %35 = call i32 @assert_eq_int(i32 %32, i32 %34)
  ; Expression result: %35
  %36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %36
  ; Expression result: %i
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %37
  %38 = load [9 x i32]*, [9 x i32]** %23, align 4
  %39 = call i32 @len(i32 %38)
  %40 = call i32 @range(i32 %39)
  ; Expression result: %40
  %41 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %41
  %42 = alloca [0x i32], align 4
  ; Expression result: %42
  %43 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %43
  %44 = load [9 x i32]*, [9 x i32]** %23, align 4
  ; Expression result: %44
  %45 = alloca [0x i32], align 4
  ; Expression result: %45
  %46 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %46
  ret i32 0
}

define i32 @test_directory_operations() {
entry:
  %0 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.26, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.27, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable test_dir allocated
  %4 = load i8*, i8** %3, align 4
  %5 = call i32 @create_directory(i32 %4)
  %6 = call i32 @assert_true(i32 %5)
  ; Expression result: %6
  %7 = load i8*, i8** %3, align 4
  %8 = call i32 @file_exists(i32 %7)
  %9 = call i32 @assert_true(i32 %8)
  ; Expression result: %9
  %10 = load i8*, i8** %3, align 4
  %11 = call i32 @is_directory(i32 %10)
  %12 = call i32 @assert_true(i32 %11)
  ; Expression result: %12
  %13 = load i8*, i8** %3, align 4
  %14 = call i32 @is_file(i32 %13)
  %15 = call i32 @assert_false(i32 %14)
  ; Expression result: %15
  %16 = call i32 @current_directory()
  %17 = alloca i32, align 4
  store i32 %16, i32* %17, align 4
  ; Variable current_dir allocated
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %18
  ; Expression result: 0
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %19
  %20 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.28, i64 0, i64 0
  %21 = call i32 @list_directory(i32 %20)
  %22 = alloca i32, align 4
  store i32 %21, i32* %22, align 4
  ; Variable dir_contents allocated
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %23
  ; Expression result: 0
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %24
  %25 = alloca [0x i32], align 4
  %26 = call i32 @path_join(i32 %25)
  %27 = alloca i32, align 4
  store i32 %26, i32* %27, align 4
  ; Variable nested_dir allocated
  %28 = load i32, i32* %27, align 4
  %29 = call i32 @create_directory_recursive(i32 %28)
  %30 = call i32 @assert_true(i32 %29)
  ; Expression result: %30
  %31 = load i32, i32* %27, align 4
  %32 = call i32 @is_directory(i32 %31)
  %33 = call i32 @assert_true(i32 %32)
  ; Expression result: %33
  %34 = load i8*, i8** %3, align 4
  %35 = call i32 @list_directory_recursive(i32 %34)
  %36 = alloca i32, align 4
  store i32 %35, i32* %36, align 4
  ; Variable recursive_contents allocated
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %37
  ; Expression result: 0
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %38
  %39 = load i8*, i8** %3, align 4
  %40 = call i32 @remove_directory_recursive(i32 %39)
  %41 = call i32 @assert_true(i32 %40)
  ; Expression result: %41
  %42 = load i8*, i8** %3, align 4
  %43 = call i32 @file_exists(i32 %42)
  %44 = call i32 @assert_false(i32 %43)
  ; Expression result: %44
  ret i32 0
}

define i32 @test_path_operations() {
entry:
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.29, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = alloca [0x i32], align 4
  %3 = call i32 @path_join(i32 %2)
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable joined_path allocated
  %5 = load i32, i32* %4, align 4
  %6 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.30, i64 0, i64 0
  %7 = call i32 @string_contains(i32 %5, i32 %6)
  %8 = call i32 @assert_true(i32 %7)
  ; Expression result: %8
  %9 = load i32, i32* %4, align 4
  %10 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.31, i64 0, i64 0
  %11 = call i32 @string_contains(i32 %9, i32 %10)
  %12 = call i32 @assert_true(i32 %11)
  ; Expression result: %12
  %13 = load i32, i32* %4, align 4
  %14 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.32, i64 0, i64 0
  %15 = call i32 @string_contains(i32 %13, i32 %14)
  %16 = call i32 @assert_true(i32 %15)
  ; Expression result: %16
  %17 = load i32, i32* %4, align 4
  %18 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.33, i64 0, i64 0
  %19 = call i32 @string_contains(i32 %17, i32 %18)
  %20 = call i32 @assert_true(i32 %19)
  ; Expression result: %20
  %21 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.34, i64 0, i64 0
  %22 = alloca i8*, align 4
  store i8* %21, i8** %22, align 4
  ; Variable test_path allocated
  %23 = load i8*, i8** %22, align 4
  %24 = call i32 @path_dirname(i32 %23)
  %25 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.35, i64 0, i64 0
  %26 = call i32 @assert_eq_string(i32 %24, i32 %25)
  ; Expression result: %26
  %27 = load i8*, i8** %22, align 4
  %28 = call i32 @path_basename(i32 %27)
  %29 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.33, i64 0, i64 0
  %30 = call i32 @assert_eq_string(i32 %28, i32 %29)
  ; Expression result: %30
  %31 = load i8*, i8** %22, align 4
  %32 = call i32 @path_extension(i32 %31)
  %33 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.36, i64 0, i64 0
  %34 = call i32 @assert_eq_string(i32 %32, i32 %33)
  ; Expression result: %34
  %35 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.37, i64 0, i64 0
  %36 = alloca i8*, align 4
  store i8* %35, i8** %36, align 4
  ; Variable relative_path allocated
  %37 = load i8*, i8** %36, align 4
  %38 = call i32 @path_absolute(i32 %37)
  %39 = alloca i32, align 4
  store i32 %38, i32* %39, align 4
  ; Variable absolute_path allocated
  %40 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %40
  %41 = load i8*, i8** %36, align 4
  %42 = call i32 @string_len(i32 %41)
  ; Expression result: %42
  %43 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %43
  %44 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.28, i64 0, i64 0
  %45 = call i32 @path_exists(i32 %44)
  %46 = call i32 @assert_true(i32 %45)
  ; Expression result: %46
  %47 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.38, i64 0, i64 0
  %48 = call i32 @path_exists(i32 %47)
  %49 = call i32 @assert_false(i32 %48)
  ; Expression result: %49
  ret i32 0
}

define i32 @test_stream_io() {
entry:
  %0 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.39, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.40, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable stream_file allocated
  %4 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.41, i64 0, i64 0
  %5 = alloca i8*, align 4
  store i8* %4, i8** %5, align 4
  ; Variable test_data allocated
  %6 = load i8*, i8** %3, align 4
  %7 = call i32 @open_file_write(i8* %6)
  %8 = alloca i32, align 4
  store i32 %7, i32* %8, align 4
  ; Variable write_handle allocated
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %9
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %11
  %12 = load i32, i32* %8, align 4
  %13 = load i8*, i8** %5, align 4
  %14 = call i32 @write_to_file(i32 %12, i32 %13)
  %15 = call i32 @assert_true(i32 %14)
  ; Expression result: %15
  %16 = load i32, i32* %8, align 4
  %17 = call i32 @flush_file(i32 %16)
  %18 = call i32 @assert_true(i32 %17)
  ; Expression result: %18
  %19 = load i32, i32* %8, align 4
  %20 = call i32 @close_file(i32 %19)
  %21 = call i32 @assert_true(i32 %20)
  ; Expression result: %21
  %22 = load i8*, i8** %3, align 4
  %23 = call i32 @open_file_read(i8* %22)
  %24 = alloca i32, align 4
  store i32 %23, i32* %24, align 4
  ; Variable read_handle allocated
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %26
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %27
  %28 = load i32, i32* %24, align 4
  %29 = load i8*, i8** %5, align 4
  %30 = call i32 @string_len(i32 %29)
  %31 = call i32 @read_from_file(i32 %28, i32 %30)
  %32 = alloca i32, align 4
  store i32 %31, i32* %32, align 4
  ; Variable read_data allocated
  %33 = load i32, i32* %32, align 4
  %34 = load i8*, i8** %5, align 4
  %35 = call i32 @assert_eq_string(i32 %33, i32 %34)
  ; Expression result: %35
  %36 = load i32, i32* %24, align 4
  %37 = call i32 @seek_file(i32 %36, i32 0)
  %38 = call i32 @assert_true(i32 %37)
  ; Expression result: %38
  %39 = load i32, i32* %24, align 4
  %40 = call i32 @tell_file(i32 %39)
  %41 = call i32 @assert_eq_int(i32 %40, i32 0)
  ; Expression result: %41
  %42 = load i32, i32* %24, align 4
  %43 = call i32 @close_file(i32 %42)
  %44 = call i32 @assert_true(i32 %43)
  ; Expression result: %44
  %45 = load i8*, i8** %3, align 4
  %46 = call i32 @delete_file(i8* %45)
  %47 = call i32 @assert_true(i32 %46)
  ; Expression result: %47
  ret i32 0
}

define i32 @test_buffered_io() {
entry:
  %0 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.42, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = alloca i32, align 4
  store i32 1024, i32* %2, align 4
  ; Variable buffer_size allocated
  %3 = load i32, i32* %2, align 4
  %4 = call i32 @create_buffer(i32 %3)
  %5 = alloca i32, align 4
  store i32 %4, i32* %5, align 4
  ; Variable buf allocated
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.43, i64 0, i64 0
  %10 = alloca i8*, align 4
  store i8* %9, i8** %10, align 4
  ; Variable test_data allocated
  %11 = load i32, i32* %5, align 4
  %12 = load i8*, i8** %10, align 4
  %13 = call i32 @buffer_write(i32 %11, i32 %12)
  %14 = call i32 @assert_true(i32 %13)
  ; Expression result: %14
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %15
  ; Expression result: 0
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %17
  ; Expression result: 0
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %18
  %19 = load i32, i32* %5, align 4
  %20 = load i8*, i8** %10, align 4
  %21 = call i32 @string_len(i32 %20)
  %22 = call i32 @buffer_read(i32 %19, i32 %21)
  %23 = alloca i32, align 4
  store i32 %22, i32* %23, align 4
  ; Variable read_data allocated
  %24 = load i32, i32* %23, align 4
  %25 = load i8*, i8** %10, align 4
  %26 = call i32 @assert_eq_string(i32 %24, i32 %25)
  ; Expression result: %26
  %27 = load i32, i32* %5, align 4
  %28 = call i32 @buffer_flush(i32 %27)
  %29 = call i32 @assert_true(i32 %28)
  ; Expression result: %29
  %30 = load i32, i32* %5, align 4
  %31 = call i32 @buffer_clear(i32 %30)
  %32 = call i32 @assert_true(i32 %31)
  ; Expression result: %32
  %33 = load i32, i32* %5, align 4
  %34 = call i32 @buffer_size(i32 %33)
  %35 = call i32 @assert_eq_int(i32 %34, i32 0)
  ; Expression result: %35
  ret i32 0
}

define i32 @test_temporary_files() {
entry:
  %0 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.44, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @create_temp_file()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable temp_file allocated
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %4
  ; Expression result: 0
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %5
  %6 = load i32, i32* %3, align 4
  %7 = call i32 @file_exists(i32 %6)
  %8 = call i32 @assert_true(i32 %7)
  ; Expression result: %8
  %9 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.45, i64 0, i64 0
  %10 = alloca i8*, align 4
  store i8* %9, i8** %10, align 4
  ; Variable temp_content allocated
  %11 = load i32, i32* %3, align 4
  %12 = load i8*, i8** %10, align 4
  %13 = call i32 @write_file(i32 %11, i32 %12)
  %14 = call i32 @assert_true(i32 %13)
  ; Expression result: %14
  %15 = load i32, i32* %3, align 4
  %16 = call i32 @read_file(i32 %15)
  %17 = load i8*, i8** %10, align 4
  %18 = call i32 @assert_eq_string(i32 %16, i32 %17)
  ; Expression result: %18
  %19 = call i32 @create_temp_directory()
  %20 = alloca i32, align 4
  store i32 %19, i32* %20, align 4
  ; Variable temp_dir allocated
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %21
  ; Expression result: 0
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %22
  %23 = load i32, i32* %20, align 4
  %24 = call i32 @is_directory(i32 %23)
  %25 = call i32 @assert_true(i32 %24)
  ; Expression result: %25
  %26 = call i32 @temp_directory()
  %27 = alloca i32, align 4
  store i32 %26, i32* %27, align 4
  ; Variable system_temp_dir allocated
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %28
  ; Expression result: 0
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %29
  %30 = load i32, i32* %27, align 4
  %31 = call i32 @is_directory(i32 %30)
  %32 = call i32 @assert_true(i32 %31)
  ; Expression result: %32
  %33 = load i32, i32* %3, align 4
  %34 = call i32 @delete_file(i32 %33)
  %35 = call i32 @assert_true(i32 %34)
  ; Expression result: %35
  %36 = load i32, i32* %20, align 4
  %37 = call i32 @remove_directory(i32 %36)
  %38 = call i32 @assert_true(i32 %37)
  ; Expression result: %38
  ret i32 0
}

define i32 @test_file_timestamps() {
entry:
  %0 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.46, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.47, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable timestamp_file allocated
  %4 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.48, i64 0, i64 0
  %5 = alloca i8*, align 4
  store i8* %4, i8** %5, align 4
  ; Variable test_content allocated
  %6 = load i8*, i8** %3, align 4
  %7 = load i8*, i8** %5, align 4
  %8 = call i32 @write_file(i32 %6, i32 %7)
  %9 = call i32 @assert_true(i32 %8)
  ; Expression result: %9
  %10 = load i8*, i8** %3, align 4
  %11 = call i32 @file_modified_time(i32 %10)
  %12 = alloca i32, align 4
  store i32 %11, i32* %12, align 4
  ; Variable modified_time allocated
  %13 = load i8*, i8** %3, align 4
  %14 = call i32 @file_created_time(i32 %13)
  %15 = alloca i32, align 4
  store i32 %14, i32* %15, align 4
  ; Variable created_time allocated
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %16
  ; Expression result: 0
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %17
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %18
  ; Expression result: 0
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %19
  %20 = load i8*, i8** %3, align 4
  %21 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.49, i64 0, i64 0
  %22 = call i32 @append_file(i32 %20, i32 %21)
  %23 = call i32 @assert_true(i32 %22)
  ; Expression result: %23
  %24 = load i8*, i8** %3, align 4
  %25 = call i32 @file_modified_time(i32 %24)
  %26 = alloca i32, align 4
  store i32 %25, i32* %26, align 4
  ; Variable new_modified_time allocated
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %27
  %28 = load i32, i32* %12, align 4
  ; Expression result: %28
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %29
  %30 = load i8*, i8** %3, align 4
  %31 = call i32 @delete_file(i32 %30)
  %32 = call i32 @assert_true(i32 %31)
  ; Expression result: %32
  ret i32 0
}

define i32 @test_io_edge_cases() {
entry:
  %0 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.50, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.51, i64 0, i64 0
  %3 = call i32 @file_exists(i32 %2)
  %4 = call i32 @assert_false(i32 %3)
  ; Expression result: %4
  %5 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.51, i64 0, i64 0
  %6 = call i32 @delete_file(i32 %5)
  %7 = call i32 @assert_false(i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.52, i64 0, i64 0
  %9 = alloca i8*, align 4
  store i8* %8, i8** %9, align 4
  ; Variable empty_file allocated
  %10 = load i8*, i8** %9, align 4
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  %12 = call i32 @write_file(i32 %10, i32 %11)
  %13 = call i32 @assert_true(i32 %12)
  ; Expression result: %13
  %14 = load i8*, i8** %9, align 4
  %15 = call i32 @file_size(i32 %14)
  %16 = call i32 @assert_eq_int(i32 %15, i32 0)
  ; Expression result: %16
  %17 = load i8*, i8** %9, align 4
  %18 = call i32 @read_file(i32 %17)
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  %20 = call i32 @assert_eq_string(i32 %18, i32 %19)
  ; Expression result: %20
  %21 = load i8*, i8** %9, align 4
  %22 = call i32 @delete_file(i32 %21)
  %23 = call i32 @assert_true(i32 %22)
  ; Expression result: %23
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  %25 = call i32 @path_dirname(i32 %24)
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  %27 = call i32 @assert_eq_string(i32 %25, i32 %26)
  ; Expression result: %27
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  %29 = call i32 @path_basename(i32 %28)
  %30 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  %31 = call i32 @assert_eq_string(i32 %29, i32 %30)
  ; Expression result: %31
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  %33 = call i32 @path_extension(i32 %32)
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  %35 = call i32 @assert_eq_string(i32 %33, i32 %34)
  ; Expression result: %35
  %36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  %37 = call i32 @is_file(i32 %36)
  %38 = call i32 @assert_false(i32 %37)
  ; Expression result: %38
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  %40 = call i32 @is_directory(i32 %39)
  %41 = call i32 @assert_false(i32 %40)
  ; Expression result: %41
  %42 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  %43 = call i32 @path_exists(i32 %42)
  %44 = call i32 @assert_false(i32 %43)
  ; Expression result: %44
  ret i32 0
}

define i32 @test_io_error_handling() {
entry:
  %0 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.53, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.54, i64 0, i64 0
  %3 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.55, i64 0, i64 0
  %4 = call i32 @write_file(i32 %2, i32 %3)
  %5 = alloca i32, align 4
  store i32 %4, i32* %5, align 4
  ; Variable readonly_result allocated
  %6 = call i32 @assert_true(i32 1)
  ; Expression result: %6
  %7 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.56, i64 0, i64 0
  %8 = call i32 @read_file(i32 %7)
  %9 = alloca i32, align 4
  store i32 %8, i32* %9, align 4
  ; Variable nonexistent_content allocated
  %10 = call i32 @assert_true(i32 1)
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  %12 = alloca i8*, align 4
  store i8* %11, i8** %12, align 4
  ; Variable invalid_handle allocated
  %13 = load i8*, i8** %12, align 4
  %14 = call i32 @close_file(i32 %13)
  %15 = call i32 @assert_false(i32 %14)
  ; Expression result: %15
  %16 = load i8*, i8** %12, align 4
  %17 = call i32 @flush_file(i32 %16)
  %18 = call i32 @assert_false(i32 %17)
  ; Expression result: %18
  ret i32 0
}

define i32 @run_all_io_tests() {
entry:
  %0 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.57, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.58, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = call i32 @test_console_io()
  ; Expression result: %6
  %7 = call i32 @test_file_operations()
  ; Expression result: %7
  %8 = call i32 @test_file_copy_move()
  ; Expression result: %8
  %9 = call i32 @test_binary_file_operations()
  ; Expression result: %9
  %10 = call i32 @test_directory_operations()
  ; Expression result: %10
  %11 = call i32 @test_path_operations()
  ; Expression result: %11
  %12 = call i32 @test_stream_io()
  ; Expression result: %12
  %13 = call i32 @test_buffered_io()
  ; Expression result: %13
  %14 = call i32 @test_temporary_files()
  ; Expression result: %14
  %15 = call i32 @test_file_timestamps()
  ; Expression result: %15
  %16 = call i32 @test_io_edge_cases()
  ; Expression result: %16
  %17 = call i32 @test_io_error_handling()
  ; Expression result: %17
  %18 = call i32 @print_test_summary()
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %19
  %20 = call i32 @run_all_tests()
  ; Expression result: %20
  ret i32 0
}



; String constants
@.str.23 = private unnamed_addr constant [4 x i8] c"255\00", align 1
@.str.34 = private unnamed_addr constant [30 x i8] c"/home/user/documents/file.txt\00", align 1
@.str.38 = private unnamed_addr constant [21 x i8] c"nonexistent_path_xyz\00", align 1
@.str.54 = private unnamed_addr constant [24 x i8] c"/root/readonly_test.txt\00", align 1
@.str.18 = private unnamed_addr constant [3 x i8] c"72\00", align 1
@.str.2 = private unnamed_addr constant [22 x i8] c"Test println function\00", align 1
@.str.10 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.17 = private unnamed_addr constant [16 x i8] c"binary_test.bin\00", align 1
@.str.29 = private unnamed_addr constant [16 x i8] c"Path Operations\00", align 1
@.str.30 = private unnamed_addr constant [5 x i8] c"home\00", align 1
@.str.32 = private unnamed_addr constant [10 x i8] c"documents\00", align 1
@.str.48 = private unnamed_addr constant [25 x i8] c"Test file for timestamps\00", align 1
@.str.27 = private unnamed_addr constant [15 x i8] c"test_directory\00", align 1
@.str.46 = private unnamed_addr constant [26 x i8] c"File Timestamp Operations\00", align 1
@.str.53 = private unnamed_addr constant [19 x i8] c"I/O Error Handling\00", align 1
@.str.55 = private unnamed_addr constant [5 x i8] c"test\00", align 1
@.str.12 = private unnamed_addr constant [16 x i8] c"source_test.txt\00", align 1
@.str.31 = private unnamed_addr constant [5 x i8] c"user\00", align 1
@.str.8 = private unnamed_addr constant [24 x i8] c"Hello, CURSED file I/O!\00", align 1
@.str.52 = private unnamed_addr constant [15 x i8] c"empty_test.txt\00", align 1
@.str.24 = private unnamed_addr constant [4 x i8] c"128\00", align 1
@.str.26 = private unnamed_addr constant [21 x i8] c"Directory Operations\00", align 1
@.str.40 = private unnamed_addr constant [16 x i8] c"stream_test.txt\00", align 1
@.str.57 = private unnamed_addr constant [38 x i8] c"💾 Running CURSED I/O Library Tests\00", align 1
@.str.49 = private unnamed_addr constant [10 x i8] c" Modified\00", align 1
@.str.3 = private unnamed_addr constant [21 x i8] c"Test eprint function\00", align 1
@.str.39 = private unnamed_addr constant [22 x i8] c"Stream I/O Operations\00", align 1
@.str.5 = private unnamed_addr constant [27 x i8] c"Test printf: %s, %d, %.2f\0A\00", align 1
@.str.28 = private unnamed_addr constant [2 x i8] c".\00", align 1
@.str.0 = private unnamed_addr constant [22 x i8] c"Console I/O Functions\00", align 1
@.str.9 = private unnamed_addr constant [15 x i8] c" More content!\00", align 1
@.str.25 = private unnamed_addr constant [3 x i8] c"64\00", align 1
@.str.50 = private unnamed_addr constant [15 x i8] c"I/O Edge Cases\00", align 1
@.str.1 = private unnamed_addr constant [20 x i8] c"Test print function\00", align 1
@.str.44 = private unnamed_addr constant [26 x i8] c"Temporary File Operations\00", align 1
@.str.7 = private unnamed_addr constant [14 x i8] c"test_file.txt\00", align 1
@.str.15 = private unnamed_addr constant [27 x i8] c"Test content for copy/move\00", align 1
@.str.43 = private unnamed_addr constant [23 x i8] c"Buffered I/O test data\00", align 1
@.str.36 = private unnamed_addr constant [5 x i8] c".txt\00", align 1
@.str.56 = private unnamed_addr constant [30 x i8] c"definitely_does_not_exist.txt\00", align 1
@.str.13 = private unnamed_addr constant [14 x i8] c"dest_test.txt\00", align 1
@.str.21 = private unnamed_addr constant [4 x i8] c"111\00", align 1
@.str.6 = private unnamed_addr constant [16 x i8] c"File Operations\00", align 1
@.str.22 = private unnamed_addr constant [2 x i8] c"0\00", align 1
@.str.45 = private unnamed_addr constant [23 x i8] c"Temporary file content\00", align 1
@.str.11 = private unnamed_addr constant [26 x i8] c"File Copy/Move Operations\00", align 1
@.str.16 = private unnamed_addr constant [23 x i8] c"Binary File Operations\00", align 1
@.str.58 = private unnamed_addr constant [34 x i8] c"=================================\00", align 1
@.str.47 = private unnamed_addr constant [19 x i8] c"timestamp_test.txt\00", align 1
@.str.14 = private unnamed_addr constant [15 x i8] c"moved_test.txt\00", align 1
@.str.20 = private unnamed_addr constant [4 x i8] c"108\00", align 1
@.str.35 = private unnamed_addr constant [21 x i8] c"/home/user/documents\00", align 1
@.str.19 = private unnamed_addr constant [4 x i8] c"101\00", align 1
@.str.33 = private unnamed_addr constant [9 x i8] c"file.txt\00", align 1
@.str.41 = private unnamed_addr constant [22 x i8] c"Line 1\0ALine 2\0ALine 3\0A\00", align 1
@.str.42 = private unnamed_addr constant [24 x i8] c"Buffered I/O Operations\00", align 1
@.str.51 = private unnamed_addr constant [21 x i8] c"nonexistent_file.txt\00", align 1
@.str.4 = private unnamed_addr constant [23 x i8] c"Test eprintln function\00", align 1
@.str.37 = private unnamed_addr constant [9 x i8] c"test.txt\00", align 1
define i32 @main() {
  %0 = call i32 @delete_file(i32 %binary_file)
  %1 = call i32 @assert_true(i32 %0)
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.10, i64 0, i64 0
  %3 = call i32 @run_all_io_tests()
  ret i32 0
}
