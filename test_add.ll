; Generated LLVM IR for CURSED program
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.str.0 = private unnamed_addr constant [5 x i8] c"Sum:\00", align 1
@.int_fmt = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1
@.float_fmt = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.bool_true = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.bool_false = private unnamed_addr constant [7 x i8] c"cringe\00", align 1

  ; Parsing function definition: slay add(a drip, b drip) drip { damn a + b }
  ; Function: add, Return: drip, Params: 'a drip, b drip', Body: 'damn a + b'
define i64 @add(i64 %a, i64 %b) {
entry:
  ; Generating body: damn a + b
  ; Return expression: a + b
  ; Binary operation: a + b
  %result = add i64 %a, %b
  ret i64 %result
}

  ; Stored function 'add' with 0 parameters
define i32 @main() {
entry:
  ; Processing statement: sus x drip = add(10, 20)
  ; Variable: x drip = add(10, 20)
  ; Calling function: add with args: '10, 20'
  %x = alloca i64, align 8
  %call_result.0 = call i64 @add(i64 10, i64 20)
  store i64 %call_result.0, i64* %x, align 8
  ; Processing statement: vibez.spill("Sum:", x)
  ; String literal: Sum:
  %str_ptr.1 = getelementptr [5 x i8], [5 x i8]* @.str.0, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.1)
  ; Looking for variable: 'x'
  ; Found variable: x (type: i64)
  %loaded.2 = load i64, i64* %x, align 8
  %fmt_ptr.2 = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt_ptr.2, i64 %loaded.2)
  ret i32 0
}
