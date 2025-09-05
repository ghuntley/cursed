fr fr Big Mood Mathematics Demo - Arbitrary Precision Arithmetic in CURSED
fr fr 
fr fr This example demonstrates the comprehensive big_mood module capabilities:
fr fr - BigInt: Unlimited precision integer arithmetic
fr fr - BigRat: Exact rational number computations
fr fr - BigFloat: High-precision floating-point operations
fr fr - Decimal: Exact decimal arithmetic for financial calculations
fr fr - BigComplex: Arbitrary precision complex numbers
fr fr - Mathematical functions with unlimited precision
fr fr - Cryptographic prime generation
fr fr - Performance optimizations for large numbers

yeet "stdlib::math::big_mood"
yeet "stdlib::io"

slay main_character() {
    io.println("🔥 CURSED Big Mood Mathematics Demo 🔥")?
    io.println("=======================================")
    
    fr fr Demonstrate BigInt operations
    demonstrate_big_integers()?
    
    fr fr Demonstrate BigRat operations
    demonstrate_big_rationals()?
    
    fr fr Demonstrate BigFloat operations
    demonstrate_big_floats()?
    
    fr fr Demonstrate Decimal operations
    demonstrate_decimal_arithmetic()?
    
    fr fr Demonstrate BigComplex operations
    demonstrate_big_complex()?
    
    fr fr Demonstrate mathematical functions
    demonstrate_mathematical_functions()?
    
    fr fr Demonstrate cryptographic applications
    demonstrate_cryptographic_primes()?
    
    fr fr Demonstrate financial calculations
    demonstrate_financial_calculations()?
    
    fr fr Demonstrate performance with large numbers
    demonstrate_large_number_performance()?
    
    io.println("\n✨ Big mood mathematics demo complete! ✨")
}

slay demonstrate_big_integers() -> MathResult<()> {
    io.println("\n🔢 BigInt: Unlimited Precision Integers")
    io.println("=====================================")
    
    fr fr Create large integers that exceed native integer limits
    facts x = big_mood.NewInt(1234567890123456789)
    facts y = big_mood.NewInt(9876543210987654321)
    
    io.printf("Large integer x: {}\n", &[x.String()])?
    io.printf("Large integer y: {}\n", &[y.String()])?
    
    fr fr Basic arithmetic operations
    facts sum = x.Add(y)
    facts diff = y.Sub(x)
    facts product = x.Mul(y)
    facts quotient = y.Div(x)
    facts remainder = y.Mod(x)
    
    io.printf("x + y = {}\n", &[sum.String()])?
    io.printf("y - x = {}\n", &[diff.String()])?
    io.printf("x * y = {}\n", &[product.String()])?
    io.printf("y / x = {}\n", &[quotient.String()])?
    io.printf("y %% x = {}\n", &[remainder.String()])?
    
    fr fr Working with very large numbers (beyond 64-bit limits)
    facts huge_number = big_mood.ParseInt("123456789012345678901234567890123456789", 10)
    io.printf("Huge number: {}\n", &[huge_number.String()])?
    
    fr fr Demonstrate different number bases
    facts hex_num = big_mood.ParseInt("DEADBEEFCAFEBABE", 16)
    facts binary_num = big_mood.ParseInt("1010101010101010", 2)
    
    io.printf("Hex DEADBEEFCAFEBABE = {}\n", &[hex_num.String()])?
    io.printf("Binary 1010101010101010 = {}\n", &[binary_num.String()])?
    io.printf("Hex representation: {}\n", &[huge_number.Text(16)])?
    io.printf("Binary representation: {}\n", &[huge_number.Text(2)])?
    
    fr fr Bitwise operations on large numbers
    facts left_shifted = huge_number.Lsh(10)  fr fr Left shift by 10 bits
    facts right_shifted = huge_number.Rsh(5)  fr fr Right shift by 5 bits
    
    io.printf("Left shift by 10 bits: {}\n", &[left_shifted.String()])?
    io.printf("Right shift by 5 bits: {}\n", &[right_shifted.String()])?
    
    fr fr Modular exponentiation for cryptography
    facts base = big_mood.NewInt(2)
    facts exponent = big_mood.NewInt(1024)
    facts modulus = big_mood.ParseInt("1000000000000000000000000000000037", 10)
    facts result = base.Exp(exponent, modulus)
    
    io.printf("2^1024 mod {} = {}\n", &[modulus.String(), result.String()])?
    
    vibez.spill("")
}

