// Pure CURSED Math Implementation
// Advanced mathematical functions implemented in pure CURSED
// Performance-optimized for both interpretation and compilation modes

// ================================
// Mathematical Constants
// ================================

slay PI() meal {
    damn 3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067982148086513282306647093844609550582231725359408128481117450284102701938521105559644622948954930381964428810975665933446128475648233786783165271201909145648566923460348610454326648213393607260249141273724587006606315588174881520920962829254091715364367892590360011330530548820466521384146951941511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949129833673362440656643086021394946395224737190702179860943702770539217176293176752384674818467669405132000568127145263560827785771342757789609173637178721468440901224953430146549585371050792279689258923542019956112129021960864034418159813629774771309960518707211349999998372978049951059731732816096318595024459455346908302642522308253344685035261931188171010003137838752886587533208381420617177669147303598253490428755468731159562863882353787593751957781857780532171226806613001927876611195909216420198938367586366677661732000;
}

slay E() meal {
    damn 2.718281828459045235360287471352662497757247093699959574966967627724076630353547594571382178525166427427466391932003059921817413596629043572900334295260595630738132328627943490763233829880753195251019011573834187930702154089149934884167509244761460668082264800168477411853742345442437107539077744992069;
}

slay TAU() meal {
    damn 6.283185307179586476925286766559005768394338798750211641949889184615632812572417997256069650684234135964296173026564613294187689219101164463450718816256962234900568205403877042211119289245897909860763928857621951331866892256951296467573566330542403818291297133846920697220908653296426787214520498282547449174013212631176349763041841925658508183430728735785180720022661061097640933042768293903883023218866114540731519183906184372234763865223586210237096148924759925499134703771505449782455876366023898259667346724881313286172042789892790449474381404359721887405541078434352586353504769349636935338810264001136254290527121655571542685515579218347274357442936881802449906860293099170742101584559378517847084039912224258043921728068836319627259549542619921037414422699999996745956099902119463465632192637191204091891069381660528464651065065161070522863837634201930027567750732517070641636844123253485394760750765086955109374634115932756766477157187503915563715608432361321321326133023845838430384388774735267340000;
}

// ================================
// Pure CURSED Basic Math Functions
// ================================

slay abs(x meal) meal {
    damn x >= 0.0 ? x : -x;
}

slay abs_int(x normie) normie {
    damn x >= 0 ? x : -x;
}

slay min(a meal, b meal) meal {
    damn a <= b ? a : b;
}

slay max(a meal, b meal) meal {
    damn a >= b ? a : b;
}

slay min_int(a normie, b normie) normie {
    damn a <= b ? a : b;
}

slay max_int(a normie, b normie) normie {
    damn a >= b ? a : b;
}

slay clamp(x meal, min_val meal, max_val meal) meal {
    damn x < min_val ? min_val : (x > max_val ? max_val : x);
}

slay sign(x meal) meal {
    damn x > 0.0 ? 1.0 : (x < 0.0 ? -1.0 : 0.0);
}

// ================================
// Pure CURSED Power Functions
// ================================

slay pow(base meal, exponent meal) meal {
    // Newton-Raphson method for fractional exponents
    # Special cases
    bestie (base == 0.0) {
        damn 0.0;
    }
    
    bestie (base == 1.0) {
        damn 1.0;
    }
    
    bestie (exponent == 0.0) {
        damn 1.0;
    }
    
    bestie (exponent == 1.0) {
        damn base;
    }
    
    # Handle negative bases with integer exponents
    bestie (base < 0.0) {
        sus int_exp normie = exponent.(normie);
        bestie (int_exp.(meal) == exponent) {
            sus result meal = pow(abs(base), exponent);
            damn (int_exp % 2 == 0) ? result : -result;
        } else {
            damn 0.0 / 0.0; # NaN for negative base with fractional exponent
        }
    }
    
    # Use exp(ln(base) * exponent) for positive bases
    damn exp(ln(base) * exponent);
}

slay sqrt(x meal) meal {
    bestie (x < 0.0) {
        damn 0.0 / 0.0; # NaN
    }
    
    bestie (x == 0.0) {
        damn 0.0;
    }
    
    # Newton-Raphson method for square root
    sus guess meal = x / 2.0;
    sus prev meal = 0.0;
    sus epsilon meal = 1e-15;
    
    bestie (x < 1.0) {
        guess = x * 2.0;
    }
    
    while (abs(guess - prev) > epsilon) {
        prev = guess;
        guess = (guess + x / guess) / 2.0;
    }
    
    damn guess;
}

