; ModuleID = '01_mathz_basic'
source_filename = "01_mathz_basic"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0
@.str.0 = private constant [26 x i8] c"=== Mathz Stdlib Test ===\00"
@.str.1 = private constant [4 x i8] c"%s\0A\00"
@.str.2 = private constant [23 x i8] c"Testing mathz.add_two:\00"
@.str.3 = private constant [17 x i8] c"Division by zero\00"
@.str.4 = private constant [4 x i8] c"%d\0A\00"
@.str.5 = private constant [19 x i8] c"Testing mathz.abs:\00"
@.str.6 = private constant [19 x i8] c"Testing mathz.max:\00"
@.str.7 = private constant [19 x i8] c"Testing mathz.min:\00"
@.str.8 = private constant [22 x i8] c"=== Test Complete ===\00"

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

declare i32 @fflush(ptr)

declare i32 @yap(ptr)

define void @main_character() {
entry:
  %result4 = alloca i32, align 4
  %result3 = alloca i32, align 4
  %result2 = alloca i32, align 4
  %result1 = alloca i32, align 4
  %printf_string_call = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.0)
  %printf_string_call1 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.2)
  %add_two_result = call i32 @mathz.add_two(i32 5, i32 3)
  store i32 %add_two_result, ptr %result1, align 4
  %load_var = load i32, ptr %result1, align 4
  %printf_call = call i32 (ptr, ...) @printf(ptr @.str.4, i32 %load_var)
  %fflush_call = call i32 @fflush(ptr null)
  %printf_string_call2 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.5)
  %abs_normie_result = call i32 @mathz.abs_normie(i32 -7)
  store i32 %abs_normie_result, ptr %result2, align 4
  %load_var3 = load i32, ptr %result2, align 4
  %printf_call4 = call i32 (ptr, ...) @printf(ptr @.str.4, i32 %load_var3)
  %fflush_call5 = call i32 @fflush(ptr null)
  %printf_string_call6 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.6)
  %max_result = call i32 @mathz.max(i32 10, i32 15)
  store i32 %max_result, ptr %result3, align 4
  %load_var7 = load i32, ptr %result3, align 4
  %printf_call8 = call i32 (ptr, ...) @printf(ptr @.str.4, i32 %load_var7)
  %fflush_call9 = call i32 @fflush(ptr null)
  %printf_string_call10 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.7)
  %min_result = call i32 @mathz.min(i32 8, i32 12)
  store i32 %min_result, ptr %result4, align 4
  %load_var11 = load i32, ptr %result4, align 4
  %printf_call12 = call i32 (ptr, ...) @printf(ptr @.str.4, i32 %load_var11)
  %fflush_call13 = call i32 @fflush(ptr null)
  %printf_string_call14 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.8)
  ret void
}

define i32 @mathz.abs_normie(i32 %x) {
entry:
  %x1 = alloca i32, align 4
  store i32 %x, ptr %x1, align 4
  %load_var = load i32, ptr %x1, align 4
  %lt_tmp = icmp slt i32 %load_var, 0
  br i1 %lt_tmp, label %if_then, label %if_merge

if_then:                                          ; preds = %entry
  %load_var2 = load i32, ptr %x1, align 4
  %neg = sub i32 0, %load_var2
  ret i32 %neg

if_merge:                                         ; preds = %entry
  %load_var3 = load i32, ptr %x1, align 4
  ret i32 %load_var3
}

define i32 @mathz.add_two(i32 %a, i32 %b) {
entry:
  %b2 = alloca i32, align 4
  %a1 = alloca i32, align 4
  store i32 %a, ptr %a1, align 4
  store i32 %b, ptr %b2, align 4
  %load_var = load i32, ptr %a1, align 4
  %load_var3 = load i32, ptr %b2, align 4
  %add_tmp = add i32 %load_var, %load_var3
  ret i32 %add_tmp
}

