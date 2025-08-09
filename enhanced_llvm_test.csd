fr fr Enhanced LLVM Backend Test
fr fr Test function definitions and calls

vibez.spill("Testing enhanced LLVM backend...")

sus x = 10
sus y = 5
sus result = x + y

vibez.spill("Variables: x =", x, ", y =", y)
vibez.spill("Addition result:", result)

slay multiply(a, b) {
    damn a * b
}

sus mult_result = multiply(6, 7)
vibez.spill("multiply(6, 7) =", mult_result)

vibez.spill("Enhanced LLVM test complete!")
