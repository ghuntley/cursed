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
define i32 @test_basic_tracing() {
entry:
  %0 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  ; Expression result: %err
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %4 = call i32 @trace_tea_Start(i32 %3)
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %6 = call i32 @assert_eq_string(i32 %err, i32 %5)
  ; Expression result: %6
  %7 = call i32 @trace_tea_IsTraceActive()
  %8 = call i32 @assert_true(i32 %7)
  ; Expression result: %8
  ; Expression result: %err
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %9
  %10 = call i32 @trace_tea_Stop()
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %12 = call i32 @assert_eq_string(i32 %err, i32 %11)
  ; Expression result: %12
  %13 = call i32 @trace_tea_IsTraceActive()
  %14 = call i32 @assert_false(i32 %13)
  ; Expression result: %14
  ret i32 0
}

define i32 @test_task_management() {
entry:
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.2, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %3 = call i32 @trace_tea_Start(i32 %2)
  ; Expression result: %3
  ; Expression result: %ctx
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %4
  %5 = call i32 @vibe_context_Background()
  ; Expression result: %5
  ; Expression result: %ctx
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %6
  ; Expression result: %task
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %7
  %8 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %9 = call i32 @trace_tea_NewTask(i32 %ctx, i32 %8)
  ; Expression result: %9
  ; Expression result: %registry
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %10
  %11 = call i32 @trace_tea_GetTaskRegistry()
  ; Expression result: %11
  %12 = call i32 @len(i8* %registry)
  %13 = call i32 @assert_eq_int(i32 %12, i32 1)
  ; Expression result: %13
  %14 = call i32 @task_End()
  ; Expression result: %14
  %15 = call i32 @trace_tea_Stop()
  ; Expression result: %15
  ret i32 0
}

define i32 @test_region_functionality() {
entry:
  %0 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.4, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %3 = call i32 @trace_tea_Start(i32 %2)
  ; Expression result: %3
  ; Expression result: %ctx
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %4
  %5 = call i32 @vibe_context_Background()
  ; Expression result: %5
  ; Expression result: %ctx
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %6
  ; Expression result: %task
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %7
  %8 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.5, i64 0, i64 0
  %9 = call i32 @trace_tea_NewTask(i32 %ctx, i32 %8)
  ; Expression result: %9
  ; Expression result: %region
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %10
  %11 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.6, i64 0, i64 0
  %12 = call i32 @trace_tea_StartRegion(i32 %ctx, i32 %11)
  ; Expression result: %12
  %13 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.7, i64 0, i64 0
  %14 = call i32 @region_LazyLog(i32 %13)
  ; Expression result: %14
  %15 = call i32 @region_End()
  ; Expression result: %15
  %16 = call i32 @task_End()
  ; Expression result: %16
  %17 = call i32 @trace_tea_Stop()
  ; Expression result: %17
  ret i32 0
}

define i32 @test_event_logging() {
entry:
  %0 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.8, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %3 = call i32 @trace_tea_Start(i32 %2)
  ; Expression result: %3
  ; Expression result: %ctx
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %4
  %5 = call i32 @vibe_context_Background()
  ; Expression result: %5
  ; Member access: %trace_tea.EventUserDefined
  %6 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %7 = load i32, i32* %6, align 4
  %8 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.9, i64 0, i64 0
  %9 = call i32 @trace_tea_Log(i32 %ctx, i32 %7, i32 %8)
  ; Expression result: %9
  ; Member access: %trace_tea.EventAPI
  %10 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %11 = load i32, i32* %10, align 4
  %12 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.10, i64 0, i64 0
  %13 = call i32 @trace_tea_Logf(i32 %ctx, i32 %11, i32 %12, i32 42)
  ; Expression result: %13
  ; Expression result: %event
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  ; Member access: %trace_tea.EventDatabase
  %15 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %16 = load i32, i32* %15, align 4
  %17 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.11, i64 0, i64 0
  %18 = call i32 @trace_tea_NewEvent(i32 %16, i32 %17)
  ; Expression result: %18
  %19 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.12, i64 0, i64 0
  %20 = call i32 @event_LazyLog(i32 %19)
  ; Expression result: %20
  ; Expression result: %buffer
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %21
  %22 = call i32 @trace_tea_GetTraceBuffer()
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  ; Expression result: 0
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %24
  %25 = call i32 @trace_tea_Stop()
  ; Expression result: %25
  ret i32 0
}