define i32 @mathz.max(i32 %a, i32 %b) {
entry:
  %b2 = alloca i32, align 4
  %a1 = alloca i32, align 4
  store i32 %a, ptr %a1, align 4
  store i32 %b, ptr %b2, align 4
  %load_var = load i32, ptr %a1, align 4
  %load_var3 = load i32, ptr %b2, align 4
  %gt_tmp = icmp sgt i32 %load_var, %load_var3
  br i1 %gt_tmp, label %if_then, label %if_merge

if_then:                                          ; preds = %entry
  %load_var4 = load i32, ptr %a1, align 4
  ret i32 %load_var4

if_merge:                                         ; preds = %entry
  %load_var5 = load i32, ptr %b2, align 4
  ret i32 %load_var5
}

define i32 @mathz.min(i32 %a, i32 %b) {
entry:
  %b2 = alloca i32, align 4
  %a1 = alloca i32, align 4
  store i32 %a, ptr %a1, align 4
  store i32 %b, ptr %b2, align 4
  %load_var = load i32, ptr %a1, align 4
  %load_var3 = load i32, ptr %b2, align 4
  %lt_tmp = icmp slt i32 %load_var, %load_var3
  br i1 %lt_tmp, label %if_then, label %if_merge

if_then:                                          ; preds = %entry
  %load_var4 = load i32, ptr %a1, align 4
  ret i32 %load_var4

if_merge:                                         ; preds = %entry
  %load_var5 = load i32, ptr %b2, align 4
  ret i32 %load_var5
}

define i32 @mathz.pow(i32 %base, i32 %exponent) {
entry:
  %base_power = alloca i32, align 4
  %exp = alloca i32, align 4
  %result = alloca i32, align 4
  %exponent2 = alloca i32, align 4
  %base1 = alloca i32, align 4
  store i32 %base, ptr %base1, align 4
  store i32 %exponent, ptr %exponent2, align 4
  %load_var = load i32, ptr %exponent2, align 4
  %eq_tmp = icmp eq i32 %load_var, 0
  br i1 %eq_tmp, label %if_then, label %if_merge

if_then:                                          ; preds = %entry
  ret i32 1

if_merge:                                         ; preds = %entry
  %load_var5 = load i32, ptr %exponent2, align 4
  %lt_tmp = icmp slt i32 %load_var5, 0
  br i1 %lt_tmp, label %if_then3, label %if_merge4

if_then3:                                         ; preds = %if_merge
  ret i32 0

if_merge4:                                        ; preds = %if_merge
  %load_var8 = load i32, ptr %base1, align 4
  %eq_tmp9 = icmp eq i32 %load_var8, 0
  br i1 %eq_tmp9, label %if_then6, label %if_merge7

if_then6:                                         ; preds = %if_merge4
  ret i32 0

if_merge7:                                        ; preds = %if_merge4
  %load_var12 = load i32, ptr %base1, align 4
  %eq_tmp13 = icmp eq i32 %load_var12, 1
  br i1 %eq_tmp13, label %if_then10, label %if_merge11

if_then10:                                        ; preds = %if_merge7
  ret i32 1

if_merge11:                                       ; preds = %if_merge7
  store i32 1, ptr %result, align 4
  %load_var14 = load i32, ptr %exponent2, align 4
  store i32 %load_var14, ptr %exp, align 4
  %load_var15 = load i32, ptr %base1, align 4
  store i32 %load_var15, ptr %base_power, align 4
  br label %for_cond

for_cond:                                         ; preds = %for_update, %if_merge11
  %load_var16 = load i32, ptr %exp, align 4
  %gt_tmp = icmp sgt i32 %load_var16, 0
  %for_bool = icmp ne i1 %gt_tmp, false
  br i1 %for_bool, label %for_body, label %for_exit

for_body:                                         ; preds = %for_cond
  %load_var19 = load i32, ptr %exp, align 4
  br i1 false, label %mod_by_zero, label %mod_ok

for_update:                                       ; preds = %div_ok
  br label %for_cond

for_exit:                                         ; preds = %for_cond
  %load_var27 = load i32, ptr %result, align 4
  ret i32 %load_var27

if_then17:                                        ; preds = %mod_ok
  %load_var21 = load i32, ptr %result, align 4
  %load_var22 = load i32, ptr %base_power, align 4
  %mul_tmp = mul i32 %load_var21, %load_var22
  store i32 %mul_tmp, ptr %result, align 4
  br label %if_merge18

if_merge18:                                       ; preds = %if_then17, %mod_ok
  %load_var23 = load i32, ptr %base_power, align 4
  %load_var24 = load i32, ptr %base_power, align 4
  %mul_tmp25 = mul i32 %load_var23, %load_var24
  store i32 %mul_tmp25, ptr %base_power, align 4
  %load_var26 = load i32, ptr %exp, align 4
  br i1 false, label %div_by_zero, label %div_ok

mod_by_zero:                                      ; preds = %for_body
  %0 = call i32 (ptr, ...) @printf(ptr @.str.3)
  call void @exit(i32 1)
  unreachable

mod_ok:                                           ; preds = %for_body
  %mod_tmp = srem i32 %load_var19, 2
  %eq_tmp20 = icmp eq i32 %mod_tmp, 1
  br i1 %eq_tmp20, label %if_then17, label %if_merge18

div_by_zero:                                      ; preds = %if_merge18
  %1 = call i32 (ptr, ...) @printf(ptr @.str.3)
  call void @exit(i32 1)
  unreachable

div_ok:                                           ; preds = %if_merge18
  %sdiv_tmp = sdiv i32 %load_var26, 2
  store i32 %sdiv_tmp, ptr %exp, align 4
  br label %for_update
}

