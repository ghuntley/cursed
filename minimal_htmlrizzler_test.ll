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

define i32 @find_char(i8* %str, i8 %ch) {
entry:
  %1 = load i32, i32* %0, align 4
  %2 = call i32 @len(i8* %str)
  %3 = icmp slt i32 %1, %2
  %10 = add i32 1, 0 ; increment placeholder
  %0 = alloca i32, align 4
  store i32 0, i32* %0, align 4
  ; Short declaration: i := 0
  br label %label0
label0:
  br i1 %3, label %label1, label %label3
label1:
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %4
  ; Expression result: %str
  %5 = alloca [0x i32], align 4
  %6 = icmp eq i32 %5, %ch
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %8
  %9 = load i32, i32* %0, align 4
  ; Expression result: %9
  br label %label2
label2:
  br label %label0
label3:
  ret i32 0
}



; String constants
@.str.0 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.4 = private unnamed_addr constant [12 x i8] c"Hello World\00", align 1
@.str.2 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.5 = private unnamed_addr constant [29 x i8] c"Basic CURSED test completed!\00", align 1
@.str.3 = private unnamed_addr constant [4 x i8] c"div\00", align 1
@.str.1 = private unnamed_addr constant [38 x i8] c"Testing basic CURSED functionality...\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.1, i64 0, i64 0
  ; Converting complex expression to output
  %1 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %2 = call i32 (i8*, ...) @printf(i8* %1, i32 %0)
  %3 = alloca %struct.struct, align 4
  store %struct.struct null, %struct.struct* %3, align 4
  ; Variable element allocated at %3
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %9 = load i32, i32* %3, align 4
  ; Member access: %9.tag
  %10 = getelementptr inbounds %struct.object, %struct.object* %9, i32 0, i32 0
  %11 = load i32, i32* %10, align 4
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %13 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %14 = load i32, i32* %3, align 4
  ; Member access: %14.content
  %15 = getelementptr inbounds %struct.object, %struct.object* %14, i32 0, i32 0
  %16 = load i32, i32* %15, align 4
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %18 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.4, i64 0, i64 0
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %20 = load i32, i32* %3, align 4
  ; Member access: %20.tag
  %21 = getelementptr inbounds %struct.object, %struct.object* %20, i32 0, i32 0
  %22 = load i32, i32* %21, align 4
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %25 = load i32, i32* %3, align 4
  ; Member access: %25.content
  %26 = getelementptr inbounds %struct.object, %struct.object* %25, i32 0, i32 0
  %27 = load i32, i32* %26, align 4
  %28 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %30 = sub i32 %29, 1
  %31 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %32 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.4, i64 0, i64 0
  %33 = alloca i8*, align 4
  store i8* %32, i8** %33, align 4
  ; Variable test_str allocated at %33
  %34 = load i32, i32* %33, align 4
  %35 = call i32 @find_char(i32 %34, i32 87)
  %36 = alloca i32, align 4
  store i32 %35, i32* %36, align 4
  ; Variable pos allocated at %36
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %40 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.5, i64 0, i64 0
  ; Converting complex expression to output
  %41 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %42 = call i32 (i8*, ...) @printf(i8* %41, i32 %40)
  ret i32 0
}
