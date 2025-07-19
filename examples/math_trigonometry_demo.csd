fr fr Comprehensive demonstration of CURSED trigonometric functions
fr fr This example showcases all trigonometric and hyperbolic functions
fr fr with practical usage examples and error handling

yeet "stdlib::math::trigonometry"
yeet "stdlib::math::constants"
yeet "stdlib::io::console"

func main() -> Result<(), Error> {
    println("=== CURSED Trigonometric Functions Demo ===")?;
    
    // Basic trigonometric functions
    demo_basic_trig()?;
    
    // Inverse trigonometric functions
    demo_inverse_trig()?;
    
    // Hyperbolic functions
    demo_hyperbolic()?;
    
    // Special angles and optimizations
    demo_special_angles()?;
    
    // Degree/radian conversions
    demo_conversions()?;
    
    // Advanced functions
    demo_advanced_functions()?;
    
    // Error handling examples
    demo_error_handling()?;
    
    // Practical applications
    demo_practical_applications()?;
    
    println("\n=== Demo Complete ===")?;
    return Ok(());
}

func demo_basic_trig() -> Result<(), Error> {
    println("\n--- Basic Trigonometric Functions ---")?;
    
    sus angles = [0.0, PI/6, PI/4, PI/3, PI/2, PI, 3*PI/2, 2*PI];
    sus angle_names = ["0°", "30°", "45°", "60°", "90°", "180°", "270°", "360°"];
    
    lowkey (sus i = 0; i < angles.len(); i++) {
        sus angle = angles[i];
        sus name = angle_names[i];
        
        printf("Angle {}: {} radians\n", [name, angle])?;
        
        // Basic trig functions with error handling
        match sin(angle) {
            Ok(value) => printf("  sin({}) = {:.6f}\n", [name, value])?,
            Err(e) => printf("  sin({}) = ERROR: {}\n", [name, e])?
        }
        
        match cos(angle) {
            Ok(value) => printf("  cos({}) = {:.6f}\n", [name, value])?,
            Err(e) => printf("  cos({}) = ERROR: {}\n", [name, e])?
        }
        
        match tan(angle) {
            Ok(value) => printf("  tan({}) = {:.6f}\n", [name, value])?,
            Err(e) => printf("  tan({}) = ERROR: {}\n", [name, e])?
        }
        
        println("")?;
    }
    
    return Ok(());
}

func demo_inverse_trig() -> Result<(), Error> {
    println("\n--- Inverse Trigonometric Functions ---")?;
    
    sus values = [-1.0, -0.866, -0.707, -0.5, 0.0, 0.5, 0.707, 0.866, 1.0];
    
    lowkey (value in values) {
        printf("Value: {:.3f}\n", [value])?;
        
        // Arcsine
        match asin(value) {
            Ok(result) => {
                sus degrees = rad_to_deg(result)?;
                printf("  asin({:.3f}) = {:.6f} rad ({:.2f}°)\n", [value, result, degrees])?;
            },
            Err(e) => printf("  asin({:.3f}) = ERROR: {}\n", [value, e])?
        }
        
        // Arccosine
        match acos(value) {
            Ok(result) => {
                sus degrees = rad_to_deg(result)?;
                printf("  acos({:.3f}) = {:.6f} rad ({:.2f}°)\n", [value, result, degrees])?;
            },
            Err(e) => printf("  acos({:.3f}) = ERROR: {}\n", [value, e])?
        }
        
        // Arctangent (no domain restrictions)
        sus atan_result = atan(value)?;
        sus atan_degrees = rad_to_deg(atan_result)?;
        printf("  atan({:.3f}) = {:.6f} rad ({:.2f}°)\n", [value, atan_result, atan_degrees])?;
        
        println("")?;
    }
    
    // atan2 examples
    println("atan2 examples (quadrant-aware):")?;
    sus coords = [(1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0), (1.0, -1.0), (0.0, 1.0)];
    
    lowkey ((y, x) in coords) {
        match atan2(y, x) {
            Ok(result) => {
                sus degrees = rad_to_deg(result)?;
                printf("  atan2({:.1f}, {:.1f}) = {:.6f} rad ({:.2f}°)\n", 
                       [y, x, result, degrees])?;
            },
            Err(e) => printf("  atan2({:.1f}, {:.1f}) = ERROR: {}\n", [y, x, e])?
        }
    }
    
    return Ok(());
}

