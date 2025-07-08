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
define i8* @test_start(i8* %name) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  ; Expression result: %name
  %1 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %1
  ret i32 0
}

define void @assert_eq_string(i8* %actual, i8* %expected) {
entry:
  %0 = icmp eq i32 %actual, %expected
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.1, i64 0, i64 0
  %2 = call i32 @puts(i8* %1)
  %3 = add i32 0, 0
  ; Expression result: %3
  br label %label2
label1:
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %4
  %5 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.2, i64 0, i64 0
  %6 = add i32 %actual, %5
  %7 = add i32 %6, %expected
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %8
  br label %label2
label2:
  ret void
}

define void @assert_eq_int(i32 %actual, i32 %expected) {
entry:
  %0 = icmp eq i32 %actual, %expected
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.3, i64 0, i64 0
  %2 = call i32 @puts(i8* %1)
  %3 = add i32 0, 0
  ; Expression result: %3
  br label %label2
label1:
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %5
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %6
  %7 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.2, i64 0, i64 0
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %9 = add i32 %7, %8
  ; Expression result: %9
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %10
  br label %label2
label2:
  ret void
}

define void @assert_true(i1 %value) {
entry:
  %0 = icmp eq i1 %value, 1
  ; DEBUG FC: compile_if_statement_with_init called
  ; DEBUG FC: about to process condition
  br i1 %0, label %label0, label %label1
label0:
  %1 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.4, i64 0, i64 0
  %2 = call i32 @puts(i8* %1)
  %3 = add i32 0, 0
  ; Expression result: %3
  br label %label2
label1:
  %4 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.5, i64 0, i64 0
  %5 = call i32 @puts(i8* %4)
  %6 = add i32 0, 0
  ; Expression result: %6
  br label %label2
label2:
  ret void
}

define i32 @print_test_summary() {
entry:
  %0 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.6, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  ret i32 0
}

define i8* @NewSlayReader(i8* %reader) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.7, i64 0, i64 0
  %2 = add i32 %1, %reader
  %3 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.8, i64 0, i64 0
  %4 = add i32 %2, %3
  ; Expression result: %4
  ret i32 0
}

define i8* @NewSlayReaderSize(i8* %reader, i32 %size) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.7, i64 0, i64 0
  %2 = add i32 %1, %reader
  %3 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.9, i64 0, i64 0
  %4 = add i32 %2, %3
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %6 = add i32 %4, %5
  ; Expression result: %6
  %7 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %7
  %8 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.10, i64 0, i64 0
  ; Expression result: %8
  ret i32 0
}

define i8* @ReadData(i8* %reader, i32 %size) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.11, i64 0, i64 0
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %3 = add i32 %1, %2
  ; Expression result: %3
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %4
  %5 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.12, i64 0, i64 0
  %6 = add i32 %5, %reader
  ; Expression result: %6
  ret i32 0
}

define i8* @ReadLine(i8* %reader) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.13, i64 0, i64 0
  %2 = add i32 %1, %reader
  ; Expression result: %2
  ret i32 0
}

define i8* @NewSlayWriter(i8* %writer) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.14, i64 0, i64 0
  %2 = add i32 %1, %writer
  %3 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.8, i64 0, i64 0
  %4 = add i32 %2, %3
  ; Expression result: %4
  ret i32 0
}

define i32 @WriteData(i8* %writer, i8* %data) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  ; Expression result: 42
  ret i32 0
}

define i8* @FlushWriter(i8* %writer) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.15, i64 0, i64 0
  ; Expression result: %1
  ret i32 0
}

define i8* @NewSlayScanner(i8* %reader) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.16, i64 0, i64 0
  %2 = add i32 %1, %reader
  %3 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.17, i64 0, i64 0
  %4 = add i32 %2, %3
  ; Expression result: %4
  ret i32 0
}

define i1 @ScanNext(i8* %scanner) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  ; Expression result: 1
  ret i32 0
}

