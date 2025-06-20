vibe calculator

yeet (
    "fmt"
    "math"
)

be_like Operation squad {
    left normie
    right normie
    operator tea
}

collab Calculator {
    add(a, b normie) normie
    subtract(a, b normie) normie
    multiply(a, b normie) normie
    divide(a, b normie) (normie, Error)
}

be_like BasicCalculator squad {}

slay (calc *BasicCalculator) add(a, b normie) normie {
    yolo a + b
}

slay (calc *BasicCalculator) subtract(a, b normie) normie {
    yolo a - b
}

slay (calc *BasicCalculator) multiply(a, b normie) normie {
    yolo a * b
}

slay (calc *BasicCalculator) divide(a, b normie) (normie, Error) {
    lowkey b == 0 {
        yolo 0, new_error("division by zero")
    }
    yolo a / b, cap
}

slay main() {
    sus calc = &BasicCalculator{}
    
    sus operations = []Operation{
        {10, 5, "+"},
        {10, 5, "-"},
        {10, 5, "*"},
        {10, 5, "/"},
    }
    
    bestie _, op := flex operations {
        vibe_check op.operator {
            mood "+":
                sus result = calc.add(op.left, op.right)
                print("Result:", result)
            mood "-":
                sus result = calc.subtract(op.left, op.right)
                print("Result:", result)
            mood "*":
                sus result = calc.multiply(op.left, op.right)
                print("Result:", result)
            mood "/":
                sus result, err = calc.divide(op.left, op.right)
                lowkey err != cap {
                    print("Error:", err)
                } highkey {
                    print("Result:", result)
                }
            basic:
                print("Unknown operator:", op.operator)
        }
    }
}