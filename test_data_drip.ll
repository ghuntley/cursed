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

define i8* @test_start(i8* %name) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  ; Expression result: %name
  %1 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %1
  ret i32 0
}

define void @assert_eq_string(i8* %actual, i8* %expected) {
entry:
  %0 = icmp eq i32 %actual, %expected
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %1
  %2 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.1, i64 0, i64 0
  %3 = add i32 %actual, %2
  %4 = add i32 %3, %expected
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %5
  br label %label2
label1:
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %6
  %7 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.2, i64 0, i64 0
  %8 = add i32 %actual, %7
  %9 = add i32 %8, %expected
  ; Expression result: %9
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %10
  br label %label2
label2:
  ret void
}

define void @assert_true(i1 %value) {
entry:
  %0 = icmp eq i1 %value, 1
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.3, i64 0, i64 0
  %2 = call i32 @puts(i8* %1)
  %3 = add i32 0, 0
  ; Expression result: %3
  br label %label2
label1:
  %4 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.4, i64 0, i64 0
  %5 = call i32 @puts(i8* %4)
  %6 = add i32 0, 0
  ; Expression result: %6
  br label %label2
label2:
  ret void
}

define void @assert_eq_int(i32 %actual, i32 %expected) {
entry:
  %0 = icmp eq i32 %actual, %expected
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %3
  %4 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.1, i64 0, i64 0
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %6 = add i32 %4, %5
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %7
  br label %label2
label1:
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %9
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %10
  %11 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.2, i64 0, i64 0
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %13 = add i32 %11, %12
  ; Expression result: %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %14
  br label %label2
label2:
  ret void
}

define i32 @print_test_summary() {
entry:
  %0 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.5, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  ret i32 0
}

define i8* @OpenDB(i8* %driverName, i8* %dataSourceName) {
entry:
  %0 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.6, i64 0, i64 0
  %1 = add i32 %driverName, %0
  %2 = add i32 %1, %dataSourceName
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable connectionString allocated
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %4
  %5 = load i8*, i8** %3, align 4
  ; Expression result: %5
  ret i32 0
}

define i8* @Close(i8* %db) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.7, i64 0, i64 0
  ; Expression result: %1
  ret i32 0
}

define i8* @Ping(i8* %db) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.8, i64 0, i64 0
  ; Expression result: %1
  ret i32 0
}

define i8* @Query(i8* %db, i8* %query, i8* %args) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.9, i64 0, i64 0
  ; Expression result: %1
  ret i32 0
}

define i8* @Exec(i8* %db, i8* %query, i8* %args) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %1
  ret i32 0
}

define i8* @Begin(i8* %db) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.11, i64 0, i64 0
  %2 = add i32 %1, %db
  ; Expression result: %2
  ret i32 0
}

define i8* @Commit(i8* %tx) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.12, i64 0, i64 0
  ; Expression result: %1
  ret i32 0
}

define i8* @Rollback(i8* %tx) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.13, i64 0, i64 0
  ; Expression result: %1
  ret i32 0
}

define i32 @LastInsertId(i8* %result) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  ; Expression result: 123
  ret i32 0
}

define i32 @RowsAffected(i8* %result) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  ; Expression result: 1
  ret i32 0
}



