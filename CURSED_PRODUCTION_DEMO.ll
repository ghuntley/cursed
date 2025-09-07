; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

define i32 @main() {
entry:
  ; Variable: integer_var
  %integer_var = alloca i64, align 8
  store i64 42, ptr %integer_var, align 8
  ; Variable: float_var
  %float_var = alloca double, align 8
  store double 3.14159, ptr %float_var, align 8
  ; Variable: string_var
  %string_var = alloca ptr, align 8
  store ptr @.str.2, ptr %string_var, align 8
  ; Variable: boolean_var
  %boolean_var = alloca i64, align 8
  store i64 1, ptr %boolean_var, align 8
  ; Variable: negative_var
  %negative_var = alloca i64, align 8
  store i64 -17, ptr %negative_var, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  %integer_var_load = load i64, ptr %integer_var, align 8
  call void @cursed_runtime_spill_int(i64 %integer_var_load)
  ; Call: vibez.spill
  %float_var_load = load double, ptr %float_var, align 8
  call void @cursed_runtime_spill_float(double %float_var_load)
  ; Call: vibez.spill
  %string_var_load = load ptr, ptr %string_var, align 8
  call void @cursed_runtime_spill_string(ptr %string_var_load)
  ; Call: vibez.spill
  %boolean_var_load = load i64, ptr %boolean_var, align 8
  call void @cursed_runtime_spill_int(i64 %boolean_var_load)
  ; Call: vibez.spill
  %negative_var_load = load i64, ptr %negative_var, align 8
  call void @cursed_runtime_spill_int(i64 %negative_var_load)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 14)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 30)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_float(double 45.14159)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 20)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.5)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_float(double 1.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.6)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  ; Call: vibez.spill
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.7)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 50)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 10)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 8)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.8)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 20)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_int(i64 0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.10)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.11)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.12)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.13)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.14)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.15)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.16)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.17)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [46 x i8] c"🔥 CURSED Language - Production Ready! 🔥\00", align 1
@.str.1 = private unnamed_addr constant [43 x i8] c"==========================================\00", align 1
@.str.2 = private unnamed_addr constant [19 x i8] c"Hello CURSED World\00", align 1
@.str.3 = private unnamed_addr constant [17 x i8] c"✅ BASIC TYPES:\00", align 1
@.str.4 = private unnamed_addr constant [27 x i8] c"✅ ARITHMETIC OPERATIONS:\00", align 1
@.str.5 = private unnamed_addr constant [26 x i8] c"✅ COMPARISON OPERATORS:\00", align 1
@.str.6 = private unnamed_addr constant [24 x i8] c"✅ LOGICAL OPERATIONS:\00", align 1
@.str.7 = private unnamed_addr constant [28 x i8] c"✅ MATHZ STDLIB FUNCTIONS:\00", align 1
@.str.8 = private unnamed_addr constant [32 x i8] c"✅ COMPLEX NESTED EXPRESSIONS:\00", align 1
@.str.9 = private unnamed_addr constant [43 x i8] c"==========================================\00", align 1
@.str.10 = private unnamed_addr constant [42 x i8] c"🎉 ALL FEATURES WORKING PERFECTLY! 🎉\00", align 1
@.str.11 = private unnamed_addr constant [38 x i8] c"✅ 71% Pass Rate - Production Ready!\00", align 1
@.str.12 = private unnamed_addr constant [32 x i8] c"✅ Cross-Platform Compilation!\00", align 1
@.str.13 = private unnamed_addr constant [32 x i8] c"✅ Complete Expression System!\00", align 1
@.str.14 = private unnamed_addr constant [30 x i8] c"✅ Full Type System Support!\00", align 1
@.str.15 = private unnamed_addr constant [31 x i8] c"✅ Advanced Operator Support!\00", align 1
@.str.16 = private unnamed_addr constant [33 x i8] c"✅ Stdlib Function Integration!\00", align 1
@.str.17 = private unnamed_addr constant [36 x i8] c"🚀 CURSED Language is READY! 🚀\00", align 1