define i32 @test_with_region() {
entry:
  %0 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.13, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %3 = call i32 @trace_tea_Start(i32 %2)
  ; Expression result: %3
  ; Expression result: %ctx
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %4
  %5 = call i32 @vibe_context_Background()
  ; Expression result: %5
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %6
  ; Expression result: %x
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %7
  %8 = add i32 1, 2
  ; Expression result: %8
  %9 = call i32 @assert_eq_int(i32 %x, i32 3)
  ; Expression result: %9
  ret i32 0
}

define i32 @test_with_span() {
entry:
  %0 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.14, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %3 = call i32 @trace_tea_Start(i32 %2)
  ; Expression result: %3
  ; Expression result: %ctx
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %4
  %5 = call i32 @vibe_context_Background()
  ; Expression result: %5
  ; Member access: %vibe_context.Context
  %6 = getelementptr inbounds %struct.object, %struct.object* %vibe_context, i32 0, i32 0
  %7 = load i32, i32* %6, align 4
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %9
  ; Member access: %trace_tea.EventCompute
  %10 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %11 = load i32, i32* %10, align 4
  %12 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.15, i64 0, i64 0
  %13 = call i32 @trace_tea_Log(i32 %spanCtx, i32 %11, i32 %12)
  ; Expression result: %13
  ret i32 0
}

define i32 @test_filter_functionality() {
entry:
  %0 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.16, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  ; Expression result: %filter
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @trace_tea_NewFilter()
  ; Expression result: %3
  %4 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.17, i64 0, i64 0
  %5 = call i32 @filter_IncludeGoroutine(i32 %4)
  ; Expression result: %5
  ; Member access: %trace_tea.EventGC
  %6 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %7 = load i32, i32* %6, align 4
  %8 = call i32 @filter_ExcludeEvent(i32 %7)
  ; Expression result: %8
  ; Member access: %trace_tea.EventAPI
  %9 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %10 = load i32, i32* %9, align 4
  %11 = call i32 @filter_IncludeEvent(i32 %10)
  ; Expression result: %11
  ; Expression result: %err
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %12
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %14 = call i32 @trace_tea_StartWithFilter(i32 %13, i32 %filter)
  ; Expression result: %14
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %16 = call i32 @assert_eq_string(i32 %err, i32 %15)
  ; Expression result: %16
  %17 = call i32 @trace_tea_Stop()
  ; Expression result: %17
  ret i32 0
}

define i8* @test_real_time_analyzer() {
entry:
  %0 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.18, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  ; Expression result: %analyzer
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  %3 = call i32 @trace_tea_NewRealTimeAnalyzer()
  ; Expression result: %3
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %5
  ; Expression result: %duration
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %9
  ; Expression result: 0
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %10
  ret i32 0
}

define i8* @test_visualizer() {
entry:
  %0 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.19, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %3 = call i32 @trace_tea_Start(i32 %2)
  ; Expression result: %3
  ; Expression result: %ctx
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %4
  %5 = call i32 @vibe_context_Background()
  ; Expression result: %5
  ; Expression result: %ctx
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %6
  ; Expression result: %task
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %7
  %8 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.20, i64 0, i64 0
  %9 = call i32 @trace_tea_NewTask(i32 %ctx, i32 %8)
  ; Expression result: %9
  ; Member access: %trace_tea.EventAPI
  %10 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %11 = load i32, i32* %10, align 4
  %12 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.21, i64 0, i64 0
  %13 = call i32 @trace_tea_Log(i32 %ctx, i32 %11, i32 %12)
  ; Expression result: %13
  %14 = call i32 @task_End()
  ; Expression result: %14
  ; Expression result: %traceData
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %15
  %16 = call i32 @trace_tea_GetTraceBuffer()
  ; Expression result: %16
  %17 = call i32 @trace_tea_Stop()
  ; Expression result: %17
  ; Expression result: %visualizer
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %18
  %19 = call i32 @trace_tea_NewVisualizer(i32 %traceData)
  ; Expression result: %19
  ; Expression result: %timeline
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  %21 = call i32 @visualizer_GenerateTimeline()
  ; Expression result: %21
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %22
  ; Expression result: 0
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %24
  ; Expression result: 0
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %25
  ret i32 0
}

