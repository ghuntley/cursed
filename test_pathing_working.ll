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

define i8* @path_join(i32 %parts) {
entry:
  ; Expression result: %skit
  %0 = call i32 @len(i32 %parts)
  %1 = add i32 0, 0 ; placeholder
  %2 = icmp eq i32 %0, %1
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  ; Expression result: %3
  %4 = add i32 0, 0 ; placeholder
  ; Expression result: %4
  %5 = add i32 0, 0 ; placeholder
  ; Expression result: %5
  ret i32 0
}

define i32 @len(i32 %arr) {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = alloca i32, align 4
  store i32 %0, i32* %1, align 4
  ; Variable count allocated
  %4 = load i32, i32* %3, align 4
  %5 = add i32 0, 0 ; placeholder
  %6 = icmp slt i32 %4, %5
  %16 = add i32 0, 0 ; placeholder
  %2 = add i32 0, 0 ; placeholder
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Short declaration: i := %2
  br label %label0
label0:
  br i1 %6, label %label1, label %label3
label1:
  ; Expression result: %skit
  %7 = load i32, i32* %3, align 4
  %8 = add i32 0, 0 ; placeholder
  %9 = icmp slt i32 %7, %8
  ; Expression result: %9
  %10 = add i32 0, 0 ; placeholder
  ; Expression result: %10
  %11 = load i32, i32* %1, align 4
  ; Expression result: %11
  %12 = add i32 0, 0 ; placeholder
  ; Expression result: %12
  %13 = load i32, i32* %1, align 4
  %14 = add i32 0, 0 ; placeholder
  %15 = add i32 %13, %14
  ; Expression result: %15
  br label %label2
label2:
  br label %label0
label3:
  %17 = add i32 0, 0 ; placeholder
  ; Expression result: %17
  %18 = add i32 0, 0 ; placeholder
  ; Expression result: %18
  %19 = add i32 0, 0 ; placeholder
  ; Expression result: %19
  ret i32 0
}



; String constants
@.str.6 = private unnamed_addr constant [20 x i8] c"Created parts array\00", align 1
@.str.4 = private unnamed_addr constant [2 x i8] c"/\00", align 1
@.str.9 = private unnamed_addr constant [37 x i8] c"Pathing test completed successfully!\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.0 = private unnamed_addr constant [29 x i8] c"Testing pathing functions...\00", align 1
@.str.3 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.7 = private unnamed_addr constant [2 x i8] c"1\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c"0\00", align 1
@.str.5 = private unnamed_addr constant [21 x i8] c"Testing path_join...\00", align 1
@.str.8 = private unnamed_addr constant [2 x i8] c"2\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.0, i64 0, i64 0
  ; Converting complex expression to output
  %1 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %2 = call i32 (i8*, ...) @printf(i8* %1, i32 %0)
  %3 = alloca i8*, align 4
  store i8* %parts, i8** %3, align 4
  ; Variable result allocated at %3
  %4 = alloca [1 x i32], align 4
  %5 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.2, i64 0, i64 0
  %6 = getelementptr inbounds [1 x i32], [1 x i32]* %4, i64 0, i64 0
  store i32 %5, i32* %6, align 4
  %7 = alloca i32, align 4
  store i32 1, i32* %7, align 4
  ; Short declaration: i := 1 (i32)
  br label %label0
label0:
  %8 = load i32, i32* %7, align 4
  %9 = call i32 @len(i32 %parts)
  %10 = icmp slt i32 %8, %9
  br i1 %10, label %label1, label %label3
label1:
  %11 = load i32, i32* %3, align 4
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %13 = load i32, i32* %3, align 4
  %14 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.4, i64 0, i64 0
  %15 = add i32 %13, %14
  %16 = add i32 %15, %parts
  %17 = inttoptr i64 0 to [0 x i32]*
  br label %label2
label2:
  %18 = load i32, i32* %7, align 4
  %19 = add i32 %18, 1
  store i32 %19, i32* %7, align 4
  br label %label0
label3:
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %21 = load i32, i32* %3, align 4
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %26 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.5, i64 0, i64 0
  ; Converting complex expression to output
  %27 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %28 = call i32 (i8*, ...) @printf(i8* %27, i32 %26)
  %29 = inttoptr i64 0 to [0 x i32]*
  %30 = alloca i32, align 4
  store i32 %29, i32* %30, align 4
  ; Variable parts allocated at %30
  %31 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.6, i64 0, i64 0
  ; Converting complex expression to output
  %32 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %33 = call i32 (i8*, ...) @printf(i8* %32, i32 %31)
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %35 = load i32, i32* %30, align 4
  %36 = alloca [1 x i32], align 4
  %37 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.2, i64 0, i64 0
  %38 = getelementptr inbounds [1 x i32], [1 x i32]* %36, i64 0, i64 0
  store i32 %37, i32* %38, align 4
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %40 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %41 = load i32, i32* %30, align 4
  %42 = alloca [1 x i32], align 4
  %43 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.7, i64 0, i64 0
  %44 = getelementptr inbounds [1 x i32], [1 x i32]* %42, i64 0, i64 0
  store i32 %43, i32* %44, align 4
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %46 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %47 = load i32, i32* %30, align 4
  %48 = alloca [1 x i32], align 4
  %49 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.8, i64 0, i64 0
  %50 = getelementptr inbounds [1 x i32], [1 x i32]* %48, i64 0, i64 0
  store i32 %49, i32* %50, align 4
  %51 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %52 = load i32, i32* %30, align 4
  %53 = call i32 @path_join(i32 %52)
  %54 = alloca i8*, align 4
  store i8* %53, i8** %54, align 4
  ; Variable result allocated at %54
  %55 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %56 = load i32, i32* %54, align 4
  %57 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %58 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.9, i64 0, i64 0
  ; Converting complex expression to output
  %59 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %60 = call i32 (i8*, ...) @printf(i8* %59, i32 %58)
  ret i32 0
}
