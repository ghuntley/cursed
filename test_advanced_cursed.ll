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


; String constants
@.str.0 = private unnamed_addr constant [21 x i8] c"Destructured values:\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
define i32 @main() {
entry:
  %0 = add i32 0, 0 ; placeholder
  %1 = alloca i32, align 4
  store i32 %0, i32* %1, align 4
  ; Variable point allocated
  %2 = load i32, i32* %1, align 4
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Tuple destructuring let statement
  %tuple_field_4 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %2, i32 0, i32 0
  %tuple_val_5 = load i32, i32* %tuple_field_4, align 4
  ; Extracted x = %tuple_val_5 from tuple
  %tuple_field_6 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %2, i32 0, i32 1
  %tuple_val_7 = load i32, i32* %tuple_field_6, align 4
  ; Extracted y = %tuple_val_7 from tuple
  %tuple_field_8 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %2, i32 0, i32 2
  %tuple_val_9 = load i32, i32* %tuple_field_8, align 4
  ; Extracted z = %tuple_val_9 from tuple
  %10 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.0, i64 0, i64 0
  %11 = call i32 @puts(i8* %10)
  %12 = add i32 0, 0
  ; Expression result: %12
  %13 = load i32, i32* %tuple_val_5, align 4
  %14 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.fmt.d.1, i64 0, i64 0
  %15 = call i32 (i8*, ...) @printf(i8* %14, i32 %13)
  %16 = add i32 0, 0
  ; Expression result: %16
  %17 = load i32, i32* %tuple_val_7, align 4
  %18 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %19 = call i32 (i8*, ...) @printf(i8* %18, i32 %17)
  %20 = add i32 0, 0
  ; Expression result: %20
  %21 = load i32, i32* %tuple_val_9, align 4
  %22 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %23 = call i32 (i8*, ...) @printf(i8* %22, i32 %21)
  %24 = add i32 0, 0
  ; Expression result: %24
  %25 = load i32, i32* %tuple_val_5, align 4
  %26 = load i32, i32* %tuple_val_7, align 4
  %27 = add i32 %25, %26
  %28 = load i32, i32* %tuple_val_9, align 4
  %29 = add i32 %27, %28
  ret i32 %29
}