define i8* @test_metrics_extraction() {
entry:
  %0 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.22, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %3 = call i32 @trace_tea_Start(i32 %2)
  ; Expression result: %3
  ; Expression result: %ctx
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %4
  %5 = call i32 @vibe_context_Background()
  ; Expression result: %5
  ; Member access: %trace_tea.EventAPI
  %6 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %7 = load i32, i32* %6, align 4
  %8 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.10, i64 0, i64 0
  %9 = call i32 @trace_tea_Logf(i32 %ctx, i32 %7, i32 %8, i32 1)
  ; Expression result: %9
  ; Member access: %trace_tea.EventDatabase
  %10 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %11 = load i32, i32* %10, align 4
  %12 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.23, i64 0, i64 0
  %13 = call i32 @trace_tea_Logf(i32 %ctx, i32 %11, i32 %12)
  ; Expression result: %13
  ; Expression result: %traceData
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  %15 = call i32 @trace_tea_GetTraceBuffer()
  ; Expression result: %15
  %16 = call i32 @trace_tea_Stop()
  ; Expression result: %16
  ; Expression result: %metrics
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = call i32 @trace_tea_ExtractMetrics(i32 %traceData)
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %19
  ; Expression result: 0
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %21
  ; Expression result: 0
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  ; Expression result: 0
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %24
  ret i32 0
}

define i32 @test_event_categories() {
entry:
  %0 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.24, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  ; Member access: %trace_tea.EventGoroutine
  %2 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %3 = load i32, i32* %2, align 4
  %4 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.25, i64 0, i64 0
  %5 = call i32 @assert_eq_string(i32 %3, i32 %4)
  ; Expression result: %5
  ; Member access: %trace_tea.EventNet
  %6 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %7 = load i32, i32* %6, align 4
  %8 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.26, i64 0, i64 0
  %9 = call i32 @assert_eq_string(i32 %7, i32 %8)
  ; Expression result: %9
  ; Member access: %trace_tea.EventSyscall
  %10 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %11 = load i32, i32* %10, align 4
  %12 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.27, i64 0, i64 0
  %13 = call i32 @assert_eq_string(i32 %11, i32 %12)
  ; Expression result: %13
  ; Member access: %trace_tea.EventMemory
  %14 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %15 = load i32, i32* %14, align 4
  %16 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.28, i64 0, i64 0
  %17 = call i32 @assert_eq_string(i32 %15, i32 %16)
  ; Expression result: %17
  ; Member access: %trace_tea.EventCPUSample
  %18 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %19 = load i32, i32* %18, align 4
  %20 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.29, i64 0, i64 0
  %21 = call i32 @assert_eq_string(i32 %19, i32 %20)
  ; Expression result: %21
  ; Member access: %trace_tea.EventConcurrency
  %22 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %23 = load i32, i32* %22, align 4
  %24 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.30, i64 0, i64 0
  %25 = call i32 @assert_eq_string(i32 %23, i32 %24)
  ; Expression result: %25
  ; Member access: %trace_tea.EventGC
  %26 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %27 = load i32, i32* %26, align 4
  %28 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.31, i64 0, i64 0
  %29 = call i32 @assert_eq_string(i32 %27, i32 %28)
  ; Expression result: %29
  ; Member access: %trace_tea.EventBlock
  %30 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %31 = load i32, i32* %30, align 4
  %32 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.32, i64 0, i64 0
  %33 = call i32 @assert_eq_string(i32 %31, i32 %32)
  ; Expression result: %33
  ; Member access: %trace_tea.EventUserDefined
  %34 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %35 = load i32, i32* %34, align 4
  %36 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.33, i64 0, i64 0
  %37 = call i32 @assert_eq_string(i32 %35, i32 %36)
  ; Expression result: %37
  ; Member access: %trace_tea.EventAPI
  %38 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %39 = load i32, i32* %38, align 4
  %40 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.34, i64 0, i64 0
  %41 = call i32 @assert_eq_string(i32 %39, i32 %40)
  ; Expression result: %41
  ; Member access: %trace_tea.EventDatabase
  %42 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %43 = load i32, i32* %42, align 4
  %44 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.35, i64 0, i64 0
  %45 = call i32 @assert_eq_string(i32 %43, i32 %44)
  ; Expression result: %45
  ; Member access: %trace_tea.EventCache
  %46 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %47 = load i32, i32* %46, align 4
  %48 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.36, i64 0, i64 0
  %49 = call i32 @assert_eq_string(i32 %47, i32 %48)
  ; Expression result: %49
  ; Member access: %trace_tea.EventFile
  %50 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %51 = load i32, i32* %50, align 4
  %52 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.37, i64 0, i64 0
  %53 = call i32 @assert_eq_string(i32 %51, i32 %52)
  ; Expression result: %53
  ; Member access: %trace_tea.EventCompute
  %54 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %55 = load i32, i32* %54, align 4
  %56 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.38, i64 0, i64 0
  %57 = call i32 @assert_eq_string(i32 %55, i32 %56)
  ; Expression result: %57
  ; Member access: %trace_tea.EventAsyncWork
  %58 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %59 = load i32, i32* %58, align 4
  %60 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.39, i64 0, i64 0
  %61 = call i32 @assert_eq_string(i32 %59, i32 %60)
  ; Expression result: %61
  ; Member access: %trace_tea.EventNetwork
  %62 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %63 = load i32, i32* %62, align 4
  %64 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.40, i64 0, i64 0
  %65 = call i32 @assert_eq_string(i32 %63, i32 %64)
  ; Expression result: %65
  ; Member access: %trace_tea.EventRender
  %66 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %67 = load i32, i32* %66, align 4
  %68 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.41, i64 0, i64 0
  %69 = call i32 @assert_eq_string(i32 %67, i32 %68)
  ; Expression result: %69
  ; Member access: %trace_tea.EventLogger
  %70 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %71 = load i32, i32* %70, align 4
  %72 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.42, i64 0, i64 0
  %73 = call i32 @assert_eq_string(i32 %71, i32 %72)
  ; Expression result: %73
  ; Member access: %trace_tea.EventPerformance
  %74 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %75 = load i32, i32* %74, align 4
  %76 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.43, i64 0, i64 0
  %77 = call i32 @assert_eq_string(i32 %75, i32 %76)
  ; Expression result: %77
  ret i32 0
}