slay demonstrate_big_rationals() -> MathResult<()> {
    io.println("🧮 BigRat: Exact Rational Arithmetic")
    io.println("===================================")
    
    fr fr Create exact fractions
    facts pi_approx = big_mood.NewRat(22, 7)  fr fr Classic π approximation
    facts golden_ratio = big_mood.NewRat(1618, 1000)  fr fr φ approximation
    
    io.printf("π approximation (22/7): {}\n", &[pi_approx.String()])?
    io.printf("Golden ratio approximation: {}\n", &[golden_ratio.String()])?
    
    fr fr Exact fraction arithmetic - no floating point errors!
    facts one_third = big_mood.NewRat(1, 3)
    facts two_thirds = big_mood.NewRat(2, 3)
    facts sum = one_third.Add(two_thirds)
    
    io.printf("1/3 + 2/3 = {} (exactly 1!)\n", &[sum.String()])?
    
    fr fr Complex fraction operations
    facts a = big_mood.NewRat(3, 7)
    facts b = big_mood.NewRat(5, 11)
    
    facts rat_sum = a.Add(b)
    facts rat_diff = a.Sub(b)
    facts rat_product = a.Mul(b)
    facts rat_quotient = a.Quo(b)
    
    io.printf("3/7 + 5/11 = {}\n", &[rat_sum.String()])?
    io.printf("3/7 - 5/11 = {}\n", &[rat_diff.String()])?
    io.printf("3/7 * 5/11 = {}\n", &[rat_product.String()])?
    io.printf("3/7 / 5/11 = {}\n", &[rat_quotient.String()])?
    
    fr fr Convert to high-precision decimal
    facts float_val, accuracy = pi_approx.Float64()
    io.printf("22/7 as float: {} (accuracy: {})\n", &[float_val, accuracy])?
    
    fr fr Working with very precise fractions
    facts precise_fraction = big_mood.ParseRat("355/113")  fr fr Better π approximation
    io.printf("Better π approximation (355/113): {}\n", &[precise_fraction.String()])?
    io.printf("As decimal: {}\n", &[precise_fraction.FloatString(15)])?
    
    vibez.spill("")
}

slay demonstrate_big_floats() -> MathResult<()> {
    io.println("🌊 BigFloat: High-Precision Floating Point")
    io.println("==========================================")
    
    fr fr Create high-precision floating point numbers
    facts pi_100 = big_mood.ParseFloat(
        "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679", 
        400  fr fr 400 bits of precision
    )
    
    facts e_100 = big_mood.ParseFloat(
        "2.7182818284590452353602874713526624977572470936999595749669676277240766303535475945713821785251664274",
        400
    )
    
    io.printf("π with 100 digits: {}\n", &[pi_100.Text('f', 50)])?
    io.printf("e with 100 digits: {}\n", &[e_100.Text('f', 50)])?
    
    fr fr High-precision arithmetic
    facts pi_plus_e = pi_100.Add(e_100)
    facts pi_times_e = pi_100.Mul(e_100)
    facts pi_to_e = pi_100.Pow(e_100)
    
    io.printf("π + e = {}\n", &[pi_plus_e.Text('f', 30)])?
    io.printf("π * e = {}\n", &[pi_times_e.Text('f', 30)])?
    io.printf("π^e = {}\n", &[pi_to_e.Text('f', 20)])?
    
    fr fr Demonstrate precision control
    facts low_prec = big_mood.NewFloat(1.0).SetPrec(32)
    facts med_prec = big_mood.NewFloat(1.0).SetPrec(64)
    facts high_prec = big_mood.NewFloat(1.0).SetPrec(256)
    
    io.printf("1/3 with 32-bit precision: {}\n", &[low_prec.Quo(big_mood.NewFloat(3.0)).String()])?
    io.printf("1/3 with 64-bit precision: {}\n", &[med_prec.Quo(big_mood.NewFloat(3.0)).String()])?
    io.printf("1/3 with 256-bit precision: {}\n", &[high_prec.Quo(big_mood.NewFloat(3.0)).String()])?
    
    vibez.spill("")
}

