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
