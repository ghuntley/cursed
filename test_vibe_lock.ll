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
define i32 @test_mutex_basic() {
entry:
  %0 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = alloca i32, align 4
  store i32 42, i32* %3, align 4
  ; Variable mu allocated
  %4 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.1, i64 0, i64 0
  %5 = call i32 @puts(i8* %4)
  %6 = add i32 0, 0
  ; Expression result: %6
  %7 = alloca i1, align 4
  store i1 1, i1* %7, align 4
  ; Variable lock_result allocated
  %8 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.2, i64 0, i64 0
  %9 = call i32 @puts(i8* %8)
  %10 = add i32 0, 0
  ; Expression result: %10
  %11 = alloca i1, align 4
  store i1 1, i1* %11, align 4
  ; Variable unlock_result allocated
  %12 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.3, i64 0, i64 0
  %13 = call i32 @puts(i8* %12)
  %14 = add i32 0, 0
  ; Expression result: %14
  %15 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.4, i64 0, i64 0
  %16 = call i32 @puts(i8* %15)
  %17 = add i32 0, 0
  ; Expression result: %17
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  ; Expression result: %18
  ; Expression result: 0
  ret i32 0
}

define i32 @test_mutex_try_lock() {
entry:
  %0 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.6, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = alloca i1, align 4
  store i1 1, i1* %3, align 4
  ; Variable try_result allocated
  %4 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.7, i64 0, i64 0
  %5 = call i32 @puts(i8* %4)
  %6 = add i32 0, 0
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  %8 = alloca i8*, align 4
  store i8* %7, i8** %8, align 4
  ; Variable try_again allocated
  %9 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.8, i64 0, i64 0
  %10 = call i32 @puts(i8* %9)
  %11 = add i32 0, 0
  ; Expression result: %11
  %12 = alloca i1, align 4
  store i1 1, i1* %12, align 4
  ; Variable try_final allocated
  %13 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.9, i64 0, i64 0
  %14 = call i32 @puts(i8* %13)
  %15 = add i32 0, 0
  ; Expression result: %15
  %16 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.10, i64 0, i64 0
  %17 = call i32 @puts(i8* %16)
  %18 = add i32 0, 0
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  ; Expression result: %19
  ; Expression result: 0
  ret i32 0
}

define i32 @test_rwlock_basic() {
entry:
  %0 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.11, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = alloca i32, align 4
  store i32 100, i32* %3, align 4
  ; Variable rw allocated
  %4 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.12, i64 0, i64 0
  %5 = call i32 @puts(i8* %4)
  %6 = add i32 0, 0
  ; Expression result: %6
  %7 = alloca i1, align 4
  store i1 1, i1* %7, align 4
  ; Variable rlock_result allocated
  %8 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.13, i64 0, i64 0
  %9 = call i32 @puts(i8* %8)
  %10 = add i32 0, 0
  ; Expression result: %10
  %11 = alloca i1, align 4
  store i1 1, i1* %11, align 4
  ; Variable runlock_result allocated
  %12 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.14, i64 0, i64 0
  %13 = call i32 @puts(i8* %12)
  %14 = add i32 0, 0
  ; Expression result: %14
  %15 = alloca i1, align 4
  store i1 1, i1* %15, align 4
  ; Variable wlock_result allocated
  %16 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.15, i64 0, i64 0
  %17 = call i32 @puts(i8* %16)
  %18 = add i32 0, 0
  ; Expression result: %18
  %19 = alloca i1, align 4
  store i1 1, i1* %19, align 4
  ; Variable wunlock_result allocated
  %20 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.16, i64 0, i64 0
  %21 = call i32 @puts(i8* %20)
  %22 = add i32 0, 0
  ; Expression result: %22
  %23 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.17, i64 0, i64 0
  %24 = call i32 @puts(i8* %23)
  %25 = add i32 0, 0
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  ; Expression result: %26
  ; Expression result: 0
  ret i32 0
}

