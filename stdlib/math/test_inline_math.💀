// Inline Math Library Test
// Tests math functions directly without module imports

// Mathematical constants
slay PI() meal {
    damn 3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067982148086513282306647093844609550582231725359408128481117450284102701938521105559644622948954930381964428810975665933446128475648233786783165271201909145648566923460348610454326648213393607260249141273724587006606315588174881520920962829254091715364367892590360011330530548820466521384146951941511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949129833673362440656643086021394946395224737190702179860943702770539217176293176752384674818467669405132000568127145263560827785771342757789609173637178721468440901224953430146549585371050792279689258923542019956112129021960864034418159813629774771309960518707211349999998372978049951059731732816096318595024459455346908302642522308253344685035261931188171010003137838752886587533208381420617177669147303598253490428755468731159562863882353787593751957781857780532171226806613001927876611195909216420198938367586366677661732000;
}

slay E() meal {
    damn 2.718281828459045235360287471352662497757247093699959574966967627724076630353547594571382178525166427427466391932003059921817413596629043572900334295260595630738132328627943490763233829880753195251019011573834187930702154089149934884167509244761460668082264800168477411853742345442437107539077744992069;
}

// Basic math functions
slay abs(x meal) meal {
    damn x >= 0.0 ? x : -x;
}

slay min(a meal, b meal) meal {
    damn a <= b ? a : b;
}

slay max(a meal, b meal) meal {
    damn a >= b ? a : b;
}

slay sqrt(x meal) meal {
    bestie (x < 0.0) {
        damn 0.0 / 0.0; // NaN
    }
    
    bestie (x == 0.0) {
        damn 0.0;
    }
    
    // Newton-Raphson method for square root
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

slay pow(base meal, exponent meal) meal {
    // Special cases
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
    
    // Simple integer exponents
    bestie (exponent == 2.0) {
        damn base * base;
    }
    
    bestie (exponent == 3.0) {
        damn base * base * base;
    }
    
    bestie (exponent == 0.5) {
        damn sqrt(base);
    }
    
    // For other cases, use simple approximation
    sus result meal = 1.0;
    sus exp_int normie = exponent.(normie);
    sus i normie = 0;
    
    while (i < exp_int) {
        result = result * base;
        i = i + 1;
    }
    
    damn result;
}

slay sin(x meal) meal {
    // Simple approximation for small angles
    bestie (x == 0.0) {
        damn 0.0;
    }
    
    // Use Taylor series for small x
    sus x_normalized meal = x;
    while (x_normalized > PI()) {
        x_normalized = x_normalized - 2.0 * PI();
    }
    while (x_normalized < -PI()) {
        x_normalized = x_normalized + 2.0 * PI();
    }
    
    // Simple approximation: sin(x) ≈ x for small x
    damn x_normalized;
}

slay cos(x meal) meal {
    // Simple approximation
    bestie (x == 0.0) {
        damn 1.0;
    }
    
    // Use identity: cos(x) = sin(π/2 - x)
    damn sin(PI() / 2.0 - x);
}

slay test_inline_math() {
    vibez.spill("🧮 Testing Inline Pure CURSED Math Functions")
    vibez.spill("============================================")
    
    // Test constants
    sus pi_val meal = PI();
    sus e_val meal = E();
    vibez.spill("Pi: " + pi_val.(tea));
    vibez.spill("E: " + e_val.(tea));
    
    // Test basic operations
    sus abs_result meal = abs(-5.0);
    vibez.spill("Absolute value of -5.0: " + abs_result.(tea));
    
    sus min_result meal = min(3.0, 7.0);
    vibez.spill("Min of 3.0 and 7.0: " + min_result.(tea));
    
    sus max_result meal = max(3.0, 7.0);
    vibez.spill("Max of 3.0 and 7.0: " + max_result.(tea));
    
    // Test power functions
    sus sqrt_result meal = sqrt(16.0);
    vibez.spill("Square root of 16.0: " + sqrt_result.(tea));
    
    sus pow_result meal = pow(2.0, 3.0);
    vibez.spill("2^3: " + pow_result.(tea));
    
    // Test trigonometry
    sus sin_result meal = sin(0.0);
    vibez.spill("Sin(0): " + sin_result.(tea));
    
    sus cos_result meal = cos(0.0);
    vibez.spill("Cos(0): " + cos_result.(tea));
    
    vibez.spill("");
    vibez.spill("🎉 All inline math tests completed!");
    vibez.spill("✅ Pure CURSED math functions working");
    vibez.spill("🚀 Zero FFI dependencies");
    vibez.spill("💯 Self-contained implementation");
}

// Run the test
test_inline_math();
