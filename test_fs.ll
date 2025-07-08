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
define i32 @setup_test_env() {
entry:
  %0 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @fs_create_dir(i32 %0)
  ; Expression result: %1
  ret i32 0
}

define i32 @cleanup_test_env() {
entry:
  %0 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.1, i64 0, i64 0
  %1 = call i32 @fs_delete_file(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.2, i64 0, i64 0
  %3 = call i32 @fs_delete_file(i32 %2)
  ; Expression result: %3
  %4 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.3, i64 0, i64 0
  %5 = call i32 @fs_delete_file(i32 %4)
  ; Expression result: %5
  %6 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.0, i64 0, i64 0
  %7 = call i32 @fs_remove_dir(i32 %6)
  ; Expression result: %7
  ret i32 0
}

define i32 @test_file_operations() {
entry:
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.4, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @setup_test_env()
  ; Expression result: %2
  %3 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.1, i64 0, i64 0
  %4 = alloca i8*, align 4
  store i8* %3, i8** %4, align 4
  ; Variable test_path allocated
  %5 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.5, i64 0, i64 0
  %6 = alloca i8*, align 4
  store i8* %5, i8** %6, align 4
  ; Variable test_content allocated
  %7 = load i8*, i8** %4, align 4
  %8 = load i8*, i8** %6, align 4
  %9 = call i32 @fs_write_file(i32 %7, i32 %8)
  %10 = alloca i32, align 4
  store i32 %9, i32* %10, align 4
  ; Variable write_success allocated
  %11 = load i32, i32* %10, align 4
  %12 = call i32 @assert_true(i32 %11)
  ; Expression result: %12
  %13 = load i8*, i8** %4, align 4
  %14 = call i32 @fs_file_exists(i32 %13)
  %15 = alloca i32, align 4
  store i32 %14, i32* %15, align 4
  ; Variable exists allocated
  %16 = load i32, i32* %15, align 4
  %17 = call i32 @assert_true(i32 %16)
  ; Expression result: %17
  %18 = load i8*, i8** %4, align 4
  %19 = call i32 @fs_read_file(i32 %18)
  %20 = alloca i32, align 4
  store i32 %19, i32* %20, align 4
  ; Variable read_content allocated
  %21 = load i32, i32* %20, align 4
  %22 = load i8*, i8** %6, align 4
  %23 = call i32 @assert_eq_string(i32 %21, i32 %22)
  ; Expression result: %23
  %24 = add i32 0, 0 ; literal placeholder
  %25 = alloca i8*, align 4
  store i8* %24, i8** %25, align 4
  ; Variable file_size allocated
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %26
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %27
  %28 = load i8*, i8** %4, align 4
  %29 = call i32 @fs_get_file_size(i32 %28)
  ; Expression result: %29
  %30 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %30
  ; Expression result: 0
  %31 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %31
  %32 = load i8*, i8** %4, align 4
  %33 = call i32 @fs_delete_file(i32 %32)
  %34 = alloca i32, align 4
  store i32 %33, i32* %34, align 4
  ; Variable delete_success allocated
  %35 = load i32, i32* %34, align 4
  %36 = call i32 @assert_true(i32 %35)
  ; Expression result: %36
  %37 = load i8*, i8** %4, align 4
  %38 = call i32 @fs_file_exists(i32 %37)
  %39 = alloca i32, align 4
  store i32 %38, i32* %39, align 4
  ; Variable exists_after_delete allocated
  %40 = load i32, i32* %39, align 4
  %41 = call i32 @assert_false(i32 %40)
  ; Expression result: %41
  %42 = call i32 @cleanup_test_env()
  ; Expression result: %42
  ret i32 0
}

define i32 @test_directory_operations() {
entry:
  %0 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.7, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable test_dir allocated
  %4 = load i8*, i8** %3, align 4
  %5 = call i32 @fs_create_dir(i32 %4)
  %6 = alloca i32, align 4
  store i32 %5, i32* %6, align 4
  ; Variable create_success allocated
  %7 = load i32, i32* %6, align 4
  %8 = call i32 @assert_true(i32 %7)
  ; Expression result: %8
  %9 = load i8*, i8** %3, align 4
  %10 = call i32 @fs_file_exists(i32 %9)
  %11 = alloca i32, align 4
  store i32 %10, i32* %11, align 4
  ; Variable dir_exists allocated
  %12 = load i32, i32* %11, align 4
  %13 = call i32 @assert_true(i32 %12)
  ; Expression result: %13
  %14 = load i8*, i8** %3, align 4
  %15 = call i32 @fs_is_dir(i32 %14)
  %16 = alloca i32, align 4
  store i32 %15, i32* %16, align 4
  ; Variable is_directory allocated
  %17 = load i32, i32* %16, align 4
  %18 = call i32 @assert_true(i32 %17)
  ; Expression result: %18
  %19 = add i32 0, 0 ; literal placeholder
  %20 = alloca i8*, align 4
  store i8* %19, i8** %20, align 4
  ; Variable files allocated
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %21
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %22
  %23 = load i8*, i8** %3, align 4
  %24 = call i32 @fs_list_dir(i32 %23)
  ; Expression result: %24
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %25
  ; Expression result: 0
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %26
  %27 = load i8*, i8** %3, align 4
  %28 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.9, i64 0, i64 0
  %29 = add i32 %27, %28
  %30 = alloca i8*, align 4
  store i8* %29, i8** %30, align 4
  ; Variable file_in_dir allocated
  %31 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.10, i64 0, i64 0
  %32 = alloca i8*, align 4
  store i8* %31, i8** %32, align 4
  ; Variable file_content allocated
  %33 = load i8*, i8** %30, align 4
  %34 = load i8*, i8** %32, align 4
  %35 = call i32 @fs_write_file(i32 %33, i32 %34)
  %36 = alloca i32, align 4
  store i32 %35, i32* %36, align 4
  ; Variable write_success allocated
  %37 = load i32, i32* %36, align 4
  %38 = call i32 @assert_true(i32 %37)
  ; Expression result: %38
  %39 = add i32 0, 0 ; literal placeholder
  %40 = alloca i8*, align 4
  store i8* %39, i8** %40, align 4
  ; Variable files_after allocated
  %41 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %41
  %42 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %42
  %43 = load i8*, i8** %3, align 4
  %44 = call i32 @fs_list_dir(i32 %43)
  ; Expression result: %44
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %45
  ; Expression result: 1
  %46 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %46
  %47 = load i8*, i8** %30, align 4
  %48 = call i32 @fs_is_file(i32 %47)
  %49 = alloca i32, align 4
  store i32 %48, i32* %49, align 4
  ; Variable is_file allocated
  %50 = load i32, i32* %49, align 4
  %51 = call i32 @assert_true(i32 %50)
  ; Expression result: %51
  %52 = load i8*, i8** %30, align 4
  %53 = call i32 @fs_delete_file(i32 %52)
  ; Expression result: %53
  %54 = load i8*, i8** %3, align 4
  %55 = call i32 @fs_remove_dir(i32 %54)
  ; Expression result: %55
  ret i32 0
}

define i32 @test_path_utilities() {
entry:
  %0 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.11, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.12, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable base allocated
  %4 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.13, i64 0, i64 0
  %5 = alloca i8*, align 4
  store i8* %4, i8** %5, align 4
  ; Variable component allocated
  %6 = load i8*, i8** %3, align 4
  %7 = load i8*, i8** %5, align 4
  %8 = call i32 @fs_join_path(i32 %6, i32 %7)
  %9 = alloca i32, align 4
  store i32 %8, i32* %9, align 4
  ; Variable joined allocated
  %10 = load i32, i32* %9, align 4
  %11 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.14, i64 0, i64 0
  %12 = call i32 @assert_eq_string(i32 %10, i32 %11)
  ; Expression result: %12
  %13 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.15, i64 0, i64 0
  %14 = alloca i8*, align 4
  store i8* %13, i8** %14, align 4
  ; Variable base_with_slash allocated
  %15 = load i8*, i8** %14, align 4
  %16 = load i8*, i8** %5, align 4
  %17 = call i32 @fs_join_path(i32 %15, i32 %16)
  %18 = alloca i32, align 4
  store i32 %17, i32* %18, align 4
  ; Variable joined_with_slash allocated
  %19 = load i32, i32* %18, align 4
  %20 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.14, i64 0, i64 0
  %21 = call i32 @assert_eq_string(i32 %19, i32 %20)
  ; Expression result: %21
  %22 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.16, i64 0, i64 0
  %23 = alloca i8*, align 4
  store i8* %22, i8** %23, align 4
  ; Variable filename allocated
  %24 = load i8*, i8** %23, align 4
  %25 = call i32 @fs_get_extension(i32 %24)
  %26 = alloca i32, align 4
  store i32 %25, i32* %26, align 4
  ; Variable extension allocated
  %27 = load i32, i32* %26, align 4
  %28 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.17, i64 0, i64 0
  %29 = call i32 @assert_eq_string(i32 %27, i32 %28)
  ; Expression result: %29
  %30 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.18, i64 0, i64 0
  %31 = alloca i8*, align 4
  store i8* %30, i8** %31, align 4
  ; Variable full_path allocated
  %32 = load i8*, i8** %31, align 4
  %33 = call i32 @fs_get_basename(i32 %32)
  %34 = alloca i32, align 4
  store i32 %33, i32* %34, align 4
  ; Variable basename allocated
  %35 = load i32, i32* %34, align 4
  %36 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.16, i64 0, i64 0
  %37 = call i32 @assert_eq_string(i8* %35, i32 %36)
  ; Expression result: %37
  %38 = load i8*, i8** %31, align 4
  %39 = call i32 @fs_get_extension(i32 %38)
  %40 = alloca i32, align 4
  store i32 %39, i32* %40, align 4
  ; Variable path_extension allocated
  %41 = load i32, i32* %40, align 4
  %42 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.17, i64 0, i64 0
  %43 = call i32 @assert_eq_string(i32 %41, i32 %42)
  ; Expression result: %43
  %44 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.19, i64 0, i64 0
  %45 = alloca i8*, align 4
  store i8* %44, i8** %45, align 4
  ; Variable no_ext allocated
  %46 = load i8*, i8** %45, align 4
  %47 = call i32 @fs_get_extension(i32 %46)
  %48 = alloca i32, align 4
  store i32 %47, i32* %48, align 4
  ; Variable no_ext_result allocated
  %49 = load i32, i32* %48, align 4
  %50 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %51 = call i32 @assert_eq_string(i32 %49, i32 %50)
  ; Expression result: %51
  ret i32 0
}

define i32 @test_file_info() {
entry:
  %0 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.20, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.21, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable test_path allocated
  %4 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.22, i64 0, i64 0
  %5 = alloca i8*, align 4
  store i8* %4, i8** %5, align 4
  ; Variable test_content allocated
  %6 = load i8*, i8** %3, align 4
  %7 = load i8*, i8** %5, align 4
  %8 = call i32 @fs_write_file(i32 %6, i32 %7)
  %9 = alloca i32, align 4
  store i32 %8, i32* %9, align 4
  ; Variable write_success allocated
  %10 = load i32, i32* %9, align 4
  %11 = call i32 @assert_true(i32 %10)
  ; Expression result: %11
  %12 = add i32 0, 0 ; literal placeholder
  %13 = alloca i8*, align 4
  store i8* %12, i8** %13, align 4
  ; Variable info allocated
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %14
  ; Expression result: %FileInfo
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %15
  %16 = load i8*, i8** %3, align 4
  %17 = call i32 @fs_get_file_info(i32 %16)
  ; Expression result: %17
  %18 = load i8*, i8** %13, align 4
  ; Member access: %18.name
  %19 = getelementptr inbounds %struct.object, %struct.object* %18, i32 0, i32 0
  %20 = load i32, i32* %19, align 4
  %21 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.21, i64 0, i64 0
  %22 = call i32 @assert_eq_string(i32 %20, i32 %21)
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %23
  ; Expression result: 0
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %24
  %25 = load i8*, i8** %13, align 4
  ; Member access: %25.is_dir
  %26 = getelementptr inbounds %struct.object, %struct.object* %25, i32 0, i32 0
  %27 = load i32, i32* %26, align 4
  %28 = call i32 @assert_false(i32 %27)
  ; Expression result: %28
  %29 = load i8*, i8** %3, align 4
  %30 = call i32 @fs_get_permissions(i32 %29)
  %31 = alloca i32, align 4
  store i32 %30, i32* %31, align 4
  ; Variable perms allocated
  %32 = load i32, i32* %31, align 4
  %33 = call i32 @assert_eq_int(i32 %32, i32 644)
  ; Expression result: %33
  %34 = load i8*, i8** %3, align 4
  %35 = call i32 @fs_set_permissions(i32 %34, i32 755)
  %36 = alloca i32, align 4
  store i32 %35, i32* %36, align 4
  ; Variable set_perms_success allocated
  %37 = load i32, i32* %36, align 4
  %38 = call i32 @assert_true(i32 %37)
  ; Expression result: %38
  %39 = load i8*, i8** %3, align 4
  %40 = call i32 @fs_delete_file(i32 %39)
  ; Expression result: %40
  ret i32 0
}

define i8* @test_error_handling() {
entry:
  %0 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.23, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.24, i64 0, i64 0
  %3 = call i32 @fs_read_file(i32 %2)
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable nonexistent_content allocated
  %5 = load i32, i32* %4, align 4
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %7 = call i32 @assert_eq_string(i32 %5, i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.24, i64 0, i64 0
  %9 = call i32 @fs_file_exists(i32 %8)
  %10 = alloca i32, align 4
  store i32 %9, i32* %10, align 4
  ; Variable nonexistent_exists allocated
  %11 = load i32, i32* %10, align 4
  %12 = call i32 @assert_false(i32 %11)
  ; Expression result: %12
  %13 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.24, i64 0, i64 0
  %14 = call i32 @fs_delete_file(i32 %13)
  %15 = alloca i32, align 4
  store i32 %14, i32* %15, align 4
  ; Variable delete_nonexistent allocated
  %16 = load i32, i32* %15, align 4
  %17 = call i32 @assert_false(i32 %16)
  ; Expression result: %17
  %18 = add i32 0, 0 ; literal placeholder
  %19 = alloca i8*, align 4
  store i8* %18, i8** %19, align 4
  ; Variable nonexistent_files allocated
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %21
  %22 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.25, i64 0, i64 0
  %23 = call i32 @fs_list_dir(i32 %22)
  ; Expression result: %23
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %24
  ; Expression result: 0
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %25
  %26 = add i32 0, 0 ; literal placeholder
  %27 = alloca i8*, align 4
  store i8* %26, i8** %27, align 4
  ; Variable nonexistent_size allocated
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %28
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %29
  %30 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.24, i64 0, i64 0
  %31 = call i32 @fs_get_file_size(i32 %30)
  ; Expression result: %31
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %33 = sub i32 %32, 1
  ; Expression result: %33
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %34
  ret i32 0
}

define i32 @test_large_files() {
entry:
  %0 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.26, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable large_content allocated
  %5 = load i32, i32* %4, align 4
  %6 = icmp slt i32 %5, 1000
  %16 = add i32 1, 0 ; increment placeholder
  %4 = alloca i32, align 4
  store i32 0, i32* %4, align 4
  ; Short declaration: i := 0
  br label %label0
label0:
  br i1 %6, label %label1, label %label3
label1:
  %7 = load i8*, i8** %3, align 4
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %8
  %9 = load i8*, i8** %3, align 4
  %10 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.27, i64 0, i64 0
  %11 = add i32 %9, %10
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  %13 = add i32 %11, %12
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %14
  %15 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.28, i64 0, i64 0
  ; Expression result: %15
  br label %label2
label2:
  br label %label0
label3:
  %17 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.29, i64 0, i64 0
  %18 = alloca i8*, align 4
  store i8* %17, i8** %18, align 4
  ; Variable large_file_path allocated
  %19 = load i8*, i8** %18, align 4
  %20 = load i8*, i8** %3, align 4
  %21 = call i32 @fs_write_file(i32 %19, i32 %20)
  %22 = alloca i32, align 4
  store i32 %21, i32* %22, align 4
  ; Variable write_large_success allocated
  %23 = load i32, i32* %22, align 4
  %24 = call i32 @assert_true(i32 %23)
  ; Expression result: %24
  %25 = load i8*, i8** %18, align 4
  %26 = call i32 @fs_read_file(i32 %25)
  %27 = alloca i32, align 4
  store i32 %26, i32* %27, align 4
  ; Variable read_large_content allocated
  %28 = load i32, i32* %27, align 4
  %29 = load i8*, i8** %3, align 4
  %30 = call i32 @assert_eq_string(i32 %28, i32 %29)
  ; Expression result: %30
  %31 = add i32 0, 0 ; literal placeholder
  %32 = alloca i8*, align 4
  store i8* %31, i8** %32, align 4
  ; Variable large_file_size allocated
  %33 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %33
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %34
  %35 = load i8*, i8** %18, align 4
  %36 = call i32 @fs_get_file_size(i32 %35)
  ; Expression result: %36
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %37
  ; Expression result: 10000
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %38
  %39 = load i8*, i8** %18, align 4
  %40 = call i32 @fs_delete_file(i32 %39)
  ; Expression result: %40
  ret i32 0
}

define i32 @test_recursive_directory_creation() {
entry:
  %0 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.30, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.31, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable nested_path allocated
  %4 = load i8*, i8** %3, align 4
  %5 = call i32 @fs_create_dir_recursive(i32 %4)
  %6 = alloca i32, align 4
  store i32 %5, i32* %6, align 4
  ; Variable create_recursive_success allocated
  %7 = load i32, i32* %6, align 4
  %8 = call i32 @assert_true(i32 %7)
  ; Expression result: %8
  %9 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.32, i64 0, i64 0
  %10 = call i32 @fs_file_exists(i32 %9)
  %11 = call i32 @assert_true(i32 %10)
  ; Expression result: %11
  %12 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.33, i64 0, i64 0
  %13 = call i32 @fs_file_exists(i32 %12)
  %14 = call i32 @assert_true(i32 %13)
  ; Expression result: %14
  %15 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.31, i64 0, i64 0
  %16 = call i32 @fs_file_exists(i32 %15)
  %17 = call i32 @assert_true(i32 %16)
  ; Expression result: %17
  %18 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.34, i64 0, i64 0
  %19 = alloca i8*, align 4
  store i8* %18, i8** %19, align 4
  ; Variable nested_file allocated
  %20 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.35, i64 0, i64 0
  %21 = alloca i8*, align 4
  store i8* %20, i8** %21, align 4
  ; Variable nested_content allocated
  %22 = load i8*, i8** %19, align 4
  %23 = load i8*, i8** %21, align 4
  %24 = call i32 @fs_write_file(i32 %22, i32 %23)
  %25 = alloca i32, align 4
  store i32 %24, i32* %25, align 4
  ; Variable write_nested_success allocated
  %26 = load i32, i32* %25, align 4
  %27 = call i32 @assert_true(i32 %26)
  ; Expression result: %27
  %28 = load i8*, i8** %19, align 4
  %29 = call i32 @fs_read_file(i32 %28)
  %30 = alloca i32, align 4
  store i32 %29, i32* %30, align 4
  ; Variable read_nested_content allocated
  %31 = load i32, i32* %30, align 4
  %32 = load i8*, i8** %21, align 4
  %33 = call i32 @assert_eq_string(i32 %31, i32 %32)
  ; Expression result: %33
  %34 = load i8*, i8** %19, align 4
  %35 = call i32 @fs_delete_file(i32 %34)
  ; Expression result: %35
  %36 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.31, i64 0, i64 0
  %37 = call i32 @fs_remove_dir(i32 %36)
  ; Expression result: %37
  %38 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.33, i64 0, i64 0
  %39 = call i32 @fs_remove_dir(i32 %38)
  ; Expression result: %39
  %40 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.32, i64 0, i64 0
  %41 = call i32 @fs_remove_dir(i32 %40)
  ; Expression result: %41
  ret i32 0
}



; String constants
@.str.3 = private unnamed_addr constant [30 x i8] c"test_fs_temp/test_content.txt\00", align 1
@.str.2 = private unnamed_addr constant [28 x i8] c"test_fs_temp/test_write.txt\00", align 1
@.str.25 = private unnamed_addr constant [16 x i8] c"nonexistent_dir\00", align 1
@.str.9 = private unnamed_addr constant [15 x i8] c"/test_file.txt\00", align 1
@.str.14 = private unnamed_addr constant [21 x i8] c"/home/user/documents\00", align 1
@.str.1 = private unnamed_addr constant [27 x i8] c"test_fs_temp/test_file.txt\00", align 1
@.str.19 = private unnamed_addr constant [7 x i8] c"README\00", align 1
@.str.33 = private unnamed_addr constant [14 x i8] c"level1/level2\00", align 1
@.str.11 = private unnamed_addr constant [15 x i8] c"Path Utilities\00", align 1
@.str.22 = private unnamed_addr constant [19 x i8] c"Test file for info\00", align 1
@.str.29 = private unnamed_addr constant [20 x i8] c"large_test_file.txt\00", align 1
@.str.23 = private unnamed_addr constant [15 x i8] c"Error Handling\00", align 1
@.str.32 = private unnamed_addr constant [7 x i8] c"level1\00", align 1
@.str.13 = private unnamed_addr constant [10 x i8] c"documents\00", align 1
@.str.24 = private unnamed_addr constant [21 x i8] c"nonexistent_file.txt\00", align 1
@.str.16 = private unnamed_addr constant [13 x i8] c"document.txt\00", align 1
@.str.18 = private unnamed_addr constant [24 x i8] c"/home/user/document.txt\00", align 1
@.str.21 = private unnamed_addr constant [19 x i8] c"test_info_file.txt\00", align 1
@.str.28 = private unnamed_addr constant [24 x i8] c" of large file content\0A\00", align 1
@.str.0 = private unnamed_addr constant [13 x i8] c"test_fs_temp\00", align 1
@.str.5 = private unnamed_addr constant [26 x i8] c"Hello, CURSED filesystem!\00", align 1
@.str.8 = private unnamed_addr constant [12 x i8] c"test_fs_dir\00", align 1
@.str.6 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.27 = private unnamed_addr constant [6 x i8] c"Line \00", align 1
@.str.31 = private unnamed_addr constant [21 x i8] c"level1/level2/level3\00", align 1
@.str.7 = private unnamed_addr constant [21 x i8] c"Directory Operations\00", align 1
@.str.34 = private unnamed_addr constant [37 x i8] c"level1/level2/level3/nested_file.txt\00", align 1
@.str.35 = private unnamed_addr constant [25 x i8] c"File in nested directory\00", align 1
@.str.15 = private unnamed_addr constant [12 x i8] c"/home/user/\00", align 1
@.str.4 = private unnamed_addr constant [16 x i8] c"File Operations\00", align 1
@.str.26 = private unnamed_addr constant [22 x i8] c"Large File Operations\00", align 1
@.str.12 = private unnamed_addr constant [11 x i8] c"/home/user\00", align 1
@.str.10 = private unnamed_addr constant [18 x i8] c"File in directory\00", align 1
@.str.17 = private unnamed_addr constant [5 x i8] c".txt\00", align 1
@.str.20 = private unnamed_addr constant [17 x i8] c"File Information\00", align 1
@.str.30 = private unnamed_addr constant [29 x i8] c"Recursive Directory Creation\00", align 1
define i32 @main() {
  %0 = call i32 @test_file_operations()
  %1 = call i32 @test_directory_operations()
  %2 = call i32 @test_path_utilities()
  %3 = call i32 @test_file_info()
  %4 = call i32 @test_error_handling()
  %5 = call i32 @test_large_files()
  %6 = call i32 @test_recursive_directory_creation()
  %7 = call i32 @print_test_summary()
  ret i32 0
}
