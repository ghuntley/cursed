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

define i8* @make_request(i8* %param1, i8* %param2) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.1, i64 0, i64 0
  %2 = add i32 %1, %param1
  %3 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.2, i64 0, i64 0
  %4 = add i32 %2, %3
  %5 = add i32 %4, %param2
  %6 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.3, i64 0, i64 0
  %7 = add i32 %5, %6
  ; Expression result: %7
  ret i32 0
}

define i8* @make_response(i8* %param1, i8* %param2) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.1, i64 0, i64 0
  %2 = add i32 %1, %param1
  %3 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.4, i64 0, i64 0
  %4 = add i32 %2, %3
  %5 = add i32 %4, %param2
  %6 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.3, i64 0, i64 0
  %7 = add i32 %5, %6
  ; Expression result: %7
  ret i32 0
}

define i8* @run_method(i8* %param1) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.5, i64 0, i64 0
  %2 = icmp eq i32 %param1, %1
  ; Expression result: %2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %3
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %4
  %5 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.6, i64 0, i64 0
  ; Expression result: %5
  ret i32 0
}

define i32 @register_method(i8* %param1) {
entry:
  ; Expression result: %registered
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  ; Expression result: %param1
  ret i32 0
}

define i1 @is_registered(i8* %param1) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = icmp eq i32 %registered, %param1
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %2
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %3
  ; Expression result: 1
  ret i32 0
}

define i32 @init_count() {
entry:
  ; Expression result: %count
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  ; Expression result: 0
  ret i32 0
}

define i32 @increment_count() {
entry:
  ; Expression result: %count
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = add i32 %count, 1
  ; Expression result: %1
  ret i32 0
}

define i32 @get_count() {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  ; Expression result: %count
  ret i32 0
}



