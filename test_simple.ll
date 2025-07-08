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


; String constants
@.str.2 = private unnamed_addr constant [12 x i8] c"Testing E: \00", align 1
@.str.4 = private unnamed_addr constant [26 x i8] c"Testing abs_float(-5.5): \00", align 1
@.str.10 = private unnamed_addr constant [27 x i8] c"Testing sqrt_simple(9.0): \00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.7 = private unnamed_addr constant [32 x i8] c"Testing min_float(3.14, 2.71): \00", align 1
@.str.15 = private unnamed_addr constant [25 x i8] c"Testing ln_simple(1.0): \00", align 1
@.str.16 = private unnamed_addr constant [55 x i8] c"Testing approximately_equal_simple(3.14, 3.14, 0.01): \00", align 1
@.str.5 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.17 = private unnamed_addr constant [54 x i8] c"Simple math float module test completed successfully!\00", align 1
@.str.8 = private unnamed_addr constant [32 x i8] c"Testing max_float(3.14, 2.71): \00", align 1
@.str.0 = private unnamed_addr constant [13 x i8] c"Testing PI: \00", align 1
@.str.14 = private unnamed_addr constant [26 x i8] c"Testing exp_simple(1.0): \00", align 1
@.str.6 = private unnamed_addr constant [25 x i8] c"Testing abs_float(5.5): \00", align 1
@.str.3 = private unnamed_addr constant [14 x i8] c"Testing TAU: \00", align 1
@.str.12 = private unnamed_addr constant [26 x i8] c"Testing cos_simple(0.0): \00", align 1
@.str.11 = private unnamed_addr constant [26 x i8] c"Testing sin_simple(0.0): \00", align 1
@.str.13 = private unnamed_addr constant [26 x i8] c"Testing exp_simple(0.0): \00", align 1
@.str.9 = private unnamed_addr constant [27 x i8] c"Testing sqrt_simple(4.0): \00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.0, i64 0, i64 0
  ; Converting complex expression to output
  %1 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %2 = call i32 (i8*, ...) @printf(i8* %1, i32 %0)
  %3 = call i32 @PI()
  ; Converting complex expression to output
  %4 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %5 = call i32 (i8*, ...) @printf(i8* %4, i32 %3)
  %6 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.2, i64 0, i64 0
  ; Converting complex expression to output
  %7 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %8 = call i32 (i8*, ...) @printf(i8* %7, i32 %6)
  %9 = call i32 @E()
  ; Converting complex expression to output
  %10 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %11 = call i32 (i8*, ...) @printf(i8* %10, i32 %9)
  %12 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.3, i64 0, i64 0
  ; Converting complex expression to output
  %13 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %14 = call i32 (i8*, ...) @printf(i8* %13, i32 %12)
  %15 = call i32 @TAU()
  ; Converting complex expression to output
  %16 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %17 = call i32 (i8*, ...) @printf(i8* %16, i32 %15)
  %18 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.4, i64 0, i64 0
  ; Converting complex expression to output
  %19 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %20 = call i32 (i8*, ...) @printf(i8* %19, i32 %18)
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.5, i64 0, i64 0
  %23 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.6, i64 0, i64 0
  ; Converting complex expression to output
  %24 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %25 = call i32 (i8*, ...) @printf(i8* %24, i32 %23)
  %26 = call i32 @abs_float(i32 5.5)
  ; Converting complex expression to output
  %27 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %28 = call i32 (i8*, ...) @printf(i8* %27, i32 %26)
  %29 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.7, i64 0, i64 0
  ; Converting complex expression to output
  %30 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %31 = call i32 (i8*, ...) @printf(i8* %30, i32 %29)
  %32 = call i32 @min_float(i32 3.14, i32 2.71)
  ; Converting complex expression to output
  %33 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %34 = call i32 (i8*, ...) @printf(i8* %33, i32 %32)
  %35 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.8, i64 0, i64 0
  ; Converting complex expression to output
  %36 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %37 = call i32 (i8*, ...) @printf(i8* %36, i32 %35)
  %38 = call i32 @max_float(i32 3.14, i32 2.71)
  ; Converting complex expression to output
  %39 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %40 = call i32 (i8*, ...) @printf(i8* %39, i32 %38)
  %41 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.9, i64 0, i64 0
  ; Converting complex expression to output
  %42 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %43 = call i32 (i8*, ...) @printf(i8* %42, i32 %41)
  %44 = call i32 @sqrt_simple(i32 4)
  ; Converting complex expression to output
  %45 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %46 = call i32 (i8*, ...) @printf(i8* %45, i32 %44)
  %47 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.10, i64 0, i64 0
  ; Converting complex expression to output
  %48 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %49 = call i32 (i8*, ...) @printf(i8* %48, i32 %47)
  %50 = call i32 @sqrt_simple(i32 9)
  ; Converting complex expression to output
  %51 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %52 = call i32 (i8*, ...) @printf(i8* %51, i32 %50)
  %53 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.11, i64 0, i64 0
  ; Converting complex expression to output
  %54 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %55 = call i32 (i8*, ...) @printf(i8* %54, i32 %53)
  %56 = call i32 @sin_simple(i32 0)
  ; Converting complex expression to output
  %57 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %58 = call i32 (i8*, ...) @printf(i8* %57, i32 %56)
  %59 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.12, i64 0, i64 0
  ; Converting complex expression to output
  %60 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %61 = call i32 (i8*, ...) @printf(i8* %60, i32 %59)
  %62 = call i32 @cos_simple(i32 0)
  ; Converting complex expression to output
  %63 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %64 = call i32 (i8*, ...) @printf(i8* %63, i32 %62)
  %65 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.13, i64 0, i64 0
  ; Converting complex expression to output
  %66 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %67 = call i32 (i8*, ...) @printf(i8* %66, i32 %65)
  %68 = call i32 @exp_simple(i32 0)
  ; Converting complex expression to output
  %69 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %70 = call i32 (i8*, ...) @printf(i8* %69, i32 %68)
  %71 = getelementptr inbounds [26 x i8], [26 x i8]* @.str.14, i64 0, i64 0
  ; Converting complex expression to output
  %72 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %73 = call i32 (i8*, ...) @printf(i8* %72, i32 %71)
  %74 = call i32 @exp_simple(i32 1)
  ; Converting complex expression to output
  %75 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %76 = call i32 (i8*, ...) @printf(i8* %75, i32 %74)
  %77 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.15, i64 0, i64 0
  ; Converting complex expression to output
  %78 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %79 = call i32 (i8*, ...) @printf(i8* %78, i32 %77)
  %80 = call i32 @ln_simple(i32 1)
  ; Converting complex expression to output
  %81 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %82 = call i32 (i8*, ...) @printf(i8* %81, i32 %80)
  %83 = getelementptr inbounds [55 x i8], [55 x i8]* @.str.16, i64 0, i64 0
  ; Converting complex expression to output
  %84 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %85 = call i32 (i8*, ...) @printf(i8* %84, i32 %83)
  %86 = call i32 @approximately_equal_simple(i32 3.14, i32 3.14, i32 0.01)
  ; Converting complex expression to output
  %87 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %88 = call i32 (i8*, ...) @printf(i8* %87, i32 %86)
  %89 = getelementptr inbounds [54 x i8], [54 x i8]* @.str.17, i64 0, i64 0
  ; Converting complex expression to output
  %90 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.1, i64 0, i64 0
  %91 = call i32 (i8*, ...) @printf(i8* %90, i32 %89)
  ret i32 0
}