define i32 @test_semaphore_basic() {
entry:
  %0 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.18, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = alloca i32, align 4
  store i32 3, i32* %3, align 4
  ; Variable sem allocated
  %4 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.19, i64 0, i64 0
  %5 = call i32 @puts(i8* %4)
  %6 = add i32 0, 0
  ; Expression result: %6
  %7 = alloca i1, align 4
  store i1 1, i1* %7, align 4
  ; Variable acquire1 allocated
  %8 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.20, i64 0, i64 0
  %9 = call i32 @puts(i8* %8)
  %10 = add i32 0, 0
  ; Expression result: %10
  %11 = alloca i1, align 4
  store i1 1, i1* %11, align 4
  ; Variable acquire2 allocated
  %12 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.21, i64 0, i64 0
  %13 = call i32 @puts(i8* %12)
  %14 = add i32 0, 0
  ; Expression result: %14
  %15 = alloca i1, align 4
  store i1 1, i1* %15, align 4
  ; Variable acquire3 allocated
  %16 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.22, i64 0, i64 0
  %17 = call i32 @puts(i8* %16)
  %18 = add i32 0, 0
  ; Expression result: %18
  %19 = alloca i1, align 4
  store i1 1, i1* %19, align 4
  ; Variable release1 allocated
  %20 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.23, i64 0, i64 0
  %21 = call i32 @puts(i8* %20)
  %22 = add i32 0, 0
  ; Expression result: %22
  %23 = alloca i1, align 4
  store i1 1, i1* %23, align 4
  ; Variable acquire4 allocated
  %24 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.24, i64 0, i64 0
  %25 = call i32 @puts(i8* %24)
  %26 = add i32 0, 0
  ; Expression result: %26
  %27 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.25, i64 0, i64 0
  %28 = call i32 @puts(i8* %27)
  %29 = add i32 0, 0
  ; Expression result: %29
  %30 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  ; Expression result: %30
  ; Expression result: 0
  ret i32 0
}

define i32 @test_once_basic() {
entry:
  %0 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.26, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = alloca i32, align 4
  store i32 0, i32* %3, align 4
  ; Variable once_obj allocated
  %4 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.27, i64 0, i64 0
  %5 = call i32 @puts(i8* %4)
  %6 = add i32 0, 0
  ; Expression result: %6
  %7 = alloca i1, align 4
  store i1 1, i1* %7, align 4
  ; Variable result1 allocated
  %8 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.28, i64 0, i64 0
  %9 = call i32 @puts(i8* %8)
  %10 = add i32 0, 0
  ; Expression result: %10
  %11 = alloca i1, align 4
  store i1 1, i1* %11, align 4
  ; Variable result2 allocated
  %12 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.29, i64 0, i64 0
  %13 = call i32 @puts(i8* %12)
  %14 = add i32 0, 0
  ; Expression result: %14
  %15 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.30, i64 0, i64 0
  %16 = call i32 @puts(i8* %15)
  %17 = add i32 0, 0
  ; Expression result: %17
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  ; Expression result: %18
  ; Expression result: 0
  ret i32 0
}

define i32 @test_atomic_operations() {
entry:
  %0 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.31, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = alloca i32, align 4
  store i32 10, i32* %3, align 4
  ; Variable val allocated
  %4 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.32, i64 0, i64 0
  %5 = call i32 @puts(i8* %4)
  %6 = add i32 0, 0
  ; Expression result: %6
  %7 = alloca i32, align 4
  store i32 10, i32* %7, align 4
  ; Variable loaded allocated
  %8 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.33, i64 0, i64 0
  %9 = call i32 @puts(i8* %8)
  %10 = add i32 0, 0
  ; Expression result: %10
  %11 = alloca i32, align 4
  store i32 20, i32* %11, align 4
  ; Variable val2 allocated
  %12 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.34, i64 0, i64 0
  %13 = call i32 @puts(i8* %12)
  %14 = add i32 0, 0
  ; Expression result: %14
  %15 = alloca i32, align 4
  store i32 20, i32* %15, align 4
  ; Variable old_val allocated
  %16 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.35, i64 0, i64 0
  %17 = call i32 @puts(i8* %16)
  %18 = add i32 0, 0
  ; Expression result: %18
  %19 = alloca i32, align 4
  store i32 30, i32* %19, align 4
  ; Variable final_val allocated
  %20 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.36, i64 0, i64 0
  %21 = call i32 @puts(i8* %20)
  %22 = add i32 0, 0
  ; Expression result: %22
  %23 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.37, i64 0, i64 0
  %24 = call i32 @puts(i8* %23)
  %25 = add i32 0, 0
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  ; Expression result: %26
  ; Expression result: 0
  ret i32 0
}

