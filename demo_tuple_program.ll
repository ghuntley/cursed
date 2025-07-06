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
@.str.0 = private unnamed_addr constant [29 x i8] c"🎉 CURSED Tuple Demo! 🎉\00", align 1
@.str.1 = private unnamed_addr constant [13 x i8] c"Coordinates:\00", align 1
@.str.2 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.3 = private unnamed_addr constant [13 x i8] c"Person info:\00", align 1
@.str.4 = private unnamed_addr constant [21 x i8] c"After destructuring:\00", align 1
@.str.5 = private unnamed_addr constant [22 x i8] c"First nested element:\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = add i32 0, 0 ; placeholder
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable coordinates allocated
  %5 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.1, i64 0, i64 0
  %6 = call i32 @puts(i8* %5)
  %7 = add i32 0, 0
  ; Expression result: %7
  %8 = add i32 0, 0 ; placeholder
  %9 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.fmt.d.2, i64 0, i64 0
  %10 = call i32 (i8*, ...) @printf(i8* %9, i32 %8)
  %11 = add i32 0, 0
  ; Expression result: %11
  %12 = add i32 0, 0 ; placeholder
  %13 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %14 = call i32 (i8*, ...) @printf(i8* %13, i32 %12)
  %15 = add i32 0, 0
  ; Expression result: %15
  %16 = add i32 0, 0 ; placeholder
  %17 = alloca i32, align 4
  store i32 %16, i32* %17, align 4
  ; Variable person_info allocated
  %18 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.3, i64 0, i64 0
  %19 = call i32 @puts(i8* %18)
  %20 = add i32 0, 0
  ; Expression result: %20
  %21 = add i32 0, 0 ; placeholder
  %22 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %23 = call i32 (i8*, ...) @printf(i8* %22, i32 %21)
  %24 = add i32 0, 0
  ; Expression result: %24
  %25 = add i32 0, 0 ; placeholder
  %26 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %27 = call i32 (i8*, ...) @printf(i8* %26, i32 %25)
  %28 = add i32 0, 0
  ; Expression result: %28
  %29 = add i32 0, 0 ; placeholder
  %30 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %31 = call i32 (i8*, ...) @printf(i8* %30, i32 %29)
  %32 = add i32 0, 0
  ; Expression result: %32
  %33 = load i32, i32* %17, align 4
  ; Tuple destructuring assignment in function
  %tuple_field_34 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %33, i32 0, i32 0
  %tuple_val_35 = load i32, i32* %tuple_field_34, align 4
  ; Extracted name = %tuple_val_35 from tuple
  %tuple_field_36 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %33, i32 0, i32 1
  %tuple_val_37 = load i32, i32* %tuple_field_36, align 4
  ; Extracted age = %tuple_val_37 from tuple
  %tuple_field_38 = getelementptr inbounds {i32, i32, i32}, {i32, i32, i32}* %33, i32 0, i32 2
  %tuple_val_39 = load i32, i32* %tuple_field_38, align 4
  ; Extracted is_awesome = %tuple_val_39 from tuple
  %40 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.4, i64 0, i64 0
  %41 = call i32 @puts(i8* %40)
  %42 = add i32 0, 0
  ; Expression result: %42
  %43 = load i8*, i8** %tuple_val_35, align 8
  %44 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %45 = call i32 (i8*, ...) @printf(i8* %44, i32 %43)
  %46 = add i32 0, 0
  ; Expression result: %46
  %47 = load i32, i32* %tuple_val_37, align 4
  %48 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %49 = call i32 (i8*, ...) @printf(i8* %48, i32 %47)
  %50 = add i32 0, 0
  ; Expression result: %50
  %51 = load i1, i1* %tuple_val_39, align 1
  %52 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %53 = call i32 (i8*, ...) @printf(i8* %52, i32 %51)
  %54 = add i32 0, 0
  ; Expression result: %54
  %55 = add i32 0, 0 ; placeholder
  %56 = alloca i32, align 4
  store i32 %55, i32* %56, align 4
  ; Variable nested allocated
  %57 = add i32 0, 0 ; placeholder
  %58 = alloca i32, align 4
  store i32 %57, i32* %58, align 4
  ; Variable first_pair allocated
  %59 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.5, i64 0, i64 0
  %60 = call i32 @puts(i8* %59)
  %61 = add i32 0, 0
  ; Expression result: %61
  %62 = add i32 0, 0 ; placeholder
  %63 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %64 = call i32 (i8*, ...) @printf(i8* %63, i32 %62)
  %65 = add i32 0, 0
  ; Expression result: %65
  %66 = add i32 0, 0 ; placeholder
  %67 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %68 = call i32 (i8*, ...) @printf(i8* %67, i32 %66)
  %69 = add i32 0, 0
  ; Expression result: %69
  %70 = load i32, i32* %4, align 4
  ret i32 %70
}