declare void @exit(i32)

define i32 @mathz.sqrt(i32 %x) {
entry:
  %new_guess = alloca i32, align 4
  %i = alloca i32, align 4
  %iterations = alloca i32, align 4
  %guess = alloca i32, align 4
  %x1 = alloca i32, align 4
  store i32 %x, ptr %x1, align 4
  %load_var = load i32, ptr %x1, align 4
  %le_tmp = icmp sle i32 %load_var, 0
  br i1 %le_tmp, label %if_then, label %if_merge

if_then:                                          ; preds = %entry
  ret i32 0

if_merge:                                         ; preds = %entry
  %load_var4 = load i32, ptr %x1, align 4
  %eq_tmp = icmp eq i32 %load_var4, 1
  br i1 %eq_tmp, label %if_then2, label %if_merge3

if_then2:                                         ; preds = %if_merge
  ret i32 1

if_merge3:                                        ; preds = %if_merge
  %load_var7 = load i32, ptr %x1, align 4
  %eq_tmp8 = icmp eq i32 %load_var7, 4
  br i1 %eq_tmp8, label %if_then5, label %if_merge6

if_then5:                                         ; preds = %if_merge3
  ret i32 2

if_merge6:                                        ; preds = %if_merge3
  %load_var11 = load i32, ptr %x1, align 4
  %eq_tmp12 = icmp eq i32 %load_var11, 9
  br i1 %eq_tmp12, label %if_then9, label %if_merge10

if_then9:                                         ; preds = %if_merge6
  ret i32 3

if_merge10:                                       ; preds = %if_merge6
  %load_var15 = load i32, ptr %x1, align 4
  %eq_tmp16 = icmp eq i32 %load_var15, 16
  br i1 %eq_tmp16, label %if_then13, label %if_merge14

if_then13:                                        ; preds = %if_merge10
  ret i32 4

if_merge14:                                       ; preds = %if_merge10
  %load_var19 = load i32, ptr %x1, align 4
  %eq_tmp20 = icmp eq i32 %load_var19, 25
  br i1 %eq_tmp20, label %if_then17, label %if_merge18

if_then17:                                        ; preds = %if_merge14
  ret i32 5

if_merge18:                                       ; preds = %if_merge14
  %load_var23 = load i32, ptr %x1, align 4
  %eq_tmp24 = icmp eq i32 %load_var23, 36
  br i1 %eq_tmp24, label %if_then21, label %if_merge22

if_then21:                                        ; preds = %if_merge18
  ret i32 6

if_merge22:                                       ; preds = %if_merge18
  %load_var27 = load i32, ptr %x1, align 4
  %eq_tmp28 = icmp eq i32 %load_var27, 49
  br i1 %eq_tmp28, label %if_then25, label %if_merge26

if_then25:                                        ; preds = %if_merge22
  ret i32 7

if_merge26:                                       ; preds = %if_merge22
  %load_var31 = load i32, ptr %x1, align 4
  %eq_tmp32 = icmp eq i32 %load_var31, 64
  br i1 %eq_tmp32, label %if_then29, label %if_merge30

if_then29:                                        ; preds = %if_merge26
  ret i32 8

if_merge30:                                       ; preds = %if_merge26
  %load_var35 = load i32, ptr %x1, align 4
  %eq_tmp36 = icmp eq i32 %load_var35, 81
  br i1 %eq_tmp36, label %if_then33, label %if_merge34

if_then33:                                        ; preds = %if_merge30
  ret i32 9

if_merge34:                                       ; preds = %if_merge30
  %load_var39 = load i32, ptr %x1, align 4
  %eq_tmp40 = icmp eq i32 %load_var39, 100
  br i1 %eq_tmp40, label %if_then37, label %if_merge38

if_then37:                                        ; preds = %if_merge34
  ret i32 10

if_merge38:                                       ; preds = %if_merge34
  %load_var41 = load i32, ptr %x1, align 4
  br i1 false, label %div_by_zero, label %div_ok

div_by_zero:                                      ; preds = %if_merge38
  %0 = call i32 (ptr, ...) @printf(ptr @.str.3)
  call void @exit(i32 1)
  unreachable

div_ok:                                           ; preds = %if_merge38
  %sdiv_tmp = sdiv i32 %load_var41, 2
  store i32 %sdiv_tmp, ptr %guess, align 4
  store i32 10, ptr %iterations, align 4
  store i32 0, ptr %i, align 4
  br label %for_cond

for_cond:                                         ; preds = %for_update, %div_ok
  %load_var42 = load i32, ptr %i, align 4
  %load_var43 = load i32, ptr %iterations, align 4
  %lt_tmp = icmp slt i32 %load_var42, %load_var43
  %for_bool = icmp ne i1 %lt_tmp, false
  br i1 %for_bool, label %for_body, label %for_exit

for_body:                                         ; preds = %for_cond
  %load_var44 = load i32, ptr %guess, align 4
  %load_var45 = load i32, ptr %x1, align 4
  %load_var46 = load i32, ptr %guess, align 4
  %div_by_zero_check = icmp eq i32 %load_var46, 0
  br i1 %div_by_zero_check, label %div_by_zero47, label %div_ok48

for_update:                                       ; preds = %if_merge54
  br label %for_cond

for_exit:                                         ; preds = %for_cond
  %load_var62 = load i32, ptr %guess, align 4
  ret i32 %load_var62

div_by_zero47:                                    ; preds = %for_body
  %1 = call i32 (ptr, ...) @printf(ptr @.str.3)
  call void @exit(i32 1)
  unreachable

div_ok48:                                         ; preds = %for_body
  %sdiv_tmp49 = sdiv i32 %load_var45, %load_var46
  %add_tmp = add i32 %load_var44, %sdiv_tmp49
  br i1 false, label %div_by_zero50, label %div_ok51

div_by_zero50:                                    ; preds = %div_ok48
  %2 = call i32 (ptr, ...) @printf(ptr @.str.3)
  call void @exit(i32 1)
  unreachable

div_ok51:                                         ; preds = %div_ok48
  %sdiv_tmp52 = sdiv i32 %add_tmp, 2
  store i32 %sdiv_tmp52, ptr %new_guess, align 4
  %load_var55 = load i32, ptr %new_guess, align 4
  %load_var56 = load i32, ptr %guess, align 4
  %sub_tmp = sub i32 %load_var55, %load_var56
  %forward_call_tmp = call i32 @abs_normie(i32 %sub_tmp)
  %le_tmp57 = icmp sle i32 %forward_call_tmp, 1
  br i1 %le_tmp57, label %if_then53, label %if_merge54

if_then53:                                        ; preds = %div_ok51
  %load_var58 = load i32, ptr %new_guess, align 4
  ret i32 %load_var58

if_merge54:                                       ; preds = %div_ok51
  %load_var59 = load i32, ptr %new_guess, align 4
  store i32 %load_var59, ptr %guess, align 4
  %load_var60 = load i32, ptr %i, align 4
  %add_tmp61 = add i32 %load_var60, 1
  store i32 %add_tmp61, ptr %i, align 4
  br label %for_update
}