; String constants
@.str.28 = private unnamed_addr constant [15 x i8] c"Last insert ID\00", align 1
@.str.10 = private unnamed_addr constant [33 x i8] c"Query executed, rows affected: 1\00", align 1
@.str.23 = private unnamed_addr constant [29 x i8] c"INSERT INTO users VALUES (?)\00", align 1
@.str.22 = private unnamed_addr constant [20 x i8] c"Statement execution\00", align 1
@.str.31 = private unnamed_addr constant [22 x i8] c"Connection validation\00", align 1
@.str.17 = private unnamed_addr constant [33 x i8] c"postgres://user=test dbname=test\00", align 1
@.str.18 = private unnamed_addr constant [14 x i8] c"Database ping\00", align 1
@.str.11 = private unnamed_addr constant [13 x i8] c"Transaction-\00", align 1
@.str.19 = private unnamed_addr constant [15 x i8] c"Database close\00", align 1
@.str.2 = private unnamed_addr constant [12 x i8] c", expected \00", align 1
@.str.20 = private unnamed_addr constant [16 x i8] c"Query execution\00", align 1
@.str.25 = private unnamed_addr constant [18 x i8] c"Transaction begin\00", align 1
@.str.30 = private unnamed_addr constant [14 x i8] c"Rows affected\00", align 1
@.str.24 = private unnamed_addr constant [6 x i8] c"Alice\00", align 1
@.str.0 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.16 = private unnamed_addr constant [22 x i8] c"user=test dbname=test\00", align 1
@.str.21 = private unnamed_addr constant [20 x i8] c"SELECT * FROM users\00", align 1
@.str.14 = private unnamed_addr constant [20 x i8] c"Database connection\00", align 1
@.str.29 = private unnamed_addr constant [7 x i8] c"result\00", align 1
@.str.7 = private unnamed_addr constant [18 x i8] c"Connection closed\00", align 1
@.str.4 = private unnamed_addr constant [37 x i8] c"  ❌ FAIL: expected true, got false\00", align 1
@.str.8 = private unnamed_addr constant [16 x i8] c"Ping successful\00", align 1
@.str.27 = private unnamed_addr constant [21 x i8] c"Transaction rollback\00", align 1
@.str.26 = private unnamed_addr constant [19 x i8] c"Transaction commit\00", align 1
@.str.1 = private unnamed_addr constant [5 x i8] c" == \00", align 1
@.str.6 = private unnamed_addr constant [4 x i8] c"://\00", align 1
@.str.5 = private unnamed_addr constant [32 x i8] c"🎯 Data Drip tests completed!\00", align 1
@.str.9 = private unnamed_addr constant [18 x i8] c"Query result rows\00", align 1
@.str.13 = private unnamed_addr constant [24 x i8] c"Transaction rolled back\00", align 1
@.str.3 = private unnamed_addr constant [26 x i8] c"  ✅ PASS: value is true\00", align 1
@.str.12 = private unnamed_addr constant [22 x i8] c"Transaction committed\00", align 1
@.str.15 = private unnamed_addr constant [9 x i8] c"postgres\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.14, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  %2 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.15, i64 0, i64 0
  %3 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.16, i64 0, i64 0
  %4 = call i32 @OpenDB(i32 %2, i32 %3)
  %5 = alloca i8*, align 4
  store i8* %4, i8** %5, align 4
  ; Variable db allocated at %5
  %6 = load i32, i32* %5, align 4
  %7 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.17, i64 0, i64 0
  %8 = call i32 @assert_eq_string(i32 %6, i32 %7)
  %9 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.18, i64 0, i64 0
  %10 = call i32 @test_start(i32 %9)
  %11 = load i32, i32* %5, align 4
  %12 = call i32 @Ping(i32 %11)
  %13 = alloca i8*, align 4
  store i8* %12, i8** %13, align 4
  ; Variable pingResult allocated at %13
  %14 = load i32, i32* %13, align 4
  %15 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.8, i64 0, i64 0
  %16 = call i32 @assert_eq_string(i32 %14, i32 %15)
  %17 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.19, i64 0, i64 0
  %18 = call i32 @test_start(i32 %17)
  %19 = load i32, i32* %5, align 4
  %20 = call i32 @Close(i32 %19)
  %21 = alloca i8*, align 4
  store i8* %20, i8** %21, align 4
  ; Variable closeResult allocated at %21
  %22 = load i32, i32* %21, align 4
  %23 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.7, i64 0, i64 0
  %24 = call i32 @assert_eq_string(i32 %22, i32 %23)
  %25 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.20, i64 0, i64 0
  %26 = call i32 @test_start(i32 %25)
  %27 = load i32, i32* %5, align 4
  %28 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.21, i64 0, i64 0
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %30 = call i32 @Query(i32 %27, i32 %28, i32 %29)
  %31 = alloca i8*, align 4
  store i8* %30, i8** %31, align 4
  ; Variable queryResult allocated at %31
  %32 = load i32, i32* %31, align 4
  %33 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.9, i64 0, i64 0
  %34 = call i32 @assert_eq_string(i32 %32, i32 %33)
  %35 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.22, i64 0, i64 0
  %36 = call i32 @test_start(i32 %35)
  %37 = load i32, i32* %5, align 4
  %38 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.23, i64 0, i64 0
  %39 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.24, i64 0, i64 0
  %40 = call i32 @Exec(i32 %37, i32 %38, i32 %39)
  %41 = alloca i8*, align 4
  store i8* %40, i8** %41, align 4
  ; Variable execResult allocated at %41
  %42 = load i32, i32* %41, align 4
  %43 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.10, i64 0, i64 0
  %44 = call i32 @assert_eq_string(i32 %42, i32 %43)
  %45 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.25, i64 0, i64 0
  %46 = call i32 @test_start(i32 %45)
  %47 = load i32, i32* %5, align 4
  %48 = call i32 @Begin(i32 %47)
  %49 = alloca i8*, align 4
  store i8* %48, i8** %49, align 4
  ; Variable tx allocated at %49
  %50 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %51 = load i32, i32* %5, align 4
  %52 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %53 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.26, i64 0, i64 0
  %54 = call i32 @test_start(i32 %53)
  %55 = load i32, i32* %49, align 4
  %56 = call i32 @Commit(i32 %55)
  %57 = alloca i8*, align 4
  store i8* %56, i8** %57, align 4
  ; Variable commitResult allocated at %57
  %58 = load i32, i32* %57, align 4
  %59 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.12, i64 0, i64 0
  %60 = call i32 @assert_eq_string(i32 %58, i32 %59)
  %61 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.27, i64 0, i64 0
  %62 = call i32 @test_start(i32 %61)
  %63 = load i32, i32* %49, align 4
  %64 = call i32 @Rollback(i32 %63)
  %65 = alloca i8*, align 4
  store i8* %64, i8** %65, align 4
  ; Variable rollbackResult allocated at %65
  %66 = load i32, i32* %65, align 4
  %67 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.13, i64 0, i64 0
  %68 = call i32 @assert_eq_string(i32 %66, i32 %67)
  %69 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.28, i64 0, i64 0
  %70 = call i32 @test_start(i32 %69)
  %71 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.29, i64 0, i64 0
  %72 = call i32 @LastInsertId(i32 %71)
  %73 = alloca i32, align 4
  store i32 %72, i32* %73, align 4
  ; Variable lastId allocated at %73
  %74 = load i32, i32* %73, align 4
  %75 = call i32 @assert_eq_int(i32 %74, i32 123)
  %76 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.30, i64 0, i64 0
  %77 = call i32 @test_start(i32 %76)
  %78 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.29, i64 0, i64 0
  %79 = call i32 @RowsAffected(i32 %78)
  %80 = alloca i32, align 4
  store i32 %79, i32* %80, align 4
  ; Variable rowsAff allocated at %80
  %81 = load i32, i32* %80, align 4
  %82 = call i32 @assert_eq_int(i32 %81, i32 1)
  %83 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.31, i64 0, i64 0
  %84 = call i32 @test_start(i32 %83)
  %85 = call i32 @assert_true(i32 1)
  %86 = call i32 @print_test_summary()
  ret i32 0
}
