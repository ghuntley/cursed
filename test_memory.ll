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
declare void @cursed_panic(i8*, i64)
declare i8* @cursed_alloc(i64)
declare void @cursed_free(i8*)
declare i32 @cursed_goroutine_spawn(i8*)
declare void @cursed_channel_send(i8*, i8*)
declare i8* @cursed_channel_receive(i8*)
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
declare i8* @cursed_create_structured_error()
declare i8* @cursed_set_error_message(i8*, i8*)
declare i8* @cursed_set_error_code(i8*, i32)
declare i8* @cursed_set_error_details(i8*, i8*)
declare i8* @cursed_set_error_field(i8*, i8*, i8*)
declare i8* @cursed_get_error_field(i8*, i8*)
declare i32 @cursed_get_error_code(i8*)
declare i8* @cursed_get_error_message(i8*)
declare i8* @cursed_get_error_details(i8*)
declare void @cursed_enhanced_try_begin(i64)
declare void @cursed_enhanced_try_end(i64)
declare i8* @cursed_get_panic_context(i64)
declare i8* @cursed_extract_panic_value(i8*)
declare i8* @cursed_extract_stack_trace(i8*)
declare void @cursed_clear_panic_context(i64)
declare void @cursed_register_panic_handler(i64, i8*)
declare i8* @cursed_handle_panic(i64, i8*)
declare void @cursed_propagate_error_context(i64, i64)
declare i8* @cursed_get_goroutine_error_context(i64)
declare void @cursed_clear_goroutine_error_context(i64)
declare i8* @cursed_create_enhanced_context(i8*, i64)
declare i8* @cursed_link_error_context(i8*, i8*)
declare i8* @cursed_capture_stack_trace()
declare i64 @cursed_get_current_goroutine_id()
declare i64 @time(i64*)
declare i8* @cursed_propagate_with_context(i8*, i8*)
@error_msg_default = private unnamed_addr constant [15 x i8] c"Error occurred\00"

; Module Declarations from Imports
; mod module declarations
declare void @mod_init()
declare void @mod_cleanup()
; mod module declarations