declare i32 @abs_normie(i32)

define i32 @mathz.mod(i32 %a, i32 %b) {
entry:
  %b2 = alloca i32, align 4
  %a1 = alloca i32, align 4
  store i32 %a, ptr %a1, align 4
  store i32 %b, ptr %b2, align 4
  %load_var = load i32, ptr %b2, align 4
  %eq_tmp = icmp eq i32 %load_var, 0
  br i1 %eq_tmp, label %if_then, label %if_merge

if_then:                                          ; preds = %entry
  ret i32 0

if_merge:                                         ; preds = %entry
  %load_var3 = load i32, ptr %a1, align 4
  %load_var4 = load i32, ptr %b2, align 4
  %mod_by_zero_check = icmp eq i32 %load_var4, 0
  br i1 %mod_by_zero_check, label %mod_by_zero, label %mod_ok

mod_by_zero:                                      ; preds = %if_merge
  %0 = call i32 (ptr, ...) @printf(ptr @.str.3)
  call void @exit(i32 1)
  unreachable

mod_ok:                                           ; preds = %if_merge
  %mod_tmp = srem i32 %load_var3, %load_var4
  ret i32 %mod_tmp
}

define i32 @mathz.add(i32 %a, i32 %b) {
entry:
  %b2 = alloca i32, align 4
  %a1 = alloca i32, align 4
  store i32 %a, ptr %a1, align 4
  store i32 %b, ptr %b2, align 4
  %load_var = load i32, ptr %a1, align 4
  %load_var3 = load i32, ptr %b2, align 4
  %add_tmp = add i32 %load_var, %load_var3
  ret i32 %add_tmp
}