define i32 @test_buffer_management() {
entry:
  %0 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.44, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %3 = call i32 @trace_tea_Start(i32 %2)
  ; Expression result: %3
  ; Expression result: %ctx
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %4
  %5 = call i32 @vibe_context_Background()
  ; Expression result: %5
  ; Member access: %trace_tea.EventUserDefined
  %6 = getelementptr inbounds %struct.object, %struct.object* %trace_tea, i32 0, i32 0
  %7 = load i32, i32* %6, align 4
  %8 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.45, i64 0, i64 0
  %9 = call i32 @trace_tea_Log(i32 %ctx, i32 %7, i32 %8)
  ; Expression result: %9
  ; Expression result: %buffer
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %10
  %11 = call i32 @trace_tea_GetTraceBuffer()
  ; Expression result: %11
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %12
  ; Expression result: 0
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %13
  %14 = call i32 @trace_tea_ClearTraceBuffer()
  ; Expression result: %14
  ; Expression result: %buffer
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %15
  %16 = call i32 @trace_tea_GetTraceBuffer()
  ; Expression result: %16
  %17 = call i32 @len(i32 %buffer)
  %18 = call i32 @assert_eq_int(i32 %17, i32 0)
  ; Expression result: %18
  %19 = call i32 @trace_tea_Stop()
  ; Expression result: %19
  ret i32 0
}

define i32 @test_error_handling() {
entry:
  %0 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.46, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  ; Expression result: %err1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %4 = call i32 @trace_tea_Start(i32 %3)
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %6 = call i32 @assert_eq_string(i32 %err1, i32 %5)
  ; Expression result: %6
  ; Expression result: %err2
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %9 = call i32 @trace_tea_Start(i32 %8)
  ; Expression result: %9
  %10 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.47, i64 0, i64 0
  %11 = call i32 @assert_eq_string(i32 %err2, i32 %10)
  ; Expression result: %11
  %12 = call i32 @trace_tea_Stop()
  ; Expression result: %12
  ; Expression result: %err3
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %13
  %14 = call i32 @trace_tea_Stop()
  ; Expression result: %14
  %15 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.48, i64 0, i64 0
  %16 = call i32 @assert_eq_string(i32 %err3, i32 %15)
  ; Expression result: %16
  ret i32 0
}