; String constants
@.str.15 = private unnamed_addr constant [27 x i8] c"pool_alloc from valid pool\00", align 1
@.str.9 = private unnamed_addr constant [26 x i8] c"track_allocation with tag\00", align 1
@.str.10 = private unnamed_addr constant [16 x i8] c"test_allocation\00", align 1
@.str.1 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.31 = private unnamed_addr constant [27 x i8] c"pool allocation efficiency\00", align 1
@.str.11 = private unnamed_addr constant [29 x i8] c"memory_report returns string\00", align 1
@.str.34 = private unnamed_addr constant [23 x i8] c"align_size with size 1\00", align 1
@.str.2 = private unnamed_addr constant [21 x i8] c"malloc size tracking\00", align 1
@.str.28 = private unnamed_addr constant [39 x i8] c"memory_compact returns compacted bytes\00", align 1
@.str.4 = private unnamed_addr constant [27 x i8] c"realloc with valid pointer\00", align 1
@.str.18 = private unnamed_addr constant [26 x i8] c"pool_free to invalid pool\00", align 1
@.str.19 = private unnamed_addr constant [22 x i8] c"zero_memory operation\00", align 1
@.str.22 = private unnamed_addr constant [30 x i8] c"align_size to 8-byte boundary\00", align 1
@.str.32 = private unnamed_addr constant [19 x i8] c"reset_memory_stats\00", align 1
@.str.29 = private unnamed_addr constant [30 x i8] c"multiple allocations tracking\00", align 1
@.str.33 = private unnamed_addr constant [22 x i8] c"malloc with zero size\00", align 1
@.str.14 = private unnamed_addr constant [28 x i8] c"create_pool returns pool ID\00", align 1
@.str.20 = private unnamed_addr constant [22 x i8] c"copy_memory operation\00", align 1
@.str.21 = private unnamed_addr constant [25 x i8] c"compare_memory operation\00", align 1
@.str.23 = private unnamed_addr constant [27 x i8] c"align_size already aligned\00", align 1
@.str.8 = private unnamed_addr constant [31 x i8] c"gc_pressure returns percentage\00", align 1
@.str.13 = private unnamed_addr constant [37 x i8] c"check_stack_overflow returns boolean\00", align 1
@.str.35 = private unnamed_addr constant [26 x i8] c"gc_collect multiple times\00", align 1
@.str.30 = private unnamed_addr constant [28 x i8] c"memory pressure calculation\00", align 1
@.str.37 = private unnamed_addr constant [11 x i8] c"final_test\00", align 1
@.str.17 = private unnamed_addr constant [29 x i8] c"pool_alloc from invalid pool\00", align 1
@.str.0 = private unnamed_addr constant [24 x i8] c"malloc basic allocation\00", align 1
@.str.36 = private unnamed_addr constant [37 x i8] c"memory report contains expected data\00", align 1
@.str.6 = private unnamed_addr constant [31 x i8] c"gc_collect returns freed bytes\00", align 1
@.str.7 = private unnamed_addr constant [24 x i8] c"gc_stats returns string\00", align 1
@.str.12 = private unnamed_addr constant [38 x i8] c"get_stack_size returns positive value\00", align 1
@.str.24 = private unnamed_addr constant [22 x i8] c"is_aligned check true\00", align 1
@.str.16 = private unnamed_addr constant [24 x i8] c"pool_free to valid pool\00", align 1
@.str.27 = private unnamed_addr constant [39 x i8] c"get_memory_usage returns current usage\00", align 1
@.str.5 = private unnamed_addr constant [26 x i8] c"realloc with null pointer\00", align 1
@.str.3 = private unnamed_addr constant [14 x i8] c"free function\00", align 1
@.str.26 = private unnamed_addr constant [17 x i8] c"set_memory_limit\00", align 1
@.str.25 = private unnamed_addr constant [23 x i8] c"is_aligned check false\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  %2 = call i32 @malloc(i32 1024)
  %3 = alloca i64, align 4
  store i64 %2, i64* %3, align 4
  ; Variable ptr allocated at %3
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %6 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.2, i64 0, i64 0
  %7 = call i32 @test_start(i32 %6)
  %8 = call i32 @get_memory_usage()
  %9 = alloca i64, align 4
  store i64 %8, i64* %9, align 4
  ; Variable initial_allocated allocated at %9
  %10 = call i32 @malloc(i32 512)
  %11 = alloca i64, align 4
  store i64 %10, i64* %11, align 4
  ; Variable new_ptr allocated at %11
  %12 = call i32 @get_memory_usage()
  %13 = alloca i64, align 4
  store i64 %12, i64* %13, align 4
  ; Variable final_allocated allocated at %13
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %15 = load i32, i32* %9, align 4
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %17 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.3, i64 0, i64 0
  %18 = call i32 @test_start(i32 %17)
  %19 = load i32, i32* %3, align 4
  %20 = call i32 @free(i32 %19)
  %21 = alloca i1, align 4
  store i1 %20, i1* %21, align 4
  ; Variable free_result allocated at %21
  %22 = load i32, i32* %21, align 4
  %23 = call i32 @assert_true(i32 %22)
  %24 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.4, i64 0, i64 0
  %25 = call i32 @test_start(i32 %24)
  %26 = call i32 @malloc(i32 256)
  %27 = alloca i64, align 4
  store i64 %26, i64* %27, align 4
  ; Variable old_ptr allocated at %27
  %28 = load i32, i32* %27, align 4
  %29 = call i32 @realloc(i32 %28, i32 512)
  %30 = alloca i64, align 4
  store i64 %29, i64* %30, align 4
  ; Variable new_ptr allocated at %30
  %31 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %33 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.5, i64 0, i64 0
  %34 = call i32 @test_start(i32 %33)
  %35 = call i32 @realloc(i32 0, i32 256)
  %36 = alloca i64, align 4
  store i64 %35, i64* %36, align 4
  ; Variable null_realloc allocated at %36
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %39 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.6, i64 0, i64 0
  %40 = call i32 @test_start(i32 %39)
  %41 = call i32 @gc_collect()
  %42 = alloca i32, align 4
  store i32 %41, i32* %42, align 4
  ; Variable freed_bytes allocated at %42
  %43 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %44 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %45 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.7, i64 0, i64 0
  %46 = call i32 @test_start(i32 %45)
  %47 = call i32 @gc_stats()
  %48 = alloca i8*, align 4
  store i8* %47, i8** %48, align 4
  ; Variable stats allocated at %48
  %49 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %50 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %51 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.8, i64 0, i64 0
  %52 = call i32 @test_start(i32 %51)
  %53 = call i32 @gc_pressure()
  %54 = alloca i32, align 4
  store i32 %53, i32* %54, align 4
  ; Variable pressure allocated at %54
  %55 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %56 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %57 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %58 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %59 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.9, i64 0, i64 0
  %60 = call i32 @test_start(i32 %59)
  %61 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.10, i64 0, i64 0
  %62 = call i32 @track_allocation(i32 1024, i32 %61)
  %63 = alloca i1, align 4
  store i1 %62, i1* %63, align 4
  ; Variable track_result allocated at %63
  %64 = load i32, i32* %63, align 4
  %65 = call i32 @assert_true(i32 %64)
  %66 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.11, i64 0, i64 0
  %67 = call i32 @test_start(i32 %66)
  %68 = call i32 @memory_report()
  %69 = alloca i8*, align 4
  store i8* %68, i8** %69, align 4
  ; Variable report allocated at %69
  %70 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %71 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %72 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.12, i64 0, i64 0
  %73 = call i32 @test_start(i32 %72)
  %74 = call i32 @get_stack_size()
  %75 = alloca i32, align 4
  store i32 %74, i32* %75, align 4
  ; Variable stack_size allocated at %75
  %76 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %77 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %78 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.13, i64 0, i64 0
  %79 = call i32 @test_start(i32 %78)
  %80 = call i32 @check_stack_overflow()
  %81 = alloca i1, align 4
  store i1 %80, i1* %81, align 4
  ; Variable overflow_check allocated at %81
  %82 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %83 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %84 = load i32, i32* %81, align 4
  %85 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %86 = icmp eq i32 %84, %85
  %87 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %88 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.14, i64 0, i64 0
  %89 = call i32 @test_start(i32 %88)
  %90 = call i32 @create_pool(i32 64, i32 100)
  %91 = alloca i64, align 4
  store i64 %90, i64* %91, align 4
  ; Variable pool_id allocated at %91
  %92 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %93 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %94 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.15, i64 0, i64 0
  %95 = call i32 @test_start(i32 %94)
  %96 = load i32, i32* %91, align 4
  %97 = call i32 @pool_alloc(i32 %96, i32 64)
  %98 = alloca i64, align 4
  store i64 %97, i64* %98, align 4
  ; Variable pool_ptr allocated at %98
  %99 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %100 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %101 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.16, i64 0, i64 0
  %102 = call i32 @test_start(i32 %101)
  %103 = load i32, i32* %91, align 4
  %104 = load i32, i32* %98, align 4
  %105 = call i32 @pool_free(i32 %103, i32 %104)
  %106 = alloca i1, align 4
  store i1 %105, i1* %106, align 4
  ; Variable pool_free_result allocated at %106
  %107 = load i32, i32* %106, align 4
  %108 = call i32 @assert_true(i32 %107)
  %109 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.17, i64 0, i64 0
  %110 = call i32 @test_start(i32 %109)
  %111 = call i32 @pool_alloc(i32 999, i32 64)
  %112 = alloca i64, align 4
  store i64 %111, i64* %112, align 4
  ; Variable invalid_pool_ptr allocated at %112
  %113 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %114 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %115 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.18, i64 0, i64 0
  %116 = call i32 @test_start(i32 %115)
  %117 = call i32 @pool_free(i32 999, i32 123)
  %118 = alloca i1, align 4
  store i1 %117, i1* %118, align 4
  ; Variable invalid_pool_free allocated at %118
  %119 = load i32, i32* %118, align 4
  %120 = call i32 @assert_false(i32 %119)
  %121 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.19, i64 0, i64 0
  %122 = call i32 @test_start(i32 %121)
  %123 = call i32 @zero_memory(i32 123, i32 256)
  %124 = alloca i1, align 4
  store i1 %123, i1* %124, align 4
  ; Variable zero_result allocated at %124
  %125 = load i32, i32* %124, align 4
  %126 = call i32 @assert_true(i32 %125)
  %127 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.20, i64 0, i64 0
  %128 = call i32 @test_start(i32 %127)
  %129 = call i32 @copy_memory(i32 123, i32 456, i32 128)
  %130 = alloca i1, align 4
  store i1 %129, i1* %130, align 4
  ; Variable copy_result allocated at %130
  %131 = load i32, i32* %130, align 4
  %132 = call i32 @assert_true(i32 %131)
  %133 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.21, i64 0, i64 0
  %134 = call i32 @test_start(i32 %133)
  %135 = call i32 @compare_memory(i32 123, i32 123, i32 64)
  %136 = alloca i32, align 4
  store i32 %135, i32* %136, align 4
  ; Variable compare_result allocated at %136
  %137 = load i32, i32* %136, align 4
  %138 = call i32 @assert_eq_int(i32 %137, i32 0)
  %139 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.22, i64 0, i64 0
  %140 = call i32 @test_start(i32 %139)
  %141 = call i32 @align_size(i32 100, i32 8)
  %142 = alloca i32, align 4
  store i32 %141, i32* %142, align 4
  ; Variable aligned_size allocated at %142
  %143 = load i32, i32* %142, align 4
  %144 = call i32 @assert_eq_int(i32 %143, i32 104)
  %145 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.23, i64 0, i64 0
  %146 = call i32 @test_start(i32 %145)
  %147 = call i32 @align_size(i32 128, i32 8)
  %148 = alloca i32, align 4
  store i32 %147, i32* %148, align 4
  ; Variable already_aligned allocated at %148
  %149 = load i32, i32* %148, align 4
  %150 = call i32 @assert_eq_int(i32 %149, i32 128)
  %151 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.24, i64 0, i64 0
  %152 = call i32 @test_start(i32 %151)
  %153 = call i32 @is_aligned(i32 128, i32 8)
  %154 = alloca i1, align 4
  store i1 %153, i1* %154, align 4
  ; Variable aligned_check allocated at %154
  %155 = load i32, i32* %154, align 4
  %156 = call i32 @assert_true(i32 %155)
  %157 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.25, i64 0, i64 0
  %158 = call i32 @test_start(i32 %157)
  %159 = call i32 @is_aligned(i32 129, i32 8)
  %160 = alloca i1, align 4
  store i1 %159, i1* %160, align 4
  ; Variable unaligned_check allocated at %160
  %161 = load i32, i32* %160, align 4
  %162 = call i32 @assert_false(i32 %161)
  %163 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.26, i64 0, i64 0
  %164 = call i32 @test_start(i32 %163)
  %165 = call i32 @set_memory_limit(i32 1048576)
  %166 = alloca i1, align 4
  store i1 %165, i1* %166, align 4
  ; Variable limit_result allocated at %166
  %167 = load i32, i32* %166, align 4
  %168 = call i32 @assert_true(i32 %167)
  %169 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.27, i64 0, i64 0
  %170 = call i32 @test_start(i32 %169)
  %171 = call i32 @get_memory_usage()
  %172 = alloca i64, align 4
  store i64 %171, i64* %172, align 4
  ; Variable current_usage allocated at %172
  %173 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %174 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %175 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.28, i64 0, i64 0
  %176 = call i32 @test_start(i32 %175)
  %177 = call i32 @memory_compact()
  %178 = alloca i32, align 4
  store i32 %177, i32* %178, align 4
  ; Variable compacted allocated at %178
  %179 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %180 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %181 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.29, i64 0, i64 0
  %182 = call i32 @test_start(i32 %181)
  %183 = call i32 @malloc(i32 512)
  %184 = alloca i64, align 4
  store i64 %183, i64* %184, align 4
  ; Variable ptr1 allocated at %184
  %185 = call i32 @malloc(i32 1024)
  %186 = alloca i64, align 4
  store i64 %185, i64* %186, align 4
  ; Variable ptr2 allocated at %186
  %187 = call i32 @malloc(i32 256)
  %188 = alloca i64, align 4
  store i64 %187, i64* %188, align 4
  ; Variable ptr3 allocated at %188
  %189 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %190 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %191 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %192 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %193 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %194 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %195 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %196 = load i32, i32* %186, align 4
  %197 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %198 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %199 = load i32, i32* %188, align 4
  %200 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %201 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.30, i64 0, i64 0
  %202 = call i32 @test_start(i32 %201)
  %203 = alloca i32, align 4
  store i32 0, i32* %203, align 4
  ; Short declaration: i := 0 (i32)
  br label %label0
