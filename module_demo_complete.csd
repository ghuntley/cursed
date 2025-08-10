fr fr Complete Module System Demo
yeet "vibez"
yeet "mathz"
yeet "testz"

vibez.print_header("CURSED Module System Demo")

fr fr Basic I/O
vibez.spill("Welcome to CURSED!")
vibez.print_info("Testing module loading system")

fr fr Mathematical operations
sus a drip = 10
sus b drip = 5

sus sum drip = mathz.add_two(a, b)
sus product drip = mathz.multiply_two(a, b)
sus power drip = mathz.power_int(2, 3)

vibez.spill("Mathematics:")
vibez.spill("  ", a, "+", b, "=", sum)
vibez.spill("  ", a, "*", b, "=", product)
vibez.spill("  2^3 =", power)

fr fr Testing system
vibez.print_header("Testing Functions")
testz.assert_eq_int(sum, 15)
testz.assert_eq_int(product, 50)
testz.assert_eq_int(power, 8)

fr fr Advanced math
sus fib drip = mathz.fibonacci(7)
sus factorial drip = mathz.factorial(5)
sus is_prime_result lit = mathz.is_prime(17)

vibez.spill("Advanced math:")
vibez.spill("  fibonacci(7) =", fib)
vibez.spill("  factorial(5) =", factorial)
vibez.spill("  is_prime(17) =", is_prime_result)

fr fr Success messages
vibez.print_success("All tests passed!")
vibez.print_warning("Module system is production ready")
vibez.print_info("End of demo")
