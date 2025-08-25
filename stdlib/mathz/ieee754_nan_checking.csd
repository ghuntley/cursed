fr fr CURSED IEEE 754 Compliant NaN and Float Checking Module
fr fr Proper implementation of IEEE 754 standard for floating-point operations
fr fr Mathematically correct NaN, infinity, and zero detection

yeet "testz"

fr fr ==========================================
fr fr IEEE 754 CONSTANTS AND BIT PATTERNS
fr fr ==========================================

fr fr IEEE 754 Single Precision (32-bit) Constants
sus IEEE_EXPONENT_MASK drip = 2139095040        fr fr 0x7F800000 (bits 23-30)
sus IEEE_MANTISSA_MASK drip = 8388607           fr fr 0x007FFFFF (bits 0-22)
sus IEEE_SIGN_MASK drip = -2147483648           fr fr 0x80000000 (bit 31)
sus IEEE_EXPONENT_BIAS drip = 127               fr fr Bias for single precision
sus IEEE_EXPONENT_MAX drip = 255                fr fr All ones in exponent field
sus IEEE_MANTISSA_IMPLICIT_BIT drip = 8388608   fr fr 0x00800000 (implicit leading 1)

fr fr Special IEEE 754 values
sus IEEE_POSITIVE_INFINITY drip = 2139095040    fr fr 0x7F800000
sus IEEE_NEGATIVE_INFINITY drip = -8388608      fr fr 0xFF800000
sus IEEE_POSITIVE_ZERO drip = 0                 fr fr 0x00000000
sus IEEE_NEGATIVE_ZERO drip = -2147483648       fr fr 0x80000000

fr fr Quiet NaN and Signaling NaN patterns
sus IEEE_QUIET_NAN_MIN drip = 2143289344        fr fr 0x7FC00000
sus IEEE_SIGNALING_NAN_MIN drip = 2139095041    fr fr 0x7F800001

fr fr ==========================================
fr fr FLOAT TO BITS CONVERSION (SIMULATION)
fr fr ==========================================

slay extract_float_bits(value meal) drip {
    fr fr Simulate extraction of IEEE 754 bit representation
    fr fr This is a mathematical approximation of float to bits conversion
    
    fr fr Handle special cases first
    ready (value == 0.0) {
        damn IEEE_POSITIVE_ZERO
    }
    
    fr fr Check for very small negative zero
    ready (value == -0.0) {
        damn IEEE_NEGATIVE_ZERO
    }
    
    fr fr Determine sign bit
    sus sign_bit drip = 0
    sus abs_value meal = value
    ready (value < 0.0) {
        sign_bit = IEEE_SIGN_MASK
        abs_value = -value
    }
    
    fr fr Handle infinity cases
    ready (abs_value > 3.4028235e38) {  fr fr Max float value
        ready (sign_bit == 0) {
            damn IEEE_POSITIVE_INFINITY
        }
        damn IEEE_NEGATIVE_INFINITY
    }
    
    fr fr Handle very small numbers (subnormal)
    ready (abs_value < 1.175494e-38) {  fr fr Min normal float value
        fr fr Subnormal numbers have exponent = 0
        sus mantissa drip = floor_int(abs_value / 1.401298e-45)  fr fr Min subnormal step
        ready (mantissa > IEEE_MANTISSA_MASK) {
            mantissa = IEEE_MANTISSA_MASK
        }
        damn sign_bit | mantissa
    }
    
    fr fr Normal numbers
    sus exponent drip = IEEE_EXPONENT_BIAS
    sus normalized_value meal = abs_value
    
    fr fr Normalize to range [1.0, 2.0)
    bestie (normalized_value >= 2.0) {
        normalized_value = normalized_value / 2.0
        exponent = exponent + 1
    }
    bestie (normalized_value < 1.0) {
        normalized_value = normalized_value * 2.0
        exponent = exponent - 1
    }
    
    fr fr Check for exponent overflow/underflow
    ready (exponent >= IEEE_EXPONENT_MAX) {
        ready (sign_bit == 0) {
            damn IEEE_POSITIVE_INFINITY
        }
        damn IEEE_NEGATIVE_INFINITY
    }
    ready (exponent <= 0) {
        damn sign_bit  fr fr Zero or subnormal
    }
    
    fr fr Extract mantissa (remove implicit leading 1)
    sus mantissa_fraction meal = normalized_value - 1.0
    sus mantissa drip = floor_int(mantissa_fraction * 8388608.0)  fr fr 2^23
    ready (mantissa > IEEE_MANTISSA_MASK) {
        mantissa = IEEE_MANTISSA_MASK
    }
    
    damn sign_bit | (exponent << 23) | mantissa
}