label0:
  %204 = load i32, i32* %203, align 4
  %205 = icmp slt i32 %204, 10
  br i1 %205, label %label1, label %label3
label1:
  %206 = call i32 @malloc(i32 1024)
  %207 = alloca i64, align 4
  store i64 %206, i64* %207, align 4
  ; Variable temp_ptr allocated at %207
  br label %label2
label2:
  %208 = load i32, i32* %203, align 4
  %209 = add i32 %208, 1
  store i32 %209, i32* %203, align 4
  br label %label0
label3:
  %210 = call i32 @gc_pressure()
  %211 = alloca i32, align 4
  store i32 %210, i32* %211, align 4
  ; Variable high_pressure allocated at %211
  %212 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %213 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %214 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.31, i64 0, i64 0
  %215 = call i32 @test_start(i32 %214)
  %216 = call i32 @create_pool(i32 32, i32 50)
  %217 = alloca i64, align 4
  store i64 %216, i64* %217, align 4
  ; Variable efficient_pool allocated at %217
  %218 = alloca i32, align 4
  store i32 0, i32* %218, align 4
  ; Variable allocations_successful allocated at %218
  %219 = alloca i32, align 4
  store i32 0, i32* %219, align 4
  ; Short declaration: j := 0 (i32)
  br label %label4
