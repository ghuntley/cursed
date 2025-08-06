; Generated LLVM IR for CURSED program
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.int_fmt = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1
@.float_fmt = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.bool_true = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.bool_false = private unnamed_addr constant [7 x i8] c"cringe\00", align 1

define i32 @main() {
entry:
  ; Variable: message tea = "Hello from CURSED string!"
  %message = alloca [26 x i8], align 1
  %message.ptr.0 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 0
  store i8 72, i8* %message.ptr.0, align 1
  %message.ptr.1 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 1
  store i8 101, i8* %message.ptr.1, align 1
  %message.ptr.2 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 2
  store i8 108, i8* %message.ptr.2, align 1
  %message.ptr.3 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 3
  store i8 108, i8* %message.ptr.3, align 1
  %message.ptr.4 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 4
  store i8 111, i8* %message.ptr.4, align 1
  %message.ptr.5 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 5
  store i8 32, i8* %message.ptr.5, align 1
  %message.ptr.6 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 6
  store i8 102, i8* %message.ptr.6, align 1
  %message.ptr.7 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 7
  store i8 114, i8* %message.ptr.7, align 1
  %message.ptr.8 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 8
  store i8 111, i8* %message.ptr.8, align 1
  %message.ptr.9 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 9
  store i8 109, i8* %message.ptr.9, align 1
  %message.ptr.10 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 10
  store i8 32, i8* %message.ptr.10, align 1
  %message.ptr.11 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 11
  store i8 67, i8* %message.ptr.11, align 1
  %message.ptr.12 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 12
  store i8 85, i8* %message.ptr.12, align 1
  %message.ptr.13 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 13
  store i8 82, i8* %message.ptr.13, align 1
  %message.ptr.14 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 14
  store i8 83, i8* %message.ptr.14, align 1
  %message.ptr.15 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 15
  store i8 69, i8* %message.ptr.15, align 1
  %message.ptr.16 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 16
  store i8 68, i8* %message.ptr.16, align 1
  %message.ptr.17 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 17
  store i8 32, i8* %message.ptr.17, align 1
  %message.ptr.18 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 18
  store i8 115, i8* %message.ptr.18, align 1
  %message.ptr.19 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 19
  store i8 116, i8* %message.ptr.19, align 1
  %message.ptr.20 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 20
  store i8 114, i8* %message.ptr.20, align 1
  %message.ptr.21 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 21
  store i8 105, i8* %message.ptr.21, align 1
  %message.ptr.22 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 22
  store i8 110, i8* %message.ptr.22, align 1
  %message.ptr.23 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 23
  store i8 103, i8* %message.ptr.23, align 1
  %message.ptr.24 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 24
  store i8 33, i8* %message.ptr.24, align 1
  %message.ptr.25 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 25
  store i8 0, i8* %message.ptr.25, align 1
  ; Variable: message
  %str_ptr.1 = getelementptr [26 x i8], [26 x i8]* %message, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.1)
  ret i32 0
}
