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
@_ZTI11CursedError = constant { i8*, i8* } { i8* null, i8* getelementptr inbounds ([14 x i8], [14 x i8]* @_ZTS11CursedError, i32 0, i32 0) }
@_ZTS11CursedError = constant [14 x i8] c"11CursedError\00"

define i32 @calculateArea(i32 %radius) {
entry:
  %0 = mul i32 %radius, %radius
  %1 = mul i32 %0, %PI
  ret i32 %1
}


; String constants
@.str.0 = private unnamed_addr constant [14 x i8] c"Large circle!\00", align 1
@.str.1 = private unnamed_addr constant [14 x i8] c"Small circle!\00", align 1
define i32 @main() {
entry:
  %0 = alloca i32, align 4
  store i32 5, i32* %0, align 4
  ; Variable radius allocated
  %1 = load i32, i32* %0, align 4
  %2 = call i32 @calculateArea(i32 %1)
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable area allocated
  %4 = load i32, i32* %3, align 4
  %5 = icmp sgt i32 %4, 70
  br i1 %5, label %label0, label %label1
label0:
  %6 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.0, i64 0, i64 0
  %7 = call i32 @puts(i8* %6)
  %8 = add i32 0, 0
  ; Expression result: %8
  br label %label2
label1:
  %9 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.1, i64 0, i64 0
  %10 = call i32 @puts(i8* %9)
  %11 = add i32 0, 0
  ; Expression result: %11
  br label %label2
label2:
  %12 = load i32, i32* %3, align 4
  ret i32 %12
}

