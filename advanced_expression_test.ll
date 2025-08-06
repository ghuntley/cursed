; Generated LLVM IR for CURSED program
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

@.str.0 = private unnamed_addr constant [37 x i8] c"Advanced expression tests completed!\00", align 1
@.int_fmt = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1
@.float_fmt = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.bool_true = private unnamed_addr constant [6 x i8] c"based\00", align 1
@.bool_false = private unnamed_addr constant [7 x i8] c"cringe\00", align 1

define i32 @main() {
entry:
  ; Variable: numbers [normie] = [1, 2, 3, 4, 5]
  ; Variable: first_num normie = numbers[0]
  %first_num = alloca i32, align 4
  store i32 0, i32* %first_num, align 4
  ; Variable: last_num normie = numbers[4]
  %last_num = alloca i32, align 4
  store i32 0, i32* %last_num, align 4
  ; Unknown variable: "Array test:", first_num, last_num
  ; Variable: p1 Point = Point{x: 10, y: 20}
  ; Variable: x_coord normie = p1.x
  %x_coord = alloca i32, align 4
  store i32 0, i32* %x_coord, align 4
  ; Variable: y_coord normie = p1.y
  %y_coord = alloca i32, align 4
  store i32 0, i32* %y_coord, align 4
  ; Unknown variable: "Struct test:", x_coord, y_coord
  ; Variable: sum normie = add(5, 3)
  %sum = alloca i32, align 4
  store i32 0, i32* %sum, align 4
  ; Variable: product normie = multiply(4, 7)
  %product = alloca i32, align 4
  store i32 0, i32* %product, align 4
  ; Unknown variable: "Function calls:", sum, product
  ; Variable: complex_expr normie = add(multiply(2, 3), add(4, 5))
  %complex_expr = alloca i32, align 4
  store i32 0, i32* %complex_expr, align 4
  ; Variable: precedence_test normie = 2 + 3 * 4 + 5
  %precedence_test = alloca i32, align 4
  %add_result.9 = add i32 2, 0
  store i32 %add_result.9, i32* %precedence_test, align 4
  ; Unknown variable: "Complex expressions:", complex_expr, precedence_test
  ; Variable: points [Point] = [
  ; Variable: second_point Point = points[1]
  ; Variable: second_x normie = points[1].x
  %second_x = alloca i32, align 4
  store i32 0, i32* %second_x, align 4
  ; Unknown variable: "Array of structs:", second_point.x, second_x
  ; Variable: dx normie = p1.x - p2.x
  %dx = alloca i32, align 4
  %sub_result.14 = sub i32 0, 0
  store i32 %sub_result.14, i32* %dx, align 4
  ; Variable: dy normie = p1.y - p2.y
  %dy = alloca i32, align 4
  %sub_result.16 = sub i32 0, 0
  store i32 %sub_result.16, i32* %dy, align 4
  ; Variable: dist normie = distance(points[0], points[2])
  %dist = alloca i32, align 4
  store i32 0, i32* %dist, align 4
  ; Unknown variable: "Distance calculation:", dist
  ; Unknown variable: "After modification:", numbers[0], p1.x
  ; Unknown variable: "Drawing circle with radius:", c.radius
  ; Variable: tuple_x normie = coords.0
  %tuple_x = alloca i32, align 4
  store i32 0, i32* %tuple_x, align 4
  ; Variable: tuple_y normie = coords.1
  %tuple_y = alloca i32, align 4
  store i32 0, i32* %tuple_y, align 4
  ; Unknown variable: "Tuple test:", tuple_x, tuple_y
  ; Variable: float_val meal = 3.14
  %float_val = alloca double, align 8
  store double 3.14e0, double* %float_val, align 8
  ; Variable: int_from_float normie = float_val.(normie)
  %int_from_float = alloca i32, align 4
  store i32 0, i32* %int_from_float, align 4
  ; Variable: float_from_int meal = 42.(meal)
  %float_from_int = alloca double, align 8
  store double 0.0, double* %float_from_int, align 8
  ; Unknown variable: "Type casts:", int_from_float, float_from_int
  ; String literal: Advanced expression tests completed!
  %str_ptr.24 = getelementptr [37 x i8], [37 x i8]* @.str.0, i32 0, i32 0
  call i32 @puts(i8* %str_ptr.24)
  ret i32 0
}
