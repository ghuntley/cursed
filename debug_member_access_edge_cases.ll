; CURSED Language - Advanced LLVM Compilation
target triple = "x86_64-unknown-linux-gnu"


; Runtime function declarations
declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)
declare i8* @malloc(i64)
declare void @free(i8*)
declare i64 @strlen(i8*)
declare i8* @strcpy(i8*, i8*)

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
@_ZTI11CursedError = constant { i8*, i8* } { i8* null, i8* getelementptr inbounds ([14 x i8], [14 x i8]* @_ZTS11CursedError, i32 0, i32 0) }
@_ZTS11CursedError = constant [14 x i8] c"11CursedError\00"


; String constants
@.str.0 = private unnamed_addr constant [29 x i8] c"Test 1: Simple member access\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.2 = private unnamed_addr constant [38 x i8] c"Test 3: Member access in if statement\00", align 1
@.str.3 = private unnamed_addr constant [9 x i8] c"Test 4: \00", align 1
@.str.4 = private unnamed_addr constant [14 x i8] c"Concatenation\00", align 1
@.str.5 = private unnamed_addr constant [7 x i8] c"Call 1\00", align 1
@.str.6 = private unnamed_addr constant [7 x i8] c"Call 2\00", align 1
@.str.7 = private unnamed_addr constant [7 x i8] c"Call 3\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = alloca i32, align 4
  store i32 42, i32* %3, align 4
  ; Variable x allocated
  %4 = load i32, i32* %3, align 4
  %5 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.fmt.d.1, i64 0, i64 0
  %6 = call i32 (i8*, ...) @printf(i8* %5, i32 %4)
  %7 = add i32 0, 0
  ; Expression result: %7
  %8 = load i32, i32* %3, align 4
  %9 = icmp sgt i32 %8, 0
  br i1 %9, label %label0, label %label1
label0:
  %10 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.2, i64 0, i64 0
  %11 = call i32 @puts(i8* %10)
  %12 = add i32 0, 0
  ; Expression result: %12
  br label %label2
label1:
  br label %label2
label2:
  %13 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.3, i64 0, i64 0
  %14 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.4, i64 0, i64 0
  %15 = call i32 @puts(i8* %13)
  %16 = add i32 0, 0
  ; Expression result: %16
  %17 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.5, i64 0, i64 0
  %18 = call i32 @puts(i8* %17)
  %19 = add i32 0, 0
  ; Expression result: %19
  %20 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.6, i64 0, i64 0
  %21 = call i32 @puts(i8* %20)
  %22 = add i32 0, 0
  ; Expression result: %22
  %23 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.7, i64 0, i64 0
  %24 = call i32 @puts(i8* %23)
  %25 = add i32 0, 0
  ; Expression result: %25
  ret i32 0
}