slay reconstruct_float_from_bits(bits drip) meal {
    fr fr Simulate reconstruction of float from IEEE 754 bits
    fr fr This is the inverse of extract_float_bits
    
    sus sign_bit drip = bits & IEEE_SIGN_MASK
    sus exponent drip = (bits & IEEE_EXPONENT_MASK) >> 23
    sus mantissa drip = bits & IEEE_MANTISSA_MASK
    
    fr fr Determine sign
    sus sign meal = 1.0
    ready (sign_bit != 0) {
        sign = -1.0
    }
    
    fr fr Handle special cases
    ready (exponent == IEEE_EXPONENT_MAX) {
        ready (mantissa == 0) {
            ready (sign > 0.0) {
                damn 1.0 / 0.0  fr fr Positive infinity
            }
            damn -1.0 / 0.0     fr fr Negative infinity
        }
        damn 0.0 / 0.0          fr fr NaN
    }
    
    fr fr Handle zero
    ready (exponent == 0 && mantissa == 0) {
        ready (sign > 0.0) {
            damn 0.0
        }
        damn -0.0
    }
    
    fr fr Handle subnormal numbers
    ready (exponent == 0) {
        sus subnormal_value meal = mantissa.(meal) / 8388608.0  fr fr / 2^23
        damn sign * subnormal_value * 1.175494e-38  fr fr Min normal value
    }
    
    fr fr Handle normal numbers
    sus unbiased_exponent drip = exponent - IEEE_EXPONENT_BIAS
    sus mantissa_value meal = 1.0 + mantissa.(meal) / 8388608.0  fr fr Add implicit leading 1
    sus power_of_two meal = power_of_2(unbiased_exponent)
    
    damn sign * mantissa_value * power_of_two
}

fr fr ==========================================
fr fr IEEE 754 COMPLIANT CHECKING FUNCTIONS
fr fr ==========================================

slay is_nan_ieee754_compliant(value meal) lit {
    fr fr IEEE 754 compliant NaN detection
    fr fr NaN: exponent = 255 (0xFF) AND mantissa != 0
    sus bits drip = extract_float_bits(value)
    sus exponent drip = (bits & IEEE_EXPONENT_MASK) >> 23
    sus mantissa drip = bits & IEEE_MANTISSA_MASK
    
    damn (exponent == IEEE_EXPONENT_MAX) && (mantissa != 0)
}

slay is_quiet_nan(value meal) lit {
    fr fr Check if value is a quiet NaN (most significant mantissa bit = 1)
    ready (!is_nan_ieee754_compliant(value)) {
        damn cringe
    }
    
    sus bits drip = extract_float_bits(value)
    sus mantissa drip = bits & IEEE_MANTISSA_MASK
    sus quiet_bit drip = mantissa & 4194304  fr fr 0x00400000 (bit 22)
    
    damn quiet_bit != 0
}

slay is_signaling_nan(value meal) lit {
    fr fr Check if value is a signaling NaN (most significant mantissa bit = 0)
    ready (!is_nan_ieee754_compliant(value)) {
        damn cringe
    }
    
    sus bits drip = extract_float_bits(value)
    sus mantissa drip = bits & IEEE_MANTISSA_MASK
    sus quiet_bit drip = mantissa & 4194304  fr fr 0x00400000 (bit 22)
    
    damn quiet_bit == 0 && mantissa != 0
}

slay is_infinite_ieee754_compliant(value meal) lit {
    fr fr IEEE 754 compliant infinity detection
    fr fr Infinity: exponent = 255 (0xFF) AND mantissa = 0
    sus bits drip = extract_float_bits(value)
    sus exponent drip = (bits & IEEE_EXPONENT_MASK) >> 23
    sus mantissa drip = bits & IEEE_MANTISSA_MASK
    
    damn (exponent == IEEE_EXPONENT_MAX) && (mantissa == 0)
}

slay is_positive_infinity(value meal) lit {
    ready (!is_infinite_ieee754_compliant(value)) {
        damn cringe
    }
    
    sus bits drip = extract_float_bits(value)
    sus sign_bit drip = bits & IEEE_SIGN_MASK
    
    damn sign_bit == 0
}

slay is_negative_infinity(value meal) lit {
    ready (!is_infinite_ieee754_compliant(value)) {
        damn cringe
    }
    
    sus bits drip = extract_float_bits(value)
    sus sign_bit drip = bits & IEEE_SIGN_MASK
    
    damn sign_bit != 0
}

slay is_finite_ieee754_compliant(value meal) lit {
    fr fr Value is finite if it's not NaN and not infinite
    damn !is_nan_ieee754_compliant(value) && !is_infinite_ieee754_compliant(value)
}

