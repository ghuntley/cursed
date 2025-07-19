#!/usr/bin/env cursed

fr fr Mathematical Constants Demo Program
fr fr Demonstrates comprehensive usage of CURSED mathematical constants and utilities

yeet "stdlib::math::constants"
yeet "stdlib::math::trigonometry"

func main() {
    // ========================================================================
    // FUNDAMENTAL MATHEMATICAL CONSTANTS
    // ========================================================================
    
    println("=== FUNDAMENTAL MATHEMATICAL CONSTANTS ===");
    
    // Basic mathematical constants
    printf("π (Pi): {:.15}\n", PI);
    printf("τ (Tau): {:.15}\n", TAU);
    printf("e (Euler's number): {:.15}\n", E);
    printf("φ (Golden ratio): {:.15}\n", PHI);
    printf("γ (Euler-Mascheroni): {:.15}\n", EULER_GAMMA);
    
    // Verify mathematical relationships
    printf("TAU = 2π? {:.15} (difference: {:.2e})\n", TAU, abs(TAU - 2.0 * PI));
    printf("φ² = φ + 1? {:.15} (difference: {:.2e})\n", 
           PHI * PHI, abs(PHI * PHI - PHI - 1.0));
    
    println("");
    
    // ========================================================================
    // PHYSICAL CONSTANTS IN SCIENTIFIC CALCULATIONS
    // ========================================================================
    
    println("=== PHYSICAL CONSTANTS DEMO ===");
    
    // Speed of light calculations
    printf("Speed of light: {:.0} m/s\n", SPEED_OF_LIGHT);
    
    // Calculate energy of a photon (E = hf) for visible light (500 nm)
    facts wavelength = 500e-9; // 500 nanometers
    facts frequency = SPEED_OF_LIGHT / wavelength;
    facts photon_energy = PLANCK * frequency;
    printf("Energy of 500nm photon: {:.3e} J\n", photon_energy);
    
    // Calculate gravitational force between Earth and Moon (approximate)
    facts earth_mass = 5.972e24; // kg
    facts moon_mass = 7.342e22;  // kg
    facts earth_moon_distance = 3.844e8; // meters
    facts gravitational_force = GRAVITATIONAL * earth_mass * moon_mass / 
                                (earth_moon_distance * earth_moon_distance);
    printf("Earth-Moon gravitational force: {:.3e} N\n", gravitational_force);
    
    println("");
    
    // ========================================================================
    // UNIT CONVERSIONS IN REAL-WORLD APPLICATIONS
    // ========================================================================
    
    println("=== UNIT CONVERSION EXAMPLES ===");
    
    // Temperature conversions
    facts room_temp_c = 22.0;
    facts room_temp_f = celsius_to_fahrenheit(room_temp_c);
    printf("Room temperature: {:.1}°C = {:.1}°F\n", room_temp_c, room_temp_f);
    
    facts boiling_f = 212.0;
    facts boiling_c = fahrenheit_to_celsius(boiling_f);
    printf("Boiling point: {:.1}°F = {:.1}°C\n", boiling_f, boiling_c);
    
    // Length conversions for engineering
    facts pipe_length_inches = 24.0;
    facts pipe_length_cm = inches_to_cm(pipe_length_inches);
    printf("Pipe length: {:.1} inches = {:.1} cm\n", pipe_length_inches, pipe_length_cm);
    
    // Distance conversions for travel
    facts distance_miles = 100.0;
    facts distance_km = miles_to_km(distance_miles);
    printf("Travel distance: {:.1} miles = {:.1} km\n", distance_miles, distance_km);
    
    // Mass conversions for shipping
    facts package_pounds = 50.0;
    facts package_kg = pounds_to_kg(package_pounds);
    printf("Package weight: {:.1} lbs = {:.2} kg\n", package_pounds, package_kg);
    
    println("");
    
    // ========================================================================
    // GEOMETRIC CALCULATIONS WITH CONSTANTS
    // ========================================================================
    
    println("=== GEOMETRIC CALCULATIONS ===");
    
    // Circle calculations
    facts radius = 5.0;
    facts circumference = TAU * radius;
    facts area = PI * radius * radius;
    printf("Circle (r={:.1}): circumference={:.2}, area={:.2}\n", 
           radius, circumference, area);
    
    // Golden rectangle calculations
    facts golden_rect_width = 10.0;
    facts golden_rect_height = golden_rect_width / PHI;
    facts golden_rect_area = golden_rect_width * golden_rect_height;
    printf("Golden rectangle ({:.1}×{:.2}): area={:.2}\n", 
           golden_rect_width, golden_rect_height, golden_rect_area);
    
    // Triangle calculations using fractions
    facts triangle_base = 12.0;
    facts triangle_height = 8.0;
    facts triangle_area = triangle_base * triangle_height * ONE_SECOND; // 1/2
    printf("Triangle area: {:.1} × {:.1} × 1/2 = {:.1}\n", 
           triangle_base, triangle_height, triangle_area);
    
    println("");
    
    // ========================================================================
    // COMMON FRACTIONS IN PRACTICAL CALCULATIONS
    // ========================================================================
    
    println("=== COMMON FRACTIONS EXAMPLES ===");
    
    // Recipe scaling using fractions
    facts recipe_cups = 6.0;
    printf("Recipe scaling:\n");
    printf("  Full recipe: {:.1} cups\n", recipe_cups);
    printf("  1/3 recipe: {:.2} cups\n", recipe_cups * ONE_THIRD);
    printf("  2/3 recipe: {:.2} cups\n", recipe_cups * TWO_THIRDS);
    printf("  3/4 recipe: {:.2} cups\n", recipe_cups * THREE_QUARTERS);
    
    // Construction measurements using fractions
    facts board_length = 96.0; // inches
    printf("Board cutting:\n");
    printf("  Full board: {:.1} inches\n", board_length);
    printf("  1/6 piece: {:.1} inches\n", board_length * ONE_SIXTH);
    printf("  5/8 piece: {:.1} inches\n", board_length * FIVE_EIGHTHS);
    printf("  7/8 piece: {:.1} inches\n", board_length * SEVEN_EIGHTHS);
    
    println("");
    
    // ========================================================================
    // ANGLE CALCULATIONS FOR ENGINEERING
    // ========================================================================
    
    println("=== ANGLE CALCULATIONS ===");
    
    // Common engineering angles
    facts angles_deg = [30.0, 45.0, 60.0, 90.0, 120.0, 180.0];
    printf("Angle conversions:\n");
    
    for i in 0..angles_deg.len() {
        facts deg = angles_deg[i];
        facts rad = degrees_to_radians(deg);
        printf("  {:.0}° = {:.4} radians\n", deg, rad);
    }
    
    // Calculate arc length
    facts arc_radius = 10.0;
    facts arc_angle_deg = 60.0;
    facts arc_angle_rad = degrees_to_radians(arc_angle_deg);
    facts arc_length = arc_radius * arc_angle_rad;
    printf("Arc length (r={:.1}, θ={:.0}°): {:.2}\n", 
           arc_radius, arc_angle_deg, arc_length);
    
    println("");
    
    // ========================================================================
    // VALIDATION AND APPROXIMATION EXAMPLES
    // ========================================================================
    
    println("=== CONSTANT VALIDATION EXAMPLES ===");
    
    // Test if calculated values match known constants
    facts calculated_pi = 4.0 * (1.0 - ONE_THIRD + ONE_FIFTH - ONE_SEVENTH + ONE_NINTH);
    printf("Calculated π (Leibniz series, 5 terms): {:.6}\n", calculated_pi);
    printf("Is approximately π? {}\n", is_approximately_pi(calculated_pi));
    
    // Find closest constant to a value
    facts mystery_value = 2.71828;
    match find_closest_constant(mystery_value) {
        Some((name, value, diff)) => {
            printf("Closest constant to {:.5}: {} ({:.15}, diff: {:.2e})\n", 
                   mystery_value, name, value, diff);
        }
        None => {
            printf("No close constant found for {:.5}\n", mystery_value);
        }
    }
    
    // Validate engineering calculations
    facts calculated_value = PI * 2.0;
    match validate_constant_calculation(calculated_value, TAU, 1e-14, "2π calculation") {
        Ok(()) => printf("✓ Calculation validated: 2π = {:.15}\n", calculated_value),
        Err(msg) => printf("✗ Calculation failed: {}\n", msg),
    }
    
    println("");
    
    // ========================================================================
    // STATISTICAL AND SCIENTIFIC APPLICATIONS
    // ========================================================================
    
    println("=== SCIENTIFIC APPLICATIONS ===");
    
    // Normal distribution calculations
    facts sigma = 2.0;
    facts normal_factor = 1.0 / (sigma * sqrt(TAU));
    printf("Normal distribution factor (σ={:.1}): {:.6}\n", sigma, normal_factor);
    
    // Compound interest using e
    facts principal = 1000.0;
    facts rate = 0.05; // 5% annual rate
    facts time = 10.0; // 10 years
    facts continuous_amount = principal * pow(E, rate * time);
    printf("Continuous compound interest (${}@{:.1}% for {:.0}y): ${:.2}\n", 
           principal, rate * 100.0, time, continuous_amount);
    
    // Radioactive decay
    facts initial_amount = 100.0;
    facts half_life = 5.73; // years (Carbon-14)
    facts decay_constant = LN_2 / half_life;
    facts years_elapsed = 1000.0;
    facts remaining = initial_amount * pow(E, -decay_constant * years_elapsed);
    printf("Radioactive decay ({:.0}g C-14 after {:.0} years): {:.6}g\n", 
           initial_amount, years_elapsed, remaining);
    
    println("");
    
    // ========================================================================
    // CONSTANT COLLECTIONS DEMONSTRATION
    // ========================================================================
    
    println("=== CONSTANT COLLECTIONS ===");
    
    printf("Fundamental constants:\n");
    for (name, value) in FUNDAMENTAL_CONSTANTS {
        printf("  {}: {:.10}\n", name, value);
    }
    
    printf("\nSelected physical constants:\n");
    for (name, value, unit) in PHYSICAL_CONSTANTS[0..5] {
        printf("  {}: {:.3e} {}\n", name, value, unit);
    }
    
    printf("\nCommon fractions:\n");
    for (name, value) in COMMON_FRACTIONS[0..5] {
        printf("  {} = {:.6}\n", name, value);
    }
    
    println("");
    
    // ========================================================================
    // PRECISION AND LIMITS DEMONSTRATION
    // ========================================================================
    
    println("=== FLOATING POINT PRECISION ===");
    
    printf("Machine epsilon: {:.2e}\n", EPSILON);
    printf("Smallest positive: {:.2e}\n", MIN_POSITIVE);
    printf("Largest finite: {:.2e}\n", MAX);
    printf("Mantissa digits: {}\n", MANTISSA_DIGITS);
    printf("Decimal digits: {}\n", DIGITS);
    
    // Demonstrate precision limits
    facts tiny_diff = PI + EPSILON;
    printf("π + ε: {:.15}\n", tiny_diff);
    printf("Detectable difference? {}\n", tiny_diff != PI);
    
    println("");
    println("Mathematical constants demo completed successfully!");
}

fr fr Helper functions for the demo
func abs(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }
}

func pow(base: f64, exp: f64) -> f64 {
    base.powf(exp)
}

func sqrt(x: f64) -> f64 {
    x.sqrt()
}
