; ModuleID = '01_if_statements'
source_filename = "01_if_statements"
target triple = "x86_64-pc-linux-gnu"

@vibez = private global i64 0
@.str.0 = private constant [28 x i8] c"\22=== If Statement Test ===\22\00"
@.str.1 = private constant [16 x i8] c"\22x = 10, y = 5\22\00"
@.str.2 = private constant [22 x i8] c"\22x is greater than y\22\00"
@.str.3 = private constant [22 x i8] c"\22y is greater than x\22\00"
@.str.4 = private constant [23 x i8] c"\22x is not less than y\22\00"
@.str.5 = private constant [20 x i8] c"\22Score evaluation:\22\00"
@.str.6 = private constant [10 x i8] c"\22Grade A\22\00"
@.str.7 = private constant [10 x i8] c"\22Grade B\22\00"
@.str.8 = private constant [10 x i8] c"\22Grade C\22\00"
@.str.9 = private constant [10 x i8] c"\22Grade F\22\00"
@.str.10 = private constant [24 x i8] c"\22=== Test Complete ===\22\00"

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

declare i32 @fflush(ptr)

declare i32 @yap(ptr)

define void @main_character() {
entry:
  %score = alloca i64, align 8
  %y = alloca i64, align 8
  %x = alloca i64, align 8
  %puts_call = call i32 @puts(ptr @.str.0)
  store i64 10, ptr %x, align 4
  store i64 5, ptr %y, align 4
  %puts_call1 = call i32 @puts(ptr @.str.1)
  %load_var = load i64, ptr %x, align 4
  %load_var2 = load i64, ptr %y, align 4
  %gt_tmp = icmp sgt i64 %load_var, %load_var2
  br i1 %gt_tmp, label %if_then, label %if_merge

if_then:                                          ; preds = %entry
  %puts_call3 = call i32 @puts(ptr @.str.2)
  br label %if_merge

if_merge:                                         ; preds = %if_then, %entry
  %load_var6 = load i64, ptr %y, align 4
  %load_var7 = load i64, ptr %x, align 4
  %gt_tmp8 = icmp sgt i64 %load_var6, %load_var7
  br i1 %gt_tmp8, label %if_then4, label %if_else

if_then4:                                         ; preds = %if_merge
  %puts_call9 = call i32 @puts(ptr @.str.3)
  br label %if_merge5

if_else:                                          ; preds = %if_merge
  %puts_call10 = call i32 @puts(ptr @.str.4)
  br label %if_merge5

if_merge5:                                        ; preds = %if_else, %if_then4
  store i64 85, ptr %score, align 4
  %puts_call11 = call i32 @puts(ptr @.str.5)
  %load_var15 = load i64, ptr %score, align 4
  %eq_tmp = icmp eq i64 %load_var15, 90
  br i1 %eq_tmp, label %if_then12, label %if_else13

if_then12:                                        ; preds = %if_merge5
  %puts_call16 = call i32 @puts(ptr @.str.6)
  br label %if_merge14

if_else13:                                        ; preds = %if_merge5
  %load_var20 = load i64, ptr %score, align 4
  %eq_tmp21 = icmp eq i64 %load_var20, 80
  br i1 %eq_tmp21, label %if_then17, label %if_else18

if_merge14:                                       ; preds = %if_merge19, %if_then12
  %puts_call30 = call i32 @puts(ptr @.str.10)
  ret void

if_then17:                                        ; preds = %if_else13
  %puts_call22 = call i32 @puts(ptr @.str.7)
  br label %if_merge19

if_else18:                                        ; preds = %if_else13
  %load_var26 = load i64, ptr %score, align 4
  %eq_tmp27 = icmp eq i64 %load_var26, 70
  br i1 %eq_tmp27, label %if_then23, label %if_else24

if_merge19:                                       ; preds = %if_merge25, %if_then17
  br label %if_merge14

if_then23:                                        ; preds = %if_else18
  %puts_call28 = call i32 @puts(ptr @.str.8)
  br label %if_merge25

if_else24:                                        ; preds = %if_else18
  %puts_call29 = call i32 @puts(ptr @.str.9)
  br label %if_merge25

if_merge25:                                       ; preds = %if_else24, %if_then23
  br label %if_merge19
}

define i32 @main() {
entry:
  call void @main_character()
  ret i32 0
}