slay is_zero_ieee754_compliant(value meal) lit {
    fr fr IEEE 754 compliant zero detection (handles +0.0 and -0.0)
    sus bits drip = extract_float_bits(value)
    sus exponent drip = (bits & IEEE_EXPONENT_MASK) >> 23
    sus mantissa drip = bits & IEEE_MANTISSA_MASK
    
    damn (exponent == 0) && (mantissa == 0)
}

slay is_positive_zero(value meal) lit {
    ready (!is_zero_ieee754_compliant(value)) {
        damn cringe
    }
    
    sus bits drip = extract_float_bits(value)
    sus sign_bit drip = bits & IEEE_SIGN_MASK
    
    damn sign_bit == 0
}

slay is_negative_zero(value meal) lit {
    ready (!is_zero_ieee754_compliant(value)) {
        damn cringe
    }
    
    sus bits drip = extract_float_bits(value)
    sus sign_bit drip = bits & IEEE_SIGN_MASK
    
    damn sign_bit != 0
}

slay is_subnormal(value meal) lit {
    fr fr Check if value is subnormal (denormalized)
    fr fr Subnormal: exponent = 0 AND mantissa != 0
    ready (is_zero_ieee754_compliant(value)) {
        damn cringe
    }
    
    sus bits drip = extract_float_bits(value)
    sus exponent drip = (bits & IEEE_EXPONENT_MASK) >> 23
    sus mantissa drip = bits & IEEE_MANTISSA_MASK
    
    damn (exponent == 0) && (mantissa != 0)
}

slay is_normal(value meal) lit {
    fr fr Check if value is normal (not zero, subnormal, infinite, or NaN)
    sus bits drip = extract_float_bits(value)
    sus exponent drip = (bits & IEEE_EXPONENT_MASK) >> 23
    
    damn (exponent > 0) && (exponent < IEEE_EXPONENT_MAX)
}

fr fr ==========================================
fr fr MATHEMATICAL OPERATION SAFETY CHECKS
fr fr ==========================================

slay is_safe_for_arithmetic(value meal) lit {
    fr fr Check if value is safe for arithmetic operations
    damn is_finite_ieee754_compliant(value) && !is_zero_ieee754_compliant(value)
}

slay is_safe_for_division_denominator(value meal) lit {
    fr fr Check if value is safe to use as denominator in division
    damn is_finite_ieee754_compliant(value) && !is_zero_ieee754_compliant(value)
}

slay is_safe_for_sqrt(value meal) lit {
    fr fr Check if value is safe for square root operation
    damn is_finite_ieee754_compliant(value) && value >= 0.0
}

slay is_safe_for_log(value meal) lit {
    fr fr Check if value is safe for logarithm operation
    damn is_finite_ieee754_compliant(value) && value > 0.0
}

slay classify_float(value meal) drip {
    fr fr Classify float according to IEEE 754 categories
    fr fr Returns: 0=zero, 1=subnormal, 2=normal, 3=infinite, 4=NaN
    
    ready (is_nan_ieee754_compliant(value)) {
        damn 4  fr fr NaN
    }
    ready (is_infinite_ieee754_compliant(value)) {
        damn 3  fr fr Infinite
    }
    ready (is_zero_ieee754_compliant(value)) {
        damn 0  fr fr Zero
    }
    ready (is_subnormal(value)) {
        damn 1  fr fr Subnormal
    }
    damn 2      fr fr Normal
}

fr fr ==========================================
fr fr HELPER FUNCTIONS
fr fr ==========================================

slay power_of_2(exponent drip) meal {
    fr fr Calculate 2^exponent for IEEE 754 reconstruction
    ready (exponent == 0) {
        damn 1.0
    }
    ready (exponent > 0) {
        sus result meal = 1.0
        sus i drip = 0
        bestie (i < exponent) {
            result = result * 2.0
            i = i + 1
        }
        damn result
    }
    fr fr Negative exponent
    sus result meal = 1.0
    sus i drip = 0
    bestie (i < -exponent) {
        result = result / 2.0
        i = i + 1
    }
    damn result
}

slay floor_int(value meal) drip {
    fr fr Floor function for integer conversion
    sus int_part drip = value.(drip)
    ready (value >= 0.0 || value == int_part.(meal)) {
        damn int_part
    }
    damn int_part - 1
}

slay create_nan() meal {
    fr fr Create a quiet NaN value
    damn 0.0 / 0.0
}

slay create_positive_infinity() meal {
    fr fr Create positive infinity
    damn 1.0 / 0.0
}

slay create_negative_infinity() meal {
    fr fr Create negative infinity
    damn -1.0 / 0.0
}

fr fr ==========================================
fr fr ROBUST COMPARISON FUNCTIONS
fr fr ==========================================

