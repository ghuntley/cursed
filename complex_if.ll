; Generated LLVM IR for CURSED program
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.str.0 = private unnamed_addr constant [68 x i8] c"Greater than 3") } otherwise { vibez.spill("Less than or equal to 3\00", align 1
@.int_fmt = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1
@.float_fmt = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.bool_true = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.bool_false = private unnamed_addr constant [7 x i8] c"cringe\00", align 1

define i32 @main() {
entry:
  ; Processing statement: sus x drip = 5
  ; Variable: x drip = 5
  ; Variable: x drip = 5
  %x = alloca i64, align 8
  store i64 5, i64* %x, align 8
  ; Stored variable 'x' with type 'i64'
  ; Processing statement: ready (x > 3) {
  ; Processing statement: vibez.spill("Greater than 3")
  ; Processing statement: } otherwise {
  ; Processing statement: vibez.spill("Less than or equal to 3")
  ; Processing statement: }
  ret i32 0
}