define i8* @ScanText(i8* %scanner) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.18, i64 0, i64 0
  %2 = add i32 %1, %scanner
  ; Expression result: %2
  ret i32 0
}

define i8* @NewSlayPhraseReader(i8* %reader) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.19, i64 0, i64 0
  %2 = add i32 %1, %reader
  %3 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.20, i64 0, i64 0
  %4 = add i32 %2, %3
  ; Expression result: %4
  ret i32 0
}

define i8* @ReadPhrase(i8* %phraseReader) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.21, i64 0, i64 0
  ; Expression result: %1
  ret i32 0
}

define i32 @GetDefaultBufferSize() {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  ; Expression result: 4096
  ret i32 0
}



; String constants
@.str.8 = private unnamed_addr constant [16 x i8] c", buffer: 4096}\00", align 1
@.str.3 = private unnamed_addr constant [27 x i8] c"  ✅ PASS: integers match\00", align 1
@.str.13 = private unnamed_addr constant [24 x i8] c"Line{Hello World} from \00", align 1
@.str.15 = private unnamed_addr constant [15 x i8] c"Writer flushed\00", align 1
@.str.21 = private unnamed_addr constant [18 x i8] c"no cap fr fr 💯\00", align 1
@.str.32 = private unnamed_addr constant [37 x i8] c"SlayWriter{output.txt, buffer: 4096}\00", align 1
@.str.40 = private unnamed_addr constant [23 x i8] c"Phrase reader creation\00", align 1
@.str.45 = private unnamed_addr constant [23 x i8] c"Buffer size validation\00", align 1
@.str.26 = private unnamed_addr constant [9 x i8] c"data.txt\00", align 1
@.str.22 = private unnamed_addr constant [25 x i8] c"Buffered reader creation\00", align 1
@.str.43 = private unnamed_addr constant [20 x i8] c"Reading GenZ phrase\00", align 1
@.str.25 = private unnamed_addr constant [33 x i8] c"Buffered reader with custom size\00", align 1
@.str.7 = private unnamed_addr constant [12 x i8] c"SlayReader{\00", align 1
@.str.16 = private unnamed_addr constant [13 x i8] c"SlayScanner{\00", align 1
@.str.9 = private unnamed_addr constant [11 x i8] c", buffer: \00", align 1
@.str.27 = private unnamed_addr constant [35 x i8] c"SlayReader{data.txt, buffer: 8192}\00", align 1
@.str.38 = private unnamed_addr constant [20 x i8] c"Scanning next token\00", align 1
@.str.42 = private unnamed_addr constant [42 x i8] c"SlayPhraseReader{genZ.txt, phrases: GenZ}\00", align 1
@.str.44 = private unnamed_addr constant [20 x i8] c"Default buffer size\00", align 1
@.str.18 = private unnamed_addr constant [23 x i8] c"TokenText{World} from \00", align 1
@.str.6 = private unnamed_addr constant [30 x i8] c"🎯 Slay IO tests completed!\00", align 1
@.str.28 = private unnamed_addr constant [13 x i8] c"Reading data\00", align 1
@.str.5 = private unnamed_addr constant [26 x i8] c"  ❌ FAIL: expected true\00", align 1
@.str.14 = private unnamed_addr constant [12 x i8] c"SlayWriter{\00", align 1
@.str.33 = private unnamed_addr constant [13 x i8] c"Writing data\00", align 1
@.str.1 = private unnamed_addr constant [26 x i8] c"  ✅ PASS: strings match\00", align 1
@.str.2 = private unnamed_addr constant [12 x i8] c", expected \00", align 1
@.str.11 = private unnamed_addr constant [6 x i8] c"Data[\00", align 1
@.str.4 = private unnamed_addr constant [26 x i8] c"  ✅ PASS: value is true\00", align 1
@.str.31 = private unnamed_addr constant [11 x i8] c"output.txt\00", align 1
@.str.34 = private unnamed_addr constant [12 x i8] c"Hello World\00", align 1
@.str.35 = private unnamed_addr constant [16 x i8] c"Flushing writer\00", align 1
@.str.12 = private unnamed_addr constant [14 x i8] c" bytes] from \00", align 1
@.str.36 = private unnamed_addr constant [17 x i8] c"Scanner creation\00", align 1
@.str.30 = private unnamed_addr constant [25 x i8] c"Buffered writer creation\00", align 1
@.str.10 = private unnamed_addr constant [2 x i8] c"}\00", align 1
@.str.0 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.37 = private unnamed_addr constant [43 x i8] c"SlayScanner{input.txt, tokenizer: default}\00", align 1
@.str.24 = private unnamed_addr constant [36 x i8] c"SlayReader{input.txt, buffer: 4096}\00", align 1
@.str.19 = private unnamed_addr constant [18 x i8] c"SlayPhraseReader{\00", align 1
@.str.17 = private unnamed_addr constant [22 x i8] c", tokenizer: default}\00", align 1
@.str.20 = private unnamed_addr constant [17 x i8] c", phrases: GenZ}\00", align 1
@.str.23 = private unnamed_addr constant [10 x i8] c"input.txt\00", align 1
@.str.29 = private unnamed_addr constant [13 x i8] c"Reading line\00", align 1
@.str.39 = private unnamed_addr constant [18 x i8] c"Getting scan text\00", align 1
@.str.41 = private unnamed_addr constant [9 x i8] c"genZ.txt\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.22, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  %2 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.23, i64 0, i64 0
  %3 = call i32 @NewSlayReader(i32 %2)
  %4 = alloca i8*, align 4
  store i8* %3, i8** %4, align 4
  ; Variable reader allocated at %4
  %5 = load i32, i32* %4, align 4
  %6 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.24, i64 0, i64 0
  %7 = call i32 @assert_eq_string(i32 %5, i32 %6)
  %8 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.25, i64 0, i64 0
  %9 = call i32 @test_start(i32 %8)
  %10 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.26, i64 0, i64 0
  %11 = call i32 @NewSlayReaderSize(i32 %10, i32 8192)
  %12 = alloca i8*, align 4
  store i8* %11, i8** %12, align 4
  ; Variable customReader allocated at %12
  %13 = load i32, i32* %12, align 4
  %14 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.27, i64 0, i64 0
  %15 = call i32 @assert_eq_string(i32 %13, i32 %14)
  %16 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.28, i64 0, i64 0
  %17 = call i32 @test_start(i32 %16)
  %18 = load i32, i32* %4, align 4
  %19 = call i32 @ReadData(i32 %18, i32 100)
  %20 = alloca i8*, align 4
  store i8* %19, i8** %20, align 4
  ; Variable data allocated at %20
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %22 = load i32, i32* %4, align 4
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %24 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.29, i64 0, i64 0
  %25 = call i32 @test_start(i32 %24)
  %26 = load i32, i32* %4, align 4
  %27 = call i32 @ReadLine(i32 %26)
  %28 = alloca i8*, align 4
  store i8* %27, i8** %28, align 4
  ; Variable line allocated at %28
  %29 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %30 = load i32, i32* %4, align 4
  %31 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %32 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.30, i64 0, i64 0
  %33 = call i32 @test_start(i32 %32)
  %34 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.31, i64 0, i64 0
  %35 = call i32 @NewSlayWriter(i32 %34)
  %36 = alloca i8*, align 4
  store i8* %35, i8** %36, align 4
  ; Variable writer allocated at %36
  %37 = load i32, i32* %36, align 4
  %38 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.32, i64 0, i64 0
  %39 = call i32 @assert_eq_string(i32 %37, i32 %38)
  %40 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.33, i64 0, i64 0
  %41 = call i32 @test_start(i32 %40)
  %42 = load i32, i32* %36, align 4
  %43 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.34, i64 0, i64 0
  %44 = call i32 @WriteData(i32 %42, i32 %43)
  %45 = alloca i32, align 4
  store i32 %44, i32* %45, align 4
  ; Variable bytesWritten allocated at %45
  %46 = load i32, i32* %45, align 4
  %47 = call i32 @assert_eq_int(i32 %46, i32 42)
  %48 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.35, i64 0, i64 0
  %49 = call i32 @test_start(i32 %48)
  %50 = load i32, i32* %36, align 4
  %51 = call i32 @FlushWriter(i32 %50)
  %52 = alloca i8*, align 4
  store i8* %51, i8** %52, align 4
  ; Variable flushResult allocated at %52
  %53 = load i32, i32* %52, align 4
  %54 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.15, i64 0, i64 0
  %55 = call i32 @assert_eq_string(i32 %53, i32 %54)
  %56 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.36, i64 0, i64 0
  %57 = call i32 @test_start(i32 %56)
  %58 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.23, i64 0, i64 0
  %59 = call i32 @NewSlayScanner(i32 %58)
  %60 = alloca i8*, align 4
  store i8* %59, i8** %60, align 4
  ; Variable scanner allocated at %60
  %61 = load i32, i32* %60, align 4
  %62 = getelementptr inbounds [43 x i8], [43 x i8]* @.str.37, i64 0, i64 0
  %63 = call i32 @assert_eq_string(i32 %61, i32 %62)
  %64 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.38, i64 0, i64 0
  %65 = call i32 @test_start(i32 %64)
  %66 = load i32, i32* %60, align 4
  %67 = call i32 @ScanNext(i32 %66)
  %68 = alloca i1, align 4
  store i1 %67, i1* %68, align 4
  ; Variable hasNext allocated at %68
  %69 = load i32, i32* %68, align 4
  %70 = call i32 @assert_true(i32 %69)
  %71 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.39, i64 0, i64 0
  %72 = call i32 @test_start(i32 %71)
  %73 = load i32, i32* %60, align 4
  %74 = call i32 @ScanText(i32 %73)
  %75 = alloca i8*, align 4
  store i8* %74, i8** %75, align 4
  ; Variable tokenText allocated at %75
  %76 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %77 = load i32, i32* %60, align 4
  %78 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %79 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.40, i64 0, i64 0
  %80 = call i32 @test_start(i32 %79)
  %81 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.41, i64 0, i64 0
  %82 = call i32 @NewSlayPhraseReader(i32 %81)
  %83 = alloca i8*, align 4
  store i8* %82, i8** %83, align 4
  ; Variable phraseReader allocated at %83
  %84 = load i32, i32* %83, align 4
  %85 = getelementptr inbounds [42 x i8], [42 x i8]* @.str.42, i64 0, i64 0
  %86 = call i32 @assert_eq_string(i32 %84, i32 %85)
  %87 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.43, i64 0, i64 0
  %88 = call i32 @test_start(i32 %87)
  %89 = load i32, i32* %83, align 4
  %90 = call i32 @ReadPhrase(i32 %89)
  %91 = alloca i8*, align 4
  store i8* %90, i8** %91, align 4
  ; Variable phrase allocated at %91
  %92 = load i32, i32* %91, align 4
  %93 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.21, i64 0, i64 0
  %94 = call i32 @assert_eq_string(i32 %92, i32 %93)
  %95 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.44, i64 0, i64 0
  %96 = call i32 @test_start(i32 %95)
  %97 = call i32 @GetDefaultBufferSize()
  %98 = alloca i32, align 4
  store i32 %97, i32* %98, align 4
  ; Variable bufferSize allocated at %98
  %99 = load i32, i32* %98, align 4
  %100 = call i32 @assert_eq_int(i32 %99, i32 4096)
  %101 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.45, i64 0, i64 0
  %102 = call i32 @test_start(i32 %101)
  %103 = call i32 @assert_true(i32 1)
  %104 = call i32 @print_test_summary()
  ret i32 0
}
