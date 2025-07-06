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
declare i8* @string_concat(i8*, i8*)

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

define i32 @calculateArea(i32 %radius) {
entry:
  %0 = alloca double, align 4
  store double 3.14159, double* %0, align 4
  ; Variable PI allocated
  %1 = load double, double* %0, align 4
  %2 = sitofp i32 %radius to double
  %3 = fmul double %1, %2
  %4 = sitofp i32 %radius to double
  %5 = fmul double %3, %4
  %6 = alloca i32, align 4
  %7 = fptosi double %5 to i32
  store i32 %7, i32* %6, align 4
  ; Variable area allocated
  %8 = load i32, i32* %6, align 4
  ret i32 %8
}

define void @greetUser(i8* %name, i32 %age) {
entry:
  %0 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.0, i64 0, i64 0
  %1 = call i8* @string_concat(i8* %0, i8* %name)
  %2 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.1, i64 0, i64 0
  %3 = call i8* @string_concat(i8* %1, i8* %2)
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.2, i64 0, i64 0
  %7 = call i8* @i32_to_string(i32 %age)
  %8 = call i8* @string_concat(i8* %6, i8* %7)
  %9 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.3, i64 0, i64 0
  %10 = call i8* @string_concat(i8* %8, i8* %9)
  %11 = call i32 @puts(i8* %10)
  %12 = add i32 0, 0
  ; Expression result: %12
  %13 = icmp sge i32 %age, 18
  br i1 %13, label %label0, label %label1
label0:
  %14 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.4, i64 0, i64 0
  %15 = call i32 @puts(i8* %14)
  %16 = add i32 0, 0
  ; Expression result: %16
  br label %label2
label1:
  %17 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.5, i64 0, i64 0
  %18 = call i32 @puts(i8* %17)
  %19 = add i32 0, 0
  ; Expression result: %19
  br label %label2
label2:
  ret void
}



; String constants
@.str.1 = private unnamed_addr constant [2 x i8] c"!\00", align 1
@.str.7 = private unnamed_addr constant [6 x i8] c" is: \00", align 1
@.str.0 = private unnamed_addr constant [8 x i8] c"Hello, \00", align 1
@.str.9 = private unnamed_addr constant [4 x i8] c"Bob\00", align 1
@.str.5 = private unnamed_addr constant [17 x i8] c"You are a minor!\00", align 1
@.str.8 = private unnamed_addr constant [6 x i8] c"Alice\00", align 1
@.str.2 = private unnamed_addr constant [9 x i8] c"You are \00", align 1
@.str.3 = private unnamed_addr constant [11 x i8] c" years old\00", align 1
@.str.4 = private unnamed_addr constant [18 x i8] c"You are an adult!\00", align 1
@.str.6 = private unnamed_addr constant [28 x i8] c"Area of circle with radius \00", align 1
define i32 @main() {
  %1 = alloca i32, align 4
  store i32 5, i32* %1, align 4
  ; Variable radius allocated at %1
  %2 = load i32, i32* %1, align 4
  %3 = call i32 @calculateArea(i32 %2)
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable result allocated at %4
  %5 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.6, i64 0, i64 0
  %6 = load i32, i32* %1, align 4
  %7 = call i8* @i32_to_string(i32 %6)
  %8 = call i8* @string_concat(i8* %5, i8* %7)
  %9 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.7, i64 0, i64 0
  %10 = call i8* @string_concat(i8* %8, i8* %9)
  %11 = load i32, i32* %4, align 4
  %12 = add i32 %10, %11
  %13 = call i32 @puts(i8* %12)
  %14 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.8, i64 0, i64 0
  %15 = call i32 @greetUser(i32 %14, i32 25)
  %16 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.9, i64 0, i64 0
  %17 = call i32 @greetUser(i32 %16, i32 16)
  ret i32 0
}