define i32 @test_comprehensive_synchronization() {
entry:
  %0 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.38, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = alloca i32, align 4
  store i32 1, i32* %3, align 4
  ; Variable mu allocated
  %4 = alloca i32, align 4
  store i32 2, i32* %4, align 4
  ; Variable rw allocated
  %5 = alloca i32, align 4
  store i32 3, i32* %5, align 4
  ; Variable sem allocated
  %6 = alloca i32, align 4
  store i32 4, i32* %6, align 4
  ; Variable once_obj allocated
  %7 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.39, i64 0, i64 0
  %8 = call i32 @puts(i8* %7)
  %9 = add i32 0, 0
  ; Expression result: %9
  %10 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.40, i64 0, i64 0
  %11 = call i32 @puts(i8* %10)
  %12 = add i32 0, 0
  ; Expression result: %12
  %13 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.41, i64 0, i64 0
  %14 = call i32 @puts(i8* %13)
  %15 = add i32 0, 0
  ; Expression result: %15
  %16 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.42, i64 0, i64 0
  %17 = call i32 @puts(i8* %16)
  %18 = add i32 0, 0
  ; Expression result: %18
  %19 = getelementptr inbounds [43 x i8], [43 x i8]* @.str.43, i64 0, i64 0
  %20 = call i32 @puts(i8* %19)
  %21 = add i32 0, 0
  ; Expression result: %21
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  ; Expression result: %22
  ; Expression result: 0
  ret i32 0
}


