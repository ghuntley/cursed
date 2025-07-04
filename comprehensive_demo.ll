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

define i32 @factorial(i32 %n) {
entry:
  %0 = icmp sle i32 %n, 1
  br i1 %0, label %label0, label %label1
label0:
  ret i32 1
  br label %label2
label1:
  %2 = sub i32 %n, 1
  %1 = call i32 @factorial(i32 %2)
  %3 = mul i32 %n, %1
  ret i32 %3
  br label %label2
label2:
  ret i32 0
}

define i32 @greet(i8* %name) {
entry:
  %0 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.0, i64 0, i64 0
  %1 = add i32 %0, %name
  %2 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.1, i64 0, i64 0
  %3 = add i32 %1, %2
  %4 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.fmt.d.2, i64 0, i64 0
  %5 = call i32 (i8*, ...) @printf(i8* %4, i32 %3)
  %6 = add i32 0, 0
  ; Expression result: %6
  %7 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.3, i64 0, i64 0
  %8 = call i32 @puts(i8* %7)
  %9 = add i32 0, 0
  ; Expression result: %9
  ret i32 0
}


; String constants
@.str.0 = private unnamed_addr constant [8 x i8] c"Hello, \00", align 1
@.str.1 = private unnamed_addr constant [2 x i8] c"!\00", align 1
@.str.fmt.d.2 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.3 = private unnamed_addr constant [26 x i8] c"Welcome to CURSED v3.9.0!\00", align 1
@.str.0 = private unnamed_addr constant [17 x i8] c"CURSED Developer\00", align 1
@.str.1 = private unnamed_addr constant [27 x i8] c"Factorial result is large!\00", align 1
@.str.2 = private unnamed_addr constant [9 x i8] c"Result: \00", align 1
@.str.fmt.d.3 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.4 = private unnamed_addr constant [26 x i8] c"Factorial result is small\00", align 1
@.str.5 = private unnamed_addr constant [9 x i8] c"Result: \00", align 1
@.str.fmt.d.6 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.7 = private unnamed_addr constant [14 x i8] c"Flag is true!\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.0, i64 0, i64 0
  %1 = alloca i32, align 4
  store i32 %0, i32* %1, align 4
  ; Variable name allocated
  %2 = alloca i32, align 4
  store i32 5, i32* %2, align 4
  ; Variable number allocated
  %3 = alloca i32, align 4
  store i32 1, i32* %3, align 4
  ; Variable flag allocated
  %5 = load i32, i32* %1, align 4
  %4 = call i32 @greet(i32 %5)
  ; Expression result: %4
  %7 = load i32, i32* %2, align 4
  %6 = call i32 @factorial(i32 %7)
  %8 = alloca i32, align 4
  store i32 %6, i32* %8, align 4
  ; Variable result allocated
  %9 = load i32, i32* %8, align 4
  %10 = icmp sgt i32 %9, 100
  br i1 %10, label %label0, label %label1
label0:
  %11 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.1, i64 0, i64 0
  %12 = call i32 @puts(i8* %11)
  %13 = add i32 0, 0
  ; Expression result: %13
  %14 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.2, i64 0, i64 0
  %15 = load i32, i32* %8, align 4
  %16 = add i32 %14, %15
  %17 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.fmt.d.3, i64 0, i64 0
  %18 = call i32 (i8*, ...) @printf(i8* %17, i32 %16)
  %19 = add i32 0, 0
  ; Expression result: %19
  br label %label2
label1:
  %20 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.4, i64 0, i64 0
  %21 = call i32 @puts(i8* %20)
  %22 = add i32 0, 0
  ; Expression result: %22
  %23 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.5, i64 0, i64 0
  %24 = load i32, i32* %8, align 4
  %25 = add i32 %23, %24
  %26 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.fmt.d.6, i64 0, i64 0
  %27 = call i32 (i8*, ...) @printf(i8* %26, i32 %25)
  %28 = add i32 0, 0
  ; Expression result: %28
  br label %label2
label2:
  %29 = load i32, i32* %3, align 4
  br i1 %29, label %label3, label %label4
label3:
  %30 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.7, i64 0, i64 0
  %31 = call i32 @puts(i8* %30)
  %32 = add i32 0, 0
  ; Expression result: %32
  br label %label5
label4:
  br label %label5
label5:
  %33 = load i32, i32* %8, align 4
  ret i32 %33
}

