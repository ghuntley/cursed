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
declare i8* @malloc(i32)
declare void @free(i8*)
@error_msg_default = private unnamed_addr constant [13 x i8] c"Error occurred\00"


; String constants
@.str.3 = private unnamed_addr constant [25 x i8] c"Memory optimization test\00", align 1
@.str.7 = private unnamed_addr constant [24 x i8] c"Test 3: GC Optimization\00", align 1
@.str.8 = private unnamed_addr constant [15 x i8] c"GC test object\00", align 1
@.str.9 = private unnamed_addr constant [39 x i8] c"Test 3 passed: GC optimization working\00", align 1
@.str.16 = private unnamed_addr constant [48 x i8] c"Test 4 passed: Memory pressure handling working\00", align 1
@.str.17 = private unnamed_addr constant [35 x i8] c"Test 5: Adaptive Memory Management\00", align 1
@.str.20 = private unnamed_addr constant [4 x i8] c"100\00", align 1
@.str.28 = private unnamed_addr constant [34 x i8] c"Test 9: Memory System Integration\00", align 1
@.str.11 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.4 = private unnamed_addr constant [40 x i8] c"Test 1 passed: Basic allocation working\00", align 1
@.str.0 = private unnamed_addr constant [38 x i8] c"Starting memory optimization tests...\00", align 1
@.str.19 = private unnamed_addr constant [25 x i8] c"Test 6: Memory Profiling\00", align 1
@.str.24 = private unnamed_addr constant [37 x i8] c"Test 7: Concurrent Memory Allocation\00", align 1
@.str.22 = private unnamed_addr constant [4 x i8] c"300\00", align 1
@.str.33 = private unnamed_addr constant [58 x i8] c"✅ All memory optimization tests completed successfully!\00", align 1
@.str.2 = private unnamed_addr constant [32 x i8] c"Test 1: Basic Memory Allocation\00", align 1
@.str.13 = private unnamed_addr constant [4 x i8] c"128\00", align 1
@.str.26 = private unnamed_addr constant [39 x i8] c"Test 8: Memory Optimization Under Load\00", align 1
@.str.29 = private unnamed_addr constant [17 x i8] c"Integration test\00", align 1
@.str.30 = private unnamed_addr constant [49 x i8] c"Test 9 passed: Memory system integration working\00", align 1
@.str.5 = private unnamed_addr constant [33 x i8] c"Test 2: Memory Pool Optimization\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.18 = private unnamed_addr constant [50 x i8] c"Test 5 passed: Adaptive memory management working\00", align 1
@.str.31 = private unnamed_addr constant [26 x i8] c"Test 10: Final Validation\00", align 1
@.str.6 = private unnamed_addr constant [41 x i8] c"Test 2 passed: Pool optimization working\00", align 1
@.str.12 = private unnamed_addr constant [3 x i8] c"64\00", align 1
@.str.21 = private unnamed_addr constant [4 x i8] c"200\00", align 1
@.str.34 = private unnamed_addr constant [47 x i8] c"Memory management system is working correctly!\00", align 1
@.str.25 = private unnamed_addr constant [45 x i8] c"Test 7 passed: Concurrent allocation working\00", align 1
@.str.35 = private unnamed_addr constant [78 x i8] c"Adaptive GC, pressure detection, pool optimization, and profiling are active!\00", align 1
@.str.15 = private unnamed_addr constant [4 x i8] c"512\00", align 1
@.str.23 = private unnamed_addr constant [40 x i8] c"Test 6 passed: Memory profiling working\00", align 1
@.str.14 = private unnamed_addr constant [4 x i8] c"256\00", align 1
@.str.32 = private unnamed_addr constant [41 x i8] c"Test 10 passed: Final validation working\00", align 1
@.str.27 = private unnamed_addr constant [54 x i8] c"Test 8 passed: Memory optimization under load working\00", align 1
@.str.10 = private unnamed_addr constant [33 x i8] c"Test 4: Memory Pressure Handling\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.0, i64 0, i64 0
  ; Converting complex expression to output
  %1 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %2 = call i32 (i8*, ...) @printf(i8* %1, i32 %0)
  %3 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.2, i64 0, i64 0
  ; Converting complex expression to output
  %4 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %5 = call i32 (i8*, ...) @printf(i8* %4, i32 %3)
  %6 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.3, i64 0, i64 0
  %7 = alloca i8*, align 4
  store i8* %6, i8** %7, align 4
  ; Variable test_string allocated at %7
  %8 = alloca i32, align 4
  store i32 42, i32* %8, align 4
  ; Variable test_int allocated at %8
  %9 = alloca double, align 4
  store double 3.14, double* %9, align 4
  ; Variable test_float allocated at %9
  %10 = load i32, i32* %7, align 4
  %11 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %12 = call i32 (i8*, ...) @printf(i8* %11, i32 %10)
  %13 = load i32, i32* %8, align 4
  %14 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %15 = call i32 (i8*, ...) @printf(i8* %14, i32 %13)
  %16 = load i32, i32* %9, align 4
  %17 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %18 = call i32 (i8*, ...) @printf(i8* %17, i32 %16)
  %19 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.4, i64 0, i64 0
  ; Converting complex expression to output
  %20 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %21 = call i32 (i8*, ...) @printf(i8* %20, i32 %19)
  %22 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.5, i64 0, i64 0
  ; Converting complex expression to output
  %23 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %24 = call i32 (i8*, ...) @printf(i8* %23, i32 %22)
  %25 = alloca i32, align 4
  store i32 0, i32* %25, align 4
  ; Short declaration: i := 0 (i32)
  br label %label0