; String constants
@.str.3 = private unnamed_addr constant [3 x i8] c"\"}\00", align 1
@.str.2 = private unnamed_addr constant [13 x i8] c"\",\"method\":\"\00", align 1
@.str.7 = private unnamed_addr constant [4 x i8] c"add\00", align 1
@.str.8 = private unnamed_addr constant [3 x i8] c"42\00", align 1
@.str.19 = private unnamed_addr constant [30 x i8] c"✅ JSON-RPC message creation\00", align 1
@.str.21 = private unnamed_addr constant [21 x i8] c"✅ Method execution\00", align 1
@.str.0 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.4 = private unnamed_addr constant [13 x i8] c"\",\"result\":\"\00", align 1
@.str.13 = private unnamed_addr constant [8 x i8] c"test_id\00", align 1
@.str.18 = private unnamed_addr constant [31 x i8] c"🎉 RPC Vibes module working!\00", align 1
@.str.1 = private unnamed_addr constant [24 x i8] c"{\"jsonrpc\":\"2.0\",\"id\":\"\00", align 1
@.str.5 = private unnamed_addr constant [5 x i8] c"ping\00", align 1
@.str.20 = private unnamed_addr constant [24 x i8] c"✅ Method registration\00", align 1
@.str.22 = private unnamed_addr constant [24 x i8] c"✅ Statistics tracking\00", align 1
@.str.6 = private unnamed_addr constant [5 x i8] c"pong\00", align 1
@.str.16 = private unnamed_addr constant [20 x i8] c"✅ Statistics work\00", align 1
@.str.15 = private unnamed_addr constant [31 x i8] c"❌ Method registration failed\00", align 1
@.str.10 = private unnamed_addr constant [28 x i8] c"🚀 RPC Vibes - Final Test\00", align 1
@.str.12 = private unnamed_addr constant [25 x i8] c"========================\00", align 1
@.str.9 = private unnamed_addr constant [6 x i8] c"error\00", align 1
@.str.11 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.14 = private unnamed_addr constant [30 x i8] c"✅ Method registration works\00", align 1
@.str.17 = private unnamed_addr constant [22 x i8] c"❌ Statistics failed\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %1 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %2 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.7, i64 0, i64 0
  %3 = icmp eq i32 %param1, %2
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %6 = getelementptr inbounds [3 x i8], [3 x i8]* @.str.8, i64 0, i64 0
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %9 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.9, i64 0, i64 0
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %12 = alloca i8*, align 4
  store i8* %11, i8** %12, align 4
  ; Variable registered allocated at %12
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %16 = alloca i32, align 4
  store i32 0, i32* %16, align 4
  ; Variable count allocated at %16
  %17 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.10, i64 0, i64 0
  ; Converting complex expression to output
  %18 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.11, i64 0, i64 0
  %19 = call i32 (i8*, ...) @printf(i8* %18, i32 %17)
  %20 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.12, i64 0, i64 0
  ; Converting complex expression to output
  %21 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.11, i64 0, i64 0
  %22 = call i32 (i8*, ...) @printf(i8* %21, i32 %20)
  %23 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.13, i64 0, i64 0
  %24 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.5, i64 0, i64 0
  %25 = call i32 @make_request(i32 %23, i32 %24)
  %26 = alloca i8*, align 4
  store i8* %25, i8** %26, align 4
  ; Variable req allocated at %26
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %28 = load i32, i32* %26, align 4
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %30 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.13, i64 0, i64 0
  %31 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.6, i64 0, i64 0
  %32 = call i32 @make_response(i32 %30, i32 %31)
  %33 = alloca i8*, align 4
  store i8* %32, i8** %33, align 4
  ; Variable resp allocated at %33
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %35 = load i32, i32* %33, align 4
  %36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %37 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.5, i64 0, i64 0
  %38 = call i32 @run_method(i32 %37)
  %39 = alloca i8*, align 4
  store i8* %38, i8** %39, align 4
  ; Variable result allocated at %39
  %40 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %41 = load i32, i32* %39, align 4
  %42 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %43 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.5, i64 0, i64 0
  %44 = call i32 @register_method(i32 %43)
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %46 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.5, i64 0, i64 0
  %47 = call i32 @is_registered(i32 %46)
  %48 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %49 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.14, i64 0, i64 0
  ; Converting complex expression to output
  %50 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.11, i64 0, i64 0
  %51 = call i32 (i8*, ...) @printf(i8* %50, i32 %49)
  %52 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %53 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %54 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %55 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.15, i64 0, i64 0
  ; Converting complex expression to output
  %56 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.11, i64 0, i64 0
  %57 = call i32 (i8*, ...) @printf(i8* %56, i32 %55)
  %58 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %59 = call i32 @init_count()
  %60 = call i32 @increment_count()
  %61 = call i32 @get_count()
  %62 = alloca i32, align 4
  store i32 %61, i32* %62, align 4
  ; Variable current allocated at %62
  %63 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %64 = load i32, i32* %62, align 4
  %65 = icmp eq i32 %64, 1
  %66 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %67 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.16, i64 0, i64 0
  ; Converting complex expression to output
  %68 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.11, i64 0, i64 0
  %69 = call i32 (i8*, ...) @printf(i8* %68, i32 %67)
  %70 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %71 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %72 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %73 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.17, i64 0, i64 0
  ; Converting complex expression to output
  %74 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.11, i64 0, i64 0
  %75 = call i32 (i8*, ...) @printf(i8* %74, i32 %73)
  %76 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %77 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.12, i64 0, i64 0
  ; Converting complex expression to output
  %78 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.11, i64 0, i64 0
  %79 = call i32 (i8*, ...) @printf(i8* %78, i32 %77)
  %80 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.18, i64 0, i64 0
  ; Converting complex expression to output
  %81 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.11, i64 0, i64 0
  %82 = call i32 (i8*, ...) @printf(i8* %81, i32 %80)
  %83 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.19, i64 0, i64 0
  ; Converting complex expression to output
  %84 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.11, i64 0, i64 0
  %85 = call i32 (i8*, ...) @printf(i8* %84, i32 %83)
  %86 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.20, i64 0, i64 0
  ; Converting complex expression to output
  %87 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.11, i64 0, i64 0
  %88 = call i32 (i8*, ...) @printf(i8* %87, i32 %86)
  %89 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.21, i64 0, i64 0
  ; Converting complex expression to output
  %90 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.11, i64 0, i64 0
  %91 = call i32 (i8*, ...) @printf(i8* %90, i32 %89)
  %92 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.22, i64 0, i64 0
  ; Converting complex expression to output
  %93 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.11, i64 0, i64 0
  %94 = call i32 (i8*, ...) @printf(i8* %93, i32 %92)
  %95 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.12, i64 0, i64 0
  ; Converting complex expression to output
  %96 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.11, i64 0, i64 0
  %97 = call i32 (i8*, ...) @printf(i8* %96, i32 %95)
  ret i32 0
}
