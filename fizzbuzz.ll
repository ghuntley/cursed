target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@.str = private unnamed_addr constant [10 x i8] c"FizzBuzz\0A\00", align 1
@.str.1 = private unnamed_addr constant [6 x i8] c"Fizz\0A\00", align 1  
@.str.2 = private unnamed_addr constant [6 x i8] c"Buzz\0A\00", align 1
@.str.3 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

declare i32 @printf(i8* noundef, ...)

define dso_local i32 @main() {
entry:
  br label %for.cond

for.cond:
  %i.0 = phi i32 [ 1, %entry ], [ %inc, %for.inc ]
  %cmp = icmp sle i32 %i.0, 100
  br i1 %cmp, label %for.body, label %for.end

for.body:
  %rem = srem i32 %i.0, 15
  %cmp1 = icmp eq i32 %rem, 0
  br i1 %cmp1, label %if.then, label %if.else

if.then:
  %call = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([10 x i8], [10 x i8]* @.str, i64 0, i64 0))
  br label %for.inc

if.else:
  %rem2 = srem i32 %i.0, 3
  %cmp3 = icmp eq i32 %rem2, 0
  br i1 %cmp3, label %if.then4, label %if.else6

if.then4:
  %call5 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([6 x i8], [6 x i8]* @.str.1, i64 0, i64 0))
  br label %for.inc

if.else6:
  %rem7 = srem i32 %i.0, 5
  %cmp8 = icmp eq i32 %rem7, 0
  br i1 %cmp8, label %if.then9, label %if.else11

if.then9:
  %call10 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([6 x i8], [6 x i8]* @.str.2, i64 0, i64 0))
  br label %for.inc

if.else11:
  %call12 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([4 x i8], [4 x i8]* @.str.3, i64 0, i64 0), i32 noundef %i.0)
  br label %for.inc

for.inc:
  %inc = add nsw i32 %i.0, 1
  br label %for.cond

for.end:
  ret i32 0
}
