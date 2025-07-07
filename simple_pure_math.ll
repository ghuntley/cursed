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

define i8* @math_abs_pure(i8* %x) {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = icmp slt i32 %x, %0
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %1, label %label0, label %label1
label0:
  %2 = add i32 0, 0 ; placeholder
  %3 = sub i32 %2, %x
  ; Expression result: %3
  br label %label2
label1:
  %4 = add i32 0, 0 ; placeholder
  ; Expression result: %4
  ; Expression result: %x
  br label %label2
label2:
  ret i32 0
}

define i8* @math_min_pure(i8* %a, i8* %b) {
entry:
  %0 = icmp slt i32 %a, %b
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %0, label %label0, label %label1
label0:
  %1 = add i32 0, 0 ; placeholder
  ; Expression result: %1
  ; Expression result: %a
  br label %label2
label1:
  %2 = add i32 0, 0 ; placeholder
  ; Expression result: %2
  ; Expression result: %b
  br label %label2
label2:
  ret i32 0
}

define {i8*}* @test_basic_math() {
entry:
  %0 = add i32 0, 0 ; placeholder
  ; Expression result: %0
  %1 = alloca {i32, i32, i32}, align 4
  %2 = add i32 0, 0 ; placeholder
  %3 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %1, i32 0, i32 0
  store i32 %2, i32* %3, align 4
  ; Expression result: %1
  %4 = add i32 0, 0 ; placeholder
  %5 = alloca i8*, align 4
  store i8* %4, i8** %5, align 4
  ; Variable abs_result allocated
  %6 = add i32 0, 0 ; placeholder
  ; Expression result: %6
  %7 = add i32 0, 0 ; placeholder
  ; Expression result: %7
  %8 = add i32 0, 0 ; placeholder
  ; Expression result: %8
  %9 = add i32 0, 0 ; placeholder
  ; Expression result: %9
  %10 = add i32 0, 0 ; placeholder
  ; Expression result: %10
  %11 = alloca {i32, i32, i32}, align 4
  %12 = add i32 0, 0 ; placeholder
  %13 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %11, i32 0, i32 0
  store i32 %12, i32* %13, align 4
  ; Expression result: %11
  %14 = add i32 0, 0 ; placeholder
  %15 = alloca i8*, align 4
  store i8* %14, i8** %15, align 4
  ; Variable min_result allocated
  %16 = add i32 0, 0 ; placeholder
  ; Expression result: %16
  %17 = add i32 0, 0 ; placeholder
  ; Expression result: %17
  %18 = add i32 0, 0 ; placeholder
  %19 = add i32 0, 0 ; placeholder
  %20 = call i32 @math_min_pure(i32 %18, i32 %19)
  ; Expression result: %20
  %21 = add i32 0, 0 ; placeholder
  ; Expression result: %21
  %22 = alloca {i32, i32, i32}, align 4
  %23 = add i32 0, 0 ; placeholder
  %24 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %22, i32 0, i32 0
  store i32 %23, i32* %24, align 4
  ; Expression result: %22
  %25 = add i32 0, 0 ; placeholder
  ; Expression result: %25
  %26 = alloca {i32, i32, i32}, align 4
  %27 = add i32 0, 0 ; placeholder
  %28 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %26, i32 0, i32 0
  store i32 %27, i32* %28, align 4
  ; Expression result: %26
  ret i32 0
}

define i32 @main() {
entry:
  %0 = call i32 @test_basic_math()
  ; Expression result: %0
  ret i32 0
}

