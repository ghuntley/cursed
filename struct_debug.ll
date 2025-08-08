; Generated LLVM IR for CURSED program
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.str.0 = private unnamed_addr constant [8 x i8] c"Values:\00", align 1
@.int_fmt = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1
@.float_fmt = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.bool_true = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.bool_false = private unnamed_addr constant [7 x i8] c"cringe\00", align 1

define i32 @main() {
entry:
  ; Processing statement: squad Point { spill x drip
  ; Processing statement: spill y drip }
  ; Processing statement: sus p Point = Point{x: 10, y: 20}
  ; Variable: p Point = Point{x: 10, y: 20}
  ; Variable: p Point = Point{x: 10, y: 20}
  ; Processing statement: vibez.spill("Values:", p.x, p.y)
  ; String literal: Values:
  %str_ptr.1 = getelementptr [8 x i8], [8 x i8]* @.str.0, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.1)
  ; Struct field access: p.x
  %field_ptr.2 = getelementptr %struct.Point, %struct.Point* %p, i32 0, i32 0
  %field_val.2 = load i64, i64* %field_ptr.2, align 8
  %fmt_ptr.2 = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt_ptr.2, i64 %field_val.2)
  ; Struct field access: p.y
  %field_ptr.3 = getelementptr %struct.Point, %struct.Point* %p, i32 0, i32 1
  %field_val.3 = load i64, i64* %field_ptr.3, align 8
  %fmt_ptr.3 = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt_ptr.3, i64 %field_val.3)
  ret i32 0
}
