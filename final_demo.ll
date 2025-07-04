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

define i32 @add(i32 %x, i32 %y) {
entry:
  %0 = add i32 %x, %y
  ret i32 %0
}

define i32 @multiply(i32 %a, i32 %b) {
entry:
  %0 = mul i32 %a, %b
  ret i32 %0
}


; String constants
@.str.0 = private unnamed_addr constant [23 x i8] c"Sum is greater than 10\00", align 1
@.str.1 = private unnamed_addr constant [32 x i8] c"Product is also greater than 40\00", align 1
@.str.2 = private unnamed_addr constant [22 x i8] c"Product is 40 or less\00", align 1
@.str.3 = private unnamed_addr constant [18 x i8] c"Sum is 10 or less\00", align 1
@.str.4 = private unnamed_addr constant [23 x i8] c"Sum equals exactly 15!\00", align 1
@.str.5 = private unnamed_addr constant [21 x i8] c"Boolean flag is true\00", align 1
define i32 @main() {
entry:
  %0 = alloca i32, align 4
  store i32 10, i32* %0, align 4
  ; Variable x allocated
  %1 = alloca i32, align 4
  store i32 5, i32* %1, align 4
  ; Variable y allocated
  %3 = load i32, i32* %0, align 4
  %4 = load i32, i32* %1, align 4
  %2 = call i32 @add(i32 %3, i32 %4)
  %5 = alloca i32, align 4
  store i32 %2, i32* %5, align 4
  ; Variable sum allocated
  %7 = load i32, i32* %0, align 4
  %8 = load i32, i32* %1, align 4
  %6 = call i32 @multiply(i32 %7, i32 %8)
  %9 = alloca i32, align 4
  store i32 %6, i32* %9, align 4
  ; Variable product allocated
  %10 = load i32, i32* %5, align 4
  %11 = icmp sgt i32 %10, 10
  br i1 %11, label %label0, label %label1
label0:
  %12 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.0, i64 0, i64 0
  %13 = call i32 @puts(i8* %12)
  %14 = add i32 0, 0
  ; Expression result: %14
  %15 = load i32, i32* %9, align 4
  %16 = icmp sgt i32 %15, 40
  br i1 %16, label %label3, label %label4
label3:
  %17 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.1, i64 0, i64 0
  %18 = call i32 @puts(i8* %17)
  %19 = add i32 0, 0
  ; Expression result: %19
  br label %label5
label4:
  %20 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.2, i64 0, i64 0
  %21 = call i32 @puts(i8* %20)
  %22 = add i32 0, 0
  ; Expression result: %22
  br label %label5
label5:
  br label %label2
label1:
  %23 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.3, i64 0, i64 0
  %24 = call i32 @puts(i8* %23)
  %25 = add i32 0, 0
  ; Expression result: %25
  br label %label2
label2:
  %26 = load i32, i32* %5, align 4
  %27 = icmp eq i32 %26, 15
  br i1 %27, label %label6, label %label7
label6:
  %28 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.4, i64 0, i64 0
  %29 = call i32 @puts(i8* %28)
  %30 = add i32 0, 0
  ; Expression result: %30
  br label %label8
label7:
  br label %label8
label8:
  %31 = alloca i32, align 4
  store i32 1, i32* %31, align 4
  ; Variable flag allocated
  %32 = load i32, i32* %31, align 4
  br i1 %32, label %label9, label %label10
label9:
  %33 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.5, i64 0, i64 0
  %34 = call i32 @puts(i8* %33)
  %35 = add i32 0, 0
  ; Expression result: %35
  br label %label11
label10:
  br label %label11
label11:
  %36 = load i32, i32* %5, align 4
  ret i32 %36
}

