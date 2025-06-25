/// Mathematical constants

/// The constant π (pi) - ratio of circumference to diameter of a circle
pub const PI: f64 = std::f64::consts::PI;

/// The constant τ (tau) - equivalent to 2π
pub const TAU: f64 = std::f64::consts::TAU;

/// The natural logarithm base e (Euler's number)
pub const E: f64 = std::f64::consts::E;

/// π/2
pub const FRAC_PI_2: f64 = std::f64::consts::FRAC_PI_2;

/// π/3
pub const FRAC_PI_3: f64 = std::f64::consts::FRAC_PI_3;

/// π/4
pub const FRAC_PI_4: f64 = std::f64::consts::FRAC_PI_4;

/// π/6
pub const FRAC_PI_6: f64 = std::f64::consts::FRAC_PI_6;

/// π/8
pub const FRAC_PI_8: f64 = std::f64::consts::FRAC_PI_8;

/// 1/π
pub const FRAC_1_PI: f64 = std::f64::consts::FRAC_1_PI;

/// 2/π
pub const FRAC_2_PI: f64 = std::f64::consts::FRAC_2_PI;

/// 2/√π
pub const FRAC_2_SQRT_PI: f64 = std::f64::consts::FRAC_2_SQRT_PI;

/// √2
pub const SQRT_2: f64 = std::f64::consts::SQRT_2;

/// 1/√2
pub const FRAC_1_SQRT_2: f64 = std::f64::consts::FRAC_1_SQRT_2;

/// √3
pub const SQRT_3: f64 = 1.7320508075688772;

/// √5
pub const SQRT_5: f64 = 2.23606797749979;

/// √π
pub const SQRT_PI: f64 = 1.7724538509055159;

/// ln(2)
pub const LN_2: f64 = std::f64::consts::LN_2;

/// ln(10)
pub const LN_10: f64 = std::f64::consts::LN_10;

/// log₂(e)
pub const LOG2_E: f64 = std::f64::consts::LOG2_E;

/// log₂(10)
pub const LOG2_10: f64 = std::f64::consts::LOG2_10;

/// log₁₀(e)
pub const LOG10_E: f64 = std::f64::consts::LOG10_E;

/// log₁₀(2)
pub const LOG10_2: f64 = std::f64::consts::LOG10_2;

/// The golden ratio φ = (1 + √5) / 2
pub const PHI: f64 = 1.618033988749895;

/// The inverse golden ratio 1/φ = (√5 - 1) / 2
pub const INV_PHI: f64 = 0.6180339887498948;

/// Euler-Mascheroni constant γ
pub const EULER_GAMMA: f64 = 0.5772156649015329;

/// Catalan's constant
pub const CATALAN: f64 = 0.9159655941772190;

/// Apéry's constant ζ(3)
pub const APERY: f64 = 1.2020569031595943;

/// Conway's constant (growth rate of look-and-say sequence)
pub const CONWAY: f64 = 1.3035772690342964;

/// Khinchin's constant
pub const KHINCHIN: f64 = 2.6854520010653062;

/// Glaisher-Kinkelin constant
pub const GLAISHER: f64 = 1.2824271291006226;

/// Feigenbaum delta constant
pub const FEIGENBAUM_DELTA: f64 = 4.669201609102991;

/// Feigenbaum alpha constant
pub const FEIGENBAUM_ALPHA: f64 = 2.502907875095893;

/// Twin prime constant
pub const TWIN_PRIME: f64 = 0.6601618158468696;

/// Meissel-Mertens constant
pub const MEISSEL_MERTENS: f64 = 0.2614972128476428;

/// Brun's constant for twin primes
pub const BRUN_TWIN_PRIMES: f64 = 1.9021605823;

/// Champernowne constant (normal number)
pub const CHAMPERNOWNE: f64 = 0.12345678910111213;

/// Plastic number (real root of x³ = x + 1)
pub const PLASTIC: f64 = 1.3247179572447461;

/// Degrees to radians conversion factor (π/180)
pub const DEG_TO_RAD: f64 = PI / 180.0;

/// Radians to degrees conversion factor (180/π)
pub const RAD_TO_DEG: f64 = 180.0 / PI;

/// Speed of light in vacuum (m/s)
pub const SPEED_OF_LIGHT: f64 = 299792458.0;

/// Planck's constant (J⋅s)
pub const PLANCK: f64 = 6.62607015e-34;

/// Reduced Planck constant (ℏ = h/2π)
pub const HBAR: f64 = PLANCK / TAU;

/// Avogadro's number
pub const AVOGADRO: f64 = 6.02214076e23;

/// Boltzmann constant (J/K)
pub const BOLTZMANN: f64 = 1.380649e-23;

/// Gas constant (J/(mol⋅K))
pub const GAS_CONSTANT: f64 = 8.314462618;

/// Gravitational constant (m³/(kg⋅s²))
pub const GRAVITATIONAL: f64 = 6.67430e-11;

/// Elementary charge (C)
pub const ELEMENTARY_CHARGE: f64 = 1.602176634e-19;

