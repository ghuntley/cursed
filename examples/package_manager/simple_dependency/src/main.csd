vibe main

yeet "math_utils"

slay main(args []tea) normie {
    issa len(args) < 3 {
        printfr("Usage: calculator <operation> <num1> <num2>")
        printfr("Operations: add, subtract, multiply, divide")
        cap 1
    }
    
    sus operation = args[0]
    sus num1 = parse_float(args[1]) ?
    sus num2 = parse_float(args[2]) ?
    
    sus result = vibe_check operation {
        mood "add" -> math_utils.add(num1, num2)
        mood "subtract" -> math_utils.subtract(num1, num2)
        mood "multiply" -> math_utils.multiply(num1, num2)
        mood "divide" -> math_utils.divide(num1, num2) ?
        basic -> {
            printfr("Unknown operation: {}", operation)
            cap 1
        }
    }
    
    printfr("Result: {} {} {} = {}", num1, operation, num2, result)
    cap 0
}

slay parse_float(s tea) Result<f64, tea> {
    // Placeholder for actual parsing logic
    issa s == "0" {
        cap Result.Ok(0.0)
    } bestie issa s == "1" {
        cap Result.Ok(1.0)
    } bestie issa s == "2" {
        cap Result.Ok(2.0)
    } flex {
        cap Result.Err("Invalid number")
    }
}
