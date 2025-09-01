; ModuleID = 'control_flow_if_else'
source_filename = "control_flow_if_else"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0
@.str.0 = private constant [15 x i8] c"\22Winner is x:\22\00"
@.str.1 = private constant [4 x i8] c"%s \00"
@.str.2 = private constant [6 x i8] c"%lld\0A\00"
@.str.3 = private constant [15 x i8] c"\22Winner is y:\22\00"
@.str.4 = private constant [11 x i8] c"\22Grade: A\22\00"
@.str.5 = private constant [4 x i8] c"%s\0A\00"
@.str.6 = private constant [14 x i8] c"\22High honor!\22\00"
@.str.7 = private constant [11 x i8] c"\22Grade: B\22\00"
@.str.8 = private constant [20 x i8] c"\22Grade: C or below\22\00"
@.str.9 = private constant [30 x i8] c"\22Number is positive and even\22\00"
@.str.10 = private constant [29 x i8] c"\22Number is positive but odd\22\00"
@.str.11 = private constant [21 x i8] c"\22Number is negative\22\00"

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

declare i32 @fflush(ptr)

declare i32 @yap(ptr)

define i32 @main_character() {
entry:
  %result44 = alloca i64, align 8
  %result42 = alloca i64, align 8
  %result = alloca i64, align 8
  %isEven = alloca i1, align 1
  %isPositive = alloca i1, align 1
  %grade27 = alloca i64, align 8
  %grade25 = alloca i64, align 8
  %honor = alloca i64, align 8
  %grade = alloca i64, align 8
  %score = alloca i64, align 8
  %winner5 = alloca i64, align 8
  %winner = alloca i64, align 8
  %y = alloca i64, align 8
  %x = alloca i64, align 8
  store i64 15, ptr %x, align 4
  store i64 10, ptr %y, align 4
  %load_var = load i64, ptr %x, align 4
  %load_var1 = load i64, ptr %y, align 4
  %gt_tmp = icmp sgt i64 %load_var, %load_var1
  br i1 %gt_tmp, label %if_then, label %if_else

if_then:                                          ; preds = %entry
  %load_var2 = load i64, ptr %x, align 4
  store i64 %load_var2, ptr %winner, align 4
  %printf_string_call = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.0)
  %load_var3 = load i64, ptr %winner, align 4
  %printf_call = call i32 (ptr, ...) @printf(ptr @.str.2, i64 %load_var3)
  %fflush_call = call i32 @fflush(ptr null)
  br label %if_merge

if_else:                                          ; preds = %entry
  %load_var4 = load i64, ptr %y, align 4
  store i64 %load_var4, ptr %winner5, align 4
  %printf_string_call6 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.3)
  %load_var7 = load i64, ptr %winner5, align 4
  %printf_call8 = call i32 (ptr, ...) @printf(ptr @.str.2, i64 %load_var7)
  %fflush_call9 = call i32 @fflush(ptr null)
  br label %if_merge

if_merge:                                         ; preds = %if_else, %if_then
  store i64 85, ptr %score, align 4
  %load_var13 = load i64, ptr %score, align 4
  %ge_tmp = icmp sge i64 %load_var13, 90
  br i1 %ge_tmp, label %if_then10, label %if_else11

if_then10:                                        ; preds = %if_merge
  store i64 1, ptr %grade, align 4
  %printf_string_call14 = call i32 (ptr, ...) @printf(ptr @.str.5, ptr @.str.4)
  %load_var17 = load i64, ptr %score, align 4
  %ge_tmp18 = icmp sge i64 %load_var17, 95
  br i1 %ge_tmp18, label %if_then15, label %if_merge16

if_else11:                                        ; preds = %if_merge
  %load_var23 = load i64, ptr %score, align 4
  %ge_tmp24 = icmp sge i64 %load_var23, 80
  br i1 %ge_tmp24, label %if_then20, label %if_else21

if_merge12:                                       ; preds = %if_merge22, %if_merge16
  %load_var29 = load i64, ptr %x, align 4
  %gt_tmp30 = icmp sgt i64 %load_var29, 0
  store i1 %gt_tmp30, ptr %isPositive, align 1
  %load_var31 = load i64, ptr %x, align 4
  %mod_tmp = srem i64 %load_var31, 2
  %eq_tmp = icmp eq i64 %mod_tmp, 0
  store i1 %eq_tmp, ptr %isEven, align 1
  %load_var35 = load i1, ptr %isPositive, align 1
  %load_var36 = load i1, ptr %isEven, align 1
  %and_tmp = and i1 %load_var35, %load_var36
  br i1 %and_tmp, label %if_then32, label %if_else33

if_then15:                                        ; preds = %if_then10
  store i64 1, ptr %honor, align 4
  %printf_string_call19 = call i32 (ptr, ...) @printf(ptr @.str.5, ptr @.str.6)
  br label %if_merge16

if_merge16:                                       ; preds = %if_then15, %if_then10
  br label %if_merge12

if_then20:                                        ; preds = %if_else11
  store i64 2, ptr %grade25, align 4
  %printf_string_call26 = call i32 (ptr, ...) @printf(ptr @.str.5, ptr @.str.7)
  br label %if_merge22

if_else21:                                        ; preds = %if_else11
  store i64 3, ptr %grade27, align 4
  %printf_string_call28 = call i32 (ptr, ...) @printf(ptr @.str.5, ptr @.str.8)
  br label %if_merge22

if_merge22:                                       ; preds = %if_else21, %if_then20
  br label %if_merge12

if_then32:                                        ; preds = %if_merge12
  store i64 1, ptr %result, align 4
  %printf_string_call37 = call i32 (ptr, ...) @printf(ptr @.str.5, ptr @.str.9)
  br label %if_merge34

if_else33:                                        ; preds = %if_merge12
  %load_var41 = load i1, ptr %isPositive, align 1
  br i1 %load_var41, label %if_then38, label %if_else39

if_merge34:                                       ; preds = %if_merge40, %if_then32
  ret i32 0

if_then38:                                        ; preds = %if_else33
  store i64 2, ptr %result42, align 4
  %printf_string_call43 = call i32 (ptr, ...) @printf(ptr @.str.5, ptr @.str.10)
  br label %if_merge40

if_else39:                                        ; preds = %if_else33
  store i64 0, ptr %result44, align 4
  %printf_string_call45 = call i32 (ptr, ...) @printf(ptr @.str.5, ptr @.str.11)
  br label %if_merge40

if_merge40:                                       ; preds = %if_else39, %if_then38
  br label %if_merge34
}

define i32 @main() {
entry:
  call void @main_character()
  ret i32 0
}