/// Electron mass (kg)
pub const ELECTRON_MASS: f64 = 9.1093837015e-31;

/// Proton mass (kg)
pub const PROTON_MASS: f64 = 1.67262192369e-27;

/// Fine structure constant
pub const FINE_STRUCTURE: f64 = 7.2973525693e-3;

/// Machine epsilon for f64
pub const EPSILON: f64 = f64::EPSILON;

/// Smallest positive normalized f64
pub const MIN_POSITIVE: f64 = f64::MIN_POSITIVE;

/// Largest finite f64 value
pub const MAX: f64 = f64::MAX;

/// Smallest finite f64 value
pub const MIN: f64 = f64::MIN;

/// Positive infinity
pub const INFINITY: f64 = f64::INFINITY;

/// Negative infinity
pub const NEG_INFINITY: f64 = f64::NEG_INFINITY;

/// Not a Number
pub const NAN: f64 = f64::NAN;

/// Number of bits in the mantissa of f64
pub const MANTISSA_DIGITS: u32 = f64::MANTISSA_DIGITS;

/// Number of decimal digits that can be represented
pub const DIGITS: u32 = f64::DIGITS;

/// Maximum possible power of 2 exponent
pub const MAX_EXP: i32 = f64::MAX_EXP;

/// Minimum possible power of 2 exponent
pub const MIN_EXP: i32 = f64::MIN_EXP;

/// Maximum possible power of 10 exponent
pub const MAX_10_EXP: i32 = f64::MAX_10_EXP;

/// Minimum possible power of 10 exponent  
pub const MIN_10_EXP: i32 = f64::MIN_10_EXP;

/// Radix (base) of the internal representation
pub const RADIX: u32 = f64::RADIX;

// =============================================================================
// COMMON FRACTIONS
// =============================================================================

/// 1/3 - One third
pub const ONE_THIRD: f64 = 1.0 / 3.0;

/// 2/3 - Two thirds
pub const TWO_THIRDS: f64 = 2.0 / 3.0;

/// 1/6 - One sixth
pub const ONE_SIXTH: f64 = 1.0 / 6.0;

/// 5/6 - Five sixths
pub const FIVE_SIXTHS: f64 = 5.0 / 6.0;

/// 1/7 - One seventh
pub const ONE_SEVENTH: f64 = 1.0 / 7.0;

/// 1/9 - One ninth
pub const ONE_NINTH: f64 = 1.0 / 9.0;

/// 1/12 - One twelfth
pub const ONE_TWELFTH: f64 = 1.0 / 12.0;

/// 3/4 - Three quarters
pub const THREE_QUARTERS: f64 = 3.0 / 4.0;

/// 5/8 - Five eighths
pub const FIVE_EIGHTHS: f64 = 5.0 / 8.0;

/// 7/8 - Seven eighths
pub const SEVEN_EIGHTHS: f64 = 7.0 / 8.0;

// =============================================================================
// UNIT CONVERSION CONSTANTS
// =============================================================================

/// Inches to centimeters
pub const INCH_TO_CM: f64 = 2.54;

/// Centimeters to inches
pub const CM_TO_INCH: f64 = 1.0 / INCH_TO_CM;

/// Feet to meters
pub const FOOT_TO_METER: f64 = 0.3048;

/// Meters to feet
pub const METER_TO_FOOT: f64 = 1.0 / FOOT_TO_METER;

/// Miles to kilometers
pub const MILE_TO_KM: f64 = 1.609344;

/// Kilometers to miles
pub const KM_TO_MILE: f64 = 1.0 / MILE_TO_KM;

/// Pounds to kilograms
pub const POUND_TO_KG: f64 = 0.45359237;

/// Kilograms to pounds
pub const KG_TO_POUND: f64 = 1.0 / POUND_TO_KG;

/// Fahrenheit to Celsius conversion offset
pub const FAHRENHEIT_OFFSET: f64 = 32.0;

/// Fahrenheit to Celsius conversion factor
pub const FAHRENHEIT_SCALE: f64 = 5.0 / 9.0;

/// Celsius to Fahrenheit conversion factor
pub const CELSIUS_SCALE: f64 = 9.0 / 5.0;

/// Atmospheric pressure at sea level (Pa)
pub const ATM_PRESSURE: f64 = 101325.0;

/// Standard acceleration due to gravity (m/s²)
pub const STANDARD_GRAVITY: f64 = 9.80665;

// =============================================================================
// ORGANIZED CONSTANT COLLECTIONS
// =============================================================================

/// Array of fundamental mathematical constants
pub const FUNDAMENTAL_CONSTANTS: [(&str, f64); 6] = [
];

/// Array of physical constants with their units
pub const PHYSICAL_CONSTANTS: [(&str, f64, &str); 12] = [
    ("SPEED_OF_LIGHT", SPEED_OF_LIGHT, "m/s"),
    ("BOLTZMANN", BOLTZMANN, "J/K"),
    ("GAS_CONSTANT", GAS_CONSTANT, "J/(mol⋅K)"),
    ("GRAVITATIONAL", GRAVITATIONAL, "m³/(kg⋅s²)"),
    ("STANDARD_GRAVITY", STANDARD_GRAVITY, "m/s²"),
];