define i32 @mathz.subtract(i32 %a, i32 %b) {
entry:
  %b2 = alloca i32, align 4
  %a1 = alloca i32, align 4
  store i32 %a, ptr %a1, align 4
  store i32 %b, ptr %b2, align 4
  %load_var = load i32, ptr %a1, align 4
  %load_var3 = load i32, ptr %b2, align 4
  %sub_tmp = sub i32 %load_var, %load_var3
  ret i32 %sub_tmp
}

define i32 @mathz.multiply(i32 %a, i32 %b) {
entry:
  %b2 = alloca i32, align 4
  %a1 = alloca i32, align 4
  store i32 %a, ptr %a1, align 4
  store i32 %b, ptr %b2, align 4
  %load_var = load i32, ptr %a1, align 4
  %load_var3 = load i32, ptr %b2, align 4
  %mul_tmp = mul i32 %load_var, %load_var3
  ret i32 %mul_tmp
}

define i32 @mathz.divide(i32 %a, i32 %b) {
entry:
  %b2 = alloca i32, align 4
  %a1 = alloca i32, align 4
  store i32 %a, ptr %a1, align 4
  store i32 %b, ptr %b2, align 4
  %load_var = load i32, ptr %b2, align 4
  %eq_tmp = icmp eq i32 %load_var, 0
  br i1 %eq_tmp, label %if_then, label %if_merge

if_then:                                          ; preds = %entry
  ret i32 0

if_merge:                                         ; preds = %entry
  %load_var3 = load i32, ptr %a1, align 4
  %load_var4 = load i32, ptr %b2, align 4
  %div_by_zero_check = icmp eq i32 %load_var4, 0
  br i1 %div_by_zero_check, label %div_by_zero, label %div_ok

div_by_zero:                                      ; preds = %if_merge
  %0 = call i32 (ptr, ...) @printf(ptr @.str.3)
  call void @exit(i32 1)
  unreachable

div_ok:                                           ; preds = %if_merge
  %sdiv_tmp = sdiv i32 %load_var3, %load_var4
  ret i32 %sdiv_tmp
}

define i32 @main() {
entry:
  call void @main_character()
  ret i32 0
}