slay demonstrate_decimal_arithmetic() -> MathResult<()> {
    io.println("💰 Decimal: Exact Financial Arithmetic")
    io.println("======================================")
    
    fr fr Financial calculations require exact decimal arithmetic
    facts price = big_mood.NewDecimal("19.99")
    facts quantity = big_mood.NewDecimal("3")
    facts tax_rate = big_mood.NewDecimal("0.0825")  fr fr 8.25% sales tax
    
    io.printf("Item price: ${}\n", &[price.String()])?
    io.printf("Quantity: {}\n", &[quantity.String()])?
    io.printf("Tax rate: {}%%\n", &[tax_rate.String()])?
    
    fr fr Calculate subtotal, tax, and total
    facts subtotal = price.Mul(quantity)
    facts tax_amount = subtotal.Mul(tax_rate)
    facts total = subtotal.Add(tax_amount)
    
    io.printf("Subtotal: ${}\n", &[subtotal.String()])?
    io.printf("Tax amount: ${}\n", &[tax_amount.String()])?
    io.printf("Total: ${}\n", &[total.String()])?
    
    fr fr Demonstrate exact decimal arithmetic (no 0.1 + 0.2 = 0.30000000000000004!)
    facts tenth = big_mood.NewDecimal("0.1")
    facts fifth = big_mood.NewDecimal("0.2")
    facts sum = tenth.Add(fifth)
    
    io.printf("0.1 + 0.2 = {} (exactly!)\n", &[sum.String()])?
    
    fr fr Currency exchange calculations
    facts usd_amount = big_mood.NewDecimal("1000.00")
    facts exchange_rate = big_mood.NewDecimal("1.2345")  fr fr USD to EUR
    facts eur_amount = usd_amount.Mul(exchange_rate)
    
    io.printf("${} USD = €{} EUR (rate: {})\n", &[usd_amount.String(), eur_amount.String(), exchange_rate.String()])?
    
    fr fr Compound interest calculation
    facts principal = big_mood.NewDecimal("10000.00")
    facts annual_rate = big_mood.NewDecimal("0.05")  fr fr 5% annual
    facts years = 10
    
    facts compound_factor = big_mood.NewDecimal("1").Add(annual_rate)
    facts final_amount = principal
    
    lowkey (sus year = 0; year < years; year++) {
        final_amount = final_amount.Mul(compound_factor)
    }
    
    io.printf("${} at 5%% for {} years = ${}\n", &[principal.String(), years, final_amount.String()])?
    
    vibez.spill("")
}