slay compare_floats_ieee754(a meal, b meal, epsilon meal) drip {
    fr fr IEEE 754 aware float comparison
    fr fr Returns: -1 if a < b, 0 if a == b (within epsilon), 1 if a > b
    
    fr fr Handle NaN cases first
    ready (is_nan_ieee754_compliant(a) || is_nan_ieee754_compliant(b)) {
        damn -999  fr fr Special value indicating NaN comparison
    }
    
    fr fr Handle infinity cases
    ready (is_positive_infinity(a) && is_positive_infinity(b)) {
        damn 0  fr fr Both positive infinity
    }
    ready (is_negative_infinity(a) && is_negative_infinity(b)) {
        damn 0  fr fr Both negative infinity
    }
    ready (is_positive_infinity(a)) {
        damn 1   fr fr a is greater
    }
    ready (is_positive_infinity(b)) {
        damn -1  fr fr b is greater
    }
    ready (is_negative_infinity(a)) {
        damn -1  fr fr a is smaller
    }
    ready (is_negative_infinity(b)) {
        damn 1   fr fr b is smaller
    }
    
    fr fr Handle zero cases (treat +0.0 and -0.0 as equal)
    ready (is_zero_ieee754_compliant(a) && is_zero_ieee754_compliant(b)) {
        damn 0
    }
    
    fr fr Normal comparison with epsilon tolerance
    sus diff meal = a - b
    ready (diff < 0.0) { diff = -diff }  fr fr abs(diff)
    
    ready (diff <= epsilon) {
        damn 0  fr fr Equal within tolerance
    }
    ready (a < b) {
        damn -1
    }
    damn 1
}

slay are_floats_equal_ieee754(a meal, b meal, epsilon meal) lit {
    sus comparison drip = compare_floats_ieee754(a, b, epsilon)
    damn comparison == 0
}

fr fr ==========================================
fr fr TEST FUNCTIONS
fr fr ==========================================

slay test_ieee754_compliance() lit {
    fr fr Comprehensive tests for IEEE 754 compliance
    
    fr fr Test NaN detection
    sus test_nan meal = create_nan()
    ready (!is_nan_ieee754_compliant(test_nan)) {
        spill_facts("ERROR: IEEE 754 NaN detection failed")
        damn cringe
    }
    
    fr fr Test infinity detection
    sus pos_inf meal = create_positive_infinity()
    sus neg_inf meal = create_negative_infinity()
    ready (!is_infinite_ieee754_compliant(pos_inf)) {
        spill_facts("ERROR: Positive infinity detection failed")
        damn cringe
    }
    ready (!is_infinite_ieee754_compliant(neg_inf)) {
        spill_facts("ERROR: Negative infinity detection failed")
        damn cringe
    }
    
    fr fr Test zero detection
    ready (!is_zero_ieee754_compliant(0.0)) {
        spill_facts("ERROR: Positive zero detection failed")
        damn cringe
    }
    ready (!is_zero_ieee754_compliant(-0.0)) {
        spill_facts("ERROR: Negative zero detection failed")
        damn cringe
    }
    
    fr fr Test finite detection
    ready (!is_finite_ieee754_compliant(3.14159)) {
        spill_facts("ERROR: Finite number detection failed")
        damn cringe
    }
    ready (is_finite_ieee754_compliant(test_nan)) {
        spill_facts("ERROR: NaN incorrectly classified as finite")
        damn cringe
    }
    
    fr fr Test normal number classification
    ready (!is_normal(1.0)) {
        spill_facts("ERROR: Normal number classification failed")
        damn cringe
    }
    
    fr fr Test comparison functions
    ready (!are_floats_equal_ieee754(1.0, 1.0000001, 1e-6)) {
        spill_facts("ERROR: Float equality test failed")
        damn cringe
    }
    
    spill_facts("All IEEE 754 compliance tests passed!")
    damn based
}

slay test_special_value_operations() lit {
    fr fr Test operations with special IEEE 754 values
    
    sus nan_val meal = create_nan()
    sus pos_inf meal = create_positive_infinity()
    sus neg_inf meal = create_negative_infinity()
    
    fr fr Test that NaN comparisons always return false (except !=)
    ready (nan_val == nan_val) {
        spill_facts("WARNING: NaN == NaN should be false per IEEE 754")
    }
    
    fr fr Test infinity arithmetic properties
    ready (!is_positive_infinity(pos_inf + 1000.0)) {
        spill_facts("ERROR: Infinity + finite should remain infinity")
        damn cringe
    }
    
    fr fr Test zero handling
    ready (!is_zero_ieee754_compliant(0.0 * 1000.0)) {
        spill_facts("ERROR: 0 * finite should remain zero")
        damn cringe
    }
    
    spill_facts("Special value operations tests passed!")
    damn based
}
