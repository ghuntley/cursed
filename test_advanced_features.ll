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

define i32 @advancedTest() {
entry:
  %0 = alloca [5x i32], align 4
  %1 = getelementptr inbounds [5x i32], [5x i32]* %0, i64 0, i64 0
  store i32 1, i32* %1, align 4
  %2 = getelementptr inbounds [5x i32], [5x i32]* %0, i64 0, i64 1
  store i32 2, i32* %2, align 4
  %3 = getelementptr inbounds [5x i32], [5x i32]* %0, i64 0, i64 2
  store i32 3, i32* %3, align 4
  %4 = getelementptr inbounds [5x i32], [5x i32]* %0, i64 0, i64 3
  store i32 4, i32* %4, align 4
  %5 = getelementptr inbounds [5x i32], [5x i32]* %0, i64 0, i64 4
  store i32 5, i32* %5, align 4
  %6 = alloca [5 x i32]*, align 4
  store [5 x i32]* %0, [5 x i32]** %6, align 4
  ; Variable numbers allocated
  %7 = load [5 x i32]*, [5 x i32]** %6, align 4
  %8 = zext i32 0 to i64
  %9 = getelementptr inbounds [5 x i32], [5 x i32]* %7, i64 0, i64 %8
  %10 = load i32, i32* %9, align 4
  %11 = alloca i32, align 4
  store i32 %10, i32* %11, align 4
  ; Variable first allocated
  %12 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.0, i64 0, i64 0
  %13 = call i32 @puts(i8* %12)
  %14 = add i32 0, 0
  ; Expression result: %14
  %15 = load i32, i32* %11, align 4
  %16 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %17 = call i32 (i8*, ...) @printf(i8* %16, i32 %15)
  %18 = add i32 0, 0
  ; Expression result: %18
  %19 = load [5 x i32]*, [5 x i32]** %6, align 4
  ; for-in loop implementation
  %20 = alloca i32, align 4
  store i32 0, i32* %20, align 4
  %21 = alloca i32, align 4
  br label %label0
label0:
  %22 = load i32, i32* %20, align 4
  %23 = icmp slt i32 %22, 5
  br i1 %23, label %label1, label %label2
label1:
  %24 = load i32, i32* %20, align 4
  %25 = zext i32 %24 to i64
  %26 = getelementptr inbounds [5 x i32], [5 x i32]* %19, i64 0, i64 %25
  %27 = load i32, i32* %26, align 4
  store i32 %27, i32* %21, align 4
  %28 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.2, i64 0, i64 0
  %29 = call i32 @puts(i8* %28)
  %30 = add i32 0, 0
  ; Expression result: %30
  %31 = load i32, i32* %21, align 4
  %32 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %33 = call i32 (i8*, ...) @printf(i8* %32, i32 %31)
  %34 = add i32 0, 0
  ; Expression result: %34
  %35 = load i32, i32* %20, align 4
  %36 = add i32 %35, 1
  store i32 %36, i32* %20, align 4
  br label %label0
label2:
  %37 = alloca i8, align 4
  store i8 65, i8* %37, align 4
  ; Variable letter allocated
  %38 = load i8, i8* %37, align 4
  %39 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.3, i64 0, i64 0
  %40 = call i8* @char_to_string(i8 %38)
  %41 = call i8* @string_concat(i8* %40, i8* %39)
  %42 = alloca i8*, align 4
  store i8* %41, i8** %42, align 4
  ; Variable word allocated
  %43 = load i8*, i8** %42, align 4
  %44 = call i32 @puts(i8* %43)
  %45 = add i32 0, 0
  ; Expression result: %45
  %46 = alloca i32, align 4
  store i32 10, i32* %46, align 4
  ; Variable integer allocated
  %47 = alloca double, align 4
  store double 3.14, double* %47, align 4
  ; Variable float allocated
  %48 = load i32, i32* %46, align 4
  %49 = load double, double* %47, align 4
  %50 = sitofp i32 %48 to double
  %51 = fmul double %50, %49
  %52 = alloca double, align 4
  store double %51, double* %52, align 4
  ; Variable result allocated
  %53 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.4, i64 0, i64 0
  %54 = call i32 @puts(i8* %53)
  %55 = add i32 0, 0
  ; Expression result: %55
  %56 = load double, double* %52, align 4
  %57 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.5, i64 0, i64 0
  %58 = call i32 (i8*, ...) @printf(i8* %57, double %56)
  %59 = add i32 0, 0
  ; Expression result: %59
  %60 = alloca {i32, i32, i32}, align 4
  %61 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %60, i32 0, i32 0
  store i32 5, i32* %61, align 4
  %62 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %60, i32 0, i32 1
  store i32 10, i32* %62, align 4
  %63 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %60, i32 0, i32 2
  store i32 15, i32* %63, align 4
  %64 = alloca {i32, i32, i32}*, align 4
  store {i32, i32, i32}* %60, {i32, i32, i32}** %64, align 4
  ; Variable coords allocated
  %65 = load {i32, i32, i32}*, {i32, i32, i32}** %64, align 4
  %66 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %65, i32 0, i32 0
  %67 = load i32, i32* %66, align 4
  %68 = alloca i32, align 4
  store i32 %67, i32* %68, align 4
  ; Variable x allocated
  %69 = load {i32, i32, i32}*, {i32, i32, i32}** %64, align 4
  %70 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %69, i32 0, i32 1
  %71 = load i32, i32* %70, align 4
  %72 = alloca i32, align 4
  store i32 %71, i32* %72, align 4
  ; Variable y allocated
  %73 = load {i32, i32, i32}*, {i32, i32, i32}** %64, align 4
  %74 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %73, i32 0, i32 2
  %75 = load i32, i32* %74, align 4
  %76 = alloca i32, align 4
  store i32 %75, i32* %76, align 4
  ; Variable z allocated
  %77 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.6, i64 0, i64 0
  %78 = call i32 @puts(i8* %77)
  %79 = add i32 0, 0
  ; Expression result: %79
  %80 = load i32, i32* %68, align 4
  %81 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %82 = call i32 (i8*, ...) @printf(i8* %81, i32 %80)
  %83 = add i32 0, 0
  ; Expression result: %83
  %84 = load i32, i32* %72, align 4
  %85 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %86 = call i32 (i8*, ...) @printf(i8* %85, i32 %84)
  %87 = add i32 0, 0
  ; Expression result: %87
  %88 = load i32, i32* %76, align 4
  %89 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %90 = call i32 (i8*, ...) @printf(i8* %89, i32 %88)
  %91 = add i32 0, 0
  ; Expression result: %91
  %92 = alloca i1, align 4
  store i1 1, i1* %92, align 4
  ; Variable flag1 allocated
  %93 = alloca i1, align 4
  store i1 0, i1* %93, align 4
  ; Variable flag2 allocated
  %94 = load i1, i1* %92, align 4
  %95 = icmp eq i1 %94, 1
  br i1 %95, label %label3, label %label4
label3:
  %96 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.7, i64 0, i64 0
  %97 = call i32 @puts(i8* %96)
  %98 = add i32 0, 0
  ; Expression result: %98
  br label %label5
label4:
  br label %label5
label5:
  %99 = icmp sgt i32 %MAX_VALUE, 50
  br i1 %99, label %label6, label %label7
label6:
  %100 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.8, i64 0, i64 0
  %101 = call i32 @puts(i8* %100)
  %102 = add i32 0, 0
  ; Expression result: %102
  br label %label8
label7:
  br label %label8
label8:
  ret i32 42
}