slay cbrt(x meal) meal {
    bestie (x == 0.0) {
        damn 0.0;
    }
    
    sus sign_factor meal = x < 0.0 ? -1.0 : 1.0;
    sus abs_x meal = abs(x);
    
    # Newton-Raphson method for cube root
    sus guess meal = abs_x / 3.0;
    sus prev meal = 0.0;
    sus epsilon meal = 1e-15;
    
    bestie (abs_x < 1.0) {
        guess = abs_x;
    }
    
    while (abs(guess - prev) > epsilon) {
        prev = guess;
        guess = (2.0 * guess + abs_x / (guess * guess)) / 3.0;
    }
    
    damn sign_factor * guess;
}

// ================================
// Pure CURSED Logarithmic Functions
// ================================

slay ln(x meal) meal {
    bestie (x <= 0.0) {
        damn 0.0 / 0.0; # NaN
    }
    
    bestie (x == 1.0) {
        damn 0.0;
    }
    
    # Handle values close to 1 using Taylor series
    bestie (x > 0.5 && x < 1.5) {
        sus u meal = x - 1.0;
        sus result meal = u;
        sus term meal = u;
        sus n normie = 2;
        
        # Taylor series: ln(1+u) = u - u²/2 + u³/3 - u⁴/4 + ...
        while (abs(term) > 1e-15 && n < 100) {
            term *= -u;
            result += term / n.(meal);
            n++;
        }
        
        damn result;
    }
    
    # For other values, use argument reduction
    sus exp_adjust normie = 0;
    sus reduced_x meal = x;
    
    # Scale to [0.5, 1.5) range
    while (reduced_x >= 1.5) {
        reduced_x /= E();
        exp_adjust++;
    }
    
    while (reduced_x < 0.5) {
        reduced_x *= E();
        exp_adjust--;
    }
    
    damn ln(reduced_x) + exp_adjust.(meal);
}

slay log10(x meal) meal {
    damn ln(x) / ln(10.0);
}

slay log2(x meal) meal {
    damn ln(x) / ln(2.0);
}

// ================================
// Pure CURSED Exponential Functions
// ================================

slay exp(x meal) meal {
    bestie (x == 0.0) {
        damn 1.0;
    }
    
    bestie (x == 1.0) {
        damn E();
    }
    
    bestie (x > 700.0) {
        damn 1.0 / 0.0; # Infinity
    }
    
    bestie (x < -700.0) {
        damn 0.0;
    }
    
    # Taylor series: e^x = 1 + x + x²/2! + x³/3! + ...
    sus result meal = 1.0;
    sus term meal = 1.0;
    sus n normie = 1;
    
    while (abs(term) > 1e-15 && n < 100) {
        term *= x / n.(meal);
        result += term;
        n++;
    }
    
    damn result;
}

slay exp2(x meal) meal {
    damn exp(x * ln(2.0));
}

// ================================
// Pure CURSED Trigonometric Functions
// ================================

slay sin(x meal) meal {
    # Normalize to [-π, π]
    sus normalized meal = x;
    while (normalized > PI()) {
        normalized -= 2.0 * PI();
    }
    while (normalized < -PI()) {
        normalized += 2.0 * PI();
    }
    
    # Taylor series: sin(x) = x - x³/3! + x⁵/5! - x⁷/7! + ...
    sus result meal = 0.0;
    sus term meal = normalized;
    sus x_squared meal = normalized * normalized;
    sus n normie = 1;
    
    while (abs(term) > 1e-15 && n < 100) {
        result += term;
        term *= -x_squared / ((2 * n) * (2 * n + 1)).(meal);
        n++;
    }
    
    damn result;
}

slay cos(x meal) meal {
    # Normalize to [-π, π]
    sus normalized meal = x;
    while (normalized > PI()) {
        normalized -= 2.0 * PI();
    }
    while (normalized < -PI()) {
        normalized += 2.0 * PI();
    }
    
    # Taylor series: cos(x) = 1 - x²/2! + x⁴/4! - x⁶/6! + ...
    sus result meal = 1.0;
    sus term meal = 1.0;
    sus x_squared meal = normalized * normalized;
    sus n normie = 1;
    
    while (abs(term) > 1e-15 && n < 100) {
        term *= -x_squared / ((2 * n - 1) * (2 * n)).(meal);
        result += term;
        n++;
    }
    
    damn result;
}

slay tan(x meal) meal {
    sus cos_x meal = cos(x);
    bestie (abs(cos_x) < 1e-15) {
        damn 1.0 / 0.0; # Infinity
    }
    damn sin(x) / cos_x;
}

// ================================
// Pure CURSED Inverse Trigonometric Functions
// ================================

