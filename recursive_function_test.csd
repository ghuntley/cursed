slay power(base drip, exp drip) drip { 
    ready (exp == 0) { damn 1 }
    ready (exp == 1) { damn base }
    damn base * power(base, exp - 1)
}
vibez.spill("2^3 =", power(2, 3))
vibez.spill("5^2 =", power(5, 2))