slay demonstrate_big_complex() -> MathResult<()> {
    io.println("🌀 BigComplex: High-Precision Complex Numbers")
    io.println("=============================================")
    
    fr fr Create complex numbers with high precision
    facts real1 = big_mood.NewFloat(3.0).SetPrec(128)
    facts imag1 = big_mood.NewFloat(4.0).SetPrec(128)
    facts c1 = big_mood.NewComplex(real1, imag1)  fr fr 3 + 4i
    
    facts real2 = big_mood.NewFloat(1.0).SetPrec(128)
    facts imag2 = big_mood.NewFloat(2.0).SetPrec(128)
    facts c2 = big_mood.NewComplex(real2, imag2)  fr fr 1 + 2i
    
    io.printf("c1 = {} + {}i\n", &[c1.Real().String(), c1.Imag().String()])?
    io.printf("c2 = {} + {}i\n", &[c2.Real().String(), c2.Imag().String()])?
    
    fr fr Complex arithmetic
    facts sum = c1.Add(c2)
    facts diff = c1.Sub(c2)
    facts product = c1.Mul(c2)
    facts quotient = c1.Div(c2)
    
    io.printf("c1 + c2 = {} + {}i\n", &[sum.Real().String(), sum.Imag().String()])?
    io.printf("c1 - c2 = {} + {}i\n", &[diff.Real().String(), diff.Imag().String()])?
    io.printf("c1 * c2 = {} + {}i\n", &[product.Real().String(), product.Imag().String()])?
    io.printf("c1 / c2 = {} + {}i\n", &[quotient.Real().String(), quotient.Imag().String()])?
    
    fr fr Complex magnitude and phase
    facts magnitude = c1.Abs()
    facts phase = c1.Phase()
    
    io.printf("|c1| = {}\n", &[magnitude.String()])?
    io.printf("arg(c1) = {}\n", &[phase.String()])?
    
    fr fr Mandelbrot set calculation example point
    facts z = big_mood.NewComplex(big_mood.NewFloat(0), big_mood.NewFloat(0))
    facts c = big_mood.NewComplex(big_mood.NewFloat(-0.5), big_mood.NewFloat(0.6))
    
    io.printf("Mandelbrot iteration for c = {} + {}i:\n", &[c.Real().String(), c.Imag().String()])?
    
    lowkey (sus iteration = 0; iteration < 10; iteration++) {
        z = z.Mul(z).Add(c)  fr fr z = z² + c
        facts mag = z.Abs()
        io.printf("  Iteration {}: |z| = {}\n", &[iteration + 1, mag.String()])?
        
        lowkey (mag.Cmp(big_mood.NewFloat(2.0)) > 0) {
            io.printf("  Diverged at iteration {}\n", &[iteration + 1])?
            bestie
        }
    }
    
    vibez.spill("")
}

slay demonstrate_mathematical_functions() -> MathResult<()> {
    io.println("📐 Mathematical Functions with Arbitrary Precision")
    io.println("=================================================")
    
    fr fr High-precision square roots
    facts two = big_mood.NewFloat(2.0).SetPrec(200)
    facts sqrt_two = big_mood.Sqrt(two)
    
    io.printf("√2 with 200-bit precision: {}\n", &[sqrt_two.Text('f', 50)])?
    
    fr fr High-precision trigonometry
    facts pi = big_mood.NewFloat(3.14159265358979323846).SetPrec(200)
    facts sin_pi_6 = big_mood.Sin(pi.Quo(big_mood.NewFloat(6.0)))  fr fr sin(π/6) = 0.5
    facts cos_pi_3 = big_mood.Cos(pi.Quo(big_mood.NewFloat(3.0)))  fr fr cos(π/3) = 0.5
    
    io.printf("sin(π/6) = {} (should be 0.5)\n", &[sin_pi_6.Text('f', 20)])?
    io.printf("cos(π/3) = {} (should be 0.5)\n", &[cos_pi_3.Text('f', 20)])?
    
    fr fr High-precision logarithms and exponentials
    facts e = big_mood.NewFloat(2.71828182845904523536).SetPrec(200)
    facts ln_e = big_mood.Log(e)
    facts exp_1 = big_mood.Exp(big_mood.NewFloat(1.0))
    
    io.printf("ln(e) = {} (should be 1)\n", &[ln_e.Text('f', 20)])?
    io.printf("exp(1) = {} (should be e)\n", &[exp_1.Text('f', 20)])?
    
    fr fr nth roots with high precision
    facts large_number = big_mood.NewFloat(1024.0)
    facts tenth_root = big_mood.Root(large_number, 10)
    
    io.printf("10th root of 1024 = {} (should be 2)\n", &[tenth_root.Text('f', 15)])?
    
    vibez.spill("")
}