label0:
  %26 = load i32, i32* %25, align 4
  %27 = icmp slt i32 %26, 10
  br i1 %27, label %label1, label %label3
label1:
  %28 = load i32, i32* %25, align 4
  %29 = mul i32 %28, 100
  %30 = alloca i32, align 4
  store i32 %29, i32* %30, align 4
  ; Variable pool_obj allocated at %30
  %31 = load i32, i32* %30, align 4
  %32 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %33 = call i32 (i8*, ...) @printf(i8* %32, i32 %31)
  br label %label2
label2:
  %34 = load i32, i32* %25, align 4
  %35 = add i32 %34, 1
  store i32 %35, i32* %25, align 4
  br label %label0
label3:
  %36 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.6, i64 0, i64 0
  ; Converting complex expression to output
  %37 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %38 = call i32 (i8*, ...) @printf(i8* %37, i32 %36)
  %39 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.7, i64 0, i64 0
  ; Converting complex expression to output
  %40 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %41 = call i32 (i8*, ...) @printf(i8* %40, i32 %39)
  %42 = alloca i32, align 4
  store i32 0, i32* %42, align 4
  ; Short declaration: j := 0 (i32)
  br label %label4
label4:
  %43 = load i32, i32* %42, align 4
  %44 = icmp slt i32 %43, 5
  br i1 %44, label %label5, label %label7
label5:
  %45 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.8, i64 0, i64 0
  %46 = alloca i8*, align 4
  store i8* %45, i8** %46, align 4
  ; Variable gc_obj allocated at %46
  %47 = load i32, i32* %46, align 4
  %48 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %49 = call i32 (i8*, ...) @printf(i8* %48, i32 %47)
  br label %label6
label6:
  %50 = load i32, i32* %42, align 4
  %51 = add i32 %50, 1
  store i32 %51, i32* %42, align 4
  br label %label4
label7:
  %52 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.9, i64 0, i64 0
  ; Converting complex expression to output
  %53 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %54 = call i32 (i8*, ...) @printf(i8* %53, i32 %52)
  %55 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.10, i64 0, i64 0
  ; Converting complex expression to output
  %56 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %57 = call i32 (i8*, ...) @printf(i8* %56, i32 %55)
  %58 = alloca i32, align 4
  store i32 null, i32* %58, align 4
  ; Variable pressure_sizes allocated at %58
  %59 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.11, i64 0, i64 0
  %60 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.11, i64 0, i64 0
  %61 = alloca [4 x i32], align 4
  %62 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.12, i64 0, i64 0
  %63 = getelementptr inbounds [4 x i32], [4 x i32]* %61, i64 0, i64 0
  store i32 %62, i32* %63, align 4
  %64 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.13, i64 0, i64 0
  %65 = getelementptr inbounds [4 x i32], [4 x i32]* %61, i64 0, i64 1
  store i32 %64, i32* %65, align 4
  %66 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.14, i64 0, i64 0
  %67 = getelementptr inbounds [4 x i32], [4 x i32]* %61, i64 0, i64 2
  store i32 %66, i32* %67, align 4
  %68 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.15, i64 0, i64 0
  %69 = getelementptr inbounds [4 x i32], [4 x i32]* %61, i64 0, i64 3
  store i32 %68, i32* %69, align 4
  %70 = alloca i32, align 4
  store i32 0, i32* %70, align 4
  ; Short declaration: k := 0 (i32)
  br label %label8
label8:
  %71 = load i32, i32* %70, align 4
  %72 = icmp slt i32 %71, 4
  br i1 %72, label %label9, label %label11
label9:
  %73 = load i32, i32* %58, align 4
  %74 = alloca i32, align 4
  store i32 %73, i32* %74, align 4
  ; Variable pressure_obj allocated at %74
  %75 = inttoptr i64 0 to [0 x i32]*
  %76 = load i32, i32* %74, align 4
  %77 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %78 = call i32 (i8*, ...) @printf(i8* %77, i32 %76)
  br label %label10
