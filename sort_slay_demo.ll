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

define i32 @swap_elements(i32 %arr, i32 %i, i32 %j) {
entry:
  %0 = alloca i32, align 4
  store i32 %arr, i32* %0, align 4
  ; Variable temp allocated
  %1 = alloca [0x i32], align 4
  ; Expression result: %1
  ; Expression result: %arr
  %2 = alloca [0x i32], align 4
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  ; Expression result: %3
  ; Expression result: %arr
  %4 = alloca [0x i32], align 4
  ; Expression result: %4
  ; Expression result: %arr
  %5 = alloca [0x i32], align 4
  ; Expression result: %5
  %6 = add i32 0, 0 ; placeholder
  ; Expression result: %6
  %7 = load i32, i32* %0, align 4
  ; Expression result: %7
  ret i32 0
}

define void @bubble_sort(i32 %arr) {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = alloca i32, align 4
  store i32 %0, i32* %1, align 4
  ; Variable n allocated
  %4 = load i32, i32* %3, align 4
  %5 = load i32, i32* %1, align 4
  %6 = icmp slt i32 %4, %5
  %7 = add i32 0, 0 ; placeholder
  %8 = sub i32 %6, %7
  %29 = add i32 0, 0 ; placeholder
  %2 = add i32 0, 0 ; placeholder
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Short declaration: i := %2
  br label %label0
label0:
  br i1 %8, label %label1, label %label3
label1:
  %11 = load i32, i32* %10, align 4
  %12 = load i32, i32* %1, align 4
  %13 = icmp slt i32 %11, %12
  %14 = load i32, i32* %3, align 4
  %15 = sub i32 %13, %14
  %16 = add i32 0, 0 ; placeholder
  %17 = sub i32 %15, %16
  %28 = add i32 0, 0 ; placeholder
  %9 = add i32 0, 0 ; placeholder
  %10 = alloca i32, align 4
  store i32 %9, i32* %10, align 4
  ; Short declaration: j := %9
  br label %label4
label4:
  br i1 %17, label %label5, label %label7
label5:
  %18 = add i32 0, 0 ; placeholder
  ; Expression result: %18
  ; Expression result: %arr
  %19 = alloca [0x i32], align 4
  %20 = icmp sgt i32 %19, %arr
  ; Expression result: %20
  %21 = alloca [1x i32], align 4
  %22 = add i32 0, 0 ; placeholder
  %23 = getelementptr inbounds [1x i32], [1x i32]* %21, i64 0, i64 0
  store i32 %22, i32* %23, align 4
  ; Expression result: %21
  %24 = add i32 0, 0 ; placeholder
  ; Expression result: %24
  %25 = add i32 0, 0 ; placeholder
  ; Expression result: %25
  %26 = add i32 0, 0 ; placeholder
  ; Expression result: %26
  %27 = add i32 0, 0 ; placeholder
  ; Expression result: %27
  br label %label6
label6:
  br label %label4
label7:
  br label %label2
label2:
  br label %label0
label3:
  ret void
}

define i8* @print_array(i32 %arr) {
entry:
  %0 = alloca [1x i32], align 4
  %1 = add i32 0, 0 ; placeholder
  %2 = getelementptr inbounds [1x i32], [1x i32]* %0, i64 0, i64 0
  store i32 %1, i32* %2, align 4
  ; Expression result: %0
  %3 = add i32 0, 0 ; placeholder
  ; Expression result: %3
  ; Expression result: %arr
  %4 = alloca [1x i32], align 4
  %5 = add i32 0, 0 ; placeholder
  %6 = getelementptr inbounds [1x i32], [1x i32]* %4, i64 0, i64 0
  store i32 %5, i32* %6, align 4
  ; Expression result: %4
  %7 = add i32 0, 0 ; placeholder
  ; Expression result: %7
  ; Expression result: %arr
  %8 = alloca [1x i32], align 4
  %9 = add i32 0, 0 ; placeholder
  %10 = getelementptr inbounds [1x i32], [1x i32]* %8, i64 0, i64 0
  store i32 %9, i32* %10, align 4
  ; Expression result: %8
  %11 = add i32 0, 0 ; placeholder
  ; Expression result: %11
  ; Expression result: %arr
  %12 = alloca [1x i32], align 4
  %13 = add i32 0, 0 ; placeholder
  %14 = getelementptr inbounds [1x i32], [1x i32]* %12, i64 0, i64 0
  store i32 %13, i32* %14, align 4
  ; Expression result: %12
  %15 = add i32 0, 0 ; placeholder
  ; Expression result: %15
  ; Expression result: %arr
  %16 = alloca [1x i32], align 4
  %17 = add i32 0, 0 ; placeholder
  %18 = getelementptr inbounds [1x i32], [1x i32]* %16, i64 0, i64 0
  store i32 %17, i32* %18, align 4
  ; Expression result: %16
  %19 = add i32 0, 0 ; placeholder
  ; Expression result: %19
  ret i32 0
}