label4:
  %220 = load i32, i32* %219, align 4
  %221 = icmp slt i32 %220, 10
  br i1 %221, label %label5, label %label7
label5:
  %222 = load i32, i32* %217, align 4
  %223 = call i32 @pool_alloc(i32 %222, i32 32)
  %224 = alloca i64, align 4
  store i64 %223, i64* %224, align 4
  ; Variable temp_pool_ptr allocated at %224
  %225 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %226 = load i32, i32* %224, align 4
  %227 = icmp sgt i32 %226, 0
  %228 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %229 = load i32, i32* %218, align 4
  %230 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %231 = load i32, i32* %218, align 4
  %232 = add i32 %231, 1
  br label %label6
label6:
  %233 = load i32, i32* %219, align 4
  %234 = add i32 %233, 1
  store i32 %234, i32* %219, align 4
  br label %label4
label7:
  %235 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %236 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %237 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %238 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.32, i64 0, i64 0
  %239 = call i32 @test_start(i32 %238)
  %240 = call i32 @reset_memory_stats()
  %241 = alloca i1, align 4
  store i1 %240, i1* %241, align 4
  ; Variable reset_result allocated at %241
  %242 = load i32, i32* %241, align 4
  %243 = call i32 @assert_true(i32 %242)
  %244 = call i32 @get_memory_usage()
  %245 = alloca i64, align 4
  store i64 %244, i64* %245, align 4
  ; Variable usage_after_reset allocated at %245
  %246 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %247 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %248 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.33, i64 0, i64 0
  %249 = call i32 @test_start(i32 %248)
  %250 = call i32 @malloc(i32 0)
  %251 = alloca i64, align 4
  store i64 %250, i64* %251, align 4
  ; Variable zero_ptr allocated at %251
  %252 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %253 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %254 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.34, i64 0, i64 0
  %255 = call i32 @test_start(i32 %254)
  %256 = call i32 @align_size(i32 1, i32 4)
  %257 = alloca i32, align 4
  store i32 %256, i32* %257, align 4
  ; Variable tiny_aligned allocated at %257
  %258 = load i32, i32* %257, align 4
  %259 = call i32 @assert_eq_int(i32 %258, i32 4)
  %260 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.35, i64 0, i64 0
  %261 = call i32 @test_start(i32 %260)
  %262 = call i32 @gc_collect()
  %263 = alloca i32, align 4
  store i32 %262, i32* %263, align 4
  ; Variable gc1 allocated at %263
  %264 = call i32 @gc_collect()
  %265 = alloca i32, align 4
  store i32 %264, i32* %265, align 4
  ; Variable gc2 allocated at %265
  %266 = call i32 @gc_collect()
  %267 = alloca i32, align 4
  store i32 %266, i32* %267, align 4
  ; Variable gc3 allocated at %267
  %268 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %269 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %270 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %271 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %272 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %273 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %274 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.36, i64 0, i64 0
  %275 = call i32 @test_start(i32 %274)
  %276 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.37, i64 0, i64 0
  %277 = call i32 @track_allocation(i32 2048, i32 %276)
  %278 = call i32 @memory_report()
  %279 = alloca i8*, align 4
  store i8* %278, i8** %279, align 4
  ; Variable final_report allocated at %279
  %280 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %281 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %282 = call i32 @print_test_summary()
  ret i32 0
}
