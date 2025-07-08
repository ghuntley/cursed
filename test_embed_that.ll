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
  %0 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.6, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  ret i32 0
}

define i8* @ThatFile(i8* %name, i8* %content) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.7, i64 0, i64 0
  %2 = add i32 %1, %name
  %3 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %4 = add i32 %2, %3
  %5 = add i32 %4, %content
  %6 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.9, i64 0, i64 0
  %7 = add i32 %5, %6
  ; Expression result: %7
  ret i32 0
}

define i8* @ThatFiles(i8* %pattern) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.10, i64 0, i64 0
  %2 = add i32 %1, %pattern
  %3 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.11, i64 0, i64 0
  %4 = add i32 %2, %3
  ; Expression result: %4
  ret i32 0
}

define i8* @GetFileName(i8* %file) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.12, i64 0, i64 0
  ; Expression result: %1
  ret i32 0
}

define i32 @GetFileSize(i8* %file) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  ; Expression result: 1024
  ret i32 0
}

define i8* @GetFileContent(i8* %file) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.13, i64 0, i64 0
  %2 = add i32 %1, %file
  ; Expression result: %2
  ret i32 0
}

define i1 @IsTextFile(i8* %file) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  ; Expression result: 1
  ret i32 0
}

define i8* @GetFileFromCollection(i8* %files, i8* %name) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.14, i64 0, i64 0
  %2 = add i32 %1, %name
  %3 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.15, i64 0, i64 0
  %4 = add i32 %2, %3
  %5 = add i32 %4, %files
  ; Expression result: %5
  ret i32 0
}

define i32 @GetFileCount(i8* %files) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  ; Expression result: 3
  ret i32 0
}

define i8* @FilterFilesByExtension(i8* %files, i8* %extension) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.16, i64 0, i64 0
  %2 = add i32 %1, %extension
  %3 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.17, i64 0, i64 0
  %4 = add i32 %2, %3
  %5 = add i32 %4, %files
  ; Expression result: %5
  ret i32 0
}

define i8* @MakeFileSystem(i8* %files) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.18, i64 0, i64 0
  %2 = add i32 %1, %files
  %3 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.19, i64 0, i64 0
  %4 = add i32 %2, %3
  ; Expression result: %4
  ret i32 0
}

define i8* @LoadThatFile(i8* %path) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.20, i64 0, i64 0
  %2 = add i32 %1, %path
  %3 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.21, i64 0, i64 0
  %4 = add i32 %2, %3
  ; Expression result: %4
  ret i32 0
}

define i8* @ParseTemplates(i8* %patterns) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.22, i64 0, i64 0
  %2 = add i32 %1, %patterns
  %3 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.23, i64 0, i64 0
  %4 = add i32 %2, %3
  ; Expression result: %4
  ret i32 0
}

define i8* @LoadImage(i8* %path) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.24, i64 0, i64 0
  %2 = add i32 %1, %path
  %3 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.25, i64 0, i64 0
  %4 = add i32 %2, %3
  ; Expression result: %4
  ret i32 0
}

define i8* @LoadJSON(i8* %path, i8* %target) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.26, i64 0, i64 0
  %2 = add i32 %1, %path
  %3 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.27, i64 0, i64 0
  %4 = add i32 %2, %3
  %5 = add i32 %4, %target
  ; Expression result: %5
  ret i32 0
}

define i8* @NewResourceCache() {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.28, i64 0, i64 0
  ; Expression result: %1
  ret i32 0
}

define i8* @GetFromCache(i8* %cache, i8* %key) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  ; Expression result: %0
  %1 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.29, i64 0, i64 0
  %2 = add i32 %1, %key
  %3 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.17, i64 0, i64 0
  %4 = add i32 %2, %3
  %5 = add i32 %4, %cache
  ; Expression result: %5
  ret i32 0
}