define i1 @is_sorted(i32 %arr) {
entry:
  %2 = load i32, i32* %1, align 4
  %3 = add i32 0, 0 ; placeholder
  %4 = icmp slt i32 %2, %3
  %14 = add i32 0, 0 ; placeholder
  %0 = add i32 0, 0 ; placeholder
  %1 = alloca i32, align 4
  store i32 %0, i32* %1, align 4
  ; Short declaration: i := %0
  br label %label0
label0:
  br i1 %4, label %label1, label %label3
label1:
  %5 = add i32 0, 0 ; placeholder
  ; Expression result: %5
  ; Expression result: %arr
  %6 = alloca [0x i32], align 4
  %7 = icmp sgt i32 %6, %arr
  ; Expression result: %7
  %8 = alloca [1x i32], align 4
  %9 = add i32 0, 0 ; placeholder
  %10 = getelementptr inbounds [1x i32], [1x i32]* %8, i64 0, i64 0
  store i32 %9, i32* %10, align 4
  ; Expression result: %8
  %11 = add i32 0, 0 ; placeholder
  ; Expression result: %11
  %12 = add i32 0, 0 ; placeholder
  ; Expression result: %12
  %13 = add i32 0, 0 ; placeholder
  ; Expression result: %13
  br label %label2
label2:
  br label %label0
label3:
  ret i32 0
}

define i32 @main() {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  %4 = alloca i8*, align 4
  store i8* %3, i8** %4, align 4
  ; Variable numbers allocated
  %5 = add i32 0, 0 ; placeholder
  ; Expression result: %5
  %6 = add i32 0, 0 ; placeholder
  ; Expression result: %6
  %7 = alloca [5x i32], align 4
  %8 = add i32 0, 0 ; placeholder
  %9 = getelementptr inbounds [5x i32], [5x i32]* %7, i64 0, i64 0
  store i32 %8, i32* %9, align 4
  %10 = add i32 0, 0 ; placeholder
  %11 = getelementptr inbounds [5x i32], [5x i32]* %7, i64 0, i64 1
  store i32 %10, i32* %11, align 4
  %12 = add i32 0, 0 ; placeholder
  %13 = getelementptr inbounds [5x i32], [5x i32]* %7, i64 0, i64 2
  store i32 %12, i32* %13, align 4
  %14 = add i32 0, 0 ; placeholder
  %15 = getelementptr inbounds [5x i32], [5x i32]* %7, i64 0, i64 3
  store i32 %14, i32* %15, align 4
  %16 = add i32 0, 0 ; placeholder
  %17 = getelementptr inbounds [5x i32], [5x i32]* %7, i64 0, i64 4
  store i32 %16, i32* %17, align 4
  ; Expression result: %7
  %18 = add i32 0, 0 ; placeholder
  %19 = call i32 @puts(i8* %18)
  %20 = add i32 0, 0
  ; Expression result: %20
  %21 = load i8*, i8** %4, align 4
  %22 = call i32 @print_array(i32 %21)
  ; Expression result: %22
  %23 = load i8*, i8** %4, align 4
  %24 = call i32 @bubble_sort(i32 %23)
  ; Expression result: %24
  %25 = add i32 0, 0 ; placeholder
  %26 = call i32 @puts(i8* %25)
  %27 = add i32 0, 0
  ; Expression result: %27
  %28 = load i8*, i8** %4, align 4
  %29 = call i32 @print_array(i32 %28)
  ; Expression result: %29
  %30 = add i32 0, 0 ; placeholder
  ; Expression result: %30
  %31 = load i8*, i8** %4, align 4
  %32 = call i32 @is_sorted(i32 %31)
  ; Expression result: %32
  %33 = add i32 0, 0 ; placeholder
  ; Expression result: %33
  %34 = add i32 0, 0 ; placeholder
  %35 = call i32 @puts(i8* %34)
  %36 = add i32 0, 0
  ; Expression result: %36
  ret i32 0
}

