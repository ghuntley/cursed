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

define i8* @parseSimpleExpression(i8* %input) {
entry:
  %0 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.0, i64 0, i64 0
  %1 = icmp eq i32 %input, %0
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %1, label %label0, label %label1
label0:
  %2 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.1, i64 0, i64 0
  ret i8* %2
label1:
  br label %label2
label2:
  %3 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %4 = icmp eq i32 %input, %3
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %4, label %label3, label %label4
label3:
  %5 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.3, i64 0, i64 0
  ret i8* %5
label4:
  br label %label5
label5:
  %6 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.4, i64 0, i64 0
  ret i8* %6
}

define i32 @evaluateExpression(i8* %operation) {
entry:
  %0 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.1, i64 0, i64 0
  %1 = icmp eq i32 %operation, %0
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %1, label %label0, label %label1
label0:
  %2 = add i32 2, 3
  ret i32 %2
label1:
  br label %label2
label2:
  %3 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.3, i64 0, i64 0
  %4 = icmp eq i32 %operation, %3
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %4, label %label3, label %label4
label3:
  %5 = mul i32 5, 4
  ret i32 %5
label4:
  br label %label5
label5:
  ret i32 0
}

define i32 @compileAndRun(i8* %source) {
entry:
  %0 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.5, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = call i32 @puts(i8* %source)
  %4 = add i32 0, 0
  ; Expression result: %4
  %5 = call i32 @parseSimpleExpression(i32 %source)
  %6 = alloca i32, align 4
  store i32 %5, i32* %6, align 4
  ; Variable operation allocated
  %7 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.6, i64 0, i64 0
  %8 = call i32 @puts(i8* %7)
  %9 = add i32 0, 0
  ; Expression result: %9
  %10 = load i32, i32* %6, align 4
  %11 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.7, i64 0, i64 0
  %12 = call i32 (i8*, ...) @printf(i8* %11, i32 %10)
  %13 = add i32 0, 0
  ; Expression result: %13
  %14 = load i32, i32* %6, align 4
  %15 = call i32 @evaluateExpression(i32 %14)
  %16 = alloca i32, align 4
  store i32 %15, i32* %16, align 4
  ; Variable result allocated
  %17 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.8, i64 0, i64 0
  %18 = call i32 @puts(i8* %17)
  %19 = add i32 0, 0
  ; Expression result: %19
  %20 = load i32, i32* %16, align 4
  %21 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.7, i64 0, i64 0
  %22 = call i32 (i8*, ...) @printf(i8* %21, i32 %20)
  %23 = add i32 0, 0
  ; Expression result: %23
  %24 = load i32, i32* %16, align 4
  ret i32 %24
}


; String constants
@.str.12 = private unnamed_addr constant [19 x i8] c"✅ Test 2 result:\00", align 1
@.str.7 = private unnamed_addr constant [6 x i8] c"%d\\0A\00", align 1
@.str.2 = private unnamed_addr constant [6 x i8] c"5 * 4\00", align 1
@.str.0 = private unnamed_addr constant [6 x i8] c"2 + 3\00", align 1
@.str.13 = private unnamed_addr constant [35 x i8] c"🎉 Self-hosting demo successful!\00", align 1
@.str.14 = private unnamed_addr constant [16 x i8] c"Total computed:\00", align 1
@.str.8 = private unnamed_addr constant [21 x i8] c"⚡ Executed result:\00", align 1
@.str.1 = private unnamed_addr constant [9 x i8] c"addition\00", align 1
@.str.5 = private unnamed_addr constant [21 x i8] c"🔍 Parsing source:\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c"unknown\00", align 1
@.str.10 = private unnamed_addr constant [29 x i8] c"============================\00", align 1
@.str.6 = private unnamed_addr constant [23 x i8] c"📋 Parsed operation:\00", align 1
@.str.11 = private unnamed_addr constant [19 x i8] c"✅ Test 1 result:\00", align 1
@.str.3 = private unnamed_addr constant [15 x i8] c"multiplication\00", align 1
@.str.9 = private unnamed_addr constant [30 x i8] c"🚀 CURSED Self-Hosting Demo\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.9, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.10, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.0, i64 0, i64 0
  %7 = call i32 @compileAndRun(i8* %6)
  %8 = alloca i32, align 4
  store i32 %7, i32* %8, align 4
  ; Variable result1 allocated
  %9 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.11, i64 0, i64 0
  %10 = call i32 @puts(i8* %9)
  %11 = add i32 0, 0
  ; Expression result: %11
  %12 = load i32, i32* %8, align 4
  %13 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.7, i64 0, i64 0
  %14 = call i32 (i8*, ...) @printf(i8* %13, i32 %12)
  %15 = add i32 0, 0
  ; Expression result: %15
  %16 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.2, i64 0, i64 0
  %17 = call i32 @compileAndRun(i8* %16)
  %18 = alloca i32, align 4
  store i32 %17, i32* %18, align 4
  ; Variable result2 allocated
  %19 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.12, i64 0, i64 0
  %20 = call i32 @puts(i8* %19)
  %21 = add i32 0, 0
  ; Expression result: %21
  %22 = load i32, i32* %18, align 4
  %23 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.7, i64 0, i64 0
  %24 = call i32 (i8*, ...) @printf(i8* %23, i32 %22)
  %25 = add i32 0, 0
  ; Expression result: %25
  %30 = load i32, i32* %29, align 4
  %31 = icmp sgt i32 %30, 20
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: processing init statement
  %26 = load i32, i32* %8, align 4
  %27 = load i32, i32* %18, align 4
  %28 = add i32 %26, %27
  %29 = alloca i32, align 4
  store i32 %28, i32* %29, align 4
  ; Short declaration: x := %28
  ; DEBUG FC: init statement complete
  ; DEBUG FC: about to process condition
  br i1 %31, label %label0, label %label1
label0:
  %32 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.13, i64 0, i64 0
  %33 = call i32 @puts(i8* %32)
  %34 = add i32 0, 0
  ; Expression result: %34
  %35 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.14, i64 0, i64 0
  %36 = call i32 @puts(i8* %35)
  %37 = add i32 0, 0
  ; Expression result: %37
  %38 = load i32, i32* %29, align 4
  %39 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.7, i64 0, i64 0
  %40 = call i32 (i8*, ...) @printf(i8* %39, i32 %38)
  %41 = add i32 0, 0
  ; Expression result: %41
  br label %label2
label1:
  br label %label2
label2:
  ret i32 0
}