func demo_hyperbolic() -> Result<(), Error> {
    println("\n--- Hyperbolic Functions ---")?;
    
    sus values = [-2.0, -1.0, -0.5, 0.0, 0.5, 1.0, 2.0];
    
    lowkey (value in values) {
        printf("Value: {:.1f}\n", [value])?;
        
        // Hyperbolic sine
        match sinh(value) {
            Ok(result) => printf("  sinh({:.1f}) = {:.6f}\n", [value, result])?,
            Err(e) => printf("  sinh({:.1f}) = ERROR: {}\n", [value, e])?
        }
        
        // Hyperbolic cosine
        match cosh(value) {
            Ok(result) => printf("  cosh({:.1f}) = {:.6f}\n", [value, result])?,
            Err(e) => printf("  cosh({:.1f}) = ERROR: {}\n", [value, e])?
        }
        
        // Hyperbolic tangent
        match tanh(value) {
            Ok(result) => printf("  tanh({:.1f}) = {:.6f}\n", [value, result])?,
            Err(e) => printf("  tanh({:.1f}) = ERROR: {}\n", [value, e])?
        }
        
        println("")?;
    }
    
    println("Inverse hyperbolic functions:")?;
    
    // asinh (no domain restrictions)
    lowkey (value in values) {
        match asinh(value) {
            Ok(result) => printf("  asinh({:.1f}) = {:.6f}\n", [value, result])?,
            Err(e) => printf("  asinh({:.1f}) = ERROR: {}\n", [value, e])?
        }
    }
    
    // acosh (domain: x >= 1)
    sus acosh_values = [1.0, 1.5, 2.0, 3.0, 5.0];
    lowkey (value in acosh_values) {
        match acosh(value) {
            Ok(result) => printf("  acosh({:.1f}) = {:.6f}\n", [value, result])?,
            Err(e) => printf("  acosh({:.1f}) = ERROR: {}\n", [value, e])?
        }
    }
    
    // atanh (domain: -1 < x < 1)
    sus atanh_values = [-0.9, -0.5, 0.0, 0.5, 0.9];
    lowkey (value in atanh_values) {
        match atanh(value) {
            Ok(result) => printf("  atanh({:.1f}) = {:.6f}\n", [value, result])?,
            Err(e) => printf("  atanh({:.1f}) = ERROR: {}\n", [value, e])?
        }
    }
    
    return Ok(());
}

func demo_special_angles() -> Result<(), Error> {
    println("\n--- Special Angles and Optimizations ---")?;
    
    println("Common special angles with exact values:")?;
    
    sus special_degrees = [0.0, 30.0, 45.0, 60.0, 90.0, 120.0, 135.0, 150.0, 180.0];
    
    lowkey (deg in special_degrees) {
        match special_degrees_to_radians(deg) {
            Some(rad) => {
                printf("{:3.0}° = {:.6f} rad", [deg, rad])?;
                
                // Show that this is a special angle
                facts is_special = is_special_angle(rad);
                printf(" (special: {})", [is_special])?;
                
                // Get exact values if available
                match sin_special_angle(rad) {
                    Some(sin_val) => printf(", sin = {:.6f}", [sin_val])?,
                    None => {}
                }
                
                match cos_special_angle(rad) {
                    Some(cos_val) => printf(", cos = {:.6f}", [cos_val])?,
                    None => {}
                }
                
                println("")?;
            },
            None => printf("{:3.0}° = not a special angle\n", [deg])?
        }
    }
    
    return Ok(());
}