; String constants
@.str.1 = private unnamed_addr constant [6 x i8] c"%d\0A\00", align 1
@.str.8 = private unnamed_addr constant [25 x i8] c"Constants work properly!\00", align 1
@.str.4 = private unnamed_addr constant [25 x i8] c"Mixed arithmetic result:\00", align 1
@.str.0 = private unnamed_addr constant [14 x i8] c"First number:\00", align 1
@.str.2 = private unnamed_addr constant [19 x i8] c"Processing number:\00", align 1
@.str.10 = private unnamed_addr constant [28 x i8] c"Test completed with result:\00", align 1
@.str.3 = private unnamed_addr constant [8 x i8] c"dvanced\00", align 1
@.str.7 = private unnamed_addr constant [21 x i8] c"Boolean test passed!\00", align 1
@.str.6 = private unnamed_addr constant [14 x i8] c"Tuple values:\00", align 1
@.str.5 = private unnamed_addr constant [6 x i8] c"%f\0A\00", align 1
@.str.9 = private unnamed_addr constant [36 x i8] c"Testing advanced CURSED features...\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.9, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = call i32 @advancedTest()
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable testResult allocated
  %5 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.10, i64 0, i64 0
  %6 = call i32 @puts(i8* %5)
  %7 = add i32 0, 0
  ; Expression result: %7
  %8 = load i32, i32* %4, align 4
  %9 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.1, i64 0, i64 0
  %10 = call i32 (i8*, ...) @printf(i8* %9, i32 %8)
  %11 = add i32 0, 0
  ; Expression result: %11
  ret i32 0
}

