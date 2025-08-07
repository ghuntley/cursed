; Generated LLVM IR for CURSED program
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.str.0 = private unnamed_addr constant [22 x i8] c"Acquiring resource...\00", align 1
@.str.1 = private unnamed_addr constant [27 x i8] c"Resource cleanup completed\00", align 1
@.str.2 = private unnamed_addr constant [29 x i8] c"Resource processing complete\00", align 1
@.str.3 = private unnamed_addr constant [42 x i8] c"=== CURSED Comprehensive Feature Test ===\00", align 1
@.str.4 = private unnamed_addr constant [21 x i8] c"This shouldn't print\00", align 1
@.str.5 = private unnamed_addr constant [19 x i8] c"No result received\00", align 1
@.str.6 = private unnamed_addr constant [44 x i8] c"=== All comprehensive features working! ===\00", align 1
@.int_fmt = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1
@.float_fmt = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.bool_true = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.bool_false = private unnamed_addr constant [7 x i8] c"cringe\00", align 1

define i32 @main() {
entry:
  %result = alloca i64, align 8
  %mul_result.1 = mul i64 0, 0
  store i64 %mul_result.1, i64* %result, align 8
  %result = alloca [67 x i8], align 1
  %result.ptr.0 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 0
  store i8 80, i8* %result.ptr.0, align 1
  %result.ptr.1 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 1
  store i8 108, i8* %result.ptr.1, align 1
  %result.ptr.2 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 2
  store i8 97, i8* %result.ptr.2, align 1
  %result.ptr.3 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 3
  store i8 121, i8* %result.ptr.3, align 1
  %result.ptr.4 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 4
  store i8 101, i8* %result.ptr.4, align 1
  %result.ptr.5 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 5
  store i8 114, i8* %result.ptr.5, align 1
  %result.ptr.6 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 6
  store i8 58, i8* %result.ptr.6, align 1
  %result.ptr.7 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 7
  store i8 32, i8* %result.ptr.7, align 1
  %result.ptr.8 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 8
  store i8 34, i8* %result.ptr.8, align 1
  %result.ptr.9 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 9
  store i8 32, i8* %result.ptr.9, align 1
  %result.ptr.10 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 10
  store i8 43, i8* %result.ptr.10, align 1
  %result.ptr.11 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 11
  store i8 32, i8* %result.ptr.11, align 1
  %result.ptr.12 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 12
  store i8 110, i8* %result.ptr.12, align 1
  %result.ptr.13 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 13
  store i8 97, i8* %result.ptr.13, align 1
  %result.ptr.14 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 14
  store i8 109, i8* %result.ptr.14, align 1
  %result.ptr.15 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 15
  store i8 101, i8* %result.ptr.15, align 1
  %result.ptr.16 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 16
  store i8 32, i8* %result.ptr.16, align 1
  %result.ptr.17 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 17
  store i8 43, i8* %result.ptr.17, align 1
  %result.ptr.18 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 18
  store i8 32, i8* %result.ptr.18, align 1
  %result.ptr.19 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 19
  store i8 34, i8* %result.ptr.19, align 1
  %result.ptr.20 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 20
  store i8 32, i8* %result.ptr.20, align 1
  %result.ptr.21 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 21
  store i8 115, i8* %result.ptr.21, align 1
  %result.ptr.22 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 22
  store i8 99, i8* %result.ptr.22, align 1
  %result.ptr.23 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 23
  store i8 111, i8* %result.ptr.23, align 1
  %result.ptr.24 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 24
  store i8 114, i8* %result.ptr.24, align 1
  %result.ptr.25 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 25
  store i8 101, i8* %result.ptr.25, align 1
  %result.ptr.26 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 26
  store i8 100, i8* %result.ptr.26, align 1
  %result.ptr.27 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 27
  store i8 32, i8* %result.ptr.27, align 1
  %result.ptr.28 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 28
  store i8 34, i8* %result.ptr.28, align 1
  %result.ptr.29 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 29
  store i8 32, i8* %result.ptr.29, align 1
  %result.ptr.30 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 30
  store i8 43, i8* %result.ptr.30, align 1
  %result.ptr.31 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 31
  store i8 32, i8* %result.ptr.31, align 1
  %result.ptr.32 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 32
  store i8 115, i8* %result.ptr.32, align 1
  %result.ptr.33 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 33
  store i8 116, i8* %result.ptr.33, align 1
  %result.ptr.34 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 34
  store i8 114, i8* %result.ptr.34, align 1
  %result.ptr.35 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 35
  store i8 105, i8* %result.ptr.35, align 1
  %result.ptr.36 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 36
  store i8 110, i8* %result.ptr.36, align 1
  %result.ptr.37 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 37
  store i8 103, i8* %result.ptr.37, align 1
  %result.ptr.38 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 38
  store i8 122, i8* %result.ptr.38, align 1
  %result.ptr.39 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 39
  store i8 46, i8* %result.ptr.39, align 1
  %result.ptr.40 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 40
  store i8 102, i8* %result.ptr.40, align 1
  %result.ptr.41 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 41
  store i8 114, i8* %result.ptr.41, align 1
  %result.ptr.42 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 42
  store i8 111, i8* %result.ptr.42, align 1
  %result.ptr.43 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 43
  store i8 109, i8* %result.ptr.43, align 1
  %result.ptr.44 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 44
  store i8 95, i8* %result.ptr.44, align 1
  %result.ptr.45 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 45
  store i8 105, i8* %result.ptr.45, align 1
  %result.ptr.46 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 46
  store i8 110, i8* %result.ptr.46, align 1
  %result.ptr.47 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 47
  store i8 116, i8* %result.ptr.47, align 1
  %result.ptr.48 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 48
  store i8 40, i8* %result.ptr.48, align 1
  %result.ptr.49 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 49
  store i8 115, i8* %result.ptr.49, align 1
  %result.ptr.50 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 50
  store i8 99, i8* %result.ptr.50, align 1
  %result.ptr.51 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 51
  store i8 111, i8* %result.ptr.51, align 1
  %result.ptr.52 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 52
  store i8 114, i8* %result.ptr.52, align 1
  %result.ptr.53 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 53
  store i8 101, i8* %result.ptr.53, align 1
  %result.ptr.54 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 54
  store i8 41, i8* %result.ptr.54, align 1
  %result.ptr.55 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 55
  store i8 32, i8* %result.ptr.55, align 1
  %result.ptr.56 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 56
  store i8 43, i8* %result.ptr.56, align 1
  %result.ptr.57 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 57
  store i8 32, i8* %result.ptr.57, align 1
  %result.ptr.58 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 58
  store i8 34, i8* %result.ptr.58, align 1
  %result.ptr.59 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 59
  store i8 32, i8* %result.ptr.59, align 1
  %result.ptr.60 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 60
  store i8 112, i8* %result.ptr.60, align 1
  %result.ptr.61 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 61
  store i8 111, i8* %result.ptr.61, align 1
  %result.ptr.62 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 62
  store i8 105, i8* %result.ptr.62, align 1
  %result.ptr.63 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 63
  store i8 110, i8* %result.ptr.63, align 1
  %result.ptr.64 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 64
  store i8 116, i8* %result.ptr.64, align 1
  %result.ptr.65 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 65
  store i8 115, i8* %result.ptr.65, align 1
  %result.ptr.66 = getelementptr [67 x i8], [67 x i8]* %result, i32 0, i32 66
  store i8 0, i8* %result.ptr.66, align 1
  %str_ptr.4 = getelementptr [22 x i8], [22 x i8]* @.str.0, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.4)
  %str_ptr.5 = getelementptr [27 x i8], [27 x i8]* @.str.1, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.5)
  %str_ptr.8 = getelementptr [29 x i8], [29 x i8]* @.str.2, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.8)
  %i = alloca i64, align 8
  store i64 0, i64* %i, align 8
  %final_count = alloca i64, align 8
  store i64 0, i64* %final_count, align 8
  %str_ptr.11 = getelementptr [42 x i8], [42 x i8]* @.str.3, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.11)
  %name = alloca [6 x i8], align 1
  %name.ptr.0 = getelementptr [6 x i8], [6 x i8]* %name, i32 0, i32 0
  store i8 65, i8* %name.ptr.0, align 1
  %name.ptr.1 = getelementptr [6 x i8], [6 x i8]* %name, i32 0, i32 1
  store i8 108, i8* %name.ptr.1, align 1
  %name.ptr.2 = getelementptr [6 x i8], [6 x i8]* %name, i32 0, i32 2
  store i8 105, i8* %name.ptr.2, align 1
  %name.ptr.3 = getelementptr [6 x i8], [6 x i8]* %name, i32 0, i32 3
  store i8 99, i8* %name.ptr.3, align 1
  %name.ptr.4 = getelementptr [6 x i8], [6 x i8]* %name, i32 0, i32 4
  store i8 101, i8* %name.ptr.4, align 1
  %name.ptr.5 = getelementptr [6 x i8], [6 x i8]* %name, i32 0, i32 5
  store i8 0, i8* %name.ptr.5, align 1
  %score = alloca i64, align 8
  store i64 95, i64* %score, align 8
  %percentage = alloca double, align 8
  store double 9.55e1, double* %percentage, align 8
  %formatted = alloca i8*, align 8
  store i8* null, i8** %formatted, align 8
  %str_ptr.16 = load i8*, i8** %formatted, align 8
  call i32 @puts(i8* %str_ptr.16)
  %str_ptr.17 = getelementptr [21 x i8], [21 x i8]* @.str.4, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.17)
  %analysis = alloca i8*, align 8
  store i8* null, i8** %analysis, align 8
  %str_ptr.21 = getelementptr [19 x i8], [19 x i8]* @.str.5, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.21)
  %str_ptr.22 = getelementptr [44 x i8], [44 x i8]* @.str.6, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.22)
  ret i32 0
}