func demo_conversions() -> Result<(), Error> {
    println("\n--- Degree/Radian Conversions ---")?;
    
    println("Degree to Radian conversions:")?;
    sus degrees = [0.0, 30.0, 45.0, 60.0, 90.0, 180.0, 270.0, 360.0];
    
    lowkey (deg in degrees) {
        sus rad = degrees_to_radians(deg)?;
        printf("{:3.0}° = {:.6f} radians\n", [deg, rad])?;
    }
    
    println("\nRadian to Degree conversions:")?;
    sus radians = [0.0, PI/6, PI/4, PI/3, PI/2, PI, 3*PI/2, 2*PI];
    
    lowkey (rad in radians) {
        sus deg = radians_to_degrees(rad)?;
        printf("{:.6f} rad = {:.2f}°\n", [rad, deg])?;
    }
    
    println("\nTrigonometric functions with degree input:")?;
    lowkey (deg in [0.0, 30.0, 45.0, 60.0, 90.0]) {
        sus sin_val = sin_deg(deg)?;
        sus cos_val = cos_deg(deg)?;
        printf("{:2.0}°: sin = {:.6f}, cos = {:.6f}\n", [deg, sin_val, cos_val])?;
    }
    
    return Ok(());
}

func demo_advanced_functions() -> Result<(), Error> {
    println("\n--- Advanced Trigonometric Functions ---")?;
    
    sus angle = PI/3; // 60 degrees
    printf("For angle = π/3 ({:.2f}°):\n", [radians_to_degrees(angle)?])?;
    
    // Reciprocal functions
    printf("  sec(π/3) = {:.6f}\n", [sec(angle)?])?;
    printf("  csc(π/3) = {:.6f}\n", [csc(angle)?])?;
    printf("  cot(π/3) = {:.6f}\n", [cot(angle)?])?;
    
    // Versine functions
    printf("  versin(π/3) = {:.6f}\n", [versin(angle)?])?;
    printf("  coversin(π/3) = {:.6f}\n", [coversin(angle)?])?;
    printf("  haversin(π/3) = {:.6f}\n", [haversin(angle)?])?;
    
    // Extended functions
    printf("  exsec(π/3) = {:.6f}\n", [exsec(angle)?])?;
    printf("  excsc(π/3) = {:.6f}\n", [excsc(angle)?])?;
    printf("  chord(π/3) = {:.6f}\n", [chord(angle)?])?;
    
    println("\nCombined operations:")?;
    
    // sincos - compute both efficiently
    sus (sin_val, cos_val) = sincos(angle)?;
    printf("  sincos(π/3) = ({:.6f}, {:.6f})\n", [sin_val, cos_val])?;
    
    // All six trig functions at once
    match trig_all(angle) {
        Ok((sin_val, cos_val, tan_val, sec_val, csc_val, cot_val)) => {
            printf("  trig_all(π/3) = sin:{:.3f} cos:{:.3f} tan:{:.3f}\n", 
                   [sin_val, cos_val, tan_val])?;
            printf("                   sec:{:.3f} csc:{:.3f} cot:{:.3f}\n",
                   [sec_val, csc_val, cot_val])?;
        },
        Err(e) => printf("  trig_all(π/3) = ERROR: {}\n", [e])?
    }
    
    return Ok(());
}

func demo_error_handling() -> Result<(), Error> {
    println("\n--- Error Handling Examples ---")?;
    
    println("Domain errors:")?;
    
    // asin/acos domain errors
    sus invalid_values = [1.5, -1.5, 2.0];
    lowkey (value in invalid_values) {
        match asin(value) {
            Ok(_) => printf("  asin({}) = unexpected success\n", [value])?,
            Err(e) => printf("  asin({}) = ERROR: {}\n", [value, e])?
        }
    }
    
    // tan undefined values
    sus tan_undefined = [PI/2, 3*PI/2, -PI/2];
    lowkey (value in tan_undefined) {
        match tan(value) {
            Ok(_) => printf("  tan({:.3f}) = unexpected success\n", [value])?,
            Err(e) => printf("  tan({:.3f}) = ERROR: {}\n", [value, e])?
        }
    }
    
    // atanh domain errors
    sus atanh_invalid = [-1.0, 1.0, 1.5];
    lowkey (value in atanh_invalid) {
        match atanh(value) {
            Ok(_) => printf("  atanh({}) = unexpected success\n", [value])?,
            Err(e) => printf("  atanh({}) = ERROR: {}\n", [value, e])?
        }
    }
    
    // acosh domain errors
    sus acosh_invalid = [0.5, 0.0, -1.0];
    lowkey (value in acosh_invalid) {
        match acosh(value) {
            Ok(_) => printf("  acosh({}) = unexpected success\n", [value])?,
            Err(e) => printf("  acosh({}) = ERROR: {}\n", [value, e])?
        }
    }
    
    // Overflow cases
    println("\nOverflow handling:")?;
    sus large_values = [1000.0, -1000.0];
    lowkey (value in large_values) {
        match sinh(value) {
            Ok(result) => printf("  sinh({}) = {}\n", [value, result])?,
            Err(e) => printf("  sinh({}) = ERROR: {}\n", [value, e])?
        }
    }
    
    return Ok(());
}

