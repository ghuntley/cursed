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

define i32 @main() {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = add i32 0, 0 ; placeholder
  %7 = alloca i32, align 4
  store i32 %6, i32* %7, align 4
  ; Variable a allocated
  %8 = add i32 0, 0 ; placeholder
  %9 = alloca i32, align 4
  store i32 %8, i32* %9, align 4
  ; Variable b allocated
  %10 = load i32, i32* %7, align 4
  %11 = load i32, i32* %9, align 4
  %12 = add i32 %10, %11
  %13 = alloca i32, align 4
  store i32 %12, i32* %13, align 4
  ; Variable result allocated
  %14 = add i32 0, 0 ; placeholder
  %15 = call i32 @puts(i8* %14)
  %16 = add i32 0, 0
  ; Expression result: %16
  %17 = add i32 0, 0 ; placeholder
  %18 = alloca i8*, align 4
  store i8* %17, i8** %18, align 4
  ; Variable message allocated
  %19 = add i32 0, 0 ; placeholder
  %20 = call i32 @puts(i8* %19)
  %21 = add i32 0, 0
  ; Expression result: %21
  %22 = add i32 0, 0 ; placeholder
  %23 = alloca i1, align 4
  store i1 %22, i1* %23, align 4
  ; Variable flag allocated
  %24 = add i32 0, 0 ; placeholder
  %25 = alloca i1, align 4
  store i1 %24, i1* %25, align 4
  ; Variable check allocated
  %26 = add i32 0, 0 ; placeholder
  %27 = call i32 @puts(i8* %26)
  %28 = add i32 0, 0
  ; Expression result: %28
  %29 = add i32 0, 0 ; placeholder
  %30 = alloca i8*, align 4
  store i8* %29, i8** %30, align 4
  ; Variable numbers allocated
  %31 = add i32 0, 0 ; placeholder
  ; Expression result: %31
  %32 = alloca [5x i32], align 4
  %33 = add i32 0, 0 ; placeholder
  %34 = getelementptr inbounds [5x i32], [5x i32]* %32, i64 0, i64 0
  store i32 %33, i32* %34, align 4
  %35 = add i32 0, 0 ; placeholder
  %36 = getelementptr inbounds [5x i32], [5x i32]* %32, i64 0, i64 1
  store i32 %35, i32* %36, align 4
  %37 = add i32 0, 0 ; placeholder
  %38 = getelementptr inbounds [5x i32], [5x i32]* %32, i64 0, i64 2
  store i32 %37, i32* %38, align 4
  %39 = add i32 0, 0 ; placeholder
  %40 = getelementptr inbounds [5x i32], [5x i32]* %32, i64 0, i64 3
  store i32 %39, i32* %40, align 4
  %41 = add i32 0, 0 ; placeholder
  %42 = getelementptr inbounds [5x i32], [5x i32]* %32, i64 0, i64 4
  store i32 %41, i32* %42, align 4
  ; Expression result: %32
  %43 = load i8*, i8** %30, align 4
  %44 = alloca i8*, align 4
  store i8* %43, i8** %44, align 4
  ; Variable first allocated
  %45 = alloca [1x i32], align 4
  %46 = add i32 0, 0 ; placeholder
  %47 = getelementptr inbounds [1x i32], [1x i32]* %45, i64 0, i64 0
  store i32 %46, i32* %47, align 4
  ; Expression result: %45
  %48 = add i32 0, 0 ; placeholder
  %49 = call i32 @puts(i8* %48)
  %50 = add i32 0, 0
  ; Expression result: %50
  %51 = load i32, i32* %13, align 4
  %52 = add i32 0, 0 ; placeholder
  %53 = icmp sgt i32 %51, %52
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %53, label %label0, label %label1
label0:
  %54 = add i32 0, 0 ; placeholder
  %55 = call i32 @puts(i8* %54)
  %56 = add i32 0, 0
  ; Expression result: %56
  br label %label2
label1:
  %57 = add i32 0, 0 ; placeholder
  %58 = call i32 @puts(i8* %57)
  %59 = add i32 0, 0
  ; Expression result: %59
  br label %label2
label2:
  %60 = add i32 0, 0 ; placeholder
  %61 = alloca i32, align 4
  store i32 %60, i32* %61, align 4
  ; Variable counter allocated
  %62 = load i32, i32* %61, align 4
  ; Expression result: %62
  %63 = add i32 0, 0 ; placeholder
  ; Expression result: %63
  %64 = load i32, i32* %61, align 4
  %65 = add i32 0, 0 ; placeholder
  %66 = add i32 %64, %65
  ; Expression result: %66
  %67 = add i32 0, 0 ; placeholder
  %68 = call i32 @puts(i8* %67)
  %69 = add i32 0, 0
  ; Expression result: %69
  %70 = load i32, i32* %61, align 4
  ; Expression result: %70
  %71 = add i32 0, 0 ; placeholder
  ; Expression result: %71
  %72 = load i32, i32* %61, align 4
  %73 = add i32 0, 0 ; placeholder
  %74 = add i32 %72, %73
  ; Expression result: %74
  %75 = add i32 0, 0 ; placeholder
  %76 = call i32 @puts(i8* %75)
  %77 = add i32 0, 0
  ; Expression result: %77
  %78 = load i32, i32* %61, align 4
  ; Expression result: %78
  %79 = add i32 0, 0 ; placeholder
  ; Expression result: %79
  %80 = load i32, i32* %61, align 4
  %81 = add i32 0, 0 ; placeholder
  %82 = add i32 %80, %81
  ; Expression result: %82
  %83 = add i32 0, 0 ; placeholder
  %84 = call i32 @puts(i8* %83)
  %85 = add i32 0, 0
  ; Expression result: %85
  %86 = add i32 0, 0 ; placeholder
  %87 = call i32 @puts(i8* %86)
  %88 = add i32 0, 0
  ; Expression result: %88
  %89 = add i32 0, 0 ; placeholder
  %90 = call i32 @puts(i8* %89)
  %91 = add i32 0, 0
  ; Expression result: %91
  ret i32 0
}