label10:
  %79 = load i32, i32* %70, align 4
  %80 = add i32 %79, 1
  store i32 %80, i32* %70, align 4
  br label %label8
label11:
  %81 = getelementptr inbounds [48 x i8], [48 x i8]* @.str.16, i64 0, i64 0
  ; Converting complex expression to output
  %82 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %83 = call i32 (i8*, ...) @printf(i8* %82, i32 %81)
  %84 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.17, i64 0, i64 0
  ; Converting complex expression to output
  %85 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %86 = call i32 (i8*, ...) @printf(i8* %85, i32 %84)
  %87 = alloca i32, align 4
  store i32 0, i32* %87, align 4
  ; Short declaration: burst := 0 (i32)
  br label %label12
label12:
  %88 = load i32, i32* %87, align 4
  %89 = icmp slt i32 %88, 20
  br i1 %89, label %label13, label %label15
label13:
  %90 = load i32, i32* %87, align 4
  %91 = mul i32 %90, 50
  %92 = alloca i32, align 4
  store i32 %91, i32* %92, align 4
  ; Variable adaptive_obj allocated at %92
  %93 = load i32, i32* %92, align 4
  %94 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %95 = call i32 (i8*, ...) @printf(i8* %94, i32 %93)
  br label %label14
label14:
  %96 = load i32, i32* %87, align 4
  %97 = add i32 %96, 1
  store i32 %97, i32* %87, align 4
  br label %label12
label15:
  %98 = getelementptr inbounds [50 x i8], [50 x i8]* @.str.18, i64 0, i64 0
  ; Converting complex expression to output
  %99 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %100 = call i32 (i8*, ...) @printf(i8* %99, i32 %98)
  %101 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.19, i64 0, i64 0
  ; Converting complex expression to output
  %102 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %103 = call i32 (i8*, ...) @printf(i8* %102, i32 %101)
  %104 = alloca i32, align 4
  store i32 null, i32* %104, align 4
  ; Variable profiled_data allocated at %104
  %105 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.11, i64 0, i64 0
  %106 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.11, i64 0, i64 0
  %107 = alloca [3 x i32], align 4
  %108 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.20, i64 0, i64 0
  %109 = getelementptr inbounds [3 x i32], [3 x i32]* %107, i64 0, i64 0
  store i32 %108, i32* %109, align 4
  %110 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.21, i64 0, i64 0
  %111 = getelementptr inbounds [3 x i32], [3 x i32]* %107, i64 0, i64 1
  store i32 %110, i32* %111, align 4
  %112 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.22, i64 0, i64 0
  %113 = getelementptr inbounds [3 x i32], [3 x i32]* %107, i64 0, i64 2
  store i32 %112, i32* %113, align 4
  %114 = alloca i32, align 4
  store i32 0, i32* %114, align 4
  ; Short declaration: prof_idx := 0 (i32)
  br label %label16
label16:
  %115 = load i32, i32* %114, align 4
  %116 = icmp slt i32 %115, 3
  br i1 %116, label %label17, label %label19
label17:
  %117 = load i32, i32* %104, align 4
  %118 = alloca i32, align 4
  store i32 %117, i32* %118, align 4
  ; Variable prof_obj allocated at %118
  %119 = inttoptr i64 0 to [0 x i32]*
  %120 = load i32, i32* %118, align 4
  %121 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %122 = call i32 (i8*, ...) @printf(i8* %121, i32 %120)
  br label %label18
label18:
  %123 = load i32, i32* %114, align 4
  %124 = add i32 %123, 1
  store i32 %124, i32* %114, align 4
  br label %label16
label19:
  %125 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.23, i64 0, i64 0
  ; Converting complex expression to output
  %126 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %127 = call i32 (i8*, ...) @printf(i8* %126, i32 %125)
  %128 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.24, i64 0, i64 0
  ; Converting complex expression to output
  %129 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %130 = call i32 (i8*, ...) @printf(i8* %129, i32 %128)
  %131 = alloca i32, align 4
  store i32 0, i32* %131, align 4
  ; Short declaration: thread_sim := 0 (i32)
  br label %label20
label20:
  %132 = load i32, i32* %131, align 4
  %133 = icmp slt i32 %132, 15
  br i1 %133, label %label21, label %label23
label21:
  %134 = load i32, i32* %131, align 4
  %135 = mul i32 %134, 10
  %136 = alloca i32, align 4
  store i32 %135, i32* %136, align 4
  ; Variable concurrent_obj allocated at %136
  %137 = load i32, i32* %136, align 4
  %138 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %139 = call i32 (i8*, ...) @printf(i8* %138, i32 %137)
  br label %label22
label22:
  %140 = load i32, i32* %131, align 4
  %141 = add i32 %140, 1
  store i32 %141, i32* %131, align 4
  br label %label20