func demo_practical_applications() -> Result<(), Error> {
    println("\n--- Practical Applications ---")?;
    
    // 1. Calculate angle between two vectors
    println("1. Angle between vectors:")?;
    sus v1 = (3.0, 4.0);
    sus v2 = (1.0, 1.0);
    
    sus dot_product = v1.0 * v2.0 + v1.1 * v2.1;
    sus magnitude1 = sqrt(v1.0 * v1.0 + v1.1 * v1.1)?;
    sus magnitude2 = sqrt(v2.0 * v2.0 + v2.1 * v2.1)?;
    
    sus cos_angle = dot_product / (magnitude1 * magnitude2);
    sus angle_rad = acos(cos_angle)?;
    sus angle_deg = radians_to_degrees(angle_rad)?;
    
    printf("   Vector 1: ({:.1f}, {:.1f})\n", [v1.0, v1.1])?;
    printf("   Vector 2: ({:.1f}, {:.1f})\n", [v2.0, v2.1])?;
    printf("   Angle between: {:.2f}° ({:.4f} rad)\n", [angle_deg, angle_rad])?;
    
    // 2. Polar to Cartesian conversion
    println("\n2. Polar to Cartesian conversion:")?;
    sus r = 5.0;
    sus theta_deg = 30.0;
    sus theta_rad = degrees_to_radians(theta_deg)?;
    
    sus x = r * cos(theta_rad)?;
    sus y = r * sin(theta_rad)?;
    
    printf("   Polar: (r={:.1f}, θ={:.1f}°)\n", [r, theta_deg])?;
    printf("   Cartesian: (x={:.3f}, y={:.3f})\n", [x, y])?;
    
    // 3. Cartesian to Polar conversion
    println("\n3. Cartesian to Polar conversion:")?;
    sus x_cart = 3.0;
    sus y_cart = 4.0;
    
    sus r_calc = sqrt(x_cart * x_cart + y_cart * y_cart)?;
    sus theta_calc = atan2(y_cart, x_cart)?;
    sus theta_deg_calc = radians_to_degrees(theta_calc)?;
    
    printf("   Cartesian: (x={:.1f}, y={:.1f})\n", [x_cart, y_cart])?;
    printf("   Polar: (r={:.3f}, θ={:.2f}°)\n", [r_calc, theta_deg_calc])?;
    
    // 4. Sine wave generation
    println("\n4. Sine wave values (amplitude=2, frequency=1, phase=0):")?;
    sus amplitude = 2.0;
    sus frequency = 1.0;
    sus phase = 0.0;
    
    lowkey (sus t = 0; t <= 360; t += 45) {
        sus time_rad = degrees_to_radians(t)?;
        sus value = amplitude * sin(frequency * time_rad + phase)?;
        printf("   t={:3}°: y = {:.3f}\n", [t, value])?;
    }
    
    // 5. Verify trigonometric identities
    println("\n5. Verifying trig identities at random angle:")?;
    sus test_angle = 0.7; // random angle in radians
    
    sus sin_val = sin(test_angle)?;
    sus cos_val = cos(test_angle)?;
    sus tan_val = tan(test_angle)?;
    
    // Pythagorean identity: sin²(x) + cos²(x) = 1
    sus pythagorean = sin_val * sin_val + cos_val * cos_val;
    printf("   sin²(x) + cos²(x) = {:.10f} (should be 1.0)\n", [pythagorean])?;
    
    // tan(x) = sin(x) / cos(x)
    sus tan_identity = sin_val / cos_val;
    printf("   tan(x) = {:.6f}, sin(x)/cos(x) = {:.6f}\n", [tan_val, tan_identity])?;
    
    return Ok(());
}
