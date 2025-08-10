fr fr Test basic functionality with new pure CURSED implementations

vibez.spill("Testing basic CURSED math operations...")

fr fr Test the basic abs function that was in stdlib
sus x drip = -5
ready (x < 0) {
    x = -x
}
vibez.spill("abs(-5) =", x)

fr fr Test simple power calculation
sus base drip = 2
sus result drip = base * base * base
vibez.spill("power(2, 3) =", result)

fr fr Test basic string operations
sus str1 tea = "hello"
sus str2 tea = "world"
sus combined tea = str1 + " " + str2
vibez.spill("String concatenation:", combined)

fr fr Test basic array operations
sus nums []drip = [1, 2, 3, 4, 5]
sus total drip = 0
sus i drip = 0
bestie (i < len(nums)) {
    total = total + nums[i]
    i = i + 1
}
vibez.spill("Array sum:", total)

vibez.spill("✅ Basic operations working successfully!")
