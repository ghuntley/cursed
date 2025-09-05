/**
 * CURSED Math Basic Functions Demo
 * 
 * Demonstrates the fundamental mathematical functions available
 * in the CURSED standard library's math::basic module.
 */

yeet "stdlib::math::basic"

func main() -> Result<(), String> {
    println!("CURSED Math Basic Functions Demo")?;
    println!("================================")?;
    
    // Basic operations
    println!("\n--- Basic Operations ---")?;
    let x = -5.7;
    let y = 3.2;
    
    println!("abs({}) = {}", x, abs(x))?;
    println!("min({}, {}) = {}", x, y, min(x, y))?;
    println!("max({}, {}) = {}", x, y, max(x, y))?;
    println!("sign({}) = {}", x, sign(x))?;
    println!("sign({}) = {}", y, sign(y))?;
    
    // Rounding functions
    println!("\n--- Rounding Functions ---")?;
    let val = 3.14159;
    println!("floor({}) = {}", val, floor(val)?)?;
    println!("ceil({}) = {}", val, ceil(val)?)?;
    println!("round({}) = {}", val, round(val)?)?;
    println!("trunc({}) = {}", val, math_truncate(val)?)?;
    println!("fract({}) = {}", val, fract(val)?)?;
    println!("round_to_decimals({}, 2) = {}", val, round_to_decimals(val, 2)?)?;
    
    // Power and root functions
    println!("\n--- Power and Root Functions ---")?;
    let base = 9.0;
    println!("sqrt({}) = {}", base, sqrt(base)?)?;
    println!("cbrt(8.0) = {}", cbrt(8.0)?)?;
    println!("square({}) = {}", base, square(base)?)?;
    println!("cube(3.0) = {}", cube(3.0)?)?;
    println!("pow({}, 2.0) = {}", base, pow(base, 2.0)?)?;
    println!("pow2(10) = {}", pow2(10)?)?;
    println!("pow10(3) = {}", pow10(3)?)?;
    println!("nth_root(16.0, 4.0) = {}", nth_root(16.0, 4.0)?)?;
    
    // Geometric functions
    println!("\n--- Geometric Functions ---")?;
    let a = 3.0;
    let b = 4.0;
    println!("hypot({}, {}) = {}", a, b, hypot(a, b)?)?;
    println!("reciprocal({}) = {}", a, reciprocal(a)?)?;
    
    // Comparison and utility functions
    println!("\n--- Comparison Functions ---")?;
    let epsilon = 0.001;
    println!("is_zero(0.0005, {}) = {}", epsilon, is_zero(0.0005, epsilon)?)?;
    println!("is_equal(1.0, 1.0005, {}) = {}", epsilon, is_equal(1.0, 1.0005, epsilon)?)?;
    
    // Interpolation functions
    println!("\n--- Interpolation Functions ---")?;
    let start = 0.0;
    let end = 10.0;
    let t = 0.3;
    let lerp_result = lerp(start, end, t)?;
    println!("lerp({}, {}, {}) = {}", start, end, t, lerp_result)?;
    println!("inverse_lerp({}, {}, {}) = {}", start, end, lerp_result, inverse_lerp(start, end, lerp_result)?)?;
    println!("smooth_step({}, {}, {}) = {}", start, end, t, smooth_step(start, end, t)?)?;
    println!("smoother_step({}, {}, {}) = {}", start, end, t, smoother_step(start, end, t)?)?;
    
    // Mean functions
    println!("\n--- Mean Functions ---")?;
    let num1 = 4.0;
    let num2 = 16.0;
    println!("average({}, {}) = {}", num1, num2, average(num1, num2)?)?;
    println!("geometric_mean({}, {}) = {}", num1, num2, geometric_mean(num1, num2)?)?;
    println!("harmonic_mean({}, {}) = {}", num1, num2, harmonic_mean(num1, num2)?)?;
    
    // Range mapping
    println!("\n--- Range Mapping ---")?;
    let value = 5.0;
    let mapped = map_range(value, 0.0, 10.0, 0.0, 100.0)?;
    println!("map_range({}, 0-10, 0-100) = {}", value, mapped)?;
    
    // Integer functions
    println!("\n--- Integer Functions ---")?;
    let int_a = 12;
    let int_b = 8;
    println!("gcd({}, {}) = {}", int_a, int_b, gcd(int_a, int_b)?)?;
    println!("lcm({}, {}) = {}", int_a, int_b, lcm(int_a, int_b)?)?;
    println!("is_even({}) = {}", int_a, is_even(int_a))?;
    println!("is_odd({}) = {}", int_a, is_odd(int_a))?;
    println!("abs_i32(-15) = {}", abs_i32(-15))?;
    println!("min_i32({}, {}) = {}", int_a, int_b, min_i32(int_a, int_b))?;
    println!("max_i32({}, {}) = {}", int_a, int_b, max_i32(int_a, int_b))?;
    println!("clamp_i32(15, 5, 10) = {}", clamp_i32(15, 5, 10)?)?;
    
    // Modular arithmetic
    println!("\n--- Modular Arithmetic ---")?;
    let dividend = 7.5;
    let divisor = 3.0;
    println!("remainder({}, {}) = {}", dividend, divisor, remainder(dividend, divisor)?)?;
    println!("modulo({}, {}) = {}", dividend, divisor, modulo(dividend, divisor)?)?;
    println!("modulo(-{}, {}) = {}", dividend, divisor, modulo(-dividend, divisor)?)?;
    
    // Clamping examples
    println!("\n--- Clamping Examples ---")?;
    let test_values = vec![0.5, 5.0, 15.0];
    let min_val = 1.0;
    let max_val = 10.0;
    
    for &val in &test_values {
        println!("clamp({}, {}, {}) = {}", val, min_val, max_val, clamp(val, min_val, max_val)?)?;
    }
    
    println!("\n--- Error Handling Demo ---")?;
    // Demonstrate error handling
    match sqrt(-1.0) {
        Ok(result) => println!("sqrt(-1.0) = {}", result)?,
        Err(e) => println!("sqrt(-1.0) failed: {}", e)?,
    }
    
    match pow(0.0, -1.0) {
        Ok(result) => println!("pow(0.0, -1.0) = {}", result)?,
        Err(e) => println!("pow(0.0, -1.0) failed: {}", e)?,
    }
    
    match clamp(5.0, 10.0, 1.0) {
        Ok(result) => println!("clamp(5.0, 10.0, 1.0) = {}", result)?,
        Err(e) => println!("clamp(5.0, 10.0, 1.0) failed: {}", e)?,
    }
    
    println!("\nDemo completed successfully!")?;
    Ok(())
}