/// Array of common fractions
pub const COMMON_FRACTIONS: [(&str, f64); 10] = [
    ("1/3", ONE_THIRD),
    ("2/3", TWO_THIRDS),
    ("1/6", ONE_SIXTH),
    ("5/6", FIVE_SIXTHS),
    ("1/7", ONE_SEVENTH),
    ("1/9", ONE_NINTH),
    ("1/12", ONE_TWELFTH),
    ("3/4", THREE_QUARTERS),
    ("5/8", FIVE_EIGHTHS),
    ("7/8", SEVEN_EIGHTHS),
];

// =============================================================================
// UTILITY FUNCTIONS
// =============================================================================

/// Check if a value is approximately equal to a mathematical constant
/// 
/// # Arguments
/// * `value` - The value to check
/// * `constant` - The constant to compare against
/// * `tolerance` - The tolerance for comparison (default: EPSILON * 10.0)
/// 
/// # Returns
/// True if the value is within tolerance of the constant
pub fn is_close_to_constant(value: f64, constant: f64, tolerance: Option<f64>) -> bool {
    let tol = tolerance.unwrap_or(EPSILON * 10.0);
    (value - constant).abs() < tol
/// Check if a value is approximately π
pub fn is_approximately_pi(value: f64) -> bool {
    is_close_to_constant(value, PI, Some(1e-4))
/// Check if a value is approximately e
pub fn is_approximately_e(value: f64) -> bool {
    is_close_to_constant(value, E, Some(1e-4))
/// Check if a value is approximately the golden ratio φ
pub fn is_approximately_phi(value: f64) -> bool {
    is_close_to_constant(value, PHI, Some(1e-4))
/// Find the closest mathematical constant to a given value
/// 
/// # Arguments
/// * `value` - The value to find the closest constant for
/// 
/// # Returns
/// Option containing (name, constant_value, difference) of the closest constant
pub fn find_closest_constant(value: f64) -> Option<(&'static str, f64, f64)> {
    let mut closest: Option<(&str, f64, f64)> = None;
    let mut min_diff = f64::INFINITY;
    
    for &(name, constant) in &FUNDAMENTAL_CONSTANTS {
        let diff = (value - constant).abs();
        if diff < min_diff {
            min_diff = diff;
            closest = Some((name, constant, diff));
        }
    }
    
    closest
// Note: angle conversion functions are available in the trigonometry module
// as degrees_to_radians() and radians_to_degrees()

/// Convert Fahrenheit to Celsius
pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - FAHRENHEIT_OFFSET) * FAHRENHEIT_SCALE
/// Convert Celsius to Fahrenheit
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * CELSIUS_SCALE + FAHRENHEIT_OFFSET
/// Convert inches to centimeters
pub fn inches_to_cm(inches: f64) -> f64 {
    inches * INCH_TO_CM
/// Convert centimeters to inches
pub fn cm_to_inches(cm: f64) -> f64 {
    cm * CM_TO_INCH
/// Convert miles to kilometers
pub fn miles_to_km(miles: f64) -> f64 {
    miles * MILE_TO_KM
/// Convert kilometers to miles
pub fn km_to_miles(km: f64) -> f64 {
    km * KM_TO_MILE
/// Convert pounds to kilograms
pub fn pounds_to_kg(pounds: f64) -> f64 {
    pounds * POUND_TO_KG
/// Convert kilograms to pounds
pub fn kg_to_pounds(kg: f64) -> f64 {
    kg * KG_TO_POUND
/// Get all fundamental constants as a formatted string
pub fn list_fundamental_constants() -> String {
    let mut result = String::from("Fundamental Mathematical Constants:\n");
    for &(name, value) in &FUNDAMENTAL_CONSTANTS {
        result.push_str(&format!("  {} = {:.15}\n", name, value));
    }
    result
/// Get all physical constants as a formatted string with units
pub fn list_physical_constants() -> String {
    let mut result = String::from("Physical Constants:\n");
    for &(name, value, unit) in &PHYSICAL_CONSTANTS {
        result.push_str(&format!("  {} = {:.6e} {}\n", name, value, unit));
    }
    result
/// Validate that a calculated value matches a known constant within tolerance
/// 
/// # Arguments
/// * `calculated` - The calculated value
/// * `expected_constant` - The expected constant
/// * `tolerance` - The acceptable tolerance
/// * `description` - Description of what was calculated
/// 
/// # Returns
/// Result indicating if the validation passed
pub fn validate_constant_calculation(
    description: &str
) -> Result<(), String> {
    let diff = (calculated - expected_constant).abs();
    if diff <= tolerance {
        Ok(())
    } else {
        Err(format!(
            description, calculated, expected_constant, diff, tolerance
        ))
    }
}