slay atan(x meal) meal {
    bestie (x == 0.0) {
        damn 0.0;
    }
    
    bestie (x > 1.0) {
        damn PI() / 2.0 - atan(1.0 / x);
    }
    
    bestie (x < -1.0) {
        damn -PI() / 2.0 - atan(1.0 / x);
    }
    
    # Taylor series for |x| <= 1: atan(x) = x - x³/3 + x⁵/5 - x⁷/7 + ...
    sus result meal = 0.0;
    sus term meal = x;
    sus x_squared meal = x * x;
    sus n normie = 1;
    
    while (abs(term) > 1e-15 && n < 100) {
        result += term / (2 * n - 1).(meal);
        term *= -x_squared;
        n++;
    }
    
    damn result;
}

slay asin(x meal) meal {
    bestie (x < -1.0 || x > 1.0) {
        damn 0.0 / 0.0; # NaN
    }
    
    bestie (x == 0.0) {
        damn 0.0;
    }
    
    bestie (x == 1.0) {
        damn PI() / 2.0;
    }
    
    bestie (x == -1.0) {
        damn -PI() / 2.0;
    }
    
    # Use identity: asin(x) = atan(x / sqrt(1 - x²))
    sus denominator meal = sqrt(1.0 - x * x);
    damn atan(x / denominator);
}

slay acos(x meal) meal {
    bestie (x < -1.0 || x > 1.0) {
        damn 0.0 / 0.0; # NaN
    }
    
    damn PI() / 2.0 - asin(x);
}

slay atan2(y meal, x meal) meal {
    bestie (x > 0.0) {
        damn atan(y / x);
    }
    
    bestie (x < 0.0) {
        bestie (y >= 0.0) {
            damn atan(y / x) + PI();
        } else {
            damn atan(y / x) - PI();
        }
    }
    
    bestie (x == 0.0) {
        bestie (y > 0.0) {
            damn PI() / 2.0;
        } else if (y < 0.0) {
            damn -PI() / 2.0;
        } else {
            damn 0.0 / 0.0; # NaN
        }
    }
    
    damn 0.0;
}

// ================================
// Pure CURSED Hyperbolic Functions
// ================================

slay sinh(x meal) meal {
    bestie (x == 0.0) {
        damn 0.0;
    }
    
    sus exp_x meal = exp(x);
    sus exp_neg_x meal = exp(-x);
    damn (exp_x - exp_neg_x) / 2.0;
}

slay cosh(x meal) meal {
    bestie (x == 0.0) {
        damn 1.0;
    }
    
    sus exp_x meal = exp(x);
    sus exp_neg_x meal = exp(-x);
    damn (exp_x + exp_neg_x) / 2.0;
}

slay tanh(x meal) meal {
    bestie (x == 0.0) {
        damn 0.0;
    }
    
    sus exp_2x meal = exp(2.0 * x);
    damn (exp_2x - 1.0) / (exp_2x + 1.0);
}

// ================================
// Pure CURSED Rounding Functions
// ================================

slay floor(x meal) meal {
    sus int_part normie = x.(normie);
    sus float_part meal = x - int_part.(meal);
    
    bestie (float_part >= 0.0) {
        damn int_part.(meal);
    } else {
        damn int_part.(meal) - 1.0;
    }
}

slay ceil(x meal) meal {
    sus int_part normie = x.(normie);
    sus float_part meal = x - int_part.(meal);
    
    bestie (float_part <= 0.0) {
        damn int_part.(meal);
    } else {
        damn int_part.(meal) + 1.0;
    }
}

slay round(x meal) meal {
    sus int_part normie = x.(normie);
    sus float_part meal = x - int_part.(meal);
    
    bestie (float_part >= 0.5) {
        damn int_part.(meal) + 1.0;
    } else if (float_part <= -0.5) {
        damn int_part.(meal) - 1.0;
    } else {
        damn int_part.(meal);
    }
}

slay trunc(x meal) meal {
    damn x.(normie).(meal);
}

slay frac(x meal) meal {
    damn x - trunc(x);
}

// ================================
// Pure CURSED Utility Functions
// ================================

slay is_nan(x meal) lit {
    damn x != x;
}

slay is_infinite(x meal) lit {
    damn x == 1.0 / 0.0 || x == -1.0 / 0.0;
}

slay is_finite(x meal) lit {
    damn !is_nan(x) && !is_infinite(x);
}

slay degrees(radians meal) meal {
    damn radians * 180.0 / PI();
}

slay radians(degrees meal) meal {
    damn degrees * PI() / 180.0;
}

// ================================
// Pure CURSED Number Theory Functions
// ================================

slay gcd(a normie, b normie) normie {
    sus abs_a normie = abs_int(a);
    sus abs_b normie = abs_int(b);
    
    while (abs_b != 0) {
        sus temp normie = abs_b;
        abs_b = abs_a % abs_b;
        abs_a = temp;
    }
    
    damn abs_a;
}