; String constants
@.str.17 = private unnamed_addr constant [9 x i8] c"worker-*\00", align 1
@.str.25 = private unnamed_addr constant [10 x i8] c"goroutine\00", align 1
@.str.40 = private unnamed_addr constant [8 x i8] c"network\00", align 1
@.str.26 = private unnamed_addr constant [4 x i8] c"net\00", align 1
@.str.10 = private unnamed_addr constant [12 x i8] c"API call %d\00", align 1
@.str.11 = private unnamed_addr constant [9 x i8] c"db-query\00", align 1
@.str.37 = private unnamed_addr constant [5 x i8] c"file\00", align 1
@.str.44 = private unnamed_addr constant [18 x i8] c"Buffer management\00", align 1
@.str.32 = private unnamed_addr constant [6 x i8] c"block\00", align 1
@.str.4 = private unnamed_addr constant [21 x i8] c"Region functionality\00", align 1
@.str.19 = private unnamed_addr constant [11 x i8] c"Visualizer\00", align 1
@.str.30 = private unnamed_addr constant [12 x i8] c"concurrency\00", align 1
@.str.29 = private unnamed_addr constant [11 x i8] c"cpu-sample\00", align 1
@.str.0 = private unnamed_addr constant [14 x i8] c"Basic tracing\00", align 1
@.str.43 = private unnamed_addr constant [12 x i8] c"performance\00", align 1
@.str.18 = private unnamed_addr constant [19 x i8] c"Real-time analyzer\00", align 1
@.str.24 = private unnamed_addr constant [17 x i8] c"Event categories\00", align 1
@.str.9 = private unnamed_addr constant [13 x i8] c"Test message\00", align 1
@.str.39 = private unnamed_addr constant [6 x i8] c"async\00", align 1
@.str.33 = private unnamed_addr constant [5 x i8] c"user\00", align 1
@.str.7 = private unnamed_addr constant [16 x i8] c"Region log test\00", align 1
@.str.14 = private unnamed_addr constant [16 x i8] c"WithSpan helper\00", align 1
@.str.21 = private unnamed_addr constant [9 x i8] c"API call\00", align 1
@.str.46 = private unnamed_addr constant [15 x i8] c"Error handling\00", align 1
@.str.34 = private unnamed_addr constant [4 x i8] c"api\00", align 1
@.str.15 = private unnamed_addr constant [17 x i8] c"Computation work\00", align 1
@.str.3 = private unnamed_addr constant [10 x i8] c"test-task\00", align 1
@.str.6 = private unnamed_addr constant [12 x i8] c"test-region\00", align 1
@.str.23 = private unnamed_addr constant [15 x i8] c"Database query\00", align 1
@.str.5 = private unnamed_addr constant [12 x i8] c"region-test\00", align 1
@.str.36 = private unnamed_addr constant [6 x i8] c"cache\00", align 1
@.str.8 = private unnamed_addr constant [14 x i8] c"Event logging\00", align 1
@.str.47 = private unnamed_addr constant [23 x i8] c"Tracing already active\00", align 1
@.str.27 = private unnamed_addr constant [8 x i8] c"syscall\00", align 1
@.str.28 = private unnamed_addr constant [7 x i8] c"memory\00", align 1
@.str.2 = private unnamed_addr constant [16 x i8] c"Task management\00", align 1
@.str.22 = private unnamed_addr constant [19 x i8] c"Metrics extraction\00", align 1
@.str.12 = private unnamed_addr constant [24 x i8] c"Database query executed\00", align 1
@.str.35 = private unnamed_addr constant [9 x i8] c"database\00", align 1
@.str.1 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.13 = private unnamed_addr constant [18 x i8] c"WithRegion helper\00", align 1
@.str.38 = private unnamed_addr constant [8 x i8] c"compute\00", align 1
@.str.42 = private unnamed_addr constant [7 x i8] c"logger\00", align 1
@.str.45 = private unnamed_addr constant [11 x i8] c"Test event\00", align 1
@.str.20 = private unnamed_addr constant [9 x i8] c"viz-task\00", align 1
@.str.41 = private unnamed_addr constant [7 x i8] c"render\00", align 1
@.str.48 = private unnamed_addr constant [19 x i8] c"Tracing not active\00", align 1
@.str.31 = private unnamed_addr constant [3 x i8] c"gc\00", align 1
@.str.16 = private unnamed_addr constant [21 x i8] c"Filter functionality\00", align 1
define i32 @main() {
entry:
  %0 = call i32 @test_basic_tracing()
  ; Expression result: %0
  %1 = call i32 @test_task_management()
  ; Expression result: %1
  %2 = call i32 @test_region_functionality()
  ; Expression result: %2
  %3 = call i32 @test_event_logging()
  ; Expression result: %3
  %4 = call i32 @test_with_region()
  ; Expression result: %4
  %5 = call i32 @test_with_span()
  ; Expression result: %5
  %6 = call i32 @test_filter_functionality()
  ; Expression result: %6
  %7 = call i32 @test_real_time_analyzer()
  ; Expression result: %7
  %8 = call i32 @test_visualizer()
  ; Expression result: %8
  %9 = call i32 @test_metrics_extraction()
  ; Expression result: %9
  %10 = call i32 @test_event_categories()
  ; Expression result: %10
  %11 = call i32 @test_buffer_management()
  ; Expression result: %11
  %12 = call i32 @test_error_handling()
  ; Expression result: %12
  %13 = call i32 @print_test_summary()
  ; Expression result: %13
  ret i32 0
}

