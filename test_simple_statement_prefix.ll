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


; String constants
@.str.3 = private unnamed_addr constant [22 x i8] c"sum is greater than 2\00", align 1
@.str.5 = private unnamed_addr constant [28 x i8] c"inner is greater than outer\00", align 1
@.str.4 = private unnamed_addr constant [23 x i8] c"old syntax still works\00", align 1
@.str.1 = private unnamed_addr constant [25 x i8] c"count is greater than 10\00", align 1
@.str.0 = private unnamed_addr constant [14 x i8] c"x is positive\00", align 1
@.str.2 = private unnamed_addr constant [18 x i8] c"i is now positive\00", align 1
define i32 @main() {
entry:
  %1 = load i32, i32* %0, align 4
  %2 = icmp sgt i32 %1, 0
  %0 = alloca i32, align 4
  store i32 5, i32* %0, align 4
  ; Short declaration: x := 5
  br i1 %2, label %label0, label %label1
label0:
  %3 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.0, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  br label %label2
label1:
  br label %label2
label2:
  %6 = alloca i32, align 4
  store i32 10, i32* %6, align 4
  ; Variable count allocated
  %9 = load i32, i32* %6, align 4
  %10 = icmp sgt i32 %9, 10
  %7 = load i32, i32* %6, align 4
  %8 = add i32 %7, 1
  store i32 %8, i32* %6, align 4
  ; Assignment to count = %8
  br i1 %10, label %label3, label %label4
label3:
  %11 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.1, i64 0, i64 0
  %12 = call i32 @puts(i8* %11)
  %13 = add i32 0, 0
  ; Expression result: %13
  br label %label5
label4:
  br label %label5
label5:
  %14 = alloca i32, align 4
  store i32 0, i32* %14, align 4
  ; Variable i allocated
  %15 = load i32, i32* %14, align 4
  %16 = icmp sgt i32 %15, 0
  ; Unsupported statement
  br i1 %16, label %label6, label %label7
label6:
  %17 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.2, i64 0, i64 0
  %18 = call i32 @puts(i8* %17)
  %19 = add i32 0, 0
  ; Expression result: %19
  br label %label8
label7:
  br label %label8
label8:
  ; Unsupported statement
  %29 = load i32, i32* %25, align 4
  %30 = load i32, i32* %28, align 4
  %31 = add i32 %29, %30
  %32 = icmp sgt i32 %31, 2
  %20 = alloca {i32, i32, i32}, align 4
  %21 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %20, i32 0, i32 0
  store i32 1, i32* %21, align 4
  %22 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %20, i32 0, i32 1
  store i32 2, i32* %22, align 4
  ; Tuple destructuring short declaration in function
  %23 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %20, i32 0, i32 0
  %24 = load i32, i32* %23, align 4
  %25 = alloca i32, align 4
  store i32 %24, i32* %25, align 4
  ; Short declaration: a := %24 from tuple
  %26 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %20, i32 0, i32 1
  %27 = load i32, i32* %26, align 4
  %28 = alloca i32, align 4
  store i32 %27, i32* %28, align 4
  ; Short declaration: b := %27 from tuple
  br i1 %32, label %label9, label %label10
label9:
  %33 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.3, i64 0, i64 0
  %34 = call i32 @puts(i8* %33)
  %35 = add i32 0, 0
  ; Expression result: %35
  br label %label11
label10:
  br label %label11
label11:
  %36 = icmp sgt i32 5, 3
  br i1 %36, label %label12, label %label13
label12:
  %37 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.4, i64 0, i64 0
  %38 = call i32 @puts(i8* %37)
  %39 = add i32 0, 0
  ; Expression result: %39
  br label %label14
label13:
  br label %label14
label14:
  %41 = load i32, i32* %40, align 4
  %42 = icmp sgt i32 %41, 5
  %40 = alloca i32, align 4
  store i32 10, i32* %40, align 4
  ; Short declaration: outer := 10
  br i1 %42, label %label15, label %label16
label15:
  %44 = load i32, i32* %43, align 4
  %45 = load i32, i32* %40, align 4
  %46 = icmp sgt i32 %44, %45
  %43 = alloca i32, align 4
  store i32 20, i32* %43, align 4
  ; Short declaration: inner := 20
  br i1 %46, label %label18, label %label19
label18:
  %47 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.5, i64 0, i64 0
  %48 = call i32 @puts(i8* %47)
  %49 = add i32 0, 0
  ; Expression result: %49
  br label %label20
label19:
  br label %label20
label20:
  br label %label17
label16:
  br label %label17
label17:
  ret i32 0
}

