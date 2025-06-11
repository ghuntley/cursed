// Trigonometric Functions Demo for CURSED Programming Language
// 
// This example demonstrates the comprehensive trigonometric functionality
// available in the CURSED standard library, including:
// - Basic trigonometric functions (sin, cos, tan)
// - Inverse trigonometric functions (asin, acos, atan, atan2)
// - Hyperbolic functions (sinh, cosh, tanh)
// - Inverse hyperbolic functions (asinh, acosh, atanh)
// - Angle conversion utilities
// - Advanced trigonometric functions

import "stdlib::math";

facts main() {
    println("=== CURSED Trigonometric Functions Demo ===");
    
    // Basic Trigonometric Functions
    println("\n📐 Basic Trigonometric Functions:");
    
    // Common angles in radians
    facts angles = [0.0, PI / 6.0, PI / 4.0, PI / 3.0, PI / 2.0, PI];
    facts angle_names = ["0°", "30°", "45°", "60°", "90°", "180°"];
    
    lowkey (sus i = 0; i < len(angles); i++) {
        facts angle = angles[i];
        facts name = angle_names[i];
        
        facts sin_val = sin(angle)?;
        facts cos_val = cos(angle)?;
        facts tan_val = tan(angle)?;
        
        printf("Angle {}: sin={:.4f}, cos={:.4f}, tan={:.4f}\n", 
               &[name, sin_val, cos_val, tan_val])?;
    }
    
    // Inverse Trigonometric Functions
    println("\n🔄 Inverse Trigonometric Functions:");
    
    facts values = [-1.0, -0.5, 0.0, 0.5, 1.0];
    lowkey (sus value in values) {
        // asin and acos have domain [-1, 1]
        facts asin_val = asin(value)?;
        facts acos_val = acos(value)?;
        
        printf("asin({:.1f}) = {:.4f} rad, acos({:.1f}) = {:.4f} rad\n",
               &[value, asin_val, value, acos_val])?;
    }
    
    // atan and atan2 (no domain restrictions for atan)
    facts atan_val = atan(1.0)?;
    facts atan2_val = atan2(1.0, 1.0)?;
    printf("atan(1.0) = {:.4f} rad (π/4), atan2(1,1) = {:.4f} rad\n", 
           &[atan_val, atan2_val])?;
    
    // Angle Conversion
    println("\n🔄 Angle Conversion:");
    
    facts deg_angles = [0.0, 30.0, 45.0, 60.0, 90.0, 180.0, 360.0];
    lowkey (sus deg in deg_angles) {
        facts rad = degrees_to_radians(deg)?;
        facts back_to_deg = radians_to_degrees(rad)?;
        
        printf("{:.0f}° = {:.4f} rad = {:.0f}°\n", &[deg, rad, back_to_deg])?;
    }
    
    // Degree-based trigonometric functions
    println("\n📐 Degree-based Functions:");
    
    lowkey (sus deg in [0.0, 30.0, 45.0, 60.0, 90.0]) {
        facts sin_deg_val = sin_deg(deg)?;
        facts cos_deg_val = cos_deg(deg)?;
        facts tan_deg_val = tan_deg(deg)?;
        
        printf("{:.0f}°: sin={:.4f}, cos={:.4f}, tan={:.4f}\n",
               &[deg, sin_deg_val, cos_deg_val, tan_deg_val])?;
    }
    
    // Hyperbolic Functions
    println("\n📈 Hyperbolic Functions:");
    
    facts hyperbolic_values = [-2.0, -1.0, 0.0, 1.0, 2.0];
    lowkey (sus x in hyperbolic_values) {
        facts sinh_val = sinh(x)?;
        facts cosh_val = cosh(x)?;
        facts tanh_val = tanh(x)?;
        
        printf("x={:.1f}: sinh={:.4f}, cosh={:.4f}, tanh={:.4f}\n",
               &[x, sinh_val, cosh_val, tanh_val])?;
    }
    
    // Inverse Hyperbolic Functions
    println("\n🔄 Inverse Hyperbolic Functions:");
    
    // asinh - domain: all real numbers
    facts asinh_val = asinh(1.0)?;
    printf("asinh(1.0) = {:.4f}\n", &[asinh_val])?;
    
    // acosh - domain: x >= 1
    facts acosh_val = acosh(2.0)?;
    printf("acosh(2.0) = {:.4f}\n", &[acosh_val])?;
    
    // atanh - domain: -1 < x < 1
    facts atanh_val = atanh(0.5)?;
    printf("atanh(0.5) = {:.4f}\n", &[atanh_val])?;
    
    // Reciprocal Trigonometric Functions
    println("\n↩️ Reciprocal Functions:");
    
    lowkey (sus angle in [PI/6.0, PI/4.0, PI/3.0]) {
        facts deg_equiv = radians_to_degrees(angle)?;
        facts sec_val = sec(angle)?;
        facts csc_val = csc(angle)?;
        facts cot_val = cot(angle)?;
        
        printf("Angle {:.0f}°: sec={:.4f}, csc={:.4f}, cot={:.4f}\n",
               &[deg_equiv, sec_val, csc_val, cot_val])?;
    }
    
    // Advanced Functions
    println("\n🚀 Advanced Trigonometric Functions:");
    
    // Sinc function
    facts sinc_values = [0.0, PI/4.0, PI/2.0, PI];
    lowkey (sus x in sinc_values) {
        facts sinc_val = sinc(x)?;
        printf("sinc({:.4f}) = {:.4f}\n", &[x, sinc_val])?;
    }
    
    // Haversine function
    facts haversine_val = haversine(PI/3.0)?;
    printf("haversine(π/3) = {:.4f}\n", &[haversine_val])?;
    
    // Law of cosines - calculate third side of triangle
    facts side_c = law_of_cosines(3.0, 4.0, PI/2.0)?; // Right triangle
    printf("Triangle with sides 3, 4 and angle π/2: third side = {:.4f}\n", &[side_c])?;
    
    // Angle normalization
    println("\n🔄 Angle Normalization:");
    
    facts unnormalized_angles = [3.0 * PI, -PI/2.0, 5.0 * PI / 2.0];
    lowkey (sus angle in unnormalized_angles) {
        facts normalized = normalize_angle(angle)?;
        facts signed_normalized = normalize_angle_signed(angle)?;
        
        printf("Angle {:.4f} → normalized: {:.4f}, signed: {:.4f}\n",
               &[angle, normalized, signed_normalized])?;
    }
    
    // Real-world Applications
    println("\n🌍 Real-world Applications:");
    
    // Calculate distance between two cities using haversine formula
    // Example: Distance between New York (40.7128°N, 74.0060°W) and London (51.5074°N, 0.1278°W)
    facts ny_lat = degrees_to_radians(40.7128)?;
    facts ny_lon = degrees_to_radians(-74.0060)?;
    facts london_lat = degrees_to_radians(51.5074)?;
    facts london_lon = degrees_to_radians(-0.1278)?;
    facts earth_radius = 6371.0; // km
    
    facts distance = haversine_distance(ny_lat, ny_lon, london_lat, london_lon, earth_radius)?;
    printf("Distance from New York to London: {:.0f} km\n", &[distance])?;
    
    // Navigation: bearing calculation using atan2
    facts delta_lon = london_lon - ny_lon;
    facts y = sin(delta_lon) * cos(london_lat);
    facts x = cos(ny_lat) * sin(london_lat) - sin(ny_lat) * cos(london_lat) * cos(delta_lon);
    facts bearing_rad = atan2(y, x)?;
    facts bearing_deg = radians_to_degrees(bearing_rad)?;
    facts bearing_normalized = normalize_angle(bearing_deg + 360.0)? % 360.0;
    
    printf("Initial bearing from New York to London: {:.1f}°\n", &[bearing_normalized])?;
    
    // Signal processing: sinc function for digital filters
    println("\n📡 Signal Processing Example:");
    facts sample_points = [-2.0, -1.0, 0.0, 1.0, 2.0];
    lowkey (sus t in sample_points) {
        facts sinc_val = sinc(PI * t)?;
        printf("sinc(π * {:.0f}) = {:.6f}\n", &[t, sinc_val])?;
    }
    
    // Error Handling Demonstration
    println("\n❌ Error Handling Examples:");
    
    // Domain error examples
    match asin(2.0) {
        Ok(_) => println("This shouldn't happen"),
        Err(error) => printf("Expected domain error: {}\n", &[error.to_string()])?;
    }
    
    match acosh(0.5) {
        Ok(_) => println("This shouldn't happen"),
        Err(error) => printf("Expected domain error: {}\n", &[error.to_string()])?;
    }
    
    match atanh(1.5) {
        Ok(_) => println("This shouldn't happen"),
        Err(error) => printf("Expected domain error: {}\n", &[error.to_string()])?;
    }
    
    // All trigonometric functions at once
    println("\n🎯 All Trigonometric Functions for π/4:");
    facts angle = PI / 4.0;
    facts (sin_val, cos_val, tan_val, sec_val, csc_val, cot_val) = all_trig_functions(angle)?;
    
    printf("sin={:.6f}, cos={:.6f}, tan={:.6f}\n", &[sin_val, cos_val, tan_val])?;
    printf("sec={:.6f}, csc={:.6f}, cot={:.6f}\n", &[sec_val, csc_val, cot_val])?;
    
    println("\n✅ Trigonometric functions demo completed successfully!");
}