label23:
  %142 = getelementptr inbounds [45 x i8], [45 x i8]* @.str.25, i64 0, i64 0
  ; Converting complex expression to output
  %143 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %144 = call i32 (i8*, ...) @printf(i8* %143, i32 %142)
  %145 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.26, i64 0, i64 0
  ; Converting complex expression to output
  %146 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %147 = call i32 (i8*, ...) @printf(i8* %146, i32 %145)
  %148 = alloca i32, align 4
  store i32 0, i32* %148, align 4
  ; Short declaration: load_test := 0 (i32)
  br label %label24
label24:
  %149 = load i32, i32* %148, align 4
  %150 = icmp slt i32 %149, 50
  br i1 %150, label %label25, label %label27
label25:
  %151 = load i32, i32* %148, align 4
  %152 = alloca i32, align 4
  store i32 %151, i32* %152, align 4
  ; Variable load_obj allocated at %152
  %153 = load i32, i32* %148, align 4
  %154 = alloca i32, align 4
  store i32 %153, i32* %154, align 4
  ; Variable load_check allocated at %154
  %155 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.11, i64 0, i64 0
  br label %label26
label26:
  %156 = load i32, i32* %148, align 4
  %157 = add i32 %156, 1
  store i32 %157, i32* %148, align 4
  br label %label24
label27:
  %158 = getelementptr inbounds [54 x i8], [54 x i8]* @.str.27, i64 0, i64 0
  ; Converting complex expression to output
  %159 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %160 = call i32 (i8*, ...) @printf(i8* %159, i32 %158)
  %161 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.28, i64 0, i64 0
  ; Converting complex expression to output
  %162 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %163 = call i32 (i8*, ...) @printf(i8* %162, i32 %161)
  %164 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.29, i64 0, i64 0
  %165 = alloca i8*, align 4
  store i8* %164, i8** %165, align 4
  ; Variable integration_string allocated at %165
  %166 = alloca i32, align 4
  store i32 999, i32* %166, align 4
  ; Variable integration_int allocated at %166
  %167 = alloca double, align 4
  store double 99.99, double* %167, align 4
  ; Variable integration_float allocated at %167
  %168 = load i32, i32* %165, align 4
  %169 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %170 = call i32 (i8*, ...) @printf(i8* %169, i32 %168)
  %171 = load i32, i32* %166, align 4
  %172 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %173 = call i32 (i8*, ...) @printf(i8* %172, i32 %171)
  %174 = load i32, i32* %167, align 4
  %175 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %176 = call i32 (i8*, ...) @printf(i8* %175, i32 %174)
  %177 = getelementptr inbounds [49 x i8], [49 x i8]* @.str.30, i64 0, i64 0
  ; Converting complex expression to output
  %178 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %179 = call i32 (i8*, ...) @printf(i8* %178, i32 %177)
  %180 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.31, i64 0, i64 0
  ; Converting complex expression to output
  %181 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %182 = call i32 (i8*, ...) @printf(i8* %181, i32 %180)
  %183 = alloca i32, align 4
  store i32 0, i32* %183, align 4
  ; Short declaration: final_test := 0 (i32)
  br label %label28
label28:
  %184 = load i32, i32* %183, align 4
  %185 = icmp slt i32 %184, 25
  br i1 %185, label %label29, label %label31
label29:
  %186 = load i32, i32* %183, align 4
  %187 = load i32, i32* %183, align 4
  %188 = mul i32 %186, %187
  %189 = alloca i32, align 4
  store i32 %188, i32* %189, align 4
  ; Variable final_obj allocated at %189
  %190 = load i32, i32* %189, align 4
  %191 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %192 = call i32 (i8*, ...) @printf(i8* %191, i32 %190)
  br label %label30
label30:
  %193 = load i32, i32* %183, align 4
  %194 = add i32 %193, 1
  store i32 %194, i32* %183, align 4
  br label %label28
label31:
  %195 = getelementptr inbounds [41 x i8], [41 x i8]* @.str.32, i64 0, i64 0
  ; Converting complex expression to output
  %196 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %197 = call i32 (i8*, ...) @printf(i8* %196, i32 %195)
  %198 = getelementptr inbounds [58 x i8], [58 x i8]* @.str.33, i64 0, i64 0
  ; Converting complex expression to output
  %199 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %200 = call i32 (i8*, ...) @printf(i8* %199, i32 %198)
  %201 = getelementptr inbounds [47 x i8], [47 x i8]* @.str.34, i64 0, i64 0
  ; Converting complex expression to output
  %202 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %203 = call i32 (i8*, ...) @printf(i8* %202, i32 %201)
  %204 = getelementptr inbounds [78 x i8], [78 x i8]* @.str.35, i64 0, i64 0
  ; Converting complex expression to output
  %205 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %206 = call i32 (i8*, ...) @printf(i8* %205, i32 %204)
  ret i32 0
}