slay demonstrate_cryptographic_primes() -> MathResult<()> {
    io.println("🔐 Cryptographic Prime Generation")
    io.println("=================================")
    
    fr fr Generate cryptographically secure random primes
    facts rand = math_rand_tea.New(math_rand_tea.NewSource(timez.Now().UnixNano()))
    
    io.println("Generating random primes for cryptographic use:")
    
    fr fr Small prime for demonstration
    facts small_prime = big_mood.RandPrime(rand, 32)  fr fr 32-bit prime
    io.printf("32-bit prime: {}\n", &[small_prime.String()])?
    
    fr fr Medium prime
    facts medium_prime = big_mood.RandPrime(rand, 64)  fr fr 64-bit prime
    io.printf("64-bit prime: {}\n", &[medium_prime.String()])?
    
    fr fr Large prime suitable for RSA
    facts large_prime = big_mood.RandPrime(rand, 256)  fr fr 256-bit prime
    io.printf("256-bit prime: {}\n", &[large_prime.String()])?
    
    fr fr Demonstrate prime testing
    facts test_number = big_mood.NewInt(97)  fr fr Known prime
    facts is_prime = test_number.ProbablyPrime(10)  fr fr Miller-Rabin test
    io.printf("{} is probably prime: {}\n", &[test_number.String(), is_prime])?
    
    fr fr RSA key generation example (simplified)
    facts p = big_mood.RandPrime(rand, 128)
    facts q = big_mood.RandPrime(rand, 128)
    facts n = p.Mul(q)  fr fr RSA modulus
    
    io.printf("RSA p: {}\n", &[p.String()])?
    io.printf("RSA q: {}\n", &[q.String()])?
    io.printf("RSA n (p*q): {}\n", &[n.String()])?
    
    vibez.spill("")
}

slay demonstrate_financial_calculations() -> MathResult<()> {
    io.println("🏦 Real-World Financial Calculations")
    io.println("====================================")
    
    fr fr Mortgage calculation
    facts principal = big_mood.NewDecimal("500000.00")  fr fr $500,000 loan
    facts annual_rate = big_mood.NewDecimal("0.035")     fr fr 3.5% APR
    facts years = 30
    
    fr fr Monthly payment calculation
    facts monthly_rate = annual_rate.Quo(big_mood.NewDecimal("12"))
    facts num_payments = years * 12
    
    fr fr Payment = P * [r(1+r)^n] / [(1+r)^n - 1]
    facts one_plus_r = big_mood.NewDecimal("1").Add(monthly_rate)
    facts power_term = one_plus_r.Pow(num_payments)
    facts numerator = principal.Mul(monthly_rate).Mul(power_term)
    facts denominator = power_term.Sub(big_mood.NewDecimal("1"))
    facts monthly_payment = numerator.Quo(denominator)
    
    io.printf("Mortgage Details:\n")?
    io.printf("  Principal: ${}\n", &[principal.String()])?
    io.printf("  Annual Rate: {}%%\n", &[annual_rate.Mul(big_mood.NewDecimal("100")).String()])?
    io.printf("  Term: {} years\n", &[years])?
    io.printf("  Monthly Payment: ${}\n", &[monthly_payment.String()])?
    
    fr fr Investment portfolio calculation
    facts stocks = big_mood.NewDecimal("50000.00")
    facts bonds = big_mood.NewDecimal("30000.00")
    facts cash = big_mood.NewDecimal("20000.00")
    facts total_portfolio = stocks.Add(bonds).Add(cash)
    
    facts stock_return = big_mood.NewDecimal("0.08")   fr fr 8% expected return
    facts bond_return = big_mood.NewDecimal("0.04")    fr fr 4% expected return
    facts cash_return = big_mood.NewDecimal("0.01")    fr fr 1% expected return
    
    facts expected_portfolio_return = stocks.Mul(stock_return)
        .Add(bonds.Mul(bond_return))
        .Add(cash.Mul(cash_return))
        .Quo(total_portfolio)
    
    io.printf("\nPortfolio Analysis:\n")?
    io.printf("  Stocks: ${} ({}%%)\n", &[stocks.String(), stocks.Quo(total_portfolio).Mul(big_mood.NewDecimal("100")).String()])?
    io.printf("  Bonds: ${} ({}%%)\n", &[bonds.String(), bonds.Quo(total_portfolio).Mul(big_mood.NewDecimal("100")).String()])?
    io.printf("  Cash: ${} ({}%%)\n", &[cash.String(), cash.Quo(total_portfolio).Mul(big_mood.NewDecimal("100")).String()])?
    io.printf("  Total: ${}\n", &[total_portfolio.String()])?
    io.printf("  Expected Return: {}%%\n", &[expected_portfolio_return.Mul(big_mood.NewDecimal("100")).String()])?
    
    fr fr Currency arbitrage calculation
    facts usd_eur = big_mood.NewDecimal("0.85")    fr fr USD to EUR
    facts eur_jpy = big_mood.NewDecimal("130.0")   fr fr EUR to JPY  
    facts jpy_usd = big_mood.NewDecimal("0.0068")  fr fr JPY to USD
    
    facts arbitrage_result = big_mood.NewDecimal("1000.00")
        .Mul(usd_eur)      fr fr USD -> EUR
        .Mul(eur_jpy)      fr fr EUR -> JPY
        .Mul(jpy_usd)      fr fr JPY -> USD
    
    io.printf("\nCurrency Arbitrage (starting with $1000):\n")?
    io.printf("  USD -> EUR: {}\n", &[usd_eur.String()])?
    io.printf("  EUR -> JPY: {}\n", &[eur_jpy.String()])?
    io.printf("  JPY -> USD: {}\n", &[jpy_usd.String()])?
    io.printf("  Final USD: ${}\n", &[arbitrage_result.String()])?
    io.printf("  Profit/Loss: ${}\n", &[arbitrage_result.Sub(big_mood.NewDecimal("1000.00")).String()])?
    
    vibez.spill("")
}