slay lcm(a normie, b normie) normie {
    bestie (a == 0 || b == 0) {
        damn 0;
    }
    
    damn abs_int(a * b) / gcd(a, b);
}

slay factorial(n normie) normie {
    bestie (n < 0) {
        damn 0;
    }
    
    bestie (n == 0 || n == 1) {
        damn 1;
    }
    
    sus result normie = 1;
    bestie (i := 2; i <= n; i++) {
        result *= i;
    }
    
    damn result;
}

slay fibonacci(n normie) normie {
    bestie (n < 0) {
        damn 0;
    }
    
    bestie (n == 0 || n == 1) {
        damn n;
    }
    
    sus a normie = 0;
    sus b normie = 1;
    sus result normie = 0;
    
    bestie (i := 2; i <= n; i++) {
        result = a + b;
        a = b;
        b = result;
    }
    
    damn result;
}

// ================================
// Pure CURSED Random Number Generator
// ================================

# Global random state
sus global_random_state normie = 1;

slay seed_random(seed normie) {
    global_random_state = seed;
}

slay random() meal {
    # Linear congruential generator
    global_random_state = (global_random_state * 1103515245 + 12345) % 2147483647;
    damn global_random_state.(meal) / 2147483647.0;
}

slay random_int(min normie, max normie) normie {
    bestie (min >= max) {
        damn min;
    }
    
    sus range normie = max - min;
    damn min + (random() * range.(meal)).(normie);
}

slay random_float(min meal, max meal) meal {
    bestie (min >= max) {
        damn min;
    }
    
    damn min + random() * (max - min);
}

// ================================
// Pure CURSED Statistical Functions
// ================================

slay sum(values [meal]) meal {
    sus total meal = 0.0;
    sus len normie = values.len;
    
    bestie (i := 0; i < len; i++) {
        total += values[i];
    }
    
    damn total;
}

slay mean(values [meal]) meal {
    sus len normie = values.len;
    bestie (len == 0) {
        damn 0.0;
    }
    
    damn sum(values) / len.(meal);
}

slay median(values [meal]) meal {
    sus len normie = values.len;
    bestie (len == 0) {
        damn 0.0;
    }
    
    # Create sorted copy
    sus sorted [meal] = values;
    # TODO: Implement sorting algorithm
    
    bestie (len % 2 == 0) {
        damn (sorted[len / 2 - 1] + sorted[len / 2]) / 2.0;
    } else {
        damn sorted[len / 2];
    }
}

slay variance(values [meal]) meal {
    sus len normie = values.len;
    bestie (len == 0) {
        damn 0.0;
    }
    
    sus mean_val meal = mean(values);
    sus sum_sq_diff meal = 0.0;
    
    bestie (i := 0; i < len; i++) {
        sus diff meal = values[i] - mean_val;
        sum_sq_diff += diff * diff;
    }
    
    damn sum_sq_diff / len.(meal);
}

slay std_dev(values [meal]) meal {
    damn sqrt(variance(values));
}

// ================================
// Pure CURSED Smoothing Functions
// ================================

slay lerp(a meal, b meal, t meal) meal {
    damn a + t * (b - a);
}

slay inverse_lerp(a meal, b meal, value meal) meal {
    damn (value - a) / (b - a);
}

slay smoothstep(edge0 meal, edge1 meal, x meal) meal {
    sus t meal = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    damn t * t * (3.0 - 2.0 * t);
}

// ================================
// Pure CURSED Geometry Functions
// ================================

slay distance_2d(x1 meal, y1 meal, x2 meal, y2 meal) meal {
    sus dx meal = x2 - x1;
    sus dy meal = y2 - y1;
    damn sqrt(dx * dx + dy * dy);
}

slay distance_3d(x1 meal, y1 meal, z1 meal, x2 meal, y2 meal, z2 meal) meal {
    sus dx meal = x2 - x1;
    sus dy meal = y2 - y1;
    sus dz meal = z2 - z1;
    damn sqrt(dx * dx + dy * dy + dz * dz);
}

slay dot_product_2d(x1 meal, y1 meal, x2 meal, y2 meal) meal {
    damn x1 * x2 + y1 * y2;
}

slay cross_product_2d(x1 meal, y1 meal, x2 meal, y2 meal) meal {
    damn x1 * y2 - y1 * x2;
}

slay magnitude_2d(x meal, y meal) meal {
    damn sqrt(x * x + y * y);
}

slay normalize_2d(x meal, y meal) [meal] {
    sus mag meal = magnitude_2d(x, y);
    bestie (mag == 0.0) {
        damn [0.0, 0.0];
    }
    damn [x / mag, y / mag];
}