; String constants
@.str.11 = private unnamed_addr constant [29 x i8] c", count: 5, totalSize: 2048}\00", align 1
@.str.32 = private unnamed_addr constant [12 x i8] c"Hello World\00", align 1
@.str.0 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.14 = private unnamed_addr constant [10 x i8] c"ThatFile{\00", align 1
@.str.60 = private unnamed_addr constant [22 x i8] c"Create resource cache\00", align 1
@.str.4 = private unnamed_addr constant [26 x i8] c"  ✅ PASS: value is true\00", align 1
@.str.63 = private unnamed_addr constant [31 x i8] c"Embed functionality validation\00", align 1
@.str.46 = private unnamed_addr constant [17 x i8] c"Make file system\00", align 1
@.str.34 = private unnamed_addr constant [21 x i8] c"ThatFiles collection\00", align 1
@.str.39 = private unnamed_addr constant [17 x i8] c"Get file content\00", align 1
@.str.28 = private unnamed_addr constant [40 x i8] c"ResourceCache{size: 0, maxEntries: 100}\00", align 1
@.str.50 = private unnamed_addr constant [16 x i8] c"Parse templates\00", align 1
@.str.3 = private unnamed_addr constant [27 x i8] c"  ✅ PASS: integers match\00", align 1
@.str.12 = private unnamed_addr constant [12 x i8] c"example.txt\00", align 1
@.str.62 = private unnamed_addr constant [10 x i8] c"templates\00", align 1
@.str.27 = private unnamed_addr constant [7 x i8] c" into \00", align 1
@.str.49 = private unnamed_addr constant [42 x i8] c"LoadedFile{static/logo.png, cached: true}\00", align 1
@.str.58 = private unnamed_addr constant [10 x i8] c"appConfig\00", align 1
@.str.55 = private unnamed_addr constant [50 x i8] c"Image{assets/banner.jpg, width: 800, height: 600}\00", align 1
@.str.1 = private unnamed_addr constant [26 x i8] c"  ✅ PASS: strings match\00", align 1
@.str.57 = private unnamed_addr constant [12 x i8] c"config.json\00", align 1
@.str.45 = private unnamed_addr constant [6 x i8] c".html\00", align 1
@.str.56 = private unnamed_addr constant [17 x i8] c"Load JSON config\00", align 1
@.str.59 = private unnamed_addr constant [44 x i8] c"JSON loaded from config.json into appConfig\00", align 1
@.str.15 = private unnamed_addr constant [19 x i8] c"} from collection \00", align 1
@.str.31 = private unnamed_addr constant [9 x i8] c"test.txt\00", align 1
@.str.54 = private unnamed_addr constant [18 x i8] c"assets/banner.jpg\00", align 1
@.str.13 = private unnamed_addr constant [24 x i8] c"File content data from \00", align 1
@.str.42 = private unnamed_addr constant [11 x i8] c"index.html\00", align 1
@.str.40 = private unnamed_addr constant [20 x i8] c"Text file detection\00", align 1
@.str.52 = private unnamed_addr constant [48 x i8] c"Templates{patterns: templates/*.html, count: 5}\00", align 1
@.str.61 = private unnamed_addr constant [17 x i8] c"Cache operations\00", align 1
@.str.6 = private unnamed_addr constant [33 x i8] c"🎯 Embed That tests completed!\00", align 1
@.str.5 = private unnamed_addr constant [26 x i8] c"  ❌ FAIL: expected true\00", align 1
@.str.21 = private unnamed_addr constant [16 x i8] c", cached: true}\00", align 1
@.str.37 = private unnamed_addr constant [14 x i8] c"Get file name\00", align 1
@.str.38 = private unnamed_addr constant [14 x i8] c"Get file size\00", align 1
@.str.19 = private unnamed_addr constant [18 x i8] c", readOnly: true}\00", align 1
@.str.23 = private unnamed_addr constant [12 x i8] c", count: 5}\00", align 1
@.str.44 = private unnamed_addr constant [20 x i8] c"Filter by extension\00", align 1
@.str.26 = private unnamed_addr constant [18 x i8] c"JSON loaded from \00", align 1
@.str.9 = private unnamed_addr constant [12 x i8] c", size: 42}\00", align 1
@.str.47 = private unnamed_addr constant [19 x i8] c"Load embedded file\00", align 1
@.str.30 = private unnamed_addr constant [18 x i8] c"ThatFile creation\00", align 1
@.str.7 = private unnamed_addr constant [16 x i8] c"ThatFile{name: \00", align 1
@.str.51 = private unnamed_addr constant [17 x i8] c"templates/*.html\00", align 1
@.str.43 = private unnamed_addr constant [15 x i8] c"Get file count\00", align 1
@.str.8 = private unnamed_addr constant [12 x i8] c", content: \00", align 1
@.str.53 = private unnamed_addr constant [11 x i8] c"Load image\00", align 1
@.str.35 = private unnamed_addr constant [7 x i8] c"*.html\00", align 1
@.str.16 = private unnamed_addr constant [20 x i8] c"FilteredFiles{ext: \00", align 1
@.str.10 = private unnamed_addr constant [20 x i8] c"ThatFiles{pattern: \00", align 1
@.str.36 = private unnamed_addr constant [54 x i8] c"ThatFiles{pattern: *.html, count: 5, totalSize: 2048}\00", align 1
@.str.25 = private unnamed_addr constant [27 x i8] c", width: 800, height: 600}\00", align 1
@.str.18 = private unnamed_addr constant [12 x i8] c"EmbeddedFS{\00", align 1
@.str.20 = private unnamed_addr constant [12 x i8] c"LoadedFile{\00", align 1
@.str.2 = private unnamed_addr constant [12 x i8] c", expected \00", align 1
@.str.41 = private unnamed_addr constant [25 x i8] c"Get file from collection\00", align 1
@.str.24 = private unnamed_addr constant [7 x i8] c"Image{\00", align 1
@.str.17 = private unnamed_addr constant [8 x i8] c"} from \00", align 1
@.str.22 = private unnamed_addr constant [21 x i8] c"Templates{patterns: \00", align 1
@.str.29 = private unnamed_addr constant [13 x i8] c"CachedValue{\00", align 1
@.str.33 = private unnamed_addr constant [57 x i8] c"ThatFile{name: test.txt, content: Hello World, size: 42}\00", align 1
@.str.48 = private unnamed_addr constant [16 x i8] c"static/logo.png\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.30, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  %2 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.31, i64 0, i64 0
  %3 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.32, i64 0, i64 0
  %4 = call i32 @ThatFile(i32 %2, i32 %3)
  %5 = alloca i8*, align 4
  store i8* %4, i8** %5, align 4
  ; Variable file allocated at %5
  %6 = load i32, i32* %5, align 4
  %7 = getelementptr inbounds [57 x i8], [57 x i8]* @.str.33, i64 0, i64 0
  %8 = call i32 @assert_eq_string(i32 %6, i32 %7)
  %9 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.34, i64 0, i64 0
  %10 = call i32 @test_start(i32 %9)
  %11 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.35, i64 0, i64 0
  %12 = call i32 @ThatFiles(i32 %11)
  %13 = alloca i8*, align 4
  store i8* %12, i8** %13, align 4
  ; Variable files allocated at %13
  %14 = load i32, i32* %13, align 4
  %15 = getelementptr inbounds [54 x i8], [54 x i8]* @.str.36, i64 0, i64 0
  %16 = call i32 @assert_eq_string(i32 %14, i32 %15)
  %17 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.37, i64 0, i64 0
  %18 = call i32 @test_start(i32 %17)
  %19 = load i32, i32* %5, align 4
  %20 = call i32 @GetFileName(i32 %19)
  %21 = alloca i8*, align 4
  store i8* %20, i8** %21, align 4
  ; Variable name allocated at %21
  %22 = load i32, i32* %21, align 4
  %23 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.12, i64 0, i64 0
  %24 = call i32 @assert_eq_string(i32 %22, i32 %23)
  %25 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.38, i64 0, i64 0
  %26 = call i32 @test_start(i32 %25)
  %27 = load i32, i32* %5, align 4
  %28 = call i32 @GetFileSize(i32 %27)
  %29 = alloca i32, align 4
  store i32 %28, i32* %29, align 4
  ; Variable size allocated at %29
  %30 = load i32, i32* %29, align 4
  %31 = call i32 @assert_eq_int(i32 %30, i32 1024)
  %32 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.39, i64 0, i64 0
  %33 = call i32 @test_start(i32 %32)
  %34 = load i32, i32* %5, align 4
  %35 = call i32 @GetFileContent(i32 %34)
  %36 = alloca i8*, align 4
  store i8* %35, i8** %36, align 4
  ; Variable content allocated at %36
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %38 = load i32, i32* %5, align 4
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %40 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.40, i64 0, i64 0
  %41 = call i32 @test_start(i32 %40)
  %42 = load i32, i32* %5, align 4
  %43 = call i32 @IsTextFile(i32 %42)
  %44 = alloca i1, align 4
  store i1 %43, i1* %44, align 4
  ; Variable isText allocated at %44
  %45 = load i32, i32* %44, align 4
  %46 = call i32 @assert_true(i32 %45)
  %47 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.41, i64 0, i64 0
  %48 = call i32 @test_start(i32 %47)
  %49 = load i32, i32* %13, align 4
  %50 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.42, i64 0, i64 0
  %51 = call i32 @GetFileFromCollection(i32 %49, i32 %50)
  %52 = alloca i8*, align 4
  store i8* %51, i8** %52, align 4
  ; Variable retrievedFile allocated at %52
  %53 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %54 = load i32, i32* %13, align 4
  %55 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %56 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.43, i64 0, i64 0
  %57 = call i32 @test_start(i32 %56)
  %58 = load i32, i32* %13, align 4
  %59 = call i32 @GetFileCount(i32 %58)
  %60 = alloca i32, align 4
  store i32 %59, i32* %60, align 4
  ; Variable count allocated at %60
  %61 = load i32, i32* %60, align 4
  %62 = call i32 @assert_eq_int(i32 %61, i32 3)
  %63 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.44, i64 0, i64 0
  %64 = call i32 @test_start(i32 %63)
  %65 = load i32, i32* %13, align 4
  %66 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.45, i64 0, i64 0
  %67 = call i32 @FilterFilesByExtension(i32 %65, i32 %66)
  %68 = alloca i8*, align 4
  store i8* %67, i8** %68, align 4
  ; Variable filtered allocated at %68
  %69 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %70 = load i32, i32* %13, align 4
  %71 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %72 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.46, i64 0, i64 0
  %73 = call i32 @test_start(i32 %72)
  %74 = load i32, i32* %13, align 4
  %75 = call i32 @MakeFileSystem(i32 %74)
  %76 = alloca i8*, align 4
  store i8* %75, i8** %76, align 4
  ; Variable fs allocated at %76
  %77 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %78 = load i32, i32* %13, align 4
  %79 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.19, i64 0, i64 0
  %80 = add i32 %78, %79
  %81 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %82 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.47, i64 0, i64 0
  %83 = call i32 @test_start(i32 %82)
  %84 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.48, i64 0, i64 0
  %85 = call i32 @LoadThatFile(i32 %84)
  %86 = alloca i8*, align 4
  store i8* %85, i8** %86, align 4
  ; Variable loaded allocated at %86
  %87 = load i32, i32* %86, align 4
  %88 = getelementptr inbounds [42 x i8], [42 x i8]* @.str.49, i64 0, i64 0
  %89 = call i32 @assert_eq_string(i32 %87, i32 %88)
  %90 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.50, i64 0, i64 0
  %91 = call i32 @test_start(i32 %90)
  %92 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.51, i64 0, i64 0
  %93 = call i32 @ParseTemplates(i32 %92)
  %94 = alloca i8*, align 4
  store i8* %93, i8** %94, align 4
  ; Variable templates allocated at %94
  %95 = load i32, i32* %94, align 4
  %96 = getelementptr inbounds [48 x i8], [48 x i8]* @.str.52, i64 0, i64 0
  %97 = call i32 @assert_eq_string(i32 %95, i32 %96)
  %98 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.53, i64 0, i64 0
  %99 = call i32 @test_start(i32 %98)
  %100 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.54, i64 0, i64 0
  %101 = call i32 @LoadImage(i32 %100)
  %102 = alloca i8*, align 4
  store i8* %101, i8** %102, align 4
  ; Variable image allocated at %102
  %103 = load i32, i32* %102, align 4
  %104 = getelementptr inbounds [50 x i8], [50 x i8]* @.str.55, i64 0, i64 0
  %105 = call i32 @assert_eq_string(i32 %103, i32 %104)
  %106 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.56, i64 0, i64 0
  %107 = call i32 @test_start(i32 %106)
  %108 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.57, i64 0, i64 0
  %109 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.58, i64 0, i64 0
  %110 = call i32 @LoadJSON(i32 %108, i32 %109)
  %111 = alloca i8*, align 4
  store i8* %110, i8** %111, align 4
  ; Variable jsonResult allocated at %111
  %112 = load i32, i32* %111, align 4
  %113 = getelementptr inbounds [44 x i8], [44 x i8]* @.str.59, i64 0, i64 0
  %114 = call i32 @assert_eq_string(i32 %112, i32 %113)
  %115 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.60, i64 0, i64 0
  %116 = call i32 @test_start(i32 %115)
  %117 = call i32 @NewResourceCache()
  %118 = alloca i8*, align 4
  store i8* %117, i8** %118, align 4
  ; Variable cache allocated at %118
  %119 = load i32, i32* %118, align 4
  %120 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.28, i64 0, i64 0
  %121 = call i32 @assert_eq_string(i32 %119, i32 %120)
  %122 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.61, i64 0, i64 0
  %123 = call i32 @test_start(i32 %122)
  %124 = load i32, i32* %118, align 4
  %125 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.62, i64 0, i64 0
  %126 = call i32 @GetFromCache(i32 %124, i32 %125)
  %127 = alloca i8*, align 4
  store i8* %126, i8** %127, align 4
  ; Variable cachedValue allocated at %127
  %128 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %129 = load i32, i32* %118, align 4
  %130 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.0, i64 0, i64 0
  %131 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.63, i64 0, i64 0
  %132 = call i32 @test_start(i32 %131)
  %133 = call i32 @assert_true(i32 1)
  %134 = call i32 @print_test_summary()
  ret i32 0
}
