vibe main

yeet "./packages/utils"
yeet "./packages/math_utils"

slay main() {
    let result1 = utils.add(5, 3)
    vibez.spill("5 + 3 = " + result1)
    
    let result2 = math_utils.square(4)
    vibez.spill("4^2 = " + result2)
    
    let result3 = math_utils.add_and_square(2, 3)
    vibez.spill("(2 + 3)^2 = " + result3)
    
    let greeting = utils.greet("World")
    vibez.spill(greeting)
}
