; ModuleID = 'feature_control_flow_comprehensive'
source_filename = "feature_control_flow_comprehensive"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0
@.str.0 = private constant [28 x i8] c"\22=== Control Flow Test ===\22\00"
@.str.1 = private constant [4 x i8] c"%s\0A\00"
@.str.2 = private constant [27 x i8] c"\22Value is greater than 10\22\00"
@.str.3 = private constant [26 x i8] c"\22Check is greater than 7\22\00"
@.str.4 = private constant [30 x i8] c"\22Check is not greater than 7\22\00"
@.str.5 = private constant [23 x i8] c"\22Starting loop count:\22\00"
@.str.6 = private constant [6 x i8] c"%lld\0A\00"
@.str.7 = private constant [22 x i8] c"\22Both conditions met\22\00"
@.str.8 = private constant [22 x i8] c"\22Boolean test passed\22\00"
@.str.9 = private constant [37 x i8] c"\22=== Control Flow Test Complete ===\22\00"

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

declare i32 @fflush(ptr)

declare i32 @yap(ptr)

define void @main_character() {
entry:
  %flag = alloca i1, align 1
  %b = alloca i64, align 8
  %a = alloca i64, align 8
  %counter = alloca i64, align 8
  %check = alloca i64, align 8
  %value = alloca i64, align 8
  %printf_string_call = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.0)
  store i64 15, ptr %value, align 4
  %load_var = load i64, ptr %value, align 4
  %gt_tmp = icmp sgt i64 %load_var, 10
  br i1 %gt_tmp, label %if_then, label %if_merge

if_then:                                          ; preds = %entry
  %printf_string_call1 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.2)
  br label %if_merge

if_merge:                                         ; preds = %if_then, %entry
  store i64 5, ptr %check, align 4
  %load_var4 = load i64, ptr %check, align 4
  %gt_tmp5 = icmp sgt i64 %load_var4, 7
  br i1 %gt_tmp5, label %if_then2, label %if_else

if_then2:                                         ; preds = %if_merge
  %printf_string_call6 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.3)
  br label %if_merge3

if_else:                                          ; preds = %if_merge
  %printf_string_call7 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.4)
  br label %if_merge3

if_merge3:                                        ; preds = %if_else, %if_then2
  store i64 0, ptr %counter, align 4
  %printf_string_call8 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.5)
  br label %while_cond

while_cond:                                       ; preds = %while_body, %if_merge3
  %load_var9 = load i64, ptr %counter, align 4
  %lt_tmp = icmp slt i64 %load_var9, 3
  %while_bool = icmp ne i1 %lt_tmp, false
  br i1 %while_bool, label %while_body, label %while_exit

while_body:                                       ; preds = %while_cond
  %load_var10 = load i64, ptr %counter, align 4
  %printf_call = call i32 (ptr, ...) @printf(ptr @.str.6, i64 %load_var10)
  %fflush_call = call i32 @fflush(ptr null)
  %load_var11 = load i64, ptr %counter, align 4
  %add_tmp = add i64 %load_var11, 1
  store i64 %add_tmp, ptr %counter, align 4
  br label %while_cond

while_exit:                                       ; preds = %while_cond
  store i64 8, ptr %a, align 4
  store i64 12, ptr %b, align 4
  %load_var14 = load i64, ptr %a, align 4
  %lt_tmp15 = icmp slt i64 %load_var14, 10
  br i1 %lt_tmp15, label %if_then12, label %if_merge13

if_then12:                                        ; preds = %while_exit
  %load_var18 = load i64, ptr %b, align 4
  %gt_tmp19 = icmp sgt i64 %load_var18, 10
  br i1 %gt_tmp19, label %if_then16, label %if_merge17

if_merge13:                                       ; preds = %if_merge17, %while_exit
  store i1 true, ptr %flag, align 1
  %load_var23 = load i1, ptr %flag, align 1
  br i1 %load_var23, label %if_then21, label %if_merge22

if_then16:                                        ; preds = %if_then12
  %printf_string_call20 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.7)
  br label %if_merge17

if_merge17:                                       ; preds = %if_then16, %if_then12
  br label %if_merge13

if_then21:                                        ; preds = %if_merge13
  %printf_string_call24 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.8)
  br label %if_merge22

if_merge22:                                       ; preds = %if_then21, %if_merge13
  %printf_string_call25 = call i32 (ptr, ...) @printf(ptr @.str.1, ptr @.str.9)
  ret void
}

define i32 @main() {
entry:
  call void @main()
  ret i32 0
}