slay demonstrate_large_number_performance() -> MathResult<()> {
    io.println("⚡ Performance with Very Large Numbers")
    io.println("=====================================")
    
    fr fr Fibonacci calculation with large numbers
    io.println("Calculating large Fibonacci numbers...")
    
    facts start_time = timez.Now()
    facts fib_1000 = calculate_fibonacci_big(1000)
    facts duration = timez.Since(start_time)
    
    io.printf("Fibonacci(1000) = {}\n", &[fib_1000.String()])?
    io.printf("Calculated in: {}ms\n", &[duration.Milliseconds()])?
    io.printf("Number of digits: {}\n", &[len(fib_1000.String())])?
    
    fr fr Factorial calculation
    io.println("\nCalculating large factorials...")
    
    start_time = timez.Now()
    facts factorial_100 = calculate_factorial_big(100)
    duration = timez.Since(start_time)
    
    io.printf("100! = {}\n", &[factorial_100.String()])?
    io.printf("Calculated in: {}ms\n", &[duration.Milliseconds()])?
    io.printf("Number of digits: {}\n", &[len(factorial_100.String())])?
    
    fr fr Large prime factorization
    io.println("\nLarge number operations...")
    
    facts large_a = big_mood.ParseInt("123456789012345678901234567890", 10)
    facts large_b = big_mood.ParseInt("987654321098765432109876543210", 10)
    
    start_time = timez.Now()
    facts large_product = big_mood.FastMul(large_a, large_b)
    duration = timez.Since(start_time)
    
    io.printf("Product of 30-digit numbers calculated in: {}μs\n", &[duration.Microseconds()])?
    io.printf("Result has {} digits\n", &[len(large_product.String())])?
    
    fr fr Memory usage demonstration
    io.printf("\nMemory usage for large numbers:\n")?
    io.printf("  Large integer: ~{} bytes\n", &[large_a.BitLen() / 8])?
    io.printf("  Product: ~{} bytes\n", &[large_product.BitLen() / 8])?
    
    vibez.spill("")
}

slay calculate_fibonacci_big(n int) -> *big_mood.Int {
    lowkey (n <= 1) {
        damn big_mood.NewInt(n)
    }
    
    facts a = big_mood.NewInt(0)
    facts b = big_mood.NewInt(1)
    
    lowkey (sus i = 2; i <= n; i++) {
        facts temp = a.Add(b)
        a = b
        b = temp
    }
    
    damn b
}

slay calculate_factorial_big(n int) -> *big_mood.Int {
    facts result = big_mood.NewInt(1)
    
    lowkey (sus i = 2; i <= n; i++) {
        result = result.Mul(big_mood.NewInt(i))
    }
    
    damn result
}
