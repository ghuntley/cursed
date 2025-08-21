; ModuleID = 'cursed_program'
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

; String constants

; Main function
define i32 @main() {
entry:
  %x = alloca i64, align 8
  store i64 42, i64* %x, align 8
  %message = alloca i64, align 8
  store i64 0, i64* %message, align 8
  ret i32 0
}

; External function declarations
declare i32 @printf(i8*, ...)
