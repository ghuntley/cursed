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
@.str.5 = private unnamed_addr constant [4 x i8] c"4.0\00", align 1
@.str.6 = private unnamed_addr constant [4 x i8] c"5.0\00", align 1
@.str.2 = private unnamed_addr constant [4 x i8] c"1.0\00", align 1
@.str.0 = private unnamed_addr constant [30 x i8] c"Pure CURSED Math Module Tests\00", align 1
@.str.1 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.4 = private unnamed_addr constant [4 x i8] c"3.0\00", align 1
@.str.3 = private unnamed_addr constant [4 x i8] c"2.0\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  %2 = call i32 @add(i32 2.5, i32 3.5)
  %3 = call i32 @assert_eq_float(i32 %2, i32 6)
  %4 = call i32 @subtract(i32 10, i32 4)
  %5 = call i32 @assert_eq_float(i32 %4, i32 6)
  %6 = call i32 @multiply(i32 3, i32 4)
  %7 = call i32 @assert_eq_float(i32 %6, i32 12)
  %8 = call i32 @divide(i32 15, i32 3)
  %9 = call i32 @assert_eq_float(i32 %8, i32 5)
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %12 = call i32 @pi()
  %13 = icmp slt i32 %12, 3.15
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %15 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %17 = call i32 @e()
  %18 = icmp slt i32 %17, 2.72
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %22 = call i32 @tau()
  %23 = icmp slt i32 %22, 6.29
  %24 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %27 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %28 = call i32 @abs(i32 5.5)
  %29 = call i32 @assert_eq_float(i32 %28, i32 5.5)
  %30 = call i32 @abs(i32 0)
  %31 = call i32 @assert_eq_float(i32 %30, i32 0)
  %32 = call i32 @min(i32 3, i32 7)
  %33 = call i32 @assert_eq_float(i32 %32, i32 3)
  %34 = call i32 @max(i32 3, i32 7)
  %35 = call i32 @assert_eq_float(i32 %34, i32 7)
  %36 = call i32 @clamp(i32 5, i32 2, i32 8)
  %37 = call i32 @assert_eq_float(i32 %36, i32 5)
  %38 = call i32 @clamp(i32 1, i32 2, i32 8)
  %39 = call i32 @assert_eq_float(i32 %38, i32 2)
  %40 = call i32 @clamp(i32 10, i32 2, i32 8)
  %41 = call i32 @assert_eq_float(i32 %40, i32 8)
  %42 = call i32 @sign(i32 5)
  %43 = call i32 @assert_eq_float(i32 %42, i32 1)
  %44 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %45 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %46 = sub i32 %45, 1
  %47 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %48 = call i32 @sign(i32 0)
  %49 = call i32 @assert_eq_float(i32 %48, i32 0)
  %50 = call i32 @power(i32 2, i32 3)
  %51 = call i32 @assert_eq_float(i32 %50, i32 8)
  %52 = call i32 @power(i32 5, i32 2)
  %53 = call i32 @assert_eq_float(i32 %52, i32 25)
  %54 = alloca i32, align 4
  store i32 null, i32* %54, align 4
  ; Variable sqrt_result allocated at %54
  %55 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %56 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %57 = call i32 @sqrt(i32 16)
  %58 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %59 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %60 = load i32, i32* %54, align 4
  %61 = icmp slt i32 %60, 4.01
  %62 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %63 = alloca i32, align 4
  store i32 null, i32* %63, align 4
  ; Variable sin_result allocated at %63
  %64 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %65 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %66 = call i32 @sin(i32 0)
  %67 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %68 = sub i32 %67, 0.01
  %69 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %70 = load i32, i32* %63, align 4
  %71 = icmp slt i32 %70, 0.01
  %72 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %73 = alloca i32, align 4
  store i32 null, i32* %73, align 4
  ; Variable cos_result allocated at %73
  %74 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %75 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %76 = call i32 @cos(i32 0)
  %77 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %78 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %79 = load i32, i32* %73, align 4
  %80 = icmp slt i32 %79, 1.01
  %81 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %82 = call i32 @floor(i32 3.7)
  %83 = call i32 @assert_eq_float(i32 %82, i32 3)
  %84 = call i32 @ceil(i32 3.2)
  %85 = call i32 @assert_eq_float(i32 %84, i32 4)
  %86 = call i32 @round(i32 3.6)
  %87 = call i32 @assert_eq_float(i32 %86, i32 4)
  %88 = call i32 @trunc(i32 3.9)
  %89 = call i32 @assert_eq_float(i32 %88, i32 3)
  %90 = alloca i32, align 4
  store i32 null, i32* %90, align 4
  ; Variable ln_result allocated at %90
  %91 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %92 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %93 = call i32 @e()
  %94 = call i32 @ln(i32 %93)
  %95 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %96 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %97 = load i32, i32* %90, align 4
  %98 = icmp slt i32 %97, 1.01
  %99 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %100 = alloca i32, align 4
  store i32 null, i32* %100, align 4
  ; Variable exp_result allocated at %100
  %101 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %102 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %103 = call i32 @exp(i32 1)
  %104 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %105 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %106 = load i32, i32* %100, align 4
  %107 = icmp slt i32 %106, 2.72
  %108 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %109 = call i32 @seed_random(i32 42)
  %110 = alloca i32, align 4
  store i32 null, i32* %110, align 4
  ; Variable rand1 allocated at %110
  %111 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %112 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %113 = call i32 @random()
  %114 = alloca i32, align 4
  store i32 null, i32* %114, align 4
  ; Variable rand2 allocated at %114
  %115 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %116 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %117 = call i32 @random()
  %118 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %119 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %120 = load i32, i32* %110, align 4
  %121 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %122 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %123 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %124 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %125 = load i32, i32* %114, align 4
  %126 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %127 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %128 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %129 = load i32, i32* %114, align 4
  %130 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %131 = call i32 @random_int(i32 1, i32 10)
  %132 = alloca i32, align 4
  store i32 %131, i32* %132, align 4
  ; Variable rand_int allocated at %132
  %133 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %134 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %135 = load i32, i32* %132, align 4
  %136 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %137 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %138 = alloca [5 x i32], align 4
  %139 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.2, i64 0, i64 0
  %140 = getelementptr inbounds [5 x i32], [5 x i32]* %138, i64 0, i64 0
  store i32 %139, i32* %140, align 4
  %141 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.3, i64 0, i64 0
  %142 = getelementptr inbounds [5 x i32], [5 x i32]* %138, i64 0, i64 1
  store i32 %141, i32* %142, align 4
  %143 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.4, i64 0, i64 0
  %144 = getelementptr inbounds [5 x i32], [5 x i32]* %138, i64 0, i64 2
  store i32 %143, i32* %144, align 4
  %145 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.5, i64 0, i64 0
  %146 = getelementptr inbounds [5 x i32], [5 x i32]* %138, i64 0, i64 3
  store i32 %145, i32* %146, align 4
  %147 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.6, i64 0, i64 0
  %148 = getelementptr inbounds [5 x i32], [5 x i32]* %138, i64 0, i64 4
  store i32 %147, i32* %148, align 4
  %149 = alloca i32, align 4
  store i32 %138, i32* %149, align 4
  ; Variable values allocated at %149
  %150 = load i32, i32* %149, align 4
  %151 = call i32 @sum(i32 %150)
  %152 = call i32 @assert_eq_float(i32 %151, i32 15)
  %153 = load i32, i32* %149, align 4
  %154 = call i32 @mean(i32 %153)
  %155 = call i32 @assert_eq_float(i32 %154, i32 3)
  %156 = alloca i32, align 4
  store i32 null, i32* %156, align 4
  ; Variable variance_result allocated at %156
  %157 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %158 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %159 = load i32, i32* %149, align 4
  %160 = call i32 @variance(i32 %159)
  %161 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %162 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %163 = load i32, i32* %156, align 4
  %164 = icmp slt i32 %163, 2.1
  %165 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %166 = call i32 @is_nan(i32 5)
  %167 = call i32 @assert_false(i32 %166)
  %168 = call i32 @is_finite(i32 5)
  %169 = call i32 @assert_true(i32 %168)
  %170 = alloca i32, align 4
  store i32 null, i32* %170, align 4
  ; Variable deg_result allocated at %170
  %171 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %172 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %173 = call i32 @pi()
  %174 = call i32 @degrees(i32 %173)
  %175 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %176 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %177 = load i32, i32* %170, align 4
  %178 = icmp slt i32 %177, 180.1
  %179 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %180 = alloca i32, align 4
  store i32 null, i32* %180, align 4
  ; Variable rad_result allocated at %180
  %181 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %182 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %183 = call i32 @radians(i32 180)
  %184 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %185 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %186 = load i32, i32* %180, align 4
  %187 = icmp slt i32 %186, 3.15
  %188 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %189 = call i32 @gcd(i32 12, i32 8)
  %190 = call i32 @assert_eq_int(i32 %189, i32 4)
  %191 = call i32 @lcm(i32 4, i32 6)
  %192 = call i32 @assert_eq_int(i32 %191, i32 12)
  %193 = call i32 @factorial(i32 5)
  %194 = call i32 @assert_eq_int(i32 %193, i32 120)
  %195 = call i32 @fibonacci(i32 7)
  %196 = call i32 @assert_eq_int(i32 %195, i32 13)
  %197 = alloca i32, align 4
  store i32 null, i32* %197, align 4
  ; Variable distance allocated at %197
  %198 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %199 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %200 = call i32 @distance_2d(i32 0, i32 0, i32 3, i32 4)
  %201 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %202 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %203 = load i32, i32* %197, align 4
  %204 = icmp slt i32 %203, 5.01
  %205 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %206 = call i32 @dot_product_2d(i32 1, i32 2, i32 3, i32 4)
  %207 = call i32 @assert_eq_float(i32 %206, i32 11)
  %208 = alloca i32, align 4
  store i32 null, i32* %208, align 4
  ; Variable lerp_result allocated at %208
  %209 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %210 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %211 = call i32 @lerp(i32 0, i32 10, i32 0.5)
  %212 = load i32, i32* %208, align 4
  %213 = call i32 @assert_eq_float(i32 %212, i32 5)
  %214 = call i32 @print_test_summary()
  ret i32 0
}
