; Generated LLVM IR from CURSED with REAL program data
target triple = "x86_64-unknown-linux-gnu"

; CURSED Runtime Function Declarations
declare void @cursed_runtime_spill_string(ptr)
declare void @cursed_runtime_spill_int(i64)
declare void @cursed_runtime_spill_float(double)
declare void @cursed_runtime_spill_bool(i64)

define i32 @main() {
entry:
  ; Variable: test5
  %test5 = alloca i64, align 8
  store i64 1, ptr %test5, align 8
  ; Variable: val2
  %val2 = alloca i64, align 8
  store i64 10, ptr %val2, align 8
  ; Variable: logic4
  %logic4 = alloca i64, align 8
  store i64 0, ptr %logic4, align 8
  ; Variable: test1
  %test1 = alloca i64, align 8
  store i64 1, ptr %test1, align 8
  ; Variable: result1
  %result1 = alloca i64, align 8
  store i64 14, ptr %result1, align 8
  ; Variable: expr5
  %expr5 = alloca i64, align 8
  store i64 44, ptr %expr5, align 8
  ; Variable: val3
  %val3 = alloca i64, align 8
  store i64 15, ptr %val3, align 8
  ; Variable: logic3
  %logic3 = alloca i64, align 8
  store i64 1, ptr %logic3, align 8
  ; Variable: func_result1
  %func_result1 = alloca i64, align 8
  store i64 8, ptr %func_result1, align 8
  ; Variable: chain1
  %chain1 = alloca i64, align 8
  store i64 0, ptr %chain1, align 8
  ; Variable: result4
  %result4 = alloca i64, align 8
  store i64 24, ptr %result4, align 8
  ; Variable: func_result3
  %func_result3 = alloca i64, align 8
  store i64 1, ptr %func_result3, align 8
  ; Variable: result6
  %result6 = alloca i64, align 8
  store i64 4, ptr %result6, align 8
  ; Variable: assign_test1
  %assign_test1 = alloca i64, align 8
  store i64 0, ptr %assign_test1, align 8
  ; Variable: complex_arg1
  %complex_arg1 = alloca i64, align 8
  store i64 0, ptr %complex_arg1, align 8
  ; Variable: test4
  %test4 = alloca i64, align 8
  store i64 1, ptr %test4, align 8
  ; Variable: func_result2
  %func_result2 = alloca i64, align 8
  store i64 8, ptr %func_result2, align 8
  ; Variable: result2
  %result2 = alloca i64, align 8
  store i64 4, ptr %result2, align 8
  ; Variable: result5
  %result5 = alloca i64, align 8
  store i64 26, ptr %result5, align 8
  ; Variable: logic2
  %logic2 = alloca i64, align 8
  store i64 1, ptr %logic2, align 8
  ; Variable: test3
  %test3 = alloca i64, align 8
  store i64 1, ptr %test3, align 8
  ; Variable: expr3
  %expr3 = alloca i64, align 8
  store i64 33, ptr %expr3, align 8
  ; Variable: expr2
  %expr2 = alloca i64, align 8
  store i64 33, ptr %expr2, align 8
  ; Variable: chain3
  %chain3 = alloca i64, align 8
  store i64 0, ptr %chain3, align 8
  ; Variable: complex_arg2
  %complex_arg2 = alloca i64, align 8
  store i64 0, ptr %complex_arg2, align 8
  ; Variable: expr1
  %expr1 = alloca i64, align 8
  store i64 5, ptr %expr1, align 8
  ; Variable: test2
  %test2 = alloca i64, align 8
  store i64 1, ptr %test2, align 8
  ; Variable: logic1
  %logic1 = alloca i64, align 8
  store i64 1, ptr %logic1, align 8
  ; Variable: expr4
  %expr4 = alloca i64, align 8
  store i64 -1, ptr %expr4, align 8
  ; Variable: val1
  %val1 = alloca i64, align 8
  store i64 5, ptr %val1, align 8
  ; Variable: assign_test2
  %assign_test2 = alloca i64, align 8
  store i64 0, ptr %assign_test2, align 8
  ; Variable: result3
  %result3 = alloca i64, align 8
  store i64 11, ptr %result3, align 8
  ; Variable: chain2
  %chain2 = alloca i64, align 8
  store i64 0, ptr %chain2, align 8
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.0)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.1)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.2)
  %result1_load_0 = load i64, ptr %result1, align 8
  call void @cursed_runtime_spill_int(i64 %result1_load_0)
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.4)
  %result2_load_1 = load i64, ptr %result2, align 8
  call void @cursed_runtime_spill_int(i64 %result2_load_1)
  call void @cursed_runtime_spill_string(ptr @.str.5)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.6)
  %result3_load_2 = load i64, ptr %result3, align 8
  call void @cursed_runtime_spill_int(i64 %result3_load_2)
  call void @cursed_runtime_spill_string(ptr @.str.7)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.8)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.9)
  %result4_load_3 = load i64, ptr %result4, align 8
  call void @cursed_runtime_spill_int(i64 %result4_load_3)
  call void @cursed_runtime_spill_string(ptr @.str.10)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.11)
  %result5_load_4 = load i64, ptr %result5, align 8
  call void @cursed_runtime_spill_int(i64 %result5_load_4)
  call void @cursed_runtime_spill_string(ptr @.str.12)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.13)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.14)
  %result1_load_5 = load i64, ptr %result1, align 8
  call void @cursed_runtime_spill_int(i64 %result1_load_5)
  call void @cursed_runtime_spill_string(ptr @.str.15)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.16)
  %result2_load_6 = load i64, ptr %result2, align 8
  call void @cursed_runtime_spill_int(i64 %result2_load_6)
  call void @cursed_runtime_spill_string(ptr @.str.3)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.18)
  %result3_load_7 = load i64, ptr %result3, align 8
  call void @cursed_runtime_spill_int(i64 %result3_load_7)
  call void @cursed_runtime_spill_string(ptr @.str.10)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.20)
  %result4_load_8 = load i64, ptr %result4, align 8
  call void @cursed_runtime_spill_int(i64 %result4_load_8)
  call void @cursed_runtime_spill_string(ptr @.str.5)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.22)
  %result5_load_9 = load i64, ptr %result5, align 8
  call void @cursed_runtime_spill_int(i64 %result5_load_9)
  call void @cursed_runtime_spill_string(ptr @.str.23)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.24)
  %result6_load_10 = load i64, ptr %result6, align 8
  call void @cursed_runtime_spill_int(i64 %result6_load_10)
  call void @cursed_runtime_spill_string(ptr @.str.5)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.26)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.27)
  %expr1_load_11 = load i64, ptr %expr1, align 8
  call void @cursed_runtime_spill_int(i64 %expr1_load_11)
  call void @cursed_runtime_spill_string(ptr @.str.28)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.29)
  %expr2_load_12 = load i64, ptr %expr2, align 8
  call void @cursed_runtime_spill_int(i64 %expr2_load_12)
  call void @cursed_runtime_spill_string(ptr @.str.30)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.31)
  %expr3_load_13 = load i64, ptr %expr3, align 8
  call void @cursed_runtime_spill_int(i64 %expr3_load_13)
  call void @cursed_runtime_spill_string(ptr @.str.30)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.33)
  %expr4_load_14 = load i64, ptr %expr4, align 8
  call void @cursed_runtime_spill_int(i64 %expr4_load_14)
  call void @cursed_runtime_spill_string(ptr @.str.34)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.35)
  %expr5_load_15 = load i64, ptr %expr5, align 8
  call void @cursed_runtime_spill_int(i64 %expr5_load_15)
  call void @cursed_runtime_spill_string(ptr @.str.36)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.37)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.38)
  %test1_load_16 = load i64, ptr %test1, align 8
  call void @cursed_runtime_spill_int(i64 %test1_load_16)
  call void @cursed_runtime_spill_string(ptr @.str.39)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.40)
  %test2_load_17 = load i64, ptr %test2, align 8
  call void @cursed_runtime_spill_int(i64 %test2_load_17)
  call void @cursed_runtime_spill_string(ptr @.str.39)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.42)
  %test3_load_18 = load i64, ptr %test3, align 8
  call void @cursed_runtime_spill_int(i64 %test3_load_18)
  call void @cursed_runtime_spill_string(ptr @.str.39)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.44)
  %test4_load_19 = load i64, ptr %test4, align 8
  call void @cursed_runtime_spill_int(i64 %test4_load_19)
  call void @cursed_runtime_spill_string(ptr @.str.39)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.46)
  %test5_load_20 = load i64, ptr %test5, align 8
  call void @cursed_runtime_spill_int(i64 %test5_load_20)
  call void @cursed_runtime_spill_string(ptr @.str.39)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.48)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.49)
  %logic1_load_21 = load i64, ptr %logic1, align 8
  call void @cursed_runtime_spill_int(i64 %logic1_load_21)
  call void @cursed_runtime_spill_string(ptr @.str.50)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.51)
  %logic2_load_22 = load i64, ptr %logic2, align 8
  call void @cursed_runtime_spill_int(i64 %logic2_load_22)
  call void @cursed_runtime_spill_string(ptr @.str.50)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.53)
  %logic3_load_23 = load i64, ptr %logic3, align 8
  call void @cursed_runtime_spill_int(i64 %logic3_load_23)
  call void @cursed_runtime_spill_string(ptr @.str.50)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.55)
  %logic4_load_24 = load i64, ptr %logic4, align 8
  call void @cursed_runtime_spill_int(i64 %logic4_load_24)
  call void @cursed_runtime_spill_string(ptr @.str.50)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.57)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.58)
  %assign_test1_load_25 = load i64, ptr %assign_test1, align 8
  call void @cursed_runtime_spill_int(i64 %assign_test1_load_25)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.59)
  %assign_test2_load_26 = load i64, ptr %assign_test2, align 8
  call void @cursed_runtime_spill_int(i64 %assign_test2_load_26)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.60)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.61)
  %chain1_load_27 = load i64, ptr %chain1, align 8
  call void @cursed_runtime_spill_int(i64 %chain1_load_27)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.62)
  %chain2_load_28 = load i64, ptr %chain2, align 8
  call void @cursed_runtime_spill_int(i64 %chain2_load_28)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.63)
  %chain3_load_29 = load i64, ptr %chain3, align 8
  call void @cursed_runtime_spill_int(i64 %chain3_load_29)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.64)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.65)
  %func_result1_load_30 = load i64, ptr %func_result1, align 8
  call void @cursed_runtime_spill_int(i64 %func_result1_load_30)
  call void @cursed_runtime_spill_string(ptr @.str.66)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.67)
  %func_result2_load_31 = load i64, ptr %func_result2, align 8
  call void @cursed_runtime_spill_int(i64 %func_result2_load_31)
  call void @cursed_runtime_spill_string(ptr @.str.66)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.69)
  %func_result3_load_32 = load i64, ptr %func_result3, align 8
  call void @cursed_runtime_spill_int(i64 %func_result3_load_32)
  call void @cursed_runtime_spill_string(ptr @.str.70)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.71)
  %complex_arg1_load_33 = load i64, ptr %complex_arg1, align 8
  call void @cursed_runtime_spill_int(i64 %complex_arg1_load_33)
  call void @cursed_runtime_spill_string(ptr @.str.72)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.73)
  %complex_arg2_load_34 = load i64, ptr %complex_arg2, align 8
  call void @cursed_runtime_spill_int(i64 %complex_arg2_load_34)
  call void @cursed_runtime_spill_string(ptr @.str.66)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.75)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.76)
  ; Call: vibez.spill
  call void @cursed_runtime_spill_string(ptr @.str.77)
  ret i32 0
}