; String constants
@.str.46 = private unnamed_addr constant [23 x i8] c"All tests PASSED! 🎉\00", align 1
@.str.21 = private unnamed_addr constant [24 x i8] c"Acquire 2 result: based\00", align 1
@.str.38 = private unnamed_addr constant [41 x i8] c"Testing comprehensive synchronization...\00", align 1
@.str.30 = private unnamed_addr constant [24 x i8] c"Once basic test: PASSED\00", align 1
@.str.5 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.40 = private unnamed_addr constant [16 x i8] c"RWLock value: 2\00", align 1
@.str.29 = private unnamed_addr constant [31 x i8] c"Second execution result: based\00", align 1
@.str.23 = private unnamed_addr constant [24 x i8] c"Release 1 result: based\00", align 1
@.str.31 = private unnamed_addr constant [29 x i8] c"Testing atomic operations...\00", align 1
@.str.45 = private unnamed_addr constant [38 x i8] c"=== ALL VIBE_LOCK TESTS COMPLETED ===\00", align 1
@.str.6 = private unnamed_addr constant [26 x i8] c"Testing mutex try lock...\00", align 1
@.str.15 = private unnamed_addr constant [25 x i8] c"Write lock result: based\00", align 1
@.str.28 = private unnamed_addr constant [30 x i8] c"First execution result: based\00", align 1
@.str.33 = private unnamed_addr constant [17 x i8] c"Loaded value: 10\00", align 1
@.str.37 = private unnamed_addr constant [31 x i8] c"Atomic operations test: PASSED\00", align 1
@.str.1 = private unnamed_addr constant [29 x i8] c"Created mutex with value: 42\00", align 1
@.str.3 = private unnamed_addr constant [21 x i8] c"Unlock result: based\00", align 1
@.str.27 = private unnamed_addr constant [34 x i8] c"Created once object with value: 0\00", align 1
@.str.43 = private unnamed_addr constant [43 x i8] c"Comprehensive synchronization test: PASSED\00", align 1
@.str.36 = private unnamed_addr constant [16 x i8] c"Final value: 30\00", align 1
@.str.18 = private unnamed_addr constant [21 x i8] c"Testing semaphore...\00", align 1
@.str.7 = private unnamed_addr constant [23 x i8] c"Try lock result: based\00", align 1
@.str.17 = private unnamed_addr constant [26 x i8] c"RWLock basic test: PASSED\00", align 1
@.str.2 = private unnamed_addr constant [19 x i8] c"Lock result: based\00", align 1
@.str.13 = private unnamed_addr constant [24 x i8] c"Read lock result: based\00", align 1
@.str.26 = private unnamed_addr constant [31 x i8] c"Testing once initialization...\00", align 1
@.str.32 = private unnamed_addr constant [25 x i8] c"Created atomic value: 10\00", align 1
@.str.9 = private unnamed_addr constant [24 x i8] c"Try final result: based\00", align 1
@.str.25 = private unnamed_addr constant [29 x i8] c"Semaphore basic test: PASSED\00", align 1
@.str.0 = private unnamed_addr constant [34 x i8] c"Testing mutex basic operations...\00", align 1
@.str.35 = private unnamed_addr constant [14 x i8] c"Old value: 20\00", align 1
@.str.10 = private unnamed_addr constant [28 x i8] c"Mutex try lock test: PASSED\00", align 1
@.str.4 = private unnamed_addr constant [25 x i8] c"Mutex basic test: PASSED\00", align 1
@.str.16 = private unnamed_addr constant [27 x i8] c"Write unlock result: based\00", align 1
@.str.20 = private unnamed_addr constant [24 x i8] c"Acquire 1 result: based\00", align 1
@.str.11 = private unnamed_addr constant [27 x i8] c"Testing read-write lock...\00", align 1
@.str.14 = private unnamed_addr constant [26 x i8] c"Read unlock result: based\00", align 1
@.str.22 = private unnamed_addr constant [24 x i8] c"Acquire 3 result: based\00", align 1
@.str.39 = private unnamed_addr constant [15 x i8] c"Mutex value: 1\00", align 1
@.str.12 = private unnamed_addr constant [31 x i8] c"Created rwlock with value: 100\00", align 1
@.str.34 = private unnamed_addr constant [18 x i8] c"Updated value: 20\00", align 1
@.str.8 = private unnamed_addr constant [22 x i8] c"Try again result: cap\00", align 1
@.str.19 = private unnamed_addr constant [32 x i8] c"Created semaphore with count: 3\00", align 1
@.str.24 = private unnamed_addr constant [24 x i8] c"Acquire 4 result: based\00", align 1
@.str.41 = private unnamed_addr constant [19 x i8] c"Semaphore value: 3\00", align 1
@.str.42 = private unnamed_addr constant [21 x i8] c"Once object value: 4\00", align 1
@.str.44 = private unnamed_addr constant [40 x i8] c"=== VIBE_LOCK SYNCHRONIZATION TESTS ===\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.44, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = call i32 @test_mutex_basic()
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  %8 = call i32 @puts(i8* %7)
  %9 = add i32 0, 0
  ; Expression result: %9
  %10 = call i32 @test_mutex_try_lock()
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  %12 = call i32 @puts(i8* %11)
  %13 = add i32 0, 0
  ; Expression result: %13
  %14 = call i32 @test_rwlock_basic()
  ; Expression result: %14
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  %16 = call i32 @puts(i8* %15)
  %17 = add i32 0, 0
  ; Expression result: %17
  %18 = call i32 @test_semaphore_basic()
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  %20 = call i32 @puts(i8* %19)
  %21 = add i32 0, 0
  ; Expression result: %21
  %22 = call i32 @test_once_basic()
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  %24 = call i32 @puts(i8* %23)
  %25 = add i32 0, 0
  ; Expression result: %25
  %26 = call i32 @test_atomic_operations()
  ; Expression result: %26
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  %28 = call i32 @puts(i8* %27)
  %29 = add i32 0, 0
  ; Expression result: %29
  %30 = call i32 @test_comprehensive_synchronization()
  ; Expression result: %30
  %31 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  %32 = call i32 @puts(i8* %31)
  %33 = add i32 0, 0
  ; Expression result: %33
  %34 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.45, i64 0, i64 0
  %35 = call i32 @puts(i8* %34)
  %36 = add i32 0, 0
  ; Expression result: %36
  %37 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.46, i64 0, i64 0
  %38 = call i32 @puts(i8* %37)
  %39 = add i32 0, 0
  ; Expression result: %39
  %40 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  ; Expression result: %40
  ; Expression result: 0
  ret i32 0
}