; String Constants
@.str.0 = private unnamed_addr constant [42 x i8] c"=== Basic Arithmetic Precedence Tests ===\00", align 1
@.str.1 = private unnamed_addr constant [43 x i8] c"Precedence: multiplication before addition\00", align 1
@.str.2 = private unnamed_addr constant [12 x i8] c"2 + 3 * 4 =\00", align 1
@.str.3 = private unnamed_addr constant [15 x i8] c"(should be 14)\00", align 1
@.str.4 = private unnamed_addr constant [13 x i8] c"10 - 2 * 3 =\00", align 1
@.str.5 = private unnamed_addr constant [14 x i8] c"(should be 4)\00", align 1
@.str.6 = private unnamed_addr constant [16 x i8] c"1 + 2 * 3 + 4 =\00", align 1
@.str.7 = private unnamed_addr constant [15 x i8] c"(should be 11)\00", align 1
@.str.8 = private unnamed_addr constant [37 x i8] c"Precedence: division before addition\00", align 1
@.str.9 = private unnamed_addr constant [13 x i8] c"20 + 8 / 2 =\00", align 1
@.str.10 = private unnamed_addr constant [15 x i8] c"(should be 24)\00", align 1
@.str.11 = private unnamed_addr constant [14 x i8] c"30 - 12 / 3 =\00", align 1
@.str.12 = private unnamed_addr constant [15 x i8] c"(should be 26)\00", align 1
@.str.13 = private unnamed_addr constant [35 x i8] c"=== Parentheses Override Tests ===\00", align 1
@.str.14 = private unnamed_addr constant [14 x i8] c"(2 + 3) * 4 =\00", align 1
@.str.15 = private unnamed_addr constant [15 x i8] c"(should be 20)\00", align 1
@.str.16 = private unnamed_addr constant [14 x i8] c"2 * (3 + 4) =\00", align 1
@.str.17 = private unnamed_addr constant [15 x i8] c"(should be 14)\00", align 1
@.str.18 = private unnamed_addr constant [15 x i8] c"(10 - 2) * 3 =\00", align 1
@.str.19 = private unnamed_addr constant [15 x i8] c"(should be 24)\00", align 1
@.str.20 = private unnamed_addr constant [15 x i8] c"20 / (2 + 3) =\00", align 1
@.str.21 = private unnamed_addr constant [14 x i8] c"(should be 4)\00", align 1
@.str.22 = private unnamed_addr constant [22 x i8] c"((2 + 3) * (4 - 1)) =\00", align 1
@.str.23 = private unnamed_addr constant [15 x i8] c"(should be 15)\00", align 1
@.str.24 = private unnamed_addr constant [27 x i8] c"(10 + (5 * 2)) / (3 + 2) =\00", align 1
@.str.25 = private unnamed_addr constant [14 x i8] c"(should be 4)\00", align 1
@.str.26 = private unnamed_addr constant [44 x i8] c"=== Complex Expression Precedence Tests ===\00", align 1
@.str.27 = private unnamed_addr constant [20 x i8] c"1 + 2 * 3 - 4 / 2 =\00", align 1
@.str.28 = private unnamed_addr constant [14 x i8] c"(should be 5)\00", align 1
@.str.29 = private unnamed_addr constant [25 x i8] c"10 * 2 + 5 * 3 - 8 / 4 =\00", align 1
@.str.30 = private unnamed_addr constant [15 x i8] c"(should be 33)\00", align 1
@.str.31 = private unnamed_addr constant [26 x i8] c"100 / 5 + 3 * 7 - 2 * 4 =\00", align 1
@.str.32 = private unnamed_addr constant [15 x i8] c"(should be 33)\00", align 1
@.str.33 = private unnamed_addr constant [30 x i8] c"(1 + 2) * (3 - 4) / (2 + 0) =\00", align 1
@.str.34 = private unnamed_addr constant [23 x i8] c"(should be -1 or -1.5)\00", align 1
@.str.35 = private unnamed_addr constant [32 x i8] c"2 * (3 + 4 * 5) - (8 - 3 * 2) =\00", align 1
@.str.36 = private unnamed_addr constant [15 x i8] c"(should be 44)\00", align 1
@.str.37 = private unnamed_addr constant [36 x i8] c"=== Comparison Precedence Tests ===\00", align 1
@.str.38 = private unnamed_addr constant [12 x i8] c"2 + 3 > 4 =\00", align 1
@.str.39 = private unnamed_addr constant [23 x i8] c"(should be true/based)\00", align 1
@.str.40 = private unnamed_addr constant [17 x i8] c"10 - 5 < 3 * 2 =\00", align 1
@.str.41 = private unnamed_addr constant [23 x i8] c"(should be true/based)\00", align 1
@.str.42 = private unnamed_addr constant [14 x i8] c"4 * 3 == 12 =\00", align 1
@.str.43 = private unnamed_addr constant [23 x i8] c"(should be true/based)\00", align 1
@.str.44 = private unnamed_addr constant [14 x i8] c"15 / 3 != 4 =\00", align 1
@.str.45 = private unnamed_addr constant [23 x i8] c"(should be true/based)\00", align 1
@.str.46 = private unnamed_addr constant [20 x i8] c"(2 + 3) > (4 - 1) =\00", align 1
@.str.47 = private unnamed_addr constant [23 x i8] c"(should be true/based)\00", align 1
@.str.48 = private unnamed_addr constant [42 x i8] c"=== Logical Operator Precedence Tests ===\00", align 1
@.str.49 = private unnamed_addr constant [20 x i8] c"5 < 10 && 10 < 15 =\00", align 1
@.str.50 = private unnamed_addr constant [17 x i8] c"(should be true)\00", align 1
@.str.51 = private unnamed_addr constant [20 x i8] c"5 > 10 || 10 < 15 =\00", align 1
@.str.52 = private unnamed_addr constant [17 x i8] c"(should be true)\00", align 1
@.str.53 = private unnamed_addr constant [26 x i8] c"(5+5)==10 && (15-10)==5 =\00", align 1
@.str.54 = private unnamed_addr constant [17 x i8] c"(should be true)\00", align 1
@.str.55 = private unnamed_addr constant [12 x i8] c"!(5 > 10) =\00", align 1
@.str.56 = private unnamed_addr constant [17 x i8] c"(should be true)\00", align 1
@.str.57 = private unnamed_addr constant [36 x i8] c"=== Assignment Precedence Tests ===\00", align 1
@.str.58 = private unnamed_addr constant [31 x i8] c"Assignment result: 2 + 3 * 4 =\00", align 1
@.str.59 = private unnamed_addr constant [33 x i8] c"Assignment result: (5 - 2) * 3 =\00", align 1
@.str.60 = private unnamed_addr constant [28 x i8] c"Chained assignment results:\00", align 1
@.str.61 = private unnamed_addr constant [9 x i8] c"chain1 =\00", align 1
@.str.62 = private unnamed_addr constant [9 x i8] c"chain2 =\00", align 1
@.str.63 = private unnamed_addr constant [9 x i8] c"chain3 =\00", align 1
@.str.64 = private unnamed_addr constant [39 x i8] c"=== Function Call Precedence Tests ===\00", align 1
@.str.65 = private unnamed_addr constant [27 x i8] c"mathz.abs_normie(-5) + 3 =\00", align 1
@.str.66 = private unnamed_addr constant [14 x i8] c"(should be 8)\00", align 1
@.str.67 = private unnamed_addr constant [27 x i8] c"2 * mathz.abs_normie(-4) =\00", align 1
@.str.68 = private unnamed_addr constant [14 x i8] c"(should be 8)\00", align 1
@.str.69 = private unnamed_addr constant [40 x i8] c"mathz.pow(2,3) + mathz.abs_normie(-1) =\00", align 1
@.str.70 = private unnamed_addr constant [14 x i8] c"(should be 9)\00", align 1
@.str.71 = private unnamed_addr constant [26 x i8] c"mathz.abs_normie(2 - 5) =\00", align 1
@.str.72 = private unnamed_addr constant [14 x i8] c"(should be 3)\00", align 1
@.str.73 = private unnamed_addr constant [22 x i8] c"mathz.pow(1+1, 2+1) =\00", align 1
@.str.74 = private unnamed_addr constant [14 x i8] c"(should be 8)\00", align 1
@.str.75 = private unnamed_addr constant [43 x i8] c"=== Parser Precedence Regression Tests ===\00", align 1
@.str.76 = private unnamed_addr constant [49 x i8] c"All parser precedence regression tests completed\00", align 1
@.str.77 = private unnamed_addr constant [62 x i8] c"Expression evaluation order verified - regression test passed\00", align 1
